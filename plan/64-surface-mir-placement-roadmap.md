# plan/64 — Surface Mir placement roadmap

## purpose

This document is repository memory for `specs/39-surface-mir-placement-elaboration.md`.

It records the brace rebaseline:

```text
canonical place scope = S { ... }
not supported = S[ ... ]
```

## current decision

Decided:

- Surface Mir user-facing place blocks use `S { ... }`.
- `S[ ... ]` is rejected and is not sugar.
- `[]` remains reserved for value-level indexing.
- generated communication / publish / observe appears in Core IR and devtools.
- `.mir` source files are semantic source authority.
- `P-SURF-01` actualized the parser floor in `crates/mir-ast::surface_alpha`
  and `samples/full-system-v1-surface/syntax/`.
- `P-SURF-03` actualized a narrow Surface-to-Core elaboration floor in
  `crates/mir-semantics::surface_to_core_elaboration` and
  `samples/full-system-v1-surface/elaboration/`.

Not decided:

- final public grammar.
- final parser API.
- dynamic place-expression syntax.

## package order

| Package | Role | Completion gate |
|---|---|---|
| `P-SURF-00B` | docs/spec rebaseline | `specs/39`, `specs/43`, plans, snapshot docs, guides, and report are synchronized |
| `P-SURF-01` | brace parser | closed: `SURF-01..09` pass via `cargo test -p mir-ast --test surface_mir_parser` and `scripts/surface_mir_samples.py check-all` |
| `P-SURF-03` | Surface-to-Core elaboration | closed: cross-locus read/write lowers to Core IR with remote requests, generated edges, source spans, obligations, and underdeclared failure-row rejection |
| `P-SURF-04` | generated communication | next: MessageEnvelope / publish / observe / failure-row obligations are generated and visible |
| `P-SURF-08` | diagnostics/devtools | source/Core mapping and generated edges are inspectable |

## actualized parser rows

- `SURF-01` accepted `S { ... }`.
- `SURF-02` rejected `S[ ... ]` with `bracket_place_scope_not_supported`.
- `SURF-03` accepted record literal.
- `SURF-04` ambiguous brace diagnostic.
- `SURF-05` accepted role instance block.
- `SURF-06` rejected undeclared place block heads.
- `SURF-07` rejected undeclared role-instance heads.
- `SURF-08` rejected invalid role-instance binders.
- `SURF-09` accepted `S[self] { ... }` when `S` resolves to a declared role,
  preserving namespace-based disambiguation.

Actualized P-SURF-03 elaboration rows:

- `ELAB-01`: cross-place read generates a remote read request and observe edge.
- `ELAB-02`: cross-place write generates an owner-directed remote write request.
- `ELAB-04`: underdeclared generated failure row is rejected.
- `ELAB-05`: generated Core IR preserves source spans.
- `ELAB-06`: unsupported statements reject rather than being silently dropped.
- `ELAB-07`: write-side underdeclared generated failure row is rejected.
- `ELAB-08`: nested place read keeps owner-directed request evidence.

Remaining generated communication row for `P-SURF-04`:

- `ELAB-03`: private field auto-publish rejected or blocked.

## validation anchors

Current anchors:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/surface_mir_samples.py check-all --format json
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
```

Future implementation anchors:

```bash
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
```

## stop lines

- do not add `S[ ... ]` as syntax or sugar.
- do not freeze final public grammar from this alpha cut.
- do not hide generated communication from Core IR / devtools.
- do not treat `package.mir.json` as semantic source authority.
