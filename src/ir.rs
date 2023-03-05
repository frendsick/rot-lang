use crate::class::op::{Op, OpType};
use crate::class::token::{Token, TokenType, Keyword};

pub fn parse_ops(tokens: Vec<Token>) -> Vec<Op> {
    let mut ops: Vec<Op> = Vec::new();
    let mut i: usize = 0;
    while i < tokens.len() {
        // Parse Ops that are mapped one to one with a Token
        if let Some(typ) = get_mapped_op_type(&tokens[i].typ) {
            ops.push(Op {
                id: ops.len(),
                typ,
                token: tokens[i].clone(),
            });
            i += 1;
            continue;
        }
        // TODO: Parse non-mapped keywords
        i += 1;
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
        TokenType::Keyword(keyword) => get_keyword_op_type(keyword),
        _ => None,
    }
}

fn get_keyword_op_type(keyword: &Keyword) -> Option<OpType> {
    match keyword {
        Keyword::Break => Some(OpType::Break),
        Keyword::Continue => Some(OpType::Continue),
        Keyword::Do => Some(OpType::Do),
        Keyword::Done => Some(OpType::Done),
        Keyword::Elif => Some(OpType::Elif),
        Keyword::Else => Some(OpType::Else),
        Keyword::Endif => Some(OpType::Endif),
        Keyword::If => Some(OpType::If),
        Keyword::Return => Some(OpType::FunctionReturn),
        Keyword::While => Some(OpType::While),
        _ => None,
    }
}
