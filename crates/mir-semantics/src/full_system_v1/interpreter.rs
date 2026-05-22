use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use mir_ast::textual_alpha::{AstContractClauseKind, TextualMirDiagnostic};
use serde::{Deserialize, Serialize};

use super::{
    checker::{analyze_textual_mir_program_path, build_check_report},
    typed_ir::{
        FullSystemV1CheckReport, FullSystemV1Obligation, TypedBinaryOp, TypedBindValue,
        TypedContractClause, TypedExpr, TypedExprKind, TypedFunction, TypedMirModule,
        TypedPerformCall, TypedStmt, TypedTransition, TypedType, TypedUnaryOp,
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
pub enum FullSystemV1EntryKind {
    Function,
    Transition,
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
    pub entry_kind: FullSystemV1EntryKind,
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
    pub effect_session: FullSystemV1EffectSessionState,
    pub observer_safe_summary: String,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1EffectSessionState {
    pub host_input_remaining: usize,
    pub host_output: Vec<FullSystemV1ValueSnapshot>,
    pub published_channels: Vec<String>,
    pub observed_channels: Vec<String>,
    pub witness_refs: Vec<String>,
    pub handoff_refs: Vec<String>,
    pub accepted_cuts: Vec<String>,
    pub all_places_sealed: bool,
    pub no_in_flight: bool,
    pub no_post_cut_send: bool,
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
    Unit,
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
struct EffectSessionState {
    host_inputs: Vec<i64>,
    host_outputs: Vec<RuntimeValue>,
    published_channels: BTreeMap<String, RuntimeValue>,
    observed_channels: Vec<String>,
    witness_store: BTreeMap<String, RuntimeValue>,
    handoff_refs: Vec<String>,
    accepted_cuts: Vec<String>,
    all_places_sealed: bool,
    no_in_flight: bool,
    no_post_cut_send: bool,
    accepted_cut_active: bool,
    next_witness_id: usize,
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
    effect_session: EffectSessionState,
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
            entry_kind: FullSystemV1EntryKind::Function,
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
            effect_session: empty_effect_session_state(),
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
            entry_kind: FullSystemV1EntryKind::Function,
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
            effect_session: empty_effect_session_state(),
            observer_safe_summary: format!(
                "static rejection before executing {entry_function}: root_module_missing"
            ),
            check_report,
            final_public_api_frozen: false,
        };
    };

    let program = ProgramIndex::new(analysis.modules);
    let entry_kind = if program
        .function(&root_module.module_path, entry_function)
        .is_some()
    {
        FullSystemV1EntryKind::Function
    } else {
        FullSystemV1EntryKind::Transition
    };
    let host_inputs = match entry_kind {
        FullSystemV1EntryKind::Function => Vec::new(),
        FullSystemV1EntryKind::Transition => program
            .transition(&root_module.module_path, entry_function)
            .filter(transition_consumes_host_input)
            .map(|_| vec![input])
            .unwrap_or_default(),
    };
    let mut interpreter = Interpreter::new(program, host_inputs);
    let outcome = match entry_kind {
        FullSystemV1EntryKind::Function => interpreter.eval_function(
            &root_module.module_path,
            entry_function,
            input_value.clone(),
        ),
        FullSystemV1EntryKind::Transition => {
            interpreter.eval_transition(&root_module.module_path, entry_function)
        }
    };
    let (compute_trace, effect_session) = interpreter.into_parts();
    let accepted_obligations = check_report.accepted_obligations.clone();
    let residual_obligations = check_report.residual_obligations.clone();

    match outcome {
        Ok(value) => {
            let output = snapshot_value(&value);
            FullSystemV1RunReport {
                surface_kind: run_surface_kind(),
                source_path: source_path_text,
                entry_function: entry_function.to_string(),
                entry_kind,
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
                effect_session,
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
                entry_kind,
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
                effect_session,
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

    fn transition(&self, module_path: &str, transition_name: &str) -> Option<TypedTransition> {
        self.module(module_path).and_then(|module| {
            module
                .transitions
                .iter()
                .find(|transition| transition.transition_name == transition_name)
                .cloned()
        })
    }
}

impl Interpreter {
    fn new(program: ProgramIndex, host_inputs: Vec<i64>) -> Self {
        Self {
            program,
            traces: Vec::new(),
            next_trace_id: 1,
            next_step_id: 1,
            effect_session: EffectSessionState::new(host_inputs),
        }
    }

    fn into_parts(
        self,
    ) -> (
        Vec<FullSystemV1ComputeTrace>,
        FullSystemV1EffectSessionState,
    ) {
        (self.traces, self.effect_session.summary())
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

    fn eval_transition(
        &mut self,
        module_path: &str,
        transition_name: &str,
    ) -> Result<RuntimeValue, RuntimeError> {
        let transition = self
            .program
            .transition(module_path, transition_name)
            .ok_or_else(|| RuntimeError {
                code: "entry_transition_not_found".to_string(),
                message: format!(
                    "module `{module_path}` does not declare transition `{transition_name}`"
                ),
                module_path: module_path.to_string(),
                function_id: transition_name.to_string(),
            })?;
        let trace_index = self.start_trace(module_path, transition_name, &RuntimeValue::Unit);
        let mut env = BTreeMap::new();
        self.record_event(
            trace_index,
            "enter_transition",
            format!(
                "enter transition {}.{} at {}",
                module_path, transition.transition_name, transition.place_ref
            ),
        );
        self.record_event(
            trace_index,
            "transition_capabilities",
            format!(
                "ambient capabilities: {}",
                transition.required_capabilities.join(", ")
            ),
        );

        let result = self.eval_block(
            module_path,
            transition_name,
            &transition.body,
            &mut env,
            trace_index,
        );

        match result {
            Ok(Some(value)) => {
                self.finalize_trace(trace_index, &env, Some(&value), None);
                Ok(value)
            }
            Ok(None) => {
                let value = RuntimeValue::Unit;
                self.finalize_trace(trace_index, &env, Some(&value), None);
                Ok(value)
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
            TypedStmt::Bind {
                name,
                binding_type,
                value,
                contract_clauses,
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
                ensure_runtime_type_supported(
                    binding_type,
                    module_path,
                    function_name,
                    "bind target",
                )?;
                let evaluated = match value {
                    TypedBindValue::Expr(expr) => {
                        self.eval_expr(module_path, function_name, expr, env, trace_index)?
                    }
                    TypedBindValue::Perform(call) => self
                        .execute_effect_call(module_path, function_name, call, env, trace_index)?
                        .unwrap_or(RuntimeValue::Unit),
                };
                if !value_matches_type(&evaluated, binding_type) {
                    return Err(RuntimeError {
                        code: "runtime_type_mismatch".to_string(),
                        message: format!(
                            "bind `{name}` expects `{}`, found `{}`",
                            binding_type.display_name(),
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
                        ty: binding_type.clone(),
                        mutable: false,
                    },
                );
                self.eval_contract_clauses(
                    module_path,
                    function_name,
                    contract_clauses,
                    env,
                    trace_index,
                    AstContractClauseKind::Require,
                )?;
                self.record_event(
                    trace_index,
                    "bind",
                    format!("{name} <- {}", snapshot_value(&evaluated).summary),
                );
                self.eval_contract_clauses(
                    module_path,
                    function_name,
                    contract_clauses,
                    env,
                    trace_index,
                    AstContractClauseKind::Ensure,
                )?;
                Ok(None)
            }
            TypedStmt::Perform {
                call,
                contract_clauses,
                ..
            } => {
                self.eval_contract_clauses(
                    module_path,
                    function_name,
                    contract_clauses,
                    env,
                    trace_index,
                    AstContractClauseKind::Require,
                )?;
                let _ =
                    self.execute_effect_call(module_path, function_name, call, env, trace_index)?;
                self.eval_contract_clauses(
                    module_path,
                    function_name,
                    contract_clauses,
                    env,
                    trace_index,
                    AstContractClauseKind::Ensure,
                )?;
                Ok(None)
            }
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

    fn eval_contract_clauses(
        &mut self,
        module_path: &str,
        function_name: &str,
        clauses: &[TypedContractClause],
        env: &BTreeMap<String, RuntimeBinding>,
        trace_index: usize,
        kind: AstContractClauseKind,
    ) -> Result<(), RuntimeError> {
        for clause in clauses.iter().filter(|clause| clause.kind == kind) {
            let evaluated = self.eval_expr(
                module_path,
                function_name,
                &clause.condition,
                env,
                trace_index,
            )?;
            let RuntimeValue::Bool(holds) = evaluated else {
                return Err(RuntimeError {
                    code: "runtime_type_mismatch".to_string(),
                    message: "contract clause must evaluate to Bool".to_string(),
                    module_path: module_path.to_string(),
                    function_id: function_name.to_string(),
                });
            };
            if !holds {
                return Err(RuntimeError {
                    code: match kind {
                        AstContractClauseKind::Require => "contract_require_failed".to_string(),
                        AstContractClauseKind::Ensure => "contract_ensure_failed".to_string(),
                    },
                    message: match kind {
                        AstContractClauseKind::Require => {
                            "require clause evaluated to false".to_string()
                        }
                        AstContractClauseKind::Ensure => {
                            "ensure clause evaluated to false".to_string()
                        }
                    },
                    module_path: module_path.to_string(),
                    function_id: function_name.to_string(),
                });
            }
        }
        Ok(())
    }

    fn execute_effect_call(
        &mut self,
        module_path: &str,
        function_name: &str,
        call: &TypedPerformCall,
        env: &mut BTreeMap<String, RuntimeBinding>,
        trace_index: usize,
    ) -> Result<Option<RuntimeValue>, RuntimeError> {
        let arguments = call
            .arguments
            .iter()
            .map(|argument| self.eval_expr(module_path, function_name, argument, env, trace_index))
            .collect::<Result<Vec<_>, _>>()?;
        let effect_name = call.effect_name.as_str();
        let boundary_ref = call.boundary_ref.as_str();

        match (effect_name, boundary_ref) {
            ("read_int", "host_input") => {
                let value = self
                    .effect_session
                    .take_host_input()
                    .ok_or_else(|| RuntimeError {
                        code: "adapter_unavailable".to_string(),
                        message: "host_input is empty".to_string(),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    })?;
                self.record_event(
                    trace_index,
                    "host_read",
                    format!("read_int -> {}", snapshot_value(&value).summary),
                );
                Ok(Some(value))
            }
            ("write_int", "host_output") => {
                let value = expect_single_argument(
                    module_path,
                    function_name,
                    effect_name,
                    &arguments,
                    &TypedType::Int64,
                )?;
                self.effect_session.host_outputs.push(value.clone());
                self.record_event(
                    trace_index,
                    "host_write",
                    format!("write_int <- {}", snapshot_value(&value).summary),
                );
                Ok(None)
            }
            ("seal_places", "session_admin") => {
                expect_zero_arguments(module_path, function_name, effect_name, &arguments)?;
                self.effect_session.all_places_sealed = true;
                self.record_event(trace_index, "seal_places", "all places sealed");
                Ok(None)
            }
            ("quiesce_messages", "session_admin") => {
                expect_zero_arguments(module_path, function_name, effect_name, &arguments)?;
                self.effect_session.no_in_flight = true;
                self.record_event(trace_index, "quiesce_messages", "messages quiesced");
                Ok(None)
            }
            ("atomic_cut", "session_cut") => {
                let label = expect_single_text_argument(
                    module_path,
                    function_name,
                    effect_name,
                    &arguments,
                )?;
                if !(self.effect_session.all_places_sealed
                    && self.effect_session.no_in_flight
                    && self.effect_session.no_post_cut_send)
                {
                    return Err(RuntimeError {
                        code: "r2_precondition_failed".to_string(),
                        message: format!(
                            "atomic_cut `{label}` requires NoInFlight={}, AllPlacesSealed={}, NoPostCutSend={}",
                            self.effect_session.no_in_flight,
                            self.effect_session.all_places_sealed,
                            self.effect_session.no_post_cut_send
                        ),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                }
                self.effect_session.accepted_cuts.push(label.clone());
                self.effect_session.accepted_cut_active = true;
                self.effect_session.no_post_cut_send = true;
                self.record_event(trace_index, "atomic_cut", format!("accepted cut `{label}`"));
                Ok(None)
            }
            ("rollback_cut", "session_cut") => {
                let label = expect_single_text_argument(
                    module_path,
                    function_name,
                    effect_name,
                    &arguments,
                )?;
                if self.effect_session.accepted_cuts.contains(&label) {
                    return Err(RuntimeError {
                        code: "rollback_across_cut_rejected".to_string(),
                        message: format!("rollback across accepted cut `{label}` is not admitted"),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                }
                Err(RuntimeError {
                    code: "cut_not_found".to_string(),
                    message: format!("cut `{label}` is not available"),
                    module_path: module_path.to_string(),
                    function_id: function_name.to_string(),
                })
            }
            ("load_cut", "session_cut") => {
                let label = expect_single_text_argument(
                    module_path,
                    function_name,
                    effect_name,
                    &arguments,
                )?;
                if self.effect_session.accepted_cuts.contains(&label) {
                    return Err(RuntimeError {
                        code: "stale_state_non_resurrection".to_string(),
                        message: format!(
                            "load of accepted cut `{label}` would resurrect stale state"
                        ),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                }
                Err(RuntimeError {
                    code: "cut_not_found".to_string(),
                    message: format!("cut `{label}` is not available"),
                    module_path: module_path.to_string(),
                    function_id: function_name.to_string(),
                })
            }
            _ if effect_name.starts_with("publish_") && boundary_ref == "publish_bus" => {
                let channel = effect_name.trim_start_matches("publish_").to_string();
                let value = expect_single_runtime_value(
                    module_path,
                    function_name,
                    effect_name,
                    &arguments,
                )?;
                self.effect_session.record_send_after_cut();
                self.effect_session.no_in_flight = false;
                self.effect_session
                    .published_channels
                    .insert(channel.clone(), value.clone());
                self.record_event(
                    trace_index,
                    "publish",
                    format!("{channel} <- {}", snapshot_value(&value).summary),
                );
                Ok(None)
            }
            _ if effect_name.starts_with("observe_") && boundary_ref == "observe_bus" => {
                expect_zero_arguments(module_path, function_name, effect_name, &arguments)?;
                let channel = effect_name.trim_start_matches("observe_").to_string();
                let value = self
                    .effect_session
                    .published_channels
                    .get(&channel)
                    .cloned()
                    .ok_or_else(|| RuntimeError {
                        code: "missing_publication".to_string(),
                        message: format!("observe `{channel}` requires a prior publication"),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    })?;
                self.effect_session.observed_channels.push(channel.clone());
                self.record_event(
                    trace_index,
                    "observe",
                    format!("{channel} -> {}", snapshot_value(&value).summary),
                );
                Ok(Some(value))
            }
            _ if effect_name.starts_with("issue_")
                && effect_name.ends_with("_witness")
                && boundary_ref == "witness_store" =>
            {
                let payload = expect_single_runtime_value(
                    module_path,
                    function_name,
                    effect_name,
                    &arguments,
                )?;
                let witness_ref = self.effect_session.issue_witness(payload);
                self.record_event(
                    trace_index,
                    "witness_create",
                    format!("issued {witness_ref}"),
                );
                Ok(Some(RuntimeValue::Text(witness_ref)))
            }
            _ if effect_name.starts_with("handoff_") && boundary_ref == "handoff_port" => {
                let witness_ref = expect_single_text_argument(
                    module_path,
                    function_name,
                    effect_name,
                    &arguments,
                )?;
                if !self.effect_session.witness_store.contains_key(&witness_ref) {
                    return Err(RuntimeError {
                        code: "missing_live_witness".to_string(),
                        message: format!("handoff requires live witness `{witness_ref}`"),
                        module_path: module_path.to_string(),
                        function_id: function_name.to_string(),
                    });
                }
                self.effect_session.record_send_after_cut();
                self.effect_session.no_in_flight = false;
                self.effect_session
                    .handoff_refs
                    .push(format!("handoff#{witness_ref}"));
                self.record_event(trace_index, "handoff", format!("handoff via {witness_ref}"));
                Ok(None)
            }
            _ => Err(RuntimeError {
                code: "unsupported_effect_runtime".to_string(),
                message: format!(
                    "effect `{effect_name}` via `{boundary_ref}` is not admitted in the current runtime floor"
                ),
                module_path: module_path.to_string(),
                function_id: function_name.to_string(),
            }),
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

impl EffectSessionState {
    fn new(host_inputs: Vec<i64>) -> Self {
        Self {
            host_inputs,
            host_outputs: Vec::new(),
            published_channels: BTreeMap::new(),
            observed_channels: Vec::new(),
            witness_store: BTreeMap::new(),
            handoff_refs: Vec::new(),
            accepted_cuts: Vec::new(),
            all_places_sealed: false,
            no_in_flight: true,
            no_post_cut_send: true,
            accepted_cut_active: false,
            next_witness_id: 1,
        }
    }

    fn summary(&self) -> FullSystemV1EffectSessionState {
        FullSystemV1EffectSessionState {
            host_input_remaining: self.host_inputs.len(),
            host_output: self.host_outputs.iter().map(snapshot_value).collect(),
            published_channels: self.published_channels.keys().cloned().collect(),
            observed_channels: self.observed_channels.clone(),
            witness_refs: self.witness_store.keys().cloned().collect(),
            handoff_refs: self.handoff_refs.clone(),
            accepted_cuts: self.accepted_cuts.clone(),
            all_places_sealed: self.all_places_sealed,
            no_in_flight: self.no_in_flight,
            no_post_cut_send: self.no_post_cut_send,
        }
    }

    fn take_host_input(&mut self) -> Option<RuntimeValue> {
        if self.host_inputs.is_empty() {
            None
        } else {
            Some(RuntimeValue::Int64(self.host_inputs.remove(0)))
        }
    }

    fn issue_witness(&mut self, payload: RuntimeValue) -> String {
        let witness_ref = format!("witness#{}", self.next_witness_id);
        self.next_witness_id += 1;
        self.witness_store.insert(witness_ref.clone(), payload);
        witness_ref
    }

    fn record_send_after_cut(&mut self) {
        if self.accepted_cut_active {
            self.no_post_cut_send = false;
        }
    }
}

fn empty_effect_session_state() -> FullSystemV1EffectSessionState {
    EffectSessionState::new(Vec::new()).summary()
}

fn transition_consumes_host_input(transition: &TypedTransition) -> bool {
    block_consumes_host_input(&transition.body)
}

fn block_consumes_host_input(block: &[TypedStmt]) -> bool {
    block.iter().any(stmt_consumes_host_input)
}

fn stmt_consumes_host_input(stmt: &TypedStmt) -> bool {
    match stmt {
        TypedStmt::If {
            then_body,
            else_body,
            ..
        } => block_consumes_host_input(then_body) || block_consumes_host_input(else_body),
        TypedStmt::While { body, .. } | TypedStmt::For { body, .. } => {
            block_consumes_host_input(body)
        }
        TypedStmt::Bind { value, .. } => match value {
            TypedBindValue::Expr(_) => false,
            TypedBindValue::Perform(call) => call.boundary_ref == "host_input",
        },
        TypedStmt::Perform { call, .. } => call.boundary_ref == "host_input",
        TypedStmt::Let { .. } | TypedStmt::Assign { .. } | TypedStmt::Return { .. } => false,
    }
}

fn expect_zero_arguments(
    module_path: &str,
    function_name: &str,
    effect_name: &str,
    arguments: &[RuntimeValue],
) -> Result<(), RuntimeError> {
    if arguments.is_empty() {
        Ok(())
    } else {
        Err(RuntimeError {
            code: "runtime_invalid_arity".to_string(),
            message: format!(
                "effect `{effect_name}` expects 0 argument(s), found {}",
                arguments.len()
            ),
            module_path: module_path.to_string(),
            function_id: function_name.to_string(),
        })
    }
}

fn expect_single_runtime_value(
    module_path: &str,
    function_name: &str,
    effect_name: &str,
    arguments: &[RuntimeValue],
) -> Result<RuntimeValue, RuntimeError> {
    if arguments.len() != 1 {
        return Err(RuntimeError {
            code: "runtime_invalid_arity".to_string(),
            message: format!(
                "effect `{effect_name}` expects 1 argument(s), found {}",
                arguments.len()
            ),
            module_path: module_path.to_string(),
            function_id: function_name.to_string(),
        });
    }
    Ok(arguments[0].clone())
}

fn expect_single_argument(
    module_path: &str,
    function_name: &str,
    effect_name: &str,
    arguments: &[RuntimeValue],
    expected: &TypedType,
) -> Result<RuntimeValue, RuntimeError> {
    let value = expect_single_runtime_value(module_path, function_name, effect_name, arguments)?;
    if value_matches_type(&value, expected) {
        Ok(value)
    } else {
        Err(RuntimeError {
            code: "runtime_type_mismatch".to_string(),
            message: format!(
                "effect `{effect_name}` expects `{}`, found `{}`",
                expected.display_name(),
                snapshot_value(&value).type_name
            ),
            module_path: module_path.to_string(),
            function_id: function_name.to_string(),
        })
    }
}

fn expect_single_text_argument(
    module_path: &str,
    function_name: &str,
    effect_name: &str,
    arguments: &[RuntimeValue],
) -> Result<String, RuntimeError> {
    let value = expect_single_argument(
        module_path,
        function_name,
        effect_name,
        arguments,
        &TypedType::Text,
    )?;
    let RuntimeValue::Text(text) = value else {
        unreachable!("value_matches_type ensured Text")
    };
    Ok(text)
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
        TypedType::Bool | TypedType::Int64 | TypedType::Text | TypedType::Unit => Ok(()),
        TypedType::Named(_) => Ok(()),
        TypedType::FixedArray { element, .. } => {
            ensure_runtime_type_supported(element, module_path, function_name, context)
        }
        TypedType::UInt64 | TypedType::Float64 | TypedType::Error => Err(RuntimeError {
            code: "unsupported_runtime_type".to_string(),
            message: format!(
                "{context} uses unsupported runtime type `{}`",
                ty.display_name()
            ),
            module_path: module_path.to_string(),
            function_id: function_name.to_string(),
        }),
    }
}

fn value_matches_type(value: &RuntimeValue, ty: &TypedType) -> bool {
    match (value, ty) {
        (RuntimeValue::Int64(_), TypedType::Int64) => true,
        (RuntimeValue::Bool(_), TypedType::Bool) => true,
        (RuntimeValue::Text(_), TypedType::Text) => true,
        (RuntimeValue::Unit, TypedType::Unit) => true,
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
        RuntimeValue::Unit => "Unit".to_string(),
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
        RuntimeValue::Unit => "Unit".to_string(),
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
