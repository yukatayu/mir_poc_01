mod ast;
mod eval;
mod typecheck;
mod value;

use std::fmt;

pub use ast::{BinaryOp, Expr, Function, Module, Stmt, Type};
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
    ImmutableAssignment,
    UnknownField,
    MissingReturn,
    OutOfBounds,
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
    module_with_imports(
        "Computational.AddOne",
        vec![],
        vec![function(
            "add_one",
            Type::Int64,
            Type::Int64,
            vec![Stmt::Return(add(variable("value"), int(1)))],
        )],
    )
}

pub fn declared_module(module_id: &str) -> Result<Module, ComputationalCoreError> {
    match module_id {
        "Computational.AddOne" => Ok(add_one_module()),
        "Computational.Scope.Positive" => Ok(module_with_imports(
            "Computational.Scope.Positive",
            vec![],
            vec![function(
                "clamp_zero",
                Type::Int64,
                Type::Int64,
                vec![
                    Stmt::Let {
                        name: "y".to_string(),
                        mutable: true,
                        ty: Type::Int64,
                        value: variable("value"),
                    },
                    Stmt::If {
                        condition: lt(variable("y"), int(0)),
                        then_body: vec![Stmt::Assign {
                            name: "y".to_string(),
                            value: int(0),
                        }],
                        else_body: vec![],
                    },
                    Stmt::Return(variable("y")),
                ],
            )],
        )),
        "Computational.Scope.NegativeUseBeforeDeclare" => Ok(module_with_imports(
            "Computational.Scope.NegativeUseBeforeDeclare",
            vec![],
            vec![function(
                "clamp_zero",
                Type::Int64,
                Type::Int64,
                vec![
                    Stmt::Let {
                        name: "x".to_string(),
                        mutable: false,
                        ty: Type::Int64,
                        value: variable("y"),
                    },
                    Stmt::Return(variable("x")),
                ],
            )],
        )),
        "Computational.Arrays.Positive" => Ok(module_with_imports(
            "Computational.Arrays.Positive",
            vec![],
            vec![function(
                "second",
                Type::Int64,
                Type::Int64,
                vec![
                    Stmt::Let {
                        name: "xs".to_string(),
                        mutable: false,
                        ty: Type::VecInt64,
                        value: Expr::ArrayLiteral(vec![
                            sub(variable("value"), int(1)),
                            variable("value"),
                            add(variable("value"), int(1)),
                        ]),
                    },
                    Stmt::Return(Expr::ArrayIndex {
                        array: Box::new(variable("xs")),
                        index: Box::new(int(1)),
                    }),
                ],
            )],
        )),
        "Computational.Arrays.NegativeOutOfBounds" => Ok(module_with_imports(
            "Computational.Arrays.NegativeOutOfBounds",
            vec![],
            vec![function(
                "second",
                Type::Int64,
                Type::Int64,
                vec![
                    Stmt::Let {
                        name: "xs".to_string(),
                        mutable: false,
                        ty: Type::VecInt64,
                        value: Expr::ArrayLiteral(vec![variable("value")]),
                    },
                    Stmt::Return(Expr::ArrayIndex {
                        array: Box::new(variable("xs")),
                        index: Box::new(int(1)),
                    }),
                ],
            )],
        )),
        "Computational.Vec3.Positive" => Ok(module_with_imports(
            "Computational.Vec3.Positive",
            vec![],
            vec![function(
                "length_squared",
                Type::Int64,
                Type::Int64,
                vec![
                    Stmt::Let {
                        name: "v".to_string(),
                        mutable: false,
                        ty: Type::Vec3,
                        value: Expr::RecordVec3 {
                            x: Box::new(variable("value")),
                            y: Box::new(add(variable("value"), int(1))),
                            z: Box::new(add(variable("value"), int(2))),
                        },
                    },
                    Stmt::Return(add(
                        add(
                            mul(field(variable("v"), "x"), field(variable("v"), "x")),
                            mul(field(variable("v"), "y"), field(variable("v"), "y")),
                        ),
                        mul(field(variable("v"), "z"), field(variable("v"), "z")),
                    )),
                ],
            )],
        )),
        "Computational.Vec3.NegativeField" => Ok(module_with_imports(
            "Computational.Vec3.NegativeField",
            vec![],
            vec![function(
                "length_squared",
                Type::Int64,
                Type::Int64,
                vec![
                    Stmt::Let {
                        name: "v".to_string(),
                        mutable: false,
                        ty: Type::Vec3,
                        value: Expr::RecordVec3 {
                            x: Box::new(variable("value")),
                            y: Box::new(add(variable("value"), int(1))),
                            z: Box::new(add(variable("value"), int(2))),
                        },
                    },
                    Stmt::Return(field(variable("v"), "w")),
                ],
            )],
        )),
        "Computational.ControlFlow.Positive" => Ok(module_with_imports(
            "Computational.ControlFlow.Positive",
            vec![],
            vec![function(
                "sum_to",
                Type::Int64,
                Type::Int64,
                vec![
                    Stmt::Let {
                        name: "i".to_string(),
                        mutable: true,
                        ty: Type::Int64,
                        value: int(0),
                    },
                    Stmt::Let {
                        name: "acc".to_string(),
                        mutable: true,
                        ty: Type::Int64,
                        value: int(0),
                    },
                    Stmt::While {
                        condition: le(variable("i"), variable("value")),
                        body: vec![
                            Stmt::Assign {
                                name: "acc".to_string(),
                                value: add(variable("acc"), variable("i")),
                            },
                            Stmt::Assign {
                                name: "i".to_string(),
                                value: add(variable("i"), int(1)),
                            },
                        ],
                    },
                    Stmt::Return(variable("acc")),
                ],
            )],
        )),
        "Computational.ControlFlow.NegativeCondition" => Ok(module_with_imports(
            "Computational.ControlFlow.NegativeCondition",
            vec![],
            vec![function(
                "sum_to",
                Type::Int64,
                Type::Int64,
                vec![Stmt::If {
                    condition: variable("value"),
                    then_body: vec![Stmt::Return(int(0))],
                    else_body: vec![Stmt::Return(int(1))],
                }],
            )],
        )),
        "Computational.Compose.Positive" => Ok(module_with_imports(
            "Computational.Compose.Positive",
            vec!["Computational.AddOne".to_string()],
            vec![function(
                "add_two",
                Type::Int64,
                Type::Int64,
                vec![Stmt::Return(Expr::Call {
                    function_id: "add_one".to_string(),
                    argument: Box::new(Expr::Call {
                        function_id: "add_one".to_string(),
                        argument: Box::new(variable("value")),
                    }),
                })],
            )],
        )),
        "Computational.Compose.NegativeMissingImport" => Ok(module_with_imports(
            "Computational.Compose.NegativeMissingImport",
            vec![],
            vec![function(
                "add_two",
                Type::Int64,
                Type::Int64,
                vec![Stmt::Return(Expr::Call {
                    function_id: "add_one".to_string(),
                    argument: Box::new(variable("value")),
                })],
            )],
        )),
        _ => Err(ComputationalCoreError {
            kind: ComputationalCoreErrorKind::UnknownModule,
            detail: format!("unknown computational module `{module_id}`"),
        }),
    }
}

