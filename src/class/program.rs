use super::function::Function;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Function>,
}
