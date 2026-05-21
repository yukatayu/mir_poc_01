use super::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int64(i64),
    Text(String),
}

impl Value {
    pub fn value_type(&self) -> Type {
        match self {
            Self::Int64(_) => Type::Int64,
            Self::Text(_) => Type::Text,
        }
    }

    pub fn summary(&self) -> String {
        match self {
            Self::Int64(value) => format!("Int({value})"),
            Self::Text(value) => format!("Text({value:?})"),
        }
    }
}
