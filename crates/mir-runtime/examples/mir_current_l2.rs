use std::{env, process};

use mir_runtime::current_l2_cli::run_current_l2_operational_cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match run_current_l2_operational_cli(args) {
        Ok(output) => {
            println!("{output}");
            Ok(())
        }
        Err(error) => {
            eprintln!("{error}");
            process::exit(error.exit_code());
        }
    }
}
