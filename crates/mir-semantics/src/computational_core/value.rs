use super::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int64(i64),
    Bool(bool),
    Text(String),
    VecInt64(Vec<i64>),
    Vec3 { x: i64, y: i64, z: i64 },
}

impl Value {
    pub fn value_type(&self) -> Type {
        match self {
            Self::Int64(_) => Type::Int64,
            Self::Bool(_) => Type::Bool,
            Self::Text(_) => Type::Text,
            Self::VecInt64(_) => Type::VecInt64,
            Self::Vec3 { .. } => Type::Vec3,
        }
    }

    pub fn summary(&self) -> String {
        match self {
            Self::Int64(value) => format!("Int({value})"),
            Self::Bool(value) => format!("Bool({value})"),
            Self::Text(value) => format!("Text({value:?})"),
            Self::VecInt64(values) => format!("VecInt64({values:?})"),
            Self::Vec3 { x, y, z } => format!("Vec3({x}, {y}, {z})"),
        }
    }
}
