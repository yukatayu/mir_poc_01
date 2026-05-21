use std::collections::BTreeMap;

use super::{
    BinaryOp, ComputationalCoreError, ComputationalCoreErrorKind, Expr, Module, Value,
    typecheck_module,
};

pub fn eval_function(
    module: &Module,
    function_id: &str,
    args: Vec<Value>,
) -> Result<Value, ComputationalCoreError> {
    typecheck_module(module)?;

    let function = module
        .functions
        .iter()
        .find(|function| function.function_id == function_id)
        .ok_or_else(|| ComputationalCoreError {
            kind: ComputationalCoreErrorKind::UnknownFunction,
            detail: format!(
                "module `{}` does not define function `{function_id}`",
                module.module_id
            ),
        })?;

    if args.len() != 1 {
        return Err(ComputationalCoreError {
            kind: ComputationalCoreErrorKind::InvalidArity,
            detail: format!(
                "function `{}` expects 1 argument, found {}",
                function.function_id,
                args.len()
            ),
        });
    }

    let argument = args.into_iter().next().expect("argument length checked");
    let argument_type = argument.value_type();
    if argument_type != function.input_type {
        return Err(ComputationalCoreError {
            kind: ComputationalCoreErrorKind::TypeMismatch,
            detail: format!(
                "function `{}` expects {}, found {}",
                function.function_id,
                function.input_type.as_str(),
                argument_type.as_str()
            ),
        });
    }

    let env = BTreeMap::from([(function.parameter_name.clone(), argument)]);
    eval_expr(&function.body, &env)
}

fn eval_expr(expr: &Expr, env: &BTreeMap<String, Value>) -> Result<Value, ComputationalCoreError> {
    match expr {
        Expr::Int64(value) => Ok(Value::Int64(*value)),
        Expr::Text(value) => Ok(Value::Text(value.clone())),
        Expr::Variable(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| ComputationalCoreError {
                kind: ComputationalCoreErrorKind::UnboundVariable,
                detail: format!("unbound variable `{name}`"),
            }),
        Expr::Binary { op, left, right } => {
            let left_value = eval_expr(left, env)?;
            let right_value = eval_expr(right, env)?;
            match (op, left_value, right_value) {
                (BinaryOp::Add, Value::Int64(left), Value::Int64(right)) => {
                    Ok(Value::Int64(left + right))
                }
                (BinaryOp::Add, left, right) => Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "Add expects Int64 + Int64, found {} + {}",
                        left.value_type().as_str(),
                        right.value_type().as_str()
                    ),
                }),
            }
        }
    }
}
