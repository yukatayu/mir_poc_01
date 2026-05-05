use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    process, thread,
    time::Duration,
};

use mir_ast::product_alpha1::{ProductAlpha1Error, check_product_alpha1_package_path};
use mir_runtime::product_alpha1_session::{
    ProductAlpha1SessionCarrier, ProductAlpha1SessionError,
    attach_product_alpha1_package_to_session_path, run_product_alpha1_local_session_path,
};
use serde_json::{Value, json};

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
        "transport"
        | "save"
        | "load"
        | "quiescent-save"
        | "export-devtools"
        | "view"
        | "build-native-bundle"
        | "demo" => (unsupported_command_payload(&command, rest), 2),
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
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in session_id.as_bytes() {
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
