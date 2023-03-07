use crate::{
    class::{
        expression::{Expression, ExpressionType},
        program::Program,
        signature::{Parameter, Signature},
        statement::{Conditional, Statement, StatementType},
        token::{Delimiter, Keyword, Token, TokenType},
    },
    compiler::CompilerError,
    constant::EXPRESSION_DELIMITERS,
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
        TokenType::Keyword(Keyword::Return) => return_statement(tokens, cursor),
        TokenType::Keyword(Keyword::If) => block_statement(tokens, cursor),
        TokenType::Keyword(Keyword::Elif) => block_statement(tokens, cursor),
        TokenType::Keyword(Keyword::Else) => block_statement(tokens, cursor),
        TokenType::Keyword(Keyword::While) => block_statement(tokens, cursor),
        _ => expression_statement(tokens, cursor),
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
    let signature: Signature = Signature {
        parameters,
        return_type: parse_return_type(tokens, cursor)?,
    };
    Ok(Statement {
        typ: StatementType::Function(signature), // TODO: Params, Return type
        value: Some(function_name),
        expression: None,
        statements: Some(vec![compound_statement(tokens, cursor)?]),
    })
}

fn return_statement(tokens: &Vec<Token>, cursor: &mut usize) -> Result<Statement, CompilerError> {
    // return <expr>;
    todo!()
}

fn block_statement(tokens: &Vec<Token>, cursor: &mut usize) -> Result<Statement, CompilerError> {
    let typ: StatementType = block_statement_type_from_str(&tokens[*cursor].value);
    *cursor += 1; // Go past initial Keyword
    advance_cursor(cursor, tokens, &TokenType::Delimiter(Delimiter::OpenParen))?;
    let expression: Option<Expression> = parse_expression(tokens, cursor)?;
    advance_cursor(cursor, tokens, &TokenType::Delimiter(Delimiter::CloseParen))?;
    let statement: Statement = compound_statement(tokens, cursor)?;
    Ok(Statement {
        typ,
        value: None,
        expression,
        statements: Some(vec![statement]),
    })
}

fn parse_expression(
    tokens: &Vec<Token>,
    cursor: &mut usize,
) -> Result<Option<Expression>, CompilerError> {
    // No expression
    if EXPRESSION_DELIMITERS.contains(&peek_cursor(*cursor, tokens)?.value.as_str()) {
        return Ok(None);
    }

    // Literal expression
    if EXPRESSION_DELIMITERS.contains(&peek_cursor(*cursor + 1, tokens)?.value.as_str()) {
        return Ok(Some(literal_expression(tokens, cursor)?));
    }
    *cursor += 1;
    // TODO: Parse other expressions
    Ok(None)
}

fn literal_expression(
    tokens: &Vec<Token>,
    cursor: &mut usize,
) -> Result<Expression, CompilerError> {
    let index: usize = *cursor;
    *cursor += 1;
    match &tokens[index].typ {
        TokenType::Literal(data_type) => Ok(Expression {
            typ: ExpressionType::Literal(Some(data_type.clone())),
            value: Some(tokens[index].value.clone()),
            expressions: None,
        }),
        TokenType::Identifier => Ok(Expression {
            typ: ExpressionType::Literal(None),
            value: Some(tokens[index].value.clone()),
            expressions: None,
        }),
        _ => panic!("Unknown literal token '{}'", tokens[*cursor].value),
    }
}

fn block_statement_type_from_str(block_type_str: &str) -> StatementType {
    match block_type_str {
        "if" => StatementType::Conditional(Conditional::If),
        "elif" => StatementType::Conditional(Conditional::Elif),
        "else" => StatementType::Conditional(Conditional::Else),
        "while" => StatementType::Loop,
        _ => panic!("Unknown block type '{}'", block_type_str),
    }
}

fn expression_statement(
    tokens: &Vec<Token>,
    cursor: &mut usize,
) -> Result<Statement, CompilerError> {
    todo!()
}

