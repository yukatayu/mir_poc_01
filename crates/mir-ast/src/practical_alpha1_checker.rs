use std::{
    fmt,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::practical_alpha1::{
    PracticalAlpha1AlphaLocalCheckerCase, PracticalAlpha1AlphaLocalCheckerFamily,
    PracticalAlpha1Package, load_practical_alpha1_package_path,
};

pub const PRACTICAL_ALPHA1_CHECKER_SCOPE: &str = "practical-alpha1-checker-floor";
pub const PRACTICAL_ALPHA1_CHECKER_SURFACE_KIND: &str = "practical_alpha1_nonfinal_checker_report";
pub const PRACTICAL_ALPHA1_CHECKER_SCOPE_KIND: &str = "alpha_local";

const PRACTICAL_ALPHA1_CHECKER_RETAINED_LATER_REFS: &[&str] = &[
    "runtime_plan_execution",
    "run_local_runtime_execution",
    "docker_transport_execution",
    "final_public_checker_api",
];

const PRACTICAL_ALPHA1_CHECKER_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-1 checker report as a runtime plan",
    "do not treat the practical alpha-1 check command as a public CLI/API freeze",
    "do not treat package-admission preview checks as hot-plug/runtime attach verdicts",
    "do not promote samples/practical-alpha1 to an active runnable root in the checker package",
];

