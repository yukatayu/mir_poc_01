use mir_runtime::current_l2_cli::run_current_l2_operational_cli;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match run_current_l2_operational_cli(args) {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(error.exit_code());
        }
    }
}
