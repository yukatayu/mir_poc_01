# 2087 — P-MIR-03 computational interpreter

## Objective

Close `P-MIR-03 computational interpreter` by adding a source-first Full System V1 pure interpreter/runtime lane, positive and negative runtime samples, docs/status synchronization, validation evidence, and package report.

## Scope and assumptions

- Scope is limited to `P-MIR-03` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- The semantic owner remains `crates/mir-semantics`; `crates/mir-runtime` only adds a thin session/report wrapper.
- This package does not claim effectful runtime completion, final grammar, final typed IR API, final ABI/SDK, projection/backend realization, provider execution, LLVM/native codegen, or broader server/client split.
- When runtime behavior was still open, the implementation took the narrow side: pure computation is executed, unsupported effectful/runtime-complete constructs are rejected explicitly, and static rejection is kept separate from runtime rejection.
- `package.mir.json` remains Product Alpha-1 compatibility/package artifact rather than the authority for the new source-first lane.

## Start state / dirty state

- Branch: `main`
- Start point: after `P-MIR-02` closeout and push
- Initial local state for this package was not clean:
  - source-first checker work from `P-MIR-02` already existed in the tree and was intentionally carried forward
  - the workspace contained in-scope uncommitted Full System V1 docs/sample updates connected to the new runtime lane
- Those diffs were treated as in-scope package work and not reverted.

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

1. Refactored the `P-MIR-02` checker surface so program analysis could be reused by a runtime/interpreter lane without duplicating import/load/type-check logic.
2. Added `crates/mir-semantics/src/full_system_v1/interpreter.rs` and exported a source-derived pure interpreter/report surface:
   - function execution from textual `.mir`
   - variables and lexical scope
   - fixed arrays and dynamic bounds checks
   - records and field reads
   - `if` / `while`
   - imports and intra-source function calls
   - explicit static rejection vs runtime rejection
   - compute trace plus observer-safe summary snapshots
3. Kept unsupported current-floor constructs explicit:
   - `Bind` / `Perform` are rejected as not yet executed in the pure runtime lane
   - `for` loops are rejected as unsupported at this floor
4. Added `crates/mir-runtime/src/full_system_v1_session.rs` and an example front door so the runtime lane can be exercised through `cargo run -p mir-runtime --example mir_full_system_v1_session`.
5. Extended the `typed_ir_interpreter` test file with pure interpreter coverage and added a new `mir-runtime` integration test file for the session/report wrapper.
6. Added/updated Full System V1 computational sample roots for runtime execution:
   - positives for variables/scope, arrays, control-flow, imports, plus existing add-one and record-field rows
   - negatives for static scope failure, static array failure, imported semantic failure, and dynamic array runtime failure
   - `expected/run.json` projections for both accepted and rejected rows
7. Reworked `scripts/full_system_v1_samples.py` into a dual-lane helper so it now validates:
   - checker matrix
   - runtime matrix
   - combined closeout path
8. Updated the helper unit test file to exercise list/matrix/run/check behavior for both checker and runtime lanes.
9. Synchronized snapshot docs, plan memory, and sample dashboards so `P-MIR-03` is closed and `P-MIR-04` is promoted.
10. Ran the package validation floor and major existing anchors, including a fresh-tempdir rerun for Product Alpha release check after an initial non-empty output directory failure.

## Files changed

- Rust source:
  - `crates/mir-semantics/src/full_system_v1/checker.rs`
  - `crates/mir-semantics/src/full_system_v1/interpreter.rs`
  - `crates/mir-semantics/src/full_system_v1/mod.rs`
  - `crates/mir-semantics/tests/typed_ir_interpreter.rs`
  - `crates/mir-runtime/src/lib.rs`
  - `crates/mir-runtime/src/full_system_v1_session.rs`
  - `crates/mir-runtime/examples/mir_full_system_v1_session.rs`
  - `crates/mir-runtime/tests/full_system_v1_session.rs`
- Scripts/tests:
  - `scripts/full_system_v1_samples.py`
  - `scripts/tests/test_full_system_v1_samples.py`
- Full System V1 computational sample roots/matrix:
  - `samples/full-system-v1/computational/README.md`
  - `samples/full-system-v1/computational/runtime-matrix.json`
  - `samples/full-system-v1/computational/add-one-positive/expected/run.json`
  - `samples/full-system-v1/computational/record-field-positive/expected/run.json`
  - `samples/full-system-v1/computational/scope-unbound-negative/expected/run.json`
  - `samples/full-system-v1/computational/static-array-bounds-negative/expected/run.json`
  - `samples/full-system-v1/computational/imported-semantic-negative/expected/run.json`
  - `samples/full-system-v1/computational/variables-scope-positive/*`
  - `samples/full-system-v1/computational/arrays-positive/*`
  - `samples/full-system-v1/computational/control-flow-positive/*`
  - `samples/full-system-v1/computational/imports-positive/*`
  - `samples/full-system-v1/computational/dynamic-array-bounds-negative/*`
