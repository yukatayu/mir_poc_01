use std::{
    fmt, fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use serde_json::error::Category;

pub const PRODUCT_ALPHA1_SCHEMA_VERSION: &str = "mirrorea-product-alpha1-v0";
pub const PRODUCT_ALPHA1_PACKAGE_FILE_NAME: &str = "package.mir.json";

const PRODUCT_ALPHA1_STOP_LINES: &[&str] = &[
    "product alpha-1 package schema is alpha-stable only, not a final public API",
    "textual .mir final grammar is not accepted by this product alpha-1 front door",
    "native output is a host launch bundle, not arbitrary native package execution",
    "standalone package checking does not by itself claim product alpha-1 release readiness; use demo/release validation evidence",
];

const PRODUCT_ALPHA1_RESIDUALS: &[(&str, &str, &str)] = &[
    (
        "runtime_preflight",
        "message_recovery_runtime_evidence",
        "MessageState / TransportContract / RecoveryPolicy transitions require runtime evidence",
    ),
    (
        "runtime_preflight",
        "quiescent_save_runtime_evidence",
        "R2 requires R1/load-admissibility subset plus NoInFlight / AllPlacesSealed / NoPostCutSend",
    ),
    (
        "release_validation",
        "same_session_release_validation",
        "standalone checker output must be paired with same-session demo/release validation for product alpha-1 readiness",
    ),
    (
        "release_validation",
        "product_devtools_viewer",
        "viewer UX and observer-safe leak checks are release-validation evidence, not standalone checker evidence",
    ),
    (
        "release_validation",
        "native_launch_bundle",
        "native host launch bundle evidence is produced by build-native-bundle/demo, not by standalone package checking",
    ),
    (
        "release_validation",
        "clean_clone_release_validation",
        "clean-clone product alpha-1 release validation is carried by scripts/product_alpha1_release_check.py",
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProductAlpha1ErrorKind {
    MissingPackageFile,
    Io,
    JsonParse,
    SchemaDecode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProductAlpha1Error {
    pub kind: ProductAlpha1ErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for ProductAlpha1Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} at {}: {}",
            self.kind,
            self.path.display(),
            self.detail
        )
    }
}

impl std::error::Error for ProductAlpha1Error {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1Package {
    pub schema_version: String,
    pub package_id: String,
    pub package_version: String,
    pub package_kind: String,
    pub dependencies: Vec<String>,
    pub effects: Vec<String>,
    pub failures: Vec<String>,
    pub capabilities: Vec<String>,
    pub witness_requirements: Vec<String>,
    pub membership_requirements: Vec<String>,
    pub auth_policy: ProductAlpha1AuthPolicy,
    pub auth_stack: Vec<String>,
    pub contracts: Vec<ProductAlpha1Contract>,
    pub observation_policy: ProductAlpha1ObservationPolicy,
    pub redaction_policy: ProductAlpha1RedactionPolicy,
    pub retention_policy: ProductAlpha1RetentionPolicy,
    pub message_recovery_policy: ProductAlpha1MessageRecoveryPolicy,
    pub savepoint_policy: ProductAlpha1SavepointPolicy,
    #[serde(default)]
    pub runtime_input: ProductAlpha1RuntimeInput,
    pub native_policy: ProductAlpha1NativePolicy,
    pub compatibility: ProductAlpha1CompatibilityPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1AuthPolicy {
    pub policy_id: String,
    pub required_bindings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1Contract {
    pub contract_id: String,
    pub variance: String,
    pub effect_row: Vec<String>,
    pub failure_row: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1ObservationPolicy {
    pub view_role: String,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1RedactionPolicy {
    pub level: String,
    pub redacted_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1RetentionPolicy {
    pub scope: String,
    pub retained_artifacts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1MessageRecoveryPolicy {
    pub handled_failures: Vec<String>,
    pub recovery: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1SavepointPolicy {
    pub classes: Vec<String>,
    pub quiescent_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1RuntimeInput {
    #[serde(default)]
    pub entry_place: Option<String>,
    #[serde(default)]
    pub host_io: Option<ProductAlpha1HostIoRuntimeInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1HostIoRuntimeInput {
    pub adapter_kind: String,
    pub effect_ref: String,
    pub request_payload: ProductAlpha1HostIoPayload,
    pub expected_response: ProductAlpha1HostIoPayload,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProductAlpha1HostIoPayload {
    Int { value: i64 },
    Text { value: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1NativePolicy {
    pub execution_policy: String,
    pub provenance_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductAlpha1CompatibilityPolicy {
    pub min_cli_schema_version: String,
    pub migration_policy: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1CheckReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    pub schema_version: String,
    pub package_id: String,
    pub package_version: String,
    pub package_kind: String,
    pub verdict: String,
    #[serde(default)]
    pub diagnostics: Vec<ProductAlpha1Diagnostic>,
    #[serde(default)]
    pub accepted_obligations: Vec<ProductAlpha1AcceptedObligation>,
    #[serde(default)]
    pub residual_obligations: Vec<ProductAlpha1ResidualObligation>,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default)]
    pub product_alpha1_ready: bool,
    #[serde(default)]
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1Diagnostic {
    pub severity: String,
    pub kind: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1AcceptedObligation {
    pub line: String,
    pub kind: String,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1ResidualObligation {
    pub line: String,
    pub kind: String,
    pub reason: String,
}

pub fn load_product_alpha1_package_path(
    path: impl AsRef<Path>,
) -> Result<ProductAlpha1Package, ProductAlpha1Error> {
    let resolved_path = resolve_package_path(path.as_ref())?;
    let text = fs::read_to_string(&resolved_path).map_err(|error| ProductAlpha1Error {
        kind: ProductAlpha1ErrorKind::Io,
        path: resolved_path.clone(),
        detail: error.to_string(),
    })?;

    parse_product_alpha1_package_text_at_path(&text, &resolved_path)
}

pub fn check_product_alpha1_package_path(
    path: impl AsRef<Path>,
) -> Result<ProductAlpha1CheckReport, ProductAlpha1Error> {
    let resolved_path = resolve_package_path(path.as_ref())?;
    let text = fs::read_to_string(&resolved_path).map_err(|error| ProductAlpha1Error {
        kind: ProductAlpha1ErrorKind::Io,
        path: resolved_path.clone(),
        detail: error.to_string(),
    })?;
    let package = parse_product_alpha1_package_text_at_path(&text, &resolved_path)?;
    validate_dependency_packages(&package, &resolved_path)?;
    let mut report = check_product_alpha1_package(&package)?;
    if !package.dependencies.is_empty() {
        report.accepted_obligations.push(accepted(
            "static_checker",
            "dependency_packages",
            "declared dependency package files exist and parse",
        ));
    }
    Ok(report)
}

pub fn parse_product_alpha1_package_text(
    text: &str,
) -> Result<ProductAlpha1Package, ProductAlpha1Error> {
    parse_product_alpha1_package_text_at_path(text, Path::new("<inline>"))
}

fn parse_product_alpha1_package_text_at_path(
    text: &str,
    path: &Path,
) -> Result<ProductAlpha1Package, ProductAlpha1Error> {
    let package = serde_json::from_str::<ProductAlpha1Package>(text).map_err(|error| {
        let kind = match error.classify() {
            Category::Syntax | Category::Eof => ProductAlpha1ErrorKind::JsonParse,
            Category::Data => ProductAlpha1ErrorKind::SchemaDecode,
            Category::Io => ProductAlpha1ErrorKind::Io,
        };
        ProductAlpha1Error {
            kind,
            path: path.to_path_buf(),
            detail: error.to_string(),
        }
    })?;

    validate_package_shape(&package, path)?;
    Ok(package)
}

pub fn check_product_alpha1_package(
    package: &ProductAlpha1Package,
) -> Result<ProductAlpha1CheckReport, ProductAlpha1Error> {
    validate_package_shape(package, Path::new("<inline>"))?;

    Ok(ProductAlpha1CheckReport {
        surface_kind: surface_kind(),
        schema_version: package.schema_version.clone(),
        package_id: package.package_id.clone(),
        package_version: package.package_version.clone(),
        package_kind: package.package_kind.clone(),
        verdict: "accepted".to_string(),
        diagnostics: Vec::new(),
        accepted_obligations: accepted_obligations(package),
        residual_obligations: residual_obligations_default(),
        stop_lines: stop_lines_default(),
        product_alpha1_ready: false,
        final_public_api_frozen: false,
    })
}

fn resolve_package_path(path: &Path) -> Result<PathBuf, ProductAlpha1Error> {
    if path.is_dir() {
        let candidate = path.join(PRODUCT_ALPHA1_PACKAGE_FILE_NAME);
        if candidate.exists() {
            return Ok(candidate);
        }

        return Err(ProductAlpha1Error {
            kind: ProductAlpha1ErrorKind::MissingPackageFile,
            path: candidate,
            detail: format!(
                "package directory is missing {}",
                PRODUCT_ALPHA1_PACKAGE_FILE_NAME
            ),
        });
    }

    if path.file_name().and_then(|name| name.to_str()) != Some(PRODUCT_ALPHA1_PACKAGE_FILE_NAME) {
        return Err(ProductAlpha1Error {
            kind: ProductAlpha1ErrorKind::MissingPackageFile,
            path: path.to_path_buf(),
            detail: format!(
                "product alpha-1 front door only accepts {}",
                PRODUCT_ALPHA1_PACKAGE_FILE_NAME
            ),
        });
    }

    if path.exists() {
        return Ok(path.to_path_buf());
    }

    Err(ProductAlpha1Error {
        kind: ProductAlpha1ErrorKind::MissingPackageFile,
        path: path.to_path_buf(),
        detail: "package file does not exist".to_string(),
    })
}

fn validate_package_shape(
    package: &ProductAlpha1Package,
    path: &Path,
) -> Result<(), ProductAlpha1Error> {
    if package.schema_version != PRODUCT_ALPHA1_SCHEMA_VERSION {
        return Err(schema_error(
            path,
            format!(
                "unsupported schema_version `{}`; expected `{}`",
                package.schema_version, PRODUCT_ALPHA1_SCHEMA_VERSION
            ),
        ));
    }

    match package.package_kind.as_str() {
        "world" | "layer" | "object" | "avatar_preview" | "adapter" => {}
        other => {
            return Err(schema_error(
                path,
                format!("unsupported package_kind `{other}`"),
            ));
        }
    }

    if package.native_policy.execution_policy != "disabled" {
        return Err(schema_error(
            path,
            "NativeExecutionPolicy must remain `disabled` in product alpha-1 schema".to_string(),
        ));
    }

    if !matches!(
        package.observation_policy.view_role.as_str(),
        "observer_safe" | "admin_debug"
    ) {
        return Err(schema_error(
            path,
            "observation_policy.view_role must be `observer_safe` or `admin_debug`".to_string(),
        ));
    }

    if package.compatibility.min_cli_schema_version != PRODUCT_ALPHA1_SCHEMA_VERSION {
        return Err(schema_error(
            path,
            format!(
                "compatibility.min_cli_schema_version must equal `{}`",
                PRODUCT_ALPHA1_SCHEMA_VERSION
            ),
        ));
    }

    if package.auth_policy.policy_id.trim().is_empty() {
        return Err(schema_error(
            path,
            "auth_policy.policy_id is required".to_string(),
        ));
    }

    if package.auth_stack.is_empty() {
        return Err(schema_error(
            path,
            "auth_stack must not be empty".to_string(),
        ));
    }

    for contract in &package.contracts {
        if !matches!(contract.variance.as_str(), "invariant" | "observe_only") {
            return Err(schema_error(
                path,
                format!(
                    "unsupported contract variance `{}`; expected `invariant` or `observe_only`",
                    contract.variance
                ),
            ));
        }

        for effect in &contract.effect_row {
            if !package.effects.contains(effect) {
                return Err(schema_error(
                    path,
                    format!(
                        "contract `{}` effect `{}` is not contained in package effects",
                        contract.contract_id, effect
                    ),
                ));
            }
        }

        for failure in &contract.failure_row {
            if !package.failures.contains(failure) {
                return Err(schema_error(
                    path,
                    format!(
                        "contract `{}` failure `{}` is not contained in package failures",
                        contract.contract_id, failure
                    ),
                ));
            }
        }
    }

    if package.message_recovery_policy.handled_failures.is_empty() {
        return Err(schema_error(
            path,
            "message_recovery_policy.handled_failures must not be empty".to_string(),
        ));
    }

    for failure in &package.message_recovery_policy.handled_failures {
        if !matches!(
            failure.as_str(),
            "loss" | "timeout" | "dropped" | "retry" | "reject"
        ) {
            return Err(schema_error(
                path,
                format!("unsupported message failure class `{failure}`"),
            ));
        }
    }

    if !matches!(
        package.message_recovery_policy.recovery.as_str(),
        "retry_then_reject" | "retry" | "reject" | "fallback"
    ) {
        return Err(schema_error(
            path,
            format!(
                "unsupported recovery policy `{}`",
                package.message_recovery_policy.recovery
            ),
        ));
    }

    if package.savepoint_policy.classes.is_empty() {
        return Err(schema_error(
            path,
            "savepoint_policy.classes must not be empty".to_string(),
        ));
    }

    for class in &package.savepoint_policy.classes {
        if !matches!(class.as_str(), "R0" | "R1" | "R2") {
            return Err(schema_error(
                path,
                format!("unsupported savepoint class `{class}` for product alpha-1"),
            ));
        }
    }

    if package.package_kind == "world"
        && package
            .effects
            .iter()
            .any(|effect| effect == "typed_host_io.add_one")
    {
        let Some(host_io) = &package.runtime_input.host_io else {
            return Err(schema_error(
                path,
                "world packages declaring `typed_host_io.add_one` must declare runtime_input.host_io"
                    .to_string(),
            ));
        };
        if host_io.adapter_kind != "AddOne" {
            return Err(schema_error(
                path,
                "runtime_input.host_io.adapter_kind must be `AddOne` for effect `typed_host_io.add_one`"
                    .to_string(),
            ));
        }
        if host_io.effect_ref != "typed_host_io.add_one" {
            return Err(schema_error(
                path,
                "runtime_input.host_io.effect_ref must match `typed_host_io.add_one`".to_string(),
            ));
        }
        match (&host_io.request_payload, &host_io.expected_response) {
            (
                ProductAlpha1HostIoPayload::Int { value },
                ProductAlpha1HostIoPayload::Int {
                    value: expected_value,
                },
            ) if *expected_value == value + 1 => {}
            _ => {
                return Err(schema_error(
                    path,
                    "runtime_input.host_io expected_response must equal AddOne(request_payload)"
                        .to_string(),
                ));
            }
        }
    }

    Ok(())
}

fn validate_dependency_packages(
    package: &ProductAlpha1Package,
    resolved_path: &Path,
) -> Result<(), ProductAlpha1Error> {
    let package_root = resolved_path.parent().unwrap_or_else(|| Path::new("."));

    for dependency in &package.dependencies {
        let dependency_path = Path::new(dependency);
        if dependency_path.is_absolute()
            || dependency_path
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            return Err(schema_error(
                resolved_path,
                format!("declared dependency `{dependency}` must be a relative package path"),
            ));
        }

        let candidate = package_root.join(dependency_path);
        let dependency_file = if candidate.is_dir() {
            candidate.join(PRODUCT_ALPHA1_PACKAGE_FILE_NAME)
        } else {
            candidate
        };

        if !dependency_file.exists() {
            return Err(ProductAlpha1Error {
                kind: ProductAlpha1ErrorKind::MissingPackageFile,
                path: dependency_file,
                detail: format!("declared dependency `{dependency}` is missing"),
            });
        }

        let text = fs::read_to_string(&dependency_file).map_err(|error| ProductAlpha1Error {
            kind: ProductAlpha1ErrorKind::Io,
            path: dependency_file.clone(),
            detail: error.to_string(),
        })?;
        parse_product_alpha1_package_text_at_path(&text, &dependency_file)?;
    }

    Ok(())
}

fn accepted_obligations(package: &ProductAlpha1Package) -> Vec<ProductAlpha1AcceptedObligation> {
    let mut rows = vec![
        accepted(
            "static_checker",
            "package_schema_version",
            "schema version accepted",
        ),
        accepted("static_checker", "package_kind", "package kind accepted"),
        accepted(
            "static_checker",
            "auth_policy",
            "auth policy declaration present",
        ),
        accepted(
            "static_checker",
            "message_recovery_policy",
            "message recovery policy declaration present",
        ),
        accepted(
            "static_checker",
            "savepoint_policy",
            "savepoint policy declaration present",
        ),
        accepted(
            "static_checker",
            "native_policy",
            "native execution disabled",
        ),
        accepted(
            "static_checker",
            "observation_redaction_retention",
            "observer policy declarations present",
        ),
    ];

    if !package.contracts.is_empty() {
        rows.push(accepted(
            "static_checker",
            "contract_rows",
            "contract declarations accepted",
        ));
    }

    if package.runtime_input.host_io.is_some() {
        rows.push(accepted(
            "static_checker",
            "runtime_input_host_io",
            "typed host-I/O runtime input declaration accepted",
        ));
    }

    rows
}

fn accepted(line: &str, kind: &str, evidence: &str) -> ProductAlpha1AcceptedObligation {
    ProductAlpha1AcceptedObligation {
        line: line.to_string(),
        kind: kind.to_string(),
        evidence: evidence.to_string(),
    }
}

fn schema_error(path: &Path, detail: String) -> ProductAlpha1Error {
    ProductAlpha1Error {
        kind: ProductAlpha1ErrorKind::SchemaDecode,
        path: path.to_path_buf(),
        detail,
    }
}

fn surface_kind() -> String {
    "mirrorea_product_alpha1_check_report".to_string()
}

fn stop_lines_default() -> Vec<String> {
    PRODUCT_ALPHA1_STOP_LINES
        .iter()
        .map(|line| (*line).to_string())
        .collect()
}

fn residual_obligations_default() -> Vec<ProductAlpha1ResidualObligation> {
    PRODUCT_ALPHA1_RESIDUALS
        .iter()
        .map(|(line, kind, reason)| ProductAlpha1ResidualObligation {
            line: (*line).to_string(),
            kind: (*kind).to_string(),
            reason: (*reason).to_string(),
        })
        .collect()
}
