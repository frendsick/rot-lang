use crate::class::{
    program::Program,
    statement::{Statement, StatementType},
    token::{Token, TokenType, Delimiter},
};

pub fn generate_ast(tokens: &Vec<Token>) -> Program {
    let mut statements: Vec<Statement> = Vec::new();
    let mut cursor: usize = 0;
    while cursor < tokens.len() {
        statements.push(get_next_statement(&tokens, &mut cursor));
        cursor += 1;
    }
    return Program { statements };
}

pub fn get_next_statement(tokens: &Vec<Token>, cursor: &mut usize) -> Statement {
    let token: &Token = tokens.get(*cursor).unwrap();
    match token.typ {
        TokenType::Delimiter(Delimiter::SemiColon) => no_operation_statement(),
        _ => todo!("Parsing statement for {:?}", token),
    }
}

fn no_operation_statement() -> Statement {
    Statement {
        typ: StatementType::NoOperation,
        value: None,
        expression: None,
        statements: None,
    }
}
