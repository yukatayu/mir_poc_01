//! Source-first selection of retained I2 carrier contracts.

use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use mir_runtime::sys5_local_slice::{Sys5SourceInput, build_project};

use crate::{SourceBoundAdapterEdge, SourceBoundEdge, SourceBoundProbe};

const ACTIVE_I2_LOGICAL_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";

/// A fail-closed rejection from the private source-bound probe boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceBoundProbeErrorKind {
    /// The source cannot be read as UTF-8 text.
    SourceUnreadable,
    /// The accepted I2 checker or projection rejected the source.
    SourceBuildRejected,
    /// No complete retained checked adapter contract was available.
    RetainedContractUnavailable,
}

/// A non-secret source-bound probe error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceBoundProbeError {
    kind: SourceBoundProbeErrorKind,
}

impl SourceBoundProbeError {
    const fn new(kind: SourceBoundProbeErrorKind) -> Self {
        Self { kind }
    }

    /// The typed, non-secret rejection classification.
    pub const fn kind(&self) -> SourceBoundProbeErrorKind {
        self.kind
    }
}

impl fmt::Display for SourceBoundProbeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            SourceBoundProbeErrorKind::SourceUnreadable => "I3 source input is unreadable",
            SourceBoundProbeErrorKind::SourceBuildRejected => {
                "I3 source check or projection rejected the input"
            }
            SourceBoundProbeErrorKind::RetainedContractUnavailable => {
                "retained checked adapter carrier contract is unavailable"
            }
        })
    }
}

impl Error for SourceBoundProbeError {}

/// Builds private edge evidence from checked ordinary source only.
///
/// The host path is used solely for the local source read. It is never placed
/// in the probe output. After source checking, the public semantic summary is
/// used only to select retained generated edge references. The legacy
/// owner-request path remains bound through `i3_probe_carrier_contract`, while
/// the separate static inventory is bound through `i3_adapter_carrier_contract`.
pub fn build_source_bound_probe(
    source_path: impl AsRef<Path>,
) -> Result<SourceBoundProbe, SourceBoundProbeError> {
    let source_text = fs::read_to_string(source_path.as_ref())
        .map_err(|_| SourceBoundProbeError::new(SourceBoundProbeErrorKind::SourceUnreadable))?;
    let project = build_project(Sys5SourceInput::inline(
        logical_source_path(source_path.as_ref()),
        source_text,
    ))
    .map_err(|_| SourceBoundProbeError::new(SourceBoundProbeErrorKind::SourceBuildRejected))?;
    let program_ref = project.checked_program_identity_ref().to_string();
    if program_ref.is_empty() {
        return Err(SourceBoundProbeError::new(
            SourceBoundProbeErrorKind::RetainedContractUnavailable,
        ));
    }

    let accepted_family_kinds = project.i3_adapter_accepted_family_kind_names();
    let mut edge_refs =
        Vec::with_capacity(project.semantic_summary().generated_communication.len());
    for edge in &project.semantic_summary().generated_communication {
        if !edge.derived_from_checked_core
            || edge.checked_program_identity != program_ref
            || !accepted_family_kinds.contains(&edge.kind.as_str())
        {
            return Err(SourceBoundProbeError::new(
                SourceBoundProbeErrorKind::RetainedContractUnavailable,
            ));
        }
        edge_refs.push(edge.edge_ref.clone());
    }
    if edge_refs.is_empty() {
        return Err(SourceBoundProbeError::new(
            SourceBoundProbeErrorKind::RetainedContractUnavailable,
        ));
    }

    let mut owner_request_edges = Vec::new();
    let mut adapter_carrier_edges = Vec::with_capacity(edge_refs.len());
    for edge_ref in edge_refs {
        let contract = project
            .i3_adapter_carrier_contract(&edge_ref)
            .map_err(|_| {
                SourceBoundProbeError::new(SourceBoundProbeErrorKind::RetainedContractUnavailable)
            })?;
        if contract.checked_program_ref() != program_ref
            || !contract.checked_core_bound()
            || contract.transfers_authority()
            || contract.mints_authority_without_source()
            || contract.public_api_or_wire_contract()
        {
            return Err(SourceBoundProbeError::new(
                SourceBoundProbeErrorKind::RetainedContractUnavailable,
            ));
        }
        if contract.edge_kind() == "owner-request" {
            let legacy_contract = project.i3_probe_carrier_contract(&edge_ref).map_err(|_| {
                SourceBoundProbeError::new(SourceBoundProbeErrorKind::RetainedContractUnavailable)
            })?;
            if legacy_contract.checked_program_ref() != program_ref
                || legacy_contract.edge_kind() != "owner-request"
                || !legacy_contract.checked_core_bound()
                || legacy_contract.transfers_authority()
                || legacy_contract.public_api_or_wire_contract()
            {
                return Err(SourceBoundProbeError::new(
                    SourceBoundProbeErrorKind::RetainedContractUnavailable,
                ));
            }
            owner_request_edges.push(SourceBoundEdge::from_sys5(legacy_contract));
        }
        adapter_carrier_edges.push(SourceBoundAdapterEdge::from_sys5_adapter(contract));
    }
    if owner_request_edges.is_empty() {
        return Err(SourceBoundProbeError::new(
            SourceBoundProbeErrorKind::RetainedContractUnavailable,
        ));
    }
    Ok(SourceBoundProbe::new(
        program_ref,
        owner_request_edges,
        adapter_carrier_edges,
    ))
}

fn logical_source_path(source_path: &Path) -> &'static str {
    let active_suffix = PathBuf::from(ACTIVE_I2_LOGICAL_SOURCE_PATH);
    let components = source_path.components().collect::<Vec<_>>();
    let suffix_components = active_suffix.components().collect::<Vec<_>>();
    if components.ends_with(&suffix_components) {
        ACTIVE_I2_LOGICAL_SOURCE_PATH
    } else {
        "i3-0-private-input.mir"
    }
}
