use std::{env, process};

use mir_runtime::alpha_local_runtime::{
    build_local_sugoroku_runtime_report, build_stale_membership_rejection_report,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scenario = env::args()
        .nth(1)
        .unwrap_or_else(|| "local-sugoroku".to_string());

    let report = match scenario.as_str() {
        "local-sugoroku" => serde_json::to_string_pretty(&build_local_sugoroku_runtime_report()?)?,
        "stale-membership" => {
            serde_json::to_string_pretty(&build_stale_membership_rejection_report()?)?
        }
        _ => {
            eprintln!(
                "usage: cargo run -q -p mir-runtime --example mirrorea_alpha_local_runtime -- <local-sugoroku|stale-membership>"
            );
            process::exit(2);
        }
    };

    println!("{report}");
    Ok(())
}