- Snapshot/docs/repository memory:
  - `README.md`
  - `Documentation.md`
  - `samples/README.md`
  - `scripts/README.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/60-computational-runtime-roadmap.md`
  - `docs/hands_on/full_system_v1_roadmap_01.md`
  - `docs/research_abstract/full_system_v1_roadmap_01.md`

## Commands run

```bash
git status --short
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
python3 -m unittest scripts.tests.test_full_system_v1_samples
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/product_alpha1_release_check.py --format json check-all --out "$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)"
python3 scripts/operational_product_samples.py check-all --format json
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- Package tests after implementation:
  - `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`: passed, 7 tests
  - `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`: passed, 2 tests
  - `python3 -m unittest scripts.tests.test_full_system_v1_samples`: passed, 7 tests
  - `python3 -m unittest scripts.tests.test_validate_docs`: passed, 17 tests
- Source-first helpers:
  - `python3 scripts/textual_mir_samples.py check-all --format json`: passed
  - `python3 scripts/full_system_v1_samples.py check-all --format json`: passed all 22 rows
    - checker lane: 12 rows
    - runtime lane: 10 rows
      - 6 positive
      - 4 negative
- Doc/source validators:
  - `python3 scripts/check_source_hierarchy.py`: passed
  - `python3 scripts/validate_docs.py`: passed
  - `cargo fmt --check`: initially failed, then passed after `cargo fmt`
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release`: first attempt failed because the output directory was not empty
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out "$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)"`: accepted
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted

## What changed in understanding

- The first runnable Full System V1 interpreter belongs in the source-first semantics lane, not in Product Alpha compatibility artifacts. Reusing checker analysis while keeping runtime execution narrow avoids inventing a second authority source.
- The right current split is:
  - static reject for parse/import/type/effect/failure/capability/support-floor issues
  - runtime reject for dynamic execution failures such as bounds checks
- A useful first runtime report needs both:
  - machine-comparable `expected/run.json` structure
  - observer-safe snapshots and compute trace for developer-facing evidence
- `P-MIR-03` can be truthfully closed without over-claiming effectful runtime completion. Explicitly rejecting `Perform`/`Bind` at this floor is safer than introducing a fake integrated effect runtime too early.

## Open questions

- `P-MIR-04` still needs a concrete source-first execution contract for `publish`, `observe`, `witness`, `handoff`, and cut/save interaction over the new runtime lane.
- The exact widening path for loops beyond `while`, richer data layout, and broader import/module packaging remains later than this package and should follow actual runtime needs.

## Suggested next prompt

```text
P-MIR-04 effectful integration
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`
- `plan/60-computational-runtime-roadmap.md`

## Documentation.md update status

Updated for `P-MIR-03` closeout and `P-MIR-04` promotion.

## progress.md update status

Updated to show:

- `P-MIR-03` closed
- current promoted package `P-MIR-04`
- next promoted package after current closeout `P-POSE-03`
- recent log entry at `2026-05-22 11:37 JST`

## tasks.md update status

Updated to promote `P-MIR-04 effectful integration` as the current package and `P-POSE-03 runtime PoseGraph` as the next promoted closeout target.

## samples_progress.md update status

Updated to keep `samples/full-system-v1/computational/` marked as evidence-closed parser+checker+pure-runtime floor and to append the `P-MIR-03` closeout log entry.

## Reviewer findings and follow-up

- Earlier package reviewers already established the correct source authority split: Mir source remains primary and `package.mir.json` stays compatibility/package artifact.
- Two package-close reviewers were available through sub-agent notifications and their findings were incorporated:
  - one reviewer highlighted the need to avoid over-claiming beyond the pure subset and to keep effectful/runtime-complete statements out of `P-MIR-03`
  - one reviewer mapped reusable runtime surfaces and reinforced the need for a new source-derived interpreter/report path instead of leaning on Product Alpha helpers as proof of Mir-owned computation
- Follow-up kept runtime scope narrow, separated static/runtime rejection, and preserved non-claims for effectful integration, PoseGraph runtime, projection/backend, and providers.

## Skipped validations and reasons

- No package-specific validation was skipped.
- Commands for later packages such as `posegraph_runtime_samples.py`, `projection_v1_samples.py`, `provider_admission_samples.py`, and `full_system_v1_release_check.py` were not run because they do not exist yet and would be an over-claim for `P-MIR-03`.

## Commit / push status

Pending at report authoring time. Commit/push will be recorded after final package closeout commands run.

## Sub-agent session close status

- `Carver`: prior mapping support already complete before this closeout
- `Avicenna`: findings received earlier in the chain and reflected in runtime scope control
- additional closeout synthesis used local diff review because no further sub-agent call was required for this package
