use strum_macros::{EnumCount, EnumIter};

#[derive(Debug, Clone, PartialEq, EnumCount, EnumIter)]
pub enum DataType {
    Boolean,
    Character,
    Integer,
    String,
    Custom(String),
}

pub fn datatype_from_string(string: &str) -> DataType {
    match string {
        "bool"  => DataType::Boolean,
        "char"  => DataType::Character,
        "int"   => DataType::Integer,
        "str"   => DataType::String,
        _       => DataType::Custom(string.to_string()),
    }
}
