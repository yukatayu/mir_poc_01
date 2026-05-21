use std::collections::{BTreeMap, BTreeSet};

use super::{
    BinaryOp, ComputationalCoreError, ComputationalCoreErrorKind, Expr, Function, Module, Stmt,
    Value, declared_module, typecheck_module,
};

#[derive(Clone)]
struct ValueBinding {
    value: Value,
    mutable: bool,
}

pub fn eval_function(
    module: &Module,
    function_id: &str,
    args: Vec<Value>,
) -> Result<Value, ComputationalCoreError> {
    typecheck_module(module)?;
    let (owner_module, function) = resolve_function(module, function_id)?;
    eval_function_impl(&owner_module, &function, args)
}

fn eval_function_impl(
    module: &Module,
    function: &Function,
    args: Vec<Value>,
) -> Result<Value, ComputationalCoreError> {
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

    let mut env = BTreeMap::from([(
        function.parameter_name.clone(),
        ValueBinding {
            value: argument,
            mutable: false,
        },
    )]);
    eval_block(module, &function.body, &mut env)?.ok_or_else(|| ComputationalCoreError {
        kind: ComputationalCoreErrorKind::MissingReturn,
        detail: format!("function `{}` did not return a value", function.function_id),
    })
}

fn eval_block(
    module: &Module,
    statements: &[Stmt],
    env: &mut BTreeMap<String, ValueBinding>,
) -> Result<Option<Value>, ComputationalCoreError> {
    for statement in statements {
        if let Some(value) = eval_stmt(module, statement, env)? {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

fn eval_stmt(
    module: &Module,
    statement: &Stmt,
    env: &mut BTreeMap<String, ValueBinding>,
) -> Result<Option<Value>, ComputationalCoreError> {
    match statement {
        Stmt::Let {
            name,
            mutable,
            ty,
            value,
        } => {
            if env.contains_key(name) {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "shadowing `{name}` is not supported in the current computational core"
                    ),
                });
            }
            let evaluated = eval_expr(module, value, env)?;
            if evaluated.value_type() != *ty {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "let `{name}` expects {}, found {}",
                        ty.as_str(),
                        evaluated.value_type().as_str()
                    ),
                });
            }
            env.insert(
                name.clone(),
                ValueBinding {
                    value: evaluated,
                    mutable: *mutable,
                },
            );
            Ok(None)
        }
        Stmt::Assign { name, value } => {
            let Some(existing) = env.get(name).cloned() else {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::UnboundVariable,
                    detail: format!("unbound variable `{name}`"),
                });
            };
            if !existing.mutable {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::ImmutableAssignment,
                    detail: format!("cannot assign to immutable binding `{name}`"),
                });
            }
            let evaluated = eval_expr(module, value, env)?;
            if evaluated.value_type() != existing.value.value_type() {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "assignment to `{name}` expects {}, found {}",
                        existing.value.value_type().as_str(),
                        evaluated.value_type().as_str()
                    ),
                });
            }
            env.insert(
                name.clone(),
                ValueBinding {
                    value: evaluated,
                    mutable: true,
                },
            );
            Ok(None)
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let condition_value = eval_expr(module, condition, env)?;
            let Value::Bool(condition_holds) = condition_value else {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "condition must be Bool, found {}",
                        condition_value.value_type().as_str()
                    ),
                });
            };
            let existing_names = env.keys().cloned().collect::<BTreeSet<_>>();
            let result = if condition_holds {
                eval_block(module, then_body, env)?
            } else {
                eval_block(module, else_body, env)?
            };
            env.retain(|name, _| existing_names.contains(name));
            Ok(result)
        }
        Stmt::While { condition, body } => {
            let existing_names = env.keys().cloned().collect::<BTreeSet<_>>();
            let mut iterations = 0usize;
            loop {
                if iterations > 10_000 {
                    return Err(ComputationalCoreError {
                        kind: ComputationalCoreErrorKind::TypeMismatch,
                        detail: "while loop exceeded 10000 iterations in bounded evaluator"
                            .to_string(),
                    });
                }
                let condition_value = eval_expr(module, condition, env)?;
                let Value::Bool(condition_holds) = condition_value else {
                    return Err(ComputationalCoreError {
                        kind: ComputationalCoreErrorKind::TypeMismatch,
                        detail: format!(
                            "condition must be Bool, found {}",
                            condition_value.value_type().as_str()
                        ),
                    });
                };
                if !condition_holds {
                    break;
                }
                if let Some(value) = eval_block(module, body, env)? {
                    env.retain(|name, _| existing_names.contains(name));
                    return Ok(Some(value));
                }
                env.retain(|name, _| existing_names.contains(name));
                iterations += 1;
            }
            Ok(None)
        }
        Stmt::Return(value) => eval_expr(module, value, env).map(Some),
    }
}

