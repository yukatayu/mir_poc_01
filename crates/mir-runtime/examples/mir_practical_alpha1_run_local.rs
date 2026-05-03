use std::{env, process};

use mir_runtime::practical_alpha1_local_runtime::run_practical_alpha1_local_runtime_path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(package_path) = env::args().nth(1) else {
        eprintln!(
            "usage: cargo run -q -p mir-runtime --example mir_practical_alpha1_run_local -- <package-path>"
        );
        process::exit(2);
    };

    let report = run_practical_alpha1_local_runtime_path(package_path)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
