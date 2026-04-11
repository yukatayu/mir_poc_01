//! Non-production current L2 parser carrier floor.
//!
//! This module intentionally exposes only the stage 1 declaration/lineage subset and
//! the stage 2 try/fallback structural subset that were frozen in the Phase 3/6 docs.

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
    pub lineage_assertion: Stage1ParsedLineageAssertion,
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
        Stage1ParsedChainEdge {
            predecessor: previous.to_string(),
            successor: successor.clone(),
            lineage_assertion: Stage1ParsedLineageAssertion {
                predecessor: lineage_pred.trim().to_string(),
                successor: lineage_succ.trim().to_string(),
            },
        },
        successor,
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage2StatementHeadKind {
    AtomicCut,
    Other,
}

impl Stage2StatementHeadKind {
    pub fn is_atomic_cut(&self) -> bool {
        matches!(self, Self::AtomicCut)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage2ParsedTryFallback {
    body: Vec<Stage2StatementHeadKind>,
    fallback_body: Vec<Stage2StatementHeadKind>,
}

impl Stage2ParsedTryFallback {
    pub fn body(&self) -> &[Stage2StatementHeadKind] {
        &self.body
    }

    pub fn fallback_body(&self) -> &[Stage2StatementHeadKind] {
        &self.fallback_body
    }
}

pub fn parse_stage2_try_rollback_text(source: &str) -> Result<Stage2ParsedTryFallback, String> {
    let lines: Vec<&str> = source
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    if lines.is_empty() {
        return Err("stage 2 try/rollback input must not be empty".to_string());
    }

    if lines[0] != "try {" {
        return Err(format!(
            "stage 2 input must start with `try {{`, got `{}`",
            lines[0]
        ));
    }

    let mut body = Vec::new();
    let mut fallback_body = Vec::new();
    let mut in_fallback = false;
    let mut closed = false;

    for line in lines.iter().skip(1) {
        if !in_fallback {
            if *line == "} fallback {" {
                in_fallback = true;
                continue;
            }
            body.push(parse_statement_head(line)?);
            continue;
        }

        if *line == "}" {
            if closed {
                return Err(format!("unexpected content after fallback close `{line}`"));
            }
            closed = true;
            continue;
        }

        if closed {
            return Err(format!("unexpected content after fallback close `{line}`"));
        }

        fallback_body.push(parse_statement_head(line)?);
    }

    if !in_fallback {
        return Err("stage 2 input is missing `} fallback {` delimiter".to_string());
    }

    if !closed {
        return Err("stage 2 input is missing closing `}` for fallback block".to_string());
    }

    Ok(Stage2ParsedTryFallback { body, fallback_body })
}

fn parse_statement_head(line: &str) -> Result<Stage2StatementHeadKind, String> {
    if line == "atomic_cut" {
        return Ok(Stage2StatementHeadKind::AtomicCut);
    }
    if line.ends_with('{') || line == "}" || line.starts_with("fallback ") {
        return Err(format!("unsupported stage 2 statement head `{line}`"));
    }
    Ok(Stage2StatementHeadKind::Other)
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
    pub decl_guard_slot: Stage1DeclGuardSlot,
    pub decl_admit_slot: Option<Stage3DeclAdmitSlot>,
}

pub type Stage3ParsedLineageAssertion = Stage1ParsedLineageAssertion;
pub type Stage3ParsedChainEdge = Stage1ParsedChainEdge;
pub type Stage3ParsedChainDecl = Stage1ParsedChainDecl;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3ParsedProgram {
    pub options: Vec<Stage3ParsedOptionDecl>,
    pub chains: Vec<Stage3ParsedChainDecl>,
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
            options.push(parse_stage3_option_decl(line).map_err(|message| {
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

        if line.starts_with("perform ") {
            return Err(format!(
                "line {}: request head is outside stage 3 admit-slot first tranche",
                line_no + 1
            ));
        }

        if line.starts_with("require ") {
            return Err(format!(
                "line {}: request-local require clause is outside stage 3 admit-slot first tranche",
                line_no + 1
            ));
        }

        if line.starts_with("ensure ") {
            return Err(format!(
                "line {}: request-local ensure clause is outside stage 3 admit-slot first tranche",
                line_no + 1
            ));
        }

        return Err(format!(
            "line {}: unsupported stage 3 admit-slot input `{}`",
            line_no + 1,
            line
        ));
    }

    Ok(Stage3ParsedProgram { options, chains })
}

fn parse_stage3_option_decl(line: &str) -> Result<Stage3ParsedOptionDecl, String> {
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
        9 if tokens[8] == "admit" => {
            return Err("missing declaration-side admit slot payload".to_string());
        }
        10 if tokens[8] == "admit" => Some(Stage3DeclAdmitSlot {
            surface_text: tokens[9].to_string(),
        }),
        _ => return Err(format!("unsupported option declaration `{line}`")),
    };

    Ok(Stage3ParsedOptionDecl {
        name: tokens[1].to_string(),
        target: tokens[3].to_string(),
        capability: tokens[5].to_string(),
        decl_guard_slot: Stage1DeclGuardSlot {
            surface_text: tokens[7].to_string(),
        },
        decl_admit_slot,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage3PredicateFragment {
    Atom { name: String },
    Call { callee: String, args: Vec<String> },
    And { terms: Vec<Stage3PredicateFragment> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Stage3PredicateToken {
    Ident(String),
    And,
    LParen,
    RParen,
    Comma,
}

pub fn parse_stage3_minimal_predicate_fragment_text(
    source: &str,
) -> Result<Stage3PredicateFragment, String> {
    let tokens = tokenize_stage3_predicate_fragment(source)?;
    let mut parser = Stage3PredicateParser::new(tokens);
    let fragment = parser.parse_expression()?;

    if !parser.is_eof() {
        return Err(format!(
            "unexpected trailing token {:?} after predicate fragment",
            parser.peek()
        ));
    }

    Ok(fragment)
}

fn tokenize_stage3_predicate_fragment(source: &str) -> Result<Vec<Stage3PredicateToken>, String> {
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
                tokens.push(Stage3PredicateToken::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Stage3PredicateToken::RParen);
            }
            ',' => {
                chars.next();
                tokens.push(Stage3PredicateToken::Comma);
            }
            _ if is_stage3_ident_start(ch) => {
                let mut ident = String::new();
                while let Some(next) = chars.peek().copied() {
                    if is_stage3_ident_continue(next) {
                        ident.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if ident == "and" {
                    tokens.push(Stage3PredicateToken::And);
                } else {
                    tokens.push(Stage3PredicateToken::Ident(ident));
                }
            }
            _ => {
                return Err(format!("unsupported character `{ch}` in predicate fragment"));
            }
        }
    }

    Ok(tokens)
}

fn is_stage3_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_stage3_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

struct Stage3PredicateParser {
    tokens: Vec<Stage3PredicateToken>,
    cursor: usize,
}

impl Stage3PredicateParser {
    fn new(tokens: Vec<Stage3PredicateToken>) -> Self {
        Self { tokens, cursor: 0 }
    }

    fn is_eof(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Stage3PredicateToken> {
        self.tokens.get(self.cursor)
    }

    fn bump(&mut self) -> Option<Stage3PredicateToken> {
        let token = self.tokens.get(self.cursor).cloned();
        if token.is_some() {
            self.cursor += 1;
        }
        token
    }

    fn parse_expression(&mut self) -> Result<Stage3PredicateFragment, String> {
        let first = self.parse_term()?;
        let mut terms = vec![first];

        while matches!(self.peek(), Some(Stage3PredicateToken::And)) {
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
            Some(Stage3PredicateToken::LParen) => {
                self.bump();
                let inner = self.parse_expression()?;
                match self.bump() {
                    Some(Stage3PredicateToken::RParen) => Ok(inner),
                    other => Err(format!("expected `)` after grouped fragment, got {other:?}")),
                }
            }
            Some(Stage3PredicateToken::Ident(_)) => self.parse_atom_or_call(),
            other => Err(format!("expected predicate term, got {other:?}")),
        }
    }

    fn parse_atom_or_call(&mut self) -> Result<Stage3PredicateFragment, String> {
        let ident = match self.bump() {
            Some(Stage3PredicateToken::Ident(ident)) => ident,
            other => return Err(format!("expected identifier, got {other:?}")),
        };

        if !matches!(self.peek(), Some(Stage3PredicateToken::LParen)) {
            return Ok(Stage3PredicateFragment::Atom { name: ident });
        }

        self.bump();
        let mut args = Vec::new();
        if !matches!(self.peek(), Some(Stage3PredicateToken::RParen)) {
            loop {
                match self.bump() {
                    Some(Stage3PredicateToken::Ident(arg)) => args.push(arg),
                    other => return Err(format!("expected call argument, got {other:?}")),
                }

                match self.peek() {
                    Some(Stage3PredicateToken::Comma) => {
                        self.bump();
                    }
                    Some(Stage3PredicateToken::RParen) => break,
                    other => {
                        return Err(format!(
                            "expected `,` or `)` after call argument, got {other:?}"
                        ))
                    }
                }
            }
        }

        match self.bump() {
            Some(Stage3PredicateToken::RParen) => Ok(Stage3PredicateFragment::Call {
                callee: ident,
                args,
            }),
            other => Err(format!("expected `)` after call arguments, got {other:?}")),
        }
    }
}
