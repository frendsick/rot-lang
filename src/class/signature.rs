use crate::data_types::DataType;

#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    pub parameters: Vec<Parameter>,
    pub return_type: Option<DataType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub typ: DataType,
}

impl Parameter {
    pub fn new(name: String, typ: DataType) -> Self {
        Self {
            name,
            typ,
        }
    }
}
