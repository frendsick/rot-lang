use super::{expression::Expression, token::Token};

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub typ: StatementType,
    pub value: Option<String>,
    pub expression: Option<Expression>,
    pub statements: Option<Vec<Statement>>,
}

#[derive(Debug, PartialEq)]
pub enum StatementType {
    Assignment(Token),
    Compound,
    Conditional(Conditional),
    Expression,
    Loop,
    NoOperation,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum Conditional {
    If,
    Elif,
    Else,
}
