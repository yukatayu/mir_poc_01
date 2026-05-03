use std::{env, process};

use mir_ast::practical_alpha1_checker::check_practical_alpha1_package_path;

fn main() {
    let mut args = env::args().skip(1);
    let path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("usage: mir_practical_alpha1_check <package-path>");
            process::exit(2);
        }
    };

    match check_practical_alpha1_package_path(path) {
        Ok(report) => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report)
                    .expect("practical alpha-1 checker report should serialize")
            );
        }
        Err(error) => {
            eprintln!("{error}");
            process::exit(2);
        }
    }
}
