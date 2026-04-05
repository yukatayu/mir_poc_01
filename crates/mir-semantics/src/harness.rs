use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    CurrentL2Fixture, EffectInput, EffectOracle, EffectVerdict, ExpectedNonAdmissibleMetadata,
    FixtureRuntimeOutcome, InterpreterError, NonAdmissibleMetadata, PlaceStore, PredicateInput,
    PredicateOracle, PredicateSite, PredicateVerdict, RequestMode, RunReport, SuccessCarrier,
    TerminalOutcome, run_to_completion,
};
use serde::Deserialize;

pub const CURRENT_L2_HOST_PLAN_SCHEMA_VERSION: &str = "current-l2-host-plan-v0";

/// bundle が runtime host plan を必須とするかどうか。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixtureRuntimeRequirement {
    StaticOnly,
    RuntimeWithHostPlan,
}

/// fixture 本体と sidecar を 1 組として扱う current L2 bundle。
#[derive(Debug, Clone)]
pub struct FixtureBundle {
    pub fixture_path: PathBuf,
    pub host_plan_path: Option<PathBuf>,
    pub fixture: CurrentL2Fixture,
    pub host_plan: Option<FixtureHostPlan>,
    pub runtime_requirement: FixtureRuntimeRequirement,
}

/// bundle 単位 helper が返す最小 run report。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleRunReport {
    pub report: RunReport,
}

/// bundle discovery 時の per-fixture failure。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleDiscoveryFailure {
    pub fixture_path: PathBuf,
    pub runtime_requirement: Option<FixtureRuntimeRequirement>,
    pub error: String,
}

/// directory 単位 discovery の結果。
#[derive(Debug, Clone)]
pub struct BundleDiscoveryReport {
    pub total_candidates: usize,
    pub runtime_bundles: usize,
    pub static_only_bundles: usize,
    pub bundles: Vec<FixtureBundle>,
    pub failures: Vec<BundleDiscoveryFailure>,
}

/// batch 実行時の per-bundle 実行 failure。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleExecutionFailure {
    pub fixture_path: PathBuf,
    pub fixture_id: String,
    pub runtime_requirement: FixtureRuntimeRequirement,
    pub error: String,
}

/// bundle 単位実行の結果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchBundleOutcome {
    Passed {
        report: BundleRunReport,
    },
    Failed {
        error: String,
        host_plan_coverage_failure: bool,
    },
}

/// batch summary に載せる per-bundle report。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchBundleReport {
    pub fixture_path: PathBuf,
    pub fixture_id: String,
    pub runtime_requirement: FixtureRuntimeRequirement,
    pub outcome: BatchBundleOutcome,
}

/// directory 単位 batch run の最小 summary。
#[derive(Debug, Clone)]
pub struct BatchRunSummary {
    pub total_bundles: usize,
    pub runtime_bundles: usize,
    pub static_only_bundles: usize,
    pub passed: usize,
    pub failed: usize,
    pub discovery_failures: Vec<BundleDiscoveryFailure>,
    pub bundle_failures: Vec<BundleExecutionFailure>,
    pub host_plan_coverage_failures: Vec<BundleExecutionFailure>,
    pub bundle_reports: Vec<BatchBundleReport>,
}

/// current L2 profile layer が使う bundle class filter。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionScope {
    RuntimeOnly,
    StaticOnly,
}

/// single-fixture selection の最小 selector。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingleFixtureSelector {
    Stem(String),
    Path(PathBuf),
}

/// batch runner の上に薄く載る current L2 selection mode。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionMode {
    RuntimeOnly,
    StaticOnly,
    SingleFixture(SingleFixtureSelector),
}

/// primitive selection を組み合わせる current L2 request。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SelectionRequest {
    pub scope: Option<SelectionScope>,
    pub single_fixture: Option<SingleFixtureSelector>,
}

