use crate::data_types::DataType;

use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub typ: StatementType,
    pub value: Option<String>,
    pub expression: Option<Expression>,
    pub statements: Option<Vec<Statement>>,
}

#[derive(Debug, PartialEq)]
pub enum StatementType {
    Assignment(DataType),
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