fn parse_function_parameters(
    tokens: &Vec<Token>,
    cursor: &mut usize,
) -> Result<Vec<Parameter>, CompilerError> {
    let mut parameters: Vec<Parameter> = Vec::new();
    // Parse parameters until ')'
    while check_cursor(
        *cursor,
        tokens,
        &TokenType::Delimiter(Delimiter::CloseParen),
    )
    .is_err()
    {
        parameters.push(parse_next_parameter(tokens, cursor)?);
        // Parameters are separated by commas
        // There can be a comma after the last parameter
        // Example: fun foo(a: int, b: int,) { }
        if check_cursor(*cursor, tokens, &TokenType::Delimiter(Delimiter::Comma)).is_ok() {
            *cursor += 1;
        }
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
    Ok(Parameter::new(name, typ))
}

fn parse_return_type(
    tokens: &Vec<Token>,
    cursor: &mut usize,
) -> Result<Option<DataType>, CompilerError> {
    if check_cursor(*cursor, tokens, &TokenType::Delimiter(Delimiter::OpenCurly)).is_ok() {
        return Ok(None);
    }
    advance_cursor(cursor, tokens, &TokenType::Delimiter(Delimiter::Arrow))?;
    Ok(Some(datatype_from_string(
        &advance_cursor(cursor, tokens, &TokenType::Identifier)?.value,
    )))
}

fn advance_cursor(
    cursor: &mut usize,
    tokens: &Vec<Token>,
    expected_type: &TokenType,
) -> Result<Token, CompilerError> {
    let token: Token = check_cursor(cursor.to_owned(), tokens, &expected_type)?;
    *cursor += 1;
    Ok(token.clone())
}

fn peek_cursor(cursor: usize, tokens: &Vec<Token>) -> Result<Token, CompilerError> {
    if cursor >= tokens.len() {
        // TODO: Enhance error reporting
        return Err(CompilerError::ParserError(
            "Unexpected EOF while parsing".to_string(),
        ));
    }
    Ok(tokens[cursor].clone())
}

fn check_cursor(
    cursor: usize,
    tokens: &Vec<Token>,
    expected_type: &TokenType,
) -> Result<Token, CompilerError> {
    let token: Token = peek_cursor(cursor, tokens)?;
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
        // The comma after last parameter is optional
        let tokens: Vec<Token> = tokenize_code("fun foo(a:int, b:str) -> bool { }", None);
        let tokens2: Vec<Token> = tokenize_code("fun foo(a:int, b:str,) -> bool { }", None);
        let program: Program = generate_ast(&tokens).expect("Could not generate AST");
        let program2: Program = generate_ast(&tokens2).expect("Could not generate AST");
        // Only one function is parsed
        assert_eq!(program.statements.len(), 1);
        // Comma after the last parameter is optional
        assert_eq!(program, program2);
        let parameters: Vec<Parameter> = vec![
            Parameter::new("a".to_string(), DataType::Integer),
            Parameter::new("b".to_string(), DataType::String),
        ];
        let return_type: Option<DataType> = Some(DataType::Boolean);
        let signature: Signature = Signature {
            parameters,
            return_type,
        };
        // Function signature is parsed correctly
        assert_eq!(
            program.statements[0].typ,
            StatementType::Function(signature)
        );
        // Function's Statement is always CompoundStatement
        first_statement_is_compound(&program);
    }

    #[test]
    fn parse_conditional_statement() {
        let tokens: Vec<Token> = tokenize_code("if() { }", None);
        let program: Program = generate_ast(&tokens).expect("Could not generate AST");
        // Conditional statement is one statement
        assert_eq!(program.statements.len(), 1);
        // ConditionalStatement's Statement is always CompoundStatement
        first_statement_is_compound(&program);
    }

    fn first_statement_is_compound(program: &Program) {
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
