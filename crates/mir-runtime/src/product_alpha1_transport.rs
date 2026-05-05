use std::{
    collections::BTreeSet,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use serde::{Deserialize, Serialize};

use crate::product_alpha1_session::{
    ProductAlpha1EventEdge, ProductAlpha1EventNode, ProductAlpha1ObserverSafeExport,
    ProductAlpha1RouteEntry, ProductAlpha1SessionCarrier, ProductAlpha1SessionError,
    ProductAlpha1SessionErrorKind,
};

pub const PRODUCT_ALPHA1_TRANSPORT_SURFACE_KIND: &str = "product_alpha1_transport_report";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1LanePreservation {
    pub transport_lane: String,
    pub auth_lane_preserved: bool,
    pub membership_lane_preserved: bool,
    pub capability_lane_preserved: bool,
    pub witness_lane_preserved: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1TransportReport {
    #[serde(default = "transport_surface_kind")]
    pub surface_kind: String,
    pub session_id: String,
    pub mode: String,
    pub transport_medium: String,
    pub transport_seam: String,
    pub envelope_id: String,
    pub route_id: String,
    pub terminal_outcome: String,
    pub wire_roundtrip_executed: bool,
    pub wire_bind_addr: String,
    pub wire_response_status: String,
    pub lane_preservation: ProductAlpha1LanePreservation,
    pub local_transport_executed: bool,
    pub docker_compose_tcp_claimed: bool,
    pub docker_compose_executed: bool,
    pub docker_compose_execution_status: String,
    pub docker_compose_file: String,
    pub docker_compose_output_dir: String,
    pub docker_world_report_path: String,
    pub docker_participant_report_path: String,
    pub docker_world_terminal_outcome: String,
    pub docker_participant_terminal_outcome: String,
    pub host_cli_fixture_execution: bool,
    pub package_native_execution_claimed: bool,
    pub native_execution_policy: String,
    pub wan_federation_claimed: bool,
    pub non_claims: Vec<String>,
    pub product_alpha1_ready: bool,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAlpha1TransportEndpointReport {
    #[serde(default = "transport_endpoint_surface_kind")]
    pub surface_kind: String,
    pub role: String,
    pub session_id: String,
    pub package_id: String,
    pub endpoint: String,
    pub transport_medium: String,
    pub transport_lane: String,
    pub terminal_outcome: String,
    pub rejection_reason_refs: Vec<String>,
    pub auth_lane_preserved: bool,
    pub membership_lane_preserved: bool,
    pub capability_lane_preserved: bool,
    pub witness_lane_preserved: bool,
    pub auth_decision_ref: String,
    pub capability_decision_ref: String,
    pub observer_safe_witness_relation_refs: Vec<String>,
    pub capability_requirement_count: usize,
    pub witness_requirement_count: usize,
    pub host_cli_fixture_execution: bool,
    pub package_native_execution_claimed: bool,
    pub wan_federation_claimed: bool,
    pub final_public_api_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ProductAlpha1TransportWireRequest {
    session_id: String,
    package_id: String,
    from_place: String,
    to_place: String,
    envelope_id: String,
    membership_epoch: u64,
    auth_lane_present: bool,
    capability_lane_present: bool,
    witness_lane_present: bool,
    auth_decision_ref: String,
    capability_decision_ref: String,
    observer_safe_witness_relation_refs: Vec<String>,
    capability_requirement_count: usize,
    witness_requirement_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ProductAlpha1TransportWireResponse {
    session_id: String,
    package_id: String,
    envelope_id: String,
    terminal_outcome: String,
    rejection_reason_refs: Vec<String>,
    transport_lane_preserved: bool,
    auth_lane_preserved: bool,
    membership_lane_preserved: bool,
    capability_lane_preserved: bool,
    witness_lane_preserved: bool,
    auth_decision_ref: String,
    capability_decision_ref: String,
    observer_safe_witness_relation_refs: Vec<String>,
    capability_requirement_count: usize,
    witness_requirement_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProductAlpha1TransportWireTrace {
    bind_addr: String,
    response_status: String,
}

pub fn run_product_alpha1_transport_for_session(
    session: &ProductAlpha1SessionCarrier,
    mode: &str,
) -> Result<(ProductAlpha1SessionCarrier, ProductAlpha1TransportReport), ProductAlpha1SessionError>
{
    let (
        transport_medium,
        transport_seam,
        local_executed,
        docker_claimed,
        docker_executed,
        docker_status,
    ) = match mode {
        "local" => (
            "local_loopback_tcp",
            "product_local_loopback",
            true,
            false,
            false,
            "not_applicable",
        ),
        "docker" => (
            "docker_compose_tcp",
            "product_docker_compose_tcp",
            false,
            true,
            false,
            "compose_tcp_boundary_reported_without_container_execution",
        ),
        _ => {
            return Err(ProductAlpha1SessionError {
                kind: ProductAlpha1SessionErrorKind::UnsupportedPackage,
                path: PathBuf::from("<transport-mode>"),
                detail: format!(
                    "product alpha-1 transport supports `local` or `docker`, found `{mode}`"
                ),
            });
        }
    };

    let mut next = session.clone();
    next.phase = "transported".to_string();
    let envelope_id = unique_id(
        &next
            .event_dag
            .nodes
            .iter()
            .filter_map(|node| node.envelope_ref.clone())
            .collect::<BTreeSet<_>>(),
        &format!("envelope#product-transport#{mode}"),
    );
    let wire_trace = if mode == "local" {
        Some(execute_local_tcp_roundtrip(session, &envelope_id)?)
    } else {
        None
    };
    let send_event_id = unique_event_id(&next, &format!("event#transport-send#{mode}"));
    let receive_event_id = unique_event_id(&next, &format!("event#transport-receive#{mode}"));
    append_transport_events(
        &mut next,
        &send_event_id,
        &receive_event_id,
        &envelope_id,
        transport_medium,
    );
    let route_id = unique_route_id(&next, &format!("route#product-transport#{mode}"));
    next.route_graph.routes.push(ProductAlpha1RouteEntry {
        route_id: route_id.clone(),
        envelope_id: envelope_id.clone(),
        from_place: next.runtime_plan.entry_place.clone(),
        to_place: next.runtime_plan.entry_place.clone(),
        transport_lane: transport_seam.to_string(),
        auth_lane_preserved: true,
        membership_lane_preserved: true,
        witness_lane_preserved: true,
        capability_lane_preserved: true,
    });
    refresh_observer_safe_transport_export(&mut next);

    let lane_preservation = ProductAlpha1LanePreservation {
        transport_lane: transport_seam.to_string(),
        auth_lane_preserved: true,
        membership_lane_preserved: true,
        capability_lane_preserved: true,
        witness_lane_preserved: true,
    };
    let report = ProductAlpha1TransportReport {
        surface_kind: transport_surface_kind(),
        session_id: session.session_id.clone(),
        mode: mode.to_string(),
        transport_medium: transport_medium.to_string(),
        transport_seam: transport_seam.to_string(),
        envelope_id,
        route_id,
        terminal_outcome: "accepted".to_string(),
        wire_roundtrip_executed: wire_trace.is_some(),
        wire_bind_addr: wire_trace
            .as_ref()
            .map(|trace| trace.bind_addr.clone())
            .unwrap_or_else(|| "not_applicable".to_string()),
        wire_response_status: wire_trace
            .as_ref()
            .map(|trace| trace.response_status.clone())
            .unwrap_or_else(|| "not_applicable".to_string()),
        lane_preservation,
        local_transport_executed: local_executed,
        docker_compose_tcp_claimed: docker_claimed,
        docker_compose_executed: docker_executed,
        docker_compose_execution_status: docker_status.to_string(),
        docker_compose_file: "not_applicable".to_string(),
        docker_compose_output_dir: "not_applicable".to_string(),
        docker_world_report_path: "not_applicable".to_string(),
        docker_participant_report_path: "not_applicable".to_string(),
        docker_world_terminal_outcome: "not_applicable".to_string(),
        docker_participant_terminal_outcome: "not_applicable".to_string(),
        host_cli_fixture_execution: false,
        package_native_execution_claimed: false,
        native_execution_policy: "package_native_execution_disabled".to_string(),
        wan_federation_claimed: false,
        non_claims: vec![
            "not WAN/federation transport".to_string(),
            "not final public transport ABI".to_string(),
            "docker mode is a bounded Docker Compose TCP alpha boundary and not production orchestration".to_string(),
        ],
        product_alpha1_ready: false,
        final_public_api_frozen: false,
    };
    Ok((next, report))
}

pub fn run_product_alpha1_transport_world_server(
    session: &ProductAlpha1SessionCarrier,
    bind_addr: &str,
    output_path: Option<&Path>,
) -> Result<ProductAlpha1TransportEndpointReport, ProductAlpha1SessionError> {
    let listener = TcpListener::bind(bind_addr).map_err(transport_error)?;
    let endpoint = listener.local_addr().map_err(transport_error)?.to_string();
    let response = serve_once(listener, session.clone())?;
    let report = endpoint_report("world", session, &endpoint, response);
    if let Some(path) = output_path {
        write_endpoint_report(path, &report)?;
    }
    Ok(report)
}

pub fn run_product_alpha1_transport_participant_client(
    session: &ProductAlpha1SessionCarrier,
    addr: &str,
    output_path: Option<&Path>,
) -> Result<ProductAlpha1TransportEndpointReport, ProductAlpha1SessionError> {
    let request = build_wire_request(
        session,
        "envelope#product-transport#docker-compose-tcp".to_string(),
    );
    let stream = connect_with_retry(addr)?;
    let response = send_request(stream, &request)?;
    let report = endpoint_report("participant", session, addr, response);
    if let Some(path) = output_path {
        write_endpoint_report(path, &report)?;
    }
    Ok(report)
}

fn execute_local_tcp_roundtrip(
    session: &ProductAlpha1SessionCarrier,
    envelope_id: &str,
) -> Result<ProductAlpha1TransportWireTrace, ProductAlpha1SessionError> {
    let listener = TcpListener::bind("127.0.0.1:0").map_err(|error| transport_error(error))?;
    let bind_addr = listener
        .local_addr()
        .map_err(|error| transport_error(error))?
        .to_string();
    let expected_session = session.clone();
    let server = thread::spawn(move || serve_once(listener, expected_session));
    let request = build_wire_request(session, envelope_id.to_string());
    let stream = connect_with_retry(&bind_addr)?;
    let response = send_request(stream, &request)?;
    let server_result = server.join().map_err(|_| ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Transport,
        path: PathBuf::from("<product-alpha1-local-transport>"),
        detail: "product alpha-1 local transport server thread panicked".to_string(),
    })?;
    let server_response = server_result?;
    if response.terminal_outcome != "accepted" {
        return Err(ProductAlpha1SessionError {
            kind: ProductAlpha1SessionErrorKind::Transport,
            path: PathBuf::from("<product-alpha1-local-transport>"),
            detail: format!(
                "product alpha-1 local transport rejected envelope {}: {:?}",
                response.envelope_id, response.rejection_reason_refs
            ),
        });
    }
    if server_response.terminal_outcome != "accepted" {
        return Err(ProductAlpha1SessionError {
            kind: ProductAlpha1SessionErrorKind::Transport,
            path: PathBuf::from("<product-alpha1-local-transport>"),
            detail: format!(
                "product alpha-1 local transport server rejected envelope {}: {:?}",
                server_response.envelope_id, server_response.rejection_reason_refs
            ),
        });
    }
    Ok(ProductAlpha1TransportWireTrace {
        bind_addr,
        response_status: response.terminal_outcome,
    })
}

fn serve_once(
    listener: TcpListener,
    expected_session: ProductAlpha1SessionCarrier,
) -> Result<ProductAlpha1TransportWireResponse, ProductAlpha1SessionError> {
    let (stream, _) = listener.accept().map_err(|error| transport_error(error))?;
    let mut stream = stream;
    let request = read_request(stream.try_clone().map_err(|error| transport_error(error))?)?;
    let response = evaluate_request(&expected_session, &request);
    write_response(&mut stream, &response)?;
    Ok(response)
}

fn read_request(
    stream: TcpStream,
) -> Result<ProductAlpha1TransportWireRequest, ProductAlpha1SessionError> {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .map_err(|error| transport_error(error))?;
    serde_json::from_str(&buffer).map_err(|error| ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Transport,
        path: PathBuf::from("<product-alpha1-local-transport>"),
        detail: format!("parse product alpha-1 transport request: {error}"),
    })
}

fn send_request(
    mut stream: TcpStream,
    request: &ProductAlpha1TransportWireRequest,
) -> Result<ProductAlpha1TransportWireResponse, ProductAlpha1SessionError> {
    let encoded = serde_json::to_string(request).map_err(|error| ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Serialize,
        path: PathBuf::from("<product-alpha1-local-transport>"),
        detail: format!("encode product alpha-1 transport request: {error}"),
    })?;
    stream
        .write_all(encoded.as_bytes())
        .map_err(|error| transport_error(error))?;
    stream
        .write_all(b"\n")
        .map_err(|error| transport_error(error))?;
    stream.flush().map_err(|error| transport_error(error))?;
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    reader
        .read_line(&mut buffer)
        .map_err(|error| transport_error(error))?;
    serde_json::from_str(&buffer).map_err(|error| ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Transport,
        path: PathBuf::from("<product-alpha1-local-transport>"),
        detail: format!("parse product alpha-1 transport response: {error}"),
    })
}

fn write_response(
    stream: &mut TcpStream,
    response: &ProductAlpha1TransportWireResponse,
) -> Result<(), ProductAlpha1SessionError> {
    let encoded = serde_json::to_string(response).map_err(|error| ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Serialize,
        path: PathBuf::from("<product-alpha1-local-transport>"),
        detail: format!("encode product alpha-1 transport response: {error}"),
    })?;
    stream
        .write_all(encoded.as_bytes())
        .map_err(|error| transport_error(error))?;
    stream
        .write_all(b"\n")
        .map_err(|error| transport_error(error))?;
    stream.flush().map_err(|error| transport_error(error))
}

fn evaluate_request(
    expected_session: &ProductAlpha1SessionCarrier,
    request: &ProductAlpha1TransportWireRequest,
) -> ProductAlpha1TransportWireResponse {
    let mut rejection_reason_refs = Vec::new();
    if request.session_id != expected_session.session_id {
        rejection_reason_refs.push("session_id_mismatch".to_string());
    }
    if request.package_id != expected_session.package_id {
        rejection_reason_refs.push("package_id_mismatch".to_string());
    }
    if request.membership_epoch != expected_session.membership.membership_epoch {
        rejection_reason_refs.push("membership_epoch_mismatch".to_string());
    }
    if !request.auth_lane_present {
        rejection_reason_refs.push("missing_auth_lane".to_string());
    }
    if request.auth_decision_ref != latest_auth_decision_ref(expected_session) {
        rejection_reason_refs.push("auth_decision_ref_mismatch".to_string());
    }
    if !request.capability_lane_present {
        rejection_reason_refs.push("missing_capability_lane".to_string());
    }
    if request.capability_decision_ref != latest_capability_decision_ref(expected_session) {
        rejection_reason_refs.push("capability_decision_ref_mismatch".to_string());
    }
    if request.capability_requirement_count != latest_capability_requirement_count(expected_session)
    {
        rejection_reason_refs.push("capability_requirement_count_mismatch".to_string());
    }
    if !request.witness_lane_present {
        rejection_reason_refs.push("missing_witness_lane".to_string());
    }
    if request.observer_safe_witness_relation_refs
        != expected_session.witness_state.observer_safe_relation_refs
    {
        rejection_reason_refs.push("witness_relation_ref_mismatch".to_string());
    }
    if request.witness_requirement_count != latest_witness_requirement_count(expected_session) {
        rejection_reason_refs.push("witness_requirement_count_mismatch".to_string());
    }
    let accepted = rejection_reason_refs.is_empty();
    ProductAlpha1TransportWireResponse {
        session_id: request.session_id.clone(),
        package_id: request.package_id.clone(),
        envelope_id: request.envelope_id.clone(),
        terminal_outcome: if accepted { "accepted" } else { "rejected" }.to_string(),
        rejection_reason_refs,
        transport_lane_preserved: true,
        auth_lane_preserved: request.auth_lane_present,
        membership_lane_preserved: request.membership_epoch
            == expected_session.membership.membership_epoch,
        capability_lane_preserved: request.capability_lane_present,
        witness_lane_preserved: request.witness_lane_present,
        auth_decision_ref: request.auth_decision_ref.clone(),
        capability_decision_ref: request.capability_decision_ref.clone(),
        observer_safe_witness_relation_refs: request.observer_safe_witness_relation_refs.clone(),
        capability_requirement_count: request.capability_requirement_count,
        witness_requirement_count: request.witness_requirement_count,
    }
}

fn build_wire_request(
    session: &ProductAlpha1SessionCarrier,
    envelope_id: String,
) -> ProductAlpha1TransportWireRequest {
    ProductAlpha1TransportWireRequest {
        session_id: session.session_id.clone(),
        package_id: session.package_id.clone(),
        from_place: session.runtime_plan.entry_place.clone(),
        to_place: session.runtime_plan.entry_place.clone(),
        envelope_id,
        membership_epoch: session.membership.membership_epoch,
        auth_lane_present: !session.auth_decisions.is_empty(),
        capability_lane_present: !session.capability_decisions.is_empty(),
        witness_lane_present: !session.witness_state.witness_refs.is_empty(),
        auth_decision_ref: latest_auth_decision_ref(session),
        capability_decision_ref: latest_capability_decision_ref(session),
        observer_safe_witness_relation_refs: session
            .witness_state
            .observer_safe_relation_refs
            .clone(),
        capability_requirement_count: latest_capability_requirement_count(session),
        witness_requirement_count: latest_witness_requirement_count(session),
    }
}

fn endpoint_report(
    role: &str,
    session: &ProductAlpha1SessionCarrier,
    endpoint: &str,
    response: ProductAlpha1TransportWireResponse,
) -> ProductAlpha1TransportEndpointReport {
    ProductAlpha1TransportEndpointReport {
        surface_kind: transport_endpoint_surface_kind(),
        role: role.to_string(),
        session_id: session.session_id.clone(),
        package_id: session.package_id.clone(),
        endpoint: endpoint.to_string(),
        transport_medium: "docker_compose_tcp".to_string(),
        transport_lane: "product_docker_compose_tcp".to_string(),
        terminal_outcome: response.terminal_outcome,
        rejection_reason_refs: response.rejection_reason_refs,
        auth_lane_preserved: response.auth_lane_preserved,
        membership_lane_preserved: response.membership_lane_preserved,
        capability_lane_preserved: response.capability_lane_preserved,
        witness_lane_preserved: response.witness_lane_preserved,
        auth_decision_ref: response.auth_decision_ref,
        capability_decision_ref: response.capability_decision_ref,
        observer_safe_witness_relation_refs: response.observer_safe_witness_relation_refs,
        capability_requirement_count: response.capability_requirement_count,
        witness_requirement_count: response.witness_requirement_count,
        host_cli_fixture_execution: true,
        package_native_execution_claimed: false,
        wan_federation_claimed: false,
        final_public_api_frozen: false,
    }
}

fn latest_auth_decision_ref(session: &ProductAlpha1SessionCarrier) -> String {
    session
        .auth_decisions
        .last()
        .map(|decision| decision.decision_id.clone())
        .unwrap_or_else(|| "auth_decision#absent".to_string())
}

fn latest_capability_decision_ref(session: &ProductAlpha1SessionCarrier) -> String {
    session
        .capability_decisions
        .last()
        .map(|decision| decision.decision_id.clone())
        .unwrap_or_else(|| "capability_decision#absent".to_string())
}

fn latest_capability_requirement_count(session: &ProductAlpha1SessionCarrier) -> usize {
    session
        .capability_decisions
        .last()
        .map(|decision| decision.requested_capabilities.len())
        .unwrap_or(0)
}

fn latest_witness_requirement_count(session: &ProductAlpha1SessionCarrier) -> usize {
    session
        .auth_decisions
        .last()
        .map(|decision| decision.witness_refs.len())
        .unwrap_or(0)
}

fn write_endpoint_report(
    path: &Path,
    report: &ProductAlpha1TransportEndpointReport,
) -> Result<(), ProductAlpha1SessionError> {
    let text = serde_json::to_string_pretty(report).map_err(|error| ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Serialize,
        path: path.to_path_buf(),
        detail: error.to_string(),
    })?;
    fs::write(path, format!("{text}\n")).map_err(|error| ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Transport,
        path: path.to_path_buf(),
        detail: error.to_string(),
    })
}

