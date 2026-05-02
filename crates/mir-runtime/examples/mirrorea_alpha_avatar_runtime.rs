use std::{env, process};

use mir_runtime::alpha_avatar_runtime::{build_closeout_reports, build_report_by_sample_id};

fn usage() -> ! {
    eprintln!(
        "usage:
  cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- run <AV-01|AV-02|AV-06|AV-08|AV-09|HP-11|HP-12|HP-15>
  cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout"
    );
    process::exit(2);
}

fn build_report(sample_id: &str) -> Result<String, Box<dyn std::error::Error>> {
    let value =
        serde_json::to_value(build_report_by_sample_id(sample_id).unwrap_or_else(|_| usage()))?;
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
        _ => usage(),
    }

    Ok(())
}
