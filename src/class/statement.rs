use crate::data_types::DataType;

use super::{expression::Expression, signature::Signature};

#[derive(Debug, PartialEq)]
pub struct Statement {
    pub typ: StatementType,
    pub value: Option<String>,
    pub expression: Option<Expression>,
    pub statements: Option<Vec<Statement>>,
}

#[derive(Debug, PartialEq)]
pub enum StatementType {
    Compound,
    Conditional,
    Expression,
    Function(Signature),
    Loop,
    NoOperation,
    Return,
    Variable(DataType),
}
