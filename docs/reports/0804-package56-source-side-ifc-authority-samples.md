# Report 0804 — package56 source side ifc authority samples

- Date: 2026-04-19T06:47:48.504831Z
- Author / agent: Codex
- Scope: Package 56 の source-side IFC authority corrected prototype pair actualization、representative Lean sample set widening、snapshot / roadmap / traceability 同期
- Decision levels touched: L1 / L2 / L3

## 1. Objective

`specs/examples/520` が reopen した Package 56 の current live debt を狭めるため、

- source-side explicit authority declassification success/failure pair を corrected prototype として actualize すること
- representative Lean sample set を `e5 / p06 / p10 / p11 / p07 / p08` へ widen すること
- docs / plan / progress / traceability を current checker-adjacent IFC first-fragment 読みに同期すること

を目的とした。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/520-current-l2-final-layer-closeout-defaults-and-reopened-selfdriven-queue.md`
- `specs/examples/521-current-l2-lean-sample-corpus-and-first-foundations.md`
- `specs/examples/522-current-l2-ifc-foundations-and-secret-valid-invalid-concrete-example.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `samples/prototype/README.md`
- `samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt`
- `samples/lean/foundations/CurrentL2LabelModel.lean`
- `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`

## 3. Actions taken

1. `p10-typed-authorized-fingerprint-declassification` と `p11-typed-unauthorized-fingerprint-release` を source-side corrected prototype として追加した。
2. それぞれに adjacent `.host-plan.json` sidecar を追加し、current runner / CLI / theorem-model-check preview floor に乗る形へ整えた。
3. 先に regression を足し、missing sample と `p11` の actual runtime outcome が `Reject` であることを red phase で確認した。
4. `current_l2_source_sample_runner`、verifier preview、model-check projection、theorem Lean actual-execution widening の focused tests を追加し、期待値を actual runtime outcome に合わせて green へ戻した。
5. `scripts/current_l2_lean_sample_sync.py` を widen し、representative Lean sample set を `e5 / p06 / p10 / p11 / p07 / p08` へ更新した。
6. `python3 scripts/current_l2_lean_sample_sync.py` を実行し、`samples/lean/current-l2/p10-*` と `p11-*` の committed Lean corpus、`samples/lean/manifest.json`、`samples/lean/README.md` を再生成した。
7. `specs/examples/523` を追加し、stronger typed surface を principal に上げず checker-adjacent principal を維持したまま、source-side explicit authority pair と representative Lean sample set widening を current recommendation として固定した。
8. `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、`specs/10..12`、`specs/00-document-map.md`、`plan/90-source-traceability.md` を current snapshot に同期した。
9. stale wording として残っていた `representative theorem quartet` / `corrected prototype nonet` を current snapshot から除去し、historical package wording と current snapshot wording を切り分けた。

## 4. Files changed

- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_verifier_preview_alignment.rs`
- `crates/mir-runtime/tests/current_l2_model_check_projection_prefloor.rs`
- `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
- `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.host-plan.json`
- `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`
- `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.host-plan.json`
- `samples/prototype/README.md`
- `samples/lean/current-l2/p10-typed-authorized-fingerprint-declassification/*`
- `samples/lean/current-l2/p11-typed-unauthorized-fingerprint-release/*`
- `samples/lean/README.md`
- `samples/lean/manifest.json`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/520-current-l2-final-layer-closeout-defaults-and-reopened-selfdriven-queue.md`
- `specs/examples/521-current-l2-lean-sample-corpus-and-first-foundations.md`
- `specs/examples/523-current-l2-source-side-ifc-authority-prototype-pair-and-representative-lean-sample-set-widening.md`
- `docs/reports/0804-package56-source-side-ifc-authority-samples.md`

## 5. Commands run and exact outputs

- `df -h .`
  - `/dev/vda2 99G size, 82G used, 13G avail`
- `free -h`
  - `Mem: 960Mi total, 708Mi used, 66Mi free, 251Mi available`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
  - red phase で `p10` / `p11` 未登録を検出、その後 green 化
- `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_ifc_authority_typed_prototype_paths -- --exact`
  - red phase で sample missing、次に `p11` の actual runtime outcome が `Reject` であることを確認、その後 green 化
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt --format json`
  - `terminal_outcome: "success"`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt --format json`
  - `terminal_outcome: "Reject"`
- `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment verifier_preview_alignment_matches_emitted_route_for_authorized_ifc_typed_runtime_prototype -- --exact`
  - pass
- `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment verifier_preview_alignment_matches_emitted_route_for_unauthorized_ifc_typed_runtime_prototype -- --exact`
  - pass
- `cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor model_check_projection_prefloor_reaches_authorized_ifc_typed_runtime_prototype -- --exact`
  - pass
- `cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor model_check_projection_prefloor_reaches_unauthorized_ifc_typed_runtime_prototype -- --exact`
  - pass
- `cargo test -p mir-runtime --test current_l2_theorem_actual_lean_execution_prototype_widening theorem_actual_lean_execution_reaches_authorized_ifc_runtime_prototype -- --exact`
  - pass
- `cargo test -p mir-runtime --test current_l2_theorem_actual_lean_execution_prototype_widening theorem_actual_lean_execution_reaches_unauthorized_ifc_runtime_prototype -- --exact`
  - pass
- `python3 scripts/current_l2_lean_sample_sync.py`
  - Lean `4.29.1` を用いた regenerate + verification が pass

## 6. Evidence / findings

- `p10` は explicit authority declassification の positive source-side example として runner / preview / theorem/model-check bridge に乗る。
- `p11` は authority を欠く negative source-side example として runner / preview / theorem/model-check bridge に乗る。
- `p11` の runtime 終端は `ExplicitFailure` ではなく `Reject` である。
  - これは `perform-failure` 後に review verdict が reject path へ落ちる current semantics をそのまま反映している。
- representative Lean sample set widening 後も existing helper route は壊れず、committed Lean corpus と manifest は再生成可能だった。
- current recommendation は維持される。
  - stronger typed surface は still early source principal に上げない。
  - checker-adjacent principal + layered IFC first-fragment + source-side explicit authority pair を first line に置く。

## 7. Changes in understanding

- Package 56 の live debt は、source-side explicit authority declassification success/failure pair 自体ではなくなった。
- current narrowed debt は、label-flow negative、checker-fragment corpus integration、broader helper/CLI hardening に移った。
- representative Lean sample corpus の current reading は historical quartet ではなく、`e5 / p06 / p10 / p11 / p07 / p08` を含む representative sample set で読む方が整合的である。

## 8. Open questions

- label-flow negative を source-side corrected prototype としてどこまで追加するか。
- `CurrentL2LabelModel.lean` / `CurrentL2IfcSecretExamples.lean` の first fragment を checker-fragment docs / tests 側へどこまで mirror するか。
- broader theorem-side / order-handoff / IFC corpus widening を Package 58 でどの順番に切るか。
- final public typed surface、final theorem/model-check public contract、final parser/public API は still mixed gate のままである。

## 9. Suggested next prompt

Package 56 の narrowed remainder として label-flow negative / checker-fragment integration を 1 つ actualize し、その後 Package 58 の helper / CLI hardening と broader representative coverage widening へ進める。
