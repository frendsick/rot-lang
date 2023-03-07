use super::statement::Statement;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}
