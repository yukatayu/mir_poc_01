use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextualMirDiagnostic {
    pub code: String,
    pub message: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextualMirParseReport {
    pub accepted: bool,
    pub module: Option<AstModule>,
    pub diagnostics: Vec<TextualMirDiagnostic>,
    pub final_public_grammar_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextualMirModuleResolution {
    Missing,
    Unique(PathBuf),
    Ambiguous(Vec<PathBuf>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstModule {
    pub module_path: String,
    pub imports: Vec<AstImport>,
    pub capabilities: Vec<AstCapabilityDecl>,
    pub effects: Vec<AstEffectDecl>,
    pub records: Vec<AstRecord>,
    pub transitions: Vec<AstTransition>,
    pub items: Vec<AstTopLevel>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstTopLevel {
    Import(AstImport),
    Capability(AstCapabilityDecl),
    Record(AstRecord),
    Effect(AstEffectDecl),
    Function(AstFunction),
    Transition(AstTransition),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstImport {
    pub module_path: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstCapabilityDecl {
    pub capability_name: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstRecord {
    pub record_name: String,
    pub fields: Vec<AstRecordField>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstRecordField {
    pub field_name: String,
    pub field_type: AstType,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstEffectDecl {
    pub effect_name: String,
    pub parameters: Vec<AstParam>,
    pub required_capabilities: Vec<String>,
    pub output: Option<AstEffectOutput>,
    pub failure_row: Vec<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstEffectOutput {
    pub name: String,
    pub output_type: AstType,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstParam {
    pub name: String,
    pub param_type: AstType,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstFunction {
    pub function_name: String,
    pub parameter_name: String,
    pub input_type: AstType,
    pub output_type: AstType,
    pub body: Vec<AstStmt>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstTransition {
    pub transition_name: String,
    pub place_ref: String,
    pub required_capabilities: Vec<String>,
    pub body: Vec<AstStmt>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstStmt {
    Let {
        name: String,
        mutable: bool,
        ty: AstType,
        value: AstExpr,
        span: SourceSpan,
    },
    Assign {
        name: String,
        value: AstExpr,
        span: SourceSpan,
    },
    If {
        condition: AstExpr,
        then_body: Vec<AstStmt>,
        else_body: Vec<AstStmt>,
        span: SourceSpan,
    },
    While {
        condition: AstExpr,
        body: Vec<AstStmt>,
        span: SourceSpan,
    },
    For {
        binding: String,
        start: AstExpr,
        end: AstExpr,
        body: Vec<AstStmt>,
        span: SourceSpan,
    },
    Bind {
        name: String,
        value: AstBindValue,
        contract_clauses: Vec<AstContractClause>,
        span: SourceSpan,
    },
    Perform {
        call: AstPerformCall,
        contract_clauses: Vec<AstContractClause>,
        span: SourceSpan,
    },
    Return {
        value: AstExpr,
        span: SourceSpan,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstBindValue {
    Expr(AstExpr),
    Perform(AstPerformCall),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstPerformCall {
    pub effect_name: String,
    pub arguments: Vec<AstExpr>,
    pub boundary_ref: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstContractClause {
    pub kind: AstContractClauseKind,
    pub condition: AstExpr,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstContractClauseKind {
    Require,
    Ensure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstExpr {
    pub kind: AstExprKind,
    pub span: SourceSpan,
}

impl AstExpr {
    fn new(kind: AstExprKind, span: SourceSpan) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstExprKind {
    IntLiteral(i64),
    FloatLiteral(String),
    BoolLiteral(bool),
    TextLiteral(String),
    Variable(String),
    ArrayLiteral(Vec<AstExpr>),
    RecordConstruct {
        record_name: String,
        fields: Vec<AstRecordConstructField>,
    },
    Call {
        callee: Box<AstExpr>,
        arguments: Vec<AstExpr>,
    },
    Index {
        base: Box<AstExpr>,
        index: Box<AstExpr>,
    },
    FieldAccess {
        base: Box<AstExpr>,
        field_name: String,
    },
    Unary {
        op: AstUnaryOp,
        expr: Box<AstExpr>,
    },
    Binary {
        op: AstBinaryOp,
        left: Box<AstExpr>,
        right: Box<AstExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AstRecordConstructField {
    pub field_name: String,
    pub value: AstExpr,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstType {
    Bool,
    Int64,
    UInt64,
    Float64,
    Text,
    Unit,
    Named(String),
    FixedArray {
        element: Box<AstType>,
        length: usize,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstUnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AstBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    And,
    Or,
}

pub fn parse_textual_mir_module(source: &str) -> Result<AstModule, Vec<TextualMirDiagnostic>> {
    let report = parse_textual_mir_report(source);
    match report.module {
        Some(module) => Ok(module),
        None => Err(report.diagnostics),
    }
}

pub fn parse_textual_mir_report(source: &str) -> TextualMirParseReport {
    match Lexer::new(source)
        .tokenize()
        .and_then(|tokens| Parser::new(tokens).parse_module())
    {
        Ok(module) => TextualMirParseReport {
            accepted: true,
            module: Some(module),
            diagnostics: Vec::new(),
            final_public_grammar_frozen: false,
        },
        Err(diagnostic) => TextualMirParseReport {
            accepted: false,
            module: None,
            diagnostics: vec![diagnostic],
            final_public_grammar_frozen: false,
        },
    }
}

pub fn parse_textual_mir_report_path(path: impl AsRef<Path>) -> TextualMirParseReport {
    let path = path.as_ref();
    match fs::read_to_string(path) {
        Ok(source) => {
            let mut report = parse_textual_mir_report(&source);
            if let Some(module) = report.module.as_ref() {
                let diagnostics = validate_imports(module, path);
                if !diagnostics.is_empty() {
                    report.accepted = false;
                    report.diagnostics.extend(diagnostics);
                }
            }
            report
        }
        Err(error) => TextualMirParseReport {
            accepted: false,
            module: None,
            diagnostics: vec![TextualMirDiagnostic {
                code: "io_error".to_string(),
                message: format!("failed to read {}: {error}", path.display()),
                span: SourceSpan {
                    start: 0,
                    end: 0,
                    line: 1,
                    column: 1,
                },
            }],
            final_public_grammar_frozen: false,
        },
    }
}

pub fn resolve_textual_mir_module_reference(
    current_path: impl AsRef<Path>,
    module_path: &str,
) -> TextualMirModuleResolution {
    let current_path = current_path.as_ref();
    let Some(search_root) = find_import_search_root(current_path) else {
        return TextualMirModuleResolution::Missing;
    };
    let mut matches = find_declared_module_paths(&search_root, module_path);
    matches.sort();
    matches.dedup();
    match matches.len() {
        0 => TextualMirModuleResolution::Missing,
        1 => TextualMirModuleResolution::Unique(matches.remove(0)),
        _ => TextualMirModuleResolution::Ambiguous(matches),
    }
}

pub fn resolve_textual_mir_module_path(
    current_path: impl AsRef<Path>,
    module_path: &str,
) -> Option<PathBuf> {
    match resolve_textual_mir_module_reference(current_path, module_path) {
        TextualMirModuleResolution::Unique(path) => Some(path),
        TextualMirModuleResolution::Missing | TextualMirModuleResolution::Ambiguous(_) => None,
    }
}

pub fn parse_textual_mir_module_path(
    path: impl AsRef<Path>,
) -> Result<AstModule, Vec<TextualMirDiagnostic>> {
    let report = parse_textual_mir_report_path(path);
    if report.accepted {
        match report.module {
            Some(module) => Ok(module),
            None => Err(report.diagnostics),
        }
    } else {
        Err(report.diagnostics)
    }
}

type ParseResult<T> = Result<T, TextualMirDiagnostic>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Keyword {
    And,
    At,
    Capability,
    Effect,
    Else,
    Ensure,
    Failure,
    False,
    Fn,
    For,
    If,
    Import,
    In,
    Let,
    Module,
    Mut,
    Not,
    Or,
    Output,
    Perform,
    Record,
    Require,
    Requires,
    Return,
    Transition,
    True,
    Via,
    While,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Identifier(String),
    Integer(String),
    Float(String),
    String(String),
    Keyword(Keyword),
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    Dot,
    Semicolon,
    Arrow,
    LeftArrow,
    Equals,
    BangEquals,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Plus,
    Minus,
    Star,
    Slash,
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    span: SourceSpan,
}

struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    index: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            index: 0,
            line: 1,
            column: 1,
        }
    }

    fn tokenize(mut self) -> ParseResult<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek_char() {
            match ch {
                ' ' | '\t' | '\r' => {
                    self.bump_char();
                }
                '\n' => {
                    let span = self.current_span(1);
                    self.bump_char();
                    tokens.push(Token {
                        kind: TokenKind::Newline,
                        span,
                    });
                }
                '/' if self.peek_next_char() == Some('/') => {
                    self.bump_char();
                    self.bump_char();
                    while let Some(next) = self.peek_char() {
                        if next == '\n' {
                            break;
                        }
                        self.bump_char();
                    }
                }
                '"' => tokens.push(self.lex_string()?),
                '0'..='9' => tokens.push(self.lex_number()?),
                'A'..='Z' | 'a'..='z' | '_' => tokens.push(self.lex_identifier_or_keyword()),
                '{' => tokens.push(self.single_char(TokenKind::LeftBrace)),
                '}' => tokens.push(self.single_char(TokenKind::RightBrace)),
                '(' => tokens.push(self.single_char(TokenKind::LeftParen)),
                ')' => tokens.push(self.single_char(TokenKind::RightParen)),
                '[' => tokens.push(self.single_char(TokenKind::LeftBracket)),
                ']' => tokens.push(self.single_char(TokenKind::RightBracket)),
                ':' => tokens.push(self.single_char(TokenKind::Colon)),
                ',' => tokens.push(self.single_char(TokenKind::Comma)),
                '.' => tokens.push(self.single_char(TokenKind::Dot)),
                ';' => tokens.push(self.single_char(TokenKind::Semicolon)),
                '+' => tokens.push(self.single_char(TokenKind::Plus)),
                '*' => tokens.push(self.single_char(TokenKind::Star)),
                '/' => tokens.push(self.single_char(TokenKind::Slash)),
                '-' if self.peek_next_char() == Some('>') => {
                    let span = self.current_span(2);
                    self.bump_char();
                    self.bump_char();
                    tokens.push(Token {
                        kind: TokenKind::Arrow,
                        span,
                    });
                }
                '<' if self.peek_next_char() == Some('-') => {
                    let span = self.current_span(2);
                    self.bump_char();
                    self.bump_char();
                    tokens.push(Token {
                        kind: TokenKind::LeftArrow,
                        span,
                    });
                }
                '!' if self.peek_next_char() == Some('=') => {
                    let span = self.current_span(2);
                    self.bump_char();
                    self.bump_char();
                    tokens.push(Token {
                        kind: TokenKind::BangEquals,
                        span,
                    });
                }
                '<' if self.peek_next_char() == Some('=') => {
                    let span = self.current_span(2);
                    self.bump_char();
                    self.bump_char();
                    tokens.push(Token {
                        kind: TokenKind::LessEqual,
                        span,
                    });
                }
                '>' if self.peek_next_char() == Some('=') => {
                    let span = self.current_span(2);
                    self.bump_char();
                    self.bump_char();
                    tokens.push(Token {
                        kind: TokenKind::GreaterEqual,
                        span,
                    });
                }
                '-' => tokens.push(self.single_char(TokenKind::Minus)),
                '=' => tokens.push(self.single_char(TokenKind::Equals)),
                '<' => tokens.push(self.single_char(TokenKind::Less)),
                '>' => tokens.push(self.single_char(TokenKind::Greater)),
                _ => {
                    return Err(TextualMirDiagnostic {
                        code: "unexpected_character".to_string(),
                        message: format!("unexpected character `{ch}`"),
                        span: self.current_span(1),
                    });
                }
            }
        }
        tokens.push(Token {
            kind: TokenKind::Eof,
            span: SourceSpan {
                start: self.source.len(),
                end: self.source.len(),
                line: self.line,
                column: self.column,
            },
        });
        Ok(tokens)
    }

    fn lex_string(&mut self) -> ParseResult<Token> {
        let start = self.index;
        let line = self.line;
        let column = self.column;
        self.bump_char();
        let mut value = String::new();
        while let Some(ch) = self.peek_char() {
            if ch == '"' {
                self.bump_char();
                return Ok(Token {
                    kind: TokenKind::String(value),
                    span: SourceSpan {
                        start,
                        end: self.index,
                        line,
                        column,
                    },
                });
            }
            if ch == '\n' {
                return Err(TextualMirDiagnostic {
                    code: "unterminated_string".to_string(),
                    message: "unterminated string literal".to_string(),
                    span: SourceSpan {
                        start,
                        end: self.index,
                        line,
                        column,
                    },
                });
            }
            value.push(ch);
            self.bump_char();
        }
        Err(TextualMirDiagnostic {
            code: "unterminated_string".to_string(),
            message: "unterminated string literal".to_string(),
            span: SourceSpan {
                start,
                end: self.index,
                line,
                column,
            },
        })
    }

    fn lex_number(&mut self) -> ParseResult<Token> {
        let start = self.index;
        let line = self.line;
        let column = self.column;
        let mut text = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_digit() {
                text.push(ch);
                self.bump_char();
            } else {
                break;
            }
        }
        let kind = if self.peek_char() == Some('.')
            && self
                .peek_next_char()
                .map(|next| next.is_ascii_digit())
                .unwrap_or(false)
        {
            text.push('.');
            self.bump_char();
            while let Some(ch) = self.peek_char() {
                if ch.is_ascii_digit() {
                    text.push(ch);
                    self.bump_char();
                } else {
                    break;
                }
            }
            TokenKind::Float(text)
        } else {
            TokenKind::Integer(text)
        };
        Ok(Token {
            kind,
            span: SourceSpan {
                start,
                end: self.index,
                line,
                column,
            },
        })
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let start = self.index;
        let line = self.line;
        let column = self.column;
        let mut text = String::new();
        while let Some(ch) = self.peek_char() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                text.push(ch);
                self.bump_char();
            } else {
                break;
            }
        }
        let kind = match text.as_str() {
            "and" => TokenKind::Keyword(Keyword::And),
            "at" => TokenKind::Keyword(Keyword::At),
            "capability" => TokenKind::Keyword(Keyword::Capability),
            "effect" => TokenKind::Keyword(Keyword::Effect),
            "else" => TokenKind::Keyword(Keyword::Else),
            "ensure" => TokenKind::Keyword(Keyword::Ensure),
            "failure" => TokenKind::Keyword(Keyword::Failure),
            "false" => TokenKind::Keyword(Keyword::False),
            "fn" => TokenKind::Keyword(Keyword::Fn),
            "for" => TokenKind::Keyword(Keyword::For),
            "if" => TokenKind::Keyword(Keyword::If),
            "import" => TokenKind::Keyword(Keyword::Import),
            "in" => TokenKind::Keyword(Keyword::In),
            "let" => TokenKind::Keyword(Keyword::Let),
            "module" => TokenKind::Keyword(Keyword::Module),
            "mut" => TokenKind::Keyword(Keyword::Mut),
            "not" => TokenKind::Keyword(Keyword::Not),
            "or" => TokenKind::Keyword(Keyword::Or),
            "output" => TokenKind::Keyword(Keyword::Output),
            "perform" => TokenKind::Keyword(Keyword::Perform),
            "record" => TokenKind::Keyword(Keyword::Record),
            "require" => TokenKind::Keyword(Keyword::Require),
            "requires" => TokenKind::Keyword(Keyword::Requires),
            "return" => TokenKind::Keyword(Keyword::Return),
            "transition" => TokenKind::Keyword(Keyword::Transition),
            "true" => TokenKind::Keyword(Keyword::True),
            "via" => TokenKind::Keyword(Keyword::Via),
            "while" => TokenKind::Keyword(Keyword::While),
            _ => TokenKind::Identifier(text),
        };
        Token {
            kind,
            span: SourceSpan {
                start,
                end: self.index,
                line,
                column,
            },
        }
    }

    fn single_char(&mut self, kind: TokenKind) -> Token {
        let span = self.current_span(1);
        self.bump_char();
        Token { kind, span }
    }

    fn current_span(&self, length: usize) -> SourceSpan {
        SourceSpan {
            start: self.index,
            end: self.index + length,
            line: self.line,
            column: self.column,
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.index).copied()
    }

    fn peek_next_char(&self) -> Option<char> {
        self.chars.get(self.index + 1).copied()
    }

    fn bump_char(&mut self) {
        if let Some(ch) = self.peek_char() {
            self.index += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn parse_module(&mut self) -> ParseResult<AstModule> {
        self.skip_newlines();
        let module_start = self.expect_keyword(Keyword::Module)?.span;
        let module_path = self.parse_path()?;
        self.skip_statement_breaks();

        let mut items = Vec::new();
        let mut imports = Vec::new();
        let mut capabilities = Vec::new();
        let mut effects = Vec::new();
        let mut records = Vec::new();
        let mut transitions = Vec::new();

        while !self.at_eof() {
            self.skip_newlines();
            if self.at_eof() {
                break;
            }

            match self.peek_kind() {
                TokenKind::Keyword(Keyword::Import) => {
                    let import = self.parse_import()?;
                    imports.push(import.clone());
                    items.push(AstTopLevel::Import(import));
                }
                TokenKind::Keyword(Keyword::Capability) => {
                    let capability = self.parse_capability()?;
                    capabilities.push(capability.clone());
                    items.push(AstTopLevel::Capability(capability));
                }
                TokenKind::Keyword(Keyword::Record) => {
                    let record = self.parse_record()?;
                    records.push(record.clone());
                    items.push(AstTopLevel::Record(record));
                }
                TokenKind::Keyword(Keyword::Effect) => {
                    let effect = self.parse_effect()?;
                    effects.push(effect.clone());
                    items.push(AstTopLevel::Effect(effect));
                }
                TokenKind::Keyword(Keyword::Fn) => {
                    items.push(AstTopLevel::Function(self.parse_function()?));
                }
                TokenKind::Keyword(Keyword::Transition) => {
                    let transition = self.parse_transition()?;
                    transitions.push(transition.clone());
                    items.push(AstTopLevel::Transition(transition));
                }
                _ => {
                    return Err(self.error_here(
                        "unexpected_top_level_item",
                        "expected import, capability, record, effect, fn, or transition",
                    ));
                }
            }
            self.skip_statement_breaks();
        }

        let module_end = items
            .last()
            .map(top_level_span_end)
            .unwrap_or(module_start.end);

        Ok(AstModule {
            module_path,
            imports,
            capabilities,
            effects,
            records,
            transitions,
            items,
            span: SourceSpan {
                start: module_start.start,
                end: module_end,
                line: module_start.line,
                column: module_start.column,
            },
        })
    }

    fn parse_import(&mut self) -> ParseResult<AstImport> {
        let start = self.expect_keyword(Keyword::Import)?.span;
        let module_path = self.parse_path()?;
        Ok(AstImport {
            module_path,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_capability(&mut self) -> ParseResult<AstCapabilityDecl> {
        let start = self.expect_keyword(Keyword::Capability)?.span;
        let capability_name = self.parse_identifier()?;
        Ok(AstCapabilityDecl {
            capability_name,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_record(&mut self) -> ParseResult<AstRecord> {
        let start = self.expect_keyword(Keyword::Record)?.span;
        let record_name = self.parse_identifier()?;
        self.expect(
            TokenKind::LeftBrace,
            "expected_left_brace",
            "expected `{` after record name",
        )?;
        self.skip_newlines();
        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) {
            let field_start = self.current_span();
            let field_name = self.parse_identifier()?;
            self.expect(
                TokenKind::Colon,
                "expected_colon",
                "expected `:` after record field name",
            )?;
            let field_type = self.parse_type()?;
            fields.push(AstRecordField {
                field_name,
                field_type,
                span: span_from(field_start, self.previous_span()),
            });
            self.consume_if(&TokenKind::Comma);
            self.skip_newlines();
        }
        let end = self
            .expect(
                TokenKind::RightBrace,
                "expected_right_brace",
                "expected `}` after record fields",
            )?
            .span;
        Ok(AstRecord {
            record_name,
            fields,
            span: span_from(start, end),
        })
    }

    fn parse_effect(&mut self) -> ParseResult<AstEffectDecl> {
        let start = self.expect_keyword(Keyword::Effect)?.span;
        let effect_name = self.parse_identifier()?;
        let parameters = if self.consume_if(&TokenKind::LeftParen) {
            let params = self.parse_parameters()?;
            self.expect(
                TokenKind::RightParen,
                "expected_right_paren",
                "expected `)` after effect parameters",
            )?;
            params
        } else {
            Vec::new()
        };
        self.expect(
            TokenKind::LeftBrace,
            "expected_left_brace",
            "expected `{` after effect header",
        )?;
        self.skip_newlines();
        let mut required_capabilities = Vec::new();
        let mut output = None;
        let mut failure_row = Vec::new();
        while !self.check(&TokenKind::RightBrace) {
            match self.peek_kind() {
                TokenKind::Keyword(Keyword::Requires) => {
                    self.advance();
                    required_capabilities = self.parse_identifier_list()?;
                }
                TokenKind::Keyword(Keyword::Output) => {
                    let output_start = self.advance().span;
                    let name = self.parse_identifier()?;
                    self.expect(
                        TokenKind::Colon,
                        "expected_colon",
                        "expected `:` after output name",
                    )?;
                    let output_type = self.parse_type()?;
                    output = Some(AstEffectOutput {
                        name,
                        output_type,
                        span: span_from(output_start, self.previous_span()),
                    });
                }
                TokenKind::Keyword(Keyword::Failure) => {
                    self.advance();
                    failure_row = self.parse_identifier_list()?;
                }
                _ => {
                    return Err(self.error_here(
                        "unexpected_effect_member",
                        "expected requires, output, or failure in effect body",
                    ));
                }
            }
            self.skip_statement_breaks();
        }
        let end = self
            .expect(
                TokenKind::RightBrace,
                "expected_right_brace",
                "expected `}` after effect body",
            )?
            .span;
        Ok(AstEffectDecl {
            effect_name,
            parameters,
            required_capabilities,
            output,
            failure_row,
            span: span_from(start, end),
        })
    }

    fn parse_function(&mut self) -> ParseResult<AstFunction> {
        let start = self.expect_keyword(Keyword::Fn)?.span;
        let function_name = self.parse_identifier()?;
        self.expect(
            TokenKind::LeftParen,
            "expected_left_paren",
            "expected `(` after function name",
        )?;
        let parameter_name = self.parse_identifier()?;
        self.expect(
            TokenKind::Colon,
            "expected_colon",
            "expected `:` after function parameter name",
        )?;
        let input_type = self.parse_type()?;
        self.expect(
            TokenKind::RightParen,
            "expected_right_paren",
            "expected `)` after function parameter",
        )?;
        self.expect(
            TokenKind::Arrow,
            "expected_arrow",
            "expected `->` after function parameters",
        )?;
        let output_type = self.parse_type()?;
        let body = self.parse_block()?;
        Ok(AstFunction {
            function_name,
            parameter_name,
            input_type,
            output_type,
            body,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_transition(&mut self) -> ParseResult<AstTransition> {
        let start = self.expect_keyword(Keyword::Transition)?.span;
        let transition_name = self.parse_identifier()?;
        if !self.consume_keyword(Keyword::At) {
            return Err(self.error_here(
                "expected_at_after_transition",
                "expected `at` after transition name",
            ));
        }
        let place_ref = self.parse_path()?;
        let required_capabilities = if self.consume_keyword(Keyword::Requires) {
            self.parse_identifier_list()?
        } else {
            Vec::new()
        };
        let body = self.parse_block()?;
        Ok(AstTransition {
            transition_name,
            place_ref,
            required_capabilities,
            body,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_block(&mut self) -> ParseResult<Vec<AstStmt>> {
        self.expect(
            TokenKind::LeftBrace,
            "expected_left_brace",
            "expected `{` to begin block",
        )?;
        self.skip_newlines();
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) {
            statements.push(self.parse_statement()?);
            self.skip_statement_breaks();
        }
        self.expect(
            TokenKind::RightBrace,
            "expected_right_brace",
            "expected `}` to close block",
        )?;
        Ok(statements)
    }

    fn parse_statement(&mut self) -> ParseResult<AstStmt> {
        self.skip_newlines();
        match self.peek_kind() {
            TokenKind::Keyword(Keyword::Let) => self.parse_let_statement(),
            TokenKind::Keyword(Keyword::If) => self.parse_if_statement(),
            TokenKind::Keyword(Keyword::While) => self.parse_while_statement(),
            TokenKind::Keyword(Keyword::For) => self.parse_for_statement(),
            TokenKind::Keyword(Keyword::Perform) => self.parse_perform_statement(),
            TokenKind::Keyword(Keyword::Return) => self.parse_return_statement(),
            TokenKind::Keyword(Keyword::Require) | TokenKind::Keyword(Keyword::Ensure) => Err(self
                .error_here(
                    "contract_clause_outside_allowed_position",
                    "contract clause must follow perform/bind statement",
                )),
            TokenKind::Identifier(_) => {
                if self.lookahead_is_left_arrow() {
                    self.parse_bind_statement()
                } else if self.lookahead_is_equals() {
                    self.parse_assign_statement()
                } else {
                    Err(self.error_here(
                        "unsupported_statement",
                        "expected assignment or bind statement",
                    ))
                }
            }
            _ => Err(self.error_here("unexpected_statement", "expected statement in block")),
        }
    }

    fn parse_let_statement(&mut self) -> ParseResult<AstStmt> {
        let start = self.expect_keyword(Keyword::Let)?.span;
        let mutable = self.consume_keyword(Keyword::Mut);
        let name = self.parse_identifier()?;
        self.expect(
            TokenKind::Colon,
            "expected_colon",
            "expected `:` after binding name",
        )?;
        let ty = self.parse_type()?;
        self.expect(
            TokenKind::Equals,
            "expected_equals",
            "expected `=` in let binding",
        )?;
        let value = self.parse_expr(0)?;
        Ok(AstStmt::Let {
            name,
            mutable,
            ty,
            value,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_assign_statement(&mut self) -> ParseResult<AstStmt> {
        let start = self.current_span();
        let name = self.parse_identifier()?;
        self.expect(
            TokenKind::Equals,
            "expected_equals",
            "expected `=` in assignment",
        )?;
        let value = self.parse_expr(0)?;
        Ok(AstStmt::Assign {
            name,
            value,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_if_statement(&mut self) -> ParseResult<AstStmt> {
        let start = self.expect_keyword(Keyword::If)?.span;
        let condition = self.parse_expr(0)?;
        let then_body = self.parse_block()?;
        let else_body = if self.consume_keyword(Keyword::Else) {
            self.parse_block()?
        } else {
            Vec::new()
        };
        Ok(AstStmt::If {
            condition,
            then_body,
            else_body,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_while_statement(&mut self) -> ParseResult<AstStmt> {
        let start = self.expect_keyword(Keyword::While)?.span;
        let condition = self.parse_expr(0)?;
        let body = self.parse_block()?;
        Ok(AstStmt::While {
            condition,
            body,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_for_statement(&mut self) -> ParseResult<AstStmt> {
        let start = self.expect_keyword(Keyword::For)?.span;
        let binding = self.parse_identifier()?;
        self.expect_keyword(Keyword::In)?;
        let start_expr = self.parse_expr(0)?;
        self.expect(
            TokenKind::Dot,
            "expected_dot",
            "expected `..` range in for loop",
        )?;
        self.expect(
            TokenKind::Dot,
            "expected_dot",
            "expected `..` range in for loop",
        )?;
        let end_expr = self.parse_expr(0)?;
        let body = self.parse_block()?;
        Ok(AstStmt::For {
            binding,
            start: start_expr,
            end: end_expr,
            body,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_bind_statement(&mut self) -> ParseResult<AstStmt> {
        let start = self.current_span();
        let name = self.parse_identifier()?;
        self.expect(
            TokenKind::LeftArrow,
            "expected_left_arrow",
            "expected `<-` in bind statement",
        )?;
        let value = if self.check_keyword(Keyword::Perform) {
            AstBindValue::Perform(self.parse_perform_call()?)
        } else {
            AstBindValue::Expr(self.parse_expr(0)?)
        };
        let contract_clauses = self.parse_attached_contract_clauses()?;
        Ok(AstStmt::Bind {
            name,
            value,
            contract_clauses,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_perform_statement(&mut self) -> ParseResult<AstStmt> {
        let start = self.current_span();
        let call = self.parse_perform_call()?;
        let contract_clauses = self.parse_attached_contract_clauses()?;
        Ok(AstStmt::Perform {
            call,
            contract_clauses,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_return_statement(&mut self) -> ParseResult<AstStmt> {
        let start = self.expect_keyword(Keyword::Return)?.span;
        let value = self.parse_expr(0)?;
        Ok(AstStmt::Return {
            value,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_perform_call(&mut self) -> ParseResult<AstPerformCall> {
        let start = self.expect_keyword(Keyword::Perform)?.span;
        let effect_name = self.parse_identifier()?;
        let arguments = if self.consume_if(&TokenKind::LeftParen) {
            let args = self.parse_arguments()?;
            self.expect(
                TokenKind::RightParen,
                "expected_right_paren",
                "expected `)` after perform arguments",
            )?;
            args
        } else {
            Vec::new()
        };
        if !self.consume_keyword(Keyword::Via) {
            return Err(self.error_here(
                "expected_via_after_perform",
                "expected `via` after perform call",
            ));
        }
        let boundary_ref = self.parse_path()?;
        Ok(AstPerformCall {
            effect_name,
            arguments,
            boundary_ref,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_attached_contract_clauses(&mut self) -> ParseResult<Vec<AstContractClause>> {
        let mut clauses = Vec::new();
        loop {
            let checkpoint = self.index;
            self.skip_newlines();
            let kind = if self.consume_keyword(Keyword::Require) {
                Some(AstContractClauseKind::Require)
            } else if self.consume_keyword(Keyword::Ensure) {
                Some(AstContractClauseKind::Ensure)
            } else {
                self.index = checkpoint;
                break;
            };
            let kind = kind.expect("kind already matched");
            let start = self.previous_span();
            let condition = self.parse_expr(0)?;
            clauses.push(AstContractClause {
                kind,
                condition,
                span: span_from(start, self.previous_span()),
            });
        }
        Ok(clauses)
    }

    fn parse_parameters(&mut self) -> ParseResult<Vec<AstParam>> {
        let mut params = Vec::new();
        if self.check(&TokenKind::RightParen) {
            return Ok(params);
        }
        loop {
            let start = self.current_span();
            let name = self.parse_identifier()?;
            self.expect(
                TokenKind::Colon,
                "expected_colon",
                "expected `:` after parameter name",
            )?;
            let param_type = self.parse_type()?;
            params.push(AstParam {
                name,
                param_type,
                span: span_from(start, self.previous_span()),
            });
            if !self.consume_if(&TokenKind::Comma) {
                break;
            }
        }
        Ok(params)
    }

    fn parse_arguments(&mut self) -> ParseResult<Vec<AstExpr>> {
        let mut args = Vec::new();
        if self.check(&TokenKind::RightParen) {
            return Ok(args);
        }
        loop {
            args.push(self.parse_expr(0)?);
            if !self.consume_if(&TokenKind::Comma) {
                break;
            }
        }
        Ok(args)
    }

    fn parse_identifier_list(&mut self) -> ParseResult<Vec<String>> {
        let mut items = vec![self.parse_identifier()?];
        while self.consume_if(&TokenKind::Comma) {
            items.push(self.parse_identifier()?);
        }
        Ok(items)
    }

    fn parse_type(&mut self) -> ParseResult<AstType> {
        if self.consume_if(&TokenKind::LeftBracket) {
            let element = self.parse_type()?;
            self.expect(
                TokenKind::Semicolon,
                "expected_semicolon",
                "expected `;` in fixed array type",
            )?;
            let length_text = self.expect_integer()?;
            let length = length_text.parse::<usize>().map_err(|_| {
                self.error_here(
                    "invalid_array_length",
                    "array length must be a non-negative integer",
                )
            })?;
            self.expect(
                TokenKind::RightBracket,
                "expected_right_bracket",
                "expected `]` after fixed array type",
            )?;
            return Ok(AstType::FixedArray {
                element: Box::new(element),
                length,
            });
        }

        let path = self.parse_path()?;
        Ok(match path.as_str() {
            "Bool" => AstType::Bool,
            "Int64" => AstType::Int64,
            "UInt64" => AstType::UInt64,
            "Float64" => AstType::Float64,
            "Text" => AstType::Text,
            "Unit" => AstType::Unit,
            _ => AstType::Named(path),
        })
    }

    fn parse_expr(&mut self, min_precedence: u8) -> ParseResult<AstExpr> {
        let mut expr = self.parse_prefix_expr()?;
        loop {
            expr = self.parse_postfix_expr(expr)?;
            let Some((precedence, op)) = self.current_binary_operator() else {
                break;
            };
            if precedence < min_precedence {
                break;
            }
            self.advance();
            let right = self.parse_expr(precedence + 1)?;
            let span = span_from(expr.span.clone(), right.span.clone());
            expr = AstExpr::new(
                AstExprKind::Binary {
                    op,
                    left: Box::new(expr),
                    right: Box::new(right),
                },
                span,
            );
        }
        Ok(expr)
    }

    fn parse_prefix_expr(&mut self) -> ParseResult<AstExpr> {
        match self.peek_kind() {
            TokenKind::Integer(text) => {
                let text = text.clone();
                let span = self.current_span();
                self.advance();
                let value = text.parse::<i64>().map_err(|_| {
                    self.error_here(
                        "invalid_integer_literal",
                        "integer literal is out of Int64 range",
                    )
                })?;
                Ok(AstExpr::new(AstExprKind::IntLiteral(value), span))
            }
            TokenKind::Float(text) => {
                let text = text.clone();
                let span = self.current_span();
                self.advance();
                Ok(AstExpr::new(AstExprKind::FloatLiteral(text), span))
            }
            TokenKind::String(text) => {
                let text = text.clone();
                let span = self.current_span();
                self.advance();
                Ok(AstExpr::new(AstExprKind::TextLiteral(text), span))
            }
            TokenKind::Keyword(Keyword::True) => {
                let span = self.current_span();
                self.advance();
                Ok(AstExpr::new(AstExprKind::BoolLiteral(true), span))
            }
            TokenKind::Keyword(Keyword::False) => {
                let span = self.current_span();
                self.advance();
                Ok(AstExpr::new(AstExprKind::BoolLiteral(false), span))
            }
            TokenKind::Keyword(Keyword::Not) => {
                let start = self.advance().span;
                let expr = self.parse_expr(6)?;
                let span = span_from(start, expr.span.clone());
                Ok(AstExpr::new(
                    AstExprKind::Unary {
                        op: AstUnaryOp::Not,
                        expr: Box::new(expr),
                    },
                    span,
                ))
            }
            TokenKind::Minus => {
                let start = self.advance().span;
                let expr = self.parse_expr(6)?;
                let span = span_from(start, expr.span.clone());
                Ok(AstExpr::new(
                    AstExprKind::Unary {
                        op: AstUnaryOp::Negate,
                        expr: Box::new(expr),
                    },
                    span,
                ))
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expr(0)?;
                self.expect(
                    TokenKind::RightParen,
                    "expected_right_paren",
                    "expected `)` after grouped expression",
                )?;
                Ok(expr)
            }
            TokenKind::LeftBracket => self.parse_array_literal(),
            TokenKind::Identifier(_) => {
                let start = self.current_span();
                let path = self.parse_path()?;
                let path_span = span_from(start, self.previous_span());
                if self.consume_if(&TokenKind::LeftBrace) {
                    return self.parse_record_construct(path, path_span);
                }
                Ok(AstExpr::new(AstExprKind::Variable(path), path_span))
            }
            _ => Err(self.error_here("unexpected_expression", "expected expression")),
        }
    }

    fn parse_postfix_expr(&mut self, mut expr: AstExpr) -> ParseResult<AstExpr> {
        loop {
            if self.consume_if(&TokenKind::LeftParen) {
                let arguments = self.parse_arguments()?;
                self.expect(
                    TokenKind::RightParen,
                    "expected_right_paren",
                    "expected `)` after call arguments",
                )?;
                let span = span_from(expr.span.clone(), self.previous_span());
                expr = AstExpr::new(
                    AstExprKind::Call {
                        callee: Box::new(expr),
                        arguments,
                    },
                    span,
                );
                continue;
            }
            if self.consume_if(&TokenKind::LeftBracket) {
                let index = self.parse_expr(0)?;
                self.expect(
                    TokenKind::RightBracket,
                    "expected_right_bracket",
                    "expected `]` after index expression",
                )?;
                let span = span_from(expr.span.clone(), self.previous_span());
                expr = AstExpr::new(
                    AstExprKind::Index {
                        base: Box::new(expr),
                        index: Box::new(index),
                    },
                    span,
                );
                continue;
            }
            if self.consume_if(&TokenKind::Dot) {
                let field_name = self.parse_identifier()?;
                let span = span_from(expr.span.clone(), self.previous_span());
                expr = AstExpr::new(
                    AstExprKind::FieldAccess {
                        base: Box::new(expr),
                        field_name,
                    },
                    span,
                );
                continue;
            }
            break;
        }
        Ok(expr)
    }

    fn parse_array_literal(&mut self) -> ParseResult<AstExpr> {
        let start = self
            .expect(
                TokenKind::LeftBracket,
                "expected_left_bracket",
                "expected `[` to begin array literal",
            )?
            .span;
        let mut elements = Vec::new();
        if !self.check(&TokenKind::RightBracket) {
            loop {
                elements.push(self.parse_expr(0)?);
                if !self.consume_if(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(
            TokenKind::RightBracket,
            "expected_right_bracket",
            "expected `]` after array literal",
        )?;
        Ok(AstExpr::new(
            AstExprKind::ArrayLiteral(elements),
            span_from(start, self.previous_span()),
        ))
    }

    fn parse_record_construct(
        &mut self,
        record_name: String,
        record_name_span: SourceSpan,
    ) -> ParseResult<AstExpr> {
        let mut fields = Vec::new();
        self.skip_newlines();
        while !self.check(&TokenKind::RightBrace) {
            let start = self.current_span();
            let field_name = self.parse_identifier()?;
            self.expect(
                TokenKind::Colon,
                "expected_colon",
                "expected `:` after field name",
            )?;
            let value = self.parse_expr(0)?;
            fields.push(AstRecordConstructField {
                field_name,
                value,
                span: span_from(start, self.previous_span()),
            });
            self.consume_if(&TokenKind::Comma);
            self.skip_newlines();
        }
        self.expect(
            TokenKind::RightBrace,
            "expected_right_brace",
            "expected `}` after record literal",
        )?;
        Ok(AstExpr::new(
            AstExprKind::RecordConstruct {
                record_name,
                fields,
            },
            span_from(record_name_span, self.previous_span()),
        ))
    }

    fn parse_path(&mut self) -> ParseResult<String> {
        let mut segments = vec![self.parse_identifier()?];
        while self.consume_if(&TokenKind::Dot) {
            segments.push(self.parse_identifier()?);
        }
        Ok(segments.join("."))
    }

    fn parse_identifier(&mut self) -> ParseResult<String> {
        match self.advance().kind {
            TokenKind::Identifier(name) => Ok(name),
            _ => Err(self.error_previous("expected_identifier", "expected identifier")),
        }
    }

    fn expect_integer(&mut self) -> ParseResult<String> {
        match self.advance().kind {
            TokenKind::Integer(text) => Ok(text),
            _ => Err(self.error_previous("expected_integer", "expected integer literal")),
        }
    }

    fn current_binary_operator(&self) -> Option<(u8, AstBinaryOp)> {
        match self.peek_kind() {
            TokenKind::Keyword(Keyword::Or) => Some((1, AstBinaryOp::Or)),
            TokenKind::Keyword(Keyword::And) => Some((2, AstBinaryOp::And)),
            TokenKind::Equals => Some((3, AstBinaryOp::Equal)),
            TokenKind::BangEquals => Some((3, AstBinaryOp::NotEqual)),
            TokenKind::Less => Some((3, AstBinaryOp::LessThan)),
            TokenKind::LessEqual => Some((3, AstBinaryOp::LessEqual)),
            TokenKind::Greater => Some((3, AstBinaryOp::GreaterThan)),
            TokenKind::GreaterEqual => Some((3, AstBinaryOp::GreaterEqual)),
            TokenKind::Plus => Some((4, AstBinaryOp::Add)),
            TokenKind::Minus => Some((4, AstBinaryOp::Sub)),
            TokenKind::Star => Some((5, AstBinaryOp::Mul)),
            TokenKind::Slash => Some((5, AstBinaryOp::Div)),
            _ => None,
        }
    }

    fn expect(&mut self, expected: TokenKind, code: &str, message: &str) -> ParseResult<Token> {
        if self.check(&expected) {
            Ok(self.advance())
        } else {
            Err(self.error_here(code, message))
        }
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> ParseResult<Token> {
        if self.check_keyword(keyword) {
            Ok(self.advance())
        } else {
            Err(self.error_here(
                "unexpected_keyword",
                &format!("expected keyword `{}`", keyword_name(keyword)),
            ))
        }
    }

    fn consume_if(&mut self, expected: &TokenKind) -> bool {
        if self.check(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume_keyword(&mut self, keyword: Keyword) -> bool {
        if self.check_keyword(keyword) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn skip_newlines(&mut self) {
        while matches!(self.peek_kind(), TokenKind::Newline) {
            self.advance();
        }
    }

    fn skip_statement_breaks(&mut self) {
        loop {
            let consumed =
                self.consume_if(&TokenKind::Newline) || self.consume_if(&TokenKind::Semicolon);
            if !consumed {
                break;
            }
        }
    }

    fn check(&self, expected: &TokenKind) -> bool {
        use TokenKind::*;
        matches!(
            (self.peek_kind(), expected),
            (LeftBrace, LeftBrace)
                | (RightBrace, RightBrace)
                | (LeftParen, LeftParen)
                | (RightParen, RightParen)
                | (LeftBracket, LeftBracket)
                | (RightBracket, RightBracket)
                | (Colon, Colon)
                | (Comma, Comma)
                | (Dot, Dot)
                | (Semicolon, Semicolon)
                | (Arrow, Arrow)
                | (LeftArrow, LeftArrow)
                | (Equals, Equals)
                | (BangEquals, BangEquals)
                | (Less, Less)
                | (LessEqual, LessEqual)
                | (Greater, Greater)
                | (GreaterEqual, GreaterEqual)
                | (Plus, Plus)
                | (Minus, Minus)
                | (Star, Star)
                | (Slash, Slash)
                | (Newline, Newline)
                | (Eof, Eof)
        )
    }

    fn check_keyword(&self, keyword: Keyword) -> bool {
        matches!(self.peek_kind(), TokenKind::Keyword(actual) if *actual == keyword)
    }

    fn lookahead_is_left_arrow(&self) -> bool {
        matches!(
            (
                &self.tokens[self.index].kind,
                self.tokens.get(self.index + 1).map(|token| &token.kind)
            ),
            (TokenKind::Identifier(_), Some(TokenKind::LeftArrow))
        )
    }

    fn lookahead_is_equals(&self) -> bool {
        matches!(
            (
                &self.tokens[self.index].kind,
                self.tokens.get(self.index + 1).map(|token| &token.kind)
            ),
            (TokenKind::Identifier(_), Some(TokenKind::Equals))
        )
    }

    fn at_eof(&self) -> bool {
        matches!(self.peek_kind(), TokenKind::Eof)
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.tokens[self.index].kind
    }

    fn current_span(&self) -> SourceSpan {
        self.tokens[self.index].span.clone()
    }

    fn previous_span(&self) -> SourceSpan {
        self.tokens[self.index.saturating_sub(1)].span.clone()
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.index].clone();
        self.index += 1;
        token
    }

    fn error_here(&self, code: &str, message: &str) -> TextualMirDiagnostic {
        TextualMirDiagnostic {
            code: code.to_string(),
            message: message.to_string(),
            span: self.current_span(),
        }
    }

    fn error_previous(&self, code: &str, message: &str) -> TextualMirDiagnostic {
        TextualMirDiagnostic {
            code: code.to_string(),
            message: message.to_string(),
            span: self.previous_span(),
        }
    }
}

fn keyword_name(keyword: Keyword) -> &'static str {
    match keyword {
        Keyword::And => "and",
        Keyword::At => "at",
        Keyword::Capability => "capability",
        Keyword::Effect => "effect",
        Keyword::Else => "else",
        Keyword::Ensure => "ensure",
        Keyword::Failure => "failure",
        Keyword::False => "false",
        Keyword::Fn => "fn",
        Keyword::For => "for",
        Keyword::If => "if",
        Keyword::Import => "import",
        Keyword::In => "in",
        Keyword::Let => "let",
        Keyword::Module => "module",
        Keyword::Mut => "mut",
        Keyword::Not => "not",
        Keyword::Or => "or",
        Keyword::Output => "output",
        Keyword::Perform => "perform",
        Keyword::Record => "record",
        Keyword::Require => "require",
        Keyword::Requires => "requires",
        Keyword::Return => "return",
        Keyword::Transition => "transition",
        Keyword::True => "true",
        Keyword::Via => "via",
        Keyword::While => "while",
    }
}

fn span_from(start: SourceSpan, end: SourceSpan) -> SourceSpan {
    SourceSpan {
        start: start.start,
        end: end.end,
        line: start.line,
        column: start.column,
    }
}

fn top_level_span_end(item: &AstTopLevel) -> usize {
    match item {
        AstTopLevel::Import(import) => import.span.end,
        AstTopLevel::Capability(capability) => capability.span.end,
        AstTopLevel::Record(record) => record.span.end,
        AstTopLevel::Effect(effect) => effect.span.end,
        AstTopLevel::Function(function) => function.span.end,
        AstTopLevel::Transition(transition) => transition.span.end,
    }
}

fn validate_imports(module: &AstModule, current_path: &Path) -> Vec<TextualMirDiagnostic> {
    module
        .imports
        .iter()
        .filter_map(|import| {
            match resolve_textual_mir_module_reference(current_path, &import.module_path) {
                TextualMirModuleResolution::Missing => Some(TextualMirDiagnostic {
                    code: "unresolved_import".to_string(),
                    message: format!(
                        "could not resolve import `{}` from `{}`",
                        import.module_path,
                        current_path.display()
                    ),
                    span: import.span.clone(),
                }),
                TextualMirModuleResolution::Ambiguous(paths) => Some(TextualMirDiagnostic {
                    code: "ambiguous_import_resolution".to_string(),
                    message: format!(
                        "import `{}` resolves to multiple declared textual Mir modules: {}",
                        import.module_path,
                        paths
                            .iter()
                            .map(|path| path.display().to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    span: import.span.clone(),
                }),
                TextualMirModuleResolution::Unique(_) => None,
            }
        })
        .collect()
}

fn find_import_search_root(path: &Path) -> Option<PathBuf> {
    for ancestor in path.ancestors() {
        if ancestor.join("matrix.json").exists() {
            return Some(ancestor.to_path_buf());
        }
    }
    let parent = path.parent()?;
    if parent.file_name().and_then(|name| name.to_str()) == Some("src") {
        return parent
            .parent()
            .map(Path::to_path_buf)
            .or_else(|| Some(parent.to_path_buf()));
    }
    Some(parent.to_path_buf())
}

fn find_declared_module_paths(root: &Path, module_path: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_mir_files(root, &mut files);
    let mut matches = Vec::new();
    for file in files {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };
        let report = parse_textual_mir_report(&source);
        if report
            .module
            .as_ref()
            .map(|module| module.module_path == module_path)
            .unwrap_or(false)
        {
            matches.push(file);
        }
    }
    matches
}

fn collect_mir_files(root: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_mir_files(&path, out);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("mir") {
            out.push(path);
        }
    }
}
