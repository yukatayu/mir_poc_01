use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process::{self, Command},
    thread,
    time::Duration,
};

use mir_ast::product_alpha1::{
    ProductAlpha1CheckReport, ProductAlpha1Error, ProductAlpha1Package,
    check_product_alpha1_package_path, load_product_alpha1_package_path,
};
use mir_runtime::{
    product_alpha1_devtools::{
        ProductAlpha1DevtoolsBundle, export_product_alpha1_devtools_for_session,
        render_product_alpha1_viewer_html, validate_product_alpha1_viewer_dir,
    },
    product_alpha1_session::{
        ProductAlpha1AttachReport, ProductAlpha1RunLocalReport, ProductAlpha1SessionCarrier,
        ProductAlpha1SessionError, ProductAlpha1SessionErrorKind,
        attach_product_alpha1_package_to_session_path, load_product_alpha1_session,
        quiescent_save_product_alpha1_session, run_product_alpha1_local_session_path,
        save_product_alpha1_session,
    },
    product_alpha1_transport::{
        ProductAlpha1TransportEndpointReport, run_product_alpha1_transport_for_session,
        run_product_alpha1_transport_participant_client, run_product_alpha1_transport_world_server,
    },
};
use serde_json::{Value, json};

const PRODUCT_ALPHA1_TRANSPORT_FIXTURE_HELPER_ENV: &str =
    "MIRROREA_PRODUCT_ALPHA1_TRANSPORT_HELPER";
const PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN_ENV: &str =
    "MIRROREA_PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN";
const PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN: &str = "product-alpha1-docker-compose-fixture-v0";

fn main() {
    let code = run(env::args().skip(1).collect());
    process::exit(code);
}

