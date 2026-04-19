use std::fs;
use std::path::PathBuf;

use mir_ast::current_l2::{Stage3ParsedChainDecl, Stage3ParsedOptionDecl};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureStructuralOptionDecl {
    pub name: String,
    pub target: String,
    pub capability: String,
    pub lease: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureLineageAssertion {
    pub predecessor: String,
    pub successor: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureChainEdge {
    pub predecessor: String,
    pub successor: String,
    pub lineage_assertion: Option<FixtureLineageAssertion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureChainDecl {
    pub name: String,
    pub head: String,
    pub edges: Vec<FixtureChainEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3FixtureStructuralSubset {
    pub options: Vec<FixtureStructuralOptionDecl>,
    pub chains: Vec<FixtureChainDecl>,
}

pub fn lower_stage3_option_decl_to_fixture_structural_option(
    option: &Stage3ParsedOptionDecl,
) -> FixtureStructuralOptionDecl {
    FixtureStructuralOptionDecl {
        name: option.name.clone(),
        target: option.target.clone(),
        capability: option.capability.clone(),
        lease: option.decl_guard_slot.surface_text.clone(),
    }
}

pub fn lower_stage3_chain_decl_to_fixture_chain(chain: &Stage3ParsedChainDecl) -> FixtureChainDecl {
    FixtureChainDecl {
        name: chain.name.clone(),
        head: chain.head.clone(),
        edges: chain
            .edges
            .iter()
            .map(|edge| FixtureChainEdge {
                predecessor: edge.predecessor.clone(),
                successor: edge.successor.clone(),
                lineage_assertion: edge.lineage_assertion.as_ref().map(|lineage| {
                    FixtureLineageAssertion {
                        predecessor: lineage.predecessor.clone(),
                        successor: lineage.successor.clone(),
                    }
                }),
            })
            .collect(),
    }
}

pub fn load_expected_fixture_structural_subset(
    name: &str,
) -> Result<Stage3FixtureStructuralSubset, String> {
    let path = fixture_path(name);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read fixture {}: {error}", path.display()))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse fixture {}: {error}", path.display()))?;

    let program = value
        .get("program")
        .ok_or_else(|| format!("fixture {} is missing `program`", path.display()))?;

    let mut subset = Stage3FixtureStructuralSubset {
        options: Vec::new(),
        chains: Vec::new(),
    };
    collect_fixture_structural_subset(program, &mut subset)?;
    Ok(subset)
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/current-l2")
        .join(name)
}

fn collect_fixture_structural_subset(
    node: &Value,
    subset: &mut Stage3FixtureStructuralSubset,
) -> Result<(), String> {
    let kind = node
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| "fixture node is missing `kind`".to_string())?;

    match kind {
        "Program" | "PlaceBlock" => {
            let body = node
                .get("body")
                .and_then(Value::as_array)
                .ok_or_else(|| format!("{kind} node is missing `body`"))?;
            for child in body {
                collect_fixture_structural_subset(child, subset)?;
            }
        }
        "OptionDecl" => {
            subset.options.push(FixtureStructuralOptionDecl {
                name: string_field(node, "name")?,
                target: string_field(node, "target")?,
                capability: string_field(node, "capability")?,
                lease: string_field(node, "lease")?,
            });
        }
        "ChainDecl" => {
            let edges = node
                .get("edges")
                .and_then(Value::as_array)
                .ok_or_else(|| "ChainDecl is missing `edges`".to_string())?;
            subset.chains.push(FixtureChainDecl {
                name: string_field(node, "name")?,
                head: string_field(node, "head")?,
                edges: edges
                    .iter()
                    .map(|edge| -> Result<FixtureChainEdge, String> {
                        Ok(FixtureChainEdge {
                            predecessor: string_field(edge, "predecessor")?,
                            successor: string_field(edge, "successor")?,
                            lineage_assertion: edge
                                .get("lineage_assertion")
                                .map(|lineage| -> Result<FixtureLineageAssertion, String> {
                                    Ok(FixtureLineageAssertion {
                                        predecessor: string_field(lineage, "predecessor")?,
                                        successor: string_field(lineage, "successor")?,
                                    })
                                })
                                .transpose()?,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            });
        }
        _ => {}
    }

    Ok(())
}

fn string_field(value: &Value, field: &str) -> Result<String, String> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| format!("fixture node is missing `{field}`"))
}
