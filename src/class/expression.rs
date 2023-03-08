use crate::data_types::DataType;

use super::token::BinaryOperator;

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    pub typ: ExpressionType,
    pub value: Option<String>,
    pub expressions: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionType {
    Binary(BinaryOperator),
    Enclosure,
    FunctionCall,
    Indexing,
    Literal(Option<DataType>),
}