fn run(args: Vec<String>) -> i32 {
    let (format, args) = strip_format(args);
    if !matches!(format.as_str(), "json" | "pretty") {
        print_payload("json", unsupported_format_payload(&format));
        return 2;
    }

    let Some(command) = args.first().cloned() else {
        print_payload(&format, usage_payload());
        return 2;
    };
    let rest = &args[1..];

    let (payload, code) = match command.as_str() {
        "check" => handle_check(rest),
        "run-local" => handle_run_local(rest),
        "session" => handle_session(rest),
        "attach" => handle_attach(rest),
        "save" => handle_save(rest),
        "load" => handle_load(rest),
        "quiescent-save" => handle_quiescent_save(rest),
        "transport" => handle_transport(rest),
        "export-devtools" => handle_export_devtools(rest),
        "view" => handle_view(rest),
        "build-native-bundle" => handle_build_native_bundle(rest),
        "demo" => (unsupported_command_payload(&command, rest), 2),
        "__product-transport-world-server" => handle_product_transport_world_server(rest),
        "__product-transport-participant-client" => {
            handle_product_transport_participant_client(rest)
        }
        _ => (
            json!({
                "status": "unsupported",
                "command": command,
                "diagnostic_code": "unknown_command",
                "implemented": false,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ),
    };

    print_payload(&format, payload);
    code
}

fn strip_format(args: Vec<String>) -> (String, Vec<String>) {
    let mut format = "pretty".to_string();
    let mut stripped = Vec::new();
    let mut index = 0;
    while index < args.len() {
        if args[index] == "--format" {
            if let Some(value) = args.get(index + 1) {
                format = value.clone();
                index += 2;
                continue;
            }
        }
        stripped.push(args[index].clone());
        index += 1;
    }
    (format, stripped)
}

fn handle_check(args: &[String]) -> (Value, i32) {
    let Some(path) = args.first() else {
        return (
            json!({
                "status": "error",
                "command": "check",
                "diagnostic_code": "missing_package_path",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        );
    };

    if args.len() != 1 {
        return (unexpected_arguments_payload("check", &args[1..]), 2);
    }

    if Path::new(path).extension().and_then(|value| value.to_str()) == Some("mir") {
        return (
            json!({
                "status": "unsupported",
                "command": "check",
                "diagnostic_code": "direct_mir_non_goal",
                "message": "direct textual .mir input is a product alpha-1 non-goal; use versioned package.mir.json",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        );
    }

    match check_product_alpha1_package_path(path) {
        Ok(report) => (
            serde_json::to_value(report).expect("product alpha-1 check report should serialize"),
            0,
        ),
        Err(error) => (error_payload("check", error), 2),
    }
}

fn handle_run_local(args: &[String]) -> (Value, i32) {
    let Some(path) = args.first() else {
        return (
            json!({
                "status": "error",
                "command": "run-local",
                "diagnostic_code": "missing_package_path",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        );
    };

    if args.len() != 1 {
        return (unexpected_arguments_payload("run-local", &args[1..]), 2);
    }

    if is_direct_mir_path(path) {
        return (direct_mir_non_goal_payload("run-local", true), 2);
    }

    match run_product_alpha1_local_session_path(path) {
        Ok(report) => match write_session(&report.session) {
            Ok(session_path) => {
                let mut value = serde_json::to_value(report)
                    .expect("product alpha-1 run-local report should serialize");
                insert_string(
                    &mut value,
                    "session_path",
                    session_path.display().to_string(),
                );
                (value, 0)
            }
            Err(error) => (io_error_payload("run-local", error), 2),
        },
        Err(error) => (runtime_error_payload("run-local", error), 2),
    }
}

fn handle_session(args: &[String]) -> (Value, i32) {
    let Some(session_id) = args.first() else {
        return (
            json!({
                "status": "error",
                "command": "session",
                "diagnostic_code": "missing_session_id",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        );
    };

    if args.len() != 1 {
        return (unexpected_arguments_payload("session", &args[1..]), 2);
    }
    if is_direct_mir_path(session_id) {
        return (direct_mir_non_goal_payload("session", true), 2);
    }

    match read_session(session_id) {
        Ok(session) => (
            json!({
                "surface_kind": "product_alpha1_session_report",
                "session_path": session_path_for(session_id).display().to_string(),
                "session": session,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            0,
        ),
        Err(error) => (io_error_payload("session", error), 2),
    }
}

fn handle_attach(args: &[String]) -> (Value, i32) {
    let Some(session_id) = args.first() else {
        return (
            json!({
                "status": "error",
                "command": "attach",
                "diagnostic_code": "missing_session_id",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        );
    };
    let Some(path) = args.get(1) else {
        return (
            json!({
                "status": "error",
                "command": "attach",
                "diagnostic_code": "missing_package_path",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        );
    };

    if args.len() != 2 {
        return (unexpected_arguments_payload("attach", &args[2..]), 2);
    }
    if is_direct_mir_path(session_id) || is_direct_mir_path(path) {
        return (direct_mir_non_goal_payload("attach", true), 2);
    }

    let session_path = session_path_for(session_id);
    let _lock = match acquire_session_lock(&session_path) {
        Ok(lock) => lock,
        Err(error) => return (io_error_payload("attach", error), 2),
    };
    let session = match read_session_unlocked(&session_path) {
        Ok(session) => session,
        Err(error) => return (io_error_payload("attach", error), 2),
    };

    match attach_product_alpha1_package_to_session_path(&session, path) {
        Ok((next_session, report)) => match write_session_unlocked(&next_session) {
            Ok(next_session_path) => {
                let mut value = serde_json::to_value(report)
                    .expect("product alpha-1 attach report should serialize");
                insert_value(
                    &mut value,
                    "session",
                    serde_json::to_value(next_session)
                        .expect("product alpha-1 session should serialize"),
                );
                insert_string(
                    &mut value,
                    "session_path",
                    next_session_path.display().to_string(),
                );
                (value, 0)
            }
            Err(error) => (io_error_payload("attach", error), 2),
        },
        Err(error) => (runtime_error_payload("attach", error), 2),
    }
}

fn handle_save(args: &[String]) -> (Value, i32) {
    let (session_id, savepoint_id) = match parse_session_savepoint_args("save", args) {
        Ok(parsed) => parsed,
        Err(payload) => return payload,
    };
    let session_path = session_path_for(&session_id);
    let _lock = match acquire_session_lock(&session_path) {
        Ok(lock) => lock,
        Err(error) => return (io_error_payload("save", error), 2),
    };
    let session = match read_session_unlocked(&session_path) {
        Ok(session) => session,
        Err(error) => return (io_error_payload("save", error), 2),
    };

    match save_product_alpha1_session(&session, &savepoint_id) {
        Ok((next_session, report)) => write_session_report_payload("save", next_session, report),
        Err(error) => (runtime_error_payload("save", error), 2),
    }
}

fn handle_load(args: &[String]) -> (Value, i32) {
    let load_args = match parse_load_args(args) {
        Ok(parsed) => parsed,
        Err(payload) => return payload,
    };
    let session_id = match load_args.session_id {
        Some(session_id) => session_id,
        None => match find_session_id_for_savepoint(&load_args.savepoint_id) {
            Ok(session_id) => session_id,
            Err(error) => return (io_error_payload("load", error), 2),
        },
    };
    let session_path = session_path_for(&session_id);
    let _lock = match acquire_session_lock(&session_path) {
        Ok(lock) => lock,
        Err(error) => return (io_error_payload("load", error), 2),
    };
    let session = match read_session_unlocked(&session_path) {
        Ok(session) => session,
        Err(error) => return (io_error_payload("load", error), 2),
    };

    match load_product_alpha1_session(&session, &load_args.savepoint_id) {
        Ok((next_session, report)) => write_session_report_payload("load", next_session, report),
        Err(error) => (runtime_error_payload("load", error), 2),
    }
}

fn handle_quiescent_save(args: &[String]) -> (Value, i32) {
    let (session_id, savepoint_id) = match parse_session_savepoint_args("quiescent-save", args) {
        Ok(parsed) => parsed,
        Err(payload) => return payload,
    };
    let session_path = session_path_for(&session_id);
    let _lock = match acquire_session_lock(&session_path) {
        Ok(lock) => lock,
        Err(error) => return (io_error_payload("quiescent-save", error), 2),
    };
    let session = match read_session_unlocked(&session_path) {
        Ok(session) => session,
        Err(error) => return (io_error_payload("quiescent-save", error), 2),
    };

    match quiescent_save_product_alpha1_session(&session, &savepoint_id) {
        Ok((next_session, report)) => {
            let success = report.terminal_outcome == "saved";
            let (payload, _) = write_session_report_payload("quiescent-save", next_session, report);
            (payload, if success { 0 } else { 2 })
        }
        Err(error) => (runtime_error_payload("quiescent-save", error), 2),
    }
}

fn handle_transport(args: &[String]) -> (Value, i32) {
    let (session_id, mode) = match parse_transport_args(args) {
        Ok(parsed) => parsed,
        Err(payload) => return payload,
    };
    let session_path = session_path_for(&session_id);
    let _lock = match acquire_session_lock(&session_path) {
        Ok(lock) => lock,
        Err(error) => return (io_error_payload("transport", error), 2),
    };
    let session = match read_session_unlocked(&session_path) {
        Ok(session) => session,
        Err(error) => return (io_error_payload("transport", error), 2),
    };

    let docker_evidence = if mode == "docker" {
        match run_docker_compose_product_transport(&session, &session_path) {
            Ok(evidence) => Some(evidence),
            Err(error) => return (runtime_error_payload("transport", error), 2),
        }
    } else {
        None
    };
    match run_product_alpha1_transport_for_session(&session, &mode) {
        Ok((next_session, mut report)) => {
            if let Some(evidence) = docker_evidence {
                report.docker_compose_executed = true;
                report.docker_compose_execution_status =
                    "compose_tcp_roundtrip_executed".to_string();
                report.wire_roundtrip_executed = true;
                report.wire_bind_addr =
                    "docker-compose://product_alpha1_net/world:41002".to_string();
                report.wire_response_status = evidence.participant_report.terminal_outcome.clone();
                report.docker_compose_file = evidence.compose_file.display().to_string();
                report.docker_compose_output_dir = evidence.output_dir.display().to_string();
                report.docker_world_report_path = evidence.world_report_path.display().to_string();
                report.docker_participant_report_path =
                    evidence.participant_report_path.display().to_string();
                report.docker_world_terminal_outcome =
                    evidence.world_report.terminal_outcome.clone();
                report.docker_participant_terminal_outcome =
                    evidence.participant_report.terminal_outcome.clone();
                report.host_cli_fixture_execution = true;
                report.package_native_execution_claimed = false;
                report.native_execution_policy =
                    "package_native_execution_disabled; host_cli_fixture_execution_only"
                        .to_string();
            }
            write_session_report_payload("transport", next_session, report)
        }
        Err(error) => (runtime_error_payload("transport", error), 2),
    }
}

fn handle_export_devtools(args: &[String]) -> (Value, i32) {
    let (session_id, out_dir) = match parse_export_devtools_args(args) {
        Ok(parsed) => parsed,
        Err(payload) => return payload,
    };
    let session_path = session_path_for(&session_id);
    let _lock = match acquire_session_lock(&session_path) {
        Ok(lock) => lock,
        Err(error) => return (io_error_payload("export-devtools", error), 2),
    };
    let session = match read_session_unlocked(&session_path) {
        Ok(session) => session,
        Err(error) => return (io_error_payload("export-devtools", error), 2),
    };

    let export_ref = format!(
        "devtools#{}",
        stable_session_id_hash(&out_dir.display().to_string())
    );
    let (next_session, bundle) =
        match export_product_alpha1_devtools_for_session(&session, &export_ref) {
            Ok(result) => result,
            Err(error) => return (runtime_error_payload("export-devtools", error), 2),
        };
    if let Err(error) = fs::create_dir_all(&out_dir) {
        return (io_error_payload("export-devtools", error), 2);
    }
    let bundle_path = out_dir.join("bundle.json");
    let html_path = out_dir.join("index.html");
    if let Err(error) = write_json_artifact(&bundle_path, &bundle) {
        return (io_error_payload("export-devtools", error), 2);
    }
    let html = render_product_alpha1_viewer_html(&bundle);
    if let Err(error) = fs::write(&html_path, html) {
        return (io_error_payload("export-devtools", error), 2);
    }
    if let Err(error) = write_session_unlocked(&next_session) {
        return (io_error_payload("export-devtools", error), 2);
    }

    let mut value =
        serde_json::to_value(bundle).expect("product alpha-1 devtools bundle should serialize");
    insert_string(&mut value, "out_dir", out_dir.display().to_string());
    insert_string(&mut value, "bundle_path", bundle_path.display().to_string());
    insert_string(&mut value, "html_path", html_path.display().to_string());
    insert_value(
        &mut value,
        "session",
        observer_safe_session_payload(&next_session),
    );
    (value, 0)
}

fn handle_view(args: &[String]) -> (Value, i32) {
    let (path, check) = match parse_view_args(args) {
        Ok(parsed) => parsed,
        Err(payload) => return payload,
    };
    let viewer_openable = validate_product_alpha1_viewer_dir(&path);
    let html_path = path.join("index.html");
    let bundle_path = path.join("bundle.json");
    let bundle_result = read_product_alpha1_viewer_bundle(&bundle_path);
    let (bundle_valid, bundle_error, panel_ids) = match &bundle_result {
        Ok(bundle) => (
            validate_product_alpha1_viewer_bundle(bundle),
            None,
            bundle
                .panel_ids
                .iter()
                .map(|panel| Value::String(panel.clone()))
                .collect::<Vec<_>>(),
        ),
        Err(error) => (false, Some(error.clone()), Vec::new()),
    };
    let html_contains_required_panels = if html_path.is_file() {
        fs::read_to_string(&html_path)
            .map(|html| {
                required_product_alpha1_viewer_panels()
                    .iter()
                    .all(|panel_id| html.contains(panel_id))
                    && !html.contains("raw_witness_payload")
                    && !html.contains("raw_auth_evidence")
                    && !html.contains("granted_capabilities")
            })
            .unwrap_or(false)
    } else {
        false
    };
    let success = viewer_openable && (!check || (bundle_valid && html_contains_required_panels));
    (
        json!({
            "surface_kind": "product_alpha1_view_report",
            "status": if success { "accepted" } else { "error" },
            "diagnostic_code": if success { Value::Null } else { Value::String("invalid_viewer_bundle".to_string()) },
            "viewer_openable": viewer_openable,
            "check_requested": check,
            "bundle_valid": bundle_valid,
            "bundle_error": bundle_error,
            "html_contains_required_panels": html_contains_required_panels,
            "out_dir": path.display().to_string(),
            "bundle_path": bundle_path.display().to_string(),
            "html_path": html_path.display().to_string(),
            "panel_ids": panel_ids,
            "viewer_mode": "product_alpha1_nonfinal_static_html_viewer",
            "non_claims": [
                "not final public viewer API",
                "not raw admin/debug secret export"
            ],
            "product_alpha1_ready": false,
            "final_public_viewer_frozen": false,
            "final_public_api_frozen": false
        }),
        if success { 0 } else { 2 },
    )
}

fn handle_build_native_bundle(args: &[String]) -> (Value, i32) {
    let (package_path, out_dir) = match parse_build_native_bundle_args(args) {
        Ok(parsed) => parsed,
        Err(payload) => return payload,
    };
    if is_direct_mir_path(&package_path.display().to_string()) {
        return (direct_mir_non_goal_payload("build-native-bundle", true), 2);
    }

    match build_product_alpha1_native_bundle(&package_path, &out_dir) {
        Ok(payload) => (payload, 0),
        Err(payload) => payload,
    }
}

fn handle_product_transport_world_server(args: &[String]) -> (Value, i32) {
    let (session_path, endpoint, output_path, fixture_token) =
        match parse_transport_endpoint_args("__product-transport-world-server", args, "--bind") {
            Ok(parsed) => parsed,
            Err(payload) => return payload,
        };
    if let Err(payload) =
        authorize_transport_fixture_helper("__product-transport-world-server", &fixture_token)
    {
        return payload;
    }
    let session = match read_session_unlocked(&session_path) {
        Ok(session) => session,
        Err(error) => {
            return (
                io_error_payload("__product-transport-world-server", error),
                2,
            );
        }
    };
    match run_product_alpha1_transport_world_server(&session, &endpoint, output_path.as_deref()) {
        Ok(report) => (
            serde_json::to_value(&report)
                .expect("product alpha-1 transport endpoint report should serialize"),
            if report.terminal_outcome == "accepted" {
                0
            } else {
                2
            },
        ),
        Err(error) => (
            runtime_error_payload("__product-transport-world-server", error),
            2,
        ),
    }
}

fn handle_product_transport_participant_client(args: &[String]) -> (Value, i32) {
    let (session_path, endpoint, output_path, fixture_token) = match parse_transport_endpoint_args(
        "__product-transport-participant-client",
        args,
        "--addr",
    ) {
        Ok(parsed) => parsed,
        Err(payload) => return payload,
    };
    if let Err(payload) =
        authorize_transport_fixture_helper("__product-transport-participant-client", &fixture_token)
    {
        return payload;
    }
    let session = match read_session_unlocked(&session_path) {
        Ok(session) => session,
        Err(error) => {
            return (
                io_error_payload("__product-transport-participant-client", error),
                2,
            );
        }
    };
    match run_product_alpha1_transport_participant_client(
        &session,
        &endpoint,
        output_path.as_deref(),
    ) {
        Ok(report) => (
            serde_json::to_value(&report)
                .expect("product alpha-1 transport endpoint report should serialize"),
            if report.terminal_outcome == "accepted" {
                0
            } else {
                2
            },
        ),
        Err(error) => (
            runtime_error_payload("__product-transport-participant-client", error),
            2,
        ),
    }
}

#[derive(Debug, Clone)]
struct DockerComposeProductTransportEvidence {
    compose_file: PathBuf,
    output_dir: PathBuf,
    world_report_path: PathBuf,
    participant_report_path: PathBuf,
    world_report: ProductAlpha1TransportEndpointReport,
    participant_report: ProductAlpha1TransportEndpointReport,
}

fn run_docker_compose_product_transport(
    session: &ProductAlpha1SessionCarrier,
    session_path: &Path,
) -> Result<DockerComposeProductTransportEvidence, ProductAlpha1SessionError> {
    check_docker_compose_available()?;
    let compose_file = product_alpha1_docker_compose_file();
    if !compose_file.is_file() {
        return Err(transport_cli_error(
            &compose_file,
            format!(
                "product alpha-1 Docker Compose file is missing: {}",
                compose_file.display()
            ),
        ));
    }
    let binary = env::current_exe().map_err(|error| transport_cli_error(session_path, error))?;
    let output_dir = env::temp_dir().join(format!(
        "mirrorea-product-alpha1-transport-{}-{}",
        process::id(),
        stable_session_id_hash(&session_path.display().to_string())
    ));
    fs::create_dir_all(&output_dir).map_err(|error| transport_cli_error(&output_dir, error))?;
    let session_file =
        fs::canonicalize(session_path).map_err(|error| transport_cli_error(session_path, error))?;
    let compose_file =
        fs::canonicalize(compose_file).map_err(|error| transport_cli_error(session_path, error))?;
    let project_name = format!(
        "mirrorea-product-a1-{}",
        stable_session_id_hash(&output_dir.display().to_string())
    );

    let mut up = Command::new("docker");
    up.args([
        "compose",
        "-p",
        &project_name,
        "-f",
        compose_file
            .to_str()
            .ok_or_else(|| transport_cli_error(&compose_file, "non-utf8 compose path"))?,
        "up",
        "--abort-on-container-exit",
        "--exit-code-from",
        "participant",
    ]);
    up.env("MIRROREA_PRODUCT_ALPHA1_BINARY", binary);
    up.env("MIRROREA_PRODUCT_ALPHA1_SESSION_FILE", &session_file);
    up.env("MIRROREA_PRODUCT_ALPHA1_OUTPUT_DIR", &output_dir);
    up.env(PRODUCT_ALPHA1_TRANSPORT_FIXTURE_HELPER_ENV, "1");
    up.env(
        PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN_ENV,
        PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN,
    );
    let completed = up
        .output()
        .map_err(|error| transport_cli_error(session_path, error))?;

    let mut down = Command::new("docker");
    down.args([
        "compose",
        "-p",
        &project_name,
        "-f",
        compose_file
            .to_str()
            .ok_or_else(|| transport_cli_error(&compose_file, "non-utf8 compose path"))?,
        "down",
        "--remove-orphans",
        "-v",
    ]);
    down.env("MIRROREA_PRODUCT_ALPHA1_SESSION_FILE", &session_file);
    down.env("MIRROREA_PRODUCT_ALPHA1_OUTPUT_DIR", &output_dir);
    down.env(PRODUCT_ALPHA1_TRANSPORT_FIXTURE_HELPER_ENV, "1");
    down.env(
        PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN_ENV,
        PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN,
    );
    let _ = down.output();

    if !completed.status.success() {
        return Err(transport_cli_error(
            &compose_file,
            format!(
                "product alpha-1 Docker Compose transport failed: {}{}",
                String::from_utf8_lossy(&completed.stdout),
                String::from_utf8_lossy(&completed.stderr)
            ),
        ));
    }

    let world_report_path = output_dir.join("world.json");
    let participant_report_path = output_dir.join("participant.json");
    if !world_report_path.is_file() || !participant_report_path.is_file() {
        return Err(transport_cli_error(
            &output_dir,
            "product alpha-1 Docker Compose transport did not produce both endpoint reports",
        ));
    }
    let world_report: ProductAlpha1TransportEndpointReport = read_json_artifact(&world_report_path)
        .map_err(|error| transport_cli_error(&world_report_path, error))?;
    let participant_report: ProductAlpha1TransportEndpointReport =
        read_json_artifact(&participant_report_path)
            .map_err(|error| transport_cli_error(&participant_report_path, error))?;
    validate_docker_endpoint_report("world", session, &world_report, &world_report_path)?;
    validate_docker_endpoint_report(
        "participant",
        session,
        &participant_report,
        &participant_report_path,
    )?;
    Ok(DockerComposeProductTransportEvidence {
        compose_file,
        output_dir,
        world_report_path,
        participant_report_path,
        world_report,
        participant_report,
    })
}

fn check_docker_compose_available() -> Result<(), ProductAlpha1SessionError> {
    for args in [["--version"].as_slice(), ["compose", "version"].as_slice()] {
        let completed = Command::new("docker")
            .args(args)
            .output()
            .map_err(|error| transport_cli_error("<docker>", error))?;
        if !completed.status.success() {
            return Err(transport_cli_error(
                "<docker>",
                format!(
                    "docker {:?} failed: {}{}",
                    args,
                    String::from_utf8_lossy(&completed.stdout),
                    String::from_utf8_lossy(&completed.stderr)
                ),
            ));
        }
    }
    Ok(())
}

fn validate_docker_endpoint_report(
    expected_role: &str,
    session: &ProductAlpha1SessionCarrier,
    report: &ProductAlpha1TransportEndpointReport,
    path: &Path,
) -> Result<(), ProductAlpha1SessionError> {
    let accepted = report.role == expected_role
        && report.session_id == session.session_id
        && report.package_id == session.package_id
        && report.terminal_outcome == "accepted"
        && report.rejection_reason_refs.is_empty()
        && report.transport_medium == "docker_compose_tcp"
        && report.transport_lane == "product_docker_compose_tcp"
        && report.auth_lane_preserved
        && report.membership_lane_preserved
        && report.capability_lane_preserved
        && report.witness_lane_preserved
        && !report.wan_federation_claimed
        && !report.package_native_execution_claimed;
    if accepted {
        Ok(())
    } else {
        Err(transport_cli_error(
            path,
            format!("invalid product alpha-1 Docker endpoint report: {report:?}"),
        ))
    }
}

fn transport_cli_error(
    path: impl Into<PathBuf>,
    detail: impl ToString,
) -> ProductAlpha1SessionError {
    ProductAlpha1SessionError {
        kind: ProductAlpha1SessionErrorKind::Transport,
        path: path.into(),
        detail: detail.to_string(),
    }
}

fn product_alpha1_docker_compose_file() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crate should live under repo/crates/mirrorea-cli")
        .join("samples/product-alpha1/docker/docker-compose.product-alpha1.yml")
}

fn build_product_alpha1_native_bundle(
    package_path: &Path,
    out_dir: &Path,
) -> Result<Value, (Value, i32)> {
    let check_report = check_product_alpha1_package_path(package_path)
        .map_err(|error| (error_payload("build-native-bundle", error), 2))?;
    let package = load_product_alpha1_package_path(package_path)
        .map_err(|error| (error_payload("build-native-bundle", error), 2))?;
    let package_root = product_alpha1_package_root(package_path)
        .map_err(|error| (io_error_payload("build-native-bundle", error), 2))?;
    let absolute_out_dir = prepare_native_bundle_output_dir(package_root.as_path(), out_dir)?;
    let bundle_id = format!(
        "native-bundle#{}#{}",
        package.package_id,
        stable_session_id_hash(&absolute_out_dir.display().to_string())
    );
    if package.native_policy.execution_policy != "disabled" {
        return Err((
            json!({
                "status": "error",
                "command": "build-native-bundle",
                "diagnostic_code": "native_execution_policy_not_disabled",
                "message": "product alpha-1 native launch bundles require NativeExecutionPolicy = Disabled",
                "implemented": true,
                "package_native_execution_claimed": false,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    }

    fs::create_dir_all(&absolute_out_dir).map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&absolute_out_dir, error),
            ),
            2,
        )
    })?;

    let bin_dir = absolute_out_dir.join("bin");
    let packages_dir = absolute_out_dir.join("packages");
    let package_bundle_dir = packages_dir.join(sanitize_session_id(&package.package_id));
    let devtools_dir = absolute_out_dir.join("devtools");
    let reports_dir = absolute_out_dir.join("reports");
    fs::create_dir_all(&bin_dir).map_err(|error| {
        (
            io_error_payload("build-native-bundle", io_error_with_path(&bin_dir, error)),
            2,
        )
    })?;
    fs::create_dir_all(&package_bundle_dir).map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&package_bundle_dir, error),
            ),
            2,
        )
    })?;
    fs::create_dir_all(&devtools_dir).map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&devtools_dir, error),
            ),
            2,
        )
    })?;
    fs::create_dir_all(&reports_dir).map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&reports_dir, error),
            ),
            2,
        )
    })?;

    let binary_path = bin_dir.join("mirrorea-alpha");
    copy_current_cli_binary(&binary_path).map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&binary_path, error),
            ),
            2,
        )
    })?;
    let package_bundle_summary =
        copy_declared_package_bundle(&package_root, &package, &package_bundle_dir).map_err(
            |error| {
                (
                    io_error_payload(
                        "build-native-bundle",
                        io_error_with_path(&package_bundle_dir, error),
                    ),
                    2,
                )
            },
        )?;

    let run_report = run_product_alpha1_local_session_path(package_path)
        .map_err(|error| (runtime_error_payload("build-native-bundle", error), 2))?;
    let mut session = run_report.session.clone();
    let debug_layer_path = package_root.join("packages/debug-layer");
    let attach_report = if debug_layer_path.join("package.mir.json").is_file() {
        let (next_session, report) =
            attach_product_alpha1_package_to_session_path(&session, &debug_layer_path)
                .map_err(|error| (runtime_error_payload("build-native-bundle", error), 2))?;
        session = next_session;
        Some((report, session.clone()))
    } else {
        None
    };
    let (next_session, save_report) = save_product_alpha1_session(&session, "savepoint#r0-bundle")
        .map_err(|error| (runtime_error_payload("build-native-bundle", error), 2))?;
    session = next_session;
    let save_session = session.clone();
    let (next_session, quiescent_report) =
        quiescent_save_product_alpha1_session(&session, "savepoint#r2-bundle")
            .map_err(|error| (runtime_error_payload("build-native-bundle", error), 2))?;
    session = next_session;
    let quiescent_session = session.clone();
    let (next_session, transport_report) =
        run_product_alpha1_transport_for_session(&session, "local")
            .map_err(|error| (runtime_error_payload("build-native-bundle", error), 2))?;
    session = next_session;
    let transport_session = session.clone();
    let (next_session, devtools_bundle) =
        export_product_alpha1_devtools_for_session(&session, "devtools#native-bundle")
            .map_err(|error| (runtime_error_payload("build-native-bundle", error), 2))?;
    session = next_session;
    let devtools_session = session.clone();

    write_json_artifact(&reports_dir.join("check.json"), &check_report).map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&reports_dir, error),
            ),
            2,
        )
    })?;
    write_json_artifact(
        &reports_dir.join("run-local.json"),
        &run_local_bundle_report_payload(&run_report),
    )
    .map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&reports_dir, error),
            ),
            2,
        )
    })?;
    if let Some((attach_report, attach_session)) = &attach_report {
        write_json_artifact(
            &reports_dir.join("attach-debug-layer.json"),
            &attach_bundle_report_payload(attach_report, attach_session),
        )
        .map_err(|error| {
            (
                io_error_payload(
                    "build-native-bundle",
                    io_error_with_path(&reports_dir, error),
                ),
                2,
            )
        })?;
    }
    write_json_artifact(
        &reports_dir.join("save.json"),
        &session_mutation_bundle_report_payload(&save_report, &save_session),
    )
    .map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&reports_dir, error),
            ),
            2,
        )
    })?;
    write_json_artifact(
        &reports_dir.join("quiescent-save.json"),
        &session_mutation_bundle_report_payload(&quiescent_report, &quiescent_session),
    )
    .map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&reports_dir, error),
            ),
            2,
        )
    })?;
    write_json_artifact(
        &reports_dir.join("transport-local.json"),
        &session_mutation_bundle_report_payload(&transport_report, &transport_session),
    )
    .map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&reports_dir, error),
            ),
            2,
        )
    })?;
    write_json_artifact(&devtools_dir.join("bundle.json"), &devtools_bundle).map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&devtools_dir, error),
            ),
            2,
        )
    })?;
    fs::write(
        devtools_dir.join("index.html"),
        render_product_alpha1_viewer_html(&devtools_bundle),
    )
    .map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&devtools_dir, error),
            ),
            2,
        )
    })?;
    let export_report = devtools_bundle_report_payload(&devtools_bundle, &devtools_session);
    write_json_artifact(&reports_dir.join("export-devtools.json"), &export_report).map_err(
        |error| {
            (
                io_error_payload(
                    "build-native-bundle",
                    io_error_with_path(&reports_dir, error),
                ),
                2,
            )
        },
    )?;

    let run_script_path = out_dir.join("run.sh");
    fs::write(&run_script_path, native_bundle_run_script()).map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&run_script_path, error),
            ),
            2,
        )
    })?;
    make_executable(&run_script_path).map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&run_script_path, error),
            ),
            2,
        )
    })?;

    let provenance = native_bundle_provenance_payload(
        &package,
        &package_root,
        &binary_path,
        &package_bundle_dir,
    );
    write_json_artifact(&absolute_out_dir.join("provenance.json"), &provenance).map_err(
        |error| {
            (
                io_error_payload(
                    "build-native-bundle",
                    io_error_with_path(&absolute_out_dir, error),
                ),
                2,
            )
        },
    )?;

    let report_paths = native_bundle_report_paths(attach_report.is_some());
    let manifest = native_bundle_manifest_payload(
        &package,
        &check_report,
        &binary_path,
        &package_bundle_dir,
        &devtools_dir,
        &report_paths,
        &bundle_id,
        &package_bundle_summary,
    );
    write_json_artifact(&absolute_out_dir.join("manifest.json"), &manifest).map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&absolute_out_dir, error),
            ),
            2,
        )
    })?;
    write_json_artifact(
        &absolute_out_dir.join("launch.json"),
        &native_bundle_launch_payload(),
    )
    .map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&absolute_out_dir, error),
            ),
            2,
        )
    })?;
    fs::write(
        absolute_out_dir.join("README.md"),
        native_bundle_readme(&package.package_id, &package.package_version),
    )
    .map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&absolute_out_dir, error),
            ),
            2,
        )
    })?;

    let run_script_check = run_native_bundle_script(&run_script_path, "check")?;
    let run_script_view = run_native_bundle_script(&run_script_path, "view")?;
    let verification_report = native_bundle_verification_payload(
        &package,
        &manifest,
        &run_script_check,
        &run_script_view,
        &report_paths,
        &package_bundle_summary,
    );
    write_json_artifact(
        &reports_dir.join("verification-report.json"),
        &verification_report,
    )
    .map_err(|error| {
        (
            io_error_payload(
                "build-native-bundle",
                io_error_with_path(&reports_dir, error),
            ),
            2,
        )
    })?;

    Ok(json!({
        "surface_kind": "product_alpha1_native_bundle_report",
        "status": "accepted",
        "command": "build-native-bundle",
        "implemented": true,
        "bundle_schema": "mirrorea-product-alpha1-native-launch-bundle-v0",
        "bundle_id": bundle_id,
        "bundle_path": absolute_out_dir.display().to_string(),
        "manifest_path": absolute_out_dir.join("manifest.json").display().to_string(),
        "run_script_path": run_script_path.display().to_string(),
        "binary_path": binary_path.display().to_string(),
        "package_bundle_path": package_bundle_dir.display().to_string(),
        "devtools_path": devtools_dir.display().to_string(),
        "verification_report_path": reports_dir.join("verification-report.json").display().to_string(),
        "native_execution_policy": "Disabled",
        "NativeExecutionPolicy": "Disabled",
        "host_launch_bundle_claimed": true,
        "compiled_rust_cli_included": true,
        "package_native_execution_claimed": false,
        "arbitrary_native_execution_supported": false,
        "direct_mir_to_machine_code_supported": false,
        "signature_is_safety_claimed": false,
        "provenance_only": true,
        "run_script_check_included": true,
        "run_script_view_check_included": true,
        "cli_demo_command_claimed": false,
        "copied_package_paths": package_bundle_summary.copied_relative_paths,
        "copied_package_native_policies_disabled": package_bundle_summary.all_native_policies_disabled,
        "report_paths": report_paths,
        "non_claims": native_bundle_non_claims(),
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    }))
}