fn function(function_id: &str, input_type: Type, output_type: Type, body: Vec<Stmt>) -> Function {
    Function {
        function_id: function_id.to_string(),
        parameter_name: "value".to_string(),
        input_type,
        output_type,
        body,
    }
}

fn module_with_imports(module_id: &str, imports: Vec<String>, functions: Vec<Function>) -> Module {
    Module {
        module_id: module_id.to_string(),
        imports,
        functions,
    }
}

fn int(value: i64) -> Expr {
    Expr::Int64(value)
}

fn variable(name: &str) -> Expr {
    Expr::Variable(name.to_string())
}

fn add(left: Expr, right: Expr) -> Expr {
    Expr::Binary {
        op: BinaryOp::Add,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn sub(left: Expr, right: Expr) -> Expr {
    Expr::Binary {
        op: BinaryOp::Sub,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn mul(left: Expr, right: Expr) -> Expr {
    Expr::Binary {
        op: BinaryOp::Mul,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn lt(left: Expr, right: Expr) -> Expr {
    Expr::Binary {
        op: BinaryOp::LessThan,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn le(left: Expr, right: Expr) -> Expr {
    Expr::Binary {
        op: BinaryOp::LessEqual,
        left: Box::new(left),
        right: Box::new(right),
    }
}

fn field(base: Expr, field: &str) -> Expr {
    Expr::FieldAccess {
        base: Box::new(base),
        field: field.to_string(),
    }
}
