use std::{collections::BTreeMap, fs, path::PathBuf};

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1DeclGuardSlot {
    pub surface_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ParsedOptionDecl {
    pub name: String,
    pub target: String,
    pub capability: String,
    pub decl_guard_slot: Stage1DeclGuardSlot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ParsedLineageAssertion {
    pub predecessor: String,
    pub successor: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ParsedChainEdge {
    pub predecessor: String,
    pub successor: String,
    pub lineage_assertion: Option<Stage1ParsedLineageAssertion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ParsedChainDecl {
    pub name: String,
    pub head: String,
    pub edges: Vec<Stage1ParsedChainEdge>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ParsedProgram {
    pub options: Vec<Stage1ParsedOptionDecl>,
    pub chains: Vec<Stage1ParsedChainDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureOptionDecl {
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
pub struct Stage1FixtureSubset {
    pub options: Vec<FixtureOptionDecl>,
    pub chains: Vec<FixtureChainDecl>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage1ReconnectClusters {
    pub same_lineage_floor: bool,
    pub missing_option_structure_floor: bool,
    pub capability_strengthening_floor: bool,
}

pub fn parse_stage1_program_text(source: &str) -> Result<Stage1ParsedProgram, String> {
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

        return Err(format!("line {}: unsupported stage 1 input `{}`", line_no + 1, line));
    }

    Ok(Stage1ParsedProgram { options, chains })
}

pub fn lower_stage1_option_decl_to_fixture_option(
    option: &Stage1ParsedOptionDecl,
) -> FixtureOptionDecl {
    FixtureOptionDecl {
        name: option.name.clone(),
        target: option.target.clone(),
        capability: option.capability.clone(),
        lease: option.decl_guard_slot.surface_text.clone(),
    }
}

pub fn lower_stage1_chain_decl_to_fixture_chain(
    chain: &Stage1ParsedChainDecl,
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

pub fn load_expected_fixture_subset(name: &str) -> Result<Stage1FixtureSubset, String> {
    let path = fixture_path(name);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read fixture {}: {error}", path.display()))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse fixture {}: {error}", path.display()))?;

    let program = value
        .get("program")
        .ok_or_else(|| format!("fixture {} is missing `program`", path.display()))?;

    let mut subset = Stage1FixtureSubset {
        options: Vec::new(),
        chains: Vec::new(),
    };
    collect_fixture_subset(program, &mut subset)?;
    Ok(subset)
}

pub fn summarize_stage1_reconnect_clusters(
    subset: &Stage1FixtureSubset,
) -> Stage1ReconnectClusters {
    let option_map: BTreeMap<&str, &FixtureOptionDecl> = subset
        .options
        .iter()
        .map(|option| (option.name.as_str(), option))
        .collect();
    let mut summary = Stage1ReconnectClusters {
        same_lineage_floor: false,
        missing_option_structure_floor: false,
        capability_strengthening_floor: false,
    };

    for chain in &subset.chains {
        if !option_map.contains_key(chain.head.as_str()) {
            summary.missing_option_structure_floor = true;
        }

        for edge in &chain.edges {
            if edge.lineage_assertion.is_some() {
                summary.same_lineage_floor = true;
            }

            let predecessor = option_map.get(edge.predecessor.as_str()).copied();
            let successor = option_map.get(edge.successor.as_str()).copied();
            if predecessor.is_none() || successor.is_none() {
                summary.missing_option_structure_floor = true;
            }

            if let (Some(predecessor), Some(successor)) = (predecessor, successor) {
                if capability_strengthens(
                    predecessor.capability.as_str(),
                    successor.capability.as_str(),
                ) {
                    summary.capability_strengthening_floor = true;
                }
            }
        }
    }

    summary
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/current-l2")
        .join(name)
}

fn capability_strengthens(from: &str, to: &str) -> bool {
    matches!((from, to), ("read", "write"))
}

fn parse_option_decl(line: &str) -> Result<Stage1ParsedOptionDecl, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() > 8 && tokens[8] == "admit" {
        return Err("option-local admit is outside stage 1 accepted cluster".to_string());
    }
    if tokens.len() != 8
        || tokens[0] != "option"
        || tokens[2] != "on"
        || tokens[4] != "capability"
        || tokens[6] != "lease"
    {
        return Err(format!("unsupported option declaration `{line}`"));
    }

    Ok(Stage1ParsedOptionDecl {
        name: tokens[1].to_string(),
        target: tokens[3].to_string(),
        capability: tokens[5].to_string(),
        decl_guard_slot: Stage1DeclGuardSlot {
            surface_text: tokens[7].to_string(),
        },
    })
}

fn parse_chain_decl(line: &str) -> Result<Stage1ParsedChainDecl, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() != 4 || tokens[0] != "chain" || tokens[2] != "=" {
        return Err(format!("unsupported chain declaration `{line}`"));
    }

    Ok(Stage1ParsedChainDecl {
        name: tokens[1].to_string(),
        head: tokens[3].to_string(),
        edges: Vec::new(),
    })
}

fn parse_fallback_edge(
    line: &str,
    previous: &str,
) -> Result<(Stage1ParsedChainEdge, String), String> {
    let rest = line
        .strip_prefix("fallback ")
        .ok_or_else(|| format!("unsupported fallback row `{line}`"))?;
    let (successor_part, lineage_part) = rest.split_once(" @ lineage(").ok_or_else(|| {
        "missing edge-local lineage metadata".to_string()
    })?;
    let lineage_inner = lineage_part
        .strip_suffix(')')
        .ok_or_else(|| format!("unsupported lineage row `{line}`"))?;
    let (lineage_pred, lineage_succ) = lineage_inner
        .split_once(" -> ")
        .ok_or_else(|| format!("unsupported lineage row `{line}`"))?;
    let successor = successor_part.trim().to_string();

    Ok((
        Stage1ParsedChainEdge {
            predecessor: previous.to_string(),
            successor: successor.clone(),
            lineage_assertion: Some(Stage1ParsedLineageAssertion {
                predecessor: lineage_pred.trim().to_string(),
                successor: lineage_succ.trim().to_string(),
            }),
        },
        successor,
    ))
}

fn collect_fixture_subset(
    node: &Value,
    subset: &mut Stage1FixtureSubset,
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
                collect_fixture_subset(child, subset)?;
            }
        }
        "OptionDecl" => {
            subset.options.push(FixtureOptionDecl {
                name: required_string(node, "name")?,
                target: required_string(node, "target")?,
                capability: required_string(node, "capability")?,
                lease: required_string(node, "lease")?,
            });
        }
        "ChainDecl" => {
            let edges = node
                .get("edges")
                .and_then(Value::as_array)
                .ok_or_else(|| "ChainDecl node is missing `edges`".to_string())?;
            subset.chains.push(FixtureChainDecl {
                name: required_string(node, "name")?,
                head: required_string(node, "head")?,
                edges: edges
                    .iter()
                    .map(|edge| -> Result<FixtureChainEdge, String> {
                        Ok(FixtureChainEdge {
                            predecessor: required_string(edge, "predecessor")?,
                            successor: required_string(edge, "successor")?,
                            lineage_assertion: edge
                                .get("lineage_assertion")
                                .and_then(|value| {
                                    if value.is_null() {
                                        None
                                    } else {
                                        Some(value)
                                    }
                                })
                                .map(|lineage| -> Result<FixtureLineageAssertion, String> {
                                    Ok(FixtureLineageAssertion {
                                        predecessor: required_string(lineage, "predecessor")?,
                                        successor: required_string(lineage, "successor")?,
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

fn required_string(node: &Value, field: &str) -> Result<String, String> {
    node.get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("node is missing string field `{field}`"))
}
