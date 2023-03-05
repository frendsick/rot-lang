use crate::data_types::DataType;

#[derive(Debug, Clone)]
pub struct Signature {
    pub parameters: Vec<Parameter>,
    pub return_type: Vec<DataType>,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub typ: DataType,
}