impl SelectionRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_scope(mut self, scope: SelectionScope) -> Self {
        self.scope = Some(scope);
        self
    }

    pub fn with_single_fixture(mut self, selector: SingleFixtureSelector) -> Self {
        self.single_fixture = Some(selector);
        self
    }

    fn from_mode(mode: &SelectionMode) -> Self {
        match mode {
            SelectionMode::RuntimeOnly => Self::new().with_scope(SelectionScope::RuntimeOnly),
            SelectionMode::StaticOnly => Self::new().with_scope(SelectionScope::StaticOnly),
            SelectionMode::SingleFixture(selector) => {
                Self::new().with_single_fixture(selector.clone())
            }
        }
    }
}

/// selected batch 実行に名前を付ける current L2 profile。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionProfile {
    pub profile_name: String,
    pub request: SelectionRequest,
}

impl SelectionProfile {
    pub fn new(profile_name: impl Into<String>, request: SelectionRequest) -> Self {
        Self {
            profile_name: profile_name.into(),
            request,
        }
    }
}

/// profile 名付きの selected batch summary。
#[derive(Debug, Clone)]
pub struct ProfileRunSummary {
    pub profile_name: String,
    pub total_selected_bundles: usize,
    pub runtime_selected_bundles: usize,
    pub static_selected_bundles: usize,
    pub passed: usize,
    pub failed: usize,
    pub discovery_failures: Vec<BundleDiscoveryFailure>,
    pub bundle_failures: Vec<BundleExecutionFailure>,
    pub host_plan_coverage_failures: Vec<BundleExecutionFailure>,
    pub bundle_reports: Vec<BatchBundleReport>,
}

/// current L2 の small named profile catalog を引く入口。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileCatalog;

#[derive(Clone, Copy)]
struct NamedProfileSpec {
    alias: &'static str,
    build_request: fn() -> SelectionRequest,
}

impl NamedProfileSpec {
    fn build_request(self) -> SelectionRequest {
        (self.build_request)()
    }
}

macro_rules! define_named_profile_catalog {
    ($(($alias:literal, $build_request:ident)),+ $(,)?) => {
        const NAMED_PROFILE_ALIASES: &[&str] = &[$($alias),+];
        const NAMED_PROFILE_SPECS: &[NamedProfileSpec] = &[
            $(NamedProfileSpec {
                alias: $alias,
                build_request: $build_request,
            }),+
        ];
    };
}