fn write_session_report_payload<T: serde::Serialize>(
    command: &str,
    next_session: ProductAlpha1SessionCarrier,
    report: T,
) -> (Value, i32) {
    match write_session_unlocked(&next_session) {
        Ok(session_path) => {
            let mut value = serde_json::to_value(report)
                .expect("product alpha-1 session mutation report should serialize");
            let session_payload =
                if matches!(command, "save" | "load" | "quiescent-save" | "transport") {
                    observer_safe_session_payload(&next_session)
                } else {
                    serde_json::to_value(next_session)
                        .expect("product alpha-1 session should serialize")
                };
            insert_value(&mut value, "session", session_payload);
            insert_string(
                &mut value,
                "session_path",
                session_path.display().to_string(),
            );
            (value, 0)
        }
        Err(error) => (io_error_payload(command, error), 2),
    }
}

fn observer_safe_session_payload(session: &ProductAlpha1SessionCarrier) -> Value {
    json!({
        "surface_kind": session.surface_kind.clone(),
        "session_id": session.session_id.clone(),
        "phase": session.phase.clone(),
        "package_id": session.package_id.clone(),
        "active_layers": session.active_layers.clone(),
        "save_load_state": {
            "ordinary_save_ready": session.save_load_state.ordinary_save_ready,
            "quiescent_save_ready": session.save_load_state.quiescent_save_ready,
            "local_savepoint_refs": session.save_load_state.local_savepoint_refs.clone(),
            "declared_savepoint_classes": session.save_load_state.declared_savepoint_classes.clone(),
            "required_quiescent_obligations": session.save_load_state.required_quiescent_obligations.clone(),
            "quiescence_state": {
                "sealed_place_refs": session.save_load_state.quiescence_state.sealed_place_refs.clone(),
                "post_cut_send_guard_enabled": session.save_load_state.quiescence_state.post_cut_send_guard_enabled,
                "rejected_post_cut_sends": session.save_load_state.quiescence_state.rejected_post_cut_sends.clone(),
            },
            "residual_reason": session.save_load_state.residual_reason.clone(),
        },
        "message_recovery_summary": {
            "handled_failures": session.message_recovery_state.handled_failures.clone(),
            "recovery_policy": session.message_recovery_state.recovery_policy.clone(),
            "failure_observation_count": session.message_recovery_state.failure_observations.len(),
            "runtime_recovery_claimed": session.message_recovery_state.runtime_recovery_claimed,
        },
        "observer_safe_export": session.observer_safe_export.clone(),
        "product_alpha1_ready": session.product_alpha1_ready,
        "final_public_api_frozen": session.final_public_api_frozen,
    })
}

