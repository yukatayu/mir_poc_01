use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    CurrentL2Fixture, EffectInput, EffectOracle, EffectVerdict, ExpectedNonAdmissibleMetadata,
    InterpreterError, NonAdmissibleMetadata, PlaceStore, PredicateInput, PredicateOracle,
    PredicateSite, PredicateVerdict, RequestMode, RunReport, SuccessCarrier, run_to_completion,
};
use serde::Deserialize;

pub const CURRENT_L2_HOST_PLAN_SCHEMA_VERSION: &str = "current-l2-host-plan-v0";

/// current L2 host harness が declarative に持つ最小 store mutation。
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind")]
pub enum FixtureStoreMutation {
    #[serde(rename = "append-record")]
    AppendRecord { target: String, record: String },
}

impl FixtureStoreMutation {
    pub fn append_record(target: impl Into<String>, record: impl Into<String>) -> Self {
        Self::AppendRecord {
            target: target.into(),
            record: record.into(),
        }
    }
}

/// effect success が返す current L2 harness 用 commit carrier。
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct FixtureCommitPlan {
    #[serde(default)]
    pub mutations: Vec<FixtureStoreMutation>,
}

impl FixtureCommitPlan {
    pub fn new(mutations: Vec<FixtureStoreMutation>) -> Self {
        Self { mutations }
    }
}

impl SuccessCarrier for FixtureCommitPlan {
    fn preview_place_store(&self, place_store: &PlaceStore) -> PlaceStore {
        let mut preview = place_store.clone();
        apply_mutations(&mut preview, &self.mutations);
        preview
    }

    fn apply_place_store(self, place_store: &mut PlaceStore) {
        apply_mutations(place_store, &self.mutations);
    }
}

/// predicate verdict を declarative に差し替える current L2 用 rule。
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PredicatePlanRule {
    pub site: PredicateSite,
    pub op: String,
    #[serde(default)]
    pub mode: Option<RequestMode>,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub chain_ref: Option<String>,
    #[serde(default)]
    pub option_ref: Option<String>,
    pub verdict: PredicateVerdict,
}

impl PredicatePlanRule {
    pub fn unsatisfied(site: PredicateSite, op: impl Into<String>) -> Self {
        Self {
            site,
            op: op.into(),
            mode: None,
            target: None,
            chain_ref: None,
            option_ref: None,
            verdict: PredicateVerdict::Unsatisfied,
        }
    }

    pub fn unsatisfied_for_option(
        site: PredicateSite,
        op: impl Into<String>,
        option_ref: impl Into<String>,
    ) -> Self {
        Self {
            site,
            op: op.into(),
            mode: None,
            target: None,
            chain_ref: None,
            option_ref: Some(option_ref.into()),
            verdict: PredicateVerdict::Unsatisfied,
        }
    }

    fn matches(&self, input: &PredicateInput) -> bool {
        self.site == input.site
            && self.op == input.op
            && self.mode.is_none_or(|mode| mode == input.mode)
            && self.target.as_ref().is_none_or(|target| Some(target) == input.target.as_ref())
            && self
                .chain_ref
                .as_ref()
                .is_none_or(|chain_ref| Some(chain_ref) == input.chain_ref.as_ref())
            && self
                .option_ref
                .as_ref()
                .is_none_or(|option_ref| Some(option_ref) == input.option_ref.as_ref())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.site == other.site
            && self.op == other.op
            && optional_field_overlaps(&self.mode, &other.mode)
            && optional_field_overlaps(&self.target, &other.target)
            && optional_field_overlaps(&self.chain_ref, &other.chain_ref)
            && optional_field_overlaps(&self.option_ref, &other.option_ref)
    }
}

/// effect outcome を declarative に差し替える current L2 用 rule。
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct EffectPlanRule {
    pub op: String,
    #[serde(default)]
    pub mode: Option<RequestMode>,
    #[serde(default)]
    pub selected_target: Option<String>,
    #[serde(default)]
    pub chain_ref: Option<String>,
    #[serde(default)]
    pub selected_option_ref: Option<String>,
    pub verdict: EffectPlanVerdict,
}

impl EffectPlanRule {
    pub fn success_on(
        op: impl Into<String>,
        selected_target: impl Into<String>,
        mutations: Vec<FixtureStoreMutation>,
    ) -> Self {
        Self {
            op: op.into(),
            mode: Some(RequestMode::On),
            selected_target: Some(selected_target.into()),
            chain_ref: None,
            selected_option_ref: None,
            verdict: EffectPlanVerdict::Success {
                commit: FixtureCommitPlan::new(mutations),
            },
        }
    }

