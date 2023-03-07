use crate::{
    class::{
        program::Program,
        signature::{Parameter, Signature},
        statement::{Statement, StatementType},
        token::{Delimiter, Keyword, Token, TokenType},
    },
    compiler::CompilerError,
    data_types::{datatype_from_string, DataType},
};

pub fn generate_ast(tokens: &Vec<Token>) -> Result<Program, CompilerError> {
    let mut statements: Vec<Statement> = Vec::new();
    let mut cursor: usize = 0;
    while cursor < tokens.len() {
        let original_cursor: usize = cursor;
        statements.push(get_next_statement(&tokens, &mut cursor)?);
        // Prevent infinite loop
        assert_ne!(original_cursor, cursor);
    }
    return Ok(Program { statements });
}

pub fn get_next_statement(
    tokens: &Vec<Token>,
    cursor: &mut usize,
) -> Result<Statement, CompilerError> {
    let token: &Token = tokens.get(*cursor).unwrap();
    match token.typ {
        TokenType::Delimiter(Delimiter::SemiColon) => no_operation_statement(tokens, cursor),
        TokenType::Delimiter(Delimiter::OpenCurly) => compound_statement(tokens, cursor),
        TokenType::Keyword(Keyword::Fun) => function_statement(tokens, cursor),
        _ => todo!("Parsing statement for {:?}", token),
    }
}

fn no_operation_statement(
    tokens: &Vec<Token>,
    cursor: &mut usize,
) -> Result<Statement, CompilerError> {
    advance_cursor(cursor, tokens, &TokenType::Delimiter(Delimiter::SemiColon))?;
    Ok(Statement {
        typ: StatementType::NoOperation,
        value: None,
        expression: None,
        statements: None,
    })
}

fn compound_statement(tokens: &Vec<Token>, cursor: &mut usize) -> Result<Statement, CompilerError> {
    advance_cursor(cursor, tokens, &TokenType::Delimiter(Delimiter::OpenCurly))?;
    let mut statements: Vec<Statement> = Vec::new();
    while tokens[*cursor].typ != TokenType::Delimiter(Delimiter::CloseCurly) {
        statements.push(get_next_statement(tokens, cursor)?);
    }
    advance_cursor(cursor, tokens, &TokenType::Delimiter(Delimiter::CloseCurly))?;
    Ok(Statement {
        typ: StatementType::Compound,
        value: None,
        expression: None,
        statements: Some(statements),
    })
}

/// fun name(param1: int, param2: str) -> bool { }
fn function_statement(tokens: &Vec<Token>, cursor: &mut usize) -> Result<Statement, CompilerError> {
    advance_cursor(cursor, tokens, &TokenType::Keyword(Keyword::Fun))?;
    let function_name: String = advance_cursor(cursor, tokens, &TokenType::Identifier)?.value;
    advance_cursor(cursor, tokens, &TokenType::Delimiter(Delimiter::OpenParen))?;
    let parameters: Vec<Parameter> = parse_function_parameters(tokens, cursor)?;
    advance_cursor(cursor, tokens, &TokenType::Delimiter(Delimiter::CloseParen))?;
    // TODO: Return type
    let signature: Signature = Signature {
        parameters,
        return_type: None,
    };
    Ok(Statement {
        typ: StatementType::Function(signature), // TODO: Params, Return type
        value: Some(function_name),
        expression: None,
        statements: Some(vec![compound_statement(tokens, cursor)?]),
    })
}

fn parse_function_parameters(
    tokens: &Vec<Token>,
    cursor: &mut usize,
) -> Result<Vec<Parameter>, CompilerError> {
    let mut parameters: Vec<Parameter> = Vec::new();
    // Parse parameters until ')'
    while peek_cursor(
        *cursor,
        tokens,
        &TokenType::Delimiter(Delimiter::CloseParen),
    )
    .is_err()
    {
        parameters.push(parse_next_parameter(tokens, cursor)?);
    }
    Ok(parameters)
}

fn parse_next_parameter(
    tokens: &Vec<Token>,
    cursor: &mut usize,
) -> Result<Parameter, CompilerError> {
    let name: String = advance_cursor(cursor, tokens, &TokenType::Identifier)?.value;
    advance_cursor(cursor, tokens, &TokenType::Delimiter(Delimiter::Colon))?;
    let datatype_str: String = advance_cursor(cursor, tokens, &TokenType::Identifier)?.value;
    let typ: DataType = datatype_from_string(&datatype_str);
    // If the next token is a comma, there will be more parameters
    if peek_cursor(*cursor, tokens, &TokenType::Delimiter(Delimiter::Comma)).is_ok() {
        *cursor += 1;
    }
    Ok(Parameter::new(name, typ))
}

fn advance_cursor(
    cursor: &mut usize,
    tokens: &Vec<Token>,
    expected_type: &TokenType,
) -> Result<Token, CompilerError> {
    let token: Token = peek_cursor(cursor.to_owned(), tokens, &expected_type)?;
    *cursor += 1;
    Ok(token.clone())
}

fn peek_cursor(
    cursor: usize,
    tokens: &Vec<Token>,
    expected_type: &TokenType,
) -> Result<Token, CompilerError> {
    if cursor >= tokens.len() {
        // TODO: Enhance error reporting
        return Err(CompilerError::ParserError(
            "Unexpected EOF while parsing".to_string(),
        ));
    }
    let token: &Token = &tokens[cursor];
    if &token.typ != expected_type {
        return Err(CompilerError::ParserError(format!(
            "Expected TokenType {:?} but got {:?}\n{:?}",
            expected_type, token.typ, token
        )));
    }
    Ok(token.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::class::statement::StatementType;
    use crate::lexer::tokenize_code;

    #[test]
    fn parse_no_operation_statement() {
        let tokens: Vec<Token> = tokenize_code(";", None);
        let program: Program = generate_ast(&tokens).expect("Could not generate AST");
        assert_eq!(program.statements.len(), 1);
        assert_eq!(program.statements[0].typ, StatementType::NoOperation);
    }

    #[test]
    fn parse_compound_statement() {
        // Compound statement with inner compound statement
        let tokens: Vec<Token> = tokenize_code("{{ }}", None);
        let program: Program = generate_ast(&tokens).expect("Could not generate AST");
        assert_eq!(program.statements.len(), 1);
        let statement: &Statement = program.statements.first().unwrap();
        // Outer compound statement
        assert_eq!(statement.typ, StatementType::Compound);
        // Inner compound statement
        assert_eq!(
            statement.statements.as_ref().unwrap().first().unwrap().typ,
            StatementType::Compound
        );
    }

    #[test]
    fn parse_function_statement() {
        let tokens: Vec<Token> = tokenize_code("fun foo(a:int, b:str) { }", None);
        let program: Program = generate_ast(&tokens).expect("Could not generate AST");
        assert_eq!(program.statements.len(), 1);
        let parameters: Vec<Parameter> = vec![
            Parameter::new("a".to_string(), DataType::Integer),
            Parameter::new("b".to_string(), DataType::String),
        ];
        let return_type: Option<DataType> = None;
        let signature: Signature = Signature {
            parameters,
            return_type,
        };
        assert_eq!(
            program.statements[0].typ,
            StatementType::Function(signature)
        );
        // Function's Statement is always CompoundStatement
        assert_eq!(
            program.statements[0]
                .statements
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .typ,
            StatementType::Compound
        )
    }
}
