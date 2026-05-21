#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int64,
    Text,
}

impl Type {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Int64 => "Int64",
            Self::Text => "Text",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Int64(i64),
    Text(String),
    Variable(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub function_id: String,
    pub parameter_name: String,
    pub input_type: Type,
    pub output_type: Type,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub module_id: String,
    pub functions: Vec<Function>,
}
