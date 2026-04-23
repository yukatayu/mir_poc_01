use mir_runtime::clean_near_end::{
    build_clean_near_end_closeout, build_clean_near_end_matrix, list_clean_near_end_samples,
    run_clean_near_end_family, run_clean_near_end_sample, CleanSampleFamily,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let Some(command) = args.first().map(String::as_str) else {
        return Err("usage: mir-clean-near-end list|run-sample|run-family|matrix|closeout [--format pretty|json]".into());
    };
    let format = parse_format(&args)?;
    match command {
        "list" => print_output(format, &list_clean_near_end_samples())?,
        "run-sample" => {
            let sample = args.get(1).ok_or("missing sample id/path")?;
            print_output(format, &run_clean_near_end_sample(sample)?)?;
        }
        "run-family" => {
            let family = args.get(1).ok_or("missing family")?;
            print_output(format, &run_clean_near_end_family(CleanSampleFamily::parse(family)?)?)?;
        }
        "matrix" => print_output(format, &build_clean_near_end_matrix()?)?,
        "closeout" => print_output(format, &build_clean_near_end_closeout())?,
        other => return Err(format!("unknown command `{other}`").into()),
    }
    Ok(())
}

fn parse_format(args: &[String]) -> Result<&str, Box<dyn std::error::Error>> {
    if let Some(index) = args.iter().position(|arg| arg == "--format") {
        let value = args.get(index + 1).ok_or("missing value for --format")?;
        match value.as_str() {
            "pretty" | "json" => Ok(value.as_str()),
            other => Err(format!("unsupported format `{other}`").into()),
        }
    } else {
        Ok("pretty")
    }
}

fn print_output<T: serde::Serialize>(
    format: &str,
    value: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        "pretty" | "json" => println!("{}", serde_json::to_string_pretty(value)?),
        other => return Err(format!("unsupported format `{other}`").into()),
    }
    Ok(())
}