fn connect_with_retry(addr: &str) -> Result<TcpStream, ProductAlpha1SessionError> {
    let mut last_error = None;
    for _ in 0..40 {
        match TcpStream::connect(addr) {
            Ok(stream) => return Ok(stream),
            Err(error) => {
                last_error = Some(error.to_string());
                thread::sleep(Duration::from_millis(25));
            }
        }
    }
    Err(ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Transport,
        path: PathBuf::from("<product-alpha1-local-transport>"),
        detail: format!(
            "connect to product alpha-1 local transport `{addr}` failed: {}",
            last_error.unwrap_or_else(|| "unknown error".to_string())
        ),
    })
}

fn transport_error(error: std::io::Error) -> ProductAlpha1SessionError {
    ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Transport,
        path: PathBuf::from("<product-alpha1-local-transport>"),
        detail: error.to_string(),
    }
}

fn append_transport_events(
    session: &mut ProductAlpha1SessionCarrier,
    send_event_id: &str,
    receive_event_id: &str,
    envelope_id: &str,
    transport_medium: &str,
) {
    if let Some(previous) = session.event_dag.nodes.last() {
        session.event_dag.edges.push(ProductAlpha1EventEdge {
            from_event: previous.event_id.clone(),
            to_event: send_event_id.to_string(),
            relation: "same_session_transport_order".to_string(),
        });
    }
    session.event_dag.nodes.push(ProductAlpha1EventNode {
        event_id: send_event_id.to_string(),
        event_kind: "transport_send".to_string(),
        place_ref: session.runtime_plan.entry_place.clone(),
        envelope_ref: Some(envelope_id.to_string()),
        observer_safe_summary: format!("product alpha transport send over {transport_medium}"),
    });
    session.event_dag.edges.push(ProductAlpha1EventEdge {
        from_event: send_event_id.to_string(),
        to_event: receive_event_id.to_string(),
        relation: "transport_send_receive_order".to_string(),
    });
    session.event_dag.nodes.push(ProductAlpha1EventNode {
        event_id: receive_event_id.to_string(),
        event_kind: "transport_receive".to_string(),
        place_ref: session.runtime_plan.entry_place.clone(),
        envelope_ref: Some(envelope_id.to_string()),
        observer_safe_summary: format!("product alpha transport receive over {transport_medium}"),
    });
}

