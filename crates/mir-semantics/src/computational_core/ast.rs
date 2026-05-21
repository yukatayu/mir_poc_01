#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int64,
    Bool,
    Text,
    VecInt64,
    Vec3,
}

impl Type {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Int64 => "Int64",
            Self::Bool => "Bool",
            Self::Text => "Text",
            Self::VecInt64 => "Vec[Int64]",
            Self::Vec3 => "Vec3",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    LessThan,
    LessEqual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Int64(i64),
    Bool(bool),
    Text(String),
    Variable(String),
    Binary {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    ArrayLiteral(Vec<Expr>),
    ArrayIndex {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    RecordVec3 {
        x: Box<Expr>,
        y: Box<Expr>,
        z: Box<Expr>,
    },
    FieldAccess {
        base: Box<Expr>,
        field: String,
    },
    Call {
        function_id: String,
        argument: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        ty: Type,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    Return(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub function_id: String,
    pub parameter_name: String,
    pub input_type: Type,
    pub output_type: Type,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub module_id: String,
    pub imports: Vec<String>,
    pub functions: Vec<Function>,
}