/// alias 解決後の request を含む current L2 catalog summary。
#[derive(Debug, Clone)]
pub struct NamedProfileRunSummary {
    pub profile_name: String,
    pub resolved_request: SelectionRequest,
    pub total_selected_bundles: usize,
    pub runtime_selected_bundles: usize,
    pub static_selected_bundles: usize,
    pub passed: usize,
    pub failed: usize,
    pub discovery_failures: Vec<BundleDiscoveryFailure>,
    pub bundle_failures: Vec<BundleExecutionFailure>,
    pub host_plan_coverage_failures: Vec<BundleExecutionFailure>,
    pub bundle_reports: Vec<BatchBundleReport>,
}

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
            && self
                .target
                .as_ref()
                .is_none_or(|target| Some(target) == input.target.as_ref())
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

    pub fn explicit_failure_on(op: impl Into<String>, selected_target: impl Into<String>) -> Self {
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
            && self
                .selected_target
                .as_ref()
                .is_none_or(|target| target == &input.selected_target)
            && self
                .chain_ref
                .as_ref()
                .is_none_or(|chain_ref| Some(chain_ref) == input.chain_ref.as_ref())
            && self
                .selected_option_ref
                .as_ref()
                .is_none_or(|option_ref| Some(option_ref) == input.selected_option_ref.as_ref())
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

pub fn host_plan_sidecar_path_for_fixture_path(fixture_path: impl AsRef<Path>) -> PathBuf {
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

pub fn load_bundle_from_fixture_path(
    fixture_path: impl AsRef<Path>,
) -> Result<FixtureBundle, InterpreterError> {
    let fixture_path = fixture_path.as_ref().to_path_buf();
    let fixture = crate::load_fixture_from_path(&fixture_path)?;
    bundle_from_loaded_fixture(fixture_path, fixture)
}

pub fn discover_bundles_in_directory(
    directory: impl AsRef<Path>,
) -> Result<BundleDiscoveryReport, InterpreterError> {
    let mut candidate_paths = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().is_none_or(|extension| extension != "json") {
            continue;
        }
        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".host-plan.json"))
        {
            continue;
        }
        candidate_paths.push(path);
    }
    candidate_paths.sort();

    let mut bundles = Vec::new();
    let mut failures = Vec::new();
    let mut runtime_bundles = 0usize;
    let mut static_only_bundles = 0usize;

    for fixture_path in candidate_paths.iter().cloned() {
        match crate::load_fixture_from_path(&fixture_path) {
            Ok(fixture) => {
                let runtime_requirement = runtime_requirement_for_fixture(&fixture);
                match runtime_requirement {
                    FixtureRuntimeRequirement::StaticOnly => static_only_bundles += 1,
                    FixtureRuntimeRequirement::RuntimeWithHostPlan => runtime_bundles += 1,
                }

                match bundle_from_loaded_fixture(fixture_path.clone(), fixture) {
                    Ok(bundle) => bundles.push(bundle),
                    Err(error) => failures.push(BundleDiscoveryFailure {
                        fixture_path,
                        runtime_requirement: Some(runtime_requirement),
                        error: error.to_string(),
                    }),
                }
            }
            Err(error) => failures.push(BundleDiscoveryFailure {
                fixture_path,
                runtime_requirement: None,
                error: error.to_string(),
            }),
        }
    }

    Ok(BundleDiscoveryReport {
        total_candidates: candidate_paths.len(),
        runtime_bundles,
        static_only_bundles,
        bundles,
        failures,
    })
}

pub fn select_bundles_from_discovery(
    discovery: BundleDiscoveryReport,
    mode: &SelectionMode,
) -> Result<BundleDiscoveryReport, InterpreterError> {
    let mut bundles = Vec::new();
    let mut failures = Vec::new();
    let mut runtime_bundles = 0usize;
    let mut static_only_bundles = 0usize;

    for bundle in discovery.bundles {
        if bundle_matches_selection(&bundle, mode) {
            match bundle.runtime_requirement {
                FixtureRuntimeRequirement::StaticOnly => static_only_bundles += 1,
                FixtureRuntimeRequirement::RuntimeWithHostPlan => runtime_bundles += 1,
            }
            bundles.push(bundle);
        }
    }

    for failure in discovery.failures {
        if failure_matches_selection(&failure, mode) {
            match failure.runtime_requirement {
                Some(FixtureRuntimeRequirement::StaticOnly) => static_only_bundles += 1,
                Some(FixtureRuntimeRequirement::RuntimeWithHostPlan) => runtime_bundles += 1,
                None => {}
            }
            failures.push(failure);
        }
    }

    if matches!(mode, SelectionMode::SingleFixture(_)) && bundles.is_empty() && failures.is_empty() {
        return Err(InterpreterError::InvalidProgram(
            "selected fixture was not found".to_string(),
        ));
    }

    let total_candidates = bundles.len() + failures.len();
    Ok(BundleDiscoveryReport {
        total_candidates,
        runtime_bundles,
        static_only_bundles,
        bundles,
        failures,
    })
}

pub fn select_bundles_from_request(
    discovery: BundleDiscoveryReport,
    request: &SelectionRequest,
) -> Result<BundleDiscoveryReport, InterpreterError> {
    let mut selected = discovery;

    if let Some(scope) = request.scope {
        let mode = match scope {
            SelectionScope::RuntimeOnly => SelectionMode::RuntimeOnly,
            SelectionScope::StaticOnly => SelectionMode::StaticOnly,
        };
        selected = select_bundles_from_discovery(selected, &mode)?;
    }

    if let Some(selector) = &request.single_fixture {
        selected = select_bundles_from_discovery(
            selected,
            &SelectionMode::SingleFixture(selector.clone()),
        )?;
    }

    Ok(selected)
}