fn eval_expr(
    module: &Module,
    expr: &Expr,
    env: &BTreeMap<String, ValueBinding>,
) -> Result<Value, ComputationalCoreError> {
    match expr {
        Expr::Int64(value) => Ok(Value::Int64(*value)),
        Expr::Bool(value) => Ok(Value::Bool(*value)),
        Expr::Text(value) => Ok(Value::Text(value.clone())),
        Expr::Variable(name) => env
            .get(name)
            .map(|binding| binding.value.clone())
            .ok_or_else(|| ComputationalCoreError {
                kind: ComputationalCoreErrorKind::UnboundVariable,
                detail: format!("unbound variable `{name}`"),
            }),
        Expr::Binary { op, left, right } => {
            let left_value = eval_expr(module, left, env)?;
            let right_value = eval_expr(module, right, env)?;
            match (op, left_value, right_value) {
                (BinaryOp::Add, Value::Int64(left), Value::Int64(right)) => {
                    Ok(Value::Int64(left + right))
                }
                (BinaryOp::Sub, Value::Int64(left), Value::Int64(right)) => {
                    Ok(Value::Int64(left - right))
                }
                (BinaryOp::Mul, Value::Int64(left), Value::Int64(right)) => {
                    Ok(Value::Int64(left * right))
                }
                (BinaryOp::LessThan, Value::Int64(left), Value::Int64(right)) => {
                    Ok(Value::Bool(left < right))
                }
                (BinaryOp::LessEqual, Value::Int64(left), Value::Int64(right)) => {
                    Ok(Value::Bool(left <= right))
                }
                (operator, left, right) => Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "{operator:?} expects Int64 operands, found {} and {}",
                        left.value_type().as_str(),
                        right.value_type().as_str()
                    ),
                }),
            }
        }
        Expr::ArrayLiteral(elements) => {
            let mut values = Vec::new();
            for element in elements {
                let value = eval_expr(module, element, env)?;
                match value {
                    Value::Int64(value) => values.push(value),
                    other => {
                        return Err(ComputationalCoreError {
                            kind: ComputationalCoreErrorKind::TypeMismatch,
                            detail: format!(
                                "Vec[Int64] literal expects Int64 elements, found {}",
                                other.value_type().as_str()
                            ),
                        });
                    }
                }
            }
            Ok(Value::VecInt64(values))
        }
        Expr::ArrayIndex { array, index } => {
            let array_value = eval_expr(module, array, env)?;
            let index_value = eval_expr(module, index, env)?;
            let Value::VecInt64(values) = array_value else {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "array index expects Vec[Int64], found {}",
                        array_value.value_type().as_str()
                    ),
                });
            };
            let Value::Int64(index) = index_value else {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "array index expects Int64, found {}",
                        index_value.value_type().as_str()
                    ),
                });
            };
            let Some(value) = values.get(index as usize).copied() else {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::OutOfBounds,
                    detail: format!(
                        "array index {index} is out of bounds for length {}",
                        values.len()
                    ),
                });
            };
            Ok(Value::Int64(value))
        }
        Expr::RecordVec3 { x, y, z } => {
            let Value::Int64(x) = eval_expr(module, x, env)? else {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: "Vec3 coordinates must be Int64".to_string(),
                });
            };
            let Value::Int64(y) = eval_expr(module, y, env)? else {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: "Vec3 coordinates must be Int64".to_string(),
                });
            };
            let Value::Int64(z) = eval_expr(module, z, env)? else {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: "Vec3 coordinates must be Int64".to_string(),
                });
            };
            Ok(Value::Vec3 { x, y, z })
        }
        Expr::FieldAccess { base, field } => {
            let base_value = eval_expr(module, base, env)?;
            let Value::Vec3 { x, y, z } = base_value else {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "field access expects Vec3, found {}",
                        base_value.value_type().as_str()
                    ),
                });
            };
            match field.as_str() {
                "x" => Ok(Value::Int64(x)),
                "y" => Ok(Value::Int64(y)),
                "z" => Ok(Value::Int64(z)),
                _ => Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::UnknownField,
                    detail: format!("unknown field `{field}` on Vec3"),
                }),
            }
        }
        Expr::Call {
            function_id,
            argument,
        } => {
            let argument = eval_expr(module, argument, env)?;
            let (owner_module, function) = resolve_function(module, function_id)?;
            eval_function_impl(&owner_module, &function, vec![argument])
        }
    }
}

fn resolve_function(
    module: &Module,
    function_id: &str,
) -> Result<(Module, Function), ComputationalCoreError> {
    if let Some(function) = module
        .functions
        .iter()
        .find(|function| function.function_id == function_id)
    {
        return Ok((module.clone(), function.clone()));
    }

    for import_module_id in &module.imports {
        let imported_module = declared_module(import_module_id)?;
        if let Some(function) = imported_module
            .functions
            .iter()
            .find(|function| function.function_id == function_id)
            .cloned()
        {
            return Ok((imported_module, function));
        }
    }

    Err(ComputationalCoreError {
        kind: ComputationalCoreErrorKind::UnknownFunction,
        detail: format!(
            "module `{}` does not define function `{function_id}`",
            module.module_id
        ),
    })
}
