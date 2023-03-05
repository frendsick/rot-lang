use crate::class::op::{Op, OpType};
use crate::class::token::{Token, TokenType};

pub fn parse_ops(tokens: Vec<Token>) -> Vec<Op> {
    let mut ops: Vec<Op> = Vec::new();
    for (id, token) in tokens.iter().enumerate() {
        let typ: OpType = get_op_type(&token.typ);
        ops.push(Op {
            id,
            typ,
            token: token.clone(),
        });
    }
    ops
}

fn get_op_type(token_type: &TokenType) -> OpType {
    match token_type {
        TokenType::Literal(datatype) => OpType::Push(datatype.clone()),
        TokenType::Intrinsic(intrinsic) => OpType::Intrinsic(intrinsic.clone()),
        _ => todo!()
    }
}
