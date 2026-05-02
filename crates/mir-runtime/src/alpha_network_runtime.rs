use std::{
    collections::BTreeSet,
    fmt::Display,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
    time::Duration,
};

use mirrorea_core::{
    AuthEvidence, LogicalPlaceRuntimeShell, MessageEnvelope, MirroreaCoreError, PrincipalClaim,
    auth_evidence_lanes, message_envelope_lanes,
};
use serde::{Deserialize, Serialize};

const WORLD_PLACE: &str = "WorldPlace[AlphaRoom#1]";
const WORLD_KIND: &str = "WorldPlace";
const GAME_PLACE: &str = "GamePlace[SugorokuGame#1]";
const GAME_KIND: &str = "SugorokuGamePlace";
const ALICE_PLACE: &str = "ParticipantPlace[Alice]";
const TRANSPORT_SEAM: &str = "network_transport_lane";
const LOOPBACK_MEDIUM: &str = "loopback_tcp";
const BRIDGE_KIND: &str = "tcp_json_socket";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NetworkSurface {
    TcpProcessBoundary,
    DockerComposeTcp,
}

impl NetworkSurface {
    fn as_str(self) -> &'static str {
        match self {
            Self::TcpProcessBoundary => "tcp_process_boundary",
            Self::DockerComposeTcp => "docker_compose_tcp",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NetworkScenario {
    Net02,
    Net03,
    Net04,
    Net05,
    Net07,
    Net09,
}

impl NetworkScenario {
    fn parse(sample_id: &str) -> Result<Self, MirroreaCoreError> {
        match sample_id {
            "NET-02" => Ok(Self::Net02),
            "NET-03" => Ok(Self::Net03),
            "NET-04" => Ok(Self::Net04),
            "NET-05" => Ok(Self::Net05),
            "NET-07" => Ok(Self::Net07),
            "NET-09" => Ok(Self::Net09),
            _ => Err(MirroreaCoreError::new(format!(
                "unknown alpha network sample `{sample_id}`"
            ))),
        }
    }

    fn sample_id(self) -> &'static str {
        match self {
            Self::Net02 => "NET-02",
            Self::Net03 => "NET-03",
            Self::Net04 => "NET-04",
            Self::Net05 => "NET-05",
            Self::Net07 => "NET-07",
            Self::Net09 => "NET-09",
        }
    }

    fn sample_name(self) -> &'static str {
        match self {
            Self::Net02 => "docker_two_process_envelope",
            Self::Net03 => "stale_membership_rejected",
            Self::Net04 => "missing_capability_rejected",
            Self::Net05 => "missing_witness_rejected",
            Self::Net07 => "observer_safe_route_trace",
            Self::Net09 => "auth_evidence_lane_preserved",
        }
    }

    fn required_capabilities(self) -> Vec<String> {
        match self {
            Self::Net02 | Self::Net03 | Self::Net05 | Self::Net07 | Self::Net09 => {
                vec!["PublishRoll".to_string(), "HandoffTurn".to_string()]
            }
            Self::Net04 => vec!["PublishRoll".to_string(), "HandoffTurn".to_string()],
        }
    }

    fn claimed_capabilities(self) -> Vec<String> {
        match self {
            Self::Net04 => vec!["PublishRoll".to_string()],
            Self::Net02 | Self::Net03 | Self::Net05 | Self::Net07 | Self::Net09 => {
                vec!["PublishRoll".to_string(), "HandoffTurn".to_string()]
            }
        }
    }

    fn required_witness_refs(self) -> Vec<String> {
        match self {
            Self::Net02 | Self::Net03 | Self::Net04 | Self::Net05 | Self::Net07 | Self::Net09 => {
                vec!["draw_pub#1".to_string()]
            }
        }
    }

    fn actual_witness_refs(self) -> Vec<String> {
        match self {
            Self::Net05 => vec!["membership_anchor#1".to_string()],
            Self::Net02 | Self::Net03 | Self::Net04 | Self::Net07 | Self::Net09 => {
                vec!["draw_pub#1".to_string()]
            }
        }
    }

