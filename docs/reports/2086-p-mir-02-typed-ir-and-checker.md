# 2086 — P-MIR-02 typed IR and checker

## Objective

Close `P-MIR-02 typed IR and checker` by adding a source-first Full System V1 typed IR/checker lane, runnable positive/negative samples, docs/status synchronization, validation evidence, and package report.

## Scope and assumptions

- Scope is limited to `P-MIR-02` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- The implementation stays crate-local under `mir-semantics`; it does not split a final `mir-ir` crate yet.
- `package.mir.json` remains Product Alpha-1 compatibility/package artifact and is not changed into the source-of-truth lane.
- The checker is intentionally narrow and alpha-scoped. It does not claim interpreter execution, final public grammar, final typed IR API, LLVM/native codegen, server/client split, or provider execution.
- When source semantics were still open, the implementation took the narrow side: explicit residual obligations are emitted rather than silently claiming full ambient effect/failure containment.

## Start state / dirty state

- Branch: `main`
- Start point: after `P-MIR-01` closeout and push
- Initial local state for this package was not fully clean:
  - `crates/mir-ast/src/textual_alpha.rs` already had an uncommitted resolver exposure diff
  - `crates/mir-semantics/tests/typed_ir_interpreter.rs` existed as a new failing test
- Those two files were treated as in-scope package work and carried forward rather than reverted.

## Documents consulted

- Core repo docs:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Full System V1 specs:
  - `specs/33-full-system-v1-scope.md`
  - `specs/34-textual-mir-alpha-grammar.md`
  - `specs/35-mir-typed-ir-and-interpreter.md`
  - `specs/36-projection-ir-and-boundary-preservation.md`
  - `specs/37-posegraph-runtime-semantics.md`
  - `specs/38-engine-provider-admission.md`