pub fn run_directory(directory: impl AsRef<Path>) -> Result<BatchRunSummary, InterpreterError> {
    let discovery = discover_bundles_in_directory(directory)?;
    batch_summary_from_discovery(discovery)
}

pub fn run_directory_selected(
    directory: impl AsRef<Path>,
    mode: &SelectionMode,
) -> Result<BatchRunSummary, InterpreterError> {
    let discovery = discover_bundles_in_directory(directory)?;
    let request = SelectionRequest::from_mode(mode);
    let selected = select_bundles_from_request(discovery, &request)?;
    batch_summary_from_discovery(selected)
}

/// current L2 profile helper の thin wrapper。
/// request の逐次合成と profile 名付き summary はここで持ち、
/// per-bundle expectation compare 自体は下位 helper の結果を再利用する。
pub fn run_directory_profiled(
    directory: impl AsRef<Path>,
    profile: &SelectionProfile,
) -> Result<ProfileRunSummary, InterpreterError> {
    let discovery = discover_bundles_in_directory(directory)?;
    let selected = select_bundles_from_request(discovery, &profile.request)?;
    let summary = batch_summary_from_discovery(selected)?;
    Ok(ProfileRunSummary {
        profile_name: profile.profile_name.clone(),
        total_selected_bundles: summary.total_bundles,
        runtime_selected_bundles: summary.runtime_bundles,
        static_selected_bundles: summary.static_only_bundles,
        passed: summary.passed,
        failed: summary.failed,
        discovery_failures: summary.discovery_failures,
        bundle_failures: summary.bundle_failures,
        host_plan_coverage_failures: summary.host_plan_coverage_failures,
        bundle_reports: summary.bundle_reports,
    })
}

impl ProfileCatalog {
    pub fn aliases() -> &'static [&'static str] {
        NAMED_PROFILE_ALIASES
    }

    pub fn resolve(alias: &str) -> Result<SelectionProfile, InterpreterError> {
        named_profile_specs()
            .iter()
            .find(|spec| spec.alias == alias)
            .map(|spec| SelectionProfile::new(spec.alias, spec.build_request()))
            .ok_or_else(|| InterpreterError::InvalidProgram(format!(
                "unknown named profile alias: {alias}"
            )))
    }
}

fn named_profile_specs() -> &'static [NamedProfileSpec] {
    NAMED_PROFILE_SPECS
}

define_named_profile_catalog!(
    ("smoke-runtime", smoke_runtime_request),
    ("smoke-static", smoke_static_request),
    ("runtime-e3", runtime_e3_request),
    ("static-e4", static_e4_request),
);

fn smoke_runtime_request() -> SelectionRequest {
    SelectionRequest::new().with_scope(SelectionScope::RuntimeOnly)
}

fn smoke_static_request() -> SelectionRequest {
    SelectionRequest::new().with_scope(SelectionScope::StaticOnly)
}

fn runtime_e3_request() -> SelectionRequest {
    SelectionRequest::new()
        .with_scope(SelectionScope::RuntimeOnly)
        .with_single_fixture(SingleFixtureSelector::Stem(
            "e3-option-admit-chain".to_string(),
        ))
}

fn static_e4_request() -> SelectionRequest {
    SelectionRequest::new()
        .with_scope(SelectionScope::StaticOnly)
        .with_single_fixture(SingleFixtureSelector::Stem(
            "e4-malformed-lineage".to_string(),
        ))
}

