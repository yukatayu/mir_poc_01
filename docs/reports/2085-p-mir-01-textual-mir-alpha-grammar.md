# 2085 — P-MIR-01 textual Mir alpha grammar

## Objective

Close `P-MIR-01 textual Mir alpha grammar` by actualizing a real textual Mir parser lane with explicit diagnostics, source spans, positive/negative samples, snapshot doc sync, validation, and package report discipline.

## Scope and assumptions

- Scope was limited to the `FS-01` parser floor under `specs/34` and `plan/59`.
- Product Alpha-1 `package.mir.json` remains the current product front door; this package must not silently promote `.mir` source into that lane.
- Where path context is required, unresolved import validation is handled in a path-aware parse/report path rather than the raw string-only parser entrypoint.
- No claim is made here for typed IR, interpreter execution, package artifact generation, final public grammar, or public ABI.

## Start state / dirty state

- Repository context already contained the `P-FS-00` Full System V1 rebaseline docs/spec package.
- Work for this package started from the current `main` workspace and made the worktree dirty with parser, samples, scripts, validator, and snapshot-doc changes for `P-MIR-01`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/33-full-system-v1-scope.md`
- `specs/34-textual-mir-alpha-grammar.md`
- `specs/35-mir-typed-ir-and-interpreter.md`
- `specs/36-projection-ir-and-boundary-preservation.md`
- `specs/37-posegraph-runtime-semantics.md`
- `specs/38-engine-provider-admission.md`
- `plan/58-full-system-v1-roadmap.md`
- `plan/59-textual-mir-roadmap.md`
- `plan/60-computational-runtime-roadmap.md`
- `plan/61-posegraph-runtime-roadmap.md`
- `plan/62-projection-backend-roadmap.md`
- `plan/63-engine-provider-roadmap.md`
- `sub-agent-pro/full-system-completion-001/*.md`
- `.docs/progress-task-axes.md`

## Actions taken

- Added `crates/mir-ast::textual_alpha` as a real textual Mir alpha parser surface with lexer, parser, AST, diagnostics, and source spans.
- Refactored expression AST so every `AstExpr` carries a `SourceSpan`; kept statements, contract clauses, top-level items, and module spans explicit.
- Added a path-aware parse/report path that validates unresolved imports against the current sample family and returns `unresolved_import` diagnostics without changing Product Alpha CLI behavior.
- Added parser tests for positive parse acceptance, explicit diagnostic rejection, expression span coverage, and unresolved import rejection.
- Added `crates/mir-ast/examples/textual_mir_alpha_parse.rs` as the parser-floor report carrier used by the sample helper.
- Added `samples/full-system-v1/computational/` actual rows: 2 positive rows and 8 negative rows covering the alpha negative matrix categories required by `specs/34`.
- Hardened `scripts/textual_mir_samples.py` so it validates structural/sample completeness, fails on matrix validation errors, returns non-zero on helper failure, and compares structural summaries plus span markers rather than only counts/diagnostic codes.
- Added/updated helper tests, validator required-path lists, and snapshot docs (`README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, `scripts/README.md`, `plan/58`, `plan/59`).

## Files changed

- Parser/runtime-facing source:
  `crates/mir-ast/src/lib.rs`
  `crates/mir-ast/src/textual_alpha.rs`
  `crates/mir-ast/examples/textual_mir_alpha_parse.rs`
  `crates/mir-ast/tests/textual_mir_alpha.rs`
- Full System V1 sample/helper surface:
  `samples/full-system-v1/README.md`
  `samples/full-system-v1/computational/**`
  `scripts/textual_mir_samples.py`
  `scripts/tests/test_textual_mir_samples.py`
- Validator/documentation scaffold:
  `scripts/check_source_hierarchy.py`
  `scripts/validate_docs.py`
  `scripts/tests/test_validate_docs.py`
  `README.md`
  `Documentation.md`
  `samples/README.md`
  `scripts/README.md`
  `progress.md`
  `tasks.md`
  `samples_progress.md`
  `plan/58-full-system-v1-roadmap.md`
  `plan/59-textual-mir-roadmap.md`

## Commands run

- `df -h .`
- `free -h`
- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `cargo test -p mir-ast --test textual_mir_alpha -- --nocapture`
- `python3 -m unittest scripts.tests.test_textual_mir_samples`
- `python3 scripts/textual_mir_samples.py matrix --format json`
- `python3 scripts/textual_mir_samples.py check-all --format json`
- `python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_textual_mir_samples`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `cargo fmt`
- `cargo fmt --check`
- `git diff --check`
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`
- `python3 scripts/operational_product_samples.py check-all --format json`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out <fresh tmpdir>`

## Evidence / outputs / test results

- `cargo test -p mir-ast --test textual_mir_alpha -- --nocapture`
  passed: 5 tests, including unresolved import rejection and expression span coverage.
- `python3 -m unittest scripts.tests.test_textual_mir_samples`
  passed: 5 tests.
- `python3 -m unittest scripts.tests.test_validate_docs scripts.tests.test_textual_mir_samples`
  passed: 22 tests.
- `python3 scripts/textual_mir_samples.py check-all --format json`
  accepted all 10 rows: 2 positive rows and 8 negative rows; no matrix validation errors.
- `python3 scripts/check_source_hierarchy.py`
  passed with `required: 287`, `present: 287`, `missing: 0`.
- `python3 scripts/validate_docs.py`
  reported documentation scaffold complete and found 1236 numbered reports.
- `cargo fmt --check`
  passed after formatting.
- `git diff --check`
  passed.
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`
  accepted.
- `python3 scripts/operational_product_samples.py check-all --format json`
  accepted.
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out <fresh tmpdir>`
  accepted after rerunning with an empty temp output directory; initial retry against `/tmp/mirrorea-alpha1-release` hit the expected preflight `output_dir_not_empty`.

## What changed in understanding

- `FS-01` needed more than syntactic acceptance: unresolved imports, span-bearing expression nodes, and structural helper comparisons were necessary to make the parser floor defensible.
- The right narrow implementation for unresolved imports in this package is path-aware validation at the report/module-path boundary, not changing the Product Alpha product front door.
- Snapshot docs were overstating parser-floor closure until the helper compared real structure/spans and the alpha negative matrix was covered.

## Open questions

- `P-MIR-02` still needs a concrete typed IR shape decision: crate-local IR in the existing lane versus a later dedicated crate split.
- Import resolution outside the current sample-family heuristic is intentionally narrow here; broader source package resolution remains later work.
- The parser surface is intentionally alpha and may need keyword/punctuation revision once typed IR and runtime packages consume more source forms.

## Suggested next prompt

Continue with `P-MIR-02 typed IR and checker`, lowering the textual AST into a typed IR with explicit effect/failure rows, import/type/capability rejections, and source-first checker samples.

## Plan update status

- Updated.
- `plan/58` now records the parser-floor evidence with the 10-row matrix.
- `plan/59` now records path-aware unresolved import rejection, span-bearing expression AST, and the expanded parser-floor sample family.

## Documentation.md update status

- Updated.
- Full System V1 snapshot text now points at `P-MIR-01` closeout and `P-MIR-02` as the next promoted package.

## progress.md update status

- Updated.
- Current snapshot now records the parser floor as 2 positive rows + 8 negative rows with path-aware unresolved import diagnostics and expression spans.

## tasks.md update status

- Updated.
- `P-MIR-01` closeout purpose now explicitly includes alpha negative matrix coverage and path-aware import diagnostics; next promoted package remains `P-MIR-02`.

## samples_progress.md update status

- Updated.
- Full System V1 parser-floor row, validation anchors, and recent log now include the parser helper/unit test and the richer sample matrix claim.

## Reviewer findings and follow-up

- `code_mapper` sub-agent mapped the safe integration boundary and confirmed the Product Alpha `.mir` rejection stop line must stay intact; implementation followed that guidance.
- `reviewer` sub-agent reported five issues:
  unresolved imports missing, expression spans missing, helper structure/spans under-validated, matrix validation errors not failing `check-all`, and stale status docs.
- Follow-up completed in this package:
  added path-aware unresolved import diagnostics, refactored `AstExpr` to carry spans, expanded helper projection/expected data to include structure and span markers, made helper failure/non-zero behavior strict, expanded the sample matrix, and synchronized docs/status anchors.

## Skipped validations and reasons

- No package-floor validations were skipped.
- Full System V1 later planned commands (`full_system_v1_samples.py`, `posegraph_runtime_samples.py`, `projection_v1_samples.py`, `provider_admission_samples.py`, `full_system_v1_release_check.py`) remain unimplemented by design and were not run.

## Commit / push status

- Pending at report authoring time. Commit hash and push result will be added after `git commit --no-gpg-sign` and `git push`.

## Sub-agent session close status

- `019e4d11-7468-7290-b639-084ece2a1265` (`code_mapper`): completed and closed.
- `019e4d22-4765-7f73-9510-6484e07dde2b` (`reviewer`): completed and closed.
