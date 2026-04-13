# Report 0675 — Phase 6 model-check concrete carrier first actualization package

- Date: 2026-04-13T06:53:00Z
- Author / agent: Codex
- Scope: tool-neutral formal hook を hard input にする row-local machine-facing sibling artifact を `mir-semantics` helper-local に actualize し、repo-level current line を source-sample emitted verification artifact wiring へ進める。
- Decision levels touched: L2

## 1. Objective

- `model-check concrete carrier actualization comparison` fixed 後の next package として、first actual carrier を narrow に actualize する。
- `proof_notebook_review_unit` current first theorem-side pilot を維持しつつ、machine-facing sibling artifact を row-local list として追加する。
- runtime runner / regression helper / sample-facing bless-review flow をまだ混ぜない。

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
- `specs/examples/327...328`
- `specs/examples/341...342`
- `specs/examples/359...360`
- `specs/examples/367...368`
- `specs/examples/377...378`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_003.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `crates/mir-semantics/examples/support/current_l2_formal_hook_support.rs`
- `crates/mir-semantics/examples/support/current_l2_proof_notebook_review_unit_support.rs`
- `crates/mir-semantics/examples/current_l2_emit_proof_notebook_review_unit.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`

## 3. Actions taken

- TDD で `crates/mir-semantics/tests/current_l2_model_check_carrier_support.rs` を先に追加し、support module 未実装の状態で compile failure を確認した。
- `crates/mir-semantics/examples/support/current_l2_model_check_carrier_support.rs` を追加し、tool-neutral formal hook から row-local machine-facing carrier list を作る pure transform を actualize した。
- current cut を `schema_version + artifact_kind + subject_kind + subject_ref + case(obligation_kind + evidence_refs)` に留め、schema/kind mismatch、unsupported pair、empty `subject_ref`、empty `contract_rows`、empty `evidence_refs` を fail-closed に止めた。
- `crates/mir-semantics/examples/current_l2_emit_model_check_carrier.rs` を追加し、formal-hook JSON を読んで model-check carrier JSON を出す thin example emitter を actualize した。
- runtime 1 本 (`e2`) と static 1 本 (`e4`) の formal-hook smoke から emitter を通し、generated JSON を確認した。
- `specs/examples/379...380` と snapshot / helper-stack docs を更新し、repo-level current line を `source-sample emitted verification artifact wiring` に進めた。

## 4. Files changed

- `.docs/current-l2-source-sample-authoring-policy.md`
- `Documentation.md`
- `crates/mir-semantics/examples/current_l2_emit_model_check_carrier.rs`
- `crates/mir-semantics/examples/support/current_l2_model_check_carrier_support.rs`
- `crates/mir-semantics/tests/current_l2_model_check_carrier_support.rs`
- `docs/reports/0675-phase6-model-check-concrete-carrier-first-actualization-package.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_003.md`
- `plan/01-status-at-a-glance.md`
- `plan/07-parser-free-poc-stack.md`
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
- `specs/examples/379-current-l2-model-check-concrete-carrier-actualization-comparison-ready-model-check-concrete-carrier-first-actualization-comparison.md`
- `specs/examples/380-current-l2-model-check-concrete-carrier-first-actualization-ready-minimal-model-check-concrete-carrier-first-actualization-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-13 15:53 JST`
- `cargo test -p mir-semantics --test current_l2_model_check_carrier_support`
  - `error: couldn't read crates/mir-semantics/tests/../examples/support/current_l2_model_check_carrier_support.rs: No such file or directory (os error 2)`
- `cargo test -p mir-semantics --test current_l2_model_check_carrier_support`
  - `4 passed`
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
  - `5 passed`
- `cargo test -p mir-semantics --test current_l2_proof_notebook_review_unit_support`
  - `4 passed`
- `cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder`
  - `8 passed`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-runtime e2-try-fallback --artifact-root target/current-l2-model-check-smoke --run-label pkg0675-e2 --overwrite`
  - `bundle artifact: target/current-l2-model-check-smoke/bundles/pkg0675-e2/e2-try-fallback.detached.json`
  - `formal hook artifact: target/current-l2-model-check-smoke/formal-hooks/pkg0675-e2/e2-try-fallback.formal-hook.json`
- `cargo run -p mir-semantics --example current_l2_emit_model_check_carrier -- target/current-l2-model-check-smoke/formal-hooks/pkg0675-e2/e2-try-fallback.formal-hook.json --output target/current-l2-model-check-smoke/model-check/pkg0675-e2/e2-try-fallback.model-check.json`
  - `Running target/debug/examples/current_l2_emit_model_check_carrier ...`
- `python3 scripts/current_l2_detached_loop.py smoke-formal-hook-static e4-malformed-lineage --artifact-root target/current-l2-model-check-smoke --run-label pkg0675-e4 --overwrite`
  - `static gate artifact: target/current-l2-model-check-smoke/static-gates/pkg0675-e4/e4-malformed-lineage.static-gate.json`
  - `formal hook artifact: target/current-l2-model-check-smoke/formal-hooks/pkg0675-e4/e4-malformed-lineage.formal-hook.json`
- `cargo run -p mir-semantics --example current_l2_emit_model_check_carrier -- target/current-l2-model-check-smoke/formal-hooks/pkg0675-e4/e4-malformed-lineage.formal-hook.json --output target/current-l2-model-check-smoke/model-check/pkg0675-e4/e4-malformed-lineage.model-check.json`
  - `Running target/debug/examples/current_l2_emit_model_check_carrier ...`

## 6. Evidence / findings

- row-local machine-facing carrier listは、current theorem-side pilot と最小に並走できる first actual carrier cut だった。
- hard input を formal hook に限定したことで、current `e3` guarded line を bypass せずに済んだ。
- runtime `e2` と static `e4` の generated JSON は、`subject_kind + subject_ref + case(obligation_kind + evidence_refs)` の narrow shapeに揃った。

## 7. Changes in understanding

- compare-ready bridge sketch は current package では code hard input ではなく docs-only context として十分であり、first actual carrier は formal hook only hard input の sibling artifact に留めるのが自然だと確認できた。
- source-sample emitted verification artifact wiring は actual carrier shape を受け取る next package として clean に分離できる。

## 8. Open questions

- source-sample emitted verification artifact wiring の first integration surface を runtime helper と regression helper のどちらに置くか。
- sample-facing summary package で review-unit と model-check carrier をどう並べるか。
- model-check carrier の later bundle shape を必要に応じてどこで reopen するか。

## 9. Suggested next prompt

- `tasks.md` の current line どおり、source-sample emitted verification artifact wiring を進めてください。current authored sample octet と verification ladder を壊さず、runtime helper / regression helper / emitted model-check carrier の接点を helper-local actual path として narrow に整理してください。