fn parse_session_savepoint_args(
    command: &str,
    args: &[String],
) -> Result<(String, String), (Value, i32)> {
    let Some(session_id) = args.first() else {
        return Err((
            json!({
                "status": "error",
                "command": command,
                "diagnostic_code": "missing_session_id",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    };
    if is_direct_mir_path(session_id) {
        return Err((direct_mir_non_goal_payload(command, true), 2));
    }

    let mut savepoint_id = "latest".to_string();
    let mut index = 1;
    while index < args.len() {
        if args[index] == "--savepoint" {
            let Some(value) = args.get(index + 1) else {
                return Err((unexpected_arguments_payload(command, &args[index..]), 2));
            };
            savepoint_id = value.clone();
            index += 2;
        } else {
            return Err((unexpected_arguments_payload(command, &args[index..]), 2));
        }
    }

    if savepoint_id == "latest" && command != "load" {
        savepoint_id = if command == "quiescent-save" {
            "savepoint#r2-latest".to_string()
        } else {
            "savepoint#r0-latest".to_string()
        };
    }

    Ok((session_id.clone(), savepoint_id))
}

struct LoadArgs {
    session_id: Option<String>,
    savepoint_id: String,
}

fn parse_load_args(args: &[String]) -> Result<LoadArgs, (Value, i32)> {
    let Some(first) = args.first() else {
        return Err((
            json!({
                "status": "error",
                "command": "load",
                "diagnostic_code": "missing_savepoint_id",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    };
    if is_direct_mir_path(first) {
        return Err((direct_mir_non_goal_payload("load", true), 2));
    }

    let mut session_id = None;
    let mut savepoint_id = first.clone();
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--session" => {
                let Some(value) = args.get(index + 1) else {
                    return Err((unexpected_arguments_payload("load", &args[index..]), 2));
                };
                session_id = Some(value.clone());
                index += 2;
            }
            "--savepoint" => {
                let Some(value) = args.get(index + 1) else {
                    return Err((unexpected_arguments_payload("load", &args[index..]), 2));
                };
                session_id = Some(first.clone());
                savepoint_id = value.clone();
                index += 2;
            }
            _ => return Err((unexpected_arguments_payload("load", &args[index..]), 2)),
        }
    }

    Ok(LoadArgs {
        session_id,
        savepoint_id,
    })
}

fn parse_transport_args(args: &[String]) -> Result<(String, String), (Value, i32)> {
    let Some(session_id) = args.first() else {
        return Err((
            json!({
                "status": "error",
                "command": "transport",
                "diagnostic_code": "missing_session_id",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    };
    if is_direct_mir_path(session_id) {
        return Err((direct_mir_non_goal_payload("transport", true), 2));
    }

    let mut mode = "local".to_string();
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--mode" => {
                let Some(value) = args.get(index + 1) else {
                    return Err((unexpected_arguments_payload("transport", &args[index..]), 2));
                };
                mode = value.clone();
                index += 2;
            }
            _ => return Err((unexpected_arguments_payload("transport", &args[index..]), 2)),
        }
    }
    Ok((session_id.clone(), mode))
}

fn parse_export_devtools_args(args: &[String]) -> Result<(String, PathBuf), (Value, i32)> {
    let Some(session_id) = args.first() else {
        return Err((
            json!({
                "status": "error",
                "command": "export-devtools",
                "diagnostic_code": "missing_session_id",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    };
    if is_direct_mir_path(session_id) {
        return Err((direct_mir_non_goal_payload("export-devtools", true), 2));
    }

    let mut out_dir = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--out" => {
                let Some(value) = args.get(index + 1) else {
                    return Err((
                        unexpected_arguments_payload("export-devtools", &args[index..]),
                        2,
                    ));
                };
                out_dir = Some(PathBuf::from(value));
                index += 2;
            }
            _ => {
                return Err((
                    unexpected_arguments_payload("export-devtools", &args[index..]),
                    2,
                ));
            }
        }
    }
    let Some(out_dir) = out_dir else {
        return Err((
            json!({
                "status": "error",
                "command": "export-devtools",
                "diagnostic_code": "missing_out_dir",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    };
    Ok((session_id.clone(), out_dir))
}

fn parse_view_args(args: &[String]) -> Result<(PathBuf, bool), (Value, i32)> {
    let Some(path) = args.first() else {
        return Err((
            json!({
                "status": "error",
                "command": "view",
                "diagnostic_code": "missing_viewer_path",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    };
    if is_direct_mir_path(path) {
        return Err((direct_mir_non_goal_payload("view", true), 2));
    }

    let mut check = false;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--check" => {
                check = true;
                index += 1;
            }
            _ => return Err((unexpected_arguments_payload("view", &args[index..]), 2)),
        }
    }
    Ok((PathBuf::from(path), check))
}

fn parse_build_native_bundle_args(args: &[String]) -> Result<(PathBuf, PathBuf), (Value, i32)> {
    let Some(package_path) = args.first() else {
        return Err((
            json!({
                "status": "error",
                "command": "build-native-bundle",
                "diagnostic_code": "missing_package_path",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    };

    let mut out_dir = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            "--out" => {
                let Some(value) = args.get(index + 1) else {
                    return Err((
                        unexpected_arguments_payload("build-native-bundle", &args[index..]),
                        2,
                    ));
                };
                out_dir = Some(PathBuf::from(value));
                index += 2;
            }
            _ => {
                return Err((
                    unexpected_arguments_payload("build-native-bundle", &args[index..]),
                    2,
                ));
            }
        }
    }
    let Some(out_dir) = out_dir else {
        return Err((
            json!({
                "status": "error",
                "command": "build-native-bundle",
                "diagnostic_code": "missing_out_dir",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    };
    Ok((PathBuf::from(package_path), out_dir))
}

fn parse_transport_endpoint_args(
    command: &str,
    args: &[String],
    endpoint_flag: &str,
) -> Result<(PathBuf, String, Option<PathBuf>, String), (Value, i32)> {
    let Some(session_path) = args.first() else {
        return Err((
            json!({
                "status": "error",
                "command": command,
                "diagnostic_code": "missing_session_file",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    };
    let mut endpoint = None;
    let mut output_path = None;
    let mut fixture_token = None;
    let mut index = 1;
    while index < args.len() {
        match args[index].as_str() {
            flag if flag == endpoint_flag => {
                let Some(value) = args.get(index + 1) else {
                    return Err((unexpected_arguments_payload(command, &args[index..]), 2));
                };
                endpoint = Some(value.clone());
                index += 2;
            }
            "--output" => {
                let Some(value) = args.get(index + 1) else {
                    return Err((unexpected_arguments_payload(command, &args[index..]), 2));
                };
                output_path = Some(PathBuf::from(value));
                index += 2;
            }
            "--fixture-token" => {
                let Some(value) = args.get(index + 1) else {
                    return Err((unexpected_arguments_payload(command, &args[index..]), 2));
                };
                fixture_token = Some(value.clone());
                index += 2;
            }
            _ => return Err((unexpected_arguments_payload(command, &args[index..]), 2)),
        }
    }
    let Some(endpoint) = endpoint else {
        return Err((
            json!({
                "status": "error",
                "command": command,
                "diagnostic_code": "missing_endpoint",
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    };
    let Some(fixture_token) = fixture_token else {
        return Err((transport_helper_not_authorized_payload(command), 2));
    };
    Ok((
        PathBuf::from(session_path),
        endpoint,
        output_path,
        fixture_token,
    ))
}

fn authorize_transport_fixture_helper(
    command: &str,
    fixture_token: &str,
) -> Result<(), (Value, i32)> {
    let helper_enabled = env::var(PRODUCT_ALPHA1_TRANSPORT_FIXTURE_HELPER_ENV)
        .map(|value| value == "1")
        .unwrap_or(false);
    let env_token_ok = env::var(PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN_ENV)
        .map(|value| value == PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN)
        .unwrap_or(false);
    let arg_token_ok = fixture_token == PRODUCT_ALPHA1_TRANSPORT_FIXTURE_TOKEN;
    if helper_enabled && env_token_ok && arg_token_ok {
        Ok(())
    } else {
        Err((transport_helper_not_authorized_payload(command), 2))
    }
}

fn transport_helper_not_authorized_payload(command: &str) -> Value {
    json!({
        "status": "unsupported",
        "command": command,
        "diagnostic_code": "transport_helper_not_authorized",
        "implemented": false,
        "message": "product alpha-1 transport endpoint helpers are Docker Compose fixture-internal only",
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn unsupported_command_payload(command: &str, args: &[String]) -> Value {
    if args.iter().any(|arg| is_direct_mir_path(arg)) {
        return direct_mir_non_goal_payload(command, false);
    }

    json!({
        "status": "unsupported",
        "command": command,
        "diagnostic_code": "not_yet_implemented",
        "implemented": false,
        "product_alpha1_ready": false,
        "final_public_api_frozen": false,
        "message": "command is part of the alpha-stable family but is scheduled for a later product alpha-1 package"
    })
}

fn direct_mir_non_goal_payload(command: &str, implemented: bool) -> Value {
    json!({
        "status": "unsupported",
        "command": command,
        "diagnostic_code": "direct_mir_non_goal",
        "message": "direct textual .mir input is a product alpha-1 non-goal; use versioned package.mir.json",
        "implemented": implemented,
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn unexpected_arguments_payload(command: &str, args: &[String]) -> Value {
    json!({
        "status": "error",
        "command": command,
        "diagnostic_code": "unexpected_arguments",
        "unexpected_arguments": args,
        "implemented": true,
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn unsupported_format_payload(format: &str) -> Value {
    json!({
        "status": "error",
        "diagnostic_code": "unsupported_format",
        "format": format,
        "supported_formats": ["json", "pretty"],
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn error_payload(command: &str, error: ProductAlpha1Error) -> Value {
    json!({
        "status": "error",
        "command": command,
        "diagnostic_code": format!("{:?}", error.kind),
        "message": error.detail,
        "implemented": true,
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn runtime_error_payload(command: &str, error: ProductAlpha1SessionError) -> Value {
    json!({
        "status": "error",
        "command": command,
        "diagnostic_code": format!("{:?}", error.kind),
        "message": error.detail,
        "implemented": true,
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn io_error_payload(command: &str, error: io::Error) -> Value {
    json!({
        "status": "error",
        "command": command,
        "diagnostic_code": "session_store_io",
        "message": error.to_string(),
        "implemented": true,
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn usage_payload() -> Value {
    json!({
        "status": "error",
        "diagnostic_code": "missing_command",
        "commands": [
            "check",
            "run-local",
            "session",
            "attach",
            "transport",
            "save",
            "load",
            "quiescent-save",
            "export-devtools",
            "view",
            "build-native-bundle",
            "demo"
        ],
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn read_session(session_id: &str) -> Result<ProductAlpha1SessionCarrier, io::Error> {
    let path = session_path_for(session_id);
    read_session_unlocked(&path)
}

fn read_session_unlocked(path: &Path) -> Result<ProductAlpha1SessionCarrier, io::Error> {
    let text = fs::read_to_string(path)?;
    serde_json::from_str(&text).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn find_session_id_for_savepoint(savepoint_id: &str) -> Result<String, io::Error> {
    let dir = session_dir();
    let mut matches = Vec::new();
    for entry in fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("json") {
            continue;
        }
        let Ok(session) = read_session_unlocked(&path) else {
            continue;
        };
        if session
            .savepoints
            .iter()
            .any(|savepoint| savepoint.savepoint_id == savepoint_id)
            || (savepoint_id == "latest"
                && !session.save_load_state.local_savepoint_refs.is_empty())
        {
            matches.push(session.session_id);
        }
    }
    matches.sort();
    matches.dedup();
    match matches.as_slice() {
        [session_id] => Ok(session_id.clone()),
        [] => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("no product alpha-1 session contains savepoint `{savepoint_id}`"),
        )),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("savepoint `{savepoint_id}` is ambiguous across sessions; pass --session"),
        )),
    }
}

fn write_session(session: &ProductAlpha1SessionCarrier) -> Result<PathBuf, io::Error> {
    let path = session_path_for(&session.session_id);
    let _lock = acquire_session_lock(&path)?;
    write_session_unlocked(session)
}

fn write_session_unlocked(session: &ProductAlpha1SessionCarrier) -> Result<PathBuf, io::Error> {
    let path = session_path_for(&session.session_id);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let text = serde_json::to_string_pretty(session)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let tmp_path = temp_session_path(&path);
    {
        let mut file = File::create(&tmp_path)?;
        file.write_all(text.as_bytes())?;
        file.write_all(b"\n")?;
        file.sync_all()?;
    }
    fs::rename(&tmp_path, &path)?;
    if let Some(parent) = path.parent() {
        if let Ok(parent_dir) = File::open(parent) {
            let _ = parent_dir.sync_all();
        }
    }
    Ok(path)
}

fn write_json_artifact<T: serde::Serialize>(path: &Path, payload: &T) -> Result<(), io::Error> {
    let text = serde_json::to_string_pretty(payload)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    fs::write(path, format!("{text}\n"))
}

fn read_json_artifact<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, io::Error> {
    let text = fs::read_to_string(path)?;
    serde_json::from_str(&text).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn run_local_bundle_report_payload(report: &ProductAlpha1RunLocalReport) -> Value {
    let mut value =
        serde_json::to_value(report).expect("product alpha-1 run-local report should serialize");
    insert_value(
        &mut value,
        "session",
        observer_safe_session_payload(&report.session),
    );
    value
}

fn attach_bundle_report_payload(
    report: &ProductAlpha1AttachReport,
    session: &ProductAlpha1SessionCarrier,
) -> Value {
    json!({
        "surface_kind": report.surface_kind,
        "session_id": report.session_id,
        "package_id": report.package_id,
        "package_kind": report.package_kind,
        "terminal_outcome": report.terminal_outcome,
        "session_mutated": report.session_mutated,
        "request_event_id": report.request_event_id,
        "verdict_event_id": report.verdict_event_id,
        "activation_cut_ref": report.activation_cut_ref,
        "active_layers_after": report.active_layers_after,
        "active_runtime_mutated": report.active_runtime_mutated,
        "auth_decision_summary": {
            "decision_id": report.auth_decision.decision_id,
            "decision": report.auth_decision.decision,
            "auth_stack": report.auth_decision.auth_stack,
            "membership_requirement_count": report.auth_decision.membership_requirements.len(),
            "capability_requirement_count": report.auth_decision.capability_refs.len(),
            "private_witness_requirement_count": report.auth_decision.witness_refs.len(),
            "contract_update_path": report.auth_decision.contract_update_path,
            "overlay_transparency_claimed": report.auth_decision.overlay_transparency_claimed,
            "private_reason_ref_count": report.auth_decision.reason_refs.len()
        },
        "capability_decision_summary": {
            "decision_id": report.capability_decision.decision_id,
            "decision": report.capability_decision.decision,
            "requested_capability_count": report.capability_decision.requested_capabilities.len(),
            "granted_capability_count": report.capability_decision.granted_capabilities.len(),
            "missing_capability_count": report.capability_decision.missing_capabilities.len()
        },
        "session": observer_safe_session_payload(session),
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn session_mutation_bundle_report_payload<T: serde::Serialize>(
    report: &T,
    session: &ProductAlpha1SessionCarrier,
) -> Value {
    let mut value = serde_json::to_value(report)
        .expect("product alpha-1 session mutation report should serialize");
    insert_value(
        &mut value,
        "session",
        observer_safe_session_payload(session),
    );
    value
}

fn devtools_bundle_report_payload(
    bundle: &ProductAlpha1DevtoolsBundle,
    session: &ProductAlpha1SessionCarrier,
) -> Value {
    let mut value =
        serde_json::to_value(bundle).expect("product alpha-1 devtools bundle should serialize");
    insert_string(
        &mut value,
        "bundle_path",
        "devtools/bundle.json".to_string(),
    );
    insert_string(&mut value, "html_path", "devtools/index.html".to_string());
    insert_value(
        &mut value,
        "session",
        observer_safe_session_payload(session),
    );
    value
}

fn product_alpha1_package_root(path: &Path) -> Result<PathBuf, io::Error> {
    let package_file = if path.is_dir() {
        path.join("package.mir.json")
    } else {
        path.to_path_buf()
    };
    let root = package_file
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "package path has no parent"))?;
    fs::canonicalize(root)
}

#[derive(Debug, Clone)]
struct NativeBundlePackageSummary {
    copied_relative_paths: Vec<String>,
    all_native_policies_disabled: bool,
}

fn prepare_native_bundle_output_dir(
    package_root: &Path,
    out_dir: &Path,
) -> Result<PathBuf, (Value, i32)> {
    let absolute_out = absolute_output_path(out_dir).map_err(|error| {
        (
            io_error_payload("build-native-bundle", io_error_with_path(out_dir, error)),
            2,
        )
    })?;
    if absolute_out == package_root || absolute_out.starts_with(package_root) {
        return Err((
            json!({
                "status": "error",
                "command": "build-native-bundle",
                "diagnostic_code": "unsafe_output_path",
                "message": "native bundle output directory must not be inside the product package root",
                "out_dir": out_dir.display().to_string(),
                "package_root": package_root.display().to_string(),
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    }
    if absolute_out.exists() {
        let mut entries = fs::read_dir(&absolute_out).map_err(|error| {
            (
                io_error_payload(
                    "build-native-bundle",
                    io_error_with_path(&absolute_out, error),
                ),
                2,
            )
        })?;
        if entries.next().is_some() {
            return Err((
                json!({
                    "status": "error",
                    "command": "build-native-bundle",
                    "diagnostic_code": "output_dir_not_empty",
                    "message": "native bundle output directory must be empty to avoid stale package/report artifacts",
                    "out_dir": absolute_out.display().to_string(),
                    "implemented": true,
                    "product_alpha1_ready": false,
                    "final_public_api_frozen": false
                }),
                2,
            ));
        }
    }
    Ok(absolute_out)
}

fn absolute_output_path(path: &Path) -> Result<PathBuf, io::Error> {
    if path.exists() {
        return fs::canonicalize(path);
    }
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty());
    let parent = parent.unwrap_or_else(|| Path::new("."));
    let parent = fs::canonicalize(parent)?;
    let name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "native bundle output path has no final directory name",
        )
    })?;
    Ok(parent.join(name))
}

fn copy_current_cli_binary(destination: &Path) -> Result<(), io::Error> {
    let source = env::current_exe()?;
    fs::copy(source, destination)?;
    make_executable(destination)
}

fn copy_declared_package_bundle(
    package_root: &Path,
    package: &ProductAlpha1Package,
    destination: &Path,
) -> Result<NativeBundlePackageSummary, io::Error> {
    fs::create_dir_all(destination)?;
    let mut copied = Vec::new();
    copy_package_file(
        &package_root.join("package.mir.json"),
        &destination.join("package.mir.json"),
        "package.mir.json",
        &mut copied,
    )?;
    for dependency in &package.dependencies {
        let dependency_relative = safe_package_dependency_path(dependency)?;
        let dependency_package_path = package_root.join(&dependency_relative);
        let dependency = load_product_alpha1_package_path(&dependency_package_path)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
        if dependency.native_policy.execution_policy != "disabled" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "dependency `{}` has unsupported native execution policy `{}`",
                    dependency.package_id, dependency.native_policy.execution_policy
                ),
            ));
        }
        let dependency_is_dir = dependency_package_path.is_dir();
        let source_package_file = if dependency_is_dir {
            dependency_package_path.join("package.mir.json")
        } else {
            dependency_package_path
        };
        let dest_relative = if dependency_is_dir {
            dependency_relative.join("package.mir.json")
        } else {
            dependency_relative
        };
        let destination_package_file = destination.join(&dest_relative);
        copy_package_file(
            &source_package_file,
            &destination_package_file,
            &dest_relative.display().to_string(),
            &mut copied,
        )?;
    }
    copied.sort();
    copied.dedup();
    Ok(NativeBundlePackageSummary {
        copied_relative_paths: copied,
        all_native_policies_disabled: true,
    })
}

fn safe_package_dependency_path(dependency: &str) -> Result<PathBuf, io::Error> {
    let path = Path::new(dependency);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unsafe dependency path `{dependency}`"),
        ));
    }
    Ok(path.to_path_buf())
}

fn copy_package_file(
    source: &Path,
    destination: &Path,
    relative: &str,
    copied: &mut Vec<String>,
) -> Result<(), io::Error> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(source, destination)?;
    copied.push(relative.replace('\\', "/"));
    Ok(())
}

fn make_executable(path: &Path) -> Result<(), io::Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

fn io_error_with_path(path: &Path, error: io::Error) -> io::Error {
    io::Error::new(error.kind(), format!("{}: {}", path.display(), error))
}

fn native_bundle_report_paths(attach_included: bool) -> Vec<String> {
    let mut paths = vec![
        "reports/check.json".to_string(),
        "reports/run-local.json".to_string(),
        "reports/save.json".to_string(),
        "reports/quiescent-save.json".to_string(),
        "reports/transport-local.json".to_string(),
        "reports/export-devtools.json".to_string(),
        "reports/run-script-check.json".to_string(),
        "reports/run-script-view.json".to_string(),
        "reports/verification-report.json".to_string(),
    ];
    if attach_included {
        paths.insert(2, "reports/attach-debug-layer.json".to_string());
    }
    paths
}

fn native_bundle_manifest_payload(
    package: &ProductAlpha1Package,
    check_report: &ProductAlpha1CheckReport,
    binary_path: &Path,
    package_bundle_dir: &Path,
    devtools_dir: &Path,
    report_paths: &[String],
    bundle_id: &str,
    package_summary: &NativeBundlePackageSummary,
) -> Value {
    json!({
        "surface_kind": "product_alpha1_native_launch_bundle_manifest",
        "bundle_schema": "mirrorea-product-alpha1-native-launch-bundle-v0",
        "bundle_id": bundle_id,
        "built_at": "local-alpha-build",
        "runtime_binary": {
            "path": "bin/mirrorea-alpha",
            "kind": "compiled_rust_cli",
            "host_path": binary_path.display().to_string(),
            "fnv64": stable_file_hash(binary_path).unwrap_or_else(|_| "unavailable".to_string())
        },
        "package": {
            "path": format!("packages/{}", sanitize_session_id(&package.package_id)),
            "host_path": package_bundle_dir.display().to_string(),
            "package_id": package.package_id,
            "package_version": package.package_version,
            "package_kind": package.package_kind,
            "schema_version": package.schema_version,
            "checker_verdict": check_report.verdict,
            "copied_paths": package_summary.copied_relative_paths.clone(),
            "native_policies_disabled": package_summary.all_native_policies_disabled
        },
        "devtools": {
            "path": "devtools",
            "host_path": devtools_dir.display().to_string(),
            "bundle_path": "devtools/bundle.json",
            "html_path": "devtools/index.html",
            "viewer_mode": "product_alpha1_nonfinal_static_html_viewer"
        },
        "reports": report_paths.iter().map(|path| json!({"path": path})).collect::<Vec<_>>(),
        "launch": {
            "script": "run.sh",
            "metadata": "launch.json",
            "default_command": "check",
            "supported_script_commands": ["check", "view"],
            "cli_demo_command_claimed": false
        },
        "native_execution_policy": {
            "NativeExecutionPolicy": "Disabled",
            "package_native_execution_claimed": false,
            "arbitrary_native_execution_supported": false,
            "direct_mir_to_machine_code_supported": false,
            "sandboxed_native_execution_supported": false,
            "required_for_broader_mode": [
                "sandbox",
                "effect/failure containment",
                "resource limits",
                "timeout",
                "audit",
                "revocation"
            ]
        },
        "signature_policy": {
            "provenance_only": true,
            "signature_is_safety_claimed": false,
            "semantic_safety_source": "checker/runtime validation evidence, not signature metadata"
        },
        "non_claims": native_bundle_non_claims(),
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn native_bundle_launch_payload() -> Value {
    json!({
        "surface_kind": "product_alpha1_native_launch_metadata",
        "script": "run.sh",
        "default_command": "check",
        "supported_script_commands": ["check", "view"],
        "uses_bundled_cli": true,
        "uses_versioned_package_bundle": true,
        "uses_observer_safe_devtools_bundle": true,
        "cli_demo_command_claimed": false,
        "native_execution_policy": "Disabled",
        "package_native_execution_claimed": false,
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn native_bundle_provenance_payload(
    package: &ProductAlpha1Package,
    package_root: &Path,
    binary_path: &Path,
    package_bundle_dir: &Path,
) -> Value {
    json!({
        "surface_kind": "product_alpha1_native_bundle_provenance",
        "provenance_only": true,
        "signature_is_safety_claimed": false,
        "package_id": package.package_id,
        "package_version": package.package_version,
        "source_package_root": package_root.display().to_string(),
        "bundled_package_root": package_bundle_dir.display().to_string(),
        "runtime_binary_path": binary_path.display().to_string(),
        "runtime_binary_fnv64": stable_file_hash(binary_path).unwrap_or_else(|_| "unavailable".to_string()),
        "semantic_safety_source": "product alpha checker/runtime reports",
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn native_bundle_verification_payload(
    package: &ProductAlpha1Package,
    manifest: &Value,
    run_script_check: &Value,
    run_script_view: &Value,
    report_paths: &[String],
    package_summary: &NativeBundlePackageSummary,
) -> Value {
    json!({
        "surface_kind": "product_alpha1_native_bundle_verification_report",
        "status": "accepted",
        "package_id": package.package_id,
        "bundle_schema": manifest["bundle_schema"].clone(),
        "NativeExecutionPolicy": "Disabled",
        "native_execution_policy": "Disabled",
        "package_native_execution_claimed": false,
        "arbitrary_native_execution_supported": false,
        "direct_mir_to_machine_code_supported": false,
        "signature_is_safety_claimed": false,
        "provenance_only": true,
        "run_script_check_included": run_script_check["status"] == "accepted" || run_script_check["verdict"] == "accepted",
        "run_script_view_check_included": run_script_view["status"] == "accepted",
        "run_script_demo_path_included": false,
        "cli_demo_command_claimed": false,
        "release_demo_command_deferred": true,
        "manifest_carries_non_claims": true,
        "bundle_admits_only_disabled_native_policies": package_summary.all_native_policies_disabled,
        "arbitrary_native_execution_negative_probe_included": false,
        "native_policy_admission_source": "checked copied package set and build-time disabled-policy rejection",
        "report_paths": report_paths,
        "non_claims": native_bundle_non_claims(),
        "product_alpha1_ready": false,
        "final_public_api_frozen": false
    })
}

fn native_bundle_non_claims() -> Vec<&'static str> {
    vec![
        "not direct Mir-to-machine-code",
        "not arbitrary native package execution",
        "not signature-is-safety",
        "not final public CLI/API/ABI",
        "not final public viewer or telemetry service",
        "not WAN/federation",
        "not distributed durable save/load",
    ]
}

fn native_bundle_run_script() -> &'static str {
    r#"#!/usr/bin/env sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
BIN="$ROOT/bin/mirrorea-alpha"
REPORTS="$ROOT/reports"
PACKAGE_DIR=$(find "$ROOT/packages" -mindepth 1 -maxdepth 1 -type d | sort | sed -n '1p')
if [ -z "$PACKAGE_DIR" ]; then
  echo "no bundled product alpha package found" >&2
  exit 2
fi
mkdir -p "$REPORTS"
COMMAND=${1:-check}
case "$COMMAND" in
  check)
    "$BIN" check "$PACKAGE_DIR" --format json > "$REPORTS/run-script-check.json"
    cat "$REPORTS/run-script-check.json"
    ;;
  view)
    "$BIN" view "$ROOT/devtools" --check --format json > "$REPORTS/run-script-view.json"
    cat "$REPORTS/run-script-view.json"
    ;;
  *)
    echo "unsupported native bundle run command: $COMMAND" >&2
    exit 2
    ;;
esac
"#
}

fn native_bundle_readme(package_id: &str, package_version: &str) -> String {
    format!(
        "# Mirrorea Product Alpha-1 Native Host Launch Bundle\n\nPackage: `{package_id}` `{package_version}`.\n\nThis bundle contains the compiled Rust `mirrorea-alpha` CLI, the versioned product package files, observer-safe devtools viewer assets, validation reports, launch metadata, and provenance metadata.\n\nRun `./run.sh check` to validate the bundled package or `./run.sh view` to validate the bundled viewer. Inspect `devtools/index.html` locally for the static viewer and `devtools/bundle.json` for the observer-safe data bundle.\n\nNativeExecutionPolicy is `Disabled`. This is not direct Mir-to-machine-code, not arbitrary native package execution, not a signature-is-safety claim, and not the P-A1-31 CLI `demo` release walkthrough.\n"
    )
}

fn run_native_bundle_script(run_script_path: &Path, command: &str) -> Result<Value, (Value, i32)> {
    let completed = Command::new("sh")
        .arg(run_script_path)
        .arg(command)
        .output()
        .map_err(|error| {
            (
                io_error_payload(
                    "build-native-bundle",
                    io_error_with_path(run_script_path, error),
                ),
                2,
            )
        })?;
    if !completed.status.success() {
        return Err((
            json!({
                "status": "error",
                "command": "build-native-bundle",
                "diagnostic_code": "native_bundle_run_script_failed",
                "run_script_command": command,
                "stdout": String::from_utf8_lossy(&completed.stdout),
                "stderr": String::from_utf8_lossy(&completed.stderr),
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        ));
    }
    serde_json::from_slice(&completed.stdout).map_err(|error| {
        (
            json!({
                "status": "error",
                "command": "build-native-bundle",
                "diagnostic_code": "native_bundle_run_script_invalid_json",
                "run_script_command": command,
                "message": error.to_string(),
                "stdout": String::from_utf8_lossy(&completed.stdout),
                "implemented": true,
                "product_alpha1_ready": false,
                "final_public_api_frozen": false
            }),
            2,
        )
    })
}

fn stable_file_hash(path: &Path) -> Result<String, io::Error> {
    let bytes = fs::read(path)?;
    Ok(stable_bytes_hash(&bytes))
}

fn read_product_alpha1_viewer_bundle(path: &Path) -> Result<ProductAlpha1DevtoolsBundle, String> {
    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    if observer_safe_bundle_text_contains_forbidden_keys(&text) {
        return Err(
            "viewer bundle contains forbidden observer-safe raw witness/auth/capability keys"
                .to_string(),
        );
    }
    serde_json::from_str(&text).map_err(|error| error.to_string())
}

fn validate_product_alpha1_viewer_bundle(bundle: &ProductAlpha1DevtoolsBundle) -> bool {
    bundle.surface_kind == "product_alpha1_devtools_export_report"
        && bundle.viewer_mode == "product_alpha1_nonfinal_static_html_viewer"
        && bundle.observer_authority.contains("observer_safe")
        && bundle.redaction_policy == "observer_safe"
        && bundle.admin_debug_view_status == "kept_later"
        && !bundle.final_public_viewer_frozen
        && !bundle.durable_audit_claimed
        && required_product_alpha1_viewer_panels()
            .iter()
            .all(|panel| bundle.panel_ids.iter().any(|value| value == panel))
        && !viewer_bundle_contains_forbidden_observer_safe_strings(bundle)
}

fn viewer_bundle_contains_forbidden_observer_safe_strings(
    bundle: &ProductAlpha1DevtoolsBundle,
) -> bool {
    let text = match serde_json::to_string(bundle) {
        Ok(text) => text,
        Err(_) => return true,
    };
    observer_safe_bundle_text_contains_forbidden_keys(&text)
}

fn observer_safe_bundle_text_contains_forbidden_keys(text: &str) -> bool {
    [
        "raw_witness_payload",
        "raw_auth_evidence",
        "witness_refs",
        "granted_capabilities",
        "ObserveDebugSummary",
        "AttachDebugLayer",
    ]
    .iter()
    .any(|needle| text.contains(needle))
}

fn required_product_alpha1_viewer_panels() -> &'static [&'static str] {
    &[
        "product_overview",
        "place_graph",
        "event_dag",
        "message_route_graph",
        "membership_frontier_timeline",
        "witness_relation_timeline",
        "hotplug_lifecycle",
        "save_load_quiescent_timeline",
        "message_failure_recovery",
        "fallback_degradation",
        "auth_capability_decision",
        "redaction_toggle",
        "retention_trace",
    ]
}

fn session_path_for(session_id: &str) -> PathBuf {
    session_dir().join(format!(
        "{}.{}.session.json",
        sanitize_session_id(session_id),
        stable_session_id_hash(session_id)
    ))
}

fn session_dir() -> PathBuf {
    env::var_os("MIRROREA_ALPHA_SESSION_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".mirrorea-alpha").join("sessions"))
}

fn sanitize_session_id(session_id: &str) -> String {
    let sanitized: String = session_id
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '_'
            }
        })
        .collect();
    if sanitized.is_empty() {
        "session".to_string()
    } else {
        sanitized
    }
}

fn stable_session_id_hash(session_id: &str) -> String {
    stable_bytes_hash(session_id.as_bytes())
}

fn stable_bytes_hash(bytes: &[u8]) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn temp_session_path(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("session");
    path.with_file_name(format!(".{file_name}.{}.tmp", process::id()))
}

fn session_lock_path(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("session");
    path.with_file_name(format!("{file_name}.lock"))
}

fn acquire_session_lock(path: &Path) -> Result<SessionStoreLock, io::Error> {
    let lock_path = session_lock_path(path);
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent)?;
    }
    for _ in 0..100 {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
        {
            Ok(mut file) => {
                writeln!(file, "pid={}", process::id())?;
                file.sync_all()?;
                return Ok(SessionStoreLock { path: lock_path });
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(error) => return Err(error),
        }
    }
    Err(io::Error::new(
        io::ErrorKind::WouldBlock,
        format!(
            "timed out acquiring session store lock {}",
            lock_path.display()
        ),
    ))
}

struct SessionStoreLock {
    path: PathBuf,
}

impl Drop for SessionStoreLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn is_direct_mir_path(path: &str) -> bool {
    Path::new(path).extension().and_then(|value| value.to_str()) == Some("mir")
}

fn insert_string(payload: &mut Value, key: &str, value: String) {
    insert_value(payload, key, Value::String(value));
}

fn insert_value(payload: &mut Value, key: &str, value: Value) {
    payload
        .as_object_mut()
        .expect("CLI report payload should be a JSON object")
        .insert(key.to_string(), value);
}

fn print_payload(format: &str, payload: Value) {
    if format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).expect("payload should serialize")
        );
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&payload).expect("payload should serialize")
        );
    }
}
