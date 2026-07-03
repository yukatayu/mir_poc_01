---
id: spec/01-lexical-and-modules
status: L1-fixed
maturity: draft
depends_on: [adr/ADR-0008]
summary: 字句(識別子・リテラル・コメント)、予約語、モジュールとインポート。
open_items: []
---

# 01 — Lexical structure and modules

- Encoding UTF-8. Comments: `//` to end of line, `/* ... */` non-nesting.
- Identifiers: `[A-Za-z_][A-Za-z0-9_]*`. Type/record/role/place names are
  conventionally UpperCamel; states/fields/handlers lower_snake or lowerCamel.
- Literals: Int64 `-?[0-9]+`; Float64 `-?[0-9]+\.[0-9]+`; Bool `true|false`;
  Text `"..."` with escapes `\" \\ \n \t`.
- **Reserved words** (v0): `module import place role record state init visible
  fields when fails join as via grant require publish produces witness chain
  on cap lease admit let if else return true false and or not`.
  Not reserved: `World`, `world`, `Room`, `Avatar`, `Participant`(keyspace
  name, declared), `S`(ordinary identifier; ADR-0001)。
- Core companion tokens (`perform`, `option`, `try`, `fallback`,
  `atomic_cut`) are reserved **for Core-facing notation only** (spec/04
  appendix); they are rejected in Surface v0 items with E-PARSE-005.
- Module: file begins `module Dotted.Path`; `import Dotted.Path` brings
  declarations into scope; duplicate module paths are E-NAME-004. One module
  per file; the module path is the diagnostic namespace root.