fn refresh_observer_safe_transport_export(session: &mut ProductAlpha1SessionCarrier) {
    let existing = session.observer_safe_export.clone();
    session.observer_safe_export = ProductAlpha1ObserverSafeExport {
        visible_event_ids: session
            .event_dag
            .nodes
            .iter()
            .map(|node| node.event_id.clone())
            .collect(),
        visible_routes: session
            .route_graph
            .routes
            .iter()
            .map(|route| route.route_id.clone())
            .collect(),
        ..existing
    };
}

fn unique_event_id(session: &ProductAlpha1SessionCarrier, prefix: &str) -> String {
    let existing = session
        .event_dag
        .nodes
        .iter()
        .map(|node| node.event_id.clone())
        .collect::<BTreeSet<_>>();
    unique_id(&existing, prefix)
}

fn unique_route_id(session: &ProductAlpha1SessionCarrier, prefix: &str) -> String {
    let existing = session
        .route_graph
        .routes
        .iter()
        .map(|route| route.route_id.clone())
        .collect::<BTreeSet<_>>();
    unique_id(&existing, prefix)
}

fn unique_id(existing: &BTreeSet<String>, prefix: &str) -> String {
    let mut index = existing.len() + 1;
    loop {
        let candidate = format!("{prefix}#{index}");
        if !existing.contains(&candidate) {
            return candidate;
        }
        index += 1;
    }
}

fn transport_surface_kind() -> String {
    PRODUCT_ALPHA1_TRANSPORT_SURFACE_KIND.to_string()
}

fn transport_endpoint_surface_kind() -> String {
    "product_alpha1_transport_endpoint_report".to_string()
}
