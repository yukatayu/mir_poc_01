use std::{fs, path::PathBuf};

use mir_ast::practical_alpha1::PracticalAlpha1Package;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PracticalAlpha1FixtureWorld {
    pub id: String,
    pub entry_place: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PracticalAlpha1FixturePlace {
    pub id: String,
    pub authority: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PracticalAlpha1FixtureFallbackChain {
    pub id: String,
    pub capability: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PracticalAlpha1FixtureLayer {
    pub id: String,
    pub kind: String,
    pub attach_mode: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PracticalAlpha1FixtureManifest {
    pub version: String,
    pub requires_capabilities: Vec<String>,
    pub provided_capabilities: Vec<String>,
    pub effect_row: Vec<String>,
    pub failure_row: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PracticalAlpha1FixtureSummary {
    pub format_version: String,
    pub package_id: String,
    pub package_kind: String,
    pub world: Option<PracticalAlpha1FixtureWorld>,
    pub places: Vec<PracticalAlpha1FixturePlace>,
    pub fallback_chains: Vec<PracticalAlpha1FixtureFallbackChain>,
    pub layers: Vec<PracticalAlpha1FixtureLayer>,
    pub manifest: Option<PracticalAlpha1FixtureManifest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct PracticalAlpha1NegativeExpectation {
    pub accepted: bool,
    pub error_kind: String,
}

pub fn load_expected_front_door_summary(
    name: &str,
) -> Result<PracticalAlpha1FixtureSummary, String> {
    let path = practical_expected_root().join(name);
    let text = fs::read_to_string(&path).map_err(|error| {
        format!(
            "failed to read expected summary {}: {error}",
            path.display()
        )
    })?;
    serde_json::from_str(&text).map_err(|error| {
        format!(
            "failed to parse expected summary {}: {error}",
            path.display()
        )
    })
}

pub fn load_expected_front_door_error(
    name: &str,
) -> Result<PracticalAlpha1NegativeExpectation, String> {
    let path = practical_expected_root().join(name);
    let text = fs::read_to_string(&path).map_err(|error| {
        format!(
            "failed to read expected error summary {}: {error}",
            path.display()
        )
    })?;
    serde_json::from_str(&text).map_err(|error| {
        format!(
            "failed to parse expected error summary {}: {error}",
            path.display()
        )
    })
}

pub fn lower_package_to_fixture_summary(
    package: &PracticalAlpha1Package,
) -> PracticalAlpha1FixtureSummary {
    PracticalAlpha1FixtureSummary {
        format_version: package.format_version.clone(),
        package_id: package.package_id.clone(),
        package_kind: package.package_kind.clone(),
        world: package
            .world
            .as_ref()
            .map(|world| PracticalAlpha1FixtureWorld {
                id: world.id.clone(),
                entry_place: world.entry_place.clone(),
            }),
        places: package
            .places
            .iter()
            .map(|place| PracticalAlpha1FixturePlace {
                id: place.id.clone(),
                authority: place.authority.clone(),
            })
            .collect(),
        fallback_chains: package
            .fallback_chains
            .iter()
            .map(|chain| PracticalAlpha1FixtureFallbackChain {
                id: chain.id.clone(),
                capability: chain.capability.clone(),
                options: chain.options.clone(),
            })
            .collect(),
        layers: package
            .layers
            .iter()
            .map(|layer| PracticalAlpha1FixtureLayer {
                id: layer.id.clone(),
                kind: layer.kind.clone(),
                attach_mode: layer.attach_mode.clone(),
            })
            .collect(),
        manifest: package
            .manifest
            .as_ref()
            .map(|manifest| PracticalAlpha1FixtureManifest {
                version: manifest.version.clone(),
                requires_capabilities: manifest.requires_capabilities.clone(),
                provided_capabilities: manifest.provided_capabilities.clone(),
                effect_row: manifest.effect_row.clone(),
                failure_row: manifest.failure_row.clone(),
            }),
    }
}

pub fn practical_package_dir(name: &str) -> PathBuf {
    practical_packages_root().join(name)
}

pub fn practical_package_file(name: &str) -> PathBuf {
    practical_package_dir(name).join("package.mir.json")
}

pub fn practical_expected_file(name: &str) -> PathBuf {
    practical_expected_root().join(name)
}

pub fn practical_expected_dir() -> PathBuf {
    practical_expected_root()
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
