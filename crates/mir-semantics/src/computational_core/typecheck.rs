use std::collections::BTreeMap;

use super::{BinaryOp, ComputationalCoreError, ComputationalCoreErrorKind, Expr, Module, Type};

pub fn typecheck_module(module: &Module) -> Result<(), ComputationalCoreError> {
    for function in &module.functions {
        let env = BTreeMap::from([(function.parameter_name.clone(), function.input_type.clone())]);
        let body_type = typecheck_expr(&function.body, &env)?;
        if body_type != function.output_type {
            return Err(ComputationalCoreError {
                kind: ComputationalCoreErrorKind::TypeMismatch,
                detail: format!(
                    "function `{}` returns {}, expected {}",
                    function.function_id,
                    body_type.as_str(),
                    function.output_type.as_str()
                ),
            });
        }
    }
    Ok(())
}

fn typecheck_expr(
    expr: &Expr,
    env: &BTreeMap<String, Type>,
) -> Result<Type, ComputationalCoreError> {
    match expr {
        Expr::Int64(_) => Ok(Type::Int64),
        Expr::Text(_) => Ok(Type::Text),
        Expr::Variable(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| ComputationalCoreError {
                kind: ComputationalCoreErrorKind::UnboundVariable,
                detail: format!("unbound variable `{name}`"),
            }),
        Expr::Binary { op, left, right } => {
            let left_type = typecheck_expr(left, env)?;
            let right_type = typecheck_expr(right, env)?;
            match op {
                BinaryOp::Add if left_type == Type::Int64 && right_type == Type::Int64 => {
                    Ok(Type::Int64)
                }
                BinaryOp::Add => Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "Add expects Int64 + Int64, found {} + {}",
                        left_type.as_str(),
                        right_type.as_str()
                    ),
                }),
            }
        }
    }
}
