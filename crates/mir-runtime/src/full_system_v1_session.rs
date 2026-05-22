use std::path::Path;

use mir_semantics::full_system_v1::{FullSystemV1RunReport, run_textual_mir_function_path};
use serde::{Deserialize, Serialize};

pub const FULL_SYSTEM_V1_SESSION_SURFACE_KIND: &str = "full_system_v1_session_report";
pub const FULL_SYSTEM_V1_SESSION_SCOPE: &str = "full-system-v1-same-process-runtime-v0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FullSystemV1SessionReport {
    #[serde(default = "session_surface_kind")]
    pub surface_kind: String,
    #[serde(default = "session_scope")]
    pub session_scope: String,
    pub session_id: String,
    pub source_path: String,
    pub entry_function: String,
    pub runtime: FullSystemV1RunReport,
    pub observer_safe_summary: String,
    pub final_public_api_frozen: bool,
}

pub fn run_full_system_v1_session_path(
    path: impl AsRef<Path>,
    entry_function: &str,
    input: i64,
) -> FullSystemV1SessionReport {
    let source_path = path.as_ref().display().to_string();
    let runtime = run_textual_mir_function_path(path, entry_function, input);
    FullSystemV1SessionReport {
        surface_kind: session_surface_kind(),
        session_scope: session_scope(),
        session_id: format!("full-system-v1::{}::{}", runtime.entry_function, input),
        source_path,
        entry_function: entry_function.to_string(),
        observer_safe_summary: runtime.observer_safe_summary.clone(),
        runtime,
        final_public_api_frozen: false,
    }
}

fn session_surface_kind() -> String {
    FULL_SYSTEM_V1_SESSION_SURFACE_KIND.to_string()
}

fn session_scope() -> String {
    FULL_SYSTEM_V1_SESSION_SCOPE.to_string()
}
