# Report 0628 — phase6 source sample verification ladder wiring

- Date: 2026-04-12T04:53:19.523056Z
- Author / agent: Codex
- Scope: Phase 6 syntax-backed source sample verification ladder wiring for the first-authored trio, including reached-stage inventory, test-only ratchet coverage, and snapshot mirror updates.
- Decision levels touched: current L2 package; docs-first + test-only ratchet actualization.

## 1. Objective

- Fix the current reached-stage inventory for the authored source samples `e2-try-fallback`, `e4-malformed-lineage`, and `e23-malformed-try-fallback-missing-fallback-body`.
- Keep `e1` / `e3` / `e21` explicitly in `source-target-only / not yet authored` state instead of silently widening the authored surface.
- Preserve the current tool-neutral formal-hook top while wiring runtime/static evidence through repo-local tests and mirror documents.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-mir-core.md`
- `specs/02-language-and-compiler-co-design.md`
- `specs/03-context-and-roadmap.md`
- `specs/09-current-l2.md`
- `specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
- `specs/examples/304-current-l2-phase6-compile-ready-verification-and-formal-hook-ready-minimal-phase6-compile-ready-verification-and-formal-hook-threshold.md`
- `specs/examples/317-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-ready-representative-fixture-source-mapping-matrix-comparison.md`
- `specs/examples/318-current-l2-representative-fixture-source-mapping-matrix-ready-minimal-representative-fixture-source-mapping-matrix-threshold.md`
- `specs/examples/321-current-l2-actual-parser-to-program-lowering-first-cut-ready-syntax-backed-sample-runner-first-cut-comparison.md`
- `specs/examples/322-current-l2-syntax-backed-sample-runner-first-cut-ready-minimal-syntax-backed-sample-runner-first-cut-threshold.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-semantics/tests/current_l2_formal_hook_support.rs`

## 3. Actions taken

1. Compared whether the current ladder should widen beyond the first-authored trio and fixed the package on `e2` / `e4` / `e23` only.
2. Added `specs/examples/323...324` to record the comparison and threshold for verification ladder wiring.
3. Added `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs` to ratchet the reached-stage inventory without widening the public runner surface.
4. Added a test-only `serde` dev-dependency to `crates/mir-runtime/Cargo.toml` so the shared formal-hook support helpers remain reusable from the new runtime-side test.
5. Updated `Documentation.md`, `progress.md`, `tasks.md`, relevant `plan/` mirrors, `samples/current-l2/README.md`, and the Phase 6 research abstract so the current mainline changes from `verification ladder wiring` to `source-sample authoring / bless / regression policy`.
6. Recorded the source traceability addendum in `plan/90-source-traceability.md`.

## 4. Files changed

- `Documentation.md`
- `crates/mir-runtime/Cargo.toml`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `docs/reports/0628-phase6-source-sample-verification-ladder-wiring.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `specs/00-document-map.md`
- `specs/examples/323-current-l2-syntax-backed-sample-runner-first-cut-ready-verification-ladder-wiring-comparison.md`
- `specs/examples/324-current-l2-verification-ladder-wiring-ready-minimal-verification-ladder-wiring-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder`
  - first run:
    - `error[E0432]: unresolved import 'serde'`
  - second run:
    - `running 3 tests`
    - `test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-runtime --test current_l2_source_lowering`
  - `running 3 tests`
  - `test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `running 5 tests`
  - `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
  - `running 5 tests`
  - `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e2-try-fallback --artifact-root /tmp/mir-phase6-ladder-check --run-label ladder-e2 --overwrite`
  - `bundle artifact: /tmp/mir-phase6-ladder-check/bundles/ladder-e2/e2-try-fallback.detached.json`
  - `formal hook artifact: /tmp/mir-phase6-ladder-check/formal-hooks/ladder-e2/e2-try-fallback.formal-hook.json`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e4-malformed-lineage --artifact-root /tmp/mir-phase6-ladder-check --run-label ladder-e4 --overwrite`
  - `static gate artifact: /tmp/mir-phase6-ladder-check/static-gates/ladder-e4/e4-malformed-lineage.static-gate.json`
  - `formal hook artifact: /tmp/mir-phase6-ladder-check/formal-hooks/ladder-e4/e4-malformed-lineage.formal-hook.json`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e23-malformed-try-fallback-missing-fallback-body --artifact-root /tmp/mir-phase6-ladder-check --run-label ladder-e23 --overwrite`
  - `static gate artifact: /tmp/mir-phase6-ladder-check/static-gates/ladder-e23/e23-malformed-try-fallback-missing-fallback-body.static-gate.json`
  - `formal hook artifact: /tmp/mir-phase6-ladder-check/formal-hooks/ladder-e23/e23-malformed-try-fallback-missing-fallback-body.formal-hook.json`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 627 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- The smallest coherent ladder package is still the first-authored trio only. Widening to `e1` / `e3` / `e21` would have mixed authoring backlog with current verification evidence.
- `e2-try-fallback` now has explicit reached evidence for `static gate -> interpreter success -> runtime_try_cut_cluster formal hook`.
- `e4-malformed-lineage` and `e23-malformed-try-fallback-missing-fallback-body` now have explicit reached evidence for `static gate malformed stop -> fixture_static_cluster formal hook`.
- The formal-hook reached stage can stay on the fixture-aligned detached-artifact route for now; a source-runner-native formal artifact is not required to close this package.
- The new runtime-side test ratchet keeps the per-stem composite reached-stage inventory synchronized without expanding the runner API, while still relying on the detached formal-hook route rather than a single end-to-end source-runner-native artifact.

## 7. Changes in understanding

- The essential boundary in this package is authored versus deferred source rows, not runtime versus static implementation mechanics.
- A docs-first reached-stage inventory backed by tests is enough for the current checkpoint; a public ladder/report surface would be premature.
- Reusing the existing semantics-side formal-hook support from runtime tests is viable with only a test-only dependency adjustment.

## 8. Open questions

- How much authoring/bless workflow should be encoded as scripts in the next package versus documented as a manual repo policy.
- When `e1` / `e3` / `e21` should move from `source-target-only` to authored rows.
- Whether a later package needs source-runner-native formal-hook artifact emission or whether detached-loop bridging remains sufficient longer.

## 9. Suggested next prompt

- Continue with `Phase 6 source-sample authoring / bless / regression policy`, then `Phase 6 theorem-first concrete tool pilot`, then a final mirror sweep.
