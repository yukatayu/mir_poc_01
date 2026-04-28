use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fmt, fs,
    path::{Path, PathBuf},
};

use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanNearEndError {
    message: String,
}

impl CleanNearEndError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for CleanNearEndError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for CleanNearEndError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CleanSampleFamily {
    Typing,
    OrderHandoff,
    ModelCheck,
    Modal,
}

impl CleanSampleFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Typing => "typing",
            Self::OrderHandoff => "order-handoff",
            Self::ModelCheck => "model-check",
            Self::Modal => "modal",
        }
    }

    pub fn parse(value: &str) -> Result<Self, CleanNearEndError> {
        match value.trim() {
            "typing" => Ok(Self::Typing),
            "order-handoff" => Ok(Self::OrderHandoff),
            "model-check" => Ok(Self::ModelCheck),
            "modal" => Ok(Self::Modal),
            other => Err(CleanNearEndError::new(format!(
                "unknown clean near-end family `{other}`"
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct IndexTheoryDecl {
    pub name: String,
    pub kind: String,
    pub elements: Vec<IndexElementDecl>,
    pub orders: Vec<IndexOrderDecl>,
    pub laws: Vec<IndexLawDecl>,
    pub counters: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IndexElementDecl {
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IndexOrderDecl {
    pub lower: String,
    pub upper: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct IndexLawDecl {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct FinitePreorder {
    pub name: String,
    elements: BTreeSet<String>,
    closure: BTreeSet<(String, String)>,
}

impl FinitePreorder {
    pub fn new(
        name: impl Into<String>,
        elements: impl IntoIterator<Item = impl Into<String>>,
        orders: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        let name = name.into();
        let elements = elements
            .into_iter()
            .map(Into::into)
            .collect::<BTreeSet<_>>();
        let mut closure = BTreeSet::new();
        for element in &elements {
            closure.insert((element.clone(), element.clone()));
        }
        for (lower, upper) in orders {
            closure.insert((lower.into(), upper.into()));
        }

        let mut changed = true;
        while changed {
            changed = false;
            let snapshot = closure.iter().cloned().collect::<Vec<_>>();
            for (a, b) in &snapshot {
                for (c, d) in &snapshot {
                    if b == c && closure.insert((a.clone(), d.clone())) {
                        changed = true;
                    }
                }
            }
        }

        Self {
            name,
            elements,
            closure,
        }
    }

    pub fn leq(&self, lower: &str, upper: &str) -> Option<bool> {
        if !(self.elements.contains(lower) && self.elements.contains(upper)) {
            return None;
        }
        Some(
            self.closure
                .contains(&(lower.to_string(), upper.to_string())),
        )
    }
}

#[derive(Debug, Clone)]
pub struct FiniteLattice {
    pub preorder: FinitePreorder,
}

#[derive(Debug, Clone)]
pub struct FinitePowerset {
    pub name: String,
    elements: BTreeSet<String>,
}

impl FinitePowerset {
    pub fn new(
        name: impl Into<String>,
        elements: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            elements: elements.into_iter().map(Into::into).collect(),
        }
    }

    pub fn subset(&self, lhs: &CaptureSet, rhs: &CaptureSet) -> Option<bool> {
        if !(lhs.0.is_subset(&self.elements) && rhs.0.is_subset(&self.elements)) {
            return None;
        }
        Some(lhs.0.is_subset(&rhs.0))
    }
}

#[derive(Debug, Clone)]
pub struct RegionPreorder {
    pub preorder: FinitePreorder,
}

#[derive(Debug, Clone)]
pub struct CaptureSet(pub BTreeSet<String>);

impl CaptureSet {
    pub fn from_slice(values: &[&str]) -> Self {
        Self(values.iter().map(|value| (*value).to_string()).collect())
    }
}

#[derive(Debug, Clone, Default)]
pub struct SimpleCostBound(pub BTreeMap<String, u64>);

impl SimpleCostBound {
    pub fn from_pairs(values: &[(&str, u64)]) -> Self {
        Self(
            values
                .iter()
                .map(|(key, value)| ((*key).to_string(), *value))
                .collect(),
        )
    }

    pub fn leq(&self, other: &Self) -> bool {
        let keys = self
            .0
            .keys()
            .chain(other.0.keys())
            .cloned()
            .collect::<BTreeSet<_>>();
        keys.into_iter().all(|key| {
            let lhs = self.0.get(&key).copied().unwrap_or_default();
            let rhs = other.0.get(&key).copied().unwrap_or_default();
            lhs <= rhs
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintKind {
    LabelFlow,
    AuthorityGe,
    CaptureSubset,
    RegionOutlives,
    CostLeq,
    EffectRowSubset,
    RequiresWitness,
    RequiresPublication,
    PolicyPermits,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConstraintSourceRef {
    pub sample: String,
    pub location: String,
}

#[derive(Debug, Clone)]
enum ConstraintEvaluation {
    LabelFlow {
        theory: String,
        lower: String,
        upper: String,
    },
    AuthorityGe {
        principal: String,
        theory: String,
        required: String,
    },
    CaptureSubset {
        theory: String,
        lhs: CaptureSet,
        rhs: CaptureSet,
    },
    RegionOutlives {
        theory: String,
        longer: String,
        shorter: String,
    },
    CostLeq {
        lhs: SimpleCostBound,
        rhs: SimpleCostBound,
    },
    EffectRowSubset {
        lhs: BTreeSet<String>,
        rhs: BTreeSet<String>,
    },
    RequiresWitness {
        witness: String,
        producer_stage: String,
        consumer_stage: String,
    },
    RequiresPublication {
        publication_stage: String,
        consumer_stage: String,
    },
    PolicyPermits {
        policy: String,
        principal: String,
        authority_theory: String,
        required_authority: String,
        from_label: String,
        to_label: String,
    },
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub kind: ConstraintKind,
    pub display: String,
    pub source_ref: ConstraintSourceRef,
    evaluation: ConstraintEvaluation,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResidualObligation {
    pub kind: String,
    pub subject_ref: String,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstraintVerdict {
    Valid,
    Invalid,
    Residual,
}

#[derive(Debug, Clone)]
pub struct ConstraintSolverResult {
    pub verdict: ConstraintVerdict,
    pub solved: Vec<String>,
    pub failed: Vec<String>,
    pub residuals: Vec<ResidualObligation>,
}

pub struct ConstraintSolver {
    label_theories: HashMap<String, FiniteLattice>,
    authority_theories: HashMap<String, FinitePreorder>,
    capture_theories: HashMap<String, FinitePowerset>,
    region_theories: HashMap<String, RegionPreorder>,
    policies: HashSet<(String, String, String, String)>,
    principals: HashMap<String, (String, String)>,
}

impl ConstraintSolver {
    fn new(sample: &CleanNearEndSampleSpec) -> Self {
        let mut label_theories = HashMap::new();
        let mut authority_theories = HashMap::new();
        let mut capture_theories = HashMap::new();
        let mut region_theories = HashMap::new();

        label_theories.insert(
            "SecurityLabel".to_string(),
            FiniteLattice {
                preorder: FinitePreorder::new(
                    "SecurityLabel",
                    ["Public", "UserSecret", "KeyMaterial"],
                    [("Public", "UserSecret"), ("UserSecret", "KeyMaterial")],
                ),
            },
        );
        authority_theories.insert(
            "FingerprintAuthority".to_string(),
            FinitePreorder::new(
                "FingerprintAuthority",
                ["Observer", "Holder", "Releaser", "Admin"],
                [
                    ("Observer", "Holder"),
                    ("Holder", "Releaser"),
                    ("Releaser", "Admin"),
                ],
            ),
        );
        capture_theories.insert(
            "CaptureScope".to_string(),
            FinitePowerset::new(
                "CaptureScope",
                ["RoomHistory", "EphemeralToken", "SecretKeyStore"],
            ),
        );
        region_theories.insert(
            "Region".to_string(),
            RegionPreorder {
                preorder: FinitePreorder::new(
                    "Region",
                    ["Step", "Turn", "Session"],
                    [("Step", "Turn"), ("Turn", "Session")],
                ),
            },
        );

        let mut policies = HashSet::new();
        policies.insert((
            "FingerprintReleasePolicy".to_string(),
            "KeyMaterial".to_string(),
            "Public".to_string(),
            "Releaser".to_string(),
        ));

        let principals = sample
            .principals
            .iter()
            .map(|principal| {
                (
                    principal.name.clone(),
                    (principal.theory.clone(), principal.element.clone()),
                )
            })
            .collect();

        Self {
            label_theories,
            authority_theories,
            capture_theories,
            region_theories,
            policies,
            principals,
        }
    }

    fn solve(
        &self,
        sample: &CleanNearEndSampleSpec,
        constraints: &[Constraint],
    ) -> ConstraintSolverResult {
        let mut solved = Vec::new();
        let mut failed = Vec::new();
        let mut residuals = Vec::new();

        for constraint in constraints {
            match self.solve_one(sample, constraint) {
                ConstraintVerdict::Valid => solved.push(constraint.display.clone()),
                ConstraintVerdict::Invalid => failed.push(constraint.display.clone()),
                ConstraintVerdict::Residual => residuals.push(ResidualObligation {
                    kind: "residual".to_string(),
                    subject_ref: format!("{}::{}", sample.id, constraint.source_ref.location),
                    reason: format!(
                        "constraint fell outside the first finite decidable fragment: {}",
                        constraint.display
                    ),
                }),
            }
        }

        let verdict = if !failed.is_empty() {
            ConstraintVerdict::Invalid
        } else if residuals.is_empty() {
            ConstraintVerdict::Valid
        } else {
            ConstraintVerdict::Residual
        };

        ConstraintSolverResult {
            verdict,
            solved,
            failed,
            residuals,
        }
    }

    fn solve_one(
        &self,
        sample: &CleanNearEndSampleSpec,
        constraint: &Constraint,
    ) -> ConstraintVerdict {
        match &constraint.evaluation {
            ConstraintEvaluation::LabelFlow {
                theory,
                lower,
                upper,
            } => self
                .label_theories
                .get(theory)
                .and_then(|theory| theory.preorder.leq(lower, upper))
                .map(|value| {
                    if value {
                        ConstraintVerdict::Valid
                    } else {
                        ConstraintVerdict::Invalid
                    }
                })
                .unwrap_or(ConstraintVerdict::Residual),
            ConstraintEvaluation::AuthorityGe {
                principal,
                theory,
                required,
            } => {
                let Some((principal_theory, actual)) = self.principals.get(principal) else {
                    return ConstraintVerdict::Residual;
                };
                if principal_theory != theory {
                    return ConstraintVerdict::Residual;
                }
                self.authority_theories
                    .get(theory)
                    .and_then(|theory| theory.leq(required, actual))
                    .map(|value| {
                        if value {
                            ConstraintVerdict::Valid
                        } else {
                            ConstraintVerdict::Invalid
                        }
                    })
                    .unwrap_or(ConstraintVerdict::Residual)
            }
            ConstraintEvaluation::CaptureSubset { theory, lhs, rhs } => self
                .capture_theories
                .get(theory)
                .and_then(|theory| theory.subset(lhs, rhs))
                .map(|value| {
                    if value {
                        ConstraintVerdict::Valid
                    } else {
                        ConstraintVerdict::Invalid
                    }
                })
                .unwrap_or(ConstraintVerdict::Residual),
            ConstraintEvaluation::RegionOutlives {
                theory,
                longer,
                shorter,
            } => self
                .region_theories
                .get(theory)
                .and_then(|theory| theory.preorder.leq(shorter, longer))
                .map(|value| {
                    if value {
                        ConstraintVerdict::Valid
                    } else {
                        ConstraintVerdict::Invalid
                    }
                })
                .unwrap_or(ConstraintVerdict::Residual),
            ConstraintEvaluation::CostLeq { lhs, rhs } => {
                if lhs.leq(rhs) {
                    ConstraintVerdict::Valid
                } else {
                    ConstraintVerdict::Invalid
                }
            }
            ConstraintEvaluation::EffectRowSubset { lhs, rhs } => {
                if lhs.is_subset(rhs) {
                    ConstraintVerdict::Valid
                } else {
                    ConstraintVerdict::Invalid
                }
            }
            ConstraintEvaluation::RequiresWitness {
                witness,
                producer_stage,
                consumer_stage,
            } => {
                let Some(order) = sample.order_spec.as_ref() else {
                    return ConstraintVerdict::Residual;
                };
                let Some(actual_producer) = order.produced_witnesses.get(witness) else {
                    return ConstraintVerdict::Invalid;
                };
                if actual_producer == producer_stage
                    && order.precedes(producer_stage, consumer_stage)
                {
                    ConstraintVerdict::Valid
                } else {
                    ConstraintVerdict::Invalid
                }
            }
            ConstraintEvaluation::RequiresPublication {
                publication_stage,
                consumer_stage,
            } => {
                let Some(order) = sample.order_spec.as_ref() else {
                    return ConstraintVerdict::Residual;
                };
                if order.precedes(publication_stage, consumer_stage) {
                    ConstraintVerdict::Valid
                } else {
                    ConstraintVerdict::Invalid
                }
            }
            ConstraintEvaluation::PolicyPermits {
                policy,
                principal,
                authority_theory,
                required_authority,
                from_label,
                to_label,
            } => {
                if !self.policies.contains(&(
                    policy.clone(),
                    from_label.clone(),
                    to_label.clone(),
                    required_authority.clone(),
                )) {
                    return ConstraintVerdict::Invalid;
                }
                let nested = Constraint {
                    kind: ConstraintKind::AuthorityGe,
                    display: format!(
                        "authority({principal}) >= {authority_theory}.{required_authority}"
                    ),
                    source_ref: constraint.source_ref.clone(),
                    evaluation: ConstraintEvaluation::AuthorityGe {
                        principal: principal.clone(),
                        theory: authority_theory.clone(),
                        required: required_authority.clone(),
                    },
                };
                self.solve_one(sample, &nested)
            }
        }
    }
}

#[derive(Debug, Clone)]
struct PrincipalBinding {
    name: String,
    theory: String,
    element: String,
}

#[derive(Debug, Clone)]
struct RelationSpec {
    kind: String,
    from: String,
    to: String,
}

#[derive(Debug, Clone)]
struct OrderSpec {
    stages: Vec<String>,
    produced_witnesses: HashMap<String, String>,
    derived_relations: Vec<RelationSpec>,
}

impl OrderSpec {
    fn precedes(&self, lhs: &str, rhs: &str) -> bool {
        let Some(lhs_idx) = self.stages.iter().position(|stage| stage == lhs) else {
            return false;
        };
        let Some(rhs_idx) = self.stages.iter().position(|stage| stage == rhs) else {
            return false;
        };
        lhs_idx < rhs_idx
    }
}

#[derive(Debug, Clone)]
struct ModalSpec {
    mode_constraints: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum ModelKind {
    PetersonSc,
    PetersonRelaxed,
    BrokenMutex,
}

#[derive(Debug, Clone)]
struct ProofObligationSpec {
    obligation: String,
}

#[derive(Debug, Clone)]
struct CleanNearEndSampleSpec {
    id: String,
    family: CleanSampleFamily,
    source_relpath: String,
    summary: String,
    built_in_terms: Vec<String>,
    user_defined_terms: Vec<String>,
    principals: Vec<PrincipalBinding>,
    constraints: Vec<Constraint>,
    reason_family: Option<String>,
    entered_evaluation_on_success: bool,
    terminal_outcome: Option<String>,
    order_spec: Option<OrderSpec>,
    modal_spec: Option<ModalSpec>,
    model_kind: Option<ModelKind>,
    current_owner: Option<String>,
    visible_history: Vec<String>,
    witness_core_fields: Vec<String>,
    proof_obligations: Vec<ProofObligationSpec>,
    property: Option<String>,
    checked_under: Option<String>,
    explanation: Option<String>,
    required_source_tokens: Vec<String>,
    layer_signatures: Vec<LayerSignature>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CleanNearEndSampleSummary {
    pub sample_id: String,
    pub family: CleanSampleFamily,
    pub source_path: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TermSignature {
    pub kind: String,
    pub name: String,
    pub evidence_role: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LayerSignature {
    pub name: String,
    pub requires: Vec<String>,
    pub provides: Vec<String>,
    pub transforms: Vec<String>,
    pub checks: Vec<String>,
    pub emits: Vec<String>,
    pub laws: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PrincipalClaim {
    pub principal: String,
    pub participant_place: String,
    pub claimed_authority: String,
    pub claimed_capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AuthEvidence {
    pub kind: String,
    pub subject: String,
    pub issuer: String,
    pub bindings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct MessageEnvelope {
    pub envelope_id: String,
    pub from_place: String,
    pub to_place: String,
    pub transport: String,
    pub payload_kind: String,
    pub payload_ref: String,
    pub principal_claim: PrincipalClaim,
    pub auth_evidence: Option<AuthEvidence>,
    pub membership_epoch: u64,
    pub member_incarnation: u64,
    pub capability_requirements: Vec<String>,
    pub authorization_checks: Vec<String>,
    pub witness_refs: Vec<String>,
    pub dispatch_outcome: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VisualizationView {
    pub view_name: String,
    pub layer_signature_refs: Vec<String>,
    pub message_envelope_refs: Vec<String>,
    pub focus_subjects: Vec<String>,
    pub redaction_rules: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TelemetryRow {
    pub row_name: String,
    pub channel: String,
    pub layer_signature_refs: Vec<String>,
    pub message_envelope_refs: Vec<String>,
    pub measurement: String,
    pub value: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CleanNearEndSampleReport {
    pub sample: String,
    pub family: CleanSampleFamily,
    pub source_path: String,
    pub static_verdict: Option<String>,
    pub entered_evaluation: bool,
    pub terminal_outcome: Option<String>,
    pub reason_family: Option<String>,
    pub constraints_solved: Vec<String>,
    pub constraints_failed: Vec<String>,
    pub residual_obligations: Vec<ResidualObligation>,
    pub relations: Vec<[String; 3]>,
    pub mode_constraints: Vec<String>,
    pub model_check_result: Option<String>,
    pub property: Option<String>,
    pub checked_under: Option<String>,
    pub counterexample_shape: Vec<String>,
    pub explanation: Option<String>,
    pub built_in_terms: Vec<String>,
    pub user_defined_terms: Vec<String>,
    pub theorem_obligations: Vec<String>,
    pub witness_core_fields: Vec<String>,
    pub current_owner: Option<String>,
    pub visible_history: Vec<String>,
    pub term_signatures: Vec<TermSignature>,
    pub layer_signatures: Vec<LayerSignature>,
    pub message_envelopes: Vec<MessageEnvelope>,
    pub visualization_views: Vec<VisualizationView>,
    pub telemetry_rows: Vec<TelemetryRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CleanNearEndMatrix {
    pub total_samples: usize,
    pub families: BTreeMap<String, usize>,
    pub valid_samples: Vec<String>,
    pub malformed_samples: Vec<String>,
    pub model_check_pass: Vec<String>,
    pub model_check_counterexample: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CleanNearEndCloseout {
    pub active_sample_root: String,
    pub archive_sample_root: String,
    pub built_in_vocabulary: Vec<String>,
    pub user_defined_vocabulary: Vec<String>,
    pub families: BTreeMap<String, Vec<String>>,
    pub proof_samples: Vec<String>,
    pub lean_roots: Vec<String>,
    pub signature_lanes: Vec<String>,
    pub signature_scope: String,
    pub signature_kinds: Vec<String>,
    pub signature_evidence_roles: Vec<String>,
    pub reserved_signature_kinds: Vec<String>,
    pub message_envelope_lanes: Vec<String>,
    pub auth_evidence_kinds: Vec<String>,
    pub reserved_auth_evidence_kinds: Vec<String>,
    pub transport_seams: Vec<String>,
    pub reserved_transport_seams: Vec<String>,
    pub visualization_views: Vec<VisualizationView>,
    pub visualization_view_lanes: Vec<String>,
    pub reserved_visualization_view_names: Vec<String>,
    pub telemetry_rows: Vec<TelemetryRow>,
    pub telemetry_row_lanes: Vec<String>,
    pub telemetry_channels: Vec<String>,
    pub reserved_telemetry_channels: Vec<String>,
    pub layer_signatures: Vec<LayerSignature>,
    pub layer_signature_lanes: Vec<String>,
    pub reserved_layer_signature_names: Vec<String>,
}

fn extract_declared_name(line: &str, prefix: &str) -> Option<String> {
    let stripped = line.trim_start();
    if !stripped.starts_with(prefix) {
        return None;
    }
    let remainder = stripped.strip_prefix(prefix)?.trim();
    let mut token = remainder.split_whitespace().next()?.trim_end_matches('{');
    if let Some((head, _)) = token.split_once('(') {
        token = head;
    }
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

fn extract_after_marker(line: &str, marker: &str) -> Option<String> {
    let (_, remainder) = line.split_once(marker)?;
    let token = remainder
        .trim()
        .split_whitespace()
        .next()?
        .trim_end_matches(',')
        .trim_end_matches('{');
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

fn push_term_signature(
    signatures: &mut Vec<TermSignature>,
    seen: &mut BTreeSet<(String, String, String)>,
    kind: &str,
    name: impl Into<String>,
    evidence_role: &str,
) {
    let name = name.into();
    if name.is_empty() {
        return;
    }
    let key = (
        kind.to_string(),
        name.clone(),
        evidence_role.to_string(),
    );
    if seen.insert(key.clone()) {
        signatures.push(TermSignature {
            kind: key.0,
            name: key.1,
            evidence_role: key.2,
        });
    }
}

fn layer_signature(
    name: &str,
    requires: &[&str],
    provides: &[&str],
    transforms: &[&str],
    checks: &[&str],
    emits: &[&str],
    laws: &[&str],
) -> LayerSignature {
    let collect = |values: &[&str]| values.iter().map(|value| (*value).to_string()).collect();
    LayerSignature {
        name: name.to_string(),
        requires: collect(requires),
        provides: collect(provides),
        transforms: collect(transforms),
        checks: collect(checks),
        emits: collect(emits),
        laws: collect(laws),
    }
}

fn principal_claim(
    principal: &str,
    claimed_authority: &str,
    claimed_capabilities: &[&str],
) -> PrincipalClaim {
    PrincipalClaim {
        principal: principal.to_string(),
        participant_place: format!("ParticipantPlace[{principal}]"),
        claimed_authority: claimed_authority.to_string(),
        claimed_capabilities: claimed_capabilities
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    }
}

fn message_envelope(
    envelope_id: &str,
    from_place: &str,
    to_place: &str,
    transport: &str,
    payload_kind: &str,
    payload_ref: &str,
    principal_claim: PrincipalClaim,
    membership_epoch: u64,
    member_incarnation: u64,
    capability_requirements: &[&str],
    authorization_checks: &[&str],
    witness_refs: &[&str],
    dispatch_outcome: &str,
    notes: &[&str],
) -> MessageEnvelope {
    let collect = |values: &[&str]| values.iter().map(|value| (*value).to_string()).collect();
    MessageEnvelope {
        envelope_id: envelope_id.to_string(),
        from_place: from_place.to_string(),
        to_place: to_place.to_string(),
        transport: transport.to_string(),
        payload_kind: payload_kind.to_string(),
        payload_ref: payload_ref.to_string(),
        principal_claim,
        auth_evidence: None,
        membership_epoch,
        member_incarnation,
        capability_requirements: collect(capability_requirements),
        authorization_checks: collect(authorization_checks),
        witness_refs: collect(witness_refs),
        dispatch_outcome: dispatch_outcome.to_string(),
        notes: collect(notes),
    }
}

fn visualization_view(
    view_name: &str,
    layer_signature_refs: &[String],
    message_envelope_refs: &[String],
    focus_subjects: &[&str],
    redaction_rules: &[&str],
    notes: &[&str],
) -> VisualizationView {
    let collect = |values: &[&str]| values.iter().map(|value| (*value).to_string()).collect();
    VisualizationView {
        view_name: view_name.to_string(),
        layer_signature_refs: layer_signature_refs.to_vec(),
        message_envelope_refs: message_envelope_refs.to_vec(),
        focus_subjects: collect(focus_subjects),
        redaction_rules: collect(redaction_rules),
        notes: collect(notes),
    }
}

fn telemetry_row(
    row_name: &str,
    channel: &str,
    layer_signature_refs: &[String],
    message_envelope_refs: &[String],
    measurement: &str,
    value: &str,
    notes: &[&str],
) -> TelemetryRow {
    let collect = |values: &[&str]| values.iter().map(|value| (*value).to_string()).collect();
    TelemetryRow {
        row_name: row_name.to_string(),
        channel: channel.to_string(),
        layer_signature_refs: layer_signature_refs.to_vec(),
        message_envelope_refs: message_envelope_refs.to_vec(),
        measurement: measurement.to_string(),
        value: value.to_string(),
        notes: collect(notes),
    }
}

fn layer_signature_lanes() -> Vec<String> {
    [
        "requires",
        "provides",
        "transforms",
        "checks",
        "emits",
        "laws",
    ]
    .into_iter()
    .map(|lane| lane.to_string())
    .collect()
}

fn visualization_view_lanes() -> Vec<String> {
    [
        "view_name",
        "layer_signature_refs",
        "message_envelope_refs",
        "focus_subjects",
        "redaction_rules",
        "notes",
    ]
    .into_iter()
    .map(|lane| lane.to_string())
    .collect()
}

fn telemetry_row_lanes() -> Vec<String> {
    [
        "row_name",
        "channel",
        "layer_signature_refs",
        "message_envelope_refs",
        "measurement",
        "value",
        "notes",
    ]
    .into_iter()
    .map(|lane| lane.to_string())
    .collect()
}

fn message_envelope_lanes() -> Vec<String> {
    [
        "envelope_id",
        "from_place",
        "to_place",
        "transport",
        "payload_kind",
        "payload_ref",
        "principal_claim",
        "auth_evidence",
        "membership_epoch",
        "member_incarnation",
        "capability_requirements",
        "authorization_checks",
        "witness_refs",
        "dispatch_outcome",
        "notes",
    ]
    .into_iter()
    .map(|lane| lane.to_string())
    .collect()
}

fn signature_lanes() -> Vec<String> {
    ["kind", "name", "evidence_role"]
        .into_iter()
        .map(|lane| lane.to_string())
        .collect()
}

fn reserved_layer_signature_names() -> Vec<String> {
    [
        "visualization_redacted_debug_view",
        "typed_telemetry_emitter",
    ]
    .into_iter()
    .map(|name| name.to_string())
    .collect()
}

fn reserved_visualization_view_names() -> Vec<String> {
    ["label_authority_redaction_grid"]
        .into_iter()
        .map(|name| name.to_string())
        .collect()
}

fn reserved_telemetry_channels() -> Vec<String> {
    ["typed_effect_adapter", "attach_point_activation"]
        .into_iter()
        .map(|name| name.to_string())
        .collect()
}

fn term_signatures_for_spec(spec: &CleanNearEndSampleSpec, source: &str) -> Vec<TermSignature> {
    let mut signatures = Vec::new();
    let mut seen = BTreeSet::new();

    for line in source.lines() {
        if let Some(name) = extract_declared_name(line, "transition ") {
            push_term_signature(
                &mut signatures,
                &mut seen,
                "transition",
                name,
                "source_decl",
            );
        }
        if let Some(name) = extract_declared_name(line, "effect ") {
            push_term_signature(&mut signatures, &mut seen, "effect", name, "source_decl");
        }
        if let Some(name) = extract_after_marker(line, "produces witness ") {
            push_term_signature(&mut signatures, &mut seen, "witness", name, "source_decl");
        }
    }

    if let Some(order_spec) = &spec.order_spec {
        for witness in order_spec.produced_witnesses.keys() {
            push_term_signature(
                &mut signatures,
                &mut seen,
                "witness",
                witness.clone(),
                "order_produced_witness",
            );
        }
        for relation in &order_spec.derived_relations {
            push_term_signature(
                &mut signatures,
                &mut seen,
                "relation",
                relation.kind.clone(),
                "derived_relation",
            );
        }
    }

    for constraint in &spec.constraints {
        if let ConstraintEvaluation::EffectRowSubset { lhs, rhs } = &constraint.evaluation {
            for effect_name in lhs.iter().chain(rhs.iter()) {
                push_term_signature(
                    &mut signatures,
                    &mut seen,
                    "effect",
                    effect_name.clone(),
                    "effect_row_constraint",
                );
            }
        }
    }

    if let Some(property) = &spec.property {
        push_term_signature(
            &mut signatures,
            &mut seen,
            "property",
            property.clone(),
            "sample_property",
        );
    }

    signatures
}

fn closeout_signature_kinds() -> Result<Vec<String>, CleanNearEndError> {
    let root = clean_near_end_samples_root();
    let mut kinds = BTreeSet::new();
    for spec in clean_near_end_sample_specs() {
        let source_path = root.join(&spec.source_relpath);
        let source = fs::read_to_string(&source_path).map_err(|error| {
            CleanNearEndError::new(format!(
                "failed to read clean near-end sample for closeout {}: {error}",
                source_path.display()
            ))
        })?;
        for signature in term_signatures_for_spec(&spec, &source) {
            kinds.insert(signature.kind);
        }
    }
    Ok(kinds.into_iter().collect())
}

fn closeout_signature_evidence_roles() -> Result<Vec<String>, CleanNearEndError> {
    let root = clean_near_end_samples_root();
    let mut roles = BTreeSet::new();
    for spec in clean_near_end_sample_specs() {
        let source_path = root.join(&spec.source_relpath);
        let source = fs::read_to_string(&source_path).map_err(|error| {
            CleanNearEndError::new(format!(
                "failed to read clean near-end sample for closeout {}: {error}",
                source_path.display()
            ))
        })?;
        for signature in term_signatures_for_spec(&spec, &source) {
            roles.insert(signature.evidence_role);
        }
    }
    Ok(roles.into_iter().collect())
}

fn closeout_layer_signatures() -> Vec<LayerSignature> {
    let mut signatures = BTreeMap::new();
    for spec in clean_near_end_sample_specs() {
        for layer in spec.layer_signatures {
            signatures.entry(layer.name.clone()).or_insert(layer);
        }
    }
    signatures.into_values().collect()
}

fn closeout_visualization_views() -> Vec<VisualizationView> {
    let mut views = BTreeMap::new();
    for spec in clean_near_end_sample_specs() {
        let message_envelopes = message_envelopes_for_spec(&spec);
        for view in visualization_views_for_spec(&spec, &spec.layer_signatures, &message_envelopes)
        {
            views.entry(view.view_name.clone()).or_insert(view);
        }
    }
    views.into_values().collect()
}

fn closeout_telemetry_rows() -> Vec<TelemetryRow> {
    let mut rows = BTreeMap::new();
    for spec in clean_near_end_sample_specs() {
        let message_envelopes = message_envelopes_for_spec(&spec);
        for row in telemetry_rows_for_spec(&spec, &spec.layer_signatures, &message_envelopes) {
            rows.entry(row.row_name.clone()).or_insert(row);
        }
    }
    rows.into_values().collect()
}

fn closeout_telemetry_channels(rows: &[TelemetryRow]) -> Vec<String> {
    rows.iter()
        .map(|row| row.channel.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn message_envelopes_for_spec(spec: &CleanNearEndSampleSpec) -> Vec<MessageEnvelope> {
    match spec.id.as_str() {
        "05_delegated_rng_service" => vec![
            message_envelope(
                "provider_request#1",
                "ParticipantPlace[Alice]",
                "ProviderPlace[AuthorityRng]",
                "provider_boundary",
                "effect_request",
                "delegated_rng_roll",
                principal_claim(
                    "Alice",
                    "FingerprintAuthority.Releaser",
                    &["RequestDelegatedRngRoll", "PublishProviderReceipt"],
                ),
                0,
                0,
                &["RequestDelegatedRngRoll", "PublishProviderReceipt"],
                &[
                    "effect row { rng, witness } <= { rng, witness, publish }",
                    "requires witness(provider_receipt)",
                ],
                &["provider_receipt"],
                "accepted",
                &[
                    "auth none baseline on repo-local provider boundary",
                    "transport and witness remain separate lanes",
                ],
            ),
            message_envelope(
                "provider_receipt#1",
                "ProviderPlace[AuthorityRng]",
                "ParticipantPlace[Alice]",
                "provider_boundary",
                "witness_receipt",
                "provider_receipt",
                principal_claim(
                    "Alice",
                    "FingerprintAuthority.Releaser",
                    &["ObserveProviderReceipt"],
                ),
                0,
                0,
                &["ObserveProviderReceipt"],
                &["receipt stays distinct from later room mutation authority"],
                &["provider_receipt"],
                "accepted",
                &["provider output is not itself room-state mutation"],
            ),
        ],
        "06_auditable_authority_witness" => vec![message_envelope(
            "audit_trace_request#1",
            "ParticipantPlace[Alice]",
            "AuditPlace[AuthorityTrace]",
            "audit_trace_boundary",
            "witness_trace",
            "audit(draw_pub)",
            principal_claim(
                "Alice",
                "FingerprintAuthority.Admin",
                &["AuditAuthorityWitness", "ObserveAuthorityTrace"],
            ),
            0,
            0,
            &["AuditAuthorityWitness", "ObserveAuthorityTrace"],
            &["requires witness(draw_pub)", "no_hidden_authority"],
            &["draw_pub"],
            "accepted",
            &[
                "authority witness is evidence, not authentication",
                "visualization stays downstream of witness production",
            ],
        )],
        _ => Vec::new(),
    }
}

fn visualization_views_for_spec(
    spec: &CleanNearEndSampleSpec,
    layer_signatures: &[LayerSignature],
    message_envelopes: &[MessageEnvelope],
) -> Vec<VisualizationView> {
    let layer_signature_refs = layer_signatures
        .iter()
        .map(|layer| layer.name.clone())
        .collect::<Vec<_>>();
    let message_envelope_refs = message_envelopes
        .iter()
        .map(|envelope| envelope.envelope_id.clone())
        .collect::<Vec<_>>();

    match spec.id.as_str() {
        "05_delegated_rng_service" => vec![
            visualization_view(
                "provider_boundary_redacted_flow",
                &layer_signature_refs,
                &message_envelope_refs,
                &["effect:delegated_rng_roll", "witness:provider_receipt"],
                &[
                    "auth_evidence:none_baseline",
                    "witness_payload:named_reference_only",
                ],
                &[
                    "report-local view over current layer/message inventory",
                    "transport and witness remain separate lanes",
                ],
            ),
            visualization_view(
                "cross_place_projection",
                &layer_signature_refs,
                &message_envelope_refs,
                &[
                    "projection:provider_boundary_draw_route",
                    "authority_place:ParticipantPlace[Alice]",
                    "place:ParticipantPlace[Alice]",
                    "place:ProviderPlace[AuthorityRng]",
                    "witness:provider_receipt",
                ],
                &["projection_summary_only", "auth_evidence:none_baseline"],
                &[
                    "report-local projection preview over current layer/message inventory",
                    "authority placement stays distinct from provider placement",
                    "not a final emitted place-specific program",
                ],
            ),
        ],
        "06_auditable_authority_witness" => vec![visualization_view(
            "authority_trace_redacted_view",
            &layer_signature_refs,
            &message_envelope_refs,
            &["witness:draw_pub", "view:audit_trace"],
            &[
                "auth_evidence:none_baseline",
                "label_guard:visualization_respects_labels",
            ],
            &[
                "authority witness is evidence, not authentication",
                "visualization stays downstream of witness production",
            ],
        )],
        _ => Vec::new(),
    }
}

fn telemetry_rows_for_spec(
    spec: &CleanNearEndSampleSpec,
    layer_signatures: &[LayerSignature],
    _message_envelopes: &[MessageEnvelope],
) -> Vec<TelemetryRow> {
    let layer_signature_refs = layer_signatures
        .iter()
        .map(|layer| layer.name.clone())
        .collect::<Vec<_>>();

    match spec.id.as_str() {
        "05_delegated_rng_service" => vec![telemetry_row(
            "provider_boundary_dispatch",
            "provider_boundary",
            &layer_signature_refs,
            &["provider_request#1".to_string()],
            "dispatch_outcome",
            "accepted",
            &["typed effect request observed without freezing adapter contract"],
        )],
        "06_auditable_authority_witness" => vec![telemetry_row(
            "audit_trace_dispatch",
            "audit_trace_boundary",
            &layer_signature_refs,
            &["audit_trace_request#1".to_string()],
            "dispatch_outcome",
            "accepted",
            &[
                "visualization stays downstream of witness production",
                "report-local audit flow telemetry only",
            ],
        )],
        _ => Vec::new(),
    }
}

pub fn clean_near_end_samples_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/clean-near-end")
}

pub fn clean_near_end_archive_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../samples/old/2026-04-22-pre-clean-near-end")
}

pub fn built_in_vocabulary() -> Vec<String> {
    [
        "module",
        "index",
        "policy",
        "principal",
        "resource",
        "effect",
        "place",
        "option",
        "chain",
        "fallback",
        "lineage",
        "perform",
        "via",
        "require",
        "ensure",
        "atomic_cut",
        "transition",
        "stage",
        "publish",
        "observe",
        "handoff",
        "witness",
        "model",
        "property",
    ]
    .into_iter()
    .map(|value| value.to_string())
    .collect()
}

pub fn user_defined_vocabulary() -> Vec<String> {
    [
        "SecurityLabel",
        "FingerprintAuthority",
        "CaptureScope",
        "Region",
        "CostBudget",
        "FingerprintReleasePolicy",
        "Public",
        "UserSecret",
        "KeyMaterial",
        "Observer",
        "Holder",
        "Releaser",
        "Admin",
        "RoomHistory",
        "EphemeralToken",
        "SecretKeyStore",
        "Step",
        "Turn",
        "Session",
        "Alice",
        "Bob",
        "Carol",
        "draw_pub",
        "dice_owner",
    ]
    .into_iter()
    .map(|value| value.to_string())
    .collect()
}

pub fn shared_index_theories() -> Vec<IndexTheoryDecl> {
    vec![
        IndexTheoryDecl {
            name: "SecurityLabel".to_string(),
            kind: "finite_lattice".to_string(),
            elements: ["Public", "UserSecret", "KeyMaterial"]
                .into_iter()
                .map(|name| IndexElementDecl {
                    name: name.to_string(),
                })
                .collect(),
            orders: vec![
                IndexOrderDecl {
                    lower: "Public".to_string(),
                    upper: "UserSecret".to_string(),
                },
                IndexOrderDecl {
                    lower: "UserSecret".to_string(),
                    upper: "KeyMaterial".to_string(),
                },
            ],
            laws: vec![IndexLawDecl {
                name: "finite_lattice".to_string(),
            }],
            counters: Vec::new(),
        },
        IndexTheoryDecl {
            name: "FingerprintAuthority".to_string(),
            kind: "finite_preorder".to_string(),
            elements: ["Observer", "Holder", "Releaser", "Admin"]
                .into_iter()
                .map(|name| IndexElementDecl {
                    name: name.to_string(),
                })
                .collect(),
            orders: vec![
                IndexOrderDecl {
                    lower: "Observer".to_string(),
                    upper: "Holder".to_string(),
                },
                IndexOrderDecl {
                    lower: "Holder".to_string(),
                    upper: "Releaser".to_string(),
                },
                IndexOrderDecl {
                    lower: "Releaser".to_string(),
                    upper: "Admin".to_string(),
                },
            ],
            laws: vec![IndexLawDecl {
                name: "finite_preorder".to_string(),
            }],
            counters: Vec::new(),
        },
        IndexTheoryDecl {
            name: "CaptureScope".to_string(),
            kind: "finite_powerset".to_string(),
            elements: ["RoomHistory", "EphemeralToken", "SecretKeyStore"]
                .into_iter()
                .map(|name| IndexElementDecl {
                    name: name.to_string(),
                })
                .collect(),
            orders: Vec::new(),
            laws: vec![IndexLawDecl {
                name: "finite_powerset".to_string(),
            }],
            counters: Vec::new(),
        },
        IndexTheoryDecl {
            name: "Region".to_string(),
            kind: "finite_preorder".to_string(),
            elements: ["Step", "Turn", "Session"]
                .into_iter()
                .map(|name| IndexElementDecl {
                    name: name.to_string(),
                })
                .collect(),
            orders: vec![
                IndexOrderDecl {
                    lower: "Step".to_string(),
                    upper: "Turn".to_string(),
                },
                IndexOrderDecl {
                    lower: "Turn".to_string(),
                    upper: "Session".to_string(),
                },
            ],
            laws: vec![IndexLawDecl {
                name: "finite_preorder".to_string(),
            }],
            counters: Vec::new(),
        },
        IndexTheoryDecl {
            name: "CostBudget".to_string(),
            kind: "pointwise_natural_bound".to_string(),
            elements: Vec::new(),
            orders: Vec::new(),
            laws: vec![IndexLawDecl {
                name: "pointwise_natural_bound".to_string(),
            }],
            counters: vec![
                "cpu_steps".to_string(),
                "remote_calls".to_string(),
                "writes".to_string(),
            ],
        },
    ]
}

fn proof_obligations(values: &[&str]) -> Vec<ProofObligationSpec> {
    values
        .iter()
        .map(|value| ProofObligationSpec {
            obligation: (*value).to_string(),
        })
        .collect()
}

fn source_ref(sample: &str, location: &str) -> ConstraintSourceRef {
    ConstraintSourceRef {
        sample: sample.to_string(),
        location: location.to_string(),
    }
}

fn typing_sample(
    id: &str,
    source_relpath: &str,
    summary: &str,
    user_defined_terms: &[&str],
    principals: Vec<PrincipalBinding>,
    constraints: Vec<Constraint>,
    reason_family: Option<&str>,
    terminal_outcome: Option<&str>,
    obligations: &[&str],
    required_source_tokens: &[&str],
) -> CleanNearEndSampleSpec {
    CleanNearEndSampleSpec {
        id: id.to_string(),
        family: CleanSampleFamily::Typing,
        source_relpath: source_relpath.to_string(),
        summary: summary.to_string(),
        built_in_terms: built_in_vocabulary(),
        user_defined_terms: user_defined_terms
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        principals,
        constraints,
        reason_family: reason_family.map(str::to_string),
        entered_evaluation_on_success: terminal_outcome.is_some(),
        terminal_outcome: terminal_outcome.map(str::to_string),
        order_spec: None,
        modal_spec: None,
        model_kind: None,
        current_owner: None,
        visible_history: Vec::new(),
        witness_core_fields: Vec::new(),
        proof_obligations: proof_obligations(obligations),
        property: None,
        checked_under: None,
        explanation: None,
        required_source_tokens: required_source_tokens
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        layer_signatures: Vec::new(),
    }
}

fn clean_near_end_sample_specs() -> Vec<CleanNearEndSampleSpec> {
    let alice_admin = PrincipalBinding {
        name: "Alice".to_string(),
        theory: "FingerprintAuthority".to_string(),
        element: "Admin".to_string(),
    };
    let alice_releaser = PrincipalBinding {
        name: "Alice".to_string(),
        theory: "FingerprintAuthority".to_string(),
        element: "Releaser".to_string(),
    };
    let bob_holder = PrincipalBinding {
        name: "Bob".to_string(),
        theory: "FingerprintAuthority".to_string(),
        element: "Holder".to_string(),
    };
    let bob_observer = PrincipalBinding {
        name: "Bob".to_string(),
        theory: "FingerprintAuthority".to_string(),
        element: "Observer".to_string(),
    };

    let mut specs = vec![
        typing_sample(
            "01_authorized_declassification",
            "typing/01_authorized_declassification.mir",
            "Authorized declassification through a user-defined label lattice and authority preorder.",
            &[
                "SecurityLabel",
                "FingerprintAuthority",
                "FingerprintReleasePolicy",
                "Alice",
            ],
            vec![alice_releaser.clone()],
            vec![
                Constraint {
                    kind: ConstraintKind::AuthorityGe,
                    display: "authority(Alice) >= FingerprintAuthority.Releaser".to_string(),
                    source_ref: source_ref("01_authorized_declassification", "declassify"),
                    evaluation: ConstraintEvaluation::AuthorityGe {
                        principal: "Alice".to_string(),
                        theory: "FingerprintAuthority".to_string(),
                        required: "Releaser".to_string(),
                    },
                },
                Constraint {
                    kind: ConstraintKind::PolicyPermits,
                    display: "declassify SecurityLabel.KeyMaterial -> SecurityLabel.Public permitted by FingerprintReleasePolicy".to_string(),
                    source_ref: source_ref("01_authorized_declassification", "policy"),
                    evaluation: ConstraintEvaluation::PolicyPermits {
                        policy: "FingerprintReleasePolicy".to_string(),
                        principal: "Alice".to_string(),
                        authority_theory: "FingerprintAuthority".to_string(),
                        required_authority: "Releaser".to_string(),
                        from_label: "KeyMaterial".to_string(),
                        to_label: "Public".to_string(),
                    },
                },
                Constraint {
                    kind: ConstraintKind::LabelFlow,
                    display: "label(fp_public) = SecurityLabel.Public".to_string(),
                    source_ref: source_ref("01_authorized_declassification", "publish"),
                    evaluation: ConstraintEvaluation::LabelFlow {
                        theory: "SecurityLabel".to_string(),
                        lower: "Public".to_string(),
                        upper: "Public".to_string(),
                    },
                },
            ],
            None,
            Some("success"),
            &["authorized_release_preserves_public_label"],
            &["FingerprintReleasePolicy", "Releaser", "SecurityLabel.Public"],
        ),
        typing_sample(
            "02_unauthorized_declassification_rejected",
            "typing/02_unauthorized_declassification_rejected.mir",
            "Unauthorized declassification rejected by the finite authority preorder.",
            &[
                "SecurityLabel",
                "FingerprintAuthority",
                "FingerprintReleasePolicy",
                "Bob",
            ],
            vec![bob_observer.clone()],
            vec![Constraint {
                kind: ConstraintKind::AuthorityGe,
                display: "FingerprintAuthority.Observer >= FingerprintAuthority.Releaser".to_string(),
                source_ref: source_ref(
                    "02_unauthorized_declassification_rejected",
                    "declassify",
                ),
                evaluation: ConstraintEvaluation::AuthorityGe {
                    principal: "Bob".to_string(),
                    theory: "FingerprintAuthority".to_string(),
                    required: "Releaser".to_string(),
                },
            }],
            Some("authority_preorder_constraint_failed"),
            None,
            &["unauthorized_release_is_impossible"],
            &["Observer", "Releaser"],
        ),
        typing_sample(
            "03_label_flow_rejected",
            "typing/03_label_flow_rejected.mir",
            "Direct high-to-low label flow rejected without declassification.",
            &["SecurityLabel", "KeyMaterial", "Public"],
            vec![alice_releaser.clone()],
            vec![Constraint {
                kind: ConstraintKind::LabelFlow,
                display: "SecurityLabel.KeyMaterial <= SecurityLabel.Public".to_string(),
                source_ref: source_ref("03_label_flow_rejected", "publish"),
                evaluation: ConstraintEvaluation::LabelFlow {
                    theory: "SecurityLabel".to_string(),
                    lower: "KeyMaterial".to_string(),
                    upper: "Public".to_string(),
                },
            }],
            Some("label_flow_constraint_failed"),
            None,
            &["no_high_to_low_without_policy"],
            &["KeyMaterial", "Public"],
        ),
        typing_sample(
            "04_capture_escape_rejected",
            "typing/04_capture_escape_rejected.mir",
            "Ephemeral capture cannot escape into a longer-lived public closure.",
            &["CaptureScope", "EphemeralToken", "RoomHistory", "Region"],
            Vec::new(),
            vec![
                Constraint {
                    kind: ConstraintKind::CaptureSubset,
                    display: "{EphemeralToken} <= {RoomHistory}".to_string(),
                    source_ref: source_ref("04_capture_escape_rejected", "capture"),
                    evaluation: ConstraintEvaluation::CaptureSubset {
                        theory: "CaptureScope".to_string(),
                        lhs: CaptureSet::from_slice(&["EphemeralToken"]),
                        rhs: CaptureSet::from_slice(&["RoomHistory"]),
                    },
                },
                Constraint {
                    kind: ConstraintKind::RegionOutlives,
                    display: "Region.Session <= Region.Step".to_string(),
                    source_ref: source_ref("04_capture_escape_rejected", "lifetime"),
                    evaluation: ConstraintEvaluation::RegionOutlives {
                        theory: "Region".to_string(),
                        longer: "Step".to_string(),
                        shorter: "Session".to_string(),
                    },
                },
            ],
            Some("capture_escape"),
            None,
            &["ephemeral_capture_cannot_outlive_step"],
            &["EphemeralToken", "RoomHistory", "lifetime Step"],
        ),
        typing_sample(
            "05_cost_bound_rejected",
            "typing/05_cost_bound_rejected.mir",
            "Simple remote-call budget exceeded and rejected statically.",
            &["CostBudget", "remote_calls", "fetch_remote_bonus"],
            Vec::new(),
            vec![Constraint {
                kind: ConstraintKind::CostLeq,
                display: "remote_calls 1 <= 0".to_string(),
                source_ref: source_ref("05_cost_bound_rejected", "cost"),
                evaluation: ConstraintEvaluation::CostLeq {
                    lhs: SimpleCostBound::from_pairs(&[
                        ("cpu_steps", 20),
                        ("remote_calls", 1),
                        ("writes", 0),
                    ]),
                    rhs: SimpleCostBound::from_pairs(&[
                        ("cpu_steps", 100),
                        ("remote_calls", 0),
                        ("writes", 0),
                    ]),
                },
            }],
            Some("cost_bound_exceeded"),
            None,
            &["pointwise_remote_budget_never_increases"],
            &["remote_calls: 0", "fetch_remote_bonus"],
        ),
    ];

    let order_specs = vec![
        CleanNearEndSampleSpec {
            id: "01_authorized_roll_publish_handoff".to_string(),
            family: CleanSampleFamily::OrderHandoff,
            source_relpath: "order-handoff/01_authorized_roll_publish_handoff.mir".to_string(),
            summary: "Authorized roll, publish, and witness-backed handoff.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec!["dice_owner".to_string(), "draw_pub".to_string()],
            principals: vec![alice_admin.clone(), bob_holder.clone()],
            constraints: vec![
                Constraint {
                    kind: ConstraintKind::RequiresWitness,
                    display: "requires witness(draw_pub)".to_string(),
                    source_ref: source_ref(
                        "01_authorized_roll_publish_handoff",
                        "handoff",
                    ),
                    evaluation: ConstraintEvaluation::RequiresWitness {
                        witness: "draw_pub".to_string(),
                        producer_stage: "publish".to_string(),
                        consumer_stage: "handoff".to_string(),
                    },
                },
                Constraint {
                    kind: ConstraintKind::RequiresPublication,
                    display: "publish(draw) must precede handoff".to_string(),
                    source_ref: source_ref(
                        "01_authorized_roll_publish_handoff",
                        "handoff",
                    ),
                    evaluation: ConstraintEvaluation::RequiresPublication {
                        publication_stage: "publish".to_string(),
                        consumer_stage: "handoff".to_string(),
                    },
                },
            ],
            reason_family: None,
            entered_evaluation_on_success: true,
            terminal_outcome: Some("success".to_string()),
            order_spec: Some(OrderSpec {
                stages: vec!["roll".to_string(), "publish".to_string(), "handoff".to_string()],
                produced_witnesses: HashMap::from([("draw_pub".to_string(), "publish".to_string())]),
                derived_relations: vec![
                    RelationSpec {
                        kind: "program_order".to_string(),
                        from: "roll".to_string(),
                        to: "publish".to_string(),
                    },
                    RelationSpec {
                        kind: "publication_order".to_string(),
                        from: "roll".to_string(),
                        to: "publish".to_string(),
                    },
                    RelationSpec {
                        kind: "witness_order".to_string(),
                        from: "publish".to_string(),
                        to: "handoff".to_string(),
                    },
                    RelationSpec {
                        kind: "scoped_happens_before".to_string(),
                        from: "roll".to_string(),
                        to: "handoff".to_string(),
                    },
                ],
            }),
            modal_spec: None,
            model_kind: None,
            current_owner: Some("Bob".to_string()),
            visible_history: vec![
                "draw".to_string(),
                "publish(draw)".to_string(),
                "handoff(Alice -> Bob)".to_string(),
            ],
            witness_core_fields: vec!["draw_pub".to_string()],
            proof_obligations: proof_obligations(&["handoff_requires_publication_witness"]),
            property: None,
            checked_under: None,
            explanation: None,
            required_source_tokens: vec![
                "draw_pub".to_string(),
                "handoff dice_owner Alice -> Bob".to_string(),
            ],
            layer_signatures: Vec::new(),
        },
        CleanNearEndSampleSpec {
            id: "02_missing_witness_rejected".to_string(),
            family: CleanSampleFamily::OrderHandoff,
            source_relpath: "order-handoff/02_missing_witness_rejected.mir".to_string(),
            summary: "Handoff rejected when publication exists but no witness is carried into the handoff stage.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec!["dice_owner".to_string()],
            principals: vec![alice_admin.clone(), bob_holder.clone()],
            constraints: vec![Constraint {
                kind: ConstraintKind::RequiresWitness,
                display: "requires witness(draw_pub)".to_string(),
                source_ref: source_ref("02_missing_witness_rejected", "handoff"),
                evaluation: ConstraintEvaluation::RequiresWitness {
                    witness: "draw_pub".to_string(),
                    producer_stage: "publish".to_string(),
                    consumer_stage: "handoff".to_string(),
                },
            }],
            reason_family: Some("missing_handoff_witness".to_string()),
            entered_evaluation_on_success: false,
            terminal_outcome: None,
            order_spec: Some(OrderSpec {
                stages: vec!["roll".to_string(), "publish".to_string(), "handoff".to_string()],
                produced_witnesses: HashMap::new(),
                derived_relations: Vec::new(),
            }),
            modal_spec: None,
            model_kind: None,
            current_owner: None,
            visible_history: Vec::new(),
            witness_core_fields: Vec::new(),
            proof_obligations: proof_obligations(&["handoff_without_witness_is_blocked"]),
            property: None,
            checked_under: None,
            explanation: None,
            required_source_tokens: vec!["after publish(draw)".to_string()],
            layer_signatures: Vec::new(),
        },
        CleanNearEndSampleSpec {
            id: "03_handoff_before_publication_rejected".to_string(),
            family: CleanSampleFamily::OrderHandoff,
            source_relpath: "order-handoff/03_handoff_before_publication_rejected.mir".to_string(),
            summary: "Handoff rejected because it appears before the publication stage that should justify it.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec!["dice_owner".to_string(), "draw_pub".to_string()],
            principals: vec![alice_admin.clone(), bob_holder.clone()],
            constraints: vec![Constraint {
                kind: ConstraintKind::RequiresPublication,
                display: "publish(draw) must precede handoff that requires witness(draw_pub)".to_string(),
                source_ref: source_ref(
                    "03_handoff_before_publication_rejected",
                    "handoff",
                ),
                evaluation: ConstraintEvaluation::RequiresPublication {
                    publication_stage: "publish".to_string(),
                    consumer_stage: "handoff".to_string(),
                },
            }],
            reason_family: Some("handoff_before_publication".to_string()),
            entered_evaluation_on_success: false,
            terminal_outcome: None,
            order_spec: Some(OrderSpec {
                stages: vec!["roll".to_string(), "handoff".to_string(), "publish".to_string()],
                produced_witnesses: HashMap::from([("draw_pub".to_string(), "publish".to_string())]),
                derived_relations: Vec::new(),
            }),
            modal_spec: None,
            model_kind: None,
            current_owner: None,
            visible_history: Vec::new(),
            witness_core_fields: Vec::new(),
            proof_obligations: proof_obligations(&["publication_precedes_handoff"]),
            property: None,
            checked_under: None,
            explanation: None,
            required_source_tokens: vec!["requires witness(draw_pub)".to_string()],
            layer_signatures: Vec::new(),
        },
        CleanNearEndSampleSpec {
            id: "04_stage_block_authorized_handoff".to_string(),
            family: CleanSampleFamily::OrderHandoff,
            source_relpath: "order-handoff/04_stage_block_authorized_handoff.mir".to_string(),
            summary: "Stage-block variant of authorized handoff with an explicit local finalization point.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec!["draw_pub".to_string(), "stage_block".to_string()],
            principals: vec![alice_admin.clone(), bob_holder.clone()],
            constraints: vec![
                Constraint {
                    kind: ConstraintKind::RequiresWitness,
                    display: "requires witness(draw_pub)".to_string(),
                    source_ref: source_ref("04_stage_block_authorized_handoff", "handoff"),
                    evaluation: ConstraintEvaluation::RequiresWitness {
                        witness: "draw_pub".to_string(),
                        producer_stage: "publish".to_string(),
                        consumer_stage: "handoff".to_string(),
                    },
                },
                Constraint {
                    kind: ConstraintKind::RequiresPublication,
                    display: "publish(draw) must precede handoff".to_string(),
                    source_ref: source_ref("04_stage_block_authorized_handoff", "handoff"),
                    evaluation: ConstraintEvaluation::RequiresPublication {
                        publication_stage: "publish".to_string(),
                        consumer_stage: "handoff".to_string(),
                    },
                },
            ],
            reason_family: None,
            entered_evaluation_on_success: true,
            terminal_outcome: Some("success".to_string()),
            order_spec: Some(OrderSpec {
                stages: vec![
                    "roll".to_string(),
                    "publish".to_string(),
                    "stage_block".to_string(),
                    "handoff".to_string(),
                ],
                produced_witnesses: HashMap::from([("draw_pub".to_string(), "publish".to_string())]),
                derived_relations: vec![
                    RelationSpec {
                        kind: "program_order".to_string(),
                        from: "publish".to_string(),
                        to: "stage_block".to_string(),
                    },
                    RelationSpec {
                        kind: "finalization_order".to_string(),
                        from: "stage_block".to_string(),
                        to: "handoff".to_string(),
                    },
                    RelationSpec {
                        kind: "scoped_happens_before".to_string(),
                        from: "roll".to_string(),
                        to: "handoff".to_string(),
                    },
                ],
            }),
            modal_spec: None,
            model_kind: None,
            current_owner: Some("Bob".to_string()),
            visible_history: vec![
                "draw".to_string(),
                "publish(draw)".to_string(),
                "atomic_cut(stage_block)".to_string(),
                "handoff(Alice -> Bob)".to_string(),
            ],
            witness_core_fields: vec!["draw_pub".to_string()],
            proof_obligations: proof_obligations(&["atomic_cut_is_local_finalization_only"]),
            property: None,
            checked_under: None,
            explanation: None,
            required_source_tokens: vec!["stage block".to_string(), "draw_pub".to_string()],
            layer_signatures: Vec::new(),
        },
        CleanNearEndSampleSpec {
            id: "05_delegated_rng_service".to_string(),
            family: CleanSampleFamily::OrderHandoff,
            source_relpath: "order-handoff/05_delegated_rng_service.mir".to_string(),
            summary: "Delegated RNG remains a provider boundary and does not mutate room state directly.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec!["provider_receipt".to_string(), "DelegatedRngReceipt".to_string()],
            principals: vec![alice_admin.clone(), bob_holder.clone()],
            constraints: vec![
                Constraint {
                    kind: ConstraintKind::RequiresWitness,
                    display: "requires witness(provider_receipt)".to_string(),
                    source_ref: source_ref("05_delegated_rng_service", "handoff"),
                    evaluation: ConstraintEvaluation::RequiresWitness {
                        witness: "provider_receipt".to_string(),
                        producer_stage: "provider".to_string(),
                        consumer_stage: "handoff".to_string(),
                    },
                },
                Constraint {
                    kind: ConstraintKind::EffectRowSubset,
                    display: "effect row { rng, witness } <= { rng, witness, publish }".to_string(),
                    source_ref: source_ref("05_delegated_rng_service", "provider"),
                    evaluation: ConstraintEvaluation::EffectRowSubset {
                        lhs: ["rng", "witness"]
                            .into_iter()
                            .map(|value| value.to_string())
                            .collect(),
                        rhs: ["rng", "witness", "publish"]
                            .into_iter()
                            .map(|value| value.to_string())
                            .collect(),
                    },
                },
            ],
            reason_family: None,
            entered_evaluation_on_success: true,
            terminal_outcome: Some("success".to_string()),
            order_spec: Some(OrderSpec {
                stages: vec![
                    "provider".to_string(),
                    "publish".to_string(),
                    "handoff".to_string(),
                ],
                produced_witnesses: HashMap::from([(
                    "provider_receipt".to_string(),
                    "provider".to_string(),
                )]),
                derived_relations: vec![
                    RelationSpec {
                        kind: "dependency_order".to_string(),
                        from: "provider".to_string(),
                        to: "publish".to_string(),
                    },
                    RelationSpec {
                        kind: "witness_order".to_string(),
                        from: "provider".to_string(),
                        to: "handoff".to_string(),
                    },
                    RelationSpec {
                        kind: "publication_order".to_string(),
                        from: "publish".to_string(),
                        to: "handoff".to_string(),
                    },
                ],
            }),
            modal_spec: None,
            model_kind: None,
            current_owner: Some("Bob".to_string()),
            visible_history: vec![
                "provider_roll(receipt)".to_string(),
                "publish(draw)".to_string(),
                "handoff(Alice -> Bob)".to_string(),
            ],
            witness_core_fields: vec![
                "witness_kind".to_string(),
                "action_ref".to_string(),
                "draw_slot".to_string(),
                "draw_result".to_string(),
            ],
            proof_obligations: proof_obligations(&["provider_returns_draw_not_room_commit"]),
            property: None,
            checked_under: None,
            explanation: Some(
                "delegated_rng_service stays on a provider boundary and leaves room mutation to the authority holder.".to_string(),
            ),
            required_source_tokens: vec!["provider_receipt".to_string(), "delegated_rng_roll".to_string()],
            layer_signatures: vec![layer_signature(
                "transport_provider_boundary",
                &[
                    "runtime_service:delegated_rng_roll",
                    "input_signature:witness(provider_receipt)",
                    "mode_assumption:provider_boundary",
                ],
                &[
                    "evidence:provider_receipt",
                    "effect:rng",
                    "adapter:provider_boundary",
                ],
                &[
                    "term_signatures -> provider_receipt evidence",
                    "provider result -> authority handoff input",
                ],
                &[
                    "requires witness(provider_receipt)",
                    "effect row { rng, witness } <= { rng, witness, publish }",
                ],
                &["witness:provider_receipt", "debug_trace:provider_roll(receipt)"],
                &[
                    "no_hidden_effect",
                    "evidence_preservation",
                    "residual_obligations_are_explicit",
                ],
            )],
        },
        CleanNearEndSampleSpec {
            id: "06_auditable_authority_witness".to_string(),
            family: CleanSampleFamily::OrderHandoff,
            source_relpath: "order-handoff/06_auditable_authority_witness.mir".to_string(),
            summary: "Authority witness records the draw and handoff without turning the witness schema into a builtin.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec!["AuthorityDrawWitness".to_string(), "draw_pub".to_string()],
            principals: vec![alice_admin.clone(), bob_holder.clone()],
            constraints: vec![Constraint {
                kind: ConstraintKind::RequiresWitness,
                display: "requires witness(draw_pub)".to_string(),
                source_ref: source_ref("06_auditable_authority_witness", "audit"),
                evaluation: ConstraintEvaluation::RequiresWitness {
                    witness: "draw_pub".to_string(),
                    producer_stage: "publish".to_string(),
                    consumer_stage: "audit".to_string(),
                },
            }],
            reason_family: None,
            entered_evaluation_on_success: true,
            terminal_outcome: Some("success".to_string()),
            order_spec: Some(OrderSpec {
                stages: vec![
                    "roll".to_string(),
                    "publish".to_string(),
                    "handoff".to_string(),
                    "audit".to_string(),
                ],
                produced_witnesses: HashMap::from([("draw_pub".to_string(), "publish".to_string())]),
                derived_relations: vec![
                    RelationSpec {
                        kind: "witness_order".to_string(),
                        from: "publish".to_string(),
                        to: "audit".to_string(),
                    },
                    RelationSpec {
                        kind: "scoped_happens_before".to_string(),
                        from: "roll".to_string(),
                        to: "audit".to_string(),
                    },
                ],
            }),
            modal_spec: None,
            model_kind: None,
            current_owner: Some("Bob".to_string()),
            visible_history: vec![
                "draw".to_string(),
                "publish(draw)".to_string(),
                "handoff(Alice -> Bob)".to_string(),
                "audit(draw_pub)".to_string(),
            ],
            witness_core_fields: vec![
                "witness_kind".to_string(),
                "action_ref".to_string(),
                "draw_slot".to_string(),
                "draw_result".to_string(),
            ],
            proof_obligations: proof_obligations(&["authority_witness_preserves_subject_identity"]),
            property: None,
            checked_under: None,
            explanation: Some(
                "The witness core fields are declared by the sample package, not by the language runtime.".to_string(),
            ),
            required_source_tokens: vec!["witness_kind".to_string(), "draw_pub".to_string()],
            layer_signatures: vec![layer_signature(
                "auth_authority_witness",
                &[
                    "input_signature:witness(draw_pub)",
                    "runtime_service:audit(draw_pub)",
                    "mode_assumption:authority_handoff_trace",
                ],
                &[
                    "evidence:AuthorityDrawWitness",
                    "view:audit_trace",
                    "signatures:witness_core_fields",
                ],
                &[
                    "witness payload -> audit evidence",
                    "visible history -> authority trace view",
                ],
                &["requires witness(draw_pub)"],
                &["debug_trace:audit(draw_pub)", "witness:draw_pub"],
                &[
                    "no_hidden_authority",
                    "evidence_preservation",
                    "visualization_respects_labels",
                ],
            )],
        },
    ];

    specs.extend(order_specs);
    specs.extend(vec![
        CleanNearEndSampleSpec {
            id: "01_peterson_sc_pass".to_string(),
            family: CleanSampleFamily::ModelCheck,
            source_relpath: "model-check/01_peterson_sc_pass.mir".to_string(),
            summary: "Peterson under sequential consistency preserves mutual exclusion.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec!["PetersonSC".to_string(), "mutual_exclusion".to_string()],
            principals: Vec::new(),
            constraints: Vec::new(),
            reason_family: None,
            entered_evaluation_on_success: false,
            terminal_outcome: None,
            order_spec: None,
            modal_spec: None,
            model_kind: Some(ModelKind::PetersonSc),
            current_owner: None,
            visible_history: Vec::new(),
            witness_core_fields: Vec::new(),
            proof_obligations: proof_obligations(&["peterson_sc_mutual_exclusion"]),
            property: Some("mutual_exclusion".to_string()),
            checked_under: Some("sequential_consistency".to_string()),
            explanation: None,
            required_source_tokens: vec!["sequential_consistency".to_string()],
            layer_signatures: vec![layer_signature(
                "verification_model_check",
                &[
                    "property:mutual_exclusion",
                    "runtime_service:model_check",
                    "mode_assumption:second_line_verification",
                ],
                &["evidence:model_check_result", "view:counterexample_shape"],
                &["runtime events -> verification evidence"],
                &[
                    "checked_under:sequential_consistency",
                    "property:mutual_exclusion",
                ],
                &["verification_result:pass"],
                &[
                    "residual_obligations_are_explicit",
                    "evidence_preservation",
                ],
            )],
        },
        CleanNearEndSampleSpec {
            id: "02_peterson_relaxed_counterexample".to_string(),
            family: CleanSampleFamily::ModelCheck,
            source_relpath: "model-check/02_peterson_relaxed_counterexample.mir".to_string(),
            summary: "Without publication/observation edges, Peterson admits a counterexample.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec![
                "PetersonRelaxedNoPublication".to_string(),
                "mutual_exclusion".to_string(),
            ],
            principals: Vec::new(),
            constraints: Vec::new(),
            reason_family: None,
            entered_evaluation_on_success: false,
            terminal_outcome: None,
            order_spec: None,
            modal_spec: None,
            model_kind: Some(ModelKind::PetersonRelaxed),
            current_owner: None,
            visible_history: Vec::new(),
            witness_core_fields: Vec::new(),
            proof_obligations: proof_obligations(&["publication_observation_edges_are_required"]),
            property: Some("mutual_exclusion".to_string()),
            checked_under: Some("relaxed_without_publication_observation_edges".to_string()),
            explanation: None,
            required_source_tokens: vec!["relaxed_without_publication_observation_edges".to_string()],
            layer_signatures: Vec::new(),
        },
        CleanNearEndSampleSpec {
            id: "03_broken_mutex_counterexample".to_string(),
            family: CleanSampleFamily::ModelCheck,
            source_relpath: "model-check/03_broken_mutex_counterexample.mir".to_string(),
            summary: "Broken mutex algorithm fails under ordinary interleaving and belongs to second-line model checking.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec!["BrokenMutex".to_string(), "mutual_exclusion".to_string()],
            principals: Vec::new(),
            constraints: Vec::new(),
            reason_family: None,
            entered_evaluation_on_success: false,
            terminal_outcome: None,
            order_spec: None,
            modal_spec: None,
            model_kind: Some(ModelKind::BrokenMutex),
            current_owner: None,
            visible_history: Vec::new(),
            witness_core_fields: Vec::new(),
            proof_obligations: proof_obligations(&["broken_mutex_requires_model_check"]),
            property: Some("mutual_exclusion".to_string()),
            checked_under: Some("interleaving_sc".to_string()),
            explanation: Some(
                "interleaving or visibility permits both actors to enter critical section".to_string(),
            ),
            required_source_tokens: vec!["BrokenMutex".to_string(), "critical A".to_string()],
            layer_signatures: Vec::new(),
        },
        CleanNearEndSampleSpec {
            id: "01_stage_stable_later_minimal".to_string(),
            family: CleanSampleFamily::Modal,
            source_relpath: "modal/01_stage_stable_later_minimal.mir".to_string(),
            summary: "Minimal stable/later mode bridge without exposing raw circle/box surface tokens.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec!["GameConfig".to_string(), "draw_pub".to_string()],
            principals: Vec::new(),
            constraints: vec![Constraint {
                kind: ConstraintKind::EffectRowSubset,
                display: "effect row { later, publish } <= { later, publish, witness }".to_string(),
                source_ref: source_ref("01_stage_stable_later_minimal", "publish"),
                evaluation: ConstraintEvaluation::EffectRowSubset {
                    lhs: ["later", "publish"]
                        .into_iter()
                        .map(|value| value.to_string())
                        .collect(),
                    rhs: ["later", "publish", "witness"]
                        .into_iter()
                        .map(|value| value.to_string())
                        .collect(),
                },
            }],
            reason_family: None,
            entered_evaluation_on_success: false,
            terminal_outcome: None,
            order_spec: None,
            modal_spec: Some(ModalSpec {
                mode_constraints: vec![
                    "config : stable".to_string(),
                    "draw available at later stage".to_string(),
                    "publish requires later draw and produces witness".to_string(),
                ],
            }),
            model_kind: None,
            current_owner: None,
            visible_history: Vec::new(),
            witness_core_fields: vec!["draw_pub".to_string()],
            proof_obligations: proof_obligations(&["stable_values_are_reusable_across_stages"]),
            property: None,
            checked_under: None,
            explanation: None,
            required_source_tokens: vec!["stable config".to_string(), "later".to_string()],
            layer_signatures: Vec::new(),
        },
        CleanNearEndSampleSpec {
            id: "02_published_witnessed_mode_bridge".to_string(),
            family: CleanSampleFamily::Modal,
            source_relpath: "modal/02_published_witnessed_mode_bridge.mir".to_string(),
            summary: "Published(room) to witnessed(draw_pub) bridge represented as explicit mode evidence.".to_string(),
            built_in_terms: built_in_vocabulary(),
            user_defined_terms: vec!["published(room)".to_string(), "witnessed(draw_pub)".to_string()],
            principals: Vec::new(),
            constraints: vec![
                Constraint {
                    kind: ConstraintKind::RequiresWitness,
                    display: "requires witness(draw_pub)".to_string(),
                    source_ref: source_ref("02_published_witnessed_mode_bridge", "bridge"),
                    evaluation: ConstraintEvaluation::RequiresWitness {
                        witness: "draw_pub".to_string(),
                        producer_stage: "publish".to_string(),
                        consumer_stage: "bridge".to_string(),
                    },
                },
                Constraint {
                    kind: ConstraintKind::RequiresPublication,
                    display: "publish(draw) must precede mode bridge".to_string(),
                    source_ref: source_ref("02_published_witnessed_mode_bridge", "bridge"),
                    evaluation: ConstraintEvaluation::RequiresPublication {
                        publication_stage: "publish".to_string(),
                        consumer_stage: "bridge".to_string(),
                    },
                },
            ],
            reason_family: None,
            entered_evaluation_on_success: false,
            terminal_outcome: None,
            order_spec: Some(OrderSpec {
                stages: vec!["publish".to_string(), "bridge".to_string()],
                produced_witnesses: HashMap::from([("draw_pub".to_string(), "publish".to_string())]),
                derived_relations: vec![
                    RelationSpec {
                        kind: "publication_order".to_string(),
                        from: "publish".to_string(),
                        to: "bridge".to_string(),
                    },
                    RelationSpec {
                        kind: "witness_order".to_string(),
                        from: "publish".to_string(),
                        to: "bridge".to_string(),
                    },
                ],
            }),
            modal_spec: Some(ModalSpec {
                mode_constraints: vec![
                    "value draw @ published(room)".to_string(),
                    "bridge consumes published draw".to_string(),
                    "bridge produces witnessed(draw_pub)".to_string(),
                ],
            }),
            model_kind: None,
            current_owner: None,
            visible_history: vec!["publish(draw)".to_string(), "witness(draw_pub)".to_string()],
            witness_core_fields: vec!["draw_pub".to_string()],
            proof_obligations: proof_obligations(&["published_mode_implies_witnessed_bridge_subject"]),
            property: None,
            checked_under: None,
            explanation: None,
            required_source_tokens: vec!["published(room)".to_string(), "witnessed(draw_pub)".to_string()],
            layer_signatures: Vec::new(),
        },
    ]);

    specs
}

fn canonicalize_if_exists(path: &Path) -> Result<PathBuf, CleanNearEndError> {
    fs::canonicalize(path).map_err(|error| {
        CleanNearEndError::new(format!(
            "failed to canonicalize {}: {error}",
            path.display()
        ))
    })
}

fn resolve_clean_near_end_sample_spec(
    sample_argument: &str,
) -> Result<CleanNearEndSampleSpec, CleanNearEndError> {
    let trimmed = sample_argument.trim();
    if trimmed.is_empty() {
        return Err(CleanNearEndError::new("sample argument must not be empty"));
    }

    let specs = clean_near_end_sample_specs();
    let root = clean_near_end_samples_root();
    let direct_path = PathBuf::from(trimmed);
    if direct_path.exists() {
        let canonical = canonicalize_if_exists(&direct_path)?;
        for spec in specs {
            let spec_path = root.join(&spec.source_relpath);
            if canonicalize_if_exists(&spec_path)? == canonical {
                return Ok(spec);
            }
        }
        return Err(CleanNearEndError::new(format!(
            "clean near-end sample path is outside the active suite: {}",
            direct_path.display()
        )));
    }

    let normalized = trimmed
        .trim_end_matches(".mir")
        .trim_start_matches("samples/clean-near-end/")
        .trim_start_matches("typing/")
        .trim_start_matches("order-handoff/")
        .trim_start_matches("model-check/")
        .trim_start_matches("modal/")
        .to_string();

    for spec in specs {
        let stem = Path::new(&spec.source_relpath)
            .file_stem()
            .and_then(|value| value.to_str())
            .unwrap_or_default()
            .to_string();
        if spec.id == normalized || stem == normalized {
            return Ok(spec);
        }
    }

    Err(CleanNearEndError::new(format!(
        "clean near-end sample not found: {trimmed}"
    )))
}

fn validate_source_shape(
    spec: &CleanNearEndSampleSpec,
    source: &str,
) -> Result<(), CleanNearEndError> {
    for token in &spec.required_source_tokens {
        if !source.contains(token) {
            return Err(CleanNearEndError::new(format!(
                "sample {} no longer contains required token `{token}`",
                spec.id
            )));
        }
    }
    Ok(())
}

pub fn list_clean_near_end_samples() -> Vec<CleanNearEndSampleSummary> {
    let root = clean_near_end_samples_root();
    clean_near_end_sample_specs()
        .into_iter()
        .map(|spec| CleanNearEndSampleSummary {
            sample_id: spec.id,
            family: spec.family,
            source_path: root.join(spec.source_relpath).display().to_string(),
            summary: spec.summary,
        })
        .collect()
}

pub fn proof_obligation_index() -> BTreeMap<String, Vec<String>> {
    clean_near_end_sample_specs()
        .into_iter()
        .map(|spec| {
            (
                spec.id,
                spec.proof_obligations
                    .iter()
                    .map(|obligation| obligation.obligation.clone())
                    .collect(),
            )
        })
        .collect()
}

pub fn run_clean_near_end_sample(
    sample_argument: &str,
) -> Result<CleanNearEndSampleReport, CleanNearEndError> {
    let spec = resolve_clean_near_end_sample_spec(sample_argument)?;
    let source_path = clean_near_end_samples_root().join(&spec.source_relpath);
    let source = fs::read_to_string(&source_path).map_err(|error| {
        CleanNearEndError::new(format!(
            "failed to read clean near-end sample {}: {error}",
            source_path.display()
        ))
    })?;
    validate_source_shape(&spec, &source)?;

    let solver = ConstraintSolver::new(&spec);
    let solved = solver.solve(&spec, &spec.constraints);
    let message_envelopes = message_envelopes_for_spec(&spec);
    let visualization_views =
        visualization_views_for_spec(&spec, &spec.layer_signatures, &message_envelopes);
    let telemetry_rows = telemetry_rows_for_spec(&spec, &spec.layer_signatures, &message_envelopes);

    let mut report = CleanNearEndSampleReport {
        sample: spec.id.clone(),
        family: spec.family,
        source_path: source_path.display().to_string(),
        static_verdict: None,
        entered_evaluation: false,
        terminal_outcome: None,
        reason_family: None,
        constraints_solved: solved.solved,
        constraints_failed: solved.failed,
        residual_obligations: solved.residuals,
        relations: Vec::new(),
        mode_constraints: spec
            .modal_spec
            .as_ref()
            .map(|modal| modal.mode_constraints.clone())
            .unwrap_or_default(),
        model_check_result: None,
        property: spec.property.clone(),
        checked_under: spec.checked_under.clone(),
        counterexample_shape: Vec::new(),
        explanation: spec.explanation.clone(),
        built_in_terms: spec.built_in_terms.clone(),
        user_defined_terms: spec.user_defined_terms.clone(),
        theorem_obligations: spec
            .proof_obligations
            .iter()
            .map(|obligation| obligation.obligation.clone())
            .collect(),
        witness_core_fields: spec.witness_core_fields.clone(),
        current_owner: spec.current_owner.clone(),
        visible_history: spec.visible_history.clone(),
        term_signatures: term_signatures_for_spec(&spec, &source),
        layer_signatures: spec.layer_signatures.clone(),
        message_envelopes,
        visualization_views,
        telemetry_rows,
    };

    match spec.family {
        CleanSampleFamily::Typing | CleanSampleFamily::OrderHandoff | CleanSampleFamily::Modal => {
            report.static_verdict = Some(match solved.verdict {
                ConstraintVerdict::Valid => "valid".to_string(),
                ConstraintVerdict::Invalid => "malformed".to_string(),
                ConstraintVerdict::Residual => "residual".to_string(),
            });
            if solved.verdict == ConstraintVerdict::Valid {
                report.entered_evaluation = spec.entered_evaluation_on_success;
                report.terminal_outcome = spec.terminal_outcome.clone();
                if let Some(order_spec) = &spec.order_spec {
                    report.relations = order_spec
                        .derived_relations
                        .iter()
                        .map(|relation| {
                            [
                                relation.kind.clone(),
                                relation.from.clone(),
                                relation.to.clone(),
                            ]
                        })
                        .collect();
                }
            } else {
                report.reason_family = spec.reason_family.clone();
            }
        }
        CleanSampleFamily::ModelCheck => {
            report.static_verdict = Some("valid".to_string());
            let outcome = run_model_check(spec.model_kind.expect("model kind"));
            report.model_check_result = Some(outcome.result.to_string());
            report.counterexample_shape = outcome.counterexample_shape;
            report.explanation = outcome.explanation.or(report.explanation);
        }
    }

    Ok(report)
}

pub fn run_clean_near_end_family(
    family: CleanSampleFamily,
) -> Result<Vec<CleanNearEndSampleReport>, CleanNearEndError> {
    let mut reports = Vec::new();
    for sample in list_clean_near_end_samples() {
        if sample.family == family {
            reports.push(run_clean_near_end_sample(&sample.sample_id)?);
        }
    }
    Ok(reports)
}

pub fn build_clean_near_end_matrix() -> Result<CleanNearEndMatrix, CleanNearEndError> {
    let mut families = BTreeMap::new();
    let mut valid_samples = Vec::new();
    let mut malformed_samples = Vec::new();
    let mut model_check_pass = Vec::new();
    let mut model_check_counterexample = Vec::new();

    let samples = list_clean_near_end_samples();
    for sample in &samples {
        *families
            .entry(sample.family.as_str().to_string())
            .or_insert(0) += 1;
        let report = run_clean_near_end_sample(&sample.sample_id)?;
        match sample.family {
            CleanSampleFamily::ModelCheck => match report.model_check_result.as_deref() {
                Some("pass") => model_check_pass.push(report.sample),
                Some("counterexample") => model_check_counterexample.push(report.sample),
                _ => {}
            },
            _ => match report.static_verdict.as_deref() {
                Some("valid") => valid_samples.push(report.sample),
                Some("malformed") => malformed_samples.push(report.sample),
                _ => {}
            },
        }
    }

    Ok(CleanNearEndMatrix {
        total_samples: samples.len(),
        families,
        valid_samples,
        malformed_samples,
        model_check_pass,
        model_check_counterexample,
    })
}

pub fn build_clean_near_end_closeout() -> Result<CleanNearEndCloseout, CleanNearEndError> {
    let mut families = BTreeMap::new();
    for sample in list_clean_near_end_samples() {
        families
            .entry(sample.family.as_str().to_string())
            .or_insert_with(Vec::new)
            .push(sample.sample_id);
    }
    let signature_kinds = closeout_signature_kinds()?;
    let signature_evidence_roles = closeout_signature_evidence_roles()?;
    let visualization_views = closeout_visualization_views();
    let telemetry_rows = closeout_telemetry_rows();
    let telemetry_channels = closeout_telemetry_channels(&telemetry_rows);
    Ok(CleanNearEndCloseout {
        active_sample_root: clean_near_end_samples_root().display().to_string(),
        archive_sample_root: clean_near_end_archive_root().display().to_string(),
        built_in_vocabulary: built_in_vocabulary(),
        user_defined_vocabulary: user_defined_vocabulary(),
        families,
        proof_samples: proof_obligation_index().keys().cloned().collect(),
        lean_roots: vec![
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../samples/lean/foundations")
                .display()
                .to_string(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../samples/lean/clean-near-end")
                .display()
                .to_string(),
        ],
        signature_lanes: signature_lanes(),
        signature_scope: "clean_near_end_canonical_inventory".to_string(),
        signature_kinds,
        signature_evidence_roles,
        reserved_signature_kinds: vec![
            "message".to_string(),
            "adapter".to_string(),
            "layer".to_string(),
        ],
        message_envelope_lanes: message_envelope_lanes(),
        auth_evidence_kinds: vec!["none".to_string()],
        reserved_auth_evidence_kinds: vec!["session_token".to_string(), "signature".to_string()],
        transport_seams: vec![
            "provider_boundary".to_string(),
            "audit_trace_boundary".to_string(),
        ],
        reserved_transport_seams: vec!["loopback_socket".to_string(), "network_link".to_string()],
        visualization_views,
        visualization_view_lanes: visualization_view_lanes(),
        reserved_visualization_view_names: reserved_visualization_view_names(),
        telemetry_rows,
        telemetry_row_lanes: telemetry_row_lanes(),
        telemetry_channels,
        reserved_telemetry_channels: reserved_telemetry_channels(),
        layer_signatures: closeout_layer_signatures(),
        layer_signature_lanes: layer_signature_lanes(),
        reserved_layer_signature_names: reserved_layer_signature_names(),
    })
}

#[derive(Debug)]
struct ModelCheckOutcome {
    result: ModelCheckResult,
    counterexample_shape: Vec<String>,
    explanation: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModelCheckResult {
    Pass,
    Counterexample,
}

impl fmt::Display for ModelCheckResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pass => f.write_str("pass"),
            Self::Counterexample => f.write_str("counterexample"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Actor {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PetersonPc {
    Start,
    SetTurn,
    Wait,
    Critical,
    Exit,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PetersonState {
    pc_a: PetersonPc,
    pc_b: PetersonPc,
    flag_a: bool,
    flag_b: bool,
    turn: Actor,
    in_critical_a: bool,
    in_critical_b: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PetersonRelaxedState {
    pc_a: PetersonPc,
    pc_b: PetersonPc,
    actual_flag_a: bool,
    actual_flag_b: bool,
    actual_turn: Actor,
    view_a_flag_b: bool,
    view_b_flag_a: bool,
    view_a_turn: Actor,
    view_b_turn: Actor,
    in_critical_a: bool,
    in_critical_b: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BrokenMutexPc {
    Check,
    SetFlag,
    Critical,
    Exit,
    Done,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BrokenMutexState {
    pc_a: BrokenMutexPc,
    pc_b: BrokenMutexPc,
    flag_a: bool,
    flag_b: bool,
    gate_a_open: bool,
    gate_b_open: bool,
    in_critical_a: bool,
    in_critical_b: bool,
}

fn run_model_check(kind: ModelKind) -> ModelCheckOutcome {
    match kind {
        ModelKind::PetersonSc => peterson_sc_outcome(),
        ModelKind::PetersonRelaxed => peterson_relaxed_outcome(),
        ModelKind::BrokenMutex => broken_mutex_outcome(),
    }
}

fn peterson_sc_outcome() -> ModelCheckOutcome {
    let initial = PetersonState {
        pc_a: PetersonPc::Start,
        pc_b: PetersonPc::Start,
        flag_a: false,
        flag_b: false,
        turn: Actor::A,
        in_critical_a: false,
        in_critical_b: false,
    };
    let mut queue = VecDeque::from([initial]);
    let mut seen = HashSet::new();
    while let Some(state) = queue.pop_front() {
        if !seen.insert(state.clone()) {
            continue;
        }
        if state.in_critical_a && state.in_critical_b {
            return ModelCheckOutcome {
                result: ModelCheckResult::Counterexample,
                counterexample_shape: vec!["unexpected simultaneous critical section".to_string()],
                explanation: Some("Peterson SC should not fail here".to_string()),
            };
        }
        for actor in [Actor::A, Actor::B] {
            if let Some(next) = peterson_step_sc(&state, actor) {
                queue.push_back(next);
            }
        }
    }
    ModelCheckOutcome {
        result: ModelCheckResult::Pass,
        counterexample_shape: Vec::new(),
        explanation: None,
    }
}

fn peterson_step_sc(state: &PetersonState, actor: Actor) -> Option<PetersonState> {
    let mut next = state.clone();
    match actor {
        Actor::A => match state.pc_a {
            PetersonPc::Start => {
                next.flag_a = true;
                next.pc_a = PetersonPc::SetTurn;
            }
            PetersonPc::SetTurn => {
                next.turn = Actor::B;
                next.pc_a = PetersonPc::Wait;
            }
            PetersonPc::Wait => {
                if state.flag_b && state.turn == Actor::B {
                    return None;
                }
                next.in_critical_a = true;
                next.pc_a = PetersonPc::Critical;
            }
            PetersonPc::Critical => {
                next.in_critical_a = false;
                next.pc_a = PetersonPc::Exit;
            }
            PetersonPc::Exit => {
                next.flag_a = false;
                next.pc_a = PetersonPc::Done;
            }
            PetersonPc::Done => return None,
        },
        Actor::B => match state.pc_b {
            PetersonPc::Start => {
                next.flag_b = true;
                next.pc_b = PetersonPc::SetTurn;
            }
            PetersonPc::SetTurn => {
                next.turn = Actor::A;
                next.pc_b = PetersonPc::Wait;
            }
            PetersonPc::Wait => {
                if state.flag_a && state.turn == Actor::A {
                    return None;
                }
                next.in_critical_b = true;
                next.pc_b = PetersonPc::Critical;
            }
            PetersonPc::Critical => {
                next.in_critical_b = false;
                next.pc_b = PetersonPc::Exit;
            }
            PetersonPc::Exit => {
                next.flag_b = false;
                next.pc_b = PetersonPc::Done;
            }
            PetersonPc::Done => return None,
        },
    }
    Some(next)
}

fn peterson_relaxed_outcome() -> ModelCheckOutcome {
    let initial = PetersonRelaxedState {
        pc_a: PetersonPc::Start,
        pc_b: PetersonPc::Start,
        actual_flag_a: false,
        actual_flag_b: false,
        actual_turn: Actor::A,
        view_a_flag_b: false,
        view_b_flag_a: false,
        view_a_turn: Actor::A,
        view_b_turn: Actor::A,
        in_critical_a: false,
        in_critical_b: false,
    };
    let mut queue = VecDeque::from([(initial, Vec::<String>::new())]);
    let mut seen = HashSet::new();
    while let Some((state, path)) = queue.pop_front() {
        if !seen.insert(state.clone()) {
            continue;
        }
        if state.in_critical_a && state.in_critical_b {
            return ModelCheckOutcome {
                result: ModelCheckResult::Counterexample,
                counterexample_shape: path,
                explanation: None,
            };
        }
        for actor in [Actor::A, Actor::B] {
            if let Some((next, event)) = peterson_step_relaxed(&state, actor) {
                let mut next_path = path.clone();
                next_path.push(event);
                queue.push_back((next, next_path));
            }
        }
    }
    ModelCheckOutcome {
        result: ModelCheckResult::Pass,
        counterexample_shape: Vec::new(),
        explanation: Some("expected a counterexample but none was found".to_string()),
    }
}

fn peterson_step_relaxed(
    state: &PetersonRelaxedState,
    actor: Actor,
) -> Option<(PetersonRelaxedState, String)> {
    let mut next = state.clone();
    match actor {
        Actor::A => match state.pc_a {
            PetersonPc::Start => {
                next.actual_flag_a = true;
                next.pc_a = PetersonPc::SetTurn;
                Some((
                    next,
                    "A writes flag[A] but B has not observed it".to_string(),
                ))
            }
            PetersonPc::SetTurn => {
                next.actual_turn = Actor::B;
                next.view_a_turn = Actor::B;
                next.pc_a = PetersonPc::Wait;
                Some((next, "A writes turn <- B".to_string()))
            }
            PetersonPc::Wait => {
                if state.view_a_flag_b {
                    return None;
                }
                next.in_critical_a = true;
                next.pc_a = PetersonPc::Critical;
                Some((next, "A reads flag[B] as false".to_string()))
            }
            PetersonPc::Critical => {
                next.in_critical_a = false;
                next.pc_a = PetersonPc::Exit;
                Some((next, "A enters critical".to_string()))
            }
            PetersonPc::Exit => {
                next.actual_flag_a = false;
                next.pc_a = PetersonPc::Done;
                Some((next, "A clears flag[A]".to_string()))
            }
            PetersonPc::Done => None,
        },
        Actor::B => match state.pc_b {
            PetersonPc::Start => {
                next.actual_flag_b = true;
                next.pc_b = PetersonPc::SetTurn;
                Some((
                    next,
                    "B writes flag[B] but A has not observed it".to_string(),
                ))
            }
            PetersonPc::SetTurn => {
                next.actual_turn = Actor::A;
                next.view_b_turn = Actor::A;
                next.pc_b = PetersonPc::Wait;
                Some((next, "B writes turn <- A".to_string()))
            }
            PetersonPc::Wait => {
                if state.view_b_flag_a {
                    return None;
                }
                next.in_critical_b = true;
                next.pc_b = PetersonPc::Critical;
                Some((next, "B reads flag[A] as false".to_string()))
            }
            PetersonPc::Critical => {
                next.in_critical_b = false;
                next.pc_b = PetersonPc::Exit;
                Some((next, "B enters critical".to_string()))
            }
            PetersonPc::Exit => {
                next.actual_flag_b = false;
                next.pc_b = PetersonPc::Done;
                Some((next, "B clears flag[B]".to_string()))
            }
            PetersonPc::Done => None,
        },
    }
}

fn broken_mutex_outcome() -> ModelCheckOutcome {
    let initial = BrokenMutexState {
        pc_a: BrokenMutexPc::Check,
        pc_b: BrokenMutexPc::Check,
        flag_a: false,
        flag_b: false,
        gate_a_open: false,
        gate_b_open: false,
        in_critical_a: false,
        in_critical_b: false,
    };
    let mut queue = VecDeque::from([(initial, Vec::<String>::new())]);
    let mut seen = HashSet::new();
    while let Some((state, path)) = queue.pop_front() {
        if !seen.insert(state.clone()) {
            continue;
        }
        if state.in_critical_a && state.in_critical_b {
            return ModelCheckOutcome {
                result: ModelCheckResult::Counterexample,
                counterexample_shape: path,
                explanation: Some(
                    "interleaving or visibility permits both actors to enter critical section"
                        .to_string(),
                ),
            };
        }
        for actor in [Actor::A, Actor::B] {
            if let Some((next, event)) = broken_mutex_step(&state, actor) {
                let mut next_path = path.clone();
                next_path.push(event);
                queue.push_back((next, next_path));
            }
        }
    }
    ModelCheckOutcome {
        result: ModelCheckResult::Pass,
        counterexample_shape: Vec::new(),
        explanation: Some("expected a broken mutex counterexample".to_string()),
    }
}

fn broken_mutex_step(state: &BrokenMutexState, actor: Actor) -> Option<(BrokenMutexState, String)> {
    let mut next = state.clone();
    match actor {
        Actor::A => match state.pc_a {
            BrokenMutexPc::Check => {
                next.gate_a_open = !state.flag_b;
                next.pc_a = BrokenMutexPc::SetFlag;
                Some((next, "A checks flag[B] and sees false".to_string()))
            }
            BrokenMutexPc::SetFlag => {
                if !state.gate_a_open {
                    next.pc_a = BrokenMutexPc::Done;
                    return Some((next, "A gives up".to_string()));
                }
                next.flag_a = true;
                next.pc_a = BrokenMutexPc::Critical;
                Some((next, "A sets flag[A]".to_string()))
            }
            BrokenMutexPc::Critical => {
                next.in_critical_a = true;
                next.pc_a = BrokenMutexPc::Exit;
                Some((next, "A enters critical".to_string()))
            }
            BrokenMutexPc::Exit => {
                next.in_critical_a = false;
                next.flag_a = false;
                next.pc_a = BrokenMutexPc::Done;
                Some((next, "A clears flag[A]".to_string()))
            }
            BrokenMutexPc::Done => None,
        },
        Actor::B => match state.pc_b {
            BrokenMutexPc::Check => {
                next.gate_b_open = !state.flag_a;
                next.pc_b = BrokenMutexPc::SetFlag;
                Some((next, "B checks flag[A] and sees false".to_string()))
            }
            BrokenMutexPc::SetFlag => {
                if !state.gate_b_open {
                    next.pc_b = BrokenMutexPc::Done;
                    return Some((next, "B gives up".to_string()));
                }
                next.flag_b = true;
                next.pc_b = BrokenMutexPc::Critical;
                Some((next, "B sets flag[B]".to_string()))
            }
            BrokenMutexPc::Critical => {
                next.in_critical_b = true;
                next.pc_b = BrokenMutexPc::Exit;
                Some((next, "B enters critical".to_string()))
            }
            BrokenMutexPc::Exit => {
                next.in_critical_b = false;
                next.flag_b = false;
                next.pc_b = BrokenMutexPc::Done;
                Some((next, "B clears flag[B]".to_string()))
            }
            BrokenMutexPc::Done => None,
        },
    }
}

pub fn clean_near_end_json_value(report: &CleanNearEndSampleReport) -> serde_json::Value {
    json!(report)
}
