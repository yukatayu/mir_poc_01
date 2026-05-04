use std::{env, process};

use mir_runtime::practical_alpha1_avatar::render_practical_alpha1_avatar_preview_path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(package_path) = env::args().nth(1) else {
        eprintln!(
            "usage: cargo run -q -p mir-runtime --example mir_practical_alpha1_avatar -- <package-path>"
        );
        process::exit(2);
    };

    let report = render_practical_alpha1_avatar_preview_path(package_path)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
