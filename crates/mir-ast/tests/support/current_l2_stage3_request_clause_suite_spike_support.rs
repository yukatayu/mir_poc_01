#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stage3RequestClauseSuite {
    pub require_fragment_text: Option<String>,
    pub ensure_fragment_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceLine {
    indent: usize,
    text: String,
    is_blank: bool,
}

pub fn extract_stage3_request_clause_suite(
    source: &str,
) -> Result<Stage3RequestClauseSuite, String> {
    let lines = collect_source_lines(source);
    let head_index = find_first_head(&lines, "perform ")
        .ok_or_else(|| "missing perform request head".to_string())?;
    let head_indent = lines[head_index].indent;
    let child_indent = lines.iter().skip(head_index + 1).find_map(|line| {
        if line.is_blank || line.indent <= head_indent {
            return None;
        }
        Some(line.indent)
    });

    let Some(child_indent) = child_indent else {
        return Ok(Stage3RequestClauseSuite {
            require_fragment_text: None,
            ensure_fragment_text: None,
        });
    };

    let mut suite = Stage3RequestClauseSuite {
        require_fragment_text: None,
        ensure_fragment_text: None,
    };
    let mut index = head_index + 1;
    let mut saw_clause = false;
    let mut pending_blank_between_clauses = false;

    while index < lines.len() {
        let line = &lines[index];

        if line.is_blank {
            if saw_clause {
                pending_blank_between_clauses = true;
            }
            index += 1;
            continue;
        }

        if line.indent <= head_indent {
            break;
        }

        if line.indent > child_indent {
            return Err(
                "unexpected nested continuation outside request-local clause block".to_string(),
            );
        }

        if line.indent < child_indent {
            break;
        }

        if pending_blank_between_clauses {
            return Err("blank line is not allowed between request-local clauses".to_string());
        }

        if line.text == "require:" {
            if suite.require_fragment_text.is_some() {
                return Err("duplicate `require` clause is not allowed".to_string());
            }
            if suite.ensure_fragment_text.is_some() {
                return Err("require clause cannot appear after ensure clause".to_string());
            }
            let (fragment, next_index) =
                extract_multiline_block(lines.as_slice(), index, child_indent, "require:")?;
            suite.require_fragment_text = Some(fragment);
            saw_clause = true;
            index = next_index;
            continue;
        }

        if let Some(fragment_text) = extract_single_line_fragment(&line.text, "require")? {
            if suite.require_fragment_text.is_some() {
                return Err("duplicate `require` clause is not allowed".to_string());
            }
            if suite.ensure_fragment_text.is_some() {
                return Err("require clause cannot appear after ensure clause".to_string());
            }
            suite.require_fragment_text = Some(fragment_text);
            saw_clause = true;
            index += 1;
            continue;
        }

        if line.text == "ensure:" {
            if suite.ensure_fragment_text.is_some() {
                return Err("duplicate `ensure` clause is not allowed".to_string());
            }
            let (fragment, next_index) =
                extract_multiline_block(lines.as_slice(), index, child_indent, "ensure:")?;
            suite.ensure_fragment_text = Some(fragment);
            saw_clause = true;
            index = next_index;
            continue;
        }

        if let Some(fragment_text) = extract_single_line_fragment(&line.text, "ensure")? {
            if suite.ensure_fragment_text.is_some() {
                return Err("duplicate `ensure` clause is not allowed".to_string());
            }
            suite.ensure_fragment_text = Some(fragment_text);
            saw_clause = true;
            index += 1;
            continue;
        }

        return Err(format!(
            "unsupported request-local clause line inside fixed two-slot suite: `{}`",
            line.text
        ));
    }

    Ok(suite)
}

fn collect_source_lines(source: &str) -> Vec<SourceLine> {
    source
        .lines()
        .map(|raw_line| {
            let trimmed = raw_line.trim();
            SourceLine {
                indent: raw_line.chars().take_while(|ch| *ch == ' ').count(),
                text: trimmed.to_string(),
                is_blank: trimmed.is_empty(),
            }
        })
        .collect()
}

fn find_first_head(lines: &[SourceLine], prefix: &str) -> Option<usize> {
    lines
        .iter()
        .position(|line| !line.is_blank && line.text.starts_with(prefix))
}

fn extract_single_line_fragment(text: &str, clause_name: &str) -> Result<Option<String>, String> {
    let prefix = format!("{clause_name} ");
    if !text.starts_with(&prefix) {
        return Ok(None);
    }

    let fragment = text[prefix.len()..].trim();
    if fragment.is_empty() {
        return Err(format!("missing predicate fragment after `{clause_name}`"));
    }

    Ok(Some(fragment.to_string()))
}

fn extract_multiline_block(
    lines: &[SourceLine],
    header_index: usize,
    header_indent: usize,
    header: &str,
) -> Result<(String, usize), String> {
    let mut block_lines = Vec::new();
    let mut index = header_index + 1;
    let blank_line_error =
        format!("blank line is not allowed inside multiline predicate block after {header}");

    while index < lines.len() {
        let line = &lines[index];
        if line.is_blank {
            return Err(blank_line_error);
        }
        if line.indent <= header_indent {
            break;
        }
        block_lines.push(line);
        index += 1;
    }

    if block_lines.is_empty() {
        return Err(format!("missing multiline predicate block after {header}"));
    }

    let min_indent = block_lines
        .iter()
        .map(|line| line.indent)
        .min()
        .expect("block_lines is not empty");

    let fragment = block_lines
        .iter()
        .map(|line| {
            let relative_indent = line.indent.saturating_sub(min_indent);
            format!("{}{}", " ".repeat(relative_indent), line.text)
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok((fragment, index))
}
