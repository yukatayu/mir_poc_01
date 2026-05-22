use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use mir_ast::textual_alpha::TextualMirDiagnostic;
use serde::{Deserialize, Serialize};

use super::{
    checker::{analyze_textual_mir_program_path, build_check_report},
    typed_ir::{
        FullSystemV1CheckReport, FullSystemV1Obligation, TypedBinaryOp, TypedExpr, TypedExprKind,
        TypedFunction, TypedMirModule, TypedStmt, TypedType, TypedUnaryOp,
    },
};

pub const FULL_SYSTEM_V1_RUN_SURFACE_KIND: &str = "full_system_v1_run_report";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FullSystemV1ExecutionOutcome {
    Accepted,
    StaticRejection,
    RuntimeRejection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1ValueSnapshot {
    pub type_name: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1BindingSnapshot {
    pub name: String,
    pub mutable: bool,
    pub type_name: String,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1ComputeEvent {
    pub step_id: usize,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1RuntimeRejection {
    pub code: String,
    pub message: String,
    pub module_path: String,
    pub function_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1ComputeTrace {
    pub trace_id: usize,
    pub module_path: String,
    pub function_id: String,
    pub inputs: Vec<FullSystemV1ValueSnapshot>,
    pub local_bindings_summary: Vec<FullSystemV1BindingSnapshot>,
    pub branch_taken: Vec<String>,
    pub outputs: Option<FullSystemV1ValueSnapshot>,
    pub rejected_reason: Option<FullSystemV1RuntimeRejection>,
    pub events: Vec<FullSystemV1ComputeEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1RunReport {
    #[serde(default = "run_surface_kind")]
    pub surface_kind: String,
    pub source_path: String,
    pub entry_function: String,
    pub accepted: bool,
    pub outcome: FullSystemV1ExecutionOutcome,
    pub input: FullSystemV1ValueSnapshot,
    pub output: Option<FullSystemV1ValueSnapshot>,
    pub check_report: FullSystemV1CheckReport,
    pub accepted_obligations: Vec<FullSystemV1Obligation>,
    pub residual_obligations: Vec<FullSystemV1Obligation>,
    pub diagnostics: Vec<TextualMirDiagnostic>,
    pub runtime_rejection: Option<FullSystemV1RuntimeRejection>,
    pub program_module_paths: Vec<String>,
    pub compute_trace: Vec<FullSystemV1ComputeTrace>,
    pub observer_safe_summary: String,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeBinding {
    value: RuntimeValue,
    ty: TypedType,
    mutable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuntimeValue {
    Int64(i64),
    Bool(bool),
    Text(String),
    Array(Vec<RuntimeValue>),
    Record {
        type_name: String,
        fields: BTreeMap<String, RuntimeValue>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeError {
    code: String,
    message: String,
    module_path: String,
    function_id: String,
}

#[derive(Debug, Clone)]
struct ProgramIndex {
    modules: BTreeMap<String, TypedMirModule>,
}

struct Interpreter {
    program: ProgramIndex,
    traces: Vec<FullSystemV1ComputeTrace>,
    next_trace_id: usize,
    next_step_id: usize,
}

pub fn run_textual_mir_function_path(
    path: impl AsRef<Path>,
    entry_function: &str,
    input: i64,
) -> FullSystemV1RunReport {
    let source_path = path.as_ref().to_path_buf();
    let source_path_text = source_path.display().to_string();
    let input_value = RuntimeValue::Int64(input);
    let input_snapshot = snapshot_value(&input_value);
    let analysis = analyze_textual_mir_program_path(&source_path);
    let check_report = build_check_report(&analysis);
    let program_module_paths = analysis
        .modules
        .iter()
        .map(|module| module.module_path.clone())
        .collect::<Vec<_>>();

    if !analysis.accepted {
        let diagnostics = check_report.diagnostics.clone();
        return FullSystemV1RunReport {
            surface_kind: run_surface_kind(),
            source_path: source_path_text,
            entry_function: entry_function.to_string(),
            accepted: false,
            outcome: FullSystemV1ExecutionOutcome::StaticRejection,
            input: input_snapshot,
            output: None,
            accepted_obligations: check_report.accepted_obligations.clone(),
            residual_obligations: check_report.residual_obligations.clone(),
            diagnostics: diagnostics.clone(),
            runtime_rejection: None,
            program_module_paths,
            compute_trace: Vec::new(),
            observer_safe_summary: format!(
                "static rejection before executing {entry_function}: {}",
                diagnostics
                    .iter()
                    .map(|row| row.code.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            check_report,
            final_public_api_frozen: false,
        };
    }

    let Some(root_module) = analysis.root_module.clone() else {
        let diagnostics = vec![internal_diagnostic(
            "root_module_missing",
            format!("accepted program did not retain the root module for `{source_path_text}`"),
        )];
        let check_report = FullSystemV1CheckReport {
            accepted: false,
            module: None,
            accepted_obligations: Vec::new(),
            residual_obligations: Vec::new(),
            diagnostics: diagnostics.clone(),
            final_public_api_frozen: false,
        };
        return FullSystemV1RunReport {
            surface_kind: run_surface_kind(),
            source_path: source_path_text,
            entry_function: entry_function.to_string(),
            accepted: false,
            outcome: FullSystemV1ExecutionOutcome::StaticRejection,
            input: input_snapshot,
            output: None,
            accepted_obligations: Vec::new(),
            residual_obligations: Vec::new(),
            diagnostics: diagnostics.clone(),
            runtime_rejection: None,
            program_module_paths,
            compute_trace: Vec::new(),
            observer_safe_summary: format!(
                "static rejection before executing {entry_function}: root_module_missing"
            ),
            check_report,
            final_public_api_frozen: false,
        };
    };

    let mut interpreter = Interpreter::new(ProgramIndex::new(analysis.modules));
    let outcome = interpreter.eval_function(
        &root_module.module_path,
        entry_function,
        input_value.clone(),
    );
    let compute_trace = interpreter.into_traces();
    let accepted_obligations = check_report.accepted_obligations.clone();
    let residual_obligations = check_report.residual_obligations.clone();

    match outcome {
        Ok(value) => {
            let output = snapshot_value(&value);
            FullSystemV1RunReport {
                surface_kind: run_surface_kind(),
                source_path: source_path_text,
                entry_function: entry_function.to_string(),
                accepted: true,
                outcome: FullSystemV1ExecutionOutcome::Accepted,
                input: input_snapshot,
                output: Some(output.clone()),
                accepted_obligations,
                residual_obligations,
                diagnostics: Vec::new(),
                runtime_rejection: None,
                program_module_paths,
                compute_trace,
                observer_safe_summary: format!(
                    "accepted {entry_function} from {} -> {}",
                    root_module.module_path, output.summary
                ),
                check_report,
                final_public_api_frozen: false,
            }
        }
        Err(error) => {
            let runtime_rejection = runtime_rejection(&error);
            FullSystemV1RunReport {
                surface_kind: run_surface_kind(),
                source_path: source_path_text,
                entry_function: entry_function.to_string(),
                accepted: false,
                outcome: FullSystemV1ExecutionOutcome::RuntimeRejection,
                input: input_snapshot,
                output: None,
                accepted_obligations,
                residual_obligations,
                diagnostics: Vec::new(),
                runtime_rejection: Some(runtime_rejection.clone()),
                program_module_paths,
                compute_trace,
                observer_safe_summary: format!(
                    "runtime rejection in {entry_function}: {}",
                    runtime_rejection.code
                ),
                check_report,
                final_public_api_frozen: false,
            }
        }
    }
}

impl ProgramIndex {
    fn new(modules: Vec<TypedMirModule>) -> Self {
        let modules = modules
            .into_iter()
            .map(|module| (module.module_path.clone(), module))
            .collect();
        Self { modules }
    }

    fn module(&self, module_path: &str) -> Option<&TypedMirModule> {
        self.modules.get(module_path)
    }

    fn function(&self, module_path: &str, function_name: &str) -> Option<TypedFunction> {
        self.module(module_path).and_then(|module| {
            module
                .functions
                .iter()
                .find(|function| function.function_name == function_name)
                .cloned()
        })
    }
}

impl Interpreter {
    fn new(program: ProgramIndex) -> Self {
        Self {
            program,
            traces: Vec::new(),
            next_trace_id: 1,
            next_step_id: 1,
        }
    }

    fn into_traces(self) -> Vec<FullSystemV1ComputeTrace> {
        self.traces
    }

    fn eval_function(
        &mut self,
        module_path: &str,
        function_name: &str,
        input: RuntimeValue,
    ) -> Result<RuntimeValue, RuntimeError> {
        let function = self
            .program
            .function(module_path, function_name)
            .ok_or_else(|| RuntimeError {
                code: "entry_function_not_found".to_string(),
                message: format!(
                    "module `{module_path}` does not declare function `{function_name}`"
                ),
                module_path: module_path.to_string(),
                function_id: function_name.to_string(),
            })?;

        ensure_runtime_type_supported(
            &function.parameter.param_type,
            module_path,
            function_name,
            "function parameter",
        )?;
        ensure_runtime_type_supported(
            &function.output_type,
            module_path,
            function_name,
            "function return",
        )?;

        if !value_matches_type(&input, &function.parameter.param_type) {
            return Err(RuntimeError {
                code: "runtime_type_mismatch".to_string(),
                message: format!(
                    "function `{function_name}` expects `{}`, found `{}`",
                    function.parameter.param_type.display_name(),
                    snapshot_value(&input).type_name
                ),
                module_path: module_path.to_string(),
                function_id: function_name.to_string(),
            });
        }

        let trace_index = self.start_trace(module_path, function_name, &input);
        let mut env = BTreeMap::from([(
            function.parameter.name.clone(),
            RuntimeBinding {
                value: input,
                ty: function.parameter.param_type.clone(),
                mutable: false,
            },
        )]);
        self.record_event(
            trace_index,
            "enter",
            format!("enter {}.{}", module_path, function.function_name.as_str()),
        );

        let result = self.eval_block(
            module_path,
            function_name,
            &function.body,
            &mut env,
            trace_index,
        );

        match result {
            Ok(Some(value)) => {
                if !value_matches_type(&value, &function.output_type) {
                    let error = RuntimeError {
                        code: "runtime_type_mismatch".to_string(),
                        message: format!(
                            "function `{function_name}` returned `{}` but declared `{}`",
                            snapshot_value(&value).type_name,
                            function.output_type.display_name()
                        ),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    };
                    self.finalize_trace(trace_index, &env, None, Some(&error));
                    return Err(error);
                }
                self.finalize_trace(trace_index, &env, Some(&value), None);
                Ok(value)
            }
            Ok(None) => {
                let error = RuntimeError {
                    code: "runtime_missing_return".to_string(),
                    message: format!("function `{function_name}` did not return a value"),
                    module_path: module_path.to_string(),
                    function_id: function_name.to_string(),
                };
                self.finalize_trace(trace_index, &env, None, Some(&error));
                Err(error)
            }
            Err(error) => {
                self.finalize_trace(trace_index, &env, None, Some(&error));
                Err(error)
            }
        }
    }

    fn eval_block(
        &mut self,
        module_path: &str,
        function_name: &str,
        statements: &[TypedStmt],
        env: &mut BTreeMap<String, RuntimeBinding>,
        trace_index: usize,
    ) -> Result<Option<RuntimeValue>, RuntimeError> {
        for statement in statements {
            if let Some(value) =
                self.eval_stmt(module_path, function_name, statement, env, trace_index)?
            {
                return Ok(Some(value));
            }
        }
        Ok(None)
    }

    fn eval_stmt(
        &mut self,
        module_path: &str,
        function_name: &str,
        statement: &TypedStmt,
        env: &mut BTreeMap<String, RuntimeBinding>,
        trace_index: usize,
    ) -> Result<Option<RuntimeValue>, RuntimeError> {
        match statement {
            TypedStmt::Let {
                name,
                mutable,
                ty,
                value,
                ..
            } => {
                if env.contains_key(name) {
                    return Err(RuntimeError {
                        code: "shadowing_not_supported".to_string(),
                        message: format!(
                            "shadowing `{name}` is not admitted in the current interpreter floor"
                        ),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                }
                ensure_runtime_type_supported(ty, module_path, function_name, "let binding")?;
                let evaluated =
                    self.eval_expr(module_path, function_name, value, env, trace_index)?;
                if !value_matches_type(&evaluated, ty) {
                    return Err(RuntimeError {
                        code: "runtime_type_mismatch".to_string(),
                        message: format!(
                            "let `{name}` expects `{}`, found `{}`",
                            ty.display_name(),
                            snapshot_value(&evaluated).type_name
                        ),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                }
                env.insert(
                    name.clone(),
                    RuntimeBinding {
                        value: evaluated.clone(),
                        ty: ty.clone(),
                        mutable: *mutable,
                    },
                );
                self.record_event(
                    trace_index,
                    "let",
                    format!("let {name} = {}", snapshot_value(&evaluated).summary),
                );
                Ok(None)
            }
            TypedStmt::Assign { name, value, .. } => {
                let Some(binding) = env.get(name).cloned() else {
                    return Err(RuntimeError {
                        code: "runtime_unbound_variable".to_string(),
                        message: format!("assignment target `{name}` is not in scope"),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                };
                if !binding.mutable {
                    return Err(RuntimeError {
                        code: "runtime_immutable_assignment".to_string(),
                        message: format!("cannot assign to immutable binding `{name}`"),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                }
                let evaluated =
                    self.eval_expr(module_path, function_name, value, env, trace_index)?;
                if !value_matches_type(&evaluated, &binding.ty) {
                    return Err(RuntimeError {
                        code: "runtime_type_mismatch".to_string(),
                        message: format!(
                            "assignment to `{name}` expects `{}`, found `{}`",
                            binding.ty.display_name(),
                            snapshot_value(&evaluated).type_name
                        ),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                }
                env.insert(
                    name.clone(),
                    RuntimeBinding {
                        value: evaluated.clone(),
                        ty: binding.ty,
                        mutable: true,
                    },
                );
                self.record_event(
                    trace_index,
                    "assign",
                    format!("{name} = {}", snapshot_value(&evaluated).summary),
                );
                Ok(None)
            }
            TypedStmt::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                let condition_value =
                    self.eval_expr(module_path, function_name, condition, env, trace_index)?;
                let RuntimeValue::Bool(condition_holds) = condition_value else {
                    return Err(RuntimeError {
                        code: "runtime_type_mismatch".to_string(),
                        message: "if condition must evaluate to Bool".to_string(),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                };
                let branch_label = if condition_holds { "then" } else { "else" };
                self.record_event(
                    trace_index,
                    "if",
                    format!("if condition -> {branch_label} branch"),
                );
                self.record_branch(trace_index, branch_label.to_string());
                let existing = env.keys().cloned().collect::<BTreeSet<_>>();
                let mut branch_env = env.clone();
                let result = if condition_holds {
                    self.eval_block(
                        module_path,
                        function_name,
                        then_body,
                        &mut branch_env,
                        trace_index,
                    )?
                } else {
                    self.eval_block(
                        module_path,
                        function_name,
                        else_body,
                        &mut branch_env,
                        trace_index,
                    )?
                };
                restore_existing_bindings(env, branch_env, &existing);
                Ok(result)
            }
            TypedStmt::While {
                condition, body, ..
            } => {
                let existing = env.keys().cloned().collect::<BTreeSet<_>>();
                let mut iterations = 0usize;
                loop {
                    if iterations >= 10_000 {
                        return Err(RuntimeError {
                            code: "runtime_loop_limit_exceeded".to_string(),
                            message: "while loop exceeded 10000 iterations".to_string(),
                            module_path: module_path.to_string(),
                            function_id: function_name.to_string(),
                        });
                    }
                    let condition_value =
                        self.eval_expr(module_path, function_name, condition, env, trace_index)?;
                    let RuntimeValue::Bool(condition_holds) = condition_value else {
                        return Err(RuntimeError {
                            code: "runtime_type_mismatch".to_string(),
                            message: "while condition must evaluate to Bool".to_string(),
                            module_path: module_path.to_string(),
                            function_id: function_name.to_string(),
                        });
                    };
                    if !condition_holds {
                        self.record_event(trace_index, "while", "while condition -> break");
                        self.record_branch(trace_index, format!("while-break@{iterations}"));
                        break;
                    }
                    self.record_event(trace_index, "while", "while condition -> continue");
                    self.record_branch(trace_index, format!("while-continue@{iterations}"));
                    let mut body_env = env.clone();
                    if let Some(value) = self.eval_block(
                        module_path,
                        function_name,
                        body,
                        &mut body_env,
                        trace_index,
                    )? {
                        restore_existing_bindings(env, body_env, &existing);
                        return Ok(Some(value));
                    }
                    restore_existing_bindings(env, body_env, &existing);
                    iterations += 1;
                }
                Ok(None)
            }
            TypedStmt::For { .. } => Err(RuntimeError {
                code: "unsupported_for_loop".to_string(),
                message: "for-loop execution is not admitted in the current interpreter floor"
                    .to_string(),
                module_path: module_path.to_string(),
                function_id: function_name.to_string(),
            }),
            TypedStmt::Bind { .. } | TypedStmt::Perform { .. } => Err(RuntimeError {
                code: "effectful_statement_not_supported".to_string(),
                message: "effectful statements remain deferred to P-MIR-04".to_string(),
                module_path: module_path.to_string(),
                function_id: function_name.to_string(),
            }),
            TypedStmt::Return { value, .. } => {
                let evaluated =
                    self.eval_expr(module_path, function_name, value, env, trace_index)?;
                self.record_event(
                    trace_index,
                    "return",
                    format!("return {}", snapshot_value(&evaluated).summary),
                );
                Ok(Some(evaluated))
            }
        }
    }

    fn eval_expr(
        &mut self,
        module_path: &str,
        function_name: &str,
        expr: &TypedExpr,
        env: &BTreeMap<String, RuntimeBinding>,
        trace_index: usize,
    ) -> Result<RuntimeValue, RuntimeError> {
        match &expr.kind {
            TypedExprKind::IntLiteral(value) => Ok(RuntimeValue::Int64(*value)),
            TypedExprKind::BoolLiteral(value) => Ok(RuntimeValue::Bool(*value)),
            TypedExprKind::TextLiteral(value) => Ok(RuntimeValue::Text(value.clone())),
            TypedExprKind::FloatLiteral(_) => Err(RuntimeError {
                code: "unsupported_runtime_type".to_string(),
                message: "Float64 execution is not admitted in the current interpreter floor"
                    .to_string(),
                module_path: module_path.to_string(),
                function_id: function_name.to_string(),
            }),
            TypedExprKind::Variable(name) => env
                .get(name)
                .map(|binding| binding.value.clone())
                .ok_or_else(|| RuntimeError {
                    code: "runtime_unbound_variable".to_string(),
                    message: format!("variable `{name}` is not in scope"),
                    module_path: module_path.to_string(),
                    function_id: function_name.to_string(),
                }),
            TypedExprKind::ArrayLiteral(elements) => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.eval_expr(
                        module_path,
                        function_name,
                        element,
                        env,
                        trace_index,
                    )?);
                }
                Ok(RuntimeValue::Array(values))
            }
            TypedExprKind::RecordConstruct { fields, .. } => {
                let mut values = BTreeMap::new();
                for field in fields {
                    values.insert(
                        field.field_name.clone(),
                        self.eval_expr(module_path, function_name, &field.value, env, trace_index)?,
                    );
                }
                Ok(RuntimeValue::Record {
                    type_name: match &expr.ty {
                        TypedType::Named(name) => name.clone(),
                        other => other.display_name(),
                    },
                    fields: values,
                })
            }
            TypedExprKind::Call {
                function_name: callee,
                module_path: callee_module,
                arguments,
            } => {
                if arguments.len() != 1 {
                    return Err(RuntimeError {
                        code: "runtime_invalid_arity".to_string(),
                        message: format!(
                            "function `{callee}` expects 1 argument, found {}",
                            arguments.len()
                        ),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                }
                let argument =
                    self.eval_expr(module_path, function_name, &arguments[0], env, trace_index)?;
                self.record_event(
                    trace_index,
                    "call",
                    format!("call {}.{callee}", callee_module),
                );
                self.eval_function(callee_module, callee, argument)
            }
            TypedExprKind::Index { base, index } => {
                let base_value =
                    self.eval_expr(module_path, function_name, base, env, trace_index)?;
                let index_value =
                    self.eval_expr(module_path, function_name, index, env, trace_index)?;
                let RuntimeValue::Array(values) = base_value else {
                    return Err(RuntimeError {
                        code: "runtime_type_mismatch".to_string(),
                        message: "array index base must evaluate to a fixed array".to_string(),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                };
                let RuntimeValue::Int64(index) = index_value else {
                    return Err(RuntimeError {
                        code: "runtime_type_mismatch".to_string(),
                        message: "array index must evaluate to Int64".to_string(),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                };
                if index < 0 || index as usize >= values.len() {
                    return Err(RuntimeError {
                        code: "runtime_out_of_bounds".to_string(),
                        message: format!(
                            "array index {index} is out of bounds for length {}",
                            values.len()
                        ),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                }
                Ok(values[index as usize].clone())
            }
            TypedExprKind::FieldAccess { base, field_name } => {
                let base_value =
                    self.eval_expr(module_path, function_name, base, env, trace_index)?;
                let RuntimeValue::Record { fields, .. } = base_value else {
                    return Err(RuntimeError {
                        code: "runtime_type_mismatch".to_string(),
                        message: "field access base must evaluate to a record value".to_string(),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                };
                fields.get(field_name).cloned().ok_or_else(|| RuntimeError {
                    code: "runtime_unknown_field".to_string(),
                    message: format!("record field `{field_name}` is not present"),
                    module_path: module_path.to_string(),
                    function_id: function_name.to_string(),
                })
            }
            TypedExprKind::Unary { op, expr } => {
                let value = self.eval_expr(module_path, function_name, expr, env, trace_index)?;
                match (op, value) {
                    (TypedUnaryOp::Negate, RuntimeValue::Int64(value)) => {
                        Ok(RuntimeValue::Int64(-value))
                    }
                    (TypedUnaryOp::Not, RuntimeValue::Bool(value)) => {
                        Ok(RuntimeValue::Bool(!value))
                    }
                    (TypedUnaryOp::Negate, _) => Err(RuntimeError {
                        code: "runtime_type_mismatch".to_string(),
                        message: "numeric negation expects Int64".to_string(),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    }),
                    (TypedUnaryOp::Not, _) => Err(RuntimeError {
                        code: "runtime_type_mismatch".to_string(),
                        message: "logical not expects Bool".to_string(),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    }),
                }
            }
            TypedExprKind::Binary { op, left, right } => {
                let left_value =
                    self.eval_expr(module_path, function_name, left, env, trace_index)?;
                let right_value =
                    self.eval_expr(module_path, function_name, right, env, trace_index)?;
                self.eval_binary(module_path, function_name, op, left_value, right_value)
            }
        }
    }

    fn eval_binary(
        &self,
        module_path: &str,
        function_name: &str,
        op: &TypedBinaryOp,
        left: RuntimeValue,
        right: RuntimeValue,
    ) -> Result<RuntimeValue, RuntimeError> {
        use TypedBinaryOp as Op;
        match op {
            Op::Add => match (left, right) {
                (RuntimeValue::Int64(left), RuntimeValue::Int64(right)) => {
                    Ok(RuntimeValue::Int64(left + right))
                }
                _ => Err(type_mismatch_error(
                    module_path,
                    function_name,
                    "arithmetic operator expects Int64 operands",
                )),
            },
            Op::Sub => match (left, right) {
                (RuntimeValue::Int64(left), RuntimeValue::Int64(right)) => {
                    Ok(RuntimeValue::Int64(left - right))
                }
                _ => Err(type_mismatch_error(
                    module_path,
                    function_name,
                    "arithmetic operator expects Int64 operands",
                )),
            },
            Op::Mul => match (left, right) {
                (RuntimeValue::Int64(left), RuntimeValue::Int64(right)) => {
                    Ok(RuntimeValue::Int64(left * right))
                }
                _ => Err(type_mismatch_error(
                    module_path,
                    function_name,
                    "arithmetic operator expects Int64 operands",
                )),
            },
            Op::Div => match (left, right) {
                (RuntimeValue::Int64(_), RuntimeValue::Int64(0)) => Err(RuntimeError {
                    code: "runtime_division_by_zero".to_string(),
                    message: "division by zero".to_string(),
                    module_path: module_path.to_string(),
                    function_id: function_name.to_string(),
                }),
                (RuntimeValue::Int64(left), RuntimeValue::Int64(right)) => {
                    Ok(RuntimeValue::Int64(left / right))
                }
                _ => Err(type_mismatch_error(
                    module_path,
                    function_name,
                    "arithmetic operator expects Int64 operands",
                )),
            },
            Op::Equal => Ok(RuntimeValue::Bool(left == right)),
            Op::NotEqual => Ok(RuntimeValue::Bool(left != right)),
            Op::LessThan => compare_ints(module_path, function_name, left, right, |l, r| l < r),
            Op::LessEqual => compare_ints(module_path, function_name, left, right, |l, r| l <= r),
            Op::GreaterThan => compare_ints(module_path, function_name, left, right, |l, r| l > r),
            Op::GreaterEqual => {
                compare_ints(module_path, function_name, left, right, |l, r| l >= r)
            }
            Op::And => match (left, right) {
                (RuntimeValue::Bool(left), RuntimeValue::Bool(right)) => {
                    Ok(RuntimeValue::Bool(left && right))
                }
                _ => Err(type_mismatch_error(
                    module_path,
                    function_name,
                    "logical operator expects Bool operands",
                )),
            },
            Op::Or => match (left, right) {
                (RuntimeValue::Bool(left), RuntimeValue::Bool(right)) => {
                    Ok(RuntimeValue::Bool(left || right))
                }
                _ => Err(type_mismatch_error(
                    module_path,
                    function_name,
                    "logical operator expects Bool operands",
                )),
            },
        }
    }

    fn start_trace(
        &mut self,
        module_path: &str,
        function_name: &str,
        input: &RuntimeValue,
    ) -> usize {
        let trace_index = self.traces.len();
        self.traces.push(FullSystemV1ComputeTrace {
            trace_id: self.next_trace_id,
            module_path: module_path.to_string(),
            function_id: function_name.to_string(),
            inputs: vec![snapshot_value(input)],
            local_bindings_summary: Vec::new(),
            branch_taken: Vec::new(),
            outputs: None,
            rejected_reason: None,
            events: Vec::new(),
        });
        self.next_trace_id += 1;
        trace_index
    }

    fn finalize_trace(
        &mut self,
        trace_index: usize,
        env: &BTreeMap<String, RuntimeBinding>,
        output: Option<&RuntimeValue>,
        error: Option<&RuntimeError>,
    ) {
        let local_bindings_summary = env
            .iter()
            .map(|(name, binding)| FullSystemV1BindingSnapshot {
                name: name.clone(),
                mutable: binding.mutable,
                type_name: binding.ty.display_name(),
                summary: snapshot_value(&binding.value).summary,
            })
            .collect::<Vec<_>>();
        self.traces[trace_index].local_bindings_summary = local_bindings_summary;
        self.traces[trace_index].outputs = output.map(snapshot_value);
        self.traces[trace_index].rejected_reason = error.map(runtime_rejection);
    }

    fn record_event(&mut self, trace_index: usize, kind: &str, detail: impl Into<String>) {
        self.traces[trace_index]
            .events
            .push(FullSystemV1ComputeEvent {
                step_id: self.next_step_id,
                kind: kind.to_string(),
                detail: detail.into(),
            });
        self.next_step_id += 1;
    }

    fn record_branch(&mut self, trace_index: usize, branch: String) {
        self.traces[trace_index].branch_taken.push(branch);
    }
}

fn compare_ints(
    module_path: &str,
    function_name: &str,
    left: RuntimeValue,
    right: RuntimeValue,
    cmp: impl FnOnce(i64, i64) -> bool,
) -> Result<RuntimeValue, RuntimeError> {
    match (left, right) {
        (RuntimeValue::Int64(left), RuntimeValue::Int64(right)) => {
            Ok(RuntimeValue::Bool(cmp(left, right)))
        }
        _ => Err(type_mismatch_error(
            module_path,
            function_name,
            "comparison expects Int64 operands",
        )),
    }
}

fn restore_existing_bindings(
    env: &mut BTreeMap<String, RuntimeBinding>,
    branch_env: BTreeMap<String, RuntimeBinding>,
    existing: &BTreeSet<String>,
) {
    for name in existing {
        if let Some(binding) = branch_env.get(name).cloned() {
            env.insert(name.clone(), binding);
        }
    }
}

fn ensure_runtime_type_supported(
    ty: &TypedType,
    module_path: &str,
    function_name: &str,
    context: &str,
) -> Result<(), RuntimeError> {
    match ty {
        TypedType::Bool | TypedType::Int64 | TypedType::Text => Ok(()),
        TypedType::Named(_) => Ok(()),
        TypedType::FixedArray { element, .. } => {
            ensure_runtime_type_supported(element, module_path, function_name, context)
        }
        TypedType::UInt64 | TypedType::Float64 | TypedType::Unit | TypedType::Error => {
            Err(RuntimeError {
                code: "unsupported_runtime_type".to_string(),
                message: format!(
                    "{context} uses unsupported runtime type `{}`",
                    ty.display_name()
                ),
                module_path: module_path.to_string(),
                function_id: function_name.to_string(),
            })
        }
    }
}

fn value_matches_type(value: &RuntimeValue, ty: &TypedType) -> bool {
    match (value, ty) {
        (RuntimeValue::Int64(_), TypedType::Int64) => true,
        (RuntimeValue::Bool(_), TypedType::Bool) => true,
        (RuntimeValue::Text(_), TypedType::Text) => true,
        (RuntimeValue::Array(values), TypedType::FixedArray { element, length }) => {
            values.len() == *length
                && values
                    .iter()
                    .all(|value| value_matches_type(value, element))
        }
        (RuntimeValue::Record { type_name, .. }, TypedType::Named(expected)) => {
            type_name == expected
        }
        _ => false,
    }
}

fn snapshot_value(value: &RuntimeValue) -> FullSystemV1ValueSnapshot {
    FullSystemV1ValueSnapshot {
        type_name: runtime_type_name(value),
        summary: runtime_value_summary(value),
    }
}

fn runtime_type_name(value: &RuntimeValue) -> String {
    match value {
        RuntimeValue::Int64(_) => "Int64".to_string(),
        RuntimeValue::Bool(_) => "Bool".to_string(),
        RuntimeValue::Text(_) => "Text".to_string(),
        RuntimeValue::Array(values) => {
            let element_name = values
                .first()
                .map(runtime_type_name)
                .unwrap_or_else(|| "Unknown".to_string());
            format!("[{element_name}; {}]", values.len())
        }
        RuntimeValue::Record { type_name, .. } => type_name.clone(),
    }
}

fn runtime_value_summary(value: &RuntimeValue) -> String {
    match value {
        RuntimeValue::Int64(value) => format!("Int64({value})"),
        RuntimeValue::Bool(value) => format!("Bool({value})"),
        RuntimeValue::Text(value) => format!("Text({value:?})"),
        RuntimeValue::Array(values) => format!(
            "[{}]",
            values
                .iter()
                .map(runtime_value_summary)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        RuntimeValue::Record { type_name, fields } => format!(
            "{type_name}{{{}}}",
            fields
                .iter()
                .map(|(name, value)| format!("{name}={}", runtime_value_summary(value)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn runtime_rejection(error: &RuntimeError) -> FullSystemV1RuntimeRejection {
    FullSystemV1RuntimeRejection {
        code: error.code.clone(),
        message: error.message.clone(),
        module_path: error.module_path.clone(),
        function_id: error.function_id.clone(),
    }
}

fn type_mismatch_error(module_path: &str, function_name: &str, message: &str) -> RuntimeError {
    RuntimeError {
        code: "runtime_type_mismatch".to_string(),
        message: message.to_string(),
        module_path: module_path.to_string(),
        function_id: function_name.to_string(),
    }
}

fn internal_diagnostic(code: &str, message: String) -> TextualMirDiagnostic {
    TextualMirDiagnostic {
        code: code.to_string(),
        message,
        span: mir_ast::textual_alpha::SourceSpan {
            start: 0,
            end: 0,
            line: 1,
            column: 1,
        },
    }
}

fn run_surface_kind() -> String {
    FULL_SYSTEM_V1_RUN_SURFACE_KIND.to_string()
}
