use mir_ast::textual_alpha::{AstContractClauseKind, SourceSpan, TextualMirDiagnostic};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1Obligation {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1CheckReport {
    pub accepted: bool,
    pub module: Option<TypedMirModule>,
    pub accepted_obligations: Vec<FullSystemV1Obligation>,
    pub residual_obligations: Vec<FullSystemV1Obligation>,
    pub diagnostics: Vec<TextualMirDiagnostic>,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedMirModule {
    pub module_path: String,
    pub imports: Vec<TypedMirImport>,
    pub capabilities: Vec<TypedCapabilityDecl>,
    pub records: Vec<TypedRecordType>,
    pub effects: Vec<TypedEffectDecl>,
    pub functions: Vec<TypedFunction>,
    pub transitions: Vec<TypedTransition>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedMirImport {
    pub module_path: String,
    pub resolved_path: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedCapabilityDecl {
    pub capability_name: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedRecordType {
    pub record_name: String,
    pub fields: Vec<TypedRecordField>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedRecordField {
    pub field_name: String,
    pub field_type: TypedType,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedEffectDecl {
    pub effect_name: String,
    pub parameters: Vec<TypedParam>,
    pub required_capabilities: Vec<String>,
    pub output: Option<TypedEffectOutput>,
    pub failure_row: Vec<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedEffectOutput {
    pub name: String,
    pub output_type: TypedType,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedParam {
    pub name: String,
    pub param_type: TypedType,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedFunction {
    pub function_name: String,
    pub parameter: TypedParam,
    pub output_type: TypedType,
    pub body: Vec<TypedStmt>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedTransition {
    pub transition_name: String,
    pub place_ref: String,
    pub required_capabilities: Vec<String>,
    pub body: Vec<TypedStmt>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypedStmt {
    Let {
        name: String,
        mutable: bool,
        ty: TypedType,
        value: TypedExpr,
        span: SourceSpan,
    },
    Assign {
        name: String,
        value: TypedExpr,
        span: SourceSpan,
    },
    If {
        condition: TypedExpr,
        then_body: Vec<TypedStmt>,
        else_body: Vec<TypedStmt>,
        span: SourceSpan,
    },
    While {
        condition: TypedExpr,
        body: Vec<TypedStmt>,
        span: SourceSpan,
    },
    For {
        binding: String,
        start: TypedExpr,
        end: TypedExpr,
        body: Vec<TypedStmt>,
        span: SourceSpan,
    },
    Bind {
        name: String,
        binding_type: TypedType,
        value: TypedBindValue,
        contract_clauses: Vec<TypedContractClause>,
        span: SourceSpan,
    },
    Perform {
        call: TypedPerformCall,
        contract_clauses: Vec<TypedContractClause>,
        span: SourceSpan,
    },
    Return {
        value: TypedExpr,
        span: SourceSpan,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypedBindValue {
    Expr(TypedExpr),
    Perform(TypedPerformCall),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedPerformCall {
    pub effect_name: String,
    pub arguments: Vec<TypedExpr>,
    pub boundary_ref: String,
    pub required_capabilities: Vec<String>,
    pub output_type: Option<TypedType>,
    pub failure_row: Vec<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedContractClause {
    pub kind: AstContractClauseKind,
    pub condition: TypedExpr,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedExpr {
    pub kind: TypedExprKind,
    pub ty: TypedType,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypedExprKind {
    IntLiteral(i64),
    FloatLiteral(String),
    BoolLiteral(bool),
    TextLiteral(String),
    Variable(String),
    ArrayLiteral(Vec<TypedExpr>),
    RecordConstruct {
        record_name: String,
        fields: Vec<TypedRecordConstructField>,
    },
    Call {
        function_name: String,
        module_path: String,
        arguments: Vec<TypedExpr>,
    },
    Index {
        base: Box<TypedExpr>,
        index: Box<TypedExpr>,
    },
    FieldAccess {
        base: Box<TypedExpr>,
        field_name: String,
    },
    Unary {
        op: TypedUnaryOp,
        expr: Box<TypedExpr>,
    },
    Binary {
        op: TypedBinaryOp,
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedRecordConstructField {
    pub field_name: String,
    pub value: TypedExpr,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypedType {
    Bool,
    Int64,
    UInt64,
    Float64,
    Text,
    Unit,
    Named(String),
    FixedArray {
        element: Box<TypedType>,
        length: usize,
    },
    Error,
}

impl TypedType {
    pub fn display_name(&self) -> String {
        match self {
            Self::Bool => "Bool".to_string(),
            Self::Int64 => "Int64".to_string(),
            Self::UInt64 => "UInt64".to_string(),
            Self::Float64 => "Float64".to_string(),
            Self::Text => "Text".to_string(),
            Self::Unit => "Unit".to_string(),
            Self::Named(name) => name.clone(),
            Self::FixedArray { element, length } => {
                format!("[{}; {length}]", element.display_name())
            }
            Self::Error => "<error>".to_string(),
        }
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypedUnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypedBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    And,
    Or,
}
