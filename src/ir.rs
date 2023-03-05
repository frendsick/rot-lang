use crate::class::op::{Op, OpType};
use crate::class::token::{Token, TokenType};

pub fn parse_ops(tokens: Vec<Token>) -> Vec<Op> {
    let mut ops: Vec<Op> = Vec::new();
    for (id, token) in tokens.iter().enumerate() {
        if let Some(typ) = get_mapped_op_type(&token.typ) {
            ops.push(Op {
                id,
                typ,
                token: token.clone(),
            });
            continue;
        }


    }
    dbg!(&ops);
    ops
}

/// Returns OpType that is mapped one to one with a TokenType
fn get_mapped_op_type(token_type: &TokenType) -> Option<OpType> {
    match token_type {
        TokenType::Calculation(calculation) => Some(OpType::Calculation(calculation.clone())),
        TokenType::Comparison(comparison) => Some(OpType::Comparison(comparison.clone())),
        TokenType::Intrinsic(intrinsic) => Some(OpType::Intrinsic(intrinsic.clone())),
        TokenType::Literal(datatype) => Some(OpType::Push(datatype.clone())),
        _ => None,
    }
}
