use std::{env, path::PathBuf, process};

use mir_runtime::alpha_network_runtime::{
    build_auth_evidence_lane_preserved_report, build_closeout_reports,
    build_docker_two_process_envelope_report, build_missing_capability_rejection_report,
    build_missing_witness_rejection_report, build_observer_safe_route_trace_report,
    build_stale_membership_rejection_report, run_participant_client, run_world_server,
};

fn usage() -> ! {
    eprintln!(
        "usage:
  cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- run <NET-02|NET-03|NET-04|NET-05|NET-07|NET-09>
  cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- closeout
  cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- world-server [--bind <addr>] [--transport-medium <medium>] [--output <path>]
  cargo run -q -p mir-runtime --example mirrorea_alpha_network_runtime -- participant-client <NET-02|NET-03|NET-04|NET-05|NET-07|NET-09> [--addr <addr>] [--transport-medium <medium>] [--output <path>]"
    );
    process::exit(2);
}

fn build_report(sample_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value = match sample_id {
        "NET-02" => serde_json::to_value(build_docker_two_process_envelope_report()?)?,
        "NET-03" => serde_json::to_value(build_stale_membership_rejection_report()?)?,
        "NET-04" => serde_json::to_value(build_missing_capability_rejection_report()?)?,
        "NET-05" => serde_json::to_value(build_missing_witness_rejection_report()?)?,
        "NET-07" => serde_json::to_value(build_observer_safe_route_trace_report()?)?,
        "NET-09" => serde_json::to_value(build_auth_evidence_lane_preserved_report()?)?,
        _ => usage(),
    };
    Ok(serde_json::to_string_pretty(&value)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        usage();
    };

    match command.as_str() {
        "run" => {
            let Some(sample_id) = args.next() else {
                usage();
            };
            println!("{}", build_report(&sample_id)?);
        }
        "closeout" => {
            println!(
                "{}",
                serde_json::to_string_pretty(&build_closeout_reports()?)?
            );
        }
        "world-server" => {
            let mut bind = "0.0.0.0:41001".to_string();
            let mut transport_medium = "docker_bridge_tcp".to_string();
            let mut output_path: Option<PathBuf> = None;
            let remaining: Vec<String> = args.collect();
            let mut index = 0;
            while index < remaining.len() {
                match remaining[index].as_str() {
                    "--bind" => {
                        index += 1;
                        bind = remaining.get(index).cloned().unwrap_or_else(|| usage());
                    }
                    "--transport-medium" => {
                        index += 1;
                        transport_medium = remaining.get(index).cloned().unwrap_or_else(|| usage());
                    }
                    "--output" => {
                        index += 1;
                        output_path = Some(PathBuf::from(
                            remaining.get(index).cloned().unwrap_or_else(|| usage()),
                        ));
                    }
                    _ => usage(),
                }
                index += 1;
            }
            run_world_server(&bind, &transport_medium, output_path.as_deref())?;
        }
        "participant-client" => {
            let Some(sample_id) = args.next() else {
                usage();
            };
            let mut addr = "127.0.0.1:41001".to_string();
            let mut transport_medium = "docker_bridge_tcp".to_string();
            let mut output_path: Option<PathBuf> = None;
            let remaining: Vec<String> = args.collect();
            let mut index = 0;
            while index < remaining.len() {
                match remaining[index].as_str() {
                    "--addr" => {
                        index += 1;
                        addr = remaining.get(index).cloned().unwrap_or_else(|| usage());
                    }
                    "--transport-medium" => {
                        index += 1;
                        transport_medium = remaining.get(index).cloned().unwrap_or_else(|| usage());
                    }
                    "--output" => {
                        index += 1;
                        output_path = Some(PathBuf::from(
                            remaining.get(index).cloned().unwrap_or_else(|| usage()),
                        ));
                    }
                    _ => usage(),
                }
                index += 1;
            }
            let report = run_participant_client(
                &sample_id,
                &addr,
                &transport_medium,
                output_path.as_deref(),
            )?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        _ => usage(),
    }

    Ok(())
}
