use std::fs;
use std::path::PathBuf;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3DeclGuardSlot {
    pub surface_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3DeclAdmitSlot {
    pub surface_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3ParsedOptionDecl {
    pub name: String,
    pub target: String,
    pub capability: String,
    pub decl_guard_slot: Stage3DeclGuardSlot,
    pub decl_admit_slot: Option<Stage3DeclAdmitSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3ParsedLineageAssertion {
    pub predecessor: String,
    pub successor: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3ParsedChainEdge {
    pub predecessor: String,
    pub successor: String,
    pub lineage_assertion: Option<Stage3ParsedLineageAssertion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3ParsedChainDecl {
    pub name: String,
    pub head: String,
    pub edges: Vec<Stage3ParsedChainEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3ParsedProgram {
    pub options: Vec<Stage3ParsedOptionDecl>,
    pub chains: Vec<Stage3ParsedChainDecl>,
}

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

pub fn parse_stage3_admit_slot_program_text(source: &str) -> Result<Stage3ParsedProgram, String> {
    let mut options = Vec::new();
    let mut chains = Vec::new();
    let mut active_chain: Option<(usize, String)> = None;

    for (line_no, raw_line) in source.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("option ") {
            active_chain = None;
            options.push(parse_option_decl(line).map_err(|message| {
                format!("line {}: {}", line_no + 1, message)
            })?);
            continue;
        }

        if line.starts_with("chain ") {
            let chain = parse_chain_decl(line).map_err(|message| {
                format!("line {}: {}", line_no + 1, message)
            })?;
            let head = chain.head.clone();
            chains.push(chain);
            active_chain = Some((chains.len() - 1, head));
            continue;
        }

        if line.starts_with("fallback ") {
            let (chain_index, previous) = active_chain
                .take()
                .ok_or_else(|| format!("line {}: fallback row without active chain", line_no + 1))?;
            let (edge, next_predecessor) =
                parse_fallback_edge(line, &previous).map_err(|message| {
                    format!("line {}: {}", line_no + 1, message)
                })?;
            chains[chain_index].edges.push(edge);
            active_chain = Some((chain_index, next_predecessor));
            continue;
        }

        return Err(format!("line {}: unsupported stage 3 admit-slot input `{}`", line_no + 1, line));
    }

    Ok(Stage3ParsedProgram { options, chains })
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

pub fn lower_stage3_chain_decl_to_fixture_chain(
    chain: &Stage3ParsedChainDecl,
) -> FixtureChainDecl {
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

fn parse_option_decl(line: &str) -> Result<Stage3ParsedOptionDecl, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 8
        || tokens[0] != "option"
        || tokens[2] != "on"
        || tokens[4] != "capability"
        || tokens[6] != "lease"
    {
        return Err(format!("unsupported option declaration `{line}`"));
    }

    let decl_admit_slot = match tokens.len() {
        8 => None,
        10 if tokens[8] == "admit" => Some(Stage3DeclAdmitSlot {
            surface_text: tokens[9].to_string(),
        }),
        _ => return Err(format!("unsupported option declaration `{line}`")),
    };

    Ok(Stage3ParsedOptionDecl {
        name: tokens[1].to_string(),
        target: tokens[3].to_string(),
        capability: tokens[5].to_string(),
        decl_guard_slot: Stage3DeclGuardSlot {
            surface_text: tokens[7].to_string(),
        },
        decl_admit_slot,
    })
}

fn parse_chain_decl(line: &str) -> Result<Stage3ParsedChainDecl, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() != 4 || tokens[0] != "chain" || tokens[2] != "=" {
        return Err(format!("unsupported chain declaration `{line}`"));
    }

    Ok(Stage3ParsedChainDecl {
        name: tokens[1].to_string(),
        head: tokens[3].to_string(),
        edges: Vec::new(),
    })
}

fn parse_fallback_edge(
    line: &str,
    previous: &str,
) -> Result<(Stage3ParsedChainEdge, String), String> {
    let rest = line
        .strip_prefix("fallback ")
        .ok_or_else(|| format!("unsupported fallback row `{line}`"))?;
    let (successor_part, lineage_part) = rest
        .split_once(" @ lineage(")
        .ok_or_else(|| "missing edge-local lineage metadata".to_string())?;
    let lineage_inner = lineage_part
        .strip_suffix(')')
        .ok_or_else(|| format!("unsupported lineage row `{line}`"))?;
    let (lineage_pred, lineage_succ) = lineage_inner
        .split_once(" -> ")
        .ok_or_else(|| format!("unsupported lineage row `{line}`"))?;
    let successor = successor_part.trim().to_string();

    Ok((
        Stage3ParsedChainEdge {
            predecessor: previous.to_string(),
            successor: successor.clone(),
            lineage_assertion: Some(Stage3ParsedLineageAssertion {
                predecessor: lineage_pred.trim().to_string(),
                successor: lineage_succ.trim().to_string(),
            }),
        },
        successor,
    ))
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
