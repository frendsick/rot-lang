use crate::data_types::DataType;

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub typ: ExpressionType,
    pub value: Option<String>,
    pub expressions: Option<Vec<Expression>>,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionType {
    Binary,
    Enclosure,
    FunctionCall,
    Identifier,
    Literal(DataType),
    Unary,
}
