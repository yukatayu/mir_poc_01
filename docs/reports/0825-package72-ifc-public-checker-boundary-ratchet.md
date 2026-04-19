# Report 0825 — Package 72 IFC public-checker-boundary ratchet

- Date: 2026-04-19 21:44 JST
- Author / agent: Codex
- Scope: Package 72 closeout for the checker-side public-checker-boundary line through helper-local operational CLI actualization
- Decision levels touched: current L2 docs-first boundary only (`specs/` normative additions at L2; no L0/L1 change)

## 1. Objective

- Close Package 72 by moving the public-checker-boundary line from docs-only comparison into helper-local operational summary actualization.
- Keep verifier handoff surface, final parser grammar, and final public verifier contract out of the current cut.
- Rewrite the snapshots so the live self-driven queue advances from public-checker-boundary ratchet to verifier-handoff-surface ratchet without queue drift.

## 2. Scope and assumptions

- Scope is limited to the checker-side IFC trio `p10 / p11 / p12` and the `mir-current-l2 run-source-sample` helper-local summary.
- `specs/examples/283` / `284` remain the normative source for the current minimum shape.
- The task does not adopt final parser grammar, generic shared public checker entry, emitted verifier handoff artifact, or final public verifier contract.

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/283-current-l2-minimal-shared-output-contract-ready-public-checker-boundary-comparison.md`
- `specs/examples/284-current-l2-public-checker-boundary-ready-minimal-public-checker-boundary-threshold.md`
- `specs/examples/543-current-l2-ifc-shared-output-contract-threshold-helper-mirror.md`
- `docs/reports/0602-phase5-public-checker-boundary-package.md`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 4. Actions taken

1. Re-read the Phase 5 boundary package (`0602`) and the docs-only boundary pair (`283` / `284`) to confirm the current first line and stop line.
2. Added RED assertions to `crates/mir-runtime/tests/current_l2_operational_cli.rs` for a new helper-local field, `actual_public_checker_boundary_threshold`.
3. Confirmed the RED phase by running the focused IFC success test and observing that the new field was still `Null`.
4. Implemented `actual_public_checker_boundary_threshold` in `crates/mir-runtime/src/current_l2_cli.rs` with the current minimum shape:
   - `boundary_kind`
   - `public_checker_command_surface_ref`
   - `shared_output_contract_ref`
5. Kept final parser grammar, query token / `checker_subject` public naming, generic shared public checker entry, detached loop wrapper path line, verifier handoff surface, and final public verifier contract as still-later refs.
6. Added `specs/examples/544-current-l2-ifc-public-checker-boundary-threshold-helper-mirror.md` as the Package 72 anchor.
7. Updated `Documentation.md` / `tasks.md` / `progress.md` / `plan/` / `specs/` so Package 72 is marked close and Package 73 verifier-handoff-surface ratchet is the live queue.

## 5. Files changed

- `Documentation.md`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `docs/reports/0825-package72-ifc-public-checker-boundary-ratchet.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/544-current-l2-ifc-public-checker-boundary-threshold-helper-mirror.md`
- `tasks.md`

## 6. Evidence / outputs / test results

- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-19 21:44:18 JST`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_reports_ifc_authority_success_checker_hint_preview -- --exact`
  - red phase: assertion failed because `value["actual_public_checker_boundary_threshold"]["status"]` was `Null`
- `cargo test -p mir-runtime --test current_l2_operational_cli`
  - `test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt --format json | rg -n 'actual_public_checker_boundary_threshold|"status": "reached"|boundary_kind|public_checker_command_surface_ref|shared_output_contract_ref|next_comparison_target_ref'`
  - includes `actual_public_checker_boundary_threshold`
  - includes `status: reached`
  - includes `boundary_kind: docs_only_parser_front_checker_boundary`
  - includes `public_checker_command_surface_ref: public_checker_command_surface_ready_sketch`
  - includes `shared_output_contract_ref: shared_output_contract_ready_sketch`
  - includes `next_comparison_target_ref: verifier_handoff_surface_comparison`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt --format pretty | rg -n 'actual_public_checker_boundary_threshold|boundary_kind|public_checker_command_surface_ref|shared_output_contract_ref|next_comparison_target_ref'`
  - includes `actual_public_checker_boundary_threshold:`
  - includes `boundary_kind: docs_only_parser_front_checker_boundary`
  - includes `public_checker_command_surface_ref: public_checker_command_surface_ready_sketch`
  - includes `shared_output_contract_ref: shared_output_contract_ready_sketch`
  - includes `next_comparison_target_ref: verifier_handoff_surface_comparison`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt --format json | rg -n 'actual_public_checker_boundary_threshold|guarded_not_reached|guard_reason|actual_public_checker_boundary_threshold_not_reached'`
  - includes `actual_public_checker_boundary_threshold`
  - includes `status: guarded_not_reached`
  - includes `guard:actual_public_checker_boundary_threshold_not_reached`
  - includes `current actual public checker boundary threshold only actualizes the IFC trio`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt --format json | rg -n 'actual_public_checker_boundary_threshold|next_comparison_target_ref|verifier_handoff_surface_comparison'`
  - includes `actual_public_checker_boundary_threshold`
  - includes `next_comparison_target_ref: verifier_handoff_surface_comparison`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 824 numbered report(s).`
- `git diff --check`
  - no output

## 7. What changed in understanding

- The next live self-driven line is no longer public-checker-boundary ratchet. That package is now closed; the queue has moved to verifier-handoff-surface ratchet.
- The most important remaining narrow implementation question on this checker-side chain is the docs-only mixed-row verifier bridge in `285` / `286`, not the parser-front boundary minimum itself.
- There is still no justification for freezing final parser grammar, generic shared public checker entry, or actual emitted verifier handoff artifact in the current task line.

## 8. Open questions

- Should the future verifier handoff surface minimum stay at `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode`, or is one more helper-local bridge field needed before minimal parser subset freeze?
- When the verifier handoff line is actualized, how should theorem / protocol / runtime-policy dedicated split remain separated from the docs-only mixed-row bridge?
- Should any later public checker boundary cut include detached loop wrapper path lines, or should they remain evidence-only?

## 9. Suggested next prompt

- `Package 73 として verifier-handoff-surface ratchet を進め、specs/examples/285 / 286 の current first line を helper-local operational CLI summary に narrow に mirror してください。actual emitted verifier handoff artifact、theorem / protocol / runtime-policy dedicated split、final parser grammar、final public verifier contract は still later に残してください。`
