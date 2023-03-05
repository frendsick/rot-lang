use super::signature::Signature;
use super::token::Token;

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub signature: Signature,
    pub tokens: Vec<Token>,
}

pub fn function_defined(name: &str, functions: &Vec<Function>) -> bool {
    functions.iter().any(|function| function.name == name)
}
