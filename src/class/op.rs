use crate::{data_types::DataType, intrinsics::Intrinsic};

use super::token::Token;

#[derive(Debug, Clone)]
pub struct Op {
    pub id: usize,
    pub typ: OpType,
    pub token: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpType {
    Break,
    Cast(DataType),
    Continue,
    Do,
    Done,
    Elif,
    Else,
    End,
    Endif,
    FunctionCall,
    FunctionReturn,
    If,
    In,
    Intrinsic(Intrinsic),
    Push(DataType),
    Return,
    While,
}