- Full System V1 plans:
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/59-textual-mir-roadmap.md`
  - `plan/60-computational-runtime-roadmap.md`
  - `plan/61-posegraph-runtime-roadmap.md`
  - `plan/62-projection-backend-roadmap.md`
  - `plan/63-engine-provider-roadmap.md`
- Handoff package:
  - `sub-agent-pro/full-system-completion-001/*.md`
- Additional policy/status doc required by `AGENTS.md` for roadmap/status work:
  - `.docs/progress-task-axes.md`

## Actions taken

1. Verified the failing `typed_ir_interpreter` test first and confirmed the expected RED state: `mir_semantics::full_system_v1` was missing.
2. Exposed a reusable textual import resolver from `crates/mir-ast::textual_alpha` and kept parser-side unresolved import validation path-aware.
3. Added `mir-ast` as a dependency of `mir-semantics` and exported a new `full_system_v1` module.
4. Implemented a crate-local typed IR/report surface in `crates/mir-semantics/src/full_system_v1/typed_ir.rs`.
5. Implemented source-first checker/lowering logic in `crates/mir-semantics/src/full_system_v1/checker.rs`:
   - recursive import loading
   - type lowering
   - lexical scope checks
   - function signature/call checks
   - fixed-array length and static bounds checks
   - effect declaration / failure row / capability checks
   - transition/body checks
   - explicit accepted obligations and residual obligations
6. Added `crates/mir-semantics/examples/full_system_v1_check.rs` as a JSON/pretty report front door for sample helpers.
7. Extended source-first computational samples with a typed-checker matrix:
   - reused parser-positive roots for `add-one` and host-boundary rows
   - added a positive record/field row
   - added negative rows for unresolved import, imported semantic failure, duplicate module-path ambiguity, type mismatch, unbound scope, static array bounds, undeclared effect, missing failure row, and undeclared capability
   - added `expected/check.json` projections
8. Added `scripts/full_system_v1_samples.py` and `scripts/tests/test_full_system_v1_samples.py` to validate the typed checker lane.
9. Widened module loading/checking after reviewer findings:
   - ambiguous import resolution is rejected instead of silently choosing one declaration
   - imported modules are checked semantically and their diagnostics are propagated back to the root report
   - helper projections now preserve `resolved_paths`
10. Fixed two checker narrowness issues discovered while exercising samples:
   - dotted source paths like `pair.left` are now interpreted as a field-access chain when the base binding is in scope
   - duplicate/cascading diagnostics were reduced for missing failure rows and undeclared capability rows
11. Updated status/reader-facing docs so the repo snapshot now reads `P-MIR-02` as closed and `P-MIR-03` as next.
12. Ran the package validation floor and current major anchors.

## Files changed

- Rust source:
  - `crates/mir-ast/src/textual_alpha.rs`
  - `crates/mir-semantics/Cargo.toml`
  - `crates/mir-semantics/src/lib.rs`
  - `crates/mir-semantics/src/full_system_v1/mod.rs`
  - `crates/mir-semantics/src/full_system_v1/typed_ir.rs`
  - `crates/mir-semantics/src/full_system_v1/checker.rs`
  - `crates/mir-semantics/examples/full_system_v1_check.rs`
  - `crates/mir-semantics/tests/typed_ir_interpreter.rs`
- Scripts/tests:
  - `scripts/full_system_v1_samples.py`
  - `scripts/tests/test_full_system_v1_samples.py`
- Full System V1 computational sample roots/matrix:
  - `samples/full-system-v1/computational/README.md`
  - `samples/full-system-v1/computational/typed-ir-matrix.json`
  - `samples/full-system-v1/computational/add-one-positive/expected/check.json`
  - `samples/full-system-v1/computational/host-boundary-positive/expected/check.json`
  - `samples/full-system-v1/computational/unresolved-import-negative/expected/check.json`
  - `samples/full-system-v1/computational/record-field-positive/*`
  - `samples/full-system-v1/computational/imported-semantic-negative/*`
  - `samples/full-system-v1/computational/duplicate-module-path-negative/*`
  - `samples/full-system-v1/computational/type-mismatch-negative/*`
  - `samples/full-system-v1/computational/scope-unbound-negative/*`
  - `samples/full-system-v1/computational/static-array-bounds-negative/*`
  - `samples/full-system-v1/computational/undeclared-effect-negative/*`
  - `samples/full-system-v1/computational/effect-failure-missing-negative/*`
  - `samples/full-system-v1/computational/undeclared-capability-negative/*`
- Snapshot/docs/repository memory:
  - `README.md`
  - `Documentation.md`
  - `samples/README.md`
  - `scripts/README.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/59-textual-mir-roadmap.md`
  - `plan/60-computational-runtime-roadmap.md`
  - `docs/hands_on/full_system_v1_roadmap_01.md`
  - `docs/research_abstract/full_system_v1_roadmap_01.md`

## Commands run

```bash
git status --short
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
cargo test -p mir-ast --test textual_mir_alpha -- --nocapture
python3 -m unittest scripts.tests.test_full_system_v1_samples
python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_textual_mir_samples scripts.tests.test_full_system_v1_samples
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out <fresh tmpdir>
python3 scripts/operational_product_samples.py check-all --format json
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- RED confirmation:
  - initial `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture` failed because `mir_semantics::full_system_v1` did not exist
- Package tests after implementation:
  - `cargo test -p mir-ast --test textual_mir_alpha -- --nocapture`: passed, 6 tests
  - `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`: passed, 5 tests
  - `python3 -m unittest scripts.tests.test_full_system_v1_samples`: passed, 6 tests
  - `python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_textual_mir_samples scripts.tests.test_full_system_v1_samples`: passed, 28 tests
- Source-first helpers:
  - `python3 scripts/textual_mir_samples.py check-all --format json`: passed all 10 parser rows
  - `python3 scripts/full_system_v1_samples.py check-all --format json`: passed all 12 typed-checker rows
    - 3 positive
    - 9 negative
- Doc/source validators:
  - `python3 scripts/check_source_hierarchy.py`: passed, `required: 287`, `present: 287`, `missing: 0`
  - `python3 scripts/validate_docs.py`: initially failed because the latest report headings still carried numbered prefixes; passed after aligning the report headings to the required template
  - `cargo fmt --check`: passed after `cargo fmt`
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release.<suffix>`: accepted
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted

## What changed in understanding

- The current textual parser surface treats dotted names like `pair.left` as a path-like variable token, so the first typed checker needs an explicit narrow reinterpretation step for record field chains rather than relying on AST `FieldAccess` alone.
- The right place for the first typed checker is `mir-semantics`, not Product Alpha schema/runtime crates. This keeps `.mir` source authority separate from `package.mir.json`.
- For this package, explicit residual obligations are the correct representation for still-open ambient effect/failure containment instead of over-claiming checker completion.
- Imported source modules must be part of the semantic closure, not just the parse/load closure. Otherwise the checker can over-accept a root module that depends on broken imported code.
- Path-aware textual import resolution must reject duplicate declared module paths explicitly; silently picking one makes later projection/runtime packages unsound.

## Open questions

- `P-MIR-03` still needs a concrete compute-trace shape for source-derived interpreter execution reports and clear static-vs-runtime reject partitioning in helper outputs.
- The crate-local typed IR may later deserve extraction if interpreter/projection packages create too much coupling, but there is no current need to split it now.

## Suggested next prompt

```text
P-MIR-03 computational interpreter
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`
- `plan/59-textual-mir-roadmap.md`
- `plan/60-computational-runtime-roadmap.md`

## Documentation.md update status

Updated for `P-MIR-02` closeout and `P-MIR-03` promotion.

## progress.md update status

Updated to show:

- current closeout package `P-MIR-02`
- `FS-02` as first-floor evidence
- parser+checker runnable commands
- next package `P-MIR-03`
- recent log entry at `2026-05-22 10:59 JST`

## tasks.md update status

Updated to promote `P-MIR-03 computational interpreter` as the next self-driven package and to mark `P-MIR-02` as the current closeout package.

## samples_progress.md update status

Updated to mark `samples/full-system-v1/computational/` as parser+checker evidence-closed and to add the new validation anchors/log row.

## Reviewer findings and follow-up

- Read-only code-mapping support was available earlier in the package via existing sub-agent context (`Carver`) and was used to confirm the proper crate boundary.
- A dedicated read-only reviewer sub-agent (`Avicenna`, id `019e4d56-241d-7612-bbdd-fd9d7648dad6`) returned three findings:
  - imported modules were loaded but not semantically checked
  - import resolution could silently accept duplicate declared module paths
  - helper/sample coverage did not preserve `resolved_paths` and did not exercise cross-module negatives
- Follow-up fixed each finding and reran the checker/parser/helper validation lane.
- Final local review also checked:
  - Product Alpha lane does not leak into `.mir` source-first authority
  - duplicate/cascading diagnostics remain bounded in the widened checker path
  - parser lane and typed-checker lane remain separate helpers/matrices
  - docs snapshot consistently points to `P-MIR-03`

## Skipped validations and reasons

- No package-specific validation was skipped.
- `P-MIR-03+` commands were not run because they do not exist yet and would be an over-claim for `P-MIR-02`.

## Commit / push status

Pending at report authoring time. The package commit/push is the next step after writing this report.

## Sub-agent session close status

- `Carver`: prior read-only mapping support already complete before this closeout
- `Avicenna`: returned reviewer findings; close after package close
