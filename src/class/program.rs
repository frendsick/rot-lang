use super::function::Function;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}
