use crate::{
    class::{
        program::Program,
        statement::{Statement, StatementType},
        token::{Delimiter, Token, TokenType},
    },
    compiler::CompilerError,
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
        _ => todo!("Parsing statement for {:?}", token),
    }
}

fn no_operation_statement(
    tokens: &Vec<Token>,
    cursor: &mut usize,
) -> Result<Statement, CompilerError> {
    advance_cursor(cursor, tokens, TokenType::Delimiter(Delimiter::SemiColon))?;
    Ok(Statement {
        typ: StatementType::NoOperation,
        value: None,
        expression: None,
        statements: None,
    })
}

fn compound_statement(tokens: &Vec<Token>, cursor: &mut usize) -> Result<Statement, CompilerError> {
    advance_cursor(cursor, tokens, TokenType::Delimiter(Delimiter::OpenCurly))?;
    let mut statements: Vec<Statement> = Vec::new();
    while tokens[*cursor].typ != TokenType::Delimiter(Delimiter::CloseCurly) {
        statements.push(get_next_statement(tokens, cursor)?);
    }
    advance_cursor(cursor, tokens, TokenType::Delimiter(Delimiter::CloseCurly))?;
    Ok(Statement {
        typ: StatementType::Compound,
        value: None,
        expression: None,
        statements: Some(statements),
    })
}

fn advance_cursor(
    cursor: &mut usize,
    tokens: &Vec<Token>,
    expected_type: TokenType,
) -> Result<Token, CompilerError> {
    if *cursor >= tokens.len() {
        // TODO: Enhance error reporting
        return Err(CompilerError::ParserError(
            "Unexpected EOF while parsing".to_string(),
        ));
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize_code;
    use crate::{class::statement::StatementType, constant::TEST_FOLDER};

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
}
