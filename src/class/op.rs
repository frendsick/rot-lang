use crate::{
    data_types::DataType,
    intrinsics::{Calculation, Comparison, Intrinsic},
};

use super::{location::Location, signature::Parameter, token::Token};

#[derive(Debug, Clone)]
pub struct Op {
    pub id: usize,
    pub typ: OpType,
    pub start_loc: Location,
    pub end_loc: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpType {
    Break,
    Cast(DataType),
    Calculation(Calculation),
    Comparison(Comparison),
    Continue,
    Do,
    Done,
    Elif,
    Else,
    Endif,
    FunctionCall(Vec<Parameter>),
    If,
    Intrinsic(Intrinsic),
    Push(Token),
    Return,
    While,
}
