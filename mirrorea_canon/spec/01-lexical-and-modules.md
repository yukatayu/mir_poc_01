---
id: spec/01-lexical-and-modules
status: L1-fixed
maturity: draft
depends_on: [adr/ADR-0021]
summary: M6 bounded Surface の字句、予約語、module header、source span の正本。
open_items: []
---

# 01 — M6 lexical structure and modules

This chapter defines the bounded M6 parser profile implemented by the current
fixture parser.  It is a reference grammar input to M7, not a final public
syntax or a general expression language.

- Encoding is UTF-8.  Whitespace separates tokens; only `//` line comments
  are accepted.  Block comments are not an M6 production.
- `Ident ::= [A-Za-z_][A-Za-z0-9_]*`; `ModulePath ::= Ident { "." Ident }`.
  Names have no authority, ownership, transport, or membership meaning by
  themselves.
- `IntLiteral ::= [0-9]+`.  `-` is a separate token, used by the bounded
  expression collector and `translate`.  Float, text, and generic literal
  syntax are not part of this parser profile.
- Punctuation is `, : = [ ] { } ( ) . + -`.  **Semicolons are not M6
  punctuation**: declarations, assignments, relation clauses, designated
  evaluation, and deferred forms have no terminating `;`.  Parentheses in
  `when h(...) fails(...)` are mandatory even when either list is empty.
- Grammar words are `module`, `locus`, `principal`, `type`, `state`, `Role`,
  `at`, `when`, `fails`, `relation`, `subject`, `primary`, `fallback`,
  `epoch`, `transform`, `translate`, `identity`, `bind`, `frontier`, `publish`,
  `value`, `project`, `local`, `designated`, `evaluate`, `on`, `tick`, `with`,
  `auth`, `verify`, and `mutate`.  The lexer first records them as identifiers;
  the bounded parser recognizes them only in the productions of spec/02.

`send`/`receive`, `occurrence`, and `envelope` are recognized before parsing
and produce their respective unsupported-syntax diagnostics.  `witness`,
`receipt`, and `release` have no M6 production or special lexical role; if
they occur where no `Ident` production is accepted they produce ordinary
unexpected-syntax rejection.  No M6 parser or classifier assigns any of them
communication, trace, or receipt behavior.

`Role`'s bracket token is not a general identifier binder in this profile.
`Role[self]` is the only accepted actor spelling; `Role[Name]` for a different
`Name` is the parser diagnostic `RoleActorMustBeLiteralSelf` at that actor
token's span, before semantic classification.

## Module header and spans

Every source file begins with:

```text
module Dotted.Path
```

There is one module per file.  Imports, package resolution, and file-system
layout are M7-or-later concerns and are not silently supplied by this grammar.
Every parser node has a file-qualified byte span and corresponding line/column
range.  Classification derives its M5 `SourceRef` from that span; no operation
key, trace occurrence, runtime location, or presentation context substitutes
for it.

The Core companion notation in spec/04 is not Surface syntax.  In particular,
`perform`, `option`, `try`, and `atomic_cut` are not M6 source productions.
