use std::fs;
use std::path::PathBuf;

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage3PredicateFragment {
    Atom { name: String },
    Call { callee: String, args: Vec<String> },
    And { terms: Vec<Stage3PredicateFragment> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Ident(String),
    And,
    LParen,
    RParen,
    Comma,
}

pub fn parse_stage3_minimal_predicate_fragment_text(
    source: &str,
) -> Result<Stage3PredicateFragment, String> {
    let tokens = tokenize(source)?;
    let mut parser = PredicateParser::new(tokens);
    let fragment = parser.parse_expression()?;

    if !parser.is_eof() {
        return Err(format!(
            "unexpected trailing token {:?} after predicate fragment",
            parser.peek()
        ));
    }

    Ok(fragment)
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

fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut chars = source.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(ch) = chars.peek().copied() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        match ch {
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            _ if is_ident_start(ch) => {
                let mut ident = String::new();
                while let Some(next) = chars.peek().copied() {
                    if is_ident_continue(next) {
                        ident.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if ident == "and" {
                    tokens.push(Token::And);
                } else {
                    tokens.push(Token::Ident(ident));
                }
            }
            _ => {
                return Err(format!("unsupported character `{ch}` in predicate fragment"));
            }
        }
    }

    Ok(tokens)
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

struct PredicateParser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl PredicateParser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn is_eof(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn bump(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.cursor).cloned();
        if token.is_some() {
            self.cursor += 1;
        }
        token
    }

    fn parse_expression(&mut self) -> Result<Stage3PredicateFragment, String> {
        let first = self.parse_term()?;
        let mut terms = vec![first];

        while matches!(self.peek(), Some(Token::And)) {
            self.bump();
            terms.push(self.parse_term()?);
        }

        if terms.len() == 1 {
            Ok(terms.pop().expect("one term should remain"))
        } else {
            Ok(Stage3PredicateFragment::And { terms })
        }
    }

    fn parse_term(&mut self) -> Result<Stage3PredicateFragment, String> {
        match self.peek() {
            Some(Token::LParen) => {
                self.bump();
                let inner = self.parse_expression()?;
                match self.bump() {
                    Some(Token::RParen) => Ok(inner),
                    other => Err(format!("expected `)` after grouped fragment, got {other:?}")),
                }
            }
            Some(Token::Ident(_)) => self.parse_atom_or_call(),
            other => Err(format!("expected predicate term, got {other:?}")),
        }
    }

    fn parse_atom_or_call(&mut self) -> Result<Stage3PredicateFragment, String> {
        let ident = match self.bump() {
            Some(Token::Ident(ident)) => ident,
            other => return Err(format!("expected identifier, got {other:?}")),
        };

        if !matches!(self.peek(), Some(Token::LParen)) {
            return Ok(Stage3PredicateFragment::Atom { name: ident });
        }

        self.bump();
        let mut args = Vec::new();
        if !matches!(self.peek(), Some(Token::RParen)) {
            loop {
                match self.bump() {
                    Some(Token::Ident(arg)) => args.push(arg),
                    other => return Err(format!("expected call argument, got {other:?}")),
                }

                match self.peek() {
                    Some(Token::Comma) => {
                        self.bump();
                    }
                    Some(Token::RParen) => break,
                    other => {
                        return Err(format!(
                            "expected `,` or `)` after call argument, got {other:?}"
                        ))
                    }
                }
            }
        }

        match self.bump() {
            Some(Token::RParen) => Ok(Stage3PredicateFragment::Call {
                callee: ident,
                args,
            }),
            other => Err(format!("expected `)` after call arguments, got {other:?}")),
        }
    }
}