/// current L2 named profile catalog の thin wrapper。
/// alias 一覧、alias -> request 解決、unknown alias failure をここで持ち、
/// selected counts や concrete fixture shape の coverage は profiled execution 側へ委譲する。
pub fn run_directory_named_profile(
    directory: impl AsRef<Path>,
    alias: &str,
) -> Result<NamedProfileRunSummary, InterpreterError> {
    let profile = ProfileCatalog::resolve(alias)?;
    let summary = run_directory_profiled(directory, &profile)?;
    Ok(NamedProfileRunSummary {
        profile_name: summary.profile_name,
        resolved_request: profile.request,
        total_selected_bundles: summary.total_selected_bundles,
        runtime_selected_bundles: summary.runtime_selected_bundles,
        static_selected_bundles: summary.static_selected_bundles,
        passed: summary.passed,
        failed: summary.failed,
        discovery_failures: summary.discovery_failures,
        bundle_failures: summary.bundle_failures,
        host_plan_coverage_failures: summary.host_plan_coverage_failures,
        bundle_reports: summary.bundle_reports,
    })
}

fn batch_summary_from_discovery(
    discovery: BundleDiscoveryReport,
) -> Result<BatchRunSummary, InterpreterError> {
    let mut passed = 0usize;
    let mut bundle_failures = Vec::new();
    let mut host_plan_coverage_failures = Vec::new();
    let mut bundle_reports = Vec::new();

    for bundle in &discovery.bundles {
        match run_bundle(bundle) {
            Ok(report) => {
                passed += 1;
                bundle_reports.push(BatchBundleReport {
                    fixture_path: bundle.fixture_path.clone(),
                    fixture_id: bundle.fixture.fixture_id.clone(),
                    runtime_requirement: bundle.runtime_requirement,
                    outcome: BatchBundleOutcome::Passed { report },
                });
            }
            Err(error) => {
                let error_text = error.to_string();
                let is_host_plan_coverage_failure =
                    error_text.contains("host plan did not cover all oracle calls");
                let failure = BundleExecutionFailure {
                    fixture_path: bundle.fixture_path.clone(),
                    fixture_id: bundle.fixture.fixture_id.clone(),
                    runtime_requirement: bundle.runtime_requirement,
                    error: error_text.clone(),
                };
                if is_host_plan_coverage_failure {
                    host_plan_coverage_failures.push(failure.clone());
                }
                bundle_failures.push(failure);
                bundle_reports.push(BatchBundleReport {
                    fixture_path: bundle.fixture_path.clone(),
                    fixture_id: bundle.fixture.fixture_id.clone(),
                    runtime_requirement: bundle.runtime_requirement,
                    outcome: BatchBundleOutcome::Failed {
                        error: error_text,
                        host_plan_coverage_failure: is_host_plan_coverage_failure,
                    },
                });
            }
        }
    }

    let failed = discovery.failures.len() + bundle_failures.len();

    Ok(BatchRunSummary {
        total_bundles: discovery.total_candidates,
        runtime_bundles: discovery.runtime_bundles,
        static_only_bundles: discovery.static_only_bundles,
        passed,
        failed,
        discovery_failures: discovery.failures,
        bundle_failures,
        host_plan_coverage_failures,
        bundle_reports,
    })
}

fn bundle_matches_selection(bundle: &FixtureBundle, mode: &SelectionMode) -> bool {
    match mode {
        SelectionMode::RuntimeOnly => {
            bundle.runtime_requirement == FixtureRuntimeRequirement::RuntimeWithHostPlan
        }
        SelectionMode::StaticOnly => {
            bundle.runtime_requirement == FixtureRuntimeRequirement::StaticOnly
        }
        SelectionMode::SingleFixture(selector) => {
            selector_matches_fixture_path(&bundle.fixture_path, selector)
        }
    }
}

fn failure_matches_selection(
    failure: &BundleDiscoveryFailure,
    mode: &SelectionMode,
) -> bool {
    match mode {
        SelectionMode::RuntimeOnly => {
            failure.runtime_requirement.is_none()
                || failure.runtime_requirement
                    == Some(FixtureRuntimeRequirement::RuntimeWithHostPlan)
        }
        SelectionMode::StaticOnly => {
            failure.runtime_requirement.is_none()
                || failure.runtime_requirement == Some(FixtureRuntimeRequirement::StaticOnly)
        }
        SelectionMode::SingleFixture(selector) => {
            selector_matches_fixture_path(&failure.fixture_path, selector)
        }
    }
}

