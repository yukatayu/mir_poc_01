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

Not decided:

- final public grammar.
- final parser API.
- dynamic place-expression syntax.

## package order

| Package | Role | Completion gate |
|---|---|---|
| `P-SURF-00B` | docs/spec rebaseline | `specs/39`, `specs/43`, plans, snapshot docs, guides, and report are synchronized |
| `P-SURF-01` | brace parser | `S { ... }` place blocks and `Role[instance] { ... }` role-instance blocks parse; bare role blocks and `S[ ... ]` reject; record literals remain distinct |
| `P-SURF-03` | Surface-to-Core elaboration | cross-locus read/write lowers to Core IR with source spans and obligations |
| `P-SURF-04` | generated communication | MessageEnvelope / publish / observe / failure-row obligations are generated and visible |
| `P-SURF-08` | diagnostics/devtools | source/Core mapping and generated edges are inspectable |

## planned rows

- `SURF-01` accepted `S { ... }`.
- `SURF-02` rejected `S[ ... ]`.
- `SURF-03` accepted record literal.
- `SURF-04` ambiguous brace diagnostic.
- `SURF-05` accepted role instance block.
- `ELAB-01..05` generated Core IR / communication / failure-row rows.

## validation anchors

Current docs-only anchors:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Future implementation anchors:

```bash
python3 scripts/surface_mir_samples.py matrix --format json
python3 scripts/surface_mir_samples.py check-all --format json
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
```

## stop lines

- do not add `S[ ... ]` as syntax or sugar.
- do not freeze final public grammar from this alpha cut.
- do not hide generated communication from Core IR / devtools.
- do not treat `package.mir.json` as semantic source authority.
