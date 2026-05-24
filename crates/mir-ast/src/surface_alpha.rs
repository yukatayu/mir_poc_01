use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::textual_alpha::{SourceSpan, TextualMirDiagnostic};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceMirParseReport {
    pub accepted: bool,
    pub module: Option<SurfaceModule>,
    pub diagnostics: Vec<TextualMirDiagnostic>,
    pub canonical_place_scope_syntax: String,
    pub final_public_grammar_frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceModule {
    pub module_path: String,
    pub imports: Vec<SurfaceImport>,
    pub capabilities: Vec<SurfaceCapabilityDecl>,
    pub roles: Vec<SurfaceRoleDecl>,
    pub principals: Vec<SurfacePrincipalDecl>,
    pub places: Vec<SurfacePlaceDecl>,
    pub records: Vec<SurfaceRecordDecl>,
    pub functions: Vec<SurfaceFunctionStub>,
    pub place_blocks: Vec<SurfacePlaceBlock>,
    pub role_instance_blocks: Vec<SurfaceRoleInstanceBlock>,
    pub items: Vec<SurfaceTopLevel>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfaceTopLevel {
    Import(SurfaceImport),
    Capability(SurfaceCapabilityDecl),
    Role(SurfaceRoleDecl),
    Principal(SurfacePrincipalDecl),
    Place(SurfacePlaceDecl),
    Record(SurfaceRecordDecl),
    Function(SurfaceFunctionStub),
    PlaceBlock(SurfacePlaceBlock),
    RoleInstanceBlock(SurfaceRoleInstanceBlock),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceImport {
    pub module_path: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceCapabilityDecl {
    pub capability_name: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRoleDecl {
    pub role_name: String,
    pub supports: Vec<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfacePrincipalDecl {
    pub principal_name: String,
    pub principal_type: Option<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfacePlaceDecl {
    pub place_name: String,
    pub place_type: Option<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRecordDecl {
    pub record_name: String,
    pub fields: Vec<SurfaceRecordField>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRecordField {
    pub field_name: String,
    pub field_type_text: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceFunctionStub {
    pub function_name: String,
    pub source_text: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfacePlaceBlock {
    pub place_ref: String,
    pub items: Vec<SurfacePlaceItem>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfacePlaceItem {
    State(SurfaceStateDecl),
    When(SurfaceWhenBlock),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceStateDecl {
    pub state_name: String,
    pub owner_place: String,
    pub index: Option<SurfaceStateIndex>,
    pub value_type_text: String,
    pub initial_value: Option<SurfaceExpr>,
    pub visible: Option<SurfaceVisibilityDecl>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceStateIndex {
    pub name: String,
    pub key_type_text: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceVisibilityDecl {
    pub channel: String,
    pub fields: Vec<String>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRoleInstanceBlock {
    pub role_ref: String,
    pub instance_ref: String,
    pub whens: Vec<SurfaceWhenBlock>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceWhenBlock {
    pub event_name: String,
    pub parameters: Vec<SurfaceParam>,
    pub failure_row: Vec<String>,
    pub body: Vec<SurfaceStmt>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceParam {
    pub name: String,
    pub type_text: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfaceStmt {
    Join(SurfaceJoinStmt),
    Require(SurfaceRawStmt),
    Grant(SurfaceRawStmt),
    Publish(SurfaceRawStmt),
    Assign(SurfaceAssignStmt),
    NestedPlaceBlock(SurfaceNestedPlaceBlock),
    Raw(SurfaceRawStmt),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceJoinStmt {
    pub target_place: String,
    pub role_ref: String,
    pub admission_place: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceAssignStmt {
    pub target_text: String,
    pub value: SurfaceExpr,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceNestedPlaceBlock {
    pub place_ref: String,
    pub body: Vec<SurfaceStmt>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRawStmt {
    pub text: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceExpr {
    pub text: String,
    pub kind: SurfaceExprKind,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SurfaceExprKind {
    IntLiteral(i64),
    BoolLiteral(bool),
    TextLiteral(String),
    RecordLiteral {
        record_name: String,
        fields: Vec<SurfaceRecordLiteralField>,
    },
    ArrayLiteral,
    Variable(String),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceRecordLiteralField {
    pub field_name: String,
    pub value: SurfaceExpr,
    pub span: SourceSpan,
}

pub fn parse_surface_mir_module(source: &str) -> Result<SurfaceModule, Vec<TextualMirDiagnostic>> {
    let report = parse_surface_mir_report(source);
    match report.module {
        Some(module) => Ok(module),
        None => Err(report.diagnostics),
    }
}

pub fn parse_surface_mir_report(source: &str) -> SurfaceMirParseReport {
    match Lexer::new(source)
        .tokenize()
        .and_then(|tokens| Parser::new(tokens).parse_module())
    {
        Ok(module) => SurfaceMirParseReport {
            accepted: true,
            module: Some(module),
            diagnostics: Vec::new(),
            canonical_place_scope_syntax: "S { ... }".to_string(),
            final_public_grammar_frozen: false,
        },
        Err(diagnostic) => SurfaceMirParseReport {
            accepted: false,
            module: None,
            diagnostics: vec![diagnostic],
            canonical_place_scope_syntax: "S { ... }".to_string(),
            final_public_grammar_frozen: false,
        },
    }
}

pub fn parse_surface_mir_report_path(path: impl AsRef<Path>) -> SurfaceMirParseReport {
    match std::fs::read_to_string(path.as_ref()) {
        Ok(source) => parse_surface_mir_report(&source),
        Err(error) => SurfaceMirParseReport {
            accepted: false,
            module: None,
            diagnostics: vec![TextualMirDiagnostic {
                code: "io_error".to_string(),
                message: format!("failed to read {}: {error}", path.as_ref().display()),
                span: zero_span(),
            }],
            canonical_place_scope_syntax: "S { ... }".to_string(),
            final_public_grammar_frozen: false,
        },
    }
}

type ParseResult<T> = Result<T, TextualMirDiagnostic>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Keyword {
    As,
    Capability,
    Fails,
    Fields,
    Fn,
    Grant,
    Import,
    Init,
    Join,
    Module,
    Place,
    Principal,
    Produces,
    Publish,
    Record,
    Require,
    Role,
    State,
    Supports,
    True,
    False,
    Via,
    Visible,
    When,
    Witness,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokenKind {
    Identifier(String),
    Integer(String),
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
    Equals,
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
    text: String,
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
                        text: "\n".to_string(),
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
                '0'..='9' => tokens.push(self.lex_number()),
                'A'..='Z' | 'a'..='z' | '_' => tokens.push(self.lex_identifier_or_keyword()),
                '{' => tokens.push(self.single_char(TokenKind::LeftBrace, "{")),
                '}' => tokens.push(self.single_char(TokenKind::RightBrace, "}")),
                '(' => tokens.push(self.single_char(TokenKind::LeftParen, "(")),
                ')' => tokens.push(self.single_char(TokenKind::RightParen, ")")),
                '[' => tokens.push(self.single_char(TokenKind::LeftBracket, "[")),
                ']' => tokens.push(self.single_char(TokenKind::RightBracket, "]")),
                ':' => tokens.push(self.single_char(TokenKind::Colon, ":")),
                ',' => tokens.push(self.single_char(TokenKind::Comma, ",")),
                '.' => tokens.push(self.single_char(TokenKind::Dot, ".")),
                ';' => tokens.push(self.single_char(TokenKind::Semicolon, ";")),
                '=' => tokens.push(self.single_char(TokenKind::Equals, "=")),
                '+' => tokens.push(self.single_char(TokenKind::Plus, "+")),
                '-' => tokens.push(self.single_char(TokenKind::Minus, "-")),
                '*' => tokens.push(self.single_char(TokenKind::Star, "*")),
                '/' => tokens.push(self.single_char(TokenKind::Slash, "/")),
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
            text: String::new(),
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
                    kind: TokenKind::String(value.clone()),
                    span: SourceSpan {
                        start,
                        end: self.index,
                        line,
                        column,
                    },
                    text: format!("\"{value}\""),
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

    fn lex_number(&mut self) -> Token {
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
        Token {
            kind: TokenKind::Integer(text.clone()),
            span: SourceSpan {
                start,
                end: self.index,
                line,
                column,
            },
            text,
        }
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
            "as" => TokenKind::Keyword(Keyword::As),
            "capability" => TokenKind::Keyword(Keyword::Capability),
            "fails" => TokenKind::Keyword(Keyword::Fails),
            "fields" => TokenKind::Keyword(Keyword::Fields),
            "fn" => TokenKind::Keyword(Keyword::Fn),
            "grant" => TokenKind::Keyword(Keyword::Grant),
            "import" => TokenKind::Keyword(Keyword::Import),
            "init" => TokenKind::Keyword(Keyword::Init),
            "join" => TokenKind::Keyword(Keyword::Join),
            "module" => TokenKind::Keyword(Keyword::Module),
            "place" => TokenKind::Keyword(Keyword::Place),
            "principal" => TokenKind::Keyword(Keyword::Principal),
            "produces" => TokenKind::Keyword(Keyword::Produces),
            "publish" => TokenKind::Keyword(Keyword::Publish),
            "record" => TokenKind::Keyword(Keyword::Record),
            "require" => TokenKind::Keyword(Keyword::Require),
            "role" => TokenKind::Keyword(Keyword::Role),
            "state" => TokenKind::Keyword(Keyword::State),
            "supports" => TokenKind::Keyword(Keyword::Supports),
            "true" => TokenKind::Keyword(Keyword::True),
            "false" => TokenKind::Keyword(Keyword::False),
            "via" => TokenKind::Keyword(Keyword::Via),
            "visible" => TokenKind::Keyword(Keyword::Visible),
            "when" => TokenKind::Keyword(Keyword::When),
            "witness" => TokenKind::Keyword(Keyword::Witness),
            _ => TokenKind::Identifier(text.clone()),
        };
        Token {
            kind,
            span: SourceSpan {
                start,
                end: self.index,
                line,
                column,
            },
            text,
        }
    }

    fn single_char(&mut self, kind: TokenKind, text: &str) -> Token {
        let span = self.current_span(1);
        self.bump_char();
        Token {
            kind,
            span,
            text: text.to_string(),
        }
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
    role_names: Vec<String>,
    place_names: Vec<String>,
    record_names: Vec<String>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            role_names: Vec::new(),
            place_names: Vec::new(),
            record_names: Vec::new(),
        }
    }

    fn parse_module(&mut self) -> ParseResult<SurfaceModule> {
        self.skip_statement_breaks();
        let start = self.expect_keyword(Keyword::Module)?.span;
        let module_path = self.parse_path()?;
        self.skip_statement_breaks();

        let mut imports = Vec::new();
        let mut capabilities = Vec::new();
        let mut roles = Vec::new();
        let mut principals = Vec::new();
        let mut places = Vec::new();
        let mut records = Vec::new();
        let mut functions = Vec::new();
        let mut place_blocks = Vec::new();
        let mut role_instance_blocks = Vec::new();
        let mut items = Vec::new();

        while !self.at_eof() {
            self.skip_statement_breaks();
            if self.at_eof() {
                break;
            }

            match self.peek_kind() {
                TokenKind::Keyword(Keyword::Import) => {
                    let item = self.parse_import()?;
                    imports.push(item.clone());
                    items.push(SurfaceTopLevel::Import(item));
                }
                TokenKind::Keyword(Keyword::Capability) => {
                    let item = self.parse_capability()?;
                    capabilities.push(item.clone());
                    items.push(SurfaceTopLevel::Capability(item));
                }
                TokenKind::Keyword(Keyword::Role) => {
                    let item = self.parse_role()?;
                    self.role_names.push(item.role_name.clone());
                    roles.push(item.clone());
                    items.push(SurfaceTopLevel::Role(item));
                }
                TokenKind::Keyword(Keyword::Principal) => {
                    let item = self.parse_principal()?;
                    principals.push(item.clone());
                    items.push(SurfaceTopLevel::Principal(item));
                }
                TokenKind::Keyword(Keyword::Place) => {
                    let item = self.parse_place()?;
                    self.place_names.push(item.place_name.clone());
                    places.push(item.clone());
                    items.push(SurfaceTopLevel::Place(item));
                }
                TokenKind::Keyword(Keyword::Record) => {
                    let item = self.parse_record()?;
                    self.record_names.push(item.record_name.clone());
                    records.push(item.clone());
                    items.push(SurfaceTopLevel::Record(item));
                }
                TokenKind::Keyword(Keyword::Fn) => {
                    let item = self.parse_function_stub()?;
                    functions.push(item.clone());
                    items.push(SurfaceTopLevel::Function(item));
                }
                TokenKind::Identifier(_) => {
                    let item = self.parse_brace_head_top_level()?;
                    match item {
                        SurfaceTopLevel::PlaceBlock(block) => {
                            place_blocks.push(block.clone());
                            items.push(SurfaceTopLevel::PlaceBlock(block));
                        }
                        SurfaceTopLevel::RoleInstanceBlock(block) => {
                            role_instance_blocks.push(block.clone());
                            items.push(SurfaceTopLevel::RoleInstanceBlock(block));
                        }
                        _ => unreachable!("brace head parser only returns block items"),
                    }
                }
                _ => {
                    return Err(self.error_here(
                        "unexpected_surface_top_level_item",
                        "expected import, capability, role, principal, place, record, fn, place block, or role-instance block",
                    ));
                }
            }
            self.skip_statement_breaks();
        }

        let end = items
            .last()
            .map(surface_top_level_span_end)
            .unwrap_or(start.end);
        Ok(SurfaceModule {
            module_path,
            imports,
            capabilities,
            roles,
            principals,
            places,
            records,
            functions,
            place_blocks,
            role_instance_blocks,
            items,
            span: span_from(start, end_span(end)),
        })
    }

    fn parse_import(&mut self) -> ParseResult<SurfaceImport> {
        let start = self.expect_keyword(Keyword::Import)?.span;
        let module_path = self.parse_path()?;
        Ok(SurfaceImport {
            module_path,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_capability(&mut self) -> ParseResult<SurfaceCapabilityDecl> {
        let start = self.expect_keyword(Keyword::Capability)?.span;
        let capability_name = self.parse_identifier()?;
        Ok(SurfaceCapabilityDecl {
            capability_name,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_role(&mut self) -> ParseResult<SurfaceRoleDecl> {
        let start = self.expect_keyword(Keyword::Role)?.span;
        let role_name = self.parse_identifier()?;
        let mut supports = Vec::new();
        if self.consume_if(&TokenKind::LeftBrace) {
            self.skip_statement_breaks();
            while !self.check(&TokenKind::RightBrace) {
                self.expect_keyword(Keyword::Supports)?;
                supports.push(self.parse_path()?);
                self.skip_statement_breaks();
            }
            self.expect(
                TokenKind::RightBrace,
                "expected_right_brace",
                "expected `}` after role body",
            )?;
        }
        Ok(SurfaceRoleDecl {
            role_name,
            supports,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_principal(&mut self) -> ParseResult<SurfacePrincipalDecl> {
        let start = self.expect_keyword(Keyword::Principal)?.span;
        let principal_name = self.parse_identifier()?;
        let principal_type = if self.consume_if(&TokenKind::Colon) {
            Some(self.parse_path()?)
        } else {
            None
        };
        Ok(SurfacePrincipalDecl {
            principal_name,
            principal_type,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_place(&mut self) -> ParseResult<SurfacePlaceDecl> {
        let start = self.expect_keyword(Keyword::Place)?.span;
        let place_name = self.parse_identifier()?;
        let place_type = if self.consume_if(&TokenKind::Colon) {
            Some(self.parse_path()?)
        } else {
            None
        };
        Ok(SurfacePlaceDecl {
            place_name,
            place_type,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_record(&mut self) -> ParseResult<SurfaceRecordDecl> {
        let start = self.expect_keyword(Keyword::Record)?.span;
        let record_name = self.parse_identifier()?;
        self.expect(
            TokenKind::LeftBrace,
            "expected_left_brace",
            "expected `{` after record name",
        )?;
        self.skip_statement_breaks();
        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) {
            let field_start = self.current_span();
            let field_name = self.parse_identifier()?;
            self.expect(
                TokenKind::Colon,
                "expected_colon",
                "expected `:` after record field name",
            )?;
            let field_type_text = self.parse_type_text()?;
            fields.push(SurfaceRecordField {
                field_name,
                field_type_text,
                span: span_from(field_start, self.previous_span()),
            });
            self.consume_if(&TokenKind::Comma);
            self.skip_statement_breaks();
        }
        self.expect(
            TokenKind::RightBrace,
            "expected_right_brace",
            "expected `}` after record fields",
        )?;
        Ok(SurfaceRecordDecl {
            record_name,
            fields,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_function_stub(&mut self) -> ParseResult<SurfaceFunctionStub> {
        let start = self.expect_keyword(Keyword::Fn)?.span;
        let function_name = self.parse_identifier()?;
        let mut body_depth = 0usize;
        let mut saw_body = false;
        while !self.at_eof() {
            if self.consume_if(&TokenKind::LeftBrace) {
                saw_body = true;
                body_depth += 1;
                break;
            }
            self.advance();
        }
        if !saw_body {
            return Err(self.error_here("expected_left_brace", "expected function body"));
        }
        while !self.at_eof() && body_depth > 0 {
            if self.consume_if(&TokenKind::LeftBrace) {
                body_depth += 1;
                continue;
            }
            if self.consume_if(&TokenKind::RightBrace) {
                body_depth -= 1;
                continue;
            }
            self.advance();
        }
        Ok(SurfaceFunctionStub {
            function_name,
            source_text: "<surface-function-stub>".to_string(),
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_brace_head_top_level(&mut self) -> ParseResult<SurfaceTopLevel> {
        let start = self.current_span();
        let head = self.parse_path()?;
        if self.consume_if(&TokenKind::LeftBracket) {
            if self.is_declared_role_path(&head) && self.is_declared_place_path(&head) {
                return Err(self.error_here(
                    "ambiguous_brace_construct",
                    "brace head resolves as both role and place path",
                ));
            }
            if self.is_declared_role_path(&head) {
                let instance_ref = self.parse_role_instance_binder()?;
                let block = self.parse_role_instance_block(head, instance_ref, start)?;
                return Ok(SurfaceTopLevel::RoleInstanceBlock(block));
            }
            if self.is_declared_place_path(&head) || head == "S" {
                return Err(TextualMirDiagnostic {
                    code: "bracket_place_scope_not_supported".to_string(),
                    message: "use `S { ... }`; `[]` is reserved for indexing.".to_string(),
                    span: span_from(start, self.previous_span()),
                });
            }
            return Err(self.error_here(
                "undeclared_role_instance_head",
                "role-instance block head must resolve to a declared role path",
            ));
        }
        if self.check(&TokenKind::LeftBrace) {
            let resolves_as_role = self.is_declared_role_path(&head);
            let resolves_as_place = self.is_declared_place_path(&head);
            let resolves_as_record = self.is_declared_record_path(&head);
            if resolves_as_place && (resolves_as_role || resolves_as_record) {
                return Err(self.error_here(
                    "ambiguous_brace_construct",
                    "brace head resolves to multiple namespace entries",
                ));
            }
            if resolves_as_role {
                return Err(self.error_here(
                    "bare_role_block_not_supported",
                    "use `Role[instance] { ... }` for role-instance blocks",
                ));
            }
            if resolves_as_place {
                let block = self.parse_place_block(head, start)?;
                return Ok(SurfaceTopLevel::PlaceBlock(block));
            }
            return Err(self.error_here(
                "undeclared_place_block_head",
                "place block head must resolve to a declared place path",
            ));
        }
        Err(self.error_here(
            "expected_surface_block",
            "expected `{` place block or `[instance] {` role-instance block",
        ))
    }

    fn parse_role_instance_binder(&mut self) -> ParseResult<String> {
        let start = self.current_span();
        if self.check(&TokenKind::RightBracket) {
            return Err(self.error_here(
                "invalid_role_instance_binder",
                "role-instance binder must be a non-empty principal path",
            ));
        }
        let instance_ref = self.parse_path().map_err(|_| TextualMirDiagnostic {
            code: "invalid_role_instance_binder".to_string(),
            message: "role-instance binder must be a non-empty principal path".to_string(),
            span: start.clone(),
        })?;
        if !self.check(&TokenKind::RightBracket) {
            return Err(TextualMirDiagnostic {
                code: "invalid_role_instance_binder".to_string(),
                message: "role-instance binder must be a non-empty principal path".to_string(),
                span: span_from(start, self.current_span()),
            });
        }
        self.expect(
            TokenKind::RightBracket,
            "expected_right_bracket",
            "expected `]` after role-instance binder",
        )?;
        Ok(instance_ref)
    }

    fn is_declared_role_path(&self, path: &str) -> bool {
        self.role_names.iter().any(|name| name == path)
    }

    fn is_declared_place_path(&self, path: &str) -> bool {
        self.place_names.iter().any(|name| name == path)
    }

    fn is_declared_record_path(&self, path: &str) -> bool {
        self.record_names.iter().any(|name| name == path)
    }

    fn parse_role_instance_block(
        &mut self,
        role_ref: String,
        instance_ref: String,
        start: SourceSpan,
    ) -> ParseResult<SurfaceRoleInstanceBlock> {
        self.expect(
            TokenKind::LeftBrace,
            "expected_left_brace",
            "expected `{` after role-instance head",
        )?;
        self.skip_statement_breaks();
        let mut whens = Vec::new();
        while !self.check(&TokenKind::RightBrace) {
            whens.push(self.parse_when_block()?);
            self.skip_statement_breaks();
        }
        self.expect(
            TokenKind::RightBrace,
            "expected_right_brace",
            "expected `}` after role-instance block",
        )?;
        Ok(SurfaceRoleInstanceBlock {
            role_ref,
            instance_ref: instance_ref.trim().to_string(),
            whens,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_place_block(
        &mut self,
        place_ref: String,
        start: SourceSpan,
    ) -> ParseResult<SurfacePlaceBlock> {
        self.expect(
            TokenKind::LeftBrace,
            "expected_left_brace",
            "expected `{` after place block head",
        )?;
        self.skip_statement_breaks();
        let mut items = Vec::new();
        while !self.check(&TokenKind::RightBrace) {
            match self.peek_kind() {
                TokenKind::Keyword(Keyword::State) => {
                    items.push(SurfacePlaceItem::State(
                        self.parse_state_decl(place_ref.clone())?,
                    ));
                }
                TokenKind::Keyword(Keyword::When) => {
                    items.push(SurfacePlaceItem::When(self.parse_when_block()?));
                }
                _ => {
                    return Err(self.error_here(
                        "unexpected_place_block_item",
                        "expected state declaration or when block in place block",
                    ));
                }
            }
            self.skip_statement_breaks();
        }
        self.expect(
            TokenKind::RightBrace,
            "expected_right_brace",
            "expected `}` after place block",
        )?;
        Ok(SurfacePlaceBlock {
            place_ref,
            items,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_state_decl(&mut self, owner_place: String) -> ParseResult<SurfaceStateDecl> {
        let start = self.expect_keyword(Keyword::State)?.span;
        let state_name = self.parse_identifier()?;
        let index = if self.consume_if(&TokenKind::LeftBracket) {
            let index_start = self.current_span();
            let name = self.parse_identifier()?;
            self.expect(
                TokenKind::Colon,
                "expected_colon",
                "expected `:` after indexed state key name",
            )?;
            let key_type_text = self.parse_type_text()?;
            self.expect(
                TokenKind::RightBracket,
                "expected_right_bracket",
                "expected `]` after indexed state key",
            )?;
            Some(SurfaceStateIndex {
                name,
                key_type_text,
                span: span_from(index_start, self.previous_span()),
            })
        } else {
            None
        };
        self.expect(
            TokenKind::Colon,
            "expected_colon",
            "expected `:` before state value type",
        )?;
        let value_type_text = self.parse_type_text()?;
        let mut initial_value = None;
        let mut visible = None;
        self.skip_statement_breaks();
        loop {
            if self.consume_keyword(Keyword::Init) {
                initial_value = Some(self.parse_expr_until_statement_break()?);
                self.skip_statement_breaks();
                continue;
            }
            if self.check_keyword(Keyword::Visible) {
                visible = Some(self.parse_visibility_decl()?);
                self.skip_statement_breaks();
                continue;
            }
            break;
        }
        Ok(SurfaceStateDecl {
            state_name,
            owner_place,
            index,
            value_type_text,
            initial_value,
            visible,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_visibility_decl(&mut self) -> ParseResult<SurfaceVisibilityDecl> {
        let start = self.expect_keyword(Keyword::Visible)?.span;
        let channel = self.parse_path()?;
        let mut fields = Vec::new();
        if self.consume_keyword(Keyword::Fields) {
            self.expect(
                TokenKind::LeftBrace,
                "expected_left_brace",
                "expected `{` after visible fields",
            )?;
            self.skip_statement_breaks();
            while !self.check(&TokenKind::RightBrace) {
                fields.push(self.parse_identifier()?);
                self.consume_if(&TokenKind::Comma);
                self.skip_statement_breaks();
            }
            self.expect(
                TokenKind::RightBrace,
                "expected_right_brace",
                "expected `}` after visible fields",
            )?;
        }
        Ok(SurfaceVisibilityDecl {
            channel,
            fields,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_when_block(&mut self) -> ParseResult<SurfaceWhenBlock> {
        let start = self.expect_keyword(Keyword::When)?.span;
        let event_name = self.parse_identifier()?;
        let parameters = if self.consume_if(&TokenKind::LeftParen) {
            self.parse_params_until_right_paren()?
        } else {
            Vec::new()
        };
        let failure_row = if self.consume_keyword(Keyword::Fails) {
            self.parse_identifier_list()?
        } else {
            Vec::new()
        };
        let body = self.parse_statement_block()?;
        Ok(SurfaceWhenBlock {
            event_name,
            parameters,
            failure_row,
            body,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_statement_block(&mut self) -> ParseResult<Vec<SurfaceStmt>> {
        self.expect(
            TokenKind::LeftBrace,
            "expected_left_brace",
            "expected `{` to begin when body",
        )?;
        self.skip_statement_breaks();
        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) {
            body.push(self.parse_statement()?);
            self.skip_statement_breaks();
        }
        self.expect(
            TokenKind::RightBrace,
            "expected_right_brace",
            "expected `}` after when body",
        )?;
        Ok(body)
    }

    fn parse_statement(&mut self) -> ParseResult<SurfaceStmt> {
        self.skip_statement_breaks();
        match self.peek_kind() {
            TokenKind::Keyword(Keyword::Join) => self.parse_join_stmt().map(SurfaceStmt::Join),
            TokenKind::Keyword(Keyword::Require) => self
                .parse_keyword_raw_stmt(Keyword::Require)
                .map(SurfaceStmt::Require),
            TokenKind::Keyword(Keyword::Grant) => self
                .parse_keyword_raw_stmt(Keyword::Grant)
                .map(SurfaceStmt::Grant),
            TokenKind::Keyword(Keyword::Publish) => self
                .parse_keyword_raw_stmt(Keyword::Publish)
                .map(SurfaceStmt::Publish),
            TokenKind::Identifier(_) => {
                if let Some(head) = self.lookahead_brace_block_head() {
                    if self.is_declared_place_path(&head) {
                        self.parse_nested_place_block()
                            .map(SurfaceStmt::NestedPlaceBlock)
                    } else {
                        Err(self.error_here(
                            "undeclared_place_block_head",
                            "nested place block head must resolve to a declared place path",
                        ))
                    }
                } else if self.statement_contains_equals_before_break() {
                    self.parse_assignment_stmt().map(SurfaceStmt::Assign)
                } else {
                    self.parse_raw_stmt().map(SurfaceStmt::Raw)
                }
            }
            _ => self.parse_raw_stmt().map(SurfaceStmt::Raw),
        }
    }

    fn parse_join_stmt(&mut self) -> ParseResult<SurfaceJoinStmt> {
        let start = self.expect_keyword(Keyword::Join)?.span;
        let target_place = self.parse_path()?;
        self.expect_keyword(Keyword::As)?;
        let role_ref = self.parse_path()?;
        self.expect_keyword(Keyword::Via)?;
        let admission_place = self.parse_path()?;
        Ok(SurfaceJoinStmt {
            target_place,
            role_ref,
            admission_place,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_nested_place_block(&mut self) -> ParseResult<SurfaceNestedPlaceBlock> {
        let start = self.current_span();
        let place_ref = self.parse_path()?;
        self.expect(
            TokenKind::LeftBrace,
            "expected_left_brace",
            "expected `{` after nested place block head",
        )?;
        self.skip_statement_breaks();
        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) {
            body.push(self.parse_statement()?);
            self.skip_statement_breaks();
        }
        self.expect(
            TokenKind::RightBrace,
            "expected_right_brace",
            "expected `}` after nested place block",
        )?;
        Ok(SurfaceNestedPlaceBlock {
            place_ref,
            body,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_assignment_stmt(&mut self) -> ParseResult<SurfaceAssignStmt> {
        let start = self.current_span();
        let target_tokens = self.collect_tokens_until_equals()?;
        self.expect(
            TokenKind::Equals,
            "expected_equals",
            "expected `=` in assignment statement",
        )?;
        let value = self.parse_expr_until_statement_break()?;
        Ok(SurfaceAssignStmt {
            target_text: tokens_to_text(&target_tokens),
            value,
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_keyword_raw_stmt(&mut self, keyword: Keyword) -> ParseResult<SurfaceRawStmt> {
        let start = self.expect_keyword(keyword)?.span;
        let tokens = self.collect_tokens_until_statement_break();
        Ok(SurfaceRawStmt {
            text: tokens_to_text(&tokens),
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_raw_stmt(&mut self) -> ParseResult<SurfaceRawStmt> {
        let start = self.current_span();
        let tokens = self.collect_tokens_until_statement_break();
        Ok(SurfaceRawStmt {
            text: tokens_to_text(&tokens),
            span: span_from(start, self.previous_span()),
        })
    }

    fn parse_params_until_right_paren(&mut self) -> ParseResult<Vec<SurfaceParam>> {
        let mut params = Vec::new();
        if self.check(&TokenKind::RightParen) {
            self.advance();
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
            let type_text = self.parse_type_text()?;
            params.push(SurfaceParam {
                name,
                type_text,
                span: span_from(start, self.previous_span()),
            });
            if !self.consume_if(&TokenKind::Comma) {
                break;
            }
        }
        self.expect(
            TokenKind::RightParen,
            "expected_right_paren",
            "expected `)` after parameters",
        )?;
        Ok(params)
    }

    fn parse_expr_until_statement_break(&mut self) -> ParseResult<SurfaceExpr> {
        let start = self.current_span();
        let tokens = self.collect_tokens_until_statement_break();
        if tokens.is_empty() {
            return Err(self.error_here("expected_expression", "expected expression"));
        }
        let span = span_from(
            start,
            tokens.last().expect("non-empty token list").span.clone(),
        );
        Ok(expr_from_tokens(tokens, span))
    }

    fn parse_type_text(&mut self) -> ParseResult<String> {
        if self.consume_if(&TokenKind::LeftBracket) {
            let element = self.parse_type_text()?;
            self.expect(
                TokenKind::Semicolon,
                "expected_semicolon",
                "expected `;` in fixed array type",
            )?;
            let length = self.expect_integer()?;
            self.expect(
                TokenKind::RightBracket,
                "expected_right_bracket",
                "expected `]` after fixed array type",
            )?;
            return Ok(format!("[{element}; {length}]"));
        }
        self.parse_path()
    }

    fn parse_path(&mut self) -> ParseResult<String> {
        let mut segments = vec![self.parse_identifier()?];
        while self.consume_if(&TokenKind::Dot) {
            segments.push(self.parse_identifier()?);
        }
        Ok(segments.join("."))
    }

    fn parse_identifier_list(&mut self) -> ParseResult<Vec<String>> {
        let mut items = vec![self.parse_identifier()?];
        while self.consume_if(&TokenKind::Comma) {
            items.push(self.parse_identifier()?);
        }
        Ok(items)
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

    fn collect_tokens_until_equals(&mut self) -> ParseResult<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut square_depth = 0usize;
        let mut paren_depth = 0usize;
        while !self.at_eof() {
            match self.peek_kind() {
                TokenKind::Equals if square_depth == 0 && paren_depth == 0 => break,
                TokenKind::LeftBracket => {
                    square_depth += 1;
                    tokens.push(self.advance());
                }
                TokenKind::RightBracket => {
                    square_depth = square_depth.saturating_sub(1);
                    tokens.push(self.advance());
                }
                TokenKind::LeftParen => {
                    paren_depth += 1;
                    tokens.push(self.advance());
                }
                TokenKind::RightParen => {
                    paren_depth = paren_depth.saturating_sub(1);
                    tokens.push(self.advance());
                }
                TokenKind::Newline | TokenKind::Semicolon | TokenKind::RightBrace => break,
                _ => tokens.push(self.advance()),
            }
        }
        if tokens.is_empty() {
            Err(self.error_here("expected_assignment_target", "expected assignment target"))
        } else {
            Ok(tokens)
        }
    }

    fn collect_tokens_until_statement_break(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut brace_depth = 0usize;
        let mut square_depth = 0usize;
        let mut paren_depth = 0usize;
        while !self.at_eof() {
            match self.peek_kind() {
                TokenKind::Newline | TokenKind::Semicolon
                    if brace_depth == 0 && square_depth == 0 && paren_depth == 0 =>
                {
                    break;
                }
                TokenKind::RightBrace
                    if brace_depth == 0 && square_depth == 0 && paren_depth == 0 =>
                {
                    break;
                }
                TokenKind::LeftBrace => {
                    brace_depth += 1;
                    tokens.push(self.advance());
                }
                TokenKind::RightBrace => {
                    brace_depth = brace_depth.saturating_sub(1);
                    tokens.push(self.advance());
                }
                TokenKind::LeftBracket => {
                    square_depth += 1;
                    tokens.push(self.advance());
                }
                TokenKind::RightBracket => {
                    square_depth = square_depth.saturating_sub(1);
                    tokens.push(self.advance());
                }
                TokenKind::LeftParen => {
                    paren_depth += 1;
                    tokens.push(self.advance());
                }
                TokenKind::RightParen => {
                    paren_depth = paren_depth.saturating_sub(1);
                    tokens.push(self.advance());
                }
                _ => tokens.push(self.advance()),
            }
        }
        tokens
    }

    fn lookahead_brace_block_head(&self) -> Option<String> {
        let mut index = self.index;
        let mut segments = match &self.tokens[index].kind {
            TokenKind::Identifier(name) => vec![name.clone()],
            _ => return None,
        };
        index += 1;
        while matches!(
            self.tokens.get(index).map(|token| &token.kind),
            Some(TokenKind::Dot)
        ) {
            index += 1;
            match self.tokens.get(index).map(|token| &token.kind) {
                Some(TokenKind::Identifier(name)) => segments.push(name.clone()),
                _ => return None,
            }
            index += 1;
        }
        if matches!(
            self.tokens.get(index).map(|token| &token.kind),
            Some(TokenKind::LeftBrace)
        ) {
            Some(segments.join("."))
        } else {
            None
        }
    }

    fn statement_contains_equals_before_break(&self) -> bool {
        let mut index = self.index;
        let mut square_depth = 0usize;
        let mut paren_depth = 0usize;
        while let Some(token) = self.tokens.get(index) {
            match &token.kind {
                TokenKind::Equals if square_depth == 0 && paren_depth == 0 => return true,
                TokenKind::Newline | TokenKind::Semicolon | TokenKind::RightBrace => return false,
                TokenKind::LeftBracket => square_depth += 1,
                TokenKind::RightBracket => square_depth = square_depth.saturating_sub(1),
                TokenKind::LeftParen => paren_depth += 1,
                TokenKind::RightParen => paren_depth = paren_depth.saturating_sub(1),
                TokenKind::Eof => return false,
                _ => {}
            }
            index += 1;
        }
        false
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

    fn skip_statement_breaks(&mut self) {
        while self.consume_if(&TokenKind::Newline) || self.consume_if(&TokenKind::Semicolon) {}
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
                | (Equals, Equals)
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

fn expr_from_tokens(tokens: Vec<Token>, span: SourceSpan) -> SurfaceExpr {
    let text = tokens_to_text(&tokens);
    let kind = match tokens.as_slice() {
        [
            Token {
                kind: TokenKind::Integer(value),
                ..
            },
        ] => value
            .parse::<i64>()
            .map(SurfaceExprKind::IntLiteral)
            .unwrap_or(SurfaceExprKind::Unknown),
        [
            Token {
                kind: TokenKind::String(value),
                ..
            },
        ] => SurfaceExprKind::TextLiteral(value.clone()),
        [
            Token {
                kind: TokenKind::Keyword(Keyword::True),
                ..
            },
        ] => SurfaceExprKind::BoolLiteral(true),
        [
            Token {
                kind: TokenKind::Keyword(Keyword::False),
                ..
            },
        ] => SurfaceExprKind::BoolLiteral(false),
        [
            Token {
                kind: TokenKind::Identifier(value),
                ..
            },
        ] => SurfaceExprKind::Variable(value.clone()),
        _ => parse_record_literal_tokens(&tokens).unwrap_or(SurfaceExprKind::Unknown),
    };
    SurfaceExpr { text, kind, span }
}

fn parse_record_literal_tokens(tokens: &[Token]) -> Option<SurfaceExprKind> {
    let left_brace = tokens
        .iter()
        .position(|token| matches!(token.kind, TokenKind::LeftBrace))?;
    if !matches!(
        tokens.last().map(|token| &token.kind),
        Some(TokenKind::RightBrace)
    ) {
        return None;
    }
    let record_name = tokens_to_text(&tokens[..left_brace]);
    if record_name.is_empty() {
        return None;
    }

    let mut fields = Vec::new();
    let mut index = left_brace + 1;
    while index < tokens.len().saturating_sub(1) {
        while matches!(tokens[index].kind, TokenKind::Comma | TokenKind::Newline) {
            index += 1;
        }
        if index >= tokens.len().saturating_sub(1) {
            break;
        }
        let field_start = tokens[index].span.clone();
        let field_name = match &tokens[index].kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => return None,
        };
        index += 1;
        if !matches!(
            tokens.get(index).map(|token| &token.kind),
            Some(TokenKind::Colon)
        ) {
            return None;
        }
        index += 1;
        let value_start = index;
        let mut brace_depth = 0usize;
        let mut square_depth = 0usize;
        let mut paren_depth = 0usize;
        while index < tokens.len().saturating_sub(1) {
            match &tokens[index].kind {
                TokenKind::Comma if brace_depth == 0 && square_depth == 0 && paren_depth == 0 => {
                    break;
                }
                TokenKind::LeftBrace => brace_depth += 1,
                TokenKind::RightBrace => {
                    if brace_depth == 0 {
                        break;
                    }
                    brace_depth -= 1;
                }
                TokenKind::LeftBracket => square_depth += 1,
                TokenKind::RightBracket => square_depth = square_depth.saturating_sub(1),
                TokenKind::LeftParen => paren_depth += 1,
                TokenKind::RightParen => paren_depth = paren_depth.saturating_sub(1),
                _ => {}
            }
            index += 1;
        }
        let value_tokens = tokens[value_start..index].to_vec();
        let value_end = value_tokens
            .last()
            .map(|token| token.span.clone())
            .unwrap_or_else(|| field_start.clone());
        let value = expr_from_tokens(value_tokens, span_from(field_start.clone(), value_end));
        fields.push(SurfaceRecordLiteralField {
            field_name,
            value,
            span: span_from(field_start, tokens[index.saturating_sub(1)].span.clone()),
        });
        if matches!(
            tokens.get(index).map(|token| &token.kind),
            Some(TokenKind::Comma)
        ) {
            index += 1;
        }
    }
    Some(SurfaceExprKind::RecordLiteral {
        record_name,
        fields,
    })
}

fn tokens_to_text(tokens: &[Token]) -> String {
    let mut out = String::new();
    let mut previous_was_word = false;
    for token in tokens {
        if matches!(token.kind, TokenKind::Newline | TokenKind::Eof) {
            continue;
        }
        let is_word = matches!(
            token.kind,
            TokenKind::Identifier(_)
                | TokenKind::Integer(_)
                | TokenKind::String(_)
                | TokenKind::Keyword(_)
        );
        let no_space_before = matches!(
            token.kind,
            TokenKind::RightBrace
                | TokenKind::RightBracket
                | TokenKind::RightParen
                | TokenKind::Comma
                | TokenKind::Dot
                | TokenKind::Colon
        );
        if !out.is_empty() && is_word && previous_was_word && !no_space_before {
            out.push(' ');
        }
        if !out.is_empty()
            && matches!(
                token.kind,
                TokenKind::LeftBrace | TokenKind::LeftBracket | TokenKind::LeftParen
            )
        {
            out.push(' ');
        }
        if !out.is_empty() && matches!(token.kind, TokenKind::Equals) {
            out.push(' ');
        }
        out.push_str(&token.text);
        previous_was_word = is_word;
    }
    out.trim().to_string()
}

fn keyword_name(keyword: Keyword) -> &'static str {
    match keyword {
        Keyword::As => "as",
        Keyword::Capability => "capability",
        Keyword::Fails => "fails",
        Keyword::Fields => "fields",
        Keyword::Fn => "fn",
        Keyword::Grant => "grant",
        Keyword::Import => "import",
        Keyword::Init => "init",
        Keyword::Join => "join",
        Keyword::Module => "module",
        Keyword::Place => "place",
        Keyword::Principal => "principal",
        Keyword::Produces => "produces",
        Keyword::Publish => "publish",
        Keyword::Record => "record",
        Keyword::Require => "require",
        Keyword::Role => "role",
        Keyword::State => "state",
        Keyword::Supports => "supports",
        Keyword::True => "true",
        Keyword::False => "false",
        Keyword::Via => "via",
        Keyword::Visible => "visible",
        Keyword::When => "when",
        Keyword::Witness => "witness",
    }
}

fn surface_top_level_span_end(item: &SurfaceTopLevel) -> usize {
    match item {
        SurfaceTopLevel::Import(item) => item.span.end,
        SurfaceTopLevel::Capability(item) => item.span.end,
        SurfaceTopLevel::Role(item) => item.span.end,
        SurfaceTopLevel::Principal(item) => item.span.end,
        SurfaceTopLevel::Place(item) => item.span.end,
        SurfaceTopLevel::Record(item) => item.span.end,
        SurfaceTopLevel::Function(item) => item.span.end,
        SurfaceTopLevel::PlaceBlock(item) => item.span.end,
        SurfaceTopLevel::RoleInstanceBlock(item) => item.span.end,
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

fn end_span(end: usize) -> SourceSpan {
    SourceSpan {
        start: end,
        end,
        line: 1,
        column: 1,
    }
}

fn zero_span() -> SourceSpan {
    SourceSpan {
        start: 0,
        end: 0,
        line: 1,
        column: 1,
    }
}
