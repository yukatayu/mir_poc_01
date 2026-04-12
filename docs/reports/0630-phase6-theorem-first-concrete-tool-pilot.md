# Report 0630 — phase6 theorem-first concrete tool pilot

- Date: 2026-04-12T05:49:00Z
- Author / agent: Codex
- Scope: Phase 6 theorem-first concrete tool pilot, including proof-notebook review-unit actualization, normative package `327...328`, and snapshot mirror updates.
- Decision levels touched: current L2 package; non-production theorem-side consumer actualization.

## 1. Objective

- Fix the first concrete theorem-side consumer after the tool-neutral formal hook without reopening richer bridge sketch, compare/bless metadata, or concrete theorem/model-check tool binding.
- Keep the current cut on a non-production `proof_notebook_review_unit` derived from existing formal-hook artifacts.
- Hand off the next mainline cleanly to `Phase 0 / 6 post-checkpoint drift suppression / mirror sweep`.

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
- `specs/examples/303...326`
- `specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md`
- `specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`
- `crates/mir-semantics/examples/support/current_l2_formal_hook_support.rs`

## 3. Actions taken

1. Added `crates/mir-semantics/examples/support/current_l2_proof_notebook_review_unit_support.rs` as a non-production transformer from `ToolNeutralFormalHookArtifact` to row-local `ProofNotebookReviewUnitArtifact` values.
2. Added `crates/mir-semantics/examples/current_l2_emit_proof_notebook_review_unit.rs` as a thin example CLI that reads formal-hook JSON and emits the derived review-unit list as JSON.
3. Added `crates/mir-semantics/tests/current_l2_proof_notebook_review_unit_support.rs` to ratchet runtime/static supported pairs plus schema/kind fail-closed behavior, unsupported pair rejection, and malformed row rejection.
4. Added `specs/examples/327...328` to fix the comparison and threshold for theorem-first concrete tool pilot on the proof-notebook review-unit shape.
5. Updated `Documentation.md`, `progress.md`, `tasks.md`, relevant `plan/` mirrors, and the Phase 6 research abstract so the current promoted line changes from theorem-first pilot to post-checkpoint drift suppression / mirror sweep.

## 4. Files changed

- `Documentation.md`
- `crates/mir-semantics/examples/current_l2_emit_proof_notebook_review_unit.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
- `crates/mir-semantics/examples/support/current_l2_formal_hook_support.rs`
- `crates/mir-semantics/examples/support/current_l2_proof_notebook_review_unit_support.rs`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- `crates/mir-semantics/tests/current_l2_proof_notebook_review_unit_support.rs`
- `docs/reports/0630-phase6-theorem-first-concrete-tool-pilot.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `specs/00-document-map.md`
- `specs/examples/327-current-l2-source-sample-authoring-bless-regression-policy-ready-theorem-first-concrete-tool-pilot-comparison.md`
- `specs/examples/328-current-l2-theorem-first-concrete-tool-pilot-ready-minimal-theorem-first-concrete-tool-pilot-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `cargo test -p mir-semantics --test current_l2_proof_notebook_review_unit_support`
  - `running 4 tests`
  - `test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
  - `running 5 tests`
  - `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e2-try-fallback --artifact-root /tmp/mir-proof-notebook-check --run-label task3-runtime --overwrite`
  - `bundle artifact: /tmp/mir-proof-notebook-check/bundles/task3-runtime/e2-try-fallback.detached.json`
  - `formal hook artifact: /tmp/mir-proof-notebook-check/formal-hooks/task3-runtime/e2-try-fallback.formal-hook.json`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e4-malformed-lineage --artifact-root /tmp/mir-proof-notebook-check --run-label task3-static --overwrite`
  - `static gate artifact: /tmp/mir-proof-notebook-check/static-gates/task3-static/e4-malformed-lineage.static-gate.json`
  - `formal hook artifact: /tmp/mir-proof-notebook-check/formal-hooks/task3-static/e4-malformed-lineage.formal-hook.json`
- `cargo run -p mir-semantics --example current_l2_emit_proof_notebook_review_unit -- /tmp/mir-proof-notebook-check/formal-hooks/task3-runtime/e2-try-fallback.formal-hook.json --output /tmp/mir-proof-notebook-check/proof-notebook/task3-runtime/e2-try-fallback.review-unit.json`
  - no stdout; file written successfully
- `cargo run -p mir-semantics --example current_l2_emit_proof_notebook_review_unit -- /tmp/mir-proof-notebook-check/formal-hooks/task3-static/e4-malformed-lineage.formal-hook.json --output /tmp/mir-proof-notebook-check/proof-notebook/task3-static/e4-malformed-lineage.review-unit.json`
  - no stdout; file written successfully

## 6. Evidence / findings

- The smallest coherent theorem-first consumer is a row-local proof-notebook review unit derived from the existing tool-neutral formal-hook artifact.
- The supported pair set can stay narrow and fail-closed: one runtime obligation and two static obligations are enough to establish the first theorem-side pressure, and multiple formal-hook rows can stay as a list of row-local units without collapsing into bridge sketch.
- Current pilot pressure is human-review oriented rather than solver-specific; this keeps bridge sketch, compare/bless metadata, proof-assistant adapter, and model-check side out of the current cut.
- The next clean mainline is snapshot maintenance, not immediate bridge-sketch widening.

## 7. Changes in understanding

- The key distinction is no longer “whether there is any theorem-side consumer” but “where the proof-notebook review unit stops before bridge sketch or concrete tool binding begins.”
- `goal_text` plus explicit checklist is enough current payload for the first concrete pilot; richer notebook workflow metadata is still later.
- The theorem-first pressure now exists in actual code without forcing public runtime/checker surface changes.

## 8. Open questions

- When to reopen proof-notebook bridge sketch after the review-unit pilot.
- Whether authored-row widening should precede or follow that bridge-sketch reopen.
- How long concrete theorem/model-check tool binding should stay behind the tool-neutral formal-hook boundary.

## 9. Suggested next prompt

- Continue with `Phase 0 / 6 post-checkpoint drift suppression / mirror sweep`, then `Phase 6 deferred authored-row widen sequencing`.
