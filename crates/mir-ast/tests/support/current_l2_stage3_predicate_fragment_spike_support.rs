use std::fs;
use std::path::PathBuf;

use mir_ast::current_l2::Stage3PredicateFragment;
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Stage3RequestContractSubset {
    pub require_fragment: Option<Stage3PredicateFragment>,
    pub ensure_fragment: Option<Stage3PredicateFragment>,
}

pub fn load_fixture_option_admit_fragment(
    fixture_name: &str,
    option_name: &str,
) -> Result<Stage3PredicateFragment, String> {
    let path = fixture_path(fixture_name);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read fixture {}: {error}", path.display()))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse fixture {}: {error}", path.display()))?;

    let program = value
        .get("program")
        .ok_or_else(|| format!("fixture {} is missing `program`", path.display()))?;
    let option_value = find_option_decl(program, option_name)?
        .ok_or_else(|| format!("fixture {} is missing option `{option_name}`", path.display()))?;
    let admit = option_value
        .get("admit")
        .ok_or_else(|| {
            format!(
                "fixture {} option `{option_name}` is missing `admit`",
                path.display()
            )
        })?;

    parse_fixture_predicate_fragment(admit)
}

pub fn load_fixture_request_clause_fragment(
    fixture_name: &str,
    perform_index: usize,
    clause_name: &str,
    predicate_index: usize,
) -> Result<Stage3PredicateFragment, String> {
    let path = fixture_path(fixture_name);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read fixture {}: {error}", path.display()))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse fixture {}: {error}", path.display()))?;

    let program = value
        .get("program")
        .ok_or_else(|| format!("fixture {} is missing `program`", path.display()))?;
    let mut performs = Vec::new();
    collect_performs(program, &mut performs);
    let perform = performs.get(perform_index).ok_or_else(|| {
        format!(
            "fixture {} is missing perform at index {}",
            path.display(),
            perform_index
        )
    })?;

    let clause = perform
        .get("contract")
        .and_then(|contract| contract.get(clause_name))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            format!(
                "fixture {} perform[{}] is missing contract.{}",
                path.display(),
                perform_index,
                clause_name
            )
        })?;
    let predicate = clause.get(predicate_index).ok_or_else(|| {
        format!(
            "fixture {} perform[{}].{} is missing predicate index {}",
            path.display(),
            perform_index,
            clause_name,
            predicate_index
        )
    })?;

    parse_fixture_predicate_fragment(predicate)
}

#[allow(dead_code)]
pub fn load_fixture_request_contract_subset(
    fixture_name: &str,
    perform_index: usize,
) -> Result<Stage3RequestContractSubset, String> {
    let path = fixture_path(fixture_name);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read fixture {}: {error}", path.display()))?;
    let value: Value = serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse fixture {}: {error}", path.display()))?;

    let program = value
        .get("program")
        .ok_or_else(|| format!("fixture {} is missing `program`", path.display()))?;
    let mut performs = Vec::new();
    collect_performs(program, &mut performs);
    let perform = performs.get(perform_index).ok_or_else(|| {
        format!(
            "fixture {} is missing perform at index {}",
            path.display(),
            perform_index
        )
    })?;

    Ok(Stage3RequestContractSubset {
        require_fragment: load_optional_contract_fragment(
            perform,
            path.display().to_string().as_str(),
            perform_index,
            "require",
        )?,
        ensure_fragment: load_optional_contract_fragment(
            perform,
            path.display().to_string().as_str(),
            perform_index,
            "ensure",
        )?,
    })
}

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/current-l2")
        .join(name)
}

fn find_option_decl<'a>(
    value: &'a Value,
    option_name: &str,
) -> Result<Option<&'a Value>, String> {
    match value {
        Value::Object(map) => {
            if map.get("kind").and_then(Value::as_str) == Some("OptionDecl")
                && map.get("name").and_then(Value::as_str) == Some(option_name)
            {
                return Ok(Some(value));
            }

            if let Some(body) = map.get("body").and_then(Value::as_array) {
                for item in body {
                    if let Some(found) = find_option_decl(item, option_name)? {
                        return Ok(Some(found));
                    }
                }
            }

            Ok(None)
        }
        Value::Array(items) => {
            for item in items {
                if let Some(found) = find_option_decl(item, option_name)? {
                    return Ok(Some(found));
                }
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}

fn collect_performs<'a>(value: &'a Value, performs: &mut Vec<&'a Value>) {
    match value {
        Value::Object(map) => {
            let kind = map.get("kind").and_then(Value::as_str);
            if matches!(kind, Some("PerformOn" | "PerformVia")) {
                performs.push(value);
            }

            if let Some(body) = map.get("body").and_then(Value::as_array) {
                for item in body {
                    collect_performs(item, performs);
                }
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_performs(item, performs);
            }
        }
        _ => {}
    }
}

fn parse_fixture_predicate_fragment(value: &Value) -> Result<Stage3PredicateFragment, String> {
    let kind = value
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(|| "predicate node is missing `kind`".to_string())?;

    match kind {
        "atom" => Ok(Stage3PredicateFragment::Atom {
            name: value
                .get("name")
                .and_then(Value::as_str)
                .ok_or_else(|| "atom node is missing `name`".to_string())?
                .to_string(),
        }),
        "call" => {
            let args = value
                .get("args")
                .and_then(Value::as_array)
                .ok_or_else(|| "call node is missing `args`".to_string())?
                .iter()
                .map(|arg| {
                    arg.as_str()
                        .map(ToString::to_string)
                        .ok_or_else(|| "call arg must be string".to_string())
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok(Stage3PredicateFragment::Call {
                callee: value
                    .get("callee")
                    .and_then(Value::as_str)
                    .ok_or_else(|| "call node is missing `callee`".to_string())?
                    .to_string(),
                args,
            })
        }
        "and" => {
            let terms = value
                .get("terms")
                .and_then(Value::as_array)
                .ok_or_else(|| "and node is missing `terms`".to_string())?
                .iter()
                .map(parse_fixture_predicate_fragment)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Stage3PredicateFragment::And { terms })
        }
        other => Err(format!("unsupported fixture predicate kind `{other}`")),
    }
}

#[allow(dead_code)]
fn load_optional_contract_fragment(
    perform: &Value,
    fixture_path: &str,
    perform_index: usize,
    clause_name: &str,
) -> Result<Option<Stage3PredicateFragment>, String> {
    let clause = perform
        .get("contract")
        .and_then(|contract| contract.get(clause_name))
        .and_then(Value::as_array)
        .ok_or_else(|| {
            format!(
                "fixture {} perform[{}] is missing contract.{}",
                fixture_path, perform_index, clause_name
            )
        })?;

    match clause.as_slice() {
        [] => Ok(None),
        [only] => parse_fixture_predicate_fragment(only).map(Some),
        _ => Err(format!(
            "fixture {} perform[{}].{} has more than one predicate, outside stage 3 first tranche",
            fixture_path, perform_index, clause_name
        )),
    }
}
