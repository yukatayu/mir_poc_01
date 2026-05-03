# 03 — Decisions for Practical Alpha-1

## 1. Fixed decisions

### D-A1-001: Practical 100% definition

今後の 100% は practical alpha-1 readiness を意味する。current-scope evidence closeout は別 metric にする。

### D-A1-002: library-first

Alpha-1 は library-first + CLI + devtools viewer とする。installed binary / final public packaging は later U1 gate。

### D-A1-003: native output boundary

Alpha-1 は native Rust runtime / CLI が Mir alpha source / IR bundle を実行できることを要求する。Mir source の direct native code generation は要求しない。

### D-A1-004: parser front-door needed

Practical alpha-1 では、sample-ID keyed runner だけでは不十分。alpha source front-door が必要。

### D-A1-005: alpha grammar is not final grammar

Alpha parser grammar は `alpha-source` として limited / documented にする。final public grammar ではない。

### D-A1-006: typed IR first

Parser output は typed / checkable IR に落とす。runtime は source text ではなく IR / plan を消費する。

### D-A1-007: checker coverage

Lifetime/fallback, contract variance, effect/failure row, capability, cut predicate, package admission は checker coverage に入れる。

### D-A1-008: runtime reuse

sample-ID keyed bridge から reusable runtime API へ移す。既存 bridge は regression evidence として残す。

### D-A1-009: transport boundary

Alpha-1 transport は Docker/local TCP まで。production WAN / federation / durable replay は later。

### D-A1-010: save/load boundary

Alpha-1 save/load は local save/load + invalid distributed cut rejection まで。durable distributed save/load は later。

### D-A1-011: avatar boundary

Avatar / VRM / VRChat / Unity は core primitive ではない。runtime package / adapter として扱う。

### D-A1-012: native binary policy

Native binary package は signatureだけでは safe と見なさない。Alpha-1 では native execution を基本 later とし、manifest admission / reject / fallback までを扱う。

### D-A1-013: Devtools required

Alpha-1 では event DAG / route / membership / hot-plug / fallback / redaction を viewer/devtools surface として出す。

### D-A1-014: Reversed Library later

Reversed Library は alpha-1 substrate 上の future flagship application。Alpha-1 には含めない。

### D-A1-015: PrismCascade out of scope

PrismCascade は別 kernel。Alpha-1 implementation scope に入れない。

## 2. User decisions kept open

- final public packaging
- host integration target
- first shipped public surface
- final shared-space catalog breadth
- production native binary policy
- production WAN/federation target
- full avatar compatibility target

## 3. Decisions Codex may make autonomously

Codex may decide narrow implementation details if they do not freeze final public API.

Examples:

- internal Rust module organization
- alpha-only CLI command names
- JSON schema for alpha IR, if documented as alpha-local
- test fixture file names
- temporary viewer HTML/JSON schema, if non-final

Codex must not autonomously decide final public API/ABI names.
