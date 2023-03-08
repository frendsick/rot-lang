use super::signature::Signature;
use super::statement::Statement;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub signature: Signature,
    pub statement: Statement,
}
