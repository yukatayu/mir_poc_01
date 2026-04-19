# Report 0802 — lean sample corpus and ifc first fragment

- Date: 2026-04-19T05:35:27.567190Z
- Author / agent: Codex (GPT-5)
- Scope: Package 56 / 57 の first actual fragment として、`samples/lean/` committed corpus、Lean export/sync helper、IFC label-model first fragment、proof-skeleton first fragment を repo-local evidence と docs/plan snapshot に統合する。
- Decision levels touched: L2

## 1. Objective

- `samples/lean/` を新設し、Lean で実際に通した内容を repo 内に explanation 付きで保存する。
- generated theorem stub と actual small proof を同じ強さで誤読しないよう、proof-strength wording を docs/plan に固定する。
- Package 57 の first slice を close し、Package 56 を stronger IFC corpus と explicit authority sample family に narrowed する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `crates/mir-semantics/examples/support/current_l2_lean_theorem_stub_support.rs`
- `crates/mir-semantics/tests/current_l2_lean_theorem_stub_actual_probe.rs`
- `crates/mir-runtime/tests/current_l2_theorem_actual_lean_execution_prototype_widening.rs`
- `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
- `scripts/current_l2_theorem_toolchain_probe.py`
- `scripts/current_l2_source_sample_regression.py`

## 3. Actions taken

1. Lean stub emission / actual execution route を再読し、current runtime/semantics helper から representative sample を bundle 化できる narrow helper が足りないことを確認した。
2. failing test として `scripts/tests/test_current_l2_lean_sample_sync.py` を追加し、representative quartet、foundation file set、proof-strength explanation の最低条件を先に固定した。
3. `crates/mir-runtime/examples/current_l2_emit_theorem_lean_bundle.rs` を追加し、current theorem Lean-stub pilot actualization から sample-aware JSON bundle を出せる helper を実装した。
4. `scripts/current_l2_lean_sample_sync.py` を追加し、Lean toolchain discovery、foundation file 書き出し、representative quartet bundle export、Lean verification、manifest 生成、README 生成を一括化した。
5. `lean-toolchain` を追加し、repo-local toolchain pin を明示した。
6. `samples/lean/` を生成し、`foundations/` に actual small proof fragment、`current-l2/` に representative theorem quartet generated stub corpus を保存した。
7. `specs/examples/521...`、`Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/` / `specs/` を更新し、Package 57 first slice close と Package 56 narrowed queue を反映した。

## 4. Files changed

- New
  - `crates/mir-runtime/examples/current_l2_emit_theorem_lean_bundle.rs`
  - `scripts/current_l2_lean_sample_sync.py`
  - `scripts/tests/test_current_l2_lean_sample_sync.py`
  - `lean-toolchain`
  - `specs/examples/521-current-l2-lean-sample-corpus-and-first-foundations.md`
  - `samples/lean/README.md`
  - `samples/lean/manifest.json`
  - `samples/lean/foundations/CurrentL2LabelModel.lean`
  - `samples/lean/foundations/CurrentL2LabelModel.md`
  - `samples/lean/foundations/CurrentL2ProofSkeleton.lean`
  - `samples/lean/foundations/CurrentL2ProofSkeleton.md`
  - `samples/lean/current-l2/...` representative quartet bundle / `.lean` / README files
- Updated
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `specs/00-document-map.md`
  - `specs/10-open-questions.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `specs/12-decision-register.md`
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/10-roadmap-overall.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`

## 5. Commands run and exact outputs

- `df -h .`
  - overlay 16G used 277M avail 16G
- `free -h`
  - Mem 15Gi / used 1.4Gi / avail 13Gi
- `source "$HOME/.elan/env" && lean --version`
  - `Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
  - `Ran 3 tests ... OK`
- `python3 scripts/current_l2_lean_sample_sync.py`
  - success
  - representative quartet bundle export + foundation verification completed
  - current-L2 generated files reported `warning: declaration uses 'sorry'`
- `cargo test -p mir-runtime --test current_l2_theorem_actual_lean_execution_prototype_widening --test current_l2_theorem_lean_stub_pilot_actualization`
  - `3 passed`
  - `5 passed`
- `cargo test -p mir-semantics --test current_l2_lean_theorem_stub_actual_probe`
  - `1 passed`
- `python3 scripts/current_l2_source_sample_regression.py inventory`
  - authored sixteen all present
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
- `git diff --check`
  - success

## 6. Evidence / findings

- `samples/lean/foundations/CurrentL2LabelModel.lean`
  - actual small proofs for two-point label model, `flowsTo`, `join`, and explicit authority-sensitive declassification facts
- `samples/lean/foundations/CurrentL2ProofSkeleton.lean`
  - actual small proofs for review-unit to emitted-stub shape preservation
- `samples/lean/current-l2/e5|p06|p07|p08/*.lean`
  - representative theorem quartet committed in repo
  - Lean accepts them
  - they still contain `sorry`, so they are bridge-alignment / well-formedness evidence rather than theorem completion
- `scripts/current_l2_lean_sample_sync.py`
  - regenerates committed corpus and verifies it
- `crates/mir-runtime/examples/current_l2_emit_theorem_lean_bundle.rs`
  - closes the missing helper gap between runtime theorem actualization and committed Lean artifacts

## 7. Changes in understanding

- Lean line は「toolchain probe済み / temp-fileだけで回る状態」から、「repo-local committed corpus と actual small proof fragment を持つ状態」へ進んだ。
- Package 57 は first slice を close してよい。remaining work は proof-skeleton widening や public proof contract ではなく、Package 56 の stronger IFC corpus と Package 58 の helper/CLI hardening に移った。
- generated stub と actual proof fragment は同じ「Lean verified」とは書けない。current docs ではその split を explicit に保持する必要がある。

## 8. Open questions

- secret-key valid/invalid と explicit declassification authority をどこまで source sample / checker fragment に actualize するか。
- `samples/lean/current-l2/` をどこまで broader theorem/model-check/order-handoff corpus に widen するか。
- proof-skeleton first fragment を超えて、どの時点で public proof artifact discussion に近づけるか。

## 9. Suggested next prompt

Package 56 を続け、secret-key valid/invalid と explicit declassification authority の first corpus を source-side / checker-side に actualize し、必要なら `samples/lean/` と対応づけながら docs / tests / snapshot を再同期してください。