    fn auth_evidence(self) -> Option<AuthEvidence> {
        match self {
            Self::Net09 => Some(AuthEvidence {
                kind: "session_attestation".to_string(),
                subject: "Alice".to_string(),
                issuer: "LocalAuthService".to_string(),
                bindings: vec![
                    format!("route={GAME_PLACE}"),
                    format!("transport={TRANSPORT_SEAM}"),
                ],
                notes: vec![
                    "transport carries auth as a separate lane".to_string(),
                    "signatures prove provenance, not semantic safety".to_string(),
                ],
            }),
            _ => None,
        }
    }

    fn auth_bindings_required(self) -> Vec<String> {
        match self {
            Self::Net09 => vec![
                format!("route={GAME_PLACE}"),
                format!("transport={TRANSPORT_SEAM}"),
            ],
            _ => Vec::new(),
        }
    }

    fn offered_membership_epoch(self) -> u64 {
        match self {
            Self::Net03 => 0,
            Self::Net02 | Self::Net04 | Self::Net05 | Self::Net07 | Self::Net09 => 0,
        }
    }

    fn offered_member_incarnation(self) -> u64 {
        match self {
            Self::Net03 => 0,
            Self::Net02 | Self::Net04 | Self::Net05 | Self::Net07 | Self::Net09 => 0,
        }
    }