const PRACTICAL_ALPHA1_CHECKER_LIMITATIONS: &[&str] = &[
    "alpha-local non-final checker floor only",
    "limited CHK practical sample families only",
    "no runtime plan emission, local run, Docker run, save/load, or devtools export",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1CheckErrorKind {
    FrontDoor,
    MissingCheckerSection,
    MalformedCheckerInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1CheckError {
    pub kind: PracticalAlpha1CheckErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1CheckError {
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

impl std::error::Error for PracticalAlpha1CheckError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1CheckReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "checker_scope")]
    pub checker_scope: String,
    pub sample_id: String,
    pub package_id: String,
    pub family: String,
    pub verdict: String,
    #[serde(default)]
    pub diagnostics: Vec<PracticalAlpha1CheckerDiagnostic>,
    #[serde(default)]
    pub accepted_obligations: Vec<PracticalAlpha1AcceptedObligation>,
    #[serde(default, alias = "rejected_obligations")]
    pub rejected_rows: Vec<PracticalAlpha1RejectedRow>,
    #[serde(default)]
    pub canonical_fallback_chains: Vec<PracticalAlpha1CanonicalFallbackChain>,
    #[serde(default)]
    pub contract_comparison_report: Vec<PracticalAlpha1ContractComparisonRow>,
    #[serde(default)]
    pub cut_validity_report: Vec<PracticalAlpha1CutValidityRow>,
    #[serde(default)]
    pub package_admission_report: Vec<PracticalAlpha1PackageAdmissionRow>,
    #[serde(default = "retained_later_refs_default")]
    pub retained_later_refs: Vec<String>,
    #[serde(default = "stop_lines_default")]
    pub stop_lines: Vec<String>,
    #[serde(default = "limitations_default")]
    pub limitations: Vec<String>,
    #[serde(default)]
    pub public_cli_frozen: bool,
    #[serde(default)]
    pub runtime_plan_emitted: bool,
    #[serde(default)]
    pub run_local_claimed: bool,
    #[serde(default)]
    pub run_docker_claimed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1CheckerDiagnostic {
    pub severity: String,
    pub family: String,
    pub kind: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1AcceptedObligation {
    #[serde(default = "accepted_judgment_class")]
    pub judgment_class: String,
    #[serde(default)]
    pub family: String,
    #[serde(flatten)]
    pub detail: PracticalAlpha1AcceptedObligationDetail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PracticalAlpha1AcceptedObligationDetail {
    FallbackChainCanonicalized {
        role: String,
        canonical_chain: Vec<String>,
        capability: String,
        monotone_degradation: bool,
    },
    InheritedChainSplicedWithLineage {
        source_chain: Vec<String>,
        appended_fallback: String,
        canonical_chain: Vec<String>,
        lineage_edges: Vec<String>,
        implicit_propagation: bool,
    },
    SnapshotSelectedAnchor {
        source_chain: Vec<String>,
        selected_option: String,
        appended_fallback: String,
        resulting_chain: Vec<String>,
        excluded_options: Vec<String>,
        full_chain_inherited: bool,
        snapshot_freezes_selected_option: bool,
    },
    TransparentObserveOnlyLayer {
        layer_kind: String,
        effect_delta: Vec<String>,
        failure_delta: Vec<String>,
        precondition_strengthened: bool,
        postcondition_weakened: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1RejectedRow {
    #[serde(default = "rejected_judgment_class")]
    pub judgment_class: String,
    #[serde(default)]
    pub family: String,
    #[serde(flatten)]
    pub detail: PracticalAlpha1RejectedRowDetail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PracticalAlpha1RejectedRowDetail {
    RawDanglingReference {
        source_ref: String,
        stored_into: String,
    },
    PreconditionStrengthening {
        base_precondition: String,
        layer_precondition: String,
    },
    MutableCovariance {
        base_capability: String,
        widened_capability: String,
    },
    OrphanReceive {
        receive_event: String,
        missing_predecessor: String,
    },
    UnsignedNativePackage {
        entry_ref: String,
        signature_present: bool,
        required_signature: bool,
    },
    OverCapabilityPackage {
        provided_capability: String,
        allowed_capability_prefixes: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1CanonicalFallbackChain {
    pub source_kind: String,
    pub canonical_chain: Vec<String>,
    #[serde(default)]
    pub capability: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1ContractComparisonRow {
    pub kind: String,
    pub verdict: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1CutValidityRow {
    pub kind: String,
    pub verdict: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PracticalAlpha1PackageAdmissionRow {
    pub kind: String,
    pub verdict: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1CheckerInputIr {
    sample_id: String,
    package_id: String,
    family: PracticalAlpha1CheckerFamily,
    case: PracticalAlpha1CheckerCaseIr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PracticalAlpha1CheckerFamily {
    LifetimeFallback,
    ContractVariance,
    CutPredicate,
    PackageAdmission,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PracticalAlpha1CheckerCaseIr {
    RawDanglingReference {
        source_ref: String,
        stored_into: String,
    },
    FallbackChainValid {
        role: String,
        capability: String,
        options: Vec<String>,
        monotone_degradation: bool,
    },
    InheritedChainValid {
        source_chain: Vec<String>,
        appended_fallback: String,
        lineage_edges: Vec<String>,
        implicit_propagation: bool,
    },
    SnapshotSelectedDistinction {
        source_chain: Vec<String>,
        selected_option: String,
        appended_fallback: String,
        excluded_options: Vec<String>,
    },
    TransparentObserveOnlyLayer {
        layer_kind: String,
        effect_delta: Vec<String>,
        failure_delta: Vec<String>,
        precondition_strengthened: bool,
        postcondition_weakened: bool,
    },
    PreconditionStrengthening {
        base_precondition: String,
        layer_precondition: String,
    },
    MutableCovariance {
        base_capability: String,
        widened_capability: String,
    },
    OrphanReceive {
        receive_event: String,
        missing_predecessor: String,
    },
    UnsignedNativePackage {
        entry_ref: String,
        signature_present: bool,
        required_signature: bool,
    },
    OverCapabilityPackage {
        provided_capability: String,
        allowed_capability_prefixes: Vec<String>,
    },
}

pub fn check_practical_alpha1_package_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1CheckReport, PracticalAlpha1CheckError> {
    let path = path.as_ref().to_path_buf();
    let package =
        load_practical_alpha1_package_path(&path).map_err(|error| PracticalAlpha1CheckError {
            kind: PracticalAlpha1CheckErrorKind::FrontDoor,
            path: error.path,
            detail: error.detail,
        })?;
    check_practical_alpha1_package_at_path(&package, &path)
}

pub fn check_practical_alpha1_package(
    package: &PracticalAlpha1Package,
) -> Result<PracticalAlpha1CheckReport, PracticalAlpha1CheckError> {
    check_practical_alpha1_package_at_path(package, Path::new("<inline>"))
}

fn check_practical_alpha1_package_at_path(
    package: &PracticalAlpha1Package,
    path: &Path,
) -> Result<PracticalAlpha1CheckReport, PracticalAlpha1CheckError> {
    let ir = PracticalAlpha1CheckerInputIr::try_from_package(package, path)?;
    Ok(ir.evaluate())
}

impl PracticalAlpha1CheckerInputIr {
    fn try_from_package(
        package: &PracticalAlpha1Package,
        path: &Path,
    ) -> Result<Self, PracticalAlpha1CheckError> {
        let checker = package.alpha_local_checker_input.as_ref().ok_or_else(|| {
            PracticalAlpha1CheckError {
                kind: PracticalAlpha1CheckErrorKind::MissingCheckerSection,
                path: path.to_path_buf(),
                detail: "package does not declare `alpha_local_checker_input`".to_string(),
            }
        })?;

        let family = match checker.family {
            PracticalAlpha1AlphaLocalCheckerFamily::LifetimeFallback => {
                PracticalAlpha1CheckerFamily::LifetimeFallback
            }
            PracticalAlpha1AlphaLocalCheckerFamily::ContractVariance => {
                PracticalAlpha1CheckerFamily::ContractVariance
            }
            PracticalAlpha1AlphaLocalCheckerFamily::CutPredicate => {
                PracticalAlpha1CheckerFamily::CutPredicate
            }
            PracticalAlpha1AlphaLocalCheckerFamily::PackageAdmission => {
                PracticalAlpha1CheckerFamily::PackageAdmission
            }
        };

        let (sample_id, case) = match (&family, &checker.case) {
            (
                PracticalAlpha1CheckerFamily::LifetimeFallback,
                PracticalAlpha1AlphaLocalCheckerCase::RawDanglingReference {
                    sample_id,
                    source_ref,
                    stored_into,
                },
            ) => (
                sample_id.clone(),
                PracticalAlpha1CheckerCaseIr::RawDanglingReference {
                    source_ref: source_ref.clone(),
                    stored_into: stored_into.clone(),
                },
            ),
            (
                PracticalAlpha1CheckerFamily::LifetimeFallback,
                PracticalAlpha1AlphaLocalCheckerCase::FallbackChainValid {
                    sample_id,
                    role,
                    capability,
                    options,
                    monotone_degradation,
                },
            ) => (
                sample_id.clone(),
                PracticalAlpha1CheckerCaseIr::FallbackChainValid {
                    role: role.clone(),
                    capability: capability.clone(),
                    options: options.clone(),
                    monotone_degradation: *monotone_degradation,
                },
            ),
            (
                PracticalAlpha1CheckerFamily::LifetimeFallback,
                PracticalAlpha1AlphaLocalCheckerCase::InheritedChainValid {
                    sample_id,
                    source_chain,
                    appended_fallback,
                    lineage_edges,
                    implicit_propagation,
                },
            ) => (
                sample_id.clone(),
                PracticalAlpha1CheckerCaseIr::InheritedChainValid {
                    source_chain: source_chain.clone(),
                    appended_fallback: appended_fallback.clone(),
                    lineage_edges: lineage_edges.clone(),
                    implicit_propagation: *implicit_propagation,
                },
            ),
            (
                PracticalAlpha1CheckerFamily::LifetimeFallback,
                PracticalAlpha1AlphaLocalCheckerCase::SnapshotSelectedDistinction {
                    sample_id,
                    source_chain,
                    selected_option,
                    appended_fallback,
                    excluded_options,
                },
            ) => (
                sample_id.clone(),
                PracticalAlpha1CheckerCaseIr::SnapshotSelectedDistinction {
                    source_chain: source_chain.clone(),
                    selected_option: selected_option.clone(),
                    appended_fallback: appended_fallback.clone(),
                    excluded_options: excluded_options.clone(),
                },
            ),
            (
                PracticalAlpha1CheckerFamily::ContractVariance,
                PracticalAlpha1AlphaLocalCheckerCase::TransparentObserveOnlyLayer {
                    sample_id,
                    layer_kind,
                    effect_delta,
                    failure_delta,
                    precondition_strengthened,
                    postcondition_weakened,
                },
            ) => (
                sample_id.clone(),
                PracticalAlpha1CheckerCaseIr::TransparentObserveOnlyLayer {
                    layer_kind: layer_kind.clone(),
                    effect_delta: effect_delta.clone(),
                    failure_delta: failure_delta.clone(),
                    precondition_strengthened: *precondition_strengthened,
                    postcondition_weakened: *postcondition_weakened,
                },
            ),
            (
                PracticalAlpha1CheckerFamily::ContractVariance,
                PracticalAlpha1AlphaLocalCheckerCase::PreconditionStrengthening {
                    sample_id,
                    base_precondition,
                    layer_precondition,
                },
            ) => (
                sample_id.clone(),
                PracticalAlpha1CheckerCaseIr::PreconditionStrengthening {
                    base_precondition: base_precondition.clone(),
                    layer_precondition: layer_precondition.clone(),
                },
            ),
            (
                PracticalAlpha1CheckerFamily::ContractVariance,
                PracticalAlpha1AlphaLocalCheckerCase::MutableCovariance {
                    sample_id,
                    base_capability,
                    widened_capability,
                },
            ) => (
                sample_id.clone(),
                PracticalAlpha1CheckerCaseIr::MutableCovariance {
                    base_capability: base_capability.clone(),
                    widened_capability: widened_capability.clone(),
                },
            ),
            (
                PracticalAlpha1CheckerFamily::CutPredicate,
                PracticalAlpha1AlphaLocalCheckerCase::OrphanReceive {
                    sample_id,
                    receive_event,
                    missing_predecessor,
                },
            ) => (
                sample_id.clone(),
                PracticalAlpha1CheckerCaseIr::OrphanReceive {
                    receive_event: receive_event.clone(),
                    missing_predecessor: missing_predecessor.clone(),
                },
            ),
            (
                PracticalAlpha1CheckerFamily::PackageAdmission,
                PracticalAlpha1AlphaLocalCheckerCase::UnsignedNativePackage {
                    sample_id,
                    required_signature,
                },
            ) => {
                let native = package
                    .native
                    .as_ref()
                    .ok_or_else(|| PracticalAlpha1CheckError {
                        kind: PracticalAlpha1CheckErrorKind::MalformedCheckerInput,
                        path: path.to_path_buf(),
                        detail: "unsigned_native_package requires `native` manifest input"
                            .to_string(),
                    })?;
                (
                    sample_id.clone(),
                    PracticalAlpha1CheckerCaseIr::UnsignedNativePackage {
                        entry_ref: native.entry_ref.clone(),
                        signature_present: native.signature_present,
                        required_signature: *required_signature,
                    },
                )
            }
            (
                PracticalAlpha1CheckerFamily::PackageAdmission,
                PracticalAlpha1AlphaLocalCheckerCase::OverCapabilityPackage {
                    sample_id,
                    allowed_capability_prefixes,
                },
            ) => {
                let manifest =
                    package
                        .manifest
                        .as_ref()
                        .ok_or_else(|| PracticalAlpha1CheckError {
                            kind: PracticalAlpha1CheckErrorKind::MalformedCheckerInput,
                            path: path.to_path_buf(),
                            detail: "over_capability_package requires `manifest` input".to_string(),
                        })?;
                let provided_capability = manifest
                    .provided_capabilities
                    .iter()
                    .find(|provided| {
                        !allowed_capability_prefixes.iter().any(|allowed| {
                            provided.as_str() == allowed || provided.starts_with(allowed)
                        })
                    })
                    .cloned()
                    .ok_or_else(|| PracticalAlpha1CheckError {
                        kind: PracticalAlpha1CheckErrorKind::MalformedCheckerInput,
                        path: path.to_path_buf(),
                        detail: "over_capability_package did not expose a non-admitted capability"
                            .to_string(),
                    })?;
                (
                    sample_id.clone(),
                    PracticalAlpha1CheckerCaseIr::OverCapabilityPackage {
                        provided_capability,
                        allowed_capability_prefixes: allowed_capability_prefixes.clone(),
                    },
                )
            }
            _ => {
                return Err(PracticalAlpha1CheckError {
                    kind: PracticalAlpha1CheckErrorKind::MalformedCheckerInput,
                    path: path.to_path_buf(),
                    detail:
                        "checker family/case combination is not admitted in the current alpha floor"
                            .to_string(),
                });
            }
        };

        Ok(Self {
            sample_id,
            package_id: package.package_id.clone(),
            family,
            case,
        })
    }

    fn evaluate(&self) -> PracticalAlpha1CheckReport {
        match &self.case {
            PracticalAlpha1CheckerCaseIr::RawDanglingReference {
                source_ref,
                stored_into,
            } => {
                let mut report = self.base_report("rejected");
                report.diagnostics.push(PracticalAlpha1CheckerDiagnostic {
                    severity: "error".to_string(),
                    family: self.family_name().to_string(),
                    kind: "raw_dangling_reference".to_string(),
                    message: "raw dangling reference stored into longer-lived context".to_string(),
                });
                report.rejected_rows.push(PracticalAlpha1RejectedRow {
                    judgment_class: rejected_judgment_class(),
                    family: self.family_name().to_string(),
                    detail: PracticalAlpha1RejectedRowDetail::RawDanglingReference {
                        source_ref: source_ref.clone(),
                        stored_into: stored_into.clone(),
                    },
                });
                report
            }
            PracticalAlpha1CheckerCaseIr::FallbackChainValid {
                role,
                capability,
                options,
                monotone_degradation,
            } => {
                let mut report = self.base_report("accepted");
                report
                    .accepted_obligations
                    .push(PracticalAlpha1AcceptedObligation {
                        judgment_class: accepted_judgment_class(),
                        family: self.family_name().to_string(),
                        detail:
                            PracticalAlpha1AcceptedObligationDetail::FallbackChainCanonicalized {
                                role: role.clone(),
                                canonical_chain: options.clone(),
                                capability: capability.clone(),
                                monotone_degradation: *monotone_degradation,
                            },
                    });
                report
                    .canonical_fallback_chains
                    .push(PracticalAlpha1CanonicalFallbackChain {
                        source_kind: "fallback_chain_valid".to_string(),
                        canonical_chain: options.clone(),
                        capability: Some(capability.clone()),
                    });
                report
            }
            PracticalAlpha1CheckerCaseIr::InheritedChainValid {
                source_chain,
                appended_fallback,
                lineage_edges,
                implicit_propagation,
            } => {
                let mut canonical_chain = source_chain.clone();
                canonical_chain.push(appended_fallback.clone());
                let mut report = self.base_report("accepted");
                report
                    .accepted_obligations
                    .push(PracticalAlpha1AcceptedObligation {
                    judgment_class: accepted_judgment_class(),
                    family: self.family_name().to_string(),
                    detail:
                        PracticalAlpha1AcceptedObligationDetail::InheritedChainSplicedWithLineage {
                            source_chain: source_chain.clone(),
                            appended_fallback: appended_fallback.clone(),
                            canonical_chain: canonical_chain.clone(),
                            lineage_edges: lineage_edges.clone(),
                            implicit_propagation: *implicit_propagation,
                        },
                });
                report
                    .canonical_fallback_chains
                    .push(PracticalAlpha1CanonicalFallbackChain {
                        source_kind: "inherited_chain_valid".to_string(),
                        canonical_chain,
                        capability: None,
                    });
                report
            }
            PracticalAlpha1CheckerCaseIr::SnapshotSelectedDistinction {
                source_chain,
                selected_option,
                appended_fallback,
                excluded_options,
            } => {
                let resulting_chain = vec![selected_option.clone(), appended_fallback.clone()];
                let mut report = self.base_report("accepted");
                report
                    .accepted_obligations
                    .push(PracticalAlpha1AcceptedObligation {
                        judgment_class: accepted_judgment_class(),
                        family: self.family_name().to_string(),
                        detail: PracticalAlpha1AcceptedObligationDetail::SnapshotSelectedAnchor {
                            source_chain: source_chain.clone(),
                            selected_option: selected_option.clone(),
                            appended_fallback: appended_fallback.clone(),
                            resulting_chain: resulting_chain.clone(),
                            excluded_options: excluded_options.clone(),
                            full_chain_inherited: false,
                            snapshot_freezes_selected_option: true,
                        },
                    });
                report
                    .canonical_fallback_chains
                    .push(PracticalAlpha1CanonicalFallbackChain {
                        source_kind: "snapshot_selected_distinction".to_string(),
                        canonical_chain: resulting_chain,
                        capability: None,
                    });
                report
            }
            PracticalAlpha1CheckerCaseIr::TransparentObserveOnlyLayer {
                layer_kind,
                effect_delta,
                failure_delta,
                precondition_strengthened,
                postcondition_weakened,
            } => {
                let mut report = self.base_report("accepted");
                report
                    .accepted_obligations
                    .push(PracticalAlpha1AcceptedObligation {
                        judgment_class: accepted_judgment_class(),
                        family: self.family_name().to_string(),
                        detail:
                            PracticalAlpha1AcceptedObligationDetail::TransparentObserveOnlyLayer {
                                layer_kind: layer_kind.clone(),
                                effect_delta: effect_delta.clone(),
                                failure_delta: failure_delta.clone(),
                                precondition_strengthened: *precondition_strengthened,
                                postcondition_weakened: *postcondition_weakened,
                            },
                    });
                report
                    .contract_comparison_report
                    .push(PracticalAlpha1ContractComparisonRow {
                    kind: "transparent_observe_only_layer".to_string(),
                    verdict: "accepted".to_string(),
                    detail:
                        "layer remains observe-only and does not strengthen contract obligations"
                            .to_string(),
                });
                report
            }
            PracticalAlpha1CheckerCaseIr::PreconditionStrengthening {
                base_precondition,
                layer_precondition,
            } => {
                let mut report = self.base_report("rejected");
                report.diagnostics.push(PracticalAlpha1CheckerDiagnostic {
                    severity: "error".to_string(),
                    family: self.family_name().to_string(),
                    kind: "precondition_strengthening".to_string(),
                    message: "layer precondition strengthens the base contract".to_string(),
                });
                report.rejected_rows.push(PracticalAlpha1RejectedRow {
                    judgment_class: rejected_judgment_class(),
                    family: self.family_name().to_string(),
                    detail: PracticalAlpha1RejectedRowDetail::PreconditionStrengthening {
                        base_precondition: base_precondition.clone(),
                        layer_precondition: layer_precondition.clone(),
                    },
                });
                report
                    .contract_comparison_report
                    .push(PracticalAlpha1ContractComparisonRow {
                        kind: "precondition_strengthening".to_string(),
                        verdict: "rejected".to_string(),
                        detail: "layer precondition strengthens the base contract".to_string(),
                    });
                report
            }
            PracticalAlpha1CheckerCaseIr::MutableCovariance {
                base_capability,
                widened_capability,
            } => {
                let mut report = self.base_report("rejected");
                report.diagnostics.push(PracticalAlpha1CheckerDiagnostic {
                    severity: "error".to_string(),
                    family: self.family_name().to_string(),
                    kind: "mutable_covariance".to_string(),
                    message: "mutable/read-write capability is invariant".to_string(),
                });
                report.rejected_rows.push(PracticalAlpha1RejectedRow {
                    judgment_class: rejected_judgment_class(),
                    family: self.family_name().to_string(),
                    detail: PracticalAlpha1RejectedRowDetail::MutableCovariance {
                        base_capability: base_capability.clone(),
                        widened_capability: widened_capability.clone(),
                    },
                });
                report
                    .contract_comparison_report
                    .push(PracticalAlpha1ContractComparisonRow {
                        kind: "mutable_covariance".to_string(),
                        verdict: "rejected".to_string(),
                        detail: "mutable/read-write capability is invariant".to_string(),
                    });
                report
            }
            PracticalAlpha1CheckerCaseIr::OrphanReceive {
                receive_event,
                missing_predecessor,
            } => {
                let mut report = self.base_report("rejected");
                report.diagnostics.push(PracticalAlpha1CheckerDiagnostic {
                    severity: "error".to_string(),
                    family: self.family_name().to_string(),
                    kind: "orphan_receive".to_string(),
                    message: "distributed cut includes a receive without its send predecessor"
                        .to_string(),
                });
                report.rejected_rows.push(PracticalAlpha1RejectedRow {
                    judgment_class: rejected_judgment_class(),
                    family: self.family_name().to_string(),
                    detail: PracticalAlpha1RejectedRowDetail::OrphanReceive {
                        receive_event: receive_event.clone(),
                        missing_predecessor: missing_predecessor.clone(),
                    },
                });
                report
                    .cut_validity_report
                    .push(PracticalAlpha1CutValidityRow {
                        kind: "orphan_receive".to_string(),
                        verdict: "rejected".to_string(),
                        detail: "distributed cut includes a receive without its send predecessor"
                            .to_string(),
                    });
                report
            }
            PracticalAlpha1CheckerCaseIr::UnsignedNativePackage {
                entry_ref,
                signature_present,
                required_signature,
            } => {
                let mut report = self.base_report("rejected");
                report.diagnostics.push(PracticalAlpha1CheckerDiagnostic {
                    severity: "error".to_string(),
                    family: self.family_name().to_string(),
                    kind: "unsigned_native_package".to_string(),
                    message: "native package is unsigned in the alpha admission floor".to_string(),
                });
                report.rejected_rows.push(PracticalAlpha1RejectedRow {
                    judgment_class: rejected_judgment_class(),
                    family: self.family_name().to_string(),
                    detail: PracticalAlpha1RejectedRowDetail::UnsignedNativePackage {
                        entry_ref: entry_ref.clone(),
                        signature_present: *signature_present,
                        required_signature: *required_signature,
                    },
                });
                report
                    .package_admission_report
                    .push(PracticalAlpha1PackageAdmissionRow {
                        kind: "unsigned_native_package".to_string(),
                        verdict: "rejected".to_string(),
                        detail:
                            "unsigned native package is rejected in the checker-only preview floor"
                                .to_string(),
                    });
                report
            }
            PracticalAlpha1CheckerCaseIr::OverCapabilityPackage {
                provided_capability,
                ..
            } => {
                let mut report = self.base_report("rejected");
                report.diagnostics.push(PracticalAlpha1CheckerDiagnostic {
                    severity: "error".to_string(),
                    family: self.family_name().to_string(),
                    kind: "over_capability_package".to_string(),
                    message:
                        "package provides a capability outside the declared alpha admission surface"
                            .to_string(),
                });
                report.rejected_rows.push(PracticalAlpha1RejectedRow {
                    judgment_class: rejected_judgment_class(),
                    family: self.family_name().to_string(),
                    detail: PracticalAlpha1RejectedRowDetail::OverCapabilityPackage {
                        provided_capability: provided_capability.clone(),
                        allowed_capability_prefixes: match &self.case {
                            PracticalAlpha1CheckerCaseIr::OverCapabilityPackage {
                                allowed_capability_prefixes,
                                ..
                            } => allowed_capability_prefixes.clone(),
                            _ => Vec::new(),
                        },
                    },
                });
                report
                    .package_admission_report
                    .push(PracticalAlpha1PackageAdmissionRow {
                    kind: "over_capability_package".to_string(),
                    verdict: "rejected".to_string(),
                    detail:
                        "package provides a capability outside the checker-admitted alpha surface"
                            .to_string(),
                });
                report
            }
        }
    }

    fn base_report(&self, verdict: &str) -> PracticalAlpha1CheckReport {
        PracticalAlpha1CheckReport {
            surface_kind: surface_kind(),
            scope_kind: scope_kind(),
            checker_scope: checker_scope(),
            sample_id: self.sample_id.clone(),
            package_id: self.package_id.clone(),
            family: self.family_name().to_string(),
            verdict: verdict.to_string(),
            diagnostics: Vec::new(),
            accepted_obligations: Vec::new(),
            rejected_rows: Vec::new(),
            canonical_fallback_chains: Vec::new(),
            contract_comparison_report: Vec::new(),
            cut_validity_report: Vec::new(),
            package_admission_report: Vec::new(),
            retained_later_refs: retained_later_refs_default(),
            stop_lines: stop_lines_default(),
            limitations: limitations_default(),
            public_cli_frozen: false,
            runtime_plan_emitted: false,
            run_local_claimed: false,
            run_docker_claimed: false,
        }
    }

    fn family_name(&self) -> &'static str {
        match self.family {
            PracticalAlpha1CheckerFamily::LifetimeFallback => "lifetime_fallback",
            PracticalAlpha1CheckerFamily::ContractVariance => "contract_variance",
            PracticalAlpha1CheckerFamily::CutPredicate => "cut_predicate",
            PracticalAlpha1CheckerFamily::PackageAdmission => "package_admission",
        }
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA1_CHECKER_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA1_CHECKER_SCOPE_KIND.to_string()
}

fn checker_scope() -> String {
    PRACTICAL_ALPHA1_CHECKER_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_CHECKER_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA1_CHECKER_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA1_CHECKER_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn accepted_judgment_class() -> String {
    "accepted_obligation".to_string()
}

fn rejected_judgment_class() -> String {
    "rejected_row".to_string()
}
