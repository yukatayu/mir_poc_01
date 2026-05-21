mod ast;
mod eval;
mod typecheck;
mod value;

use std::fmt;

pub use ast::{BinaryOp, Expr, Function, Module, Type};
pub use eval::eval_function;
pub use typecheck::typecheck_module;
pub use value::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputationalCoreErrorKind {
    UnknownModule,
    UnknownFunction,
    InvalidArity,
    TypeMismatch,
    UnboundVariable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputationalCoreError {
    pub kind: ComputationalCoreErrorKind,
    pub detail: String,
}

impl fmt::Display for ComputationalCoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.detail)
    }
}

impl std::error::Error for ComputationalCoreError {}

pub fn add_one_module() -> Module {
    Module {
        module_id: "Computational.AddOne".to_string(),
        functions: vec![Function {
            function_id: "add_one".to_string(),
            parameter_name: "value".to_string(),
            input_type: Type::Int64,
            output_type: Type::Int64,
            body: Expr::Binary {
                op: BinaryOp::Add,
                left: Box::new(Expr::Variable("value".to_string())),
                right: Box::new(Expr::Int64(1)),
            },
        }],
    }
}

pub fn declared_module(module_id: &str) -> Result<Module, ComputationalCoreError> {
    match module_id {
        "Computational.AddOne" => Ok(add_one_module()),
        _ => Err(ComputationalCoreError {
            kind: ComputationalCoreErrorKind::UnknownModule,
            detail: format!("unknown computational module `{module_id}`"),
        }),
    }
}
