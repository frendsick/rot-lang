use crate::assembly::generate_assembly;
use crate::class::function::{function_defined, Function};
use crate::class::op::Op;
use crate::class::signature::{Parameter, Signature};
use crate::class::token::{Delimiter, Keyword, Token, TokenType};
use crate::constant::MAIN_FUNCTION_NAME;
use crate::data_types::{datatype_from_string, DataType};
use crate::ir::parse_ops;
use crate::lexer::tokenize_code_file;

#[derive(Debug)]
pub enum CompilerError {
    IOError(std::io::Error),
    ParserError(String),
}

pub fn compile_rot_file(rot_file: &str, _out_file: Option<String>) -> Result<(), CompilerError> {
    let tokens: Vec<Token> = tokenize_code_file(&rot_file)?;
    let functions: Vec<Function> = parse_functions(&tokens)?;
    for function in functions {
        let ops: Vec<Op> = parse_ops(&function.tokens);
        dbg!(&ops);
    }
    // TODO: Generate assembly code
    // TODO: Compile the program
    Ok(())
}

fn parse_functions(tokens: &Vec<Token>) -> Result<Vec<Function>, CompilerError> {
    let mut functions: Vec<Function> = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        if token.typ == TokenType::Keyword(Keyword::Function) {
            if i >= tokens.len() - 1 {
                return Err(CompilerError::ParserError(format!(
                    "Code cannot end with '{}' keyword",
                    token.value
                )));
            }
            functions.push(parse_function(tokens[i + 1..].to_vec())?);
        }
    }
    if !function_defined(MAIN_FUNCTION_NAME, &functions) {
        panic!("The '{}' function is not defined", MAIN_FUNCTION_NAME);
    }
    Ok(functions)
}

/// In Rot, function is defined with the following syntax:
/// function <name>(param1: int, param2: str) -> bool { <code> }
fn parse_function(tokens: Vec<Token>) -> Result<Function, CompilerError> {
    let mut cursor: usize = 0;
    let name: String = tokens[0].value.clone();
    advance_cursor(&mut cursor, &tokens, TokenType::Identifier)?;
    let signature: Signature = parse_function_signature(&mut cursor, &tokens)?;
    advance_cursor(
        &mut cursor,
        &tokens,
        TokenType::Delimiter(Delimiter::OpenCurly),
    )?;

    // Parse until the closing curly brace
    let mut found_close_curly: bool = false;
    let mut function_tokens: Vec<Token> = Vec::new();
    while cursor < tokens.len() {
        found_close_curly = advance_cursor(
            &mut cursor,
            &tokens,
            TokenType::Delimiter(Delimiter::CloseCurly),
        )
        .is_ok();
        if found_close_curly {
            break;
        }
        function_tokens.push(tokens[cursor - 1].clone());
    }
    if !found_close_curly {
        return Err(CompilerError::ParserError(
            "Unexpected EOF while parsing a function".to_string(),
        ));
    }

    Ok(Function {
        name,
        signature,
        tokens: function_tokens,
    })
}

pub fn peek_next_token(cursor: usize, tokens: &Vec<Token>, expected_type: TokenType) -> bool {
    cursor < tokens.len() - 1 && tokens[cursor + 1].typ != expected_type
}

pub fn advance_cursor(
    cursor: &mut usize,
    tokens: &Vec<Token>,
    expected_type: TokenType,
) -> Result<Token, CompilerError> {
    if *cursor >= tokens.len() {
        // TODO: Enhance error reporting
        return Err(CompilerError::ParserError(format!(
            "Expected TokenType {:?} but got nothing",
            expected_type
        )));
    }
    let token: &Token = &tokens[*cursor];
    *cursor += 1;
    if token.typ != expected_type {
        return Err(CompilerError::ParserError(format!(
            "Expected TokenType {:?} but got {:?}\n{:?}",
            expected_type, token.typ, token
        )));
    }
    Ok(token.clone())
}

fn parse_function_signature(
    cursor: &mut usize,
    tokens: &Vec<Token>,
) -> Result<Signature, CompilerError> {
    let mut return_type: Vec<DataType> = Vec::new();
    advance_cursor(cursor, tokens, TokenType::Delimiter(Delimiter::OpenParen))?;
    let parameters: Vec<Parameter> = parse_function_parameters(cursor, tokens)?;
    advance_cursor(cursor, tokens, TokenType::Delimiter(Delimiter::CloseParen))?;

    // -> indicates that function has a return value
    // { indicates that function does not return anything
    match &tokens[*cursor].typ {
        TokenType::Delimiter(delimiter) => match delimiter {
            Delimiter::OpenCurly => {}
            Delimiter::Arrow => {
                advance_cursor(cursor, tokens, TokenType::Delimiter(delimiter.clone()))?;
                return_type.push(datatype_from_string(
                    &advance_cursor(cursor, tokens, TokenType::Identifier)?.value,
                ))
            }
            _ => {}
        },
        _ => {
            return Err(CompilerError::ParserError(format!(
                "Expected '->' or '{{' but got '{}'",
                tokens[*cursor].value
            )))
        }
    }
    Ok(Signature {
        parameters,
        return_type,
    })
}

fn parse_function_parameters(
    cursor: &mut usize,
    tokens: &Vec<Token>,
) -> Result<Vec<Parameter>, CompilerError> {
    let mut parameters: Vec<Parameter> = Vec::new();

    loop {
        if *cursor >= tokens.len() {
            return Err(CompilerError::ParserError(
                "Unexpected EOF while parsing function parameters".to_string(),
            ));
        }
        if tokens[*cursor].typ == TokenType::Delimiter(Delimiter::CloseParen) {
            break;
        }
        let name: String = advance_cursor(cursor, tokens, TokenType::Identifier)?.value;
        advance_cursor(cursor, tokens, TokenType::Delimiter(Delimiter::Colon))?;
        let typ =
            datatype_from_string(&advance_cursor(cursor, tokens, TokenType::Identifier)?.value);
        parameters.push(Parameter {
            name,
            typ: Some(typ),
        });
        if tokens[*cursor].typ != TokenType::Delimiter(Delimiter::Comma) {
            break;
        }
        *cursor += 1;
    }
    Ok(parameters)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FOLDER: &str = "tests";

    #[test]
    #[should_panic(expected = "The 'main' function is not defined")]
    fn program_without_main_function() {
        let tokens: Vec<Token> =
            tokenize_code_file(&format!("{TEST_FOLDER}/program_without_main_function.rot"))
                .unwrap();
        parse_functions(tokens).unwrap();
    }

    #[test]
    fn parse_function_parameters() {
        let tokens: Vec<Token> =
            tokenize_code_file(&format!("{TEST_FOLDER}/parse_function_parameters.rot")).unwrap();
        let functions: Vec<Function> = parse_functions(tokens).unwrap();
        assert_eq!(functions.len(), 1);
        let parameters: &Vec<Parameter> = &functions.first().unwrap().signature.parameters;
        assert_eq!(parameters.len(), 3);
        assert_eq!(parameters[0].name, "a".to_string());
        assert_eq!(parameters[1].name, "b".to_string());
        assert_eq!(parameters[2].name, "c".to_string());
        assert_eq!(parameters[0].typ, DataType::Boolean);
        assert_eq!(parameters[1].typ, DataType::Character);
        assert_eq!(
            parameters[2].typ,
            DataType::Custom("CustomType".to_string())
        );
    }
}