fn selector_matches_fixture_path(path: &Path, selector: &SingleFixtureSelector) -> bool {
    match selector {
        SingleFixtureSelector::Stem(stem) => path
            .file_stem()
            .is_some_and(|candidate| candidate == stem.as_str()),
        SingleFixtureSelector::Path(selector_path) => {
            path == selector_path || path.ends_with(selector_path)
        }
    }
}

fn bundle_from_loaded_fixture(
    fixture_path: PathBuf,
    fixture: CurrentL2Fixture,
) -> Result<FixtureBundle, InterpreterError> {
    let runtime_requirement = if fixture.expected_runtime.enters_evaluation {
        FixtureRuntimeRequirement::RuntimeWithHostPlan
    } else {
        FixtureRuntimeRequirement::StaticOnly
    };

    let sidecar_path = host_plan_sidecar_path_for_fixture_path(&fixture_path);
    let (host_plan_path, host_plan) = if sidecar_path.exists() {
        let plan = load_host_plan_from_path(&sidecar_path)?;
        (Some(sidecar_path), Some(plan))
    } else if runtime_requirement == FixtureRuntimeRequirement::RuntimeWithHostPlan {
        return Err(InterpreterError::InvalidProgram(format!(
            "runtime fixture requires a .host-plan.json sidecar: {}",
            fixture_path.display()
        )));
    } else {
        (None, None)
    };

    Ok(FixtureBundle {
        fixture_path,
        host_plan_path,
        fixture,
        host_plan,
        runtime_requirement,
    })
}

fn runtime_requirement_for_fixture(fixture: &CurrentL2Fixture) -> FixtureRuntimeRequirement {
    if fixture.expected_runtime.enters_evaluation {
        FixtureRuntimeRequirement::RuntimeWithHostPlan
    } else {
        FixtureRuntimeRequirement::StaticOnly
    }
}

