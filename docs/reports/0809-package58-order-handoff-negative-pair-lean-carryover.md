# Report 0809 — package58 order-handoff negative pair Lean carry-over

## 1. Title and identifier

- Report ID: 0809
- Title: Package 58 order-handoff negative pair Lean carry-over

## 2. Objective

`specs/examples/527` で helper-local static stop に actualize した
`p13 / p14` late-join visibility negative pair を、
representative Lean sample set / committed `samples/lean/current-l2/` corpus に carry over し、
broader theorem-side widening を 1 本 source-backed に閉じる。

## 3. Scope and assumptions

- scope は Package 58 の broader theorem-side widening に限定する。
- final public theorem contract、proof-object public schema、final parser/public API、final source wordingは fixed しない。
- generated current-L2 Lean stub は `sorry` を含むため、current guarantee は artifact well-formedness / bridge alignment / Lean acceptance に留める。

## 4. Documents consulted

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/520-current-l2-final-layer-closeout-defaults-and-reopened-selfdriven-queue.md`
- `specs/examples/525-current-l2-delegated-rng-provider-placement-representative-lean-sample-set-carryover.md`
- `specs/examples/527-current-l2-order-handoff-negative-static-stop-actualization.md`
- `samples/lean/README.md`
- `scripts/current_l2_lean_sample_sync.py`

## 5. Actions taken

1. RED として `scripts/tests/test_current_l2_lean_sample_sync.py` の representative sample set 期待値へ `p13 / p14` を追加した。
2. `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs` に `p13 / p14` actual Lean execution regression を追加した。
3. `scripts/current_l2_lean_sample_sync.py` の export inventory に `p13 / p14` を追加し、日本語 README explanation / top-level README / manifest へ carry over できるようにした。
4. `python3 scripts/current_l2_lean_sample_sync.py` を実行し、`samples/lean/current-l2/p13...` と `p14...` を committed corpus として materialize した。
5. snapshot / roadmap / traceability / example docs を `specs/examples/528` anchor へ同期した。

## 6. Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
  - `Ran 5 tests in 0.003s`
  - `OK`
- `cargo test -p mir-runtime --test current_l2_theorem_actual_lean_execution_prototype_widening`
  - `9 passed; 0 failed`
  - `theorem_actual_lean_execution_reaches_missing_publication_witness_static_stop_prototype ... ok`
  - `theorem_actual_lean_execution_reaches_handoff_before_publish_static_stop_prototype ... ok`
- `python3 scripts/current_l2_lean_sample_sync.py`
  - `p13-dice-late-join-missing-publication-witness.bundle.json` を生成
  - `p14-dice-late-join-handoff-before-publication.bundle.json` を生成
  - `samples/lean/manifest.json` に `p13 / p14` が追加された
  - Lean verification は両 sample で `success: true`
- committed corpus:
  - `samples/lean/current-l2/p13-dice-late-join-missing-publication-witness/`
  - `samples/lean/current-l2/p14-dice-late-join-handoff-before-publication/`

## 7. What changed in understanding

- helper-local static stop に actualize 済みの negative pair でも、theorem-side representative sample set へ carry over できる。
- runtime 未到達 sample でも `fixture_static_cluster` として Lean bridge artifact を committed corpus に残せるため、broader theorem-side widening を runtime-success sample だけに限定する必要はない。
- Package 58 の次段は、generic な “broader coverage” よりも、IFC helper widening や checker-hint / diagnostics mirror のような narrower package に切りやすくなった。

## 8. Open questions

- Package 58 の次段を IFC helper widening と checker-hint / diagnostics mirror のどちらから切るか。
- representative Lean sample set をこれ以上 widen する場合の kill criteria をどこに置くか。
- final public theorem contract、proof-object public schema、final public verifier contract は still mixed gate のままである。

## 9. Suggested next prompt

Package 58 の次段として、IFC helper widening または checker-hint / diagnostics mirror を 1 本選び、source sample / helper preview / docs snapshot / report まで narrow package で actualize してください。