    fn payload_ref(self) -> &'static str {
        match self {
            Self::Net02 | Self::Net03 | Self::Net04 | Self::Net05 => "handoff_from_roll:draw_pub#1",
            Self::Net07 => "observer_route_trace_probe",
            Self::Net09 => "auth_lane_probe",
        }
    }

    fn request_notes(self) -> Vec<String> {
        let mut notes = vec![
            "non-public Stage C transport floor".to_string(),
            "transport never mints auth/capability/witness lanes".to_string(),
        ];
        match self {
            Self::Net03 => notes
                .push("stale membership is expected to reject without hidden refresh".to_string()),
            Self::Net04 => notes.push(
                "claimed capability set intentionally omits one required capability".to_string(),
            ),
            Self::Net05 => notes.push(
                "required witness is intentionally absent from the actual witness lane".to_string(),
            ),
            Self::Net07 => {
                notes.push("observer-safe route trace remains redacted and typed".to_string())
            }
            Self::Net09 => {
                notes.push("auth evidence must remain separate from transport metadata".to_string())
            }
            Self::Net02 => {}
        }
        notes
    }

    fn expected_terminal_outcome(self) -> &'static str {
        match self {
            Self::Net02 | Self::Net07 | Self::Net09 => "accepted",
            Self::Net03 | Self::Net04 | Self::Net05 => "rejected",
        }
    }

    fn reason_family_if_rejected(self) -> Option<&'static str> {
        match self {
            Self::Net02 | Self::Net07 | Self::Net09 => None,
            Self::Net03 => Some("membership_freshness"),
            Self::Net04 => Some("capability"),
            Self::Net05 => Some("witness"),
        }
    }

    fn what_it_proves(self) -> Vec<String> {
        match self {
            Self::Net02 => vec![
                "world/participant exchange can traverse a real TCP process boundary".to_string(),
                "witness lane remains explicit across the network seam".to_string(),
                "the same runtime floor can be replayed under Docker Compose".to_string(),
            ],
            Self::Net03 => vec![
                "stale membership is rejected as an explicit freshness failure".to_string(),
                "reconnect success is not inferred from transport liveness".to_string(),
            ],
            Self::Net04 => vec![
                "capability insufficiency stays explicit across the transport seam".to_string(),
                "delivery success is separated from admission success".to_string(),
            ],
            Self::Net05 => vec![
                "required witness sufficiency stays explicit across the transport seam".to_string(),
                "transport does not synthesize missing witness evidence".to_string(),
            ],
            Self::Net07 => vec![
                "observer-safe route trace can stay typed and redacted".to_string(),
                "route trace does not need raw auth or capability payloads".to_string(),
            ],
            Self::Net09 => vec![
                "auth evidence remains a separate lane from transport delivery".to_string(),
                "auth subject/issuer/binding coherence is checked explicitly".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkPeer {
    pub process_id: String,
    pub role: String,
    pub endpoint: String,
    pub containerized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkAdmissionCheck {
    pub check_name: String,
    pub passed: bool,
    pub observed: String,
    pub reason_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObserverRouteTraceRow {
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
pub struct AuthLaneReport {
    pub auth_present: bool,
    pub subject: String,
    pub issuer: String,
    pub bindings: Vec<String>,
    pub preserved_separately: bool,
    pub transport_lane: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct NetworkRuntimeReport {
    pub runtime_scope: String,
    pub sample_id: String,
    pub sample_name: String,
    pub transport_surface: String,
    pub transport_medium: String,
    pub transport_seam: String,
    pub bridge_kind: String,
    pub bridge_processes: Vec<NetworkPeer>,
    pub message_envelope_lanes: Vec<String>,
    pub auth_evidence_lanes: Vec<String>,
    pub request_envelope: MessageEnvelope,
    pub required_capabilities: Vec<String>,
    pub required_witness_refs: Vec<String>,
    pub admission_checks: Vec<NetworkAdmissionCheck>,
    pub observer_route_trace: Vec<ObserverRouteTraceRow>,
    pub auth_lane: Option<AuthLaneReport>,
    pub world_membership_epoch: u64,
    pub world_active_participants: Vec<String>,
    pub world_places: Vec<String>,
    pub terminal_outcome: String,
    pub reason_family: Option<String>,
    pub rejection_reason_refs: Vec<String>,
    pub retained_later_refs: Vec<String>,
    pub what_it_proves: Vec<String>,
    pub what_it_does_not_prove: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TransportWireRequest {
    sample_id: String,
    request_envelope: MessageEnvelope,
    required_capabilities: Vec<String>,
    required_witness_refs: Vec<String>,
    auth_bindings_required: Vec<String>,
    notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct TransportWireResponse {
    sample_id: String,
    transport_medium: String,
    transport_seam: String,
    admission_checks: Vec<NetworkAdmissionCheck>,
    observer_route_trace: Vec<ObserverRouteTraceRow>,
    auth_lane: Option<AuthLaneReport>,
    world_membership_epoch: u64,
    world_active_participants: Vec<String>,
    world_places: Vec<String>,
    terminal_outcome: String,
    reason_family: Option<String>,
    rejection_reason_refs: Vec<String>,
    notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct NetworkWorldServerReport {
    pub sample_id: String,
    pub bind_addr: String,
    pub transport_medium: String,
    pub terminal_outcome: String,
    pub reason_family: Option<String>,
    pub rejection_reason_refs: Vec<String>,
    pub admission_checks: Vec<NetworkAdmissionCheck>,
    pub world_membership_epoch: u64,
    pub world_active_participants: Vec<String>,
    pub world_places: Vec<String>,
    pub notes: Vec<String>,
}

pub fn build_docker_two_process_envelope_report() -> Result<NetworkRuntimeReport, MirroreaCoreError>
{
    execute_local_tcp(NetworkScenario::Net02, LOOPBACK_MEDIUM)
}

pub fn build_stale_membership_rejection_report() -> Result<NetworkRuntimeReport, MirroreaCoreError>
{
    execute_local_tcp(NetworkScenario::Net03, LOOPBACK_MEDIUM)
}

pub fn build_missing_capability_rejection_report() -> Result<NetworkRuntimeReport, MirroreaCoreError>
{
    execute_local_tcp(NetworkScenario::Net04, LOOPBACK_MEDIUM)
}

pub fn build_missing_witness_rejection_report() -> Result<NetworkRuntimeReport, MirroreaCoreError> {
    execute_local_tcp(NetworkScenario::Net05, LOOPBACK_MEDIUM)
}

pub fn build_observer_safe_route_trace_report() -> Result<NetworkRuntimeReport, MirroreaCoreError> {
    execute_local_tcp(NetworkScenario::Net07, LOOPBACK_MEDIUM)
}

pub fn build_auth_evidence_lane_preserved_report() -> Result<NetworkRuntimeReport, MirroreaCoreError>
{
    execute_local_tcp(NetworkScenario::Net09, LOOPBACK_MEDIUM)
}

pub fn build_closeout_reports() -> Result<Vec<NetworkRuntimeReport>, MirroreaCoreError> {
    Ok(vec![
        build_docker_two_process_envelope_report()?,
        build_stale_membership_rejection_report()?,
        build_missing_capability_rejection_report()?,
        build_missing_witness_rejection_report()?,
        build_observer_safe_route_trace_report()?,
        build_auth_evidence_lane_preserved_report()?,
    ])
}

pub fn run_world_server(
    bind_addr: &str,
    transport_medium: &str,
    output_path: Option<&Path>,
) -> Result<(), MirroreaCoreError> {
    let listener = TcpListener::bind(bind_addr)
        .map_err(|error| io_error("bind alpha network world server", error))?;
    serve_once(listener, transport_medium, output_path)
}

pub fn run_participant_client(
    sample_id: &str,
    addr: &str,
    transport_medium: &str,
    output_path: Option<&Path>,
) -> Result<NetworkRuntimeReport, MirroreaCoreError> {
    let scenario = NetworkScenario::parse(sample_id)?;
    let request = build_wire_request(scenario, transport_medium)?;
    let stream = connect_with_retry(addr)?;
    let response = send_request(stream, &request)?;
    let report = build_report_from_wire_response(
        scenario,
        NetworkSurface::DockerComposeTcp,
        request,
        response,
        addr,
        transport_medium,
        true,
    )?;
    if report.terminal_outcome != scenario.expected_terminal_outcome() {
        return Err(MirroreaCoreError::new(format!(
            "{} expected `{}` but observed `{}`",
            scenario.sample_id(),
            scenario.expected_terminal_outcome(),
            report.terminal_outcome
        )));
    }
    if report.reason_family.as_deref() != scenario.reason_family_if_rejected() {
        return Err(MirroreaCoreError::new(format!(
            "{} expected reason family {:?} but observed {:?}",
            scenario.sample_id(),
            scenario.reason_family_if_rejected(),
            report.reason_family
        )));
    }
    if let Some(path) = output_path {
        write_json_file(path, &report)?;
    }
    Ok(report)
}

fn execute_local_tcp(
    scenario: NetworkScenario,
    transport_medium: &str,
) -> Result<NetworkRuntimeReport, MirroreaCoreError> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|error| io_error("bind loopback alpha network listener", error))?;
    let bind_addr = listener
        .local_addr()
        .map_err(|error| io_error("read loopback listener address", error))?;
    let world_addr = bind_addr.to_string();
    let server_medium = transport_medium.to_string();
    let server = thread::spawn(move || serve_once(listener, &server_medium, None));
    let request = build_wire_request(scenario, transport_medium)?;
    let stream = connect_with_retry(&world_addr)?;
    let response = send_request(stream, &request)?;
    server
        .join()
        .map_err(|_| MirroreaCoreError::new("alpha network world server thread panicked"))??;
    build_report_from_wire_response(
        scenario,
        NetworkSurface::TcpProcessBoundary,
        request,
        response,
        &world_addr,
        transport_medium,
        false,
    )
}

fn serve_once(
    listener: TcpListener,
    transport_medium: &str,
    output_path: Option<&Path>,
) -> Result<(), MirroreaCoreError> {
    let bind_addr = listener
        .local_addr()
        .map_err(|error| io_error("read alpha network world bind address", error))?;
    let (stream, _) = listener
        .accept()
        .map_err(|error| io_error("accept alpha network connection", error))?;
    let mut stream = stream;
    let request = read_request(
        stream
            .try_clone()
            .map_err(|error| io_error("clone alpha network stream for request read", error))?,
    )?;
    let response = evaluate_request(&request, transport_medium)?;
    write_response(&mut stream, &response)?;
    if let Some(path) = output_path {
        let server_report = NetworkWorldServerReport {
            sample_id: response.sample_id.clone(),
            bind_addr: bind_addr.to_string(),
            transport_medium: response.transport_medium.clone(),
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

fn read_request(stream: TcpStream) -> Result<TransportWireRequest, MirroreaCoreError> {
    let mut reader = BufReader::new(
        stream
            .try_clone()
            .map_err(|error| io_error("clone alpha network stream for read", error))?,
    );
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .map_err(|error| io_error("read alpha network request", error))?;
    serde_json::from_str(&buffer)
        .map_err(|error| MirroreaCoreError::new(format!("parse alpha network request: {error}")))
}

fn send_request(
    mut stream: TcpStream,
    request: &TransportWireRequest,
) -> Result<TransportWireResponse, MirroreaCoreError> {
    let encoded = serde_json::to_string(request).map_err(|error| {
        MirroreaCoreError::new(format!("encode alpha network request: {error}"))
    })?;
    stream
        .write_all(encoded.as_bytes())
        .map_err(|error| io_error("write alpha network request", error))?;
    stream
        .write_all(b"\n")
        .map_err(|error| io_error("write alpha network request newline", error))?;
    stream
        .flush()
        .map_err(|error| io_error("flush alpha network request", error))?;

    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .map_err(|error| io_error("read alpha network response", error))?;
    serde_json::from_str(&buffer)
        .map_err(|error| MirroreaCoreError::new(format!("parse alpha network response: {error}")))
}

fn write_response(
    stream: &mut TcpStream,
    response: &TransportWireResponse,
) -> Result<(), MirroreaCoreError> {
    let encoded = serde_json::to_string(response).map_err(|error| {
        MirroreaCoreError::new(format!("encode alpha network response: {error}"))
    })?;
    stream
        .write_all(encoded.as_bytes())
        .map_err(|error| io_error("write alpha network response", error))?;
    stream
        .write_all(b"\n")
        .map_err(|error| io_error("write alpha network response newline", error))?;
    stream
        .flush()
        .map_err(|error| io_error("flush alpha network response", error))
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
        "connect to alpha network world `{addr}` failed: {}",
        last_error.unwrap_or_else(|| "unknown error".to_string())
    )))
}

fn build_report_from_wire_response(
    scenario: NetworkScenario,
    surface: NetworkSurface,
    request: TransportWireRequest,
    response: TransportWireResponse,
    world_addr: &str,
    transport_medium: &str,
    containerized: bool,
) -> Result<NetworkRuntimeReport, MirroreaCoreError> {
    if response.sample_id != request.sample_id {
        return Err(MirroreaCoreError::new(format!(
            "alpha network response sample_id mismatch: request={} response={}",
            request.sample_id, response.sample_id
        )));
    }

    Ok(NetworkRuntimeReport {
        runtime_scope: "alpha_network_runtime_stage_c_narrow".to_string(),
        sample_id: scenario.sample_id().to_string(),
        sample_name: scenario.sample_name().to_string(),
        transport_surface: surface.as_str().to_string(),
        transport_medium: transport_medium.to_string(),
        transport_seam: TRANSPORT_SEAM.to_string(),
        bridge_kind: BRIDGE_KIND.to_string(),
        bridge_processes: vec![
            NetworkPeer {
                process_id: "world-server".to_string(),
                role: "world_server".to_string(),
                endpoint: world_addr.to_string(),
                containerized,
            },
            NetworkPeer {
                process_id: "participant-client".to_string(),
                role: "participant_client".to_string(),
                endpoint: "ephemeral-client".to_string(),
                containerized,
            },
        ],
        message_envelope_lanes: message_envelope_lanes(),
        auth_evidence_lanes: auth_evidence_lanes(),
        request_envelope: request.request_envelope,
        required_capabilities: request.required_capabilities,
        required_witness_refs: request.required_witness_refs,
        admission_checks: response.admission_checks,
        observer_route_trace: response.observer_route_trace,
        auth_lane: response.auth_lane,
        world_membership_epoch: response.world_membership_epoch,
        world_active_participants: response.world_active_participants,
        world_places: response.world_places,
        terminal_outcome: response.terminal_outcome,
        reason_family: response.reason_family,
        rejection_reason_refs: response.rejection_reason_refs,
        retained_later_refs: retained_later_refs(),
        what_it_proves: scenario.what_it_proves(),
        what_it_does_not_prove: what_it_does_not_prove(),
        notes: response.notes,
    })
}

fn build_wire_request(
    scenario: NetworkScenario,
    transport_medium: &str,
) -> Result<TransportWireRequest, MirroreaCoreError> {
    let envelope = MessageEnvelope {
        envelope_id: format!("{}_request#1", scenario.sample_id().to_lowercase()),
        from_place: ALICE_PLACE.to_string(),
        to_place: GAME_PLACE.to_string(),
        transport: TRANSPORT_SEAM.to_string(),
        transport_medium: Some(transport_medium.to_string()),
        transport_seam: TRANSPORT_SEAM.to_string(),
        payload_kind: "published_turn_transition".to_string(),
        payload_ref: scenario.payload_ref().to_string(),
        principal_claim: PrincipalClaim {
            principal: "Alice".to_string(),
            participant_place: ALICE_PLACE.to_string(),
            claimed_authority: "GameAuthority.Player".to_string(),
            claimed_capabilities: scenario.claimed_capabilities(),
        },
        auth_evidence: scenario.auth_evidence(),
        emitter_principal: "Alice".to_string(),
        membership_epoch: scenario.offered_membership_epoch(),
        member_incarnation: scenario.offered_member_incarnation(),
        freshness_checks: vec![
            "membership_epoch matches active registry frontier".to_string(),
            "member_incarnation matches active member record".to_string(),
        ],
        capability_requirements: scenario.required_capabilities(),
        authorization_checks: vec![
            "principal/place binding remains explicit".to_string(),
            "transport does not mint capability/auth/witness lanes".to_string(),
        ],
        witness_refs: scenario.actual_witness_refs(),
        dispatch_outcome: "delivery_requested".to_string(),
        notes: scenario.request_notes(),
    };
    envelope.validate()?;

    Ok(TransportWireRequest {
        sample_id: scenario.sample_id().to_string(),
        request_envelope: envelope,
        required_capabilities: scenario.required_capabilities(),
        required_witness_refs: scenario.required_witness_refs(),
        auth_bindings_required: scenario.auth_bindings_required(),
        notes: vec![
            "alpha-local transport request".to_string(),
            "non-public Stage C runtime floor".to_string(),
        ],
    })
}

fn evaluate_request(
    request: &TransportWireRequest,
    transport_medium: &str,
) -> Result<TransportWireResponse, MirroreaCoreError> {
    request.request_envelope.validate()?;
    let scenario = NetworkScenario::parse(&request.sample_id)?;
    let shell = bootstrap_network_shell(scenario)?;
    let snapshot = shell.snapshot();
    let member = snapshot
        .membership
        .members
        .get(&request.request_envelope.emitter_principal)
        .ok_or_else(|| {
            MirroreaCoreError::new(format!(
                "alpha network principal `{}` is absent from world membership",
                request.request_envelope.emitter_principal
            ))
        })?;

    let mut checks = Vec::new();
    let mut rejection_reason_refs = Vec::new();

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

    let incarnation_passed = request.request_envelope.member_incarnation == member.incarnation;
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "member_incarnation_current",
        incarnation_passed,
        format!(
            "offered={} current={}",
            request.request_envelope.member_incarnation, member.incarnation
        ),
        vec!["member_incarnation_drift".to_string()],
    );

    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "participant_active",
        member.active,
        format!("active={}", member.active),
        vec!["participant_inactive".to_string()],
    );

    let participant_place_passed = member.place == request.request_envelope.from_place;
    push_check(
        &mut checks,
        &mut rejection_reason_refs,
        "participant_place_current",
        participant_place_passed,
        format!(
            "recorded={} offered={}",
            member.place, request.request_envelope.from_place
        ),
        vec!["participant_place_drift".to_string()],
    );

    let claimed_capabilities: BTreeSet<_> = request
        .request_envelope
        .principal_claim
        .claimed_capabilities
        .iter()
        .cloned()
        .collect();
    let required_capabilities: BTreeSet<_> =
        request.required_capabilities.iter().cloned().collect();
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
            request.required_capabilities
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
    let required_witness_refs: BTreeSet<_> =
        request.required_witness_refs.iter().cloned().collect();
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
            request.request_envelope.witness_refs, request.required_witness_refs
        ),
        missing_witness_refs
            .into_iter()
            .map(|witness| format!("missing_witness:{witness}"))
            .collect(),
    );

    let auth_lane = build_auth_lane_report(
        &request.request_envelope.auth_evidence,
        &request.auth_bindings_required,
    );
    let auth_reason_refs =
        auth_lane_reason_refs(&request.request_envelope, &request.auth_bindings_required);
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

    Ok(TransportWireResponse {
        sample_id: request.sample_id.clone(),
        transport_medium: transport_medium.to_string(),
        transport_seam: TRANSPORT_SEAM.to_string(),
        admission_checks: checks,
        observer_route_trace: build_observer_route_trace(
            &request.request_envelope,
            transport_medium,
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
            "non-public Stage C network floor".to_string(),
            "controlled local TCP or Docker Compose evidence only".to_string(),
            "no production WAN/session/replay claim".to_string(),
        ],
    })
}

fn bootstrap_network_shell(
    scenario: NetworkScenario,
) -> Result<LogicalPlaceRuntimeShell, MirroreaCoreError> {
    let mut shell = LogicalPlaceRuntimeShell::default();
    shell.register_place(WORLD_PLACE, WORLD_KIND)?;
    shell.register_place(GAME_PLACE, GAME_KIND)?;
    shell.add_initial_participant("Alice")?;
    shell.add_initial_participant("Bob")?;
    if matches!(scenario, NetworkScenario::Net03) {
        shell.leave_participant("Alice")?;
    }
    Ok(shell)
}

fn build_observer_route_trace(
    request_envelope: &MessageEnvelope,
    transport_medium: &str,
    terminal_outcome: &str,
) -> Vec<ObserverRouteTraceRow> {
    vec![
        ObserverRouteTraceRow {
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
        ObserverRouteTraceRow {
            hop_index: 2,
            envelope_id: format!("delivery_receipt:{}", request_envelope.envelope_id),
            from_place: GAME_PLACE.to_string(),
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
) -> Option<AuthLaneReport> {
    auth_evidence.as_ref().map(|auth| AuthLaneReport {
        auth_present: true,
        subject: auth.subject.clone(),
        issuer: auth.issuer.clone(),
        bindings: auth.bindings.clone(),
        preserved_separately: true,
        transport_lane: TRANSPORT_SEAM.to_string(),
        notes: if required_bindings.is_empty() {
            vec!["auth lane present but not required for this sample".to_string()]
        } else {
            vec!["auth lane preserved separately from transport metadata".to_string()]
        },
    })
}

fn auth_lane_reason_refs(envelope: &MessageEnvelope, required_bindings: &[String]) -> Vec<String> {
    let Some(auth) = envelope.auth_evidence.as_ref() else {
        return if required_bindings.is_empty() {
            Vec::new()
        } else {
            vec!["auth_evidence_missing".to_string()]
        };
    };

    let mut reasons = Vec::new();
    if auth.subject != envelope.emitter_principal {
        reasons.push("auth_subject_mismatch".to_string());
    }
    if auth.issuer.is_empty() {
        reasons.push("auth_issuer_missing".to_string());
    }
    for binding in required_bindings {
        if !auth.bindings.iter().any(|candidate| candidate == binding) {
            reasons.push(format!("auth_binding_missing:{binding}"));
        }
    }
    reasons
}

fn classify_reason_family(rejection_reason_refs: &[String]) -> Option<String> {
    if rejection_reason_refs.is_empty() {
        return None;
    }
    if rejection_reason_refs
        .iter()
        .any(|reason| reason.starts_with("membership_") || reason == "participant_inactive")
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
        .any(|reason| reason.starts_with("auth_"))
    {
        return Some("auth_evidence".to_string());
    }
    Some("admission".to_string())
}

fn push_check(
    checks: &mut Vec<NetworkAdmissionCheck>,
    rejection_reason_refs: &mut Vec<String>,
    check_name: &str,
    passed: bool,
    observed: String,
    failure_reason_refs: Vec<String>,
) {
    if !passed {
        rejection_reason_refs.extend(failure_reason_refs.clone());
    }
    checks.push(NetworkAdmissionCheck {
        check_name: check_name.to_string(),
        passed,
        observed,
        reason_refs: if passed {
            Vec::new()
        } else {
            failure_reason_refs
        },
    });
}

fn retained_later_refs() -> Vec<String> {
    vec![
        "route_rebinding_no_shadow".to_string(),
        "network_partition_explicit_failure".to_string(),
        "transport_medium_change_preserves_contract".to_string(),
        "production_wan_federation".to_string(),
        "crypto_session_protocol".to_string(),
        "durable_replay_commit".to_string(),
        "continuous_shared_runtime_state".to_string(),
        "distributed_save_load".to_string(),
        "final_public_transport_abi".to_string(),
    ]
}

fn what_it_does_not_prove() -> Vec<String> {
    vec![
        "no production WAN federation".to_string(),
        "no cryptographic session protocol".to_string(),
        "no durable replay or reconnect repair".to_string(),
        "no continuous shared runtime state across worlds".to_string(),
        "no final public transport ABI".to_string(),
    ]
}

fn write_json_file<T: Serialize>(path: &Path, value: &T) -> Result<(), MirroreaCoreError> {
    let encoded = serde_json::to_string_pretty(value)
        .map_err(|error| MirroreaCoreError::new(format!("encode json output: {error}")))?;
    fs::write(path, encoded).map_err(|error| io_error("write alpha network json output", error))
}

fn io_error(context: &str, error: impl Display) -> MirroreaCoreError {
    MirroreaCoreError::new(format!("{context}: {error}"))
}
