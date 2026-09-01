//! Source-first selection of retained I2 carrier contracts.

use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use mir_runtime::sys5_local_slice::{Sys5SourceInput, build_project};

use crate::{SourceBoundEdge, SourceBoundProbe};

const ACTIVE_I2_LOGICAL_SOURCE_PATH: &str = "samples/clean-near-end/mirrorea-i2-local-toy/main.mir";

/// A fail-closed rejection from the private source-bound probe boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceBoundProbeErrorKind {
    /// The source cannot be read as UTF-8 text.
    SourceUnreadable,
    /// The accepted I2 checker or projection rejected the source.
    SourceBuildRejected,
    /// No complete retained owner-request contract was available.
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
            SourceBoundProbeErrorKind::SourceUnreadable => "I3-0 source input is unreadable",
            SourceBoundProbeErrorKind::SourceBuildRejected => {
                "I3-0 source check or projection rejected the input"
            }
            SourceBoundProbeErrorKind::RetainedContractUnavailable => {
                "I3-0 retained generated carrier contract is unavailable"
            }
        })
    }
}

impl Error for SourceBoundProbeError {}

/// Builds private edge evidence from checked ordinary source only.
///
/// The host path is used solely for the local source read. It is never placed
/// in the probe output. After source checking, the public semantic summary is
/// used only to select retained generated edge references; every carrier fact
/// then comes from `Sys5LocalProject::i3_probe_carrier_contract`.
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

    let owner_edge_refs = project
        .semantic_summary()
        .generated_communication
        .iter()
        .filter(|edge| {
            edge.kind == "owner-request"
                && edge.derived_from_checked_core
                && edge.checked_program_identity == program_ref
        })
        .map(|edge| edge.edge_ref.clone())
        .collect::<Vec<_>>();
    if owner_edge_refs.is_empty() {
        return Err(SourceBoundProbeError::new(
            SourceBoundProbeErrorKind::RetainedContractUnavailable,
        ));
    }

    let mut owner_request_edges = Vec::with_capacity(owner_edge_refs.len());
    for edge_ref in owner_edge_refs {
        let contract = project.i3_probe_carrier_contract(&edge_ref).map_err(|_| {
            SourceBoundProbeError::new(SourceBoundProbeErrorKind::RetainedContractUnavailable)
        })?;
        if contract.checked_program_ref() != program_ref
            || contract.edge_kind() != "owner-request"
            || !contract.checked_core_bound()
            || contract.transfers_authority()
            || contract.public_api_or_wire_contract()
        {
            return Err(SourceBoundProbeError::new(
                SourceBoundProbeErrorKind::RetainedContractUnavailable,
            ));
        }
        owner_request_edges.push(SourceBoundEdge::from_sys5(contract));
    }
    Ok(SourceBoundProbe::new(program_ref, owner_request_edges))
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