pub fn run_bundle(bundle: &FixtureBundle) -> Result<BundleRunReport, InterpreterError> {
    let harness = match &bundle.host_plan {
        Some(plan) => FixtureHostStub::new(plan.clone())?,
        None => FixtureHostStub::default(),
    };
    let report = harness.run_fixture(&bundle.fixture)?;
    let static_gate = crate::static_gate_detailed(&bundle.fixture);

    let expected_terminal_outcome =
        expected_terminal_outcome(bundle.fixture.expected_runtime.final_outcome);
    if report.static_verdict != bundle.fixture.expected_static.verdict {
        return Err(InterpreterError::InvalidProgram(format!(
            "bundle static verdict mismatch for {}: expected {:?}, got {:?}",
            bundle.fixture.fixture_id,
            bundle.fixture.expected_static.verdict,
            report.static_verdict
        )));
    }
    if let Some(checked_reasons) = &bundle.fixture.expected_static.checked_reasons {
        if static_gate.reasons != *checked_reasons {
            return Err(InterpreterError::InvalidProgram(format!(
                "bundle static checked_reasons mismatch for {}: expected {:?}, got {:?}",
                bundle.fixture.fixture_id, checked_reasons, static_gate.reasons
            )));
        }
    }
    if report.entered_evaluation != bundle.fixture.expected_runtime.enters_evaluation {
        return Err(InterpreterError::InvalidProgram(format!(
            "bundle enters_evaluation mismatch for {}: expected {}, got {}",
            bundle.fixture.fixture_id,
            bundle.fixture.expected_runtime.enters_evaluation,
            report.entered_evaluation
        )));
    }
    if report.terminal_outcome != expected_terminal_outcome {
        return Err(InterpreterError::InvalidProgram(format!(
            "bundle terminal outcome mismatch for {}: expected {:?}, got {:?}",
            bundle.fixture.fixture_id, expected_terminal_outcome, report.terminal_outcome
        )));
    }
    if report.trace_audit_sink.events != bundle.fixture.expected_trace_audit.event_kinds {
        return Err(InterpreterError::InvalidProgram(format!(
            "bundle event_kinds mismatch for {}: expected {:?}, got {:?}",
            bundle.fixture.fixture_id,
            bundle.fixture.expected_trace_audit.event_kinds,
            report.trace_audit_sink.events
        )));
    }

    let expected_non_admissible_metadata =
        harness.expected_non_admissible_metadata(&bundle.fixture);
    if report.trace_audit_sink.non_admissible_metadata != expected_non_admissible_metadata {
        return Err(InterpreterError::InvalidProgram(format!(
            "bundle non_admissible_metadata mismatch for {}: expected {:?}, got {:?}",
            bundle.fixture.fixture_id,
            expected_non_admissible_metadata,
            report.trace_audit_sink.non_admissible_metadata
        )));
    }

    let expected_narrative_explanations = harness.expected_narrative_explanations(&bundle.fixture);
    if report.trace_audit_sink.narrative_explanations != expected_narrative_explanations {
        return Err(InterpreterError::InvalidProgram(format!(
            "bundle narrative_explanations mismatch for {}: expected {:?}, got {:?}",
            bundle.fixture.fixture_id,
            expected_narrative_explanations,
            report.trace_audit_sink.narrative_explanations
        )));
    }

    Ok(BundleRunReport { report })
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

    pub fn run_fixture(&self, fixture: &CurrentL2Fixture) -> Result<RunReport, InterpreterError> {
        let mut predicate_oracle = PlannedPredicateOracle::new(self.plan.clone());
        let mut effect_oracle = PlannedEffectOracle::new(self.plan.clone());
        let report = run_to_completion(fixture, &mut predicate_oracle, &mut effect_oracle)?;
        if !predicate_oracle.violations.is_empty() || !effect_oracle.violations.is_empty() {
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

    pub fn expected_narrative_explanations(&self, fixture: &CurrentL2Fixture) -> Vec<String> {
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
        if let Some(rule) = self
            .plan
            .effect_rules
            .iter()
            .find(|rule| rule.matches(&input))
        {
            return match &rule.verdict {
                EffectPlanVerdict::Success { commit } => EffectVerdict::Success {
                    commit: commit.clone(),
                },
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

fn non_admissible_from_expected(expected: &ExpectedNonAdmissibleMetadata) -> NonAdmissibleMetadata {
    NonAdmissibleMetadata {
        option_ref: expected.option_ref.clone(),
        subreason: expected.subreason,
    }
}

fn expected_terminal_outcome(outcome: FixtureRuntimeOutcome) -> Option<TerminalOutcome> {
    match outcome {
        FixtureRuntimeOutcome::Success => Some(TerminalOutcome::Success),
        FixtureRuntimeOutcome::ExplicitFailure => Some(TerminalOutcome::ExplicitFailure),
        FixtureRuntimeOutcome::Reject => Some(TerminalOutcome::Reject),
        FixtureRuntimeOutcome::NotEvaluated => None,
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

#[cfg(test)]
mod tests {
    use super::*;

    // Internal tests only guard single-source-of-truth wiring around the
    // private preset table. Public contract coverage belongs in the integration
    // tests under `tests/current_l2_minimal_interpreter.rs`.
    #[test]
    fn named_profile_catalog_aliases_are_derived_from_internal_specs() {
        let aliases_from_specs: Vec<_> = named_profile_specs().iter().map(|spec| spec.alias).collect();

        assert_eq!(ProfileCatalog::aliases(), aliases_from_specs);
    }

    #[test]
    fn named_profile_catalog_resolve_is_derived_from_internal_specs() {
        for spec in named_profile_specs() {
            let resolved = ProfileCatalog::resolve(spec.alias).unwrap();

            assert_eq!(resolved.profile_name, spec.alias);
            assert_eq!(resolved.request, spec.build_request());
        }
    }
}
