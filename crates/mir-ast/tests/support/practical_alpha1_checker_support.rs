use std::{fs, path::PathBuf};

use mir_ast::practical_alpha1_checker::PracticalAlpha1CheckReport;

pub fn load_expected_check_report(name: &str) -> Result<PracticalAlpha1CheckReport, String> {
    let path = practical_expected_root().join(name);
    let text = fs::read_to_string(&path).map_err(|error| {
        format!(
            "failed to read expected check report {}: {error}",
            path.display()
        )
    })?;
    serde_json::from_str(&text).map_err(|error| {
        format!(
            "failed to parse expected check report {}: {error}",
            path.display()
        )
    })
}

pub fn practical_package_dir(name: &str) -> PathBuf {
    practical_packages_root().join(name)
}

fn practical_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../samples/practical-alpha1")
}

fn practical_packages_root() -> PathBuf {
    practical_root().join("packages")
}

fn practical_expected_root() -> PathBuf {
    practical_root().join("expected")
}
