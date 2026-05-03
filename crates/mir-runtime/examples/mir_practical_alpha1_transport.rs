use std::{env, path::PathBuf, process};

use mir_runtime::practical_alpha1_transport::{
    run_practical_alpha1_transport_participant_client_path, run_practical_alpha1_transport_path,
    run_practical_alpha1_transport_world_server_path,
};

fn usage() -> ! {
    eprintln!(
        "usage:
  cargo run -q -p mir-runtime --example mir_practical_alpha1_transport -- run <package-path>
  cargo run -q -p mir-runtime --example mir_practical_alpha1_transport -- world-server <package-path> [--bind <addr>] [--output <path>]
  cargo run -q -p mir-runtime --example mir_practical_alpha1_transport -- participant-client <package-path> [--addr <addr>] [--output <path>]"
    );
    process::exit(2);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        usage();
    };

    match command.as_str() {
        "run" => {
            let Some(package_path) = args.next() else {
                usage();
            };
            let report = run_practical_alpha1_transport_path(package_path)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        "world-server" => {
            let Some(package_path) = args.next() else {
                usage();
            };
            let mut bind = "0.0.0.0:41001".to_string();
            let mut output_path: Option<PathBuf> = None;
            let remaining: Vec<String> = args.collect();
            let mut index = 0;
            while index < remaining.len() {
                match remaining[index].as_str() {
                    "--bind" => {
                        index += 1;
                        bind = remaining.get(index).cloned().unwrap_or_else(|| usage());
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
            run_practical_alpha1_transport_world_server_path(
                package_path,
                &bind,
                output_path.as_deref(),
            )?;
        }
        "participant-client" => {
            let Some(package_path) = args.next() else {
                usage();
            };
            let mut addr = "127.0.0.1:41001".to_string();
            let mut output_path: Option<PathBuf> = None;
            let remaining: Vec<String> = args.collect();
            let mut index = 0;
            while index < remaining.len() {
                match remaining[index].as_str() {
                    "--addr" => {
                        index += 1;
                        addr = remaining.get(index).cloned().unwrap_or_else(|| usage());
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
            let report = run_practical_alpha1_transport_participant_client_path(
                package_path,
                &addr,
                output_path.as_deref(),
            )?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        _ => usage(),
    }

    Ok(())
}
