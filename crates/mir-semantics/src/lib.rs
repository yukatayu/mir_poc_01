#![doc = r#"
# mir-semantics

Type checking, effect rows, contracts, graph extraction, and semantic normalization.

This crate currently hosts a **parser-free current L2 minimal interpreter skeleton**
for representative fixtures. It is intentionally narrow and does not try to become
the full Mir runtime.
"#]

mod harness;

use std::{
    collections::{BTreeMap, HashMap},
    fmt, fs,
    path::Path,
};

use serde::{Deserialize, Serialize};

pub use harness::{
    BatchBundleOutcome, BatchBundleReport, BatchRunSummary, BundleDiscoveryFailure,
    BundleDiscoveryReport, BundleExecutionFailure, BundleRunReport,
    CURRENT_L2_HOST_PLAN_SCHEMA_VERSION, EffectPlanRule, EffectPlanVerdict, FixtureBundle,
    FixtureCommitPlan, FixtureHostPlan, FixtureHostStub, FixtureRuntimeRequirement,
    FixtureStoreMutation, NamedProfileRunSummary, PredicatePlanRule, ProfileCatalog,
    ProfileRunSummary, SelectionMode, SelectionProfile, SelectionRequest, SelectionScope,
    SingleFixtureSelector, TraceExpectationOverride, discover_bundles_in_directory,
    host_plan_sidecar_path_for_fixture_path, load_bundle_from_fixture_path,
    load_host_plan_from_path, load_host_plan_sidecar_for_fixture_path, run_bundle, run_directory,
    run_directory_named_profile, run_directory_profiled, run_directory_selected,
    select_bundles_from_discovery, select_bundles_from_request,
};

pub type PlaceStore = BTreeMap<String, Vec<String>>;
type ScopePath = Vec<String>;

/// current L2 の parser-free 最小 interpreter が使う failure kind の最小集合。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    ExplicitFailure,
    Reject,
}

/// current L2 の parser-free 最小 interpreter における terminal outcome。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum TerminalOutcome {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "explicit_failure")]
    ExplicitFailure,
    #[serde(rename = "Reject")]
    Reject,
}

impl From<FailureKind> for TerminalOutcome {
    fn from(value: FailureKind) -> Self {
        match value {
            FailureKind::ExplicitFailure => Self::ExplicitFailure,
            FailureKind::Reject => Self::Reject,
        }
    }
}

/// step semantics が node 1 個ぶんの前進で返す最小 control signal。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepControl {
    Continue,
    BubbleFailure(FailureKind),
    Halt(TerminalOutcome),
}

/// predicate evaluation の site。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum PredicateSite {
    #[serde(rename = "request-require")]
    RequestRequire,
    #[serde(rename = "request-ensure")]
    RequestEnsure,
    #[serde(rename = "option-admit")]
    OptionAdmit,
}

/// predicate oracle が返す current L2 の最小 verdict。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum PredicateVerdict {
    #[serde(rename = "satisfied")]
    Satisfied,
    #[serde(rename = "unsatisfied")]
    Unsatisfied,
}

/// effect oracle が返す current L2 の最小 verdict。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectVerdict<Commit = ()> {
    Success { commit: Commit },
    ExplicitFailure,
}

/// parser-free current L2 で使う request mode。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum RequestMode {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "via")]
    Via,
}

/// request-local non-admissible metadata の formal subreason。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum NonAdmissibleSubreason {
    #[serde(rename = "admit-miss")]
    AdmitMiss,
    #[serde(rename = "lease-expired")]
    LeaseExpired,
}

/// current L2 event surface の最小 event kind。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum EventKind {
    #[serde(rename = "perform-success")]
    PerformSuccess,
    #[serde(rename = "perform-failure")]
    PerformFailure,
    #[serde(rename = "rollback")]
    Rollback,
    #[serde(rename = "atomic-cut")]
    AtomicCut,
    #[serde(rename = "Reject")]
    Reject,
}

/// current L2 の static gate verdict。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum StaticGateVerdict {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "malformed")]
    Malformed,
    #[serde(rename = "underdeclared")]
    Underdeclared,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
