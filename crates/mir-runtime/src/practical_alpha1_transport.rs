use std::{
    collections::BTreeSet,
    fmt,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use mir_ast::practical_alpha1_transport_plan::{
    PracticalAlpha1TransportPlan, PracticalAlpha1TransportPlanError,
    build_practical_alpha1_transport_plan_path,
};
use mirrorea_core::{
    AuthEvidence, LogicalPlaceRuntimeShell, MessageEnvelope, MirroreaCoreError, PrincipalClaim,
    auth_evidence_lanes, message_envelope_lanes,
};
use serde::{Deserialize, Serialize};

pub const PRACTICAL_ALPHA1_TRANSPORT_SCOPE: &str = "practical-alpha1-transport-floor";
pub const PRACTICAL_ALPHA1_TRANSPORT_SURFACE_KIND: &str =
    "practical_alpha1_nonfinal_transport_report";
pub const PRACTICAL_ALPHA1_TRANSPORT_SCOPE_KIND: &str = "alpha_local";

const PRACTICAL_ALPHA1_TRANSPORT_RETAINED_LATER_REFS: &[&str] = &[
    "wan_federation",
    "session_replay_repair",
    "network_partition_recovery",
    "local_save_load_execution",
    "devtools_viewer_execution",
    "final_public_transport_api",
];

const PRACTICAL_ALPHA1_TRANSPORT_STOP_LINES: &[&str] = &[
    "do not treat the practical alpha-1 transport floor as WAN or federation completion",
    "do not treat the practical alpha-1 transport floor as local save/load, devtools, or product prototype completion",
    "do not treat the practical alpha-1 transport floor as a final public transport ABI",
    "do not promote samples/practical-alpha1 to an active runnable root in the transport package",
];

const PRACTICAL_ALPHA1_TRANSPORT_LIMITATIONS: &[&str] = &[
    "alpha-local non-final practical transport floor only",
    "limited TR-A1 practical sample families only",
    "no WAN federation, local save/load command, devtools export, or final public ABI",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PracticalAlpha1TransportErrorKind {
    TransportPlan,
    Runtime,
    UnsupportedSurface,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PracticalAlpha1TransportError {
    pub kind: PracticalAlpha1TransportErrorKind,
    pub path: PathBuf,
    pub detail: String,
}

impl fmt::Display for PracticalAlpha1TransportError {
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

impl std::error::Error for PracticalAlpha1TransportError {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PracticalAlpha1TransportPeer {
    pub process_id: String,
    pub role: String,
    pub endpoint: String,
    pub containerized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PracticalAlpha1TransportAdmissionCheck {
    pub check_name: String,
    pub passed: bool,
    pub observed: String,
    pub reason_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PracticalAlpha1ObserverRouteTraceRow {
    pub hop_index: usize,
    pub envelope_id: String,
    pub from_place: String,
    pub to_place: String,
    pub transport_medium: String,
    pub transport_seam: String,
    pub payload_kind: String,
    pub dispatch_outcome: String,
    pub authority: String,
    pub retention_scope: String,
    pub witness_ref_count: usize,
    pub auth_lane_present: bool,
    pub redaction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PracticalAlpha1AuthLaneReport {
    pub auth_present: bool,
    pub subject: String,
    pub issuer: String,
    pub bindings: Vec<String>,
    pub preserved_separately: bool,
    pub transport_lane: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PracticalAlpha1TransportReport {
    #[serde(default = "surface_kind")]
    pub surface_kind: String,
    #[serde(default = "scope_kind")]
    pub scope_kind: String,
    #[serde(default = "transport_scope")]
    pub transport_scope: String,
    pub transport_plan_scope: String,
    pub sample_id: String,
    pub package_id: String,
    pub world_id: String,
    pub transport_surface: String,
    pub transport_medium: String,
    pub transport_seam: String,
    pub bridge_kind: String,
    pub bridge_processes: Vec<PracticalAlpha1TransportPeer>,
    pub message_envelope_lanes: Vec<String>,
    pub auth_evidence_lanes: Vec<String>,
    pub request_envelope: MessageEnvelope,
    pub required_capabilities: Vec<String>,
    pub required_witness_refs: Vec<String>,
    pub admission_checks: Vec<PracticalAlpha1TransportAdmissionCheck>,
    pub observer_route_trace: Vec<PracticalAlpha1ObserverRouteTraceRow>,
    pub auth_lane: Option<PracticalAlpha1AuthLaneReport>,
    pub world_membership_epoch: u64,
    pub world_active_participants: Vec<String>,
    pub world_places: Vec<String>,
    pub terminal_outcome: String,
    pub reason_family: Option<String>,
    pub rejection_reason_refs: Vec<String>,
    pub retained_later_refs: Vec<String>,
    pub stop_lines: Vec<String>,
    pub limitations: Vec<String>,
    pub what_it_proves: Vec<String>,
    pub what_it_does_not_prove: Vec<String>,
    pub notes: Vec<String>,
    #[serde(default)]
    pub public_cli_frozen: bool,
    #[serde(default)]
    pub transport_plan_emitted: bool,
    #[serde(default)]
    pub run_local_claimed: bool,
    #[serde(default)]
    pub run_docker_claimed: bool,
    #[serde(default)]
    pub package_hotplug_claimed: bool,
    #[serde(default)]
    pub save_load_claimed: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PracticalAlpha1TransportWorldServerReport {
    pub sample_id: String,
    pub package_id: String,
    pub bind_addr: String,
    pub transport_medium: String,
    pub terminal_outcome: String,
    pub reason_family: Option<String>,
    pub rejection_reason_refs: Vec<String>,
    pub admission_checks: Vec<PracticalAlpha1TransportAdmissionCheck>,
    pub world_membership_epoch: u64,
    pub world_active_participants: Vec<String>,
    pub world_places: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PracticalAlpha1TransportWireRequest {
    sample_id: String,
    package_id: String,
    world_id: String,
    request_envelope: MessageEnvelope,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PracticalAlpha1TransportWireResponse {
    sample_id: String,
    package_id: String,
    world_id: String,
    admission_checks: Vec<PracticalAlpha1TransportAdmissionCheck>,
    observer_route_trace: Vec<PracticalAlpha1ObserverRouteTraceRow>,
    auth_lane: Option<PracticalAlpha1AuthLaneReport>,
    world_membership_epoch: u64,
    world_active_participants: Vec<String>,
    world_places: Vec<String>,
    terminal_outcome: String,
    reason_family: Option<String>,
    rejection_reason_refs: Vec<String>,
    notes: Vec<String>,
}

pub fn run_practical_alpha1_transport_path(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha1TransportReport, PracticalAlpha1TransportError> {
    let path = path.as_ref().to_path_buf();
    let plan = load_transport_plan_path(&path)?;
    if plan.transport_surface != "local_tcp" {
        return Err(PracticalAlpha1TransportError {
            kind: PracticalAlpha1TransportErrorKind::UnsupportedSurface,
            path,
            detail: format!(
                "run_practical_alpha1_transport_path only admits local_tcp rows, found `{}`",
                plan.transport_surface
            ),
        });
    }
    execute_local_tcp_plan_at_path(&plan, &path)
}

pub fn run_practical_alpha1_transport_world_server_path(
    path: impl AsRef<Path>,
    bind_addr: &str,
    output_path: Option<&Path>,
) -> Result<(), PracticalAlpha1TransportError> {
    let path = path.as_ref().to_path_buf();
    let plan = load_transport_plan_path(&path)?;
    if plan.transport_surface != "docker_compose_tcp" {
        return Err(PracticalAlpha1TransportError {
            kind: PracticalAlpha1TransportErrorKind::UnsupportedSurface,
            path,
            detail: format!(
                "world-server only admits docker_compose_tcp rows, found `{}`",
                plan.transport_surface
            ),
        });
    }
    run_world_server(&plan, bind_addr, output_path).map_err(|error| PracticalAlpha1TransportError {
        kind: PracticalAlpha1TransportErrorKind::Runtime,
        path,
        detail: error.to_string(),
    })
}

pub fn run_practical_alpha1_transport_participant_client_path(
    path: impl AsRef<Path>,
    addr: &str,
    output_path: Option<&Path>,
) -> Result<PracticalAlpha1TransportReport, PracticalAlpha1TransportError> {
    let path = path.as_ref().to_path_buf();
    let plan = load_transport_plan_path(&path)?;
    if plan.transport_surface != "docker_compose_tcp" {
        return Err(PracticalAlpha1TransportError {
            kind: PracticalAlpha1TransportErrorKind::UnsupportedSurface,
            path,
            detail: format!(
                "participant-client only admits docker_compose_tcp rows, found `{}`",
                plan.transport_surface
            ),
        });
    }
    run_participant_client(&plan, addr, output_path).map_err(|error| {
        PracticalAlpha1TransportError {
            kind: PracticalAlpha1TransportErrorKind::Runtime,
            path,
            detail: error.to_string(),
        }
    })
}

fn load_transport_plan_path(
    path: &Path,
) -> Result<PracticalAlpha1TransportPlan, PracticalAlpha1TransportError> {
    build_practical_alpha1_transport_plan_path(path).map_err(|error| wrap_plan_error(path, error))
}

fn execute_local_tcp_plan_at_path(
    plan: &PracticalAlpha1TransportPlan,
    path: &Path,
) -> Result<PracticalAlpha1TransportReport, PracticalAlpha1TransportError> {
    execute_local_tcp_plan(plan).map_err(|error| PracticalAlpha1TransportError {
        kind: PracticalAlpha1TransportErrorKind::Runtime,
        path: path.to_path_buf(),
        detail: error.to_string(),
    })
}

fn execute_local_tcp_plan(
    plan: &PracticalAlpha1TransportPlan,
) -> Result<PracticalAlpha1TransportReport, MirroreaCoreError> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|error| io_error("bind practical transport loopback listener", error))?;
    let bind_addr = listener
        .local_addr()
        .map_err(|error| io_error("read practical transport listener address", error))?;
    let world_addr = bind_addr.to_string();
    let server_plan = plan.clone();
    let server = thread::spawn(move || serve_once(listener, &server_plan, None));
    let request = build_wire_request(plan)?;
    let stream = connect_with_retry(&world_addr)?;
    let response = send_request(stream, &request)?;
    server.join().map_err(|_| {
        MirroreaCoreError::new("practical transport world server thread panicked")
    })??;
    build_report_from_wire_response(plan, request, response, &world_addr, false)
}

fn run_world_server(
    plan: &PracticalAlpha1TransportPlan,
    bind_addr: &str,
    output_path: Option<&Path>,
) -> Result<(), MirroreaCoreError> {
    let listener = TcpListener::bind(bind_addr)
        .map_err(|error| io_error("bind practical transport world server", error))?;
    serve_once(listener, plan, output_path)
}

fn run_participant_client(
    plan: &PracticalAlpha1TransportPlan,
    addr: &str,
    output_path: Option<&Path>,
) -> Result<PracticalAlpha1TransportReport, MirroreaCoreError> {
    let request = build_wire_request(plan)?;
    let stream = connect_with_retry(addr)?;
    let response = send_request(stream, &request)?;
    let report = build_report_from_wire_response(plan, request, response, addr, true)?;
    if let Some(path) = output_path {
        write_json_file(path, &report)?;
    }
    Ok(report)
}

fn serve_once(
    listener: TcpListener,
    plan: &PracticalAlpha1TransportPlan,
    output_path: Option<&Path>,
) -> Result<(), MirroreaCoreError> {
    let bind_addr = listener
        .local_addr()
        .map_err(|error| io_error("read practical transport world bind address", error))?;
    let (stream, _) = listener
        .accept()
        .map_err(|error| io_error("accept practical transport connection", error))?;
    let mut stream = stream;
    let request =
        read_request(stream.try_clone().map_err(|error| {
            io_error("clone practical transport stream for request read", error)
        })?)?;
    let response = evaluate_request(plan, &request)?;
    write_response(&mut stream, &response)?;
    if let Some(path) = output_path {
        let server_report = PracticalAlpha1TransportWorldServerReport {
            sample_id: response.sample_id.clone(),
            package_id: response.package_id.clone(),
            bind_addr: bind_addr.to_string(),
            transport_medium: plan.transport_medium.clone(),
            terminal_outcome: response.terminal_outcome.clone(),
            reason_family: response.reason_family.clone(),
            rejection_reason_refs: response.rejection_reason_refs.clone(),
            admission_checks: response.admission_checks.clone(),
            world_membership_epoch: response.world_membership_epoch,
            world_active_participants: response.world_active_participants.clone(),
            world_places: response.world_places.clone(),
            notes: response.notes.clone(),
        };
        write_json_file(path, &server_report)?;
    }
    Ok(())
}

fn read_request(
    stream: TcpStream,
) -> Result<PracticalAlpha1TransportWireRequest, MirroreaCoreError> {
    let mut reader = BufReader::new(
        stream
            .try_clone()
            .map_err(|error| io_error("clone practical transport stream for read", error))?,
    );
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .map_err(|error| io_error("read practical transport request", error))?;
    serde_json::from_str(&buffer).map_err(|error| {
        MirroreaCoreError::new(format!("parse practical transport request: {error}"))
    })
}

fn send_request(
    mut stream: TcpStream,
    request: &PracticalAlpha1TransportWireRequest,
) -> Result<PracticalAlpha1TransportWireResponse, MirroreaCoreError> {
    let encoded = serde_json::to_string(request).map_err(|error| {
        MirroreaCoreError::new(format!("encode practical transport request: {error}"))
    })?;
    stream
        .write_all(encoded.as_bytes())
        .map_err(|error| io_error("write practical transport request", error))?;
    stream
        .write_all(b"\n")
        .map_err(|error| io_error("write practical transport request newline", error))?;
    stream
        .flush()
        .map_err(|error| io_error("flush practical transport request", error))?;

    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .map_err(|error| io_error("read practical transport response", error))?;
    serde_json::from_str(&buffer).map_err(|error| {
        MirroreaCoreError::new(format!("parse practical transport response: {error}"))
    })
}

fn write_response(
    stream: &mut TcpStream,
    response: &PracticalAlpha1TransportWireResponse,
) -> Result<(), MirroreaCoreError> {
    let encoded = serde_json::to_string(response).map_err(|error| {
        MirroreaCoreError::new(format!("encode practical transport response: {error}"))
    })?;
    stream
        .write_all(encoded.as_bytes())
        .map_err(|error| io_error("write practical transport response", error))?;
    stream
        .write_all(b"\n")
        .map_err(|error| io_error("write practical transport response newline", error))?;
    stream
        .flush()
        .map_err(|error| io_error("flush practical transport response", error))
}

fn connect_with_retry(addr: &str) -> Result<TcpStream, MirroreaCoreError> {
    let mut last_error = None;
    for _ in 0..40 {
        match TcpStream::connect(addr) {
            Ok(stream) => return Ok(stream),
            Err(error) => {
                last_error = Some(error.to_string());
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
    Err(MirroreaCoreError::new(format!(
        "connect to practical transport world `{addr}` failed: {}",
        last_error.unwrap_or_else(|| "unknown error".to_string())
    )))
}

fn build_wire_request(
    plan: &PracticalAlpha1TransportPlan,
) -> Result<PracticalAlpha1TransportWireRequest, MirroreaCoreError> {
    let envelope = message_envelope_from_plan(&plan.request_envelope)?;
    Ok(PracticalAlpha1TransportWireRequest {
        sample_id: plan.sample_id.clone(),
        package_id: plan.package_id.clone(),
        world_id: plan.world_id.clone(),
        request_envelope: envelope,
    })
}

fn evaluate_request(
    plan: &PracticalAlpha1TransportPlan,
    request: &PracticalAlpha1TransportWireRequest,
) -> Result<PracticalAlpha1TransportWireResponse, MirroreaCoreError> {
    request.request_envelope.validate()?;
    if request.sample_id != plan.sample_id
        || request.package_id != plan.package_id
        || request.world_id != plan.world_id
    {
        return Err(MirroreaCoreError::new(
            "practical transport request/package identity mismatch",
        ));
    }

    let shell = bootstrap_transport_shell(plan)?;
    let snapshot = shell.snapshot();

    let mut checks = Vec::new();
    let mut rejection_reason_refs = Vec::new();

    let member = snapshot
        .membership
        .members
        .get(&request.request_envelope.emitter_principal);

    let principal_place_passed = request.request_envelope.principal_claim.participant_place
        == request.request_envelope.from_place
        && request.request_envelope.principal_claim.principal
            == request.request_envelope.emitter_principal;
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "principal_place_binding",
        principal_place_passed,
        format!(
            "{} -> {}",
            request.request_envelope.principal_claim.principal, request.request_envelope.from_place
        ),
        vec!["principal_place_binding_mismatch".to_string()],
    );

    let epoch_passed =
        request.request_envelope.membership_epoch == snapshot.membership.membership_epoch;
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "membership_epoch_current",
        epoch_passed,
        format!(
            "offered={} current={}",
            request.request_envelope.membership_epoch, snapshot.membership.membership_epoch
        ),
        vec!["membership_epoch_drift".to_string()],
    );

    let incarnation_passed = member
        .map(|member| request.request_envelope.member_incarnation == member.incarnation)
        .unwrap_or(false);
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "member_incarnation_current",
        incarnation_passed,
        match member {
            Some(member) => format!(
                "offered={} current={}",
                request.request_envelope.member_incarnation, member.incarnation
            ),
            None => "missing_member_record".to_string(),
        },
        vec!["member_incarnation_drift".to_string()],
    );

    let participant_active = member.map(|member| member.active).unwrap_or(false);
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "participant_active",
        participant_active,
        match member {
            Some(member) => format!("active={}", member.active),
            None => "missing_member_record".to_string(),
        },
        vec!["participant_inactive".to_string()],
    );

    let participant_place_passed = member
        .map(|member| member.place == request.request_envelope.from_place)
        .unwrap_or(false);
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "participant_place_current",
        participant_place_passed,
        match member {
            Some(member) => format!(
                "recorded={} offered={}",
                member.place, request.request_envelope.from_place
            ),
            None => "missing_member_record".to_string(),
        },
        vec!["participant_place_drift".to_string()],
    );

    let destination_registered = snapshot
        .place_catalog
        .places
        .contains_key(&request.request_envelope.to_place);
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "destination_registered",
        destination_registered,
        format!("destination={}", request.request_envelope.to_place),
        vec!["destination_place_unregistered".to_string()],
    );

    let claimed_capabilities: BTreeSet<_> = request
        .request_envelope
        .principal_claim
        .claimed_capabilities
        .iter()
        .cloned()
        .collect();
    let required_capabilities: BTreeSet<_> = plan.required_capabilities.iter().cloned().collect();
    let missing_capabilities: Vec<String> = required_capabilities
        .difference(&claimed_capabilities)
        .cloned()
        .collect();
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "capability_sufficient",
        missing_capabilities.is_empty(),
        format!(
            "claimed={:?} required={:?}",
            request
                .request_envelope
                .principal_claim
                .claimed_capabilities,
            plan.required_capabilities
        ),
        missing_capabilities
            .into_iter()
            .map(|capability| format!("missing_capability:{capability}"))
            .collect(),
    );

    let actual_witness_refs: BTreeSet<_> = request
        .request_envelope
        .witness_refs
        .iter()
        .cloned()
        .collect();
    let required_witness_refs: BTreeSet<_> = plan.required_witness_refs.iter().cloned().collect();
    let missing_witness_refs: Vec<String> = required_witness_refs
        .difference(&actual_witness_refs)
        .cloned()
        .collect();
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "required_witness_present",
        missing_witness_refs.is_empty(),
        format!(
            "actual={:?} required={:?}",
            request.request_envelope.witness_refs, plan.required_witness_refs
        ),
        missing_witness_refs
            .into_iter()
            .map(|witness| format!("missing_witness:{witness}"))
            .collect(),
    );

    let auth_lane = build_auth_lane_report(
        &request.request_envelope.auth_evidence,
        &plan.auth_bindings_required,
    );
    let auth_reason_refs =
        auth_lane_reason_refs(&request.request_envelope, &plan.auth_bindings_required);
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "auth_evidence_coherent",
        auth_reason_refs.is_empty(),
        match &request.request_envelope.auth_evidence {
            Some(auth) => format!("subject={} issuer={}", auth.subject, auth.issuer),
            None => "auth absent".to_string(),
        },
        auth_reason_refs,
    );

    let terminal_outcome = if checks.iter().all(|check| check.passed) {
        "accepted".to_string()
    } else {
        "rejected".to_string()
    };
    let reason_family = classify_reason_family(&rejection_reason_refs);

    Ok(PracticalAlpha1TransportWireResponse {
        sample_id: plan.sample_id.clone(),
        package_id: plan.package_id.clone(),
        world_id: plan.world_id.clone(),
        admission_checks: checks,
        observer_route_trace: build_observer_route_trace(
            &request.request_envelope,
            &plan.transport_medium,
            &terminal_outcome,
        ),
        auth_lane,
        world_membership_epoch: snapshot.membership.membership_epoch,
        world_active_participants: snapshot
            .membership
            .members
            .iter()
            .filter_map(|(principal, member)| member.active.then_some(principal.clone()))
            .collect(),
        world_places: snapshot.place_catalog.places.keys().cloned().collect(),
        terminal_outcome,
        reason_family,
        rejection_reason_refs,
        notes: vec![
            "non-final practical transport floor".to_string(),
            "same practical package input line as run-local/hotplug".to_string(),
            "no WAN/session/replay/partition claim".to_string(),
        ],
    })
}

fn bootstrap_transport_shell(
    plan: &PracticalAlpha1TransportPlan,
) -> Result<LogicalPlaceRuntimeShell, MirroreaCoreError> {
    let mut shell = LogicalPlaceRuntimeShell::default();
    for place in &plan.runtime_places {
        shell.register_place(&place.place_id, &place.kind)?;
    }
    for participant in &plan.bootstrap_participants {
        shell.add_initial_participant(&participant.principal)?;
    }
    for advance in &plan.pre_dispatch_membership_advances {
        match advance.kind {
            mir_ast::practical_alpha1::PracticalAlpha1RuntimeMembershipAdvanceKind::AddParticipant => {
                shell.add_participant(&advance.principal)?;
            }
        }
    }
    Ok(shell)
}

fn build_report_from_wire_response(
    plan: &PracticalAlpha1TransportPlan,
    request: PracticalAlpha1TransportWireRequest,
    response: PracticalAlpha1TransportWireResponse,
    _world_addr: &str,
    containerized: bool,
) -> Result<PracticalAlpha1TransportReport, MirroreaCoreError> {
    if response.sample_id != request.sample_id
        || response.package_id != request.package_id
        || response.world_id != request.world_id
    {
        return Err(MirroreaCoreError::new(
            "practical transport response identity mismatch",
        ));
    }

    let auth_lane_present = response.auth_lane.is_some();
    let reason_family = response.reason_family.clone();
    let terminal_outcome = response.terminal_outcome.clone();

    Ok(PracticalAlpha1TransportReport {
        surface_kind: surface_kind(),
        scope_kind: scope_kind(),
        transport_scope: transport_scope(),
        transport_plan_scope: plan.transport_plan_scope.clone(),
        sample_id: plan.sample_id.clone(),
        package_id: plan.package_id.clone(),
        world_id: plan.world_id.clone(),
        transport_surface: plan.transport_surface.clone(),
        transport_medium: plan.transport_medium.clone(),
        transport_seam: plan.transport_seam.clone(),
        bridge_kind: plan.bridge_kind.clone(),
        bridge_processes: vec![
            PracticalAlpha1TransportPeer {
                process_id: "world-server".to_string(),
                role: "world_server".to_string(),
                endpoint: if containerized {
                    "docker-compose-world".to_string()
                } else {
                    "loopback-tcp-world".to_string()
                },
                containerized,
            },
            PracticalAlpha1TransportPeer {
                process_id: "participant-client".to_string(),
                role: "participant_client".to_string(),
                endpoint: if containerized {
                    "container-ephemeral-client".to_string()
                } else {
                    "loopback-tcp-client".to_string()
                },
                containerized,
            },
        ],
        message_envelope_lanes: message_envelope_lanes(),
        auth_evidence_lanes: auth_evidence_lanes(),
        request_envelope: request.request_envelope,
        required_capabilities: plan.required_capabilities.clone(),
        required_witness_refs: plan.required_witness_refs.clone(),
        admission_checks: response.admission_checks,
        observer_route_trace: response.observer_route_trace,
        auth_lane: response.auth_lane,
        world_membership_epoch: response.world_membership_epoch,
        world_active_participants: response.world_active_participants,
        world_places: response.world_places,
        terminal_outcome: terminal_outcome.clone(),
        reason_family: reason_family.clone(),
        rejection_reason_refs: response.rejection_reason_refs,
        retained_later_refs: retained_later_refs_default(),
        stop_lines: stop_lines_default(),
        limitations: limitations_default(),
        what_it_proves: what_it_proves(
            plan,
            &terminal_outcome,
            reason_family.as_deref(),
            auth_lane_present,
        ),
        what_it_does_not_prove: what_it_does_not_prove(),
        notes: response.notes,
        public_cli_frozen: false,
        transport_plan_emitted: false,
        run_local_claimed: !containerized,
        run_docker_claimed: containerized,
        package_hotplug_claimed: false,
        save_load_claimed: false,
    })
}

fn message_envelope_from_plan(
    envelope: &mir_ast::practical_alpha1::PracticalAlpha1RuntimeEnvelope,
) -> Result<MessageEnvelope, MirroreaCoreError> {
    let message = MessageEnvelope {
        envelope_id: envelope.envelope_id.clone(),
        from_place: envelope.from_place.clone(),
        to_place: envelope.to_place.clone(),
        transport: envelope.transport.clone(),
        transport_medium: envelope.transport_medium.clone(),
        transport_seam: envelope.transport_seam.clone(),
        payload_kind: envelope.payload_kind.clone(),
        payload_ref: envelope.payload_ref.clone(),
        principal_claim: PrincipalClaim {
            principal: envelope.principal_claim.principal.clone(),
            participant_place: envelope.principal_claim.participant_place.clone(),
            claimed_authority: envelope.principal_claim.claimed_authority.clone(),
            claimed_capabilities: envelope.principal_claim.claimed_capabilities.clone(),
        },
        auth_evidence: envelope
            .auth_evidence
            .as_ref()
            .map(|evidence| AuthEvidence {
                kind: evidence.kind.clone(),
                subject: evidence.subject.clone(),
                issuer: evidence.issuer.clone(),
                bindings: evidence.bindings.clone(),
                notes: evidence.notes.clone(),
            }),
        emitter_principal: envelope.emitter_principal.clone(),
        membership_epoch: envelope.membership_epoch,
        member_incarnation: envelope.member_incarnation,
        freshness_checks: envelope.freshness_checks.clone(),
        capability_requirements: envelope.capability_requirements.clone(),
        authorization_checks: envelope.authorization_checks.clone(),
        witness_refs: envelope.witness_refs.clone(),
        dispatch_outcome: envelope.dispatch_outcome.clone(),
        notes: envelope.notes.clone(),
    };
    message.validate()?;
    Ok(message)
}

fn build_observer_route_trace(
    request_envelope: &MessageEnvelope,
    transport_medium: &str,
    terminal_outcome: &str,
) -> Vec<PracticalAlpha1ObserverRouteTraceRow> {
    vec![
        PracticalAlpha1ObserverRouteTraceRow {
            hop_index: 1,
            envelope_id: request_envelope.envelope_id.clone(),
            from_place: request_envelope.from_place.clone(),
            to_place: request_envelope.to_place.clone(),
            transport_medium: transport_medium.to_string(),
            transport_seam: request_envelope.transport_seam.clone(),
            payload_kind: request_envelope.payload_kind.clone(),
            dispatch_outcome: "delivery_requested".to_string(),
            authority: "ObserveRouteTrace(NetworkTransportLane)".to_string(),
            retention_scope: "helper_local_ephemeral".to_string(),
            witness_ref_count: request_envelope.witness_refs.len(),
            auth_lane_present: request_envelope.auth_evidence.is_some(),
            redaction: "observer_safe_route_trace".to_string(),
        },
        PracticalAlpha1ObserverRouteTraceRow {
            hop_index: 2,
            envelope_id: format!("delivery_receipt:{}", request_envelope.envelope_id),
            from_place: request_envelope.to_place.clone(),
            to_place: request_envelope.from_place.clone(),
            transport_medium: transport_medium.to_string(),
            transport_seam: request_envelope.transport_seam.clone(),
            payload_kind: "dispatch_receipt".to_string(),
            dispatch_outcome: terminal_outcome.to_string(),
            authority: "ObserveRouteTrace(NetworkTransportLane)".to_string(),
            retention_scope: "helper_local_ephemeral".to_string(),
            witness_ref_count: request_envelope.witness_refs.len(),
            auth_lane_present: request_envelope.auth_evidence.is_some(),
            redaction: "observer_safe_route_trace".to_string(),
        },
    ]
}

fn build_auth_lane_report(
    auth_evidence: &Option<AuthEvidence>,
    required_bindings: &[String],
) -> Option<PracticalAlpha1AuthLaneReport> {
    auth_evidence
        .as_ref()
        .map(|auth| PracticalAlpha1AuthLaneReport {
            auth_present: true,
            subject: auth.subject.clone(),
            issuer: auth.issuer.clone(),
            bindings: auth.bindings.clone(),
            preserved_separately: true,
            transport_lane: "network_transport_lane".to_string(),
            notes: if required_bindings.is_empty() {
                vec!["auth lane present but not required for this row".to_string()]
            } else {
                vec!["auth lane preserved separately from transport metadata".to_string()]
            },
        })
}

fn auth_lane_reason_refs(envelope: &MessageEnvelope, required_bindings: &[String]) -> Vec<String> {
    if required_bindings.is_empty() {
        return Vec::new();
    }
    let Some(auth) = envelope.auth_evidence.as_ref() else {
        return vec!["missing_auth_evidence".to_string()];
    };
    let actual_bindings: BTreeSet<_> = auth.bindings.iter().cloned().collect();
    let required_bindings: BTreeSet<_> = required_bindings.iter().cloned().collect();
    required_bindings
        .difference(&actual_bindings)
        .cloned()
        .map(|binding| format!("missing_auth_binding:{binding}"))
        .collect()
}

fn classify_reason_family(rejection_reason_refs: &[String]) -> Option<String> {
    if rejection_reason_refs
        .iter()
        .any(|reason| reason.contains("membership_epoch") || reason.contains("member_incarnation"))
    {
        return Some("membership_freshness".to_string());
    }
    if rejection_reason_refs
        .iter()
        .any(|reason| reason.starts_with("missing_capability:"))
    {
        return Some("capability".to_string());
    }
    if rejection_reason_refs
        .iter()
        .any(|reason| reason.starts_with("missing_witness:"))
    {
        return Some("witness".to_string());
    }
    if rejection_reason_refs
        .iter()
        .any(|reason| reason == "destination_place_unregistered")
    {
        return Some("routing".to_string());
    }
    if rejection_reason_refs.iter().any(|reason| {
        reason.starts_with("missing_auth_") || reason == "principal_place_binding_mismatch"
    }) {
        return Some("auth".to_string());
    }
    None
}

fn push_check(
    checks: &mut Vec<PracticalAlpha1TransportAdmissionCheck>,
    rejection_reason_refs: &mut Vec<String>,
    check_name: &str,
    passed: bool,
    observed: String,
    reason_refs: Vec<String>,
) {
    if !passed {
        rejection_reason_refs.extend(reason_refs.clone());
    }
    checks.push(PracticalAlpha1TransportAdmissionCheck {
        check_name: check_name.to_string(),
        passed,
        observed,
        reason_refs,
    });
}

fn what_it_proves(
    plan: &PracticalAlpha1TransportPlan,
    terminal_outcome: &str,
    reason_family: Option<&str>,
    auth_lane_present: bool,
) -> Vec<String> {
    let mut lines = Vec::new();
    match plan.transport_surface.as_str() {
        "local_tcp" => lines
            .push("checked practical package can cross a local TCP process boundary".to_string()),
        "docker_compose_tcp" => lines
            .push("checked practical package can cross a Docker Compose TCP boundary".to_string()),
        _ => {}
    }
    if terminal_outcome == "accepted" {
        lines.push("transport keeps delivery/admission evidence explicit".to_string());
    }
    if reason_family == Some("membership_freshness") {
        lines.push("stale membership is rejected without hidden refresh".to_string());
    }
    if reason_family == Some("capability") {
        lines.push("capability insufficiency stays explicit across transport".to_string());
    }
    if reason_family == Some("witness") {
        lines.push("required witness sufficiency stays explicit across transport".to_string());
    }
    lines.push("observer-safe route trace stays typed and redacted".to_string());
    if auth_lane_present {
        lines.push("auth evidence remains a separate lane from transport metadata".to_string());
    }
    lines
}

fn what_it_does_not_prove() -> Vec<String> {
    vec![
        "no production WAN or federation claim".to_string(),
        "no session replay repair or network partition recovery".to_string(),
        "no local save/load, devtools, or product prototype completion".to_string(),
        "no final public transport ABI".to_string(),
    ]
}

fn wrap_plan_error(
    path: &Path,
    error: PracticalAlpha1TransportPlanError,
) -> PracticalAlpha1TransportError {
    PracticalAlpha1TransportError {
        kind: PracticalAlpha1TransportErrorKind::TransportPlan,
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn surface_kind() -> String {
    PRACTICAL_ALPHA1_TRANSPORT_SURFACE_KIND.to_string()
}

fn scope_kind() -> String {
    PRACTICAL_ALPHA1_TRANSPORT_SCOPE_KIND.to_string()
}

fn transport_scope() -> String {
    PRACTICAL_ALPHA1_TRANSPORT_SCOPE.to_string()
}

fn retained_later_refs_default() -> Vec<String> {
    PRACTICAL_ALPHA1_TRANSPORT_RETAINED_LATER_REFS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn stop_lines_default() -> Vec<String> {
    PRACTICAL_ALPHA1_TRANSPORT_STOP_LINES
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn limitations_default() -> Vec<String> {
    PRACTICAL_ALPHA1_TRANSPORT_LIMITATIONS
        .iter()
        .map(|value| (*value).to_string())
        .collect()
}

fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), MirroreaCoreError> {
    let text = serde_json::to_string_pretty(value).map_err(|error| {
        MirroreaCoreError::new(format!("serialize practical transport output: {error}"))
    })?;
    std::fs::write(path, format!("{text}\n"))
        .map_err(|error| io_error("write practical transport output", error))
}

fn io_error(context: &str, error: impl fmt::Display) -> MirroreaCoreError {
    MirroreaCoreError::new(format!("{context}: {error}"))
}