    pub fn success_via(
        op: impl Into<String>,
        chain_ref: impl Into<String>,
        selected_option_ref: impl Into<String>,
        selected_target: impl Into<String>,
        mutations: Vec<FixtureStoreMutation>,
    ) -> Self {
        Self {
            op: op.into(),
            mode: Some(RequestMode::Via),
            selected_target: Some(selected_target.into()),
            chain_ref: Some(chain_ref.into()),
            selected_option_ref: Some(selected_option_ref.into()),
            verdict: EffectPlanVerdict::Success {
                commit: FixtureCommitPlan::new(mutations),
            },
        }
    }

    pub fn explicit_failure_on(
        op: impl Into<String>,
        selected_target: impl Into<String>,
    ) -> Self {
        Self {
            op: op.into(),
            mode: Some(RequestMode::On),
            selected_target: Some(selected_target.into()),
            chain_ref: None,
            selected_option_ref: None,
            verdict: EffectPlanVerdict::ExplicitFailure,
        }
    }

    fn matches(&self, input: &EffectInput) -> bool {
        self.op == input.op
            && self.mode.is_none_or(|mode| mode == input.mode)
            && self.selected_target.as_ref().is_none_or(|target| {
                target == &input.selected_target
            })
            && self
                .chain_ref
                .as_ref()
                .is_none_or(|chain_ref| Some(chain_ref) == input.chain_ref.as_ref())
            && self.selected_option_ref.as_ref().is_none_or(|option_ref| {
                Some(option_ref) == input.selected_option_ref.as_ref()
            })
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.op == other.op
            && optional_field_overlaps(&self.mode, &other.mode)
            && optional_field_overlaps(&self.selected_target, &other.selected_target)
            && optional_field_overlaps(&self.chain_ref, &other.chain_ref)
            && optional_field_overlaps(&self.selected_option_ref, &other.selected_option_ref)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "kind")]
pub enum EffectPlanVerdict {
    #[serde(rename = "success")]
    Success { commit: FixtureCommitPlan },
    #[serde(rename = "explicit-failure")]
    ExplicitFailure,
}

/// human-facing / metadata expectation override。
#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
pub struct TraceExpectationOverride {
    #[serde(default)]
    pub non_admissible_metadata: Option<Vec<NonAdmissibleMetadata>>,
    #[serde(default)]
    pub narrative_explanations: Option<Vec<String>>,
}

/// current L2 用 fixture host plan。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FixtureHostPlan {
    pub predicate_rules: Vec<PredicatePlanRule>,
    pub effect_rules: Vec<EffectPlanRule>,
    pub trace_expectation_override: TraceExpectationOverride,
}

