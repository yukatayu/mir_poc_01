use std::{env, process};

use mir_runtime::practical_alpha1_save_load::run_practical_alpha1_save_load_path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(package_path) = env::args().nth(1) else {
        eprintln!(
            "usage: cargo run -q -p mir-runtime --example mir_practical_alpha1_save_load -- <package-path>"
        );
        process::exit(2);
    };

    let report = run_practical_alpha1_save_load_path(package_path)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
