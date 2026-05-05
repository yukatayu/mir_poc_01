use std::{env, path::Path, process};

use mir_ast::product_alpha1::{ProductAlpha1Error, check_product_alpha1_package_path};
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
        "run-local"
        | "session"
        | "attach"
        | "transport"
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

fn unsupported_command_payload(command: &str, args: &[String]) -> Value {
    if args
        .iter()
        .any(|arg| Path::new(arg).extension().and_then(|value| value.to_str()) == Some("mir"))
    {
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
