use std::{env, fs, path::Path, process};

use mir_runtime::practical_alpha05_session::{
    PracticalAlpha05SessionReport, load_practical_alpha05_session,
    observe_practical_alpha05_session, save_practical_alpha05_session,
    start_practical_alpha05_session_path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        usage();
        process::exit(2);
    };

    match command.as_str() {
        "start" => {
            let Some(package_path) = args.next() else {
                usage();
                process::exit(2);
            };
            let report = start_practical_alpha05_session_path(package_path)?;
            maybe_write_session(&report, args.next().as_deref())?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        "save" => {
            let Some(session_path) = args.next() else {
                usage();
                process::exit(2);
            };
            let Some(savepoint_id) = args.next() else {
                usage();
                process::exit(2);
            };
            let session = read_session(&session_path)?;
            let report = save_practical_alpha05_session(&session, &savepoint_id)?;
            maybe_write_session(&report, args.next().as_deref())?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        "load" => {
            let Some(session_path) = args.next() else {
                usage();
                process::exit(2);
            };
            let Some(savepoint_id) = args.next() else {
                usage();
                process::exit(2);
            };
            let session = read_session(&session_path)?;
            let report = load_practical_alpha05_session(&session, &savepoint_id)?;
            maybe_write_session(&report, args.next().as_deref())?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        "observe" => {
            let Some(session_path) = args.next() else {
                usage();
                process::exit(2);
            };
            let session = read_session(&session_path)?;
            let payload = observe_practical_alpha05_session(&session);
            println!("{}", serde_json::to_string_pretty(&payload)?);
        }
        _ => {
            usage();
            process::exit(2);
        }
    }

    Ok(())
}

fn read_session(
    path: impl AsRef<Path>,
) -> Result<PracticalAlpha05SessionReport, Box<dyn std::error::Error>> {
    let text = fs::read_to_string(path.as_ref())?;
    Ok(serde_json::from_str(&text)?)
}

fn maybe_write_session(
    report: &PracticalAlpha05SessionReport,
    output_path: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = output_path {
        fs::write(path, serde_json::to_string_pretty(report)?)?;
    }
    Ok(())
}

fn usage() {
    eprintln!(
        "usage:\n  cargo run -q -p mir-runtime --example mir_practical_alpha05_session -- start <package-path> [session-out]\n  cargo run -q -p mir-runtime --example mir_practical_alpha05_session -- save <session-path> <savepoint-id> [session-out]\n  cargo run -q -p mir-runtime --example mir_practical_alpha05_session -- load <session-path> <savepoint-id> [session-out]\n  cargo run -q -p mir-runtime --example mir_practical_alpha05_session -- observe <session-path>"
    );
}