impl FixtureHostPlan {
    pub fn validate(&self) -> Result<(), InterpreterError> {
        for (i, left) in self.predicate_rules.iter().enumerate() {
            for right in self.predicate_rules.iter().skip(i + 1) {
                if left.overlaps(right) {
                    return Err(InterpreterError::InvalidProgram(format!(
                        "overlapping predicate plan rules are forbidden: {:?} and {:?}",
                        left, right
                    )));
                }
            }
        }

        for (i, left) in self.effect_rules.iter().enumerate() {
            for right in self.effect_rules.iter().skip(i + 1) {
                if left.overlaps(right) {
                    return Err(InterpreterError::InvalidProgram(format!(
                        "overlapping effect plan rules are forbidden: {:?} and {:?}",
                        left, right
                    )));
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct FixtureHostPlanAsset {
    pub schema_version: String,
    #[serde(default)]
    pub predicate_rules: Vec<PredicatePlanRule>,
    #[serde(default)]
    pub effect_rules: Vec<EffectPlanRule>,
    #[serde(default)]
    pub trace_expectation_override: TraceExpectationOverride,
}

impl FixtureHostPlanAsset {
    fn into_plan(self) -> Result<FixtureHostPlan, InterpreterError> {
        if self.schema_version != CURRENT_L2_HOST_PLAN_SCHEMA_VERSION {
            return Err(InterpreterError::InvalidProgram(format!(
                "unsupported host plan schema version: {}",
                self.schema_version
            )));
        }

        let plan = FixtureHostPlan {
            predicate_rules: self.predicate_rules,
            effect_rules: self.effect_rules,
            trace_expectation_override: self.trace_expectation_override,
        };
        plan.validate()?;
        Ok(plan)
    }
}

pub fn host_plan_sidecar_path_for_fixture_path(
    fixture_path: impl AsRef<Path>,
) -> PathBuf {
    fixture_path.as_ref().with_extension("host-plan.json")
}

pub fn load_host_plan_from_path(
    path: impl AsRef<Path>,
) -> Result<FixtureHostPlan, InterpreterError> {
    let payload = fs::read_to_string(path)?;
    let asset: FixtureHostPlanAsset = serde_json::from_str(&payload)?;
    asset.into_plan()
}

pub fn load_host_plan_sidecar_for_fixture_path(
    fixture_path: impl AsRef<Path>,
) -> Result<FixtureHostPlan, InterpreterError> {
    load_host_plan_from_path(host_plan_sidecar_path_for_fixture_path(fixture_path))
}

/// parser-free minimal interpreter を fixture ごとに差し替える test harness。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FixtureHostStub {
    plan: FixtureHostPlan,
}

impl FixtureHostStub {
    pub fn new(plan: FixtureHostPlan) -> Result<Self, InterpreterError> {
        plan.validate()?;
        Ok(Self { plan })
    }

    pub fn plan(&self) -> &FixtureHostPlan {
        &self.plan
    }

    pub fn run_fixture(
        &self,
        fixture: &CurrentL2Fixture,
    ) -> Result<RunReport, InterpreterError> {
        let mut predicate_oracle = PlannedPredicateOracle::new(self.plan.clone());
        let mut effect_oracle = PlannedEffectOracle::new(self.plan.clone());
        let report = run_to_completion(fixture, &mut predicate_oracle, &mut effect_oracle)?;
        if !predicate_oracle.violations.is_empty() || !effect_oracle.violations.is_empty()
        {
            return Err(InterpreterError::InvalidProgram(format!(
                "host plan did not cover all oracle calls: predicate_violations={:?}, effect_violations={:?}",
                predicate_oracle.violations, effect_oracle.violations
            )));
        }
        Ok(report)
    }

    pub fn expected_non_admissible_metadata(
        &self,
        fixture: &CurrentL2Fixture,
    ) -> Vec<NonAdmissibleMetadata> {
        self.plan
            .trace_expectation_override
            .non_admissible_metadata
            .clone()
            .unwrap_or_else(|| {
                fixture
                    .expected_trace_audit
                    .non_admissible_metadata
                    .iter()
                    .map(non_admissible_from_expected)
                    .collect()
            })
    }

    pub fn expected_narrative_explanations(
        &self,
        fixture: &CurrentL2Fixture,
    ) -> Vec<String> {
        self.plan
            .trace_expectation_override
            .narrative_explanations
            .clone()
            .unwrap_or_else(|| fixture.expected_trace_audit.narrative_explanations.clone())
    }
}

#[derive(Debug, Clone)]
struct PlannedPredicateOracle {
    plan: FixtureHostPlan,
    violations: Vec<PredicateInput>,
}

impl PlannedPredicateOracle {
    fn new(plan: FixtureHostPlan) -> Self {
        Self {
            plan,
            violations: Vec::new(),
        }
    }
}

impl PredicateOracle<PredicateInput> for PlannedPredicateOracle {
    fn eval_predicate(&mut self, input: PredicateInput) -> PredicateVerdict {
        self.plan
            .predicate_rules
            .iter()
            .find(|rule| rule.matches(&input))
            .map(|rule| rule.verdict)
            .unwrap_or_else(|| {
                self.violations.push(input);
                PredicateVerdict::Unsatisfied
            })
    }
}

#[derive(Debug, Clone)]
struct PlannedEffectOracle {
    plan: FixtureHostPlan,
    violations: Vec<EffectInput>,
}

impl PlannedEffectOracle {
    fn new(plan: FixtureHostPlan) -> Self {
        Self {
            plan,
            violations: Vec::new(),
        }
    }
}

impl EffectOracle<EffectInput, FixtureCommitPlan> for PlannedEffectOracle {
    fn apply_effect(&mut self, input: EffectInput) -> EffectVerdict<FixtureCommitPlan> {
        if let Some(rule) = self.plan.effect_rules.iter().find(|rule| rule.matches(&input)) {
            return match &rule.verdict {
                EffectPlanVerdict::Success { commit } => {
                    EffectVerdict::Success {
                        commit: commit.clone(),
                    }
                }
                EffectPlanVerdict::ExplicitFailure => EffectVerdict::ExplicitFailure,
            };
        }

        self.violations.push(input);
        EffectVerdict::ExplicitFailure
    }
}

fn optional_field_overlaps<T: PartialEq>(left: &Option<T>, right: &Option<T>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left == right,
        _ => true,
    }
}

fn non_admissible_from_expected(
    expected: &ExpectedNonAdmissibleMetadata,
) -> NonAdmissibleMetadata {
    NonAdmissibleMetadata {
        option_ref: expected.option_ref.clone(),
        subreason: expected.subreason,
    }
}

fn apply_mutations(place_store: &mut PlaceStore, mutations: &[FixtureStoreMutation]) {
    for mutation in mutations {
        match mutation {
            FixtureStoreMutation::AppendRecord { target, record } => {
                place_store
                    .entry(target.clone())
                    .or_default()
                    .push(record.clone());
            }
        }
    }
}
