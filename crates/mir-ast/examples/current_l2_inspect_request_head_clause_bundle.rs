use std::{env, fs, process};

use mir_ast::current_l2::{
    inspect_stage3_request_head_clause_bundle, parse_stage3_request_head_clause_bundle_text,
    render_current_l2_request_head_clause_bundle_inspection_json,
    render_current_l2_request_head_clause_bundle_inspection_pretty,
};

#[derive(Clone, Copy)]
enum OutputFormat {
    Pretty,
    Json,
}

fn usage(program: &str) -> String {
    format!(
        "Usage: cargo run -q -p mir-ast --example {program} -- <path> [--format pretty|json]"
    )
}

fn parse_args() -> Result<(String, OutputFormat), String> {
    let mut args = env::args().skip(1);
    let program = "current_l2_inspect_request_head_clause_bundle";
    let mut path: Option<String> = None;
    let mut format = OutputFormat::Pretty;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--format" => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("missing value after `--format`\n{}", usage(program)))?;
                format = match value.as_str() {
                    "pretty" => OutputFormat::Pretty,
                    "json" => OutputFormat::Json,
                    _ => {
                        return Err(format!(
                            "unsupported format `{value}`\n{}",
                            usage(program)
                        ));
                    }
                };
            }
            _ if path.is_none() => path = Some(arg),
            _ => {
                return Err(format!(
                    "unexpected argument `{arg}`\n{}",
                    usage(program)
                ));
            }
        }
    }

    let path = path.ok_or_else(|| format!("missing companion sample path\n{}", usage(program)))?;

    Ok((path, format))
}

fn run() -> Result<(), String> {
    let (path, format) = parse_args()?;
    let source =
        fs::read_to_string(&path).map_err(|error| format!("failed to read `{path}`: {error}"))?;
    let bundle = parse_stage3_request_head_clause_bundle_text(&source)
        .map_err(|error| format!("failed to parse `{path}`: {error}"))?;
    let inspection = inspect_stage3_request_head_clause_bundle(&bundle);
    let rendered = match format {
        OutputFormat::Pretty => {
            render_current_l2_request_head_clause_bundle_inspection_pretty(&inspection)
        }
        OutputFormat::Json => {
            render_current_l2_request_head_clause_bundle_inspection_json(&inspection)
        }
    };

    println!("{rendered}");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        process::exit(1);
    }
}
