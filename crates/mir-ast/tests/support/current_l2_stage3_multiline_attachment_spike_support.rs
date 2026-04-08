#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceLine {
    line_no: usize,
    indent: usize,
    text: String,
    is_blank: bool,
}

pub fn extract_stage3_option_admit_multiline_fragment_text(
    source: &str,
) -> Result<String, String> {
    let lines = collect_source_lines(source);
    let head_index = find_first_head(&lines, "option ")
        .ok_or_else(|| "missing option declaration head".to_string())?;

    extract_stage3_multiline_clause_from_head(&lines, head_index, "admit")
}

pub fn extract_stage3_request_clause_multiline_fragment_text(
    source: &str,
    clause_name: &str,
) -> Result<String, String> {
    let lines = collect_source_lines(source);
    let head_index = find_first_head(&lines, "perform ")
        .ok_or_else(|| "missing perform request head".to_string())?;

    extract_stage3_multiline_clause_from_head(&lines, head_index, clause_name)
}

fn collect_source_lines(source: &str) -> Vec<SourceLine> {
    source
        .lines()
        .enumerate()
        .map(|(index, raw_line)| {
            let trimmed = raw_line.trim();
            SourceLine {
                line_no: index + 1,
                indent: raw_line.chars().take_while(|ch| *ch == ' ').count(),
                text: trimmed.to_string(),
                is_blank: trimmed.is_empty(),
            }
        })
        .collect()
}

fn find_first_head(lines: &[SourceLine], prefix: &str) -> Option<usize> {
    lines.iter()
        .position(|line| !line.is_blank && line.text.starts_with(prefix))
}

fn extract_stage3_multiline_clause_from_head(
    lines: &[SourceLine],
    head_index: usize,
    clause_name: &str,
) -> Result<String, String> {
    let head_indent = lines[head_index].indent;
    let header = format!("{clause_name}:");
    let first_child_indent = lines
        .iter()
        .enumerate()
        .skip(head_index + 1)
        .find_map(|(index, line)| {
            if line.is_blank {
                return None;
            }
            if line.indent <= head_indent {
                return None;
            }
            Some((index, line.indent))
        })
        .map(|(_, indent)| indent)
        .ok_or_else(|| format!("missing `{header}` attachment header"))?;

    let mut index = head_index + 1;
    while index < lines.len() {
        let line = &lines[index];

        if line.is_blank {
            index += 1;
            continue;
        }
        if line.indent <= head_indent {
            break;
        }
        if line.indent == first_child_indent {
            if line.text == header {
                return extract_multiline_block(lines, index, &header);
            }

            index += 1;
            while index < lines.len() {
                let nested = &lines[index];
                if nested.is_blank {
                    index += 1;
                    continue;
                }
                if nested.indent <= first_child_indent {
                    break;
                }
                index += 1;
            }
            continue;
        }

        index += 1;
    }

    Err(format!("missing `{header}` attachment header"))
}

fn extract_multiline_block(lines: &[SourceLine], header_index: usize, header: &str) -> Result<String, String> {
    let header_indent = lines[header_index].indent;
    let mut block_lines = Vec::new();
    let mut index = header_index + 1;
    let blank_line_error = format!("blank line is not allowed inside multiline predicate block after {header}");

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

    Ok(block_lines
        .iter()
        .map(|line| line.dedented_text(min_indent))
        .collect::<Vec<_>>()
        .join("\n"))
}

impl SourceLine {
    fn dedented_text(&self, min_indent: usize) -> String {
        let relative_indent = self.indent.saturating_sub(min_indent);
        format!("{}{}", " ".repeat(relative_indent), self.text)
    }
}
