# Report 0656 — phase6 parser checker runtime public surface inventory package

- Date: 2026-04-12T14:40:36.868651Z
- Author / agent: Codex
- Scope: Phase 6 docs-first inventory for parser / checker / runtime public operational surface, plus snapshot / plan / abstract / sample-policy mirrors.
- Decision levels touched: L2

## 1. Objective

- current repo の parser / checker / runtime surfaceについて、already-public parser-free helper stack、crate-public but non-production compile-ready tranche、repo-local helper / example emitter surface を分けて inventory 化する。
- Rust の `pub` visibility と final public operational contract を同一視しない guard を明文化する。
- repo-level next line を Mirrorea / shared-space docs-first re-entry へ handoff する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/301-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-phase6-actual-checker-runtime-skeleton-first-tranche-comparison.md`
- `specs/examples/302-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-minimal-phase6-checker-runtime-skeleton-threshold.md`
- `specs/examples/303-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-ready-phase6-compile-ready-verification-and-formal-hook-comparison.md`
- `specs/examples/304-current-l2-phase6-compile-ready-verification-and-formal-hook-ready-minimal-phase6-compile-ready-verification-and-formal-hook-threshold.md`
- `specs/examples/321-current-l2-actual-parser-to-program-lowering-first-cut-ready-syntax-backed-sample-runner-first-cut-comparison.md`
- `specs/examples/322-current-l2-syntax-backed-sample-runner-first-cut-ready-minimal-syntax-backed-sample-runner-first-cut-threshold.md`
- `specs/examples/327-current-l2-source-sample-authoring-bless-regression-policy-ready-theorem-first-concrete-tool-pilot-comparison.md`
- `specs/examples/328-current-l2-theorem-first-concrete-tool-pilot-ready-minimal-theorem-first-concrete-tool-pilot-threshold.md`
- `specs/examples/353-current-l2-actual-e22-contrast-row-source-actualization-ready-stable-static-malformed-post-contrast-sequencing-comparison.md`
- `specs/examples/354-current-l2-stable-static-malformed-post-contrast-sequencing-ready-minimal-stable-static-malformed-post-contrast-sequencing-threshold.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`

## 3. Actions taken

- Added `specs/examples/355...356` to close the docs-first public surface inventory.
- Fixed the current inventory to three buckets:
  - already-public parser-free helper stack
  - crate-public but non-production compile-ready tranche
  - repo-local helper / example emitter surface
- Updated `plan/09-helper-stack-and-responsibility-map.md` with an explicit bucketed inventory section and guard rules.
- Updated snapshot and mirror documents so current repo-level line no longer points at public surface inventory.
- Left final public parser / checker / runtime API, public runner / exporter CLI, and theorem/model-check/public-checker migration as later lines.

## 4. Files changed

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/examples/355-current-l2-stable-static-malformed-post-contrast-sequencing-ready-parser-checker-runtime-public-surface-inventory-comparison.md`
- `specs/examples/356-current-l2-parser-checker-runtime-public-surface-inventory-ready-minimal-parser-checker-runtime-public-surface-inventory-threshold.md`
- `docs/reports/0656-phase6-parser-checker-runtime-public-surface-inventory-package.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`

## 5. Commands run and exact outputs

- `cargo test -p mir-runtime --test current_l2_source_sample_runner`
  - `running 9 tests`
  - `test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-semantics --test current_l2_minimal_interpreter run_bundle_checks_static_runtime_and_trace_expectations -- --exact`
  - `running 1 test`
  - `test run_bundle_checks_static_runtime_and_trace_expectations ... ok`
- `cargo test -p mir-semantics --test current_l2_minimal_interpreter run_directory_named_profiles_match_catalog_resolution_and_expected_selection -- --exact`
  - `running 1 test`
  - `test run_directory_named_profiles_match_catalog_resolution_and_expected_selection ... ok`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 655 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- parser-free helper stack still has real public behavior with passing bundle and named-profile tests.
- current compile-ready parser / runtime tranche is usable and tested, but its current docs role remains non-production thin tranche rather than final public contract.
- The cleanest inventory is therefore not public vs private, but already-public parser-free vs crate-public non-production vs repo-local helper.

## 7. Changes in understanding

- The key guard is `pub visibility != final public operational contract`.
- Public surface inventory closes a documentation gap, not a promotion event.
- After this inventory is fixed, the next global line naturally moves to Mirrorea / shared-space, while Phase 6 residue stays as:
  - stable-static edge-pair first reopen
  - public operational surface actualization gate

## 8. Open questions

- Which compile-ready tranche, if any, should become the first actual promoted public API cut.
- Whether a future public runner / exporter CLI should grow from repo-local helpers or be designed independently.
- Whether public checker migration should precede or follow public parser/runtime actualization.

## 9. Suggested next prompt

- `Mirrorea / shared-space docs-first re-entry package を閉じ、fabric / shared-space / Typed-Effect / Prism / app line を old FutureWork bucket へ戻さずに boundary 付きで整理してください。`
