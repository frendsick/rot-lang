use crate::class::op::{Op, OpType};
use crate::class::signature::Parameter;
use crate::class::token::{Delimiter, Keyword, Token, TokenType};
use crate::compiler::{advance_cursor, peek_next_token};
use crate::data_types::datatype_from_string;

pub fn parse_ops(tokens: &Vec<Token>) -> Vec<Op> {
    let mut ops: Vec<Op> = Vec::new();
    let mut cursor: usize = 0;
    while cursor < tokens.len() {
        let start_loc = tokens[cursor].start_loc.clone();
        // Parse Ops that are mapped one to one with a Token
        if let Some(typ) = get_mapped_op_type(&tokens[cursor]) {
            ops.push(Op {
                id: ops.len(),
                typ,
                start_loc,
                end_loc: tokens[cursor].end_loc.clone(),
            });
            cursor += 1;
            continue;
        }
        // Parse non-mapped Ops
        if let Some(typ) = get_non_mapped_op_type(&mut cursor, &tokens) {
            ops.push(Op {
                id: ops.len(),
                typ,
                start_loc,
                end_loc: tokens[cursor].end_loc.clone(),
            });
        }
        cursor += 1;
    }
    ops
}

/// Returns OpType that is mapped one to one with a TokenType
fn get_mapped_op_type(token: &Token) -> Option<OpType> {
    match &token.typ {
        TokenType::Calculation(calculation) => Some(OpType::Calculation(calculation.clone())),
        TokenType::Comparison(comparison) => Some(OpType::Comparison(comparison.clone())),
        // TODO: Op should not start with delimiter
        // Parse this when tokens are already parsed to functions
        TokenType::Delimiter(_) => None,
        TokenType::Intrinsic(intrinsic) => Some(OpType::Intrinsic(intrinsic.clone())),
        TokenType::Literal(_) => Some(OpType::Push(token.clone())),
        TokenType::Keyword(keyword) => {
            if let Some(op_type) = get_keyword_op_type(&keyword) {
                return Some(op_type);
            }
            return None;
        }
        TokenType::Identifier => None,
        TokenType::None => None,
    }
}

fn get_non_mapped_op_type(cursor: &mut usize, tokens: &Vec<Token>) -> Option<OpType> {
    match tokens[*cursor].typ {
        TokenType::Keyword(Keyword::Cast) => Some(parse_cast_op(cursor, tokens)),
        TokenType::Identifier => {
            let ident_token = &tokens[*cursor];
            // Identifier is a FunctionCall if it is followed by OpenParen
            *cursor += 1;
            if advance_cursor(cursor, tokens, TokenType::Delimiter(Delimiter::OpenParen)).is_ok() {
                let mut parameters: Vec<Parameter> = Vec::new();
                if peek_next_token(*cursor, tokens, TokenType::Delimiter(Delimiter::CloseParen)) {
                    return Some(OpType::FunctionCall(parameters));
                }
                loop {
                    parameters.push(Parameter {
                        name: tokens[*cursor].value.clone(),
                        typ: None,
                    });
                    if peek_next_token(*cursor, tokens, TokenType::Delimiter(Delimiter::CloseParen))
                    {
                        return Some(OpType::FunctionCall(parameters));
                    }
                    *cursor += 1;
                    advance_cursor(cursor, tokens, TokenType::Delimiter(Delimiter::Comma)).unwrap();
                }
            } else {
                *cursor -= 2;
            }
            Some(OpType::Push(ident_token.clone()))
        }
        _ => None,
    }
}

fn parse_cast_op(cursor: &mut usize, tokens: &Vec<Token>) -> OpType {
    advance_cursor(cursor, tokens, TokenType::Keyword(Keyword::Cast)).unwrap();
    advance_cursor(cursor, tokens, TokenType::Delimiter(Delimiter::OpenParen)).unwrap();
    let type_str = advance_cursor(cursor, tokens, TokenType::Identifier)
        .unwrap()
        .value;
    // Verify that the next token is closing parenthesis
    advance_cursor(cursor, tokens, TokenType::Delimiter(Delimiter::CloseParen)).unwrap();
    *cursor -= 1;
    OpType::Cast(datatype_from_string(&type_str))
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
        Keyword::Return => Some(OpType::Return),
        Keyword::While => Some(OpType::While),
        _ => None,
    }
}
