use crate::data_types::DataType;

use super::token::BinaryOperator;

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub typ: ExpressionType,
    pub value: Option<String>,
    pub expressions: Option<Vec<Expression>>,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    Binary(BinaryOperator),
    Enclosure,
    FunctionCall,
    Identifier,
    Literal(Option<DataType>),
    Unary,
}
