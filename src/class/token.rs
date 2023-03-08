use phf::phf_ordered_map;
use strum_macros::{EnumCount, EnumIter};

use crate::data_types::DataType;

use super::location::Location;

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub typ: TokenType,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    BinaryOperator(BinaryOperator),
    Delimiter(Delimiter),
    Identifier,
    Literal(DataType),
    Keyword(Keyword),
    None,
}

#[derive(Debug, Clone, PartialEq, EnumCount, EnumIter)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Division,
    Multiplication,
    Assignment,
    Equals,
    GreaterOrEqual,
    GreaterThan,
    LessOrEqual,
    LessThan,
    NotEquals,
}

impl Into<Option<BinaryOperator>> for TokenType {
    fn into(self) -> Option<BinaryOperator> {
        match self {
            TokenType::BinaryOperator(operator) => Some(operator),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, EnumCount, EnumIter)]
pub enum Delimiter {
    Arrow,
    Point,
    Comma,
    Colon,
    SemiColon,
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
}

#[derive(Debug, Clone, PartialEq, EnumCount, EnumIter)]
pub enum Keyword {
    Break,
    Continue,
    Elif,
    Else,
    Endif,
    Fun,
    If,
    Return,
    While,
}

pub const TOKEN_REGEXES: phf::OrderedMap<&str, TokenType> = phf_ordered_map!(
    r"^\s+"             => TokenType::None,

    // Comments
    r"^//.*"            => TokenType::None, // Single-line comment
    r"^/\*[\s\S]*?\*/"  => TokenType::None, // Multi-line comment

    // Literals
    r"(?i)^true"        => TokenType::Literal(DataType::Boolean),
    r"(?i)^false"       => TokenType::Literal(DataType::Boolean),
    r"^'[^']'"          => TokenType::Literal(DataType::Character),
    r"^\d+"             => TokenType::Literal(DataType::Integer),
    r#"^"[^"]*""#       => TokenType::Literal(DataType::String),

    // Keywords
    r"^break"           => TokenType::Keyword(Keyword::Break),
    r"^continue"        => TokenType::Keyword(Keyword::Continue),
    r"^elif"            => TokenType::Keyword(Keyword::Elif),
    r"^else"            => TokenType::Keyword(Keyword::Else),
    r"^endif"           => TokenType::Keyword(Keyword::Endif),
    r"^fun"             => TokenType::Keyword(Keyword::Fun),
    r"^if"              => TokenType::Keyword(Keyword::If),
    r"^return"          => TokenType::Keyword(Keyword::Return),
    r"^while"           => TokenType::Keyword(Keyword::While),

    // Delimiters
    r"^\("              => TokenType::Delimiter(Delimiter::OpenParen),
    r"^\)"              => TokenType::Delimiter(Delimiter::CloseParen),
    r"^\["              => TokenType::Delimiter(Delimiter::OpenSquare),
    r"^\]"              => TokenType::Delimiter(Delimiter::CloseSquare),
    r"^\{"              => TokenType::Delimiter(Delimiter::OpenCurly),
    r"^\}"              => TokenType::Delimiter(Delimiter::CloseCurly),
    r"^->"              => TokenType::Delimiter(Delimiter::Arrow),
    r"^\."              => TokenType::Delimiter(Delimiter::Point),
    r"^,"               => TokenType::Delimiter(Delimiter::Comma),
    r"^:"               => TokenType::Delimiter(Delimiter::Colon),
    r"^;"               => TokenType::Delimiter(Delimiter::SemiColon),

    // Binary Operators
    r"^=="              => TokenType::BinaryOperator(BinaryOperator::Equals),
    r"^>="              => TokenType::BinaryOperator(BinaryOperator::GreaterOrEqual),
    r"^>"               => TokenType::BinaryOperator(BinaryOperator::GreaterThan),
    r"^<="              => TokenType::BinaryOperator(BinaryOperator::LessOrEqual),
    r"^<"               => TokenType::BinaryOperator(BinaryOperator::LessThan),
    r"^!="              => TokenType::BinaryOperator(BinaryOperator::NotEquals),
    r"^="               => TokenType::BinaryOperator(BinaryOperator::Assignment),
    r"^\+"              => TokenType::BinaryOperator(BinaryOperator::Addition),
    r"^/"               => TokenType::BinaryOperator(BinaryOperator::Division),
    r"^\*"              => TokenType::BinaryOperator(BinaryOperator::Multiplication),
    r"^-"               => TokenType::BinaryOperator(BinaryOperator::Subtraction),

    // Identifier - Named value representing some value or other entity
    r"^[a-zA-Z_$][a-zA-Z_$0-9]*" => TokenType::Identifier,
);