enum TryRollbackStructuralVerdict {
    #[serde(rename = "no_findings")]
    NoFindings,
    #[serde(rename = "findings_present")]
    FindingsPresent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
enum TryRollbackStructuralSubjectKind {
    #[serde(rename = "TryFallback")]
    TryFallback,
    #[serde(rename = "AtomicCut")]
    AtomicCut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
enum TryRollbackStructuralFindingKind {
    #[serde(rename = "missing_fallback_body")]
    MissingFallbackBody,
    #[serde(rename = "disallowed_fallback_placement")]
    DisallowedFallbackPlacement,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct TryRollbackStructuralFindingRow {
    subject_kind: TryRollbackStructuralSubjectKind,
    finding_kind: TryRollbackStructuralFindingKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticGateResult {
    pub verdict: StaticGateVerdict,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum StaticReasonCodeRow {
    MissingLineageAssertion {
        predecessor: String,
        successor: String,
    },
    LineageAssertionEdgeMismatch {
        predecessor: String,
        successor: String,
    },
    DeclaredTargetMissing {
        predecessor: String,
        successor: String,
    },
    DeclaredTargetMismatch {
        predecessor: String,
        successor: String,
    },
    CapabilityStrengthens {
        from_capability: String,
        to_capability: String,
    },
    MissingChainHeadOption {
        head: String,
        scope: String,
    },
    MissingPredecessorOption {
        option: String,
        scope: String,
    },
    MissingSuccessorOption {
        option: String,
        scope: String,
    },
}

/// current L2 の parser-free fixture root。
#[derive(Debug, Clone, Deserialize)]
pub struct CurrentL2Fixture {
    pub schema_version: String,
    pub fixture_id: String,
    pub source_example_id: String,
    pub program: Program,
    pub expected_static: ExpectedStatic,
    pub expected_runtime: ExpectedRuntime,
    pub expected_trace_audit: ExpectedTraceAudit,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Program {
    #[serde(rename = "kind")]
    pub kind: ProgramKind,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum ProgramKind {
    #[serde(rename = "Program")]
    Program,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind")]
pub enum Statement {
    PlaceBlock {
        place: String,
        body: Vec<Statement>,
    },
    PerformOn {
        op: String,
        target: String,
        contract: Contract,
    },
    PerformVia {
        op: String,
        chain_ref: String,
        contract: Contract,
    },
    OptionDecl {
        name: String,
        target: String,
        capability: String,
        lease: String,
        admit: Option<Predicate>,
    },
    ChainDecl {
        name: String,
        head: String,
        edges: Vec<ChainEdge>,
    },
    TryFallback {
        body: Vec<Statement>,
        fallback_body: Vec<Statement>,
    },
    AtomicCut,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Contract {
    pub require: Vec<Predicate>,
    pub ensure: Vec<Predicate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind")]
pub enum Predicate {
    #[serde(rename = "atom")]
    Atom { name: String },
    #[serde(rename = "call")]
    Call { callee: String, args: Vec<String> },
    #[serde(rename = "and")]
    And { terms: Vec<Predicate> },
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ChainEdge {
    pub predecessor: String,
    pub successor: String,
    pub lineage_assertion: Option<LineageAssertion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct LineageAssertion {
    pub predecessor: String,
    pub successor: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExpectedStatic {
    pub verdict: StaticGateVerdict,
    pub reasons: Vec<String>,
    #[serde(default)]
    pub checked_reasons: Option<Vec<String>>,
    #[serde(default)]
    pub checked_reason_codes: Option<Vec<StaticReasonCodeRow>>,
    #[serde(default)]
    #[allow(dead_code)]
    checked_try_rollback_structural_verdict: Option<TryRollbackStructuralVerdict>,
    #[serde(default)]
    #[allow(dead_code)]
    checked_try_rollback_structural_findings: Option<Vec<TryRollbackStructuralFindingRow>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExpectedRuntime {
    pub enters_evaluation: bool,
    pub final_outcome: FixtureRuntimeOutcome,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum FixtureRuntimeOutcome {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "explicit_failure")]
    ExplicitFailure,
    #[serde(rename = "Reject")]
    Reject,
    #[serde(rename = "not_evaluated")]
    NotEvaluated,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExpectedTraceAudit {
    pub event_kinds: Vec<EventKind>,
    pub non_admissible_metadata: Vec<ExpectedNonAdmissibleMetadata>,
    pub narrative_explanations: Vec<String>,
    pub must_explain: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ExpectedNonAdmissibleMetadata {
    pub option_ref: String,
    pub subreason: NonAdmissibleSubreason,
}

pub(crate) fn static_reason_code_rows(reasons: &[String]) -> Vec<StaticReasonCodeRow> {
    reasons
        .iter()
        .filter_map(|reason| static_reason_code_from_reason(reason))
        .collect()
}

pub(crate) fn is_supported_checked_reason_code(row: &StaticReasonCodeRow) -> bool {
    matches!(
        row,
        StaticReasonCodeRow::MissingLineageAssertion { .. }
            | StaticReasonCodeRow::LineageAssertionEdgeMismatch { .. }
            | StaticReasonCodeRow::DeclaredTargetMissing { .. }
            | StaticReasonCodeRow::DeclaredTargetMismatch { .. }
            | StaticReasonCodeRow::CapabilityStrengthens { .. }
            | StaticReasonCodeRow::MissingChainHeadOption { .. }
            | StaticReasonCodeRow::MissingPredecessorOption { .. }
            | StaticReasonCodeRow::MissingSuccessorOption { .. }
    )
}

fn static_reason_code_from_reason(reason: &str) -> Option<StaticReasonCodeRow> {
    if let Some((predecessor, successor)) =
        parse_reason_pair(reason, "missing lineage assertion for ", " -> ")
    {
        return Some(StaticReasonCodeRow::MissingLineageAssertion {
            predecessor,
            successor,
        });
    }
    if let Some((predecessor, successor)) = parse_reason_pair(
        reason,
        "lineage assertion does not describe ",
        " -> ",
    ) {
        return Some(StaticReasonCodeRow::LineageAssertionEdgeMismatch {
            predecessor,
            successor,
        });
    }
    if let Some((predecessor, successor)) = parse_reason_pair(
        reason,
        "declared access target is missing for ",
        " -> ",
    ) {
        return Some(StaticReasonCodeRow::DeclaredTargetMissing {
            predecessor,
            successor,
        });
    }
    if let Some((predecessor, successor)) = parse_reason_pair(
        reason,
        "declared access target mismatch between ",
        " and ",
    ) {
        return Some(StaticReasonCodeRow::DeclaredTargetMismatch {
            predecessor,
            successor,
        });
    }
    if let Some((from_capability, to_capability)) =
        parse_reason_pair(reason, "capability strengthens from ", " to ")
    {
        return Some(StaticReasonCodeRow::CapabilityStrengthens {
            from_capability,
            to_capability,
        });
    }
    if let Some((head, scope)) = parse_reason_pair(
        reason,
        "missing option declaration for chain head ",
        " at ",
    ) {
        return Some(StaticReasonCodeRow::MissingChainHeadOption { head, scope });
    }
    if let Some((option, scope)) =
        parse_reason_pair(reason, "missing predecessor option ", " at ")
    {
        return Some(StaticReasonCodeRow::MissingPredecessorOption { option, scope });
    }
    if let Some((option, scope)) =
        parse_reason_pair(reason, "missing successor option ", " at ")
    {
        return Some(StaticReasonCodeRow::MissingSuccessorOption { option, scope });
    }
    None
}

fn parse_reason_pair(reason: &str, prefix: &str, separator: &str) -> Option<(String, String)> {
    let rest = reason.strip_prefix(prefix)?;
    let (left, right) = rest.split_once(separator)?;
    Some((left.to_string(), right.to_string()))
}

/// current L2 の predicate oracle input。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateInput {
    pub site: PredicateSite,
    pub op: String,
    pub mode: RequestMode,
    pub place_path: Vec<String>,
    pub target: Option<String>,
    pub chain_ref: Option<String>,
    pub option_ref: Option<String>,
    pub capability: Option<String>,
    pub lease: Option<String>,
    pub predicate: Predicate,
    pub place_store: PlaceStore,
    pub tentative_place_store: Option<PlaceStore>,
}

/// current L2 の effect oracle input。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EffectInput {
    pub op: String,
    pub mode: RequestMode,
    pub place_path: Vec<String>,
    pub selected_target: String,
    pub chain_ref: Option<String>,
    pub selected_option_ref: Option<String>,
    pub capability: Option<String>,
    pub lease: Option<String>,
    pub place_store: PlaceStore,
}

/// success-side carrier の最小 skeleton。
pub trait SuccessCarrier {
    fn preview_place_store(&self, place_store: &PlaceStore) -> PlaceStore;
    fn apply_place_store(self, place_store: &mut PlaceStore);
}

/// current L2 の predicate oracle skeleton。
pub trait PredicateOracle<Input> {
    fn eval_predicate(&mut self, input: Input) -> PredicateVerdict;
}

/// current L2 の effect oracle skeleton。
pub trait EffectOracle<Input, Commit = ()> {
    fn apply_effect(&mut self, input: Input) -> EffectVerdict<Commit>;
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct NonAdmissibleMetadata {
    pub option_ref: String,
    pub subreason: NonAdmissibleSubreason,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TraceAuditSink {
    pub events: Vec<EventKind>,
    pub non_admissible_metadata: Vec<NonAdmissibleMetadata>,
    pub narrative_explanations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestFrame {
    pub op: String,
    pub mode: RequestMode,
    pub target: Option<String>,
    pub chain_ref: Option<String>,
    pub contract: Contract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainCursor {
    pub chain_ref: String,
    pub candidate_order: Vec<String>,
    pub next_index: usize,
    pub current_option_ref: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackFrame {
    pub place_anchor: String,
    pub restore_snapshot: PlaceStore,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationState {
    pub cursor_stack: Vec<CursorFrame>,
    pub place_stack: Vec<String>,
    pub place_store: PlaceStore,
    pub current_request: Option<RequestFrame>,
    pub chain_cursor: Option<ChainCursor>,
    pub rollback_stack: Vec<RollbackFrame>,
    pub trace_audit_sink: TraceAuditSink,
    pub terminal_outcome: Option<TerminalOutcome>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CursorFrame {
    pub kind: CursorFrameKind,
    pub next_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CursorFrameKind {
    Program {
        statements: Vec<Statement>,
    },
    Place {
        place: String,
        statements: Vec<Statement>,
    },
    TryBody {
        statements: Vec<Statement>,
        fallback_body: Vec<Statement>,
    },
    TryFallback {
        statements: Vec<Statement>,
    },
}

impl CursorFrame {
    fn program(statements: Vec<Statement>) -> Self {
        Self {
            kind: CursorFrameKind::Program { statements },
            next_index: 0,
        }
    }

    fn place(place: String, statements: Vec<Statement>) -> Self {
        Self {
            kind: CursorFrameKind::Place { place, statements },
            next_index: 0,
        }
    }

    fn try_body(statements: Vec<Statement>, fallback_body: Vec<Statement>) -> Self {
        Self {
            kind: CursorFrameKind::TryBody {
                statements,
                fallback_body,
            },
            next_index: 0,
        }
    }

    fn try_fallback(statements: Vec<Statement>) -> Self {
        Self {
            kind: CursorFrameKind::TryFallback { statements },
            next_index: 0,
        }
    }

    fn statements(&self) -> &[Statement] {
        match &self.kind {
            CursorFrameKind::Program { statements }
            | CursorFrameKind::Place { statements, .. }
            | CursorFrameKind::TryFallback { statements }
            | CursorFrameKind::TryBody { statements, .. } => statements,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunReport {
    pub static_verdict: StaticGateVerdict,
    pub entered_evaluation: bool,
    pub terminal_outcome: Option<TerminalOutcome>,
    pub trace_audit_sink: TraceAuditSink,
    pub steps_executed: usize,
}

#[derive(Debug)]
pub enum InterpreterError {
    Io(std::io::Error),
    Json(serde_json::Error),
    StaticRejected(StaticGateResult),
    MissingDeclaration(String),
    InvalidProgram(String),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "io error: {err}"),
            Self::Json(err) => write!(f, "json error: {err}"),
            Self::StaticRejected(result) => {
                write!(f, "static gate rejected fixture as {:?}", result.verdict)
            }
            Self::MissingDeclaration(msg) | Self::InvalidProgram(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for InterpreterError {}

impl From<std::io::Error> for InterpreterError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for InterpreterError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

#[derive(Debug, Clone)]
struct DeclarationIndex {
    options_by_scope: HashMap<ScopePath, HashMap<String, OptionDeclView>>,
    chains_by_scope: HashMap<ScopePath, HashMap<String, ChainDeclView>>,
    duplicate_reasons: Vec<String>,
}

#[derive(Debug, Clone)]
struct OptionDeclView {
    name: String,
    target: String,
    capability: String,
    lease: String,
    admit: Option<Predicate>,
}

#[derive(Debug, Clone)]
struct ChainDeclView {
    head: String,
    edges: Vec<ChainEdge>,
}

pub struct DirectStyleEvaluator {
    declarations: DeclarationIndex,
    pub state: EvaluationState,
}

impl DirectStyleEvaluator {
    pub fn from_fixture(fixture: &CurrentL2Fixture) -> Result<Self, InterpreterError> {
        let gate = static_gate_detailed(fixture);
        if gate.verdict != StaticGateVerdict::Valid {
            return Err(InterpreterError::StaticRejected(gate));
        }

        Ok(Self {
            declarations: DeclarationIndex::from_program(&fixture.program),
            state: EvaluationState {
                cursor_stack: vec![CursorFrame::program(fixture.program.body.clone())],
                place_stack: Vec::new(),
                place_store: PlaceStore::new(),
                current_request: None,
                chain_cursor: None,
                rollback_stack: Vec::new(),
                trace_audit_sink: TraceAuditSink::default(),
                terminal_outcome: None,
            },
        })
    }

    pub fn step_once<P, E, Commit>(
        &mut self,
        predicate_oracle: &mut P,
        effect_oracle: &mut E,
    ) -> Result<StepControl, InterpreterError>
    where
        P: PredicateOracle<PredicateInput>,
        E: EffectOracle<EffectInput, Commit>,
        Commit: SuccessCarrier,
    {
        if let Some(outcome) = self.state.terminal_outcome {
            return Ok(StepControl::Halt(outcome));
        }

        if let Some(control) = self.finish_completed_frames() {
            return Ok(control);
        }

        let statement = {
            let frame =
                self.state.cursor_stack.last_mut().ok_or_else(|| {
                    InterpreterError::InvalidProgram("cursor_stack is empty".into())
                })?;
            let statement = frame
                .statements()
                .get(frame.next_index)
                .ok_or_else(|| InterpreterError::InvalidProgram("cursor out of bounds".into()))?
                .clone();
            frame.next_index += 1;
            statement
        };

        match statement {
            Statement::PlaceBlock { place, body } => {
                self.state.place_stack.push(place.clone());
                self.state
                    .cursor_stack
                    .push(CursorFrame::place(place, body));
                Ok(StepControl::Continue)
            }
            Statement::OptionDecl { .. } | Statement::ChainDecl { .. } => Ok(StepControl::Continue),
            Statement::AtomicCut => {
                self.state
                    .trace_audit_sink
                    .events
                    .push(EventKind::AtomicCut);
                let current_place = self.current_place().cloned();
                if let (Some(frame), Some(place)) = (
                    self.state.rollback_stack.last_mut(),
                    current_place.as_deref(),
                ) {
                    if frame.place_anchor == *place {
                        frame.restore_snapshot = self.state.place_store.clone();
                    }
                }
                Ok(StepControl::Continue)
            }
            Statement::TryFallback {
                body,
                fallback_body,
            } => {
                let place_anchor = self
                    .current_place()
                    .ok_or_else(|| {
                        InterpreterError::InvalidProgram(
                            "TryFallback requires an enclosing place".into(),
                        )
                    })?
                    .to_owned();
                self.state.rollback_stack.push(RollbackFrame {
                    place_anchor,
                    restore_snapshot: self.state.place_store.clone(),
                });
                self.state
                    .cursor_stack
                    .push(CursorFrame::try_body(body, fallback_body));
                Ok(StepControl::Continue)
            }
            Statement::PerformOn {
                op,
                target,
                contract,
            } => self.eval_perform_on(op, target, contract, predicate_oracle, effect_oracle),
            Statement::PerformVia {
                op,
                chain_ref,
                contract,
            } => self.eval_perform_via(op, chain_ref, contract, predicate_oracle, effect_oracle),
        }
    }

    fn finish_completed_frames(&mut self) -> Option<StepControl> {
        loop {
            let is_complete = self
                .state
                .cursor_stack
                .last()
                .is_some_and(|frame| frame.next_index >= frame.statements().len());

            if !is_complete {
                return None;
            }

            let frame = self.state.cursor_stack.pop()?;
            match frame.kind {
                CursorFrameKind::Program { .. } => {
                    let outcome = self
                        .state
                        .terminal_outcome
                        .unwrap_or(TerminalOutcome::Success);
                    self.state.terminal_outcome = Some(outcome);
                    return Some(StepControl::Halt(outcome));
                }
                CursorFrameKind::Place { .. } => {
                    self.state.place_stack.pop();
                }
                CursorFrameKind::TryBody { .. } | CursorFrameKind::TryFallback { .. } => {
                    self.state.rollback_stack.pop();
                }
            }
        }
    }

    fn eval_perform_on<P, E, Commit>(
        &mut self,
        op: String,
        target: String,
        contract: Contract,
        predicate_oracle: &mut P,
        effect_oracle: &mut E,
    ) -> Result<StepControl, InterpreterError>
    where
        P: PredicateOracle<PredicateInput>,
        E: EffectOracle<EffectInput, Commit>,
        Commit: SuccessCarrier,
    {
        self.state.current_request = Some(RequestFrame {
            op: op.clone(),
            mode: RequestMode::On,
            target: Some(target.clone()),
            chain_ref: None,
            contract: contract.clone(),
        });

        if !self.eval_predicate_list(
            predicate_oracle,
            PredicateSite::RequestRequire,
            &op,
            RequestMode::On,
            Some(&target),
            None,
            None,
            &contract.require,
            None,
        ) {
            self.state
                .trace_audit_sink
                .events
                .push(EventKind::PerformFailure);
            self.state.current_request = None;
            return Ok(self.propagate_failure(FailureKind::ExplicitFailure));
        }

        let effect_input = EffectInput {
            op: op.clone(),
            mode: RequestMode::On,
            place_path: self.state.place_stack.clone(),
            selected_target: target.clone(),
            chain_ref: None,
            selected_option_ref: None,
            capability: None,
            lease: None,
            place_store: self.state.place_store.clone(),
        };

        match effect_oracle.apply_effect(effect_input) {
            EffectVerdict::ExplicitFailure => {
                self.state
                    .trace_audit_sink
                    .events
                    .push(EventKind::PerformFailure);
                self.state.current_request = None;
                Ok(self.propagate_failure(FailureKind::ExplicitFailure))
            }
            EffectVerdict::Success { commit } => {
                let preview = commit.preview_place_store(&self.state.place_store);
                let ensure_ok = self.eval_predicate_list(
                    predicate_oracle,
                    PredicateSite::RequestEnsure,
                    &op,
                    RequestMode::On,
                    Some(&target),
                    None,
                    None,
                    &contract.ensure,
                    Some(&preview),
                );
                if !ensure_ok {
                    self.state
                        .trace_audit_sink
                        .events
                        .push(EventKind::PerformFailure);
                    self.state.current_request = None;
                    return Ok(self.propagate_failure(FailureKind::ExplicitFailure));
                }

                commit.apply_place_store(&mut self.state.place_store);
                self.state
                    .trace_audit_sink
                    .events
                    .push(EventKind::PerformSuccess);
                self.state.current_request = None;
                Ok(StepControl::Continue)
            }
        }
    }

    fn eval_perform_via<P, E, Commit>(
        &mut self,
        op: String,
        chain_ref: String,
        contract: Contract,
        predicate_oracle: &mut P,
        effect_oracle: &mut E,
    ) -> Result<StepControl, InterpreterError>
    where
        P: PredicateOracle<PredicateInput>,
        E: EffectOracle<EffectInput, Commit>,
        Commit: SuccessCarrier,
    {
        self.state.current_request = Some(RequestFrame {
            op: op.clone(),
            mode: RequestMode::Via,
            target: None,
            chain_ref: Some(chain_ref.clone()),
            contract: contract.clone(),
        });

        let current_scope = self.state.place_stack.clone();
        let candidate_order = self.resolve_chain_order(&current_scope, &chain_ref)?;
        self.state.chain_cursor = Some(ChainCursor {
            chain_ref: chain_ref.clone(),
            candidate_order: candidate_order.clone(),
            next_index: 0,
            current_option_ref: candidate_order.first().cloned(),
        });

        for (idx, option_name) in candidate_order.iter().enumerate() {
            if let Some(chain_cursor) = self.state.chain_cursor.as_mut() {
                chain_cursor.next_index = idx;
                chain_cursor.current_option_ref = Some(option_name.clone());
            }

            let option = self
                .declarations
                .resolve_option_in_scope(&current_scope, option_name)
                .cloned()
                .ok_or_else(|| {
                    InterpreterError::MissingDeclaration(format!(
                        "missing option declaration for {option_name} in current place visibility"
                    ))
                })?;

            if option.lease == "expired" {
                self.state
                    .trace_audit_sink
                    .non_admissible_metadata
                    .push(NonAdmissibleMetadata {
                        option_ref: option.name.clone(),
                        subreason: NonAdmissibleSubreason::LeaseExpired,
                    });
                continue;
            }

            if !self.capability_matches_request(&contract.require, &option.capability) {
                self.state
                    .trace_audit_sink
                    .narrative_explanations
                    .push(format!(
                        "{} remains a request/capability mismatch narrative explanation",
                        option.name
                    ));
                continue;
            }

            if let Some(admit) = option.admit.clone() {
                let admit_ok = self.eval_single_predicate(
                    predicate_oracle,
                    PredicateSite::OptionAdmit,
                    &op,
                    RequestMode::Via,
                    Some(&option.target),
                    Some(&chain_ref),
                    Some(&option),
                    admit,
                    None,
                );
                if !admit_ok {
                    self.state.trace_audit_sink.non_admissible_metadata.push(
                        NonAdmissibleMetadata {
                            option_ref: option.name.clone(),
                            subreason: NonAdmissibleSubreason::AdmitMiss,
                        },
                    );
                    continue;
                }
            }

            let require_ok = self.eval_predicate_list(
                predicate_oracle,
                PredicateSite::RequestRequire,
                &op,
                RequestMode::Via,
                Some(&option.target),
                Some(&chain_ref),
                Some(&option),
                &contract.require,
                None,
            );
            if !require_ok {
                self.state
                    .trace_audit_sink
                    .events
                    .push(EventKind::PerformFailure);
                if idx + 1 < candidate_order.len() {
                    continue;
                }
                self.state.trace_audit_sink.events.push(EventKind::Reject);
                self.state.current_request = None;
                self.state.chain_cursor = None;
                return Ok(self.propagate_failure(FailureKind::Reject));
            }

            let effect_input = EffectInput {
                op: op.clone(),
                mode: RequestMode::Via,
                place_path: self.state.place_stack.clone(),
                selected_target: option.target.clone(),
                chain_ref: Some(chain_ref.clone()),
                selected_option_ref: Some(option.name.clone()),
                capability: Some(option.capability.clone()),
                lease: Some(option.lease.clone()),
                place_store: self.state.place_store.clone(),
            };

            match effect_oracle.apply_effect(effect_input) {
                EffectVerdict::ExplicitFailure => {
                    self.state
                        .trace_audit_sink
                        .events
                        .push(EventKind::PerformFailure);
                    if idx + 1 < candidate_order.len() {
                        continue;
                    }
                    self.state.trace_audit_sink.events.push(EventKind::Reject);
                    self.state.current_request = None;
                    self.state.chain_cursor = None;
                    return Ok(self.propagate_failure(FailureKind::Reject));
                }
                EffectVerdict::Success { commit } => {
                    let preview = commit.preview_place_store(&self.state.place_store);
                    let ensure_ok = self.eval_predicate_list(
                        predicate_oracle,
                        PredicateSite::RequestEnsure,
                        &op,
                        RequestMode::Via,
                        Some(&option.target),
                        Some(&chain_ref),
                        Some(&option),
                        &contract.ensure,
                        Some(&preview),
                    );
                    if !ensure_ok {
                        self.state
                            .trace_audit_sink
                            .events
                            .push(EventKind::PerformFailure);
                        if idx + 1 < candidate_order.len() {
                            continue;
                        }
                        self.state.trace_audit_sink.events.push(EventKind::Reject);
                        self.state.current_request = None;
                        self.state.chain_cursor = None;
                        return Ok(self.propagate_failure(FailureKind::Reject));
                    }

                    commit.apply_place_store(&mut self.state.place_store);
                    self.state
                        .trace_audit_sink
                        .events
                        .push(EventKind::PerformSuccess);
                    self.state.current_request = None;
                    self.state.chain_cursor = None;
                    return Ok(StepControl::Continue);
                }
            }
        }

        self.state.trace_audit_sink.events.push(EventKind::Reject);
        self.state.current_request = None;
        self.state.chain_cursor = None;
        Ok(self.propagate_failure(FailureKind::Reject))
    }

    fn eval_predicate_list<P>(
        &self,
        predicate_oracle: &mut P,
        site: PredicateSite,
        op: &str,
        mode: RequestMode,
        target: Option<&str>,
        chain_ref: Option<&str>,
        option: Option<&OptionDeclView>,
        predicates: &[Predicate],
        tentative_place_store: Option<&PlaceStore>,
    ) -> bool
    where
        P: PredicateOracle<PredicateInput>,
    {
        predicates.iter().cloned().all(|predicate| {
            self.eval_single_predicate(
                predicate_oracle,
                site,
                op,
                mode,
                target,
                chain_ref,
                option,
                predicate,
                tentative_place_store,
            )
        })
    }

    fn eval_single_predicate<P>(
        &self,
        predicate_oracle: &mut P,
        site: PredicateSite,
        op: &str,
        mode: RequestMode,
        target: Option<&str>,
        chain_ref: Option<&str>,
        option: Option<&OptionDeclView>,
        predicate: Predicate,
        tentative_place_store: Option<&PlaceStore>,
    ) -> bool
    where
        P: PredicateOracle<PredicateInput>,
    {
        let input = PredicateInput {
            site,
            op: op.to_owned(),
            mode,
            place_path: self.state.place_stack.clone(),
            target: target.map(str::to_owned),
            chain_ref: chain_ref.map(str::to_owned),
            option_ref: option.map(|option| option.name.clone()),
            capability: option.map(|option| option.capability.clone()),
            lease: option.map(|option| option.lease.clone()),
            predicate,
            place_store: self.state.place_store.clone(),
            tentative_place_store: tentative_place_store.cloned(),
        };
        predicate_oracle.eval_predicate(input) == PredicateVerdict::Satisfied
    }

    fn capability_matches_request(&self, predicates: &[Predicate], capability: &str) -> bool {
        let required_modes = collect_mode_atoms(predicates);
        required_modes
            .iter()
            .all(|mode| capability_allows(capability, mode))
    }

    fn resolve_chain_order(
        &self,
        scope: &[String],
        chain_ref: &str,
    ) -> Result<Vec<String>, InterpreterError> {
        let chain = self
            .declarations
            .resolve_chain_in_scope(scope, chain_ref)
            .ok_or_else(|| {
                InterpreterError::MissingDeclaration(format!(
                    "missing chain declaration for {chain_ref} in current place visibility"
                ))
            })?;

        let mut order = vec![chain.head.clone()];
        for edge in &chain.edges {
            order.push(edge.successor.clone());
        }
        Ok(order)
    }

    fn current_place(&self) -> Option<&String> {
        self.state.place_stack.last()
    }

    fn propagate_failure(&mut self, kind: FailureKind) -> StepControl {
        loop {
            let Some(frame) = self.state.cursor_stack.pop() else {
                let outcome = TerminalOutcome::from(kind);
                self.state.terminal_outcome = Some(outcome);
                return StepControl::Halt(outcome);
            };

            match frame.kind {
                CursorFrameKind::Program { .. } => {
                    let outcome = TerminalOutcome::from(kind);
                    self.state.terminal_outcome = Some(outcome);
                    return StepControl::Halt(outcome);
                }
                CursorFrameKind::Place { .. } => {
                    self.state.place_stack.pop();
                }
                CursorFrameKind::TryBody { fallback_body, .. } => {
                    if let Some(frame) = self.state.rollback_stack.last() {
                        self.state.place_store = frame.restore_snapshot.clone();
                    }
                    self.state.trace_audit_sink.events.push(EventKind::Rollback);
                    self.state
                        .cursor_stack
                        .push(CursorFrame::try_fallback(fallback_body));
                    return StepControl::Continue;
                }
                CursorFrameKind::TryFallback { .. } => {
                    self.state.rollback_stack.pop();
                }
            }
        }
    }
}

impl DeclarationIndex {
    fn from_program(program: &Program) -> Self {
        let mut index = Self {
            options_by_scope: HashMap::new(),
            chains_by_scope: HashMap::new(),
            duplicate_reasons: Vec::new(),
        };
        let mut scope = ScopePath::new();
        index.collect_statements(&program.body, &mut scope);
        index
    }

    fn collect_statements(&mut self, statements: &[Statement], scope: &mut ScopePath) {
        for statement in statements {
            match statement {
                Statement::PlaceBlock { place, body } => {
                    scope.push(place.clone());
                    self.collect_statements(body, scope);
                    scope.pop();
                }
                Statement::TryFallback {
                    body,
                    fallback_body,
                } => {
                    self.collect_statements(body, scope);
                    self.collect_statements(fallback_body, scope);
                }
                Statement::OptionDecl {
                    name,
                    target,
                    capability,
                    lease,
                    admit,
                } => {
                    if let Some(existing_scope) = self.find_visible_option_scope(scope, name) {
                        self.duplicate_reasons.push(format!(
                            "duplicate option declaration {name} is visible from {} and {}",
                            display_scope(&existing_scope),
                            display_scope(scope),
                        ));
                    }
                    self.options_by_scope
                        .entry(scope.clone())
                        .or_default()
                        .insert(
                            name.clone(),
                            OptionDeclView {
                                name: name.clone(),
                                target: target.clone(),
                                capability: capability.clone(),
                                lease: lease.clone(),
                                admit: admit.clone(),
                            },
                        );
                }
                Statement::ChainDecl { name, head, edges } => {
                    if let Some(existing_scope) = self.find_visible_chain_scope(scope, name) {
                        self.duplicate_reasons.push(format!(
                            "duplicate chain declaration {name} is visible from {} and {}",
                            display_scope(&existing_scope),
                            display_scope(scope),
                        ));
                    }
                    self.chains_by_scope
                        .entry(scope.clone())
                        .or_default()
                        .insert(
                            name.clone(),
                            ChainDeclView {
                                head: head.clone(),
                                edges: edges.clone(),
                            },
                        );
                }
                Statement::PerformOn { .. }
                | Statement::PerformVia { .. }
                | Statement::AtomicCut => {}
            }
        }
    }

    fn duplicate_reasons(&self) -> &[String] {
        &self.duplicate_reasons
    }

    fn resolve_option_in_scope(&self, scope: &[String], name: &str) -> Option<&OptionDeclView> {
        self.visible_scope_paths(scope)
            .find_map(|path| self.options_by_scope.get(&path)?.get(name))
    }

    fn resolve_chain_in_scope(&self, scope: &[String], name: &str) -> Option<&ChainDeclView> {
        self.visible_scope_paths(scope)
            .find_map(|path| self.chains_by_scope.get(&path)?.get(name))
    }

    fn visible_scope_paths(&self, scope: &[String]) -> impl Iterator<Item = ScopePath> + '_ {
        let owned_scope = scope.to_vec();
        (0..=owned_scope.len())
            .rev()
            .map(move |depth| owned_scope[..depth].to_vec())
    }

    fn find_visible_option_scope(&self, scope: &[String], name: &str) -> Option<ScopePath> {
        self.visible_scope_paths(scope).find(|path| {
            self.options_by_scope
                .get(path)
                .is_some_and(|entries| entries.contains_key(name))
        })
    }

    fn find_visible_chain_scope(&self, scope: &[String], name: &str) -> Option<ScopePath> {
        self.visible_scope_paths(scope).find(|path| {
            self.chains_by_scope
                .get(path)
                .is_some_and(|entries| entries.contains_key(name))
        })
    }
}

pub fn load_fixture_from_path(
    path: impl AsRef<Path>,
) -> Result<CurrentL2Fixture, InterpreterError> {
    let text = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&text)?)
}

pub fn static_gate(fixture: &CurrentL2Fixture) -> StaticGateVerdict {
    static_gate_detailed(fixture).verdict
}

pub fn static_gate_detailed(fixture: &CurrentL2Fixture) -> StaticGateResult {
    let declarations = DeclarationIndex::from_program(&fixture.program);
    let mut verdict = StaticGateVerdict::Valid;
    let mut reasons = Vec::new();

    for reason in declarations.duplicate_reasons() {
        verdict = escalate_verdict(verdict, StaticGateVerdict::Malformed);
        reasons.push(reason.clone());
    }

    for (scope, chains) in &declarations.chains_by_scope {
        for chain in chains.values() {
            if declarations
                .resolve_option_in_scope(scope, &chain.head)
                .is_none()
            {
                verdict = escalate_verdict(verdict, StaticGateVerdict::Malformed);
                reasons.push(format!(
                    "missing option declaration for chain head {} at {}",
                    chain.head,
                    display_scope(scope),
                ));
            }

            for edge in &chain.edges {
                let Some(predecessor) =
                    declarations.resolve_option_in_scope(scope, &edge.predecessor)
                else {
                    verdict = escalate_verdict(verdict, StaticGateVerdict::Malformed);
                    reasons.push(format!(
                        "missing predecessor option {} at {}",
                        edge.predecessor,
                        display_scope(scope),
                    ));
                    continue;
                };
                let Some(successor) = declarations.resolve_option_in_scope(scope, &edge.successor)
                else {
                    verdict = escalate_verdict(verdict, StaticGateVerdict::Malformed);
                    reasons.push(format!(
                        "missing successor option {} at {}",
                        edge.successor,
                        display_scope(scope),
                    ));
                    continue;
                };

                match &edge.lineage_assertion {
                    None => {
                        verdict = escalate_verdict(verdict, StaticGateVerdict::Underdeclared);
                        reasons.push(format!(
                            "missing lineage assertion for {} -> {}",
                            edge.predecessor, edge.successor
                        ));
                    }
                    Some(assertion)
                        if assertion.predecessor != edge.predecessor
                            || assertion.successor != edge.successor =>
                    {
                        verdict = escalate_verdict(verdict, StaticGateVerdict::Malformed);
                        reasons.push(format!(
                            "lineage assertion does not describe {} -> {}",
                            edge.predecessor, edge.successor
                        ));
                    }
                    Some(_) => {}
                }

                if predecessor.target.is_empty() || successor.target.is_empty() {
                    verdict = escalate_verdict(verdict, StaticGateVerdict::Underdeclared);
                    reasons.push(format!(
                        "declared access target is missing for {} -> {}",
                        edge.predecessor, edge.successor
                    ));
                } else if predecessor.target != successor.target {
                    verdict = escalate_verdict(verdict, StaticGateVerdict::Malformed);
                    reasons.push(format!(
                        "declared access target mismatch between {} and {}",
                        edge.predecessor, edge.successor
                    ));
                }

                if capability_rank(&successor.capability) > capability_rank(&predecessor.capability)
                {
                    verdict = escalate_verdict(verdict, StaticGateVerdict::Malformed);
                    reasons.push(format!(
                        "capability strengthens from {} to {}",
                        predecessor.capability, successor.capability
                    ));
                }
            }
        }
    }

    collect_try_rollback_structural_reasons(&fixture.program.body, false, &mut reasons);
    if reasons.iter().any(|reason| {
        reason == "try fallback body must not be empty"
            || reason == "atomic cut may not appear inside fallback body"
    }) {
        verdict = escalate_verdict(verdict, StaticGateVerdict::Malformed);
    }

    reasons.sort();

    StaticGateResult { verdict, reasons }
}

fn collect_try_rollback_structural_reasons(
    statements: &[Statement],
    in_fallback_body: bool,
    reasons: &mut Vec<String>,
) {
    for statement in statements {
        match statement {
            Statement::PlaceBlock { body, .. } => {
                collect_try_rollback_structural_reasons(body, in_fallback_body, reasons);
            }
            Statement::TryFallback { body, fallback_body } => {
                if fallback_body.is_empty() {
                    reasons.push("try fallback body must not be empty".to_string());
                }
                collect_try_rollback_structural_reasons(body, in_fallback_body, reasons);
                collect_try_rollback_structural_reasons(fallback_body, true, reasons);
            }
            Statement::AtomicCut if in_fallback_body => {
                reasons.push("atomic cut may not appear inside fallback body".to_string());
            }
            Statement::PerformOn { .. }
            | Statement::PerformVia { .. }
            | Statement::OptionDecl { .. }
            | Statement::ChainDecl { .. }
            | Statement::AtomicCut => {}
        }
    }
}

pub fn run_to_completion<P, E, Commit>(
    fixture: &CurrentL2Fixture,
    predicate_oracle: &mut P,
    effect_oracle: &mut E,
) -> Result<RunReport, InterpreterError>
where
    P: PredicateOracle<PredicateInput>,
    E: EffectOracle<EffectInput, Commit>,
    Commit: SuccessCarrier,
{
    let gate = static_gate_detailed(fixture);
    if gate.verdict != StaticGateVerdict::Valid {
        return Ok(RunReport {
            static_verdict: gate.verdict,
            entered_evaluation: false,
            terminal_outcome: None,
            trace_audit_sink: TraceAuditSink::default(),
            steps_executed: 0,
        });
    }

    let mut evaluator = DirectStyleEvaluator::from_fixture(fixture)?;
    let mut steps = 0usize;

    loop {
        steps += 1;
        let control = evaluator.step_once(predicate_oracle, effect_oracle)?;
        if let StepControl::Halt(outcome) = control {
            return Ok(RunReport {
                static_verdict: StaticGateVerdict::Valid,
                entered_evaluation: true,
                terminal_outcome: Some(outcome),
                trace_audit_sink: evaluator.state.trace_audit_sink,
                steps_executed: steps,
            });
        }
    }
}

fn collect_mode_atoms(predicates: &[Predicate]) -> Vec<String> {
    let mut atoms = Vec::new();
    for predicate in predicates {
        collect_mode_atoms_inner(predicate, &mut atoms);
    }
    atoms
}

fn collect_mode_atoms_inner(predicate: &Predicate, atoms: &mut Vec<String>) {
    match predicate {
        Predicate::Atom { name } if name == "read" || name == "write" || name == "append" => {
            atoms.push(name.clone());
        }
        Predicate::And { terms } => {
            for term in terms {
                collect_mode_atoms_inner(term, atoms);
            }
        }
        Predicate::Atom { .. } | Predicate::Call { .. } => {}
    }
}

fn capability_allows(capability: &str, requested: &str) -> bool {
    match requested {
        "read" => capability == "read" || capability == "write",
        "write" => capability == "write",
        "append" => capability == "append" || capability == "write",
        _ => true,
    }
}

fn capability_rank(capability: &str) -> u8 {
    match capability {
        "write" => 2,
        "append" => 1,
        "read" => 0,
        _ => 0,
    }
}

fn escalate_verdict(current: StaticGateVerdict, incoming: StaticGateVerdict) -> StaticGateVerdict {
    match (current, incoming) {
        (StaticGateVerdict::Malformed, _) | (_, StaticGateVerdict::Malformed) => {
            StaticGateVerdict::Malformed
        }
        (StaticGateVerdict::Underdeclared, _) | (_, StaticGateVerdict::Underdeclared) => {
            StaticGateVerdict::Underdeclared
        }
        _ => StaticGateVerdict::Valid,
    }
}

fn display_scope(scope: &[String]) -> String {
    if scope.is_empty() {
        "<root>".into()
    } else {
        scope.join(" / ")
    }
}

pub fn crate_name() -> &'static str {
    "mir_semantics"
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fixture_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../mir-ast/tests/fixtures/current-l2")
            .join(name)
    }

    #[test]
    fn try_rollback_structural_first_tranche_fixtures_load_with_dedicated_expected_fields() {
        let missing_fallback_body = load_fixture_from_path(&fixture_path(
            "e23-malformed-try-fallback-missing-fallback-body.json",
        ))
        .expect("fixture should load");
        assert_eq!(
            missing_fallback_body.fixture_id,
            "e23_malformed_try_fallback_missing_fallback_body"
        );
        assert_eq!(
            missing_fallback_body
                .expected_static
                .checked_try_rollback_structural_verdict,
            Some(TryRollbackStructuralVerdict::FindingsPresent)
        );
        assert_eq!(
            missing_fallback_body
                .expected_static
                .checked_try_rollback_structural_findings,
            Some(vec![TryRollbackStructuralFindingRow {
                subject_kind: TryRollbackStructuralSubjectKind::TryFallback,
                finding_kind: TryRollbackStructuralFindingKind::MissingFallbackBody,
            }])
        );

        let atomic_cut_fallback_placement = load_fixture_from_path(&fixture_path(
            "e24-malformed-atomic-cut-fallback-placement.json",
        ))
        .expect("fixture should load");
        assert_eq!(
            atomic_cut_fallback_placement.fixture_id,
            "e24_malformed_atomic_cut_fallback_placement"
        );
        assert_eq!(
            atomic_cut_fallback_placement
                .expected_static
                .checked_try_rollback_structural_verdict,
            Some(TryRollbackStructuralVerdict::FindingsPresent)
        );
        assert_eq!(
            atomic_cut_fallback_placement
                .expected_static
                .checked_try_rollback_structural_findings,
            Some(vec![TryRollbackStructuralFindingRow {
                subject_kind: TryRollbackStructuralSubjectKind::AtomicCut,
                finding_kind: TryRollbackStructuralFindingKind::DisallowedFallbackPlacement,
            }])
        );
    }
}
