use strum_macros::{EnumCount, EnumIter};

#[derive(Debug, Clone, PartialEq, EnumCount, EnumIter)]
pub enum DataType {
    Boolean,
    Character,
    Integer(ChunkSize),
    Pointer,
    String,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Default, EnumCount, EnumIter)]
pub enum ChunkSize {
    Byte,
    Word,
    Dword,
    #[default]
    Qword,
}

pub fn datatype_from_string(string: &str) -> DataType {
    match string {
        "bool"  => DataType::Boolean,
        "char"  => DataType::Character,
        "int"   => DataType::Integer(ChunkSize::Qword),
        "ptr"   => DataType::Pointer,
        "str"   => DataType::String,
        _       => DataType::Custom(string.to_string()),
    }
}
