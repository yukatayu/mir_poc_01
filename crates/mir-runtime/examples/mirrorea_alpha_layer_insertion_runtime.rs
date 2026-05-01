use std::{env, process};

use mir_runtime::alpha_layer_insertion_runtime::{
    build_auth_layer_contract_update_report, build_closeout_reports,
    build_debug_layer_attach_report, build_debug_layer_non_admin_rejection_report,
    build_declared_ratelimit_report, build_incompatible_patch_rejection_report,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scenario = env::args().nth(1).unwrap_or_else(|| "closeout".to_string());

    let output = match scenario.as_str() {
        "debug-admin" => serde_json::to_string_pretty(&build_debug_layer_attach_report()?)?,
        "debug-non-admin" => {
            serde_json::to_string_pretty(&build_debug_layer_non_admin_rejection_report()?)?
        }
        "auth-update" => serde_json::to_string_pretty(&build_auth_layer_contract_update_report()?)?,
        "rate-limit" => serde_json::to_string_pretty(&build_declared_ratelimit_report()?)?,
        "incompatible" => {
            serde_json::to_string_pretty(&build_incompatible_patch_rejection_report()?)?
        }
        "closeout" => serde_json::to_string_pretty(&build_closeout_reports()?)?,
        _ => {
            eprintln!(
                "usage: cargo run -q -p mir-runtime --example mirrorea_alpha_layer_insertion_runtime -- <debug-admin|debug-non-admin|auth-update|rate-limit|incompatible|closeout>"
            );
            process::exit(2);
        }
    };

    println!("{output}");
    Ok(())
}
