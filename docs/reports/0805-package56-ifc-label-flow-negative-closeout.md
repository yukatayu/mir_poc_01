# Report 0805 — package56 ifc label-flow negative closeout

- Date: 2026-04-19T07:25:00Z
- Author / agent: Codex
- Scope: Package 56 の label-flow negative source-side prototype actualization、representative Lean sample set widening、Package 56 closeout sync
- Decision levels touched: L1 / L2 / L3

## 1. Objective

Package 56 の narrowed remainder だった label-flow negative を source-side corrected prototype として actualize し、

- authority-miss negative と distinct な IFC negative を sample-visible にすること
- representative Lean sample set を `p12` まで widen すること
- docs / plan / progress / tasks / traceability を Package 56 close 読みに同期すること

を目的とした。

## 2. Inputs consulted

- `AGENTS.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/520-current-l2-final-layer-closeout-defaults-and-reopened-selfdriven-queue.md`
- `specs/examples/521-current-l2-lean-sample-corpus-and-first-foundations.md`
- `specs/examples/522-current-l2-ifc-secret-valid-invalid-foundation-and-japanese-lean-corpus-sync.md`
- `specs/examples/523-current-l2-source-side-ifc-authority-prototype-pair-and-representative-lean-sample-set-widening.md`
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
- `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
- `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`

## 3. Actions taken

1. `p12-typed-classified-fingerprint-publication-block` と adjacent host-plan sidecar を追加した。
2. 先に tests を widening し、missing sample を red phase で確認した。
3. 初回 host-plan では predicate coverage だけでは足りず、effect oracle 側にも explicit failure rule が必要だと確認した。
4. host-plan に publish operation の `explicit-failure` verdict を追加し、authority-miss negative とは異なる label-flow negative を runnable にした。
5. source runner、verifier preview alignment、model-check projection prefloor、theorem actual Lean execution widening を `p12` まで広げた。
6. Lean export / sync helper を `p12` まで広げ、`samples/lean/current-l2/p12-*` を committed corpus に actualize した。
7. `specs/examples/524` と `D-112` を追加し、Package 56 を first actual adoption package として close する current reading を docs に反映した。
8. `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、traceability を current snapshot へ同期した。

## 4. Files changed

- `samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt`
- `samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.host-plan.json`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_verifier_preview_alignment.rs`
- `crates/mir-runtime/tests/current_l2_model_check_projection_prefloor.rs`
- `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`
- `samples/lean/current-l2/p12-typed-classified-fingerprint-publication-block/*`
- `samples/lean/README.md`
- `samples/lean/manifest.json`
- `samples/prototype/README.md`
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
- `specs/examples/524-current-l2-ifc-label-flow-negative-prototype-closeout-and-representative-lean-sample-set-widening.md`

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
  - red phase では `p12` missing を検出、その後 green 化
- `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_ifc_authority_typed_prototype_paths -- --exact`
  - red phase では sample missing、次に effect oracle uncovered を検出、その後 green 化
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt --format json`
  - `terminal_outcome: "Reject"`
  - events: `perform-success`, `atomic-cut`, `perform-failure`, `Reject`
- `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment verifier_preview_alignment_matches_emitted_route_for_label_flow_negative_ifc_typed_runtime_prototype -- --exact`
  - pass
- `cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor model_check_projection_prefloor_reaches_label_flow_negative_ifc_typed_runtime_prototype -- --exact`
  - pass
- `cargo test -p mir-runtime --test current_l2_theorem_actual_lean_execution_prototype_widening theorem_actual_lean_execution_reaches_label_flow_negative_ifc_runtime_prototype -- --exact`
  - pass
- `python3 scripts/current_l2_lean_sample_sync.py`
  - Lean `4.29.1` で regenerate + verification pass

## 6. Evidence / findings

- `p12` は authority 欠如ではなく label-flow mismatch によって止まる distinct negative として実装できた。
- `p12` は `request-require` を通した後、`request-ensure` / effect-side explicit failure を経て `Reject` へ落ちる。
- representative Lean sample set は `e5 / p06 / p10 / p11 / p12 / p07 / p08` に widen しても regenerate / verify 可能だった。
- Package 56 の source-side IFC floor は `p10 / p11 / p12` で close してよい。

## 7. Changes in understanding

- source-side IFC negative は 1 種類では足りず、authority-miss negative と label-flow negative を分けることで current checker-adjacent IFC line の stop line が明確になった。
- host-plan は predicate verdict だけでなく、effect oracle coverage も必要であるため、label-flow negative を runnable にするには explicit failure verdict を sidecar へ書く必要があった。
- Package 56 の live debt は label-flow negative ではなくなり、helper/CLI hardening と broader coverage widening へ移った。

## 8. Open questions

- checker-hint / diagnostics 側へ source-side IFC trio をどこまで mirror するか。
- representative sample set widening を helper/CLI hardening とどう結び付けるか。
- final typed source principal、final IFC syntax、final public verifier contract は still mixed gate である。

## 9. Suggested next prompt

Package 58 として helper / CLI hardening と broader theorem-side / IFC / order-handoff representative coverage widening を narrow package で進める。
