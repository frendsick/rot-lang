use crate::{data_types::DataType, intrinsics::{Intrinsic, Calculation, Comparison}};

use super::{token::Token, location::Location};

#[derive(Debug, Clone)]
pub struct Op {
    pub id: usize,
    pub typ: OpType,
    pub start_loc: Location,
    pub end_loc: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PushData {
    pub value: String,
    pub typ: DataType,
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
    FunctionCall,
    If,
    Intrinsic(Intrinsic),
    Push(PushData),
    Return,
    While,
}
