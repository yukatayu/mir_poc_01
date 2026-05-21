use std::collections::{BTreeMap, BTreeSet};

use super::{
    BinaryOp, ComputationalCoreError, ComputationalCoreErrorKind, Expr, Function, Module, Stmt,
    Type, declared_module,
};

#[derive(Clone)]
struct TypeBinding {
    ty: Type,
    mutable: bool,
}

pub fn typecheck_module(module: &Module) -> Result<(), ComputationalCoreError> {
    let mut visited = BTreeSet::new();
    typecheck_module_internal(module, &mut visited)
}

fn typecheck_module_internal(
    module: &Module,
    visited: &mut BTreeSet<String>,
) -> Result<(), ComputationalCoreError> {
    if !visited.insert(module.module_id.clone()) {
        return Ok(());
    }

    for function in &module.functions {
        typecheck_function(module, function, visited)?;
    }

    Ok(())
}

fn typecheck_function(
    module: &Module,
    function: &Function,
    visited: &mut BTreeSet<String>,
) -> Result<(), ComputationalCoreError> {
    let mut env = BTreeMap::from([(
        function.parameter_name.clone(),
        TypeBinding {
            ty: function.input_type.clone(),
            mutable: false,
        },
    )]);
    typecheck_block(
        module,
        &function.body,
        &mut env,
        &function.output_type,
        visited,
    )?;
    Ok(())
}

fn typecheck_block(
    module: &Module,
    statements: &[Stmt],
    env: &mut BTreeMap<String, TypeBinding>,
    expected_return: &Type,
    visited: &mut BTreeSet<String>,
) -> Result<(), ComputationalCoreError> {
    for statement in statements {
        typecheck_stmt(module, statement, env, expected_return, visited)?;
    }
    Ok(())
}

fn typecheck_stmt(
    module: &Module,
    statement: &Stmt,
    env: &mut BTreeMap<String, TypeBinding>,
    expected_return: &Type,
    visited: &mut BTreeSet<String>,
) -> Result<(), ComputationalCoreError> {
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
            let value_type = typecheck_expr(module, value, env, visited)?;
            if &value_type != ty {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "let `{name}` expects {}, found {}",
                        ty.as_str(),
                        value_type.as_str()
                    ),
                });
            }
            env.insert(
                name.clone(),
                TypeBinding {
                    ty: ty.clone(),
                    mutable: *mutable,
                },
            );
            Ok(())
        }
        Stmt::Assign { name, value } => {
            let Some(binding) = env.get(name).cloned() else {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::UnboundVariable,
                    detail: format!("unbound variable `{name}`"),
                });
            };
            if !binding.mutable {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::ImmutableAssignment,
                    detail: format!("cannot assign to immutable binding `{name}`"),
                });
            }
            let value_type = typecheck_expr(module, value, env, visited)?;
            if value_type != binding.ty {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "assignment to `{name}` expects {}, found {}",
                        binding.ty.as_str(),
                        value_type.as_str()
                    ),
                });
            }
            Ok(())
        }
        Stmt::If {
            condition,
            then_body,
            else_body,
        } => {
            let condition_type = typecheck_expr(module, condition, env, visited)?;
            if condition_type != Type::Bool {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!("condition must be Bool, found {}", condition_type.as_str()),
                });
            }
            let mut then_env = env.clone();
            let mut else_env = env.clone();
            typecheck_block(module, then_body, &mut then_env, expected_return, visited)?;
            typecheck_block(module, else_body, &mut else_env, expected_return, visited)?;
            Ok(())
        }
        Stmt::While { condition, body } => {
            let condition_type = typecheck_expr(module, condition, env, visited)?;
            if condition_type != Type::Bool {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!("condition must be Bool, found {}", condition_type.as_str()),
                });
            }
            let mut body_env = env.clone();
            typecheck_block(module, body, &mut body_env, expected_return, visited)
        }
        Stmt::Return(value) => {
            let return_type = typecheck_expr(module, value, env, visited)?;
            if &return_type != expected_return {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "function return expects {}, found {}",
                        expected_return.as_str(),
                        return_type.as_str()
                    ),
                });
            }
            Ok(())
        }
    }
}

fn typecheck_expr(
    module: &Module,
    expr: &Expr,
    env: &BTreeMap<String, TypeBinding>,
    visited: &mut BTreeSet<String>,
) -> Result<Type, ComputationalCoreError> {
    match expr {
        Expr::Int64(_) => Ok(Type::Int64),
        Expr::Bool(_) => Ok(Type::Bool),
        Expr::Text(_) => Ok(Type::Text),
        Expr::Variable(name) => env
            .get(name)
            .map(|binding| binding.ty.clone())
            .ok_or_else(|| ComputationalCoreError {
                kind: ComputationalCoreErrorKind::UnboundVariable,
                detail: format!("unbound variable `{name}`"),
            }),
        Expr::Binary { op, left, right } => {
            let left_type = typecheck_expr(module, left, env, visited)?;
            let right_type = typecheck_expr(module, right, env, visited)?;
            match op {
                BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul
                    if left_type == Type::Int64 && right_type == Type::Int64 =>
                {
                    Ok(Type::Int64)
                }
                BinaryOp::LessThan | BinaryOp::LessEqual
                    if left_type == Type::Int64 && right_type == Type::Int64 =>
                {
                    Ok(Type::Bool)
                }
                operator => Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "{operator:?} expects Int64 operands, found {} and {}",
                        left_type.as_str(),
                        right_type.as_str()
                    ),
                }),
            }
        }
        Expr::ArrayLiteral(elements) => {
            for element in elements {
                let element_type = typecheck_expr(module, element, env, visited)?;
                if element_type != Type::Int64 {
                    return Err(ComputationalCoreError {
                        kind: ComputationalCoreErrorKind::TypeMismatch,
                        detail: format!(
                            "Vec[Int64] literal expects Int64 elements, found {}",
                            element_type.as_str()
                        ),
                    });
                }
            }
            Ok(Type::VecInt64)
        }
        Expr::ArrayIndex { array, index } => {
            let array_type = typecheck_expr(module, array, env, visited)?;
            let index_type = typecheck_expr(module, index, env, visited)?;
            if array_type != Type::VecInt64 {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!(
                        "array index expects Vec[Int64], found {}",
                        array_type.as_str()
                    ),
                });
            }
            if index_type != Type::Int64 {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!("array index expects Int64, found {}", index_type.as_str()),
                });
            }
            Ok(Type::Int64)
        }
        Expr::RecordVec3 { x, y, z } => {
            for coordinate in [x, y, z] {
                let ty = typecheck_expr(module, coordinate, env, visited)?;
                if ty != Type::Int64 {
                    return Err(ComputationalCoreError {
                        kind: ComputationalCoreErrorKind::TypeMismatch,
                        detail: format!("Vec3 coordinates expect Int64, found {}", ty.as_str()),
                    });
                }
            }
            Ok(Type::Vec3)
        }
        Expr::FieldAccess { base, field } => {
            let base_type = typecheck_expr(module, base, env, visited)?;
            if base_type != Type::Vec3 {
                return Err(ComputationalCoreError {
                    kind: ComputationalCoreErrorKind::TypeMismatch,
                    detail: format!("field access expects Vec3, found {}", base_type.as_str()),
                });
            }
            match field.as_str() {
                "x" | "y" | "z" => Ok(Type::Int64),
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
            let (owner_module, function) = resolve_function(module, function_id)?;
            typecheck_module_internal(&owner_module, visited)?;
            let argument_type = typecheck_expr(module, argument, env, visited)?;
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
            Ok(function.output_type.clone())
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
