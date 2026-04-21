# Report 0898 — research abstract detailed line by line guides

- Date: 2026-04-21T00:22:32.623357Z
- Author / agent: Codex
- Scope: `docs/research_abstract/` の detailed guide 3 本追加、README 導線更新、`progress.md` 更新、検証
- Decision levels touched: L0/L1 規範変更なし。user-facing documentation hardening のみ。

## 1. Objective

- `docs/research_abstract/static_analysis_01_detail.md` を新規追加し、`static_analysis_01.md` で紹介している Problem 1 sample 群と Lean foundation を、コード全文・行単位解説・再現コマンド・出力の読み方つきで standalone に整理する。
- `docs/research_abstract/order_01_detail.md` を新規追加し、`order_01.md` で紹介している Problem 2 sample 群と reserve helper を、コード全文・行単位解説・再現コマンド・出力の読み方つきで standalone に整理する。
- `docs/research_abstract/lean_01_detail.md` を新規追加し、`lean_01.md` で紹介している Lean foundation / generated stub / 最小 success-error 例を、コード全文・行単位解説・再現コマンド・出力の読み方つきで standalone に整理する。
- 上記 3 本を README / progress から辿れる状態にし、実行コマンドと出力が current workspace で再現できることを fresh に確認する。

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
- `plan/00-index.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/static_analysis_01.md`
- `docs/research_abstract/order_01.md`
- `docs/research_abstract/lean_01.md`
- `samples/prototype/current-l2-typed-proof-model-check/p06/p10/p11/p12/p15/p16`
- `samples/prototype/current-l2-order-handoff/p07/p08/p09/p13/p14`
- `samples/lean/foundations/CurrentL2LabelModel.lean`
- `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
- `samples/lean/foundations/CurrentL2ProofSkeleton.lean`
- `samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean`
- `samples/lean/current-l2/e5-underdeclared-lineage/e5-underdeclared-lineage.lean`

## 3. Actions taken

- repo-scoped Discord baseline を `begin` で記録した。
- AGENTS.md の順序に従って README / Documentation / progress / specs / plan / current research abstracts を読み、今回の task が user-facing documentation hardening であり、規範変更ではないことを確認した。
- `df -h .` と `free -h` で資源状況を先に確認した。
- `smoke-all`、Problem 1 helper 群、Problem 2 helper 群、typed sample 実行、order sample 実行、Lean foundation、generated stub、Lean sync、unit test を再実行し、detail docs に載せる再現コマンドと current output を fresh に取り直した。
- `docs/research_abstract/static_analysis_01_detail.md` を新規作成した。
- subagent を使って `docs/research_abstract/order_01_detail.md` と `docs/research_abstract/lean_01_detail.md` を作成し、main workspace で受け取った後にリンク整合を見直した。
- `docs/research_abstract/README.md` に detail docs への導線を追加した。
- `progress.md` の最終更新時刻、evidence bullets、layer/track progress、recent log を更新した。
- `plan/` は今回の task では update 不要と判断した。
- `tasks.md` は今回の task では update 不要と判断した。

## 4. Files changed

- 追加:
  - `docs/research_abstract/static_analysis_01_detail.md`
  - `docs/research_abstract/order_01_detail.md`
  - `docs/research_abstract/lean_01_detail.md`
- 更新:
  - `docs/research_abstract/README.md`
  - `progress.md`
  - `docs/reports/0898-research-abstract-detailed-line-by-line-guides.md`

## 5. Commands run and exact outputs

- resource checks:
  - `df -h .`
    - `/dev/vda2 99G used 83G avail 12G use% 88%`
  - `free -h`
    - `Mem total 960Mi used 655Mi free 77Mi available 304Mi`
- bundle smoke:
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
    - `problem1 status: passed`
    - `problem2 status: passed`
- Problem 1 helper:
  - `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
    - `p06-typed-proof-owner-handoff: representative theorem-first sample`
    - `pilot_status: reached`
    - `lean_stub_artifact_count: 1`
  - `python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line`
    - `p10 ... terminal_outcome: success`
    - `p11 / p12 / p15 / p16 ... terminal_outcome: reject`
- Problem 1 sample runs:
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p10... --format pretty`
    - `static_gate_verdict: valid`
    - `terminal_outcome: success`
    - `typed_checker_hint_status: reached`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p11... --format pretty`
    - `static_gate_verdict: valid`
    - `terminal_outcome: Reject`
    - `typed_checker_hint_status: reached`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p12... --format pretty`
    - `static_gate_verdict: valid`
    - `terminal_outcome: Reject`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p15... --format pretty`
    - `static_gate_verdict: valid`
    - `terminal_outcome: Reject`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p16... --format pretty`
    - `static_gate_verdict: valid`
    - `terminal_outcome: Reject`
- Problem 2 helper:
  - `python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
    - `p07 / p08 success`
    - `p09 reserve practical route`
    - `p13 underdeclared`
    - `p14 malformed`
  - `python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness`
    - `p07 witness_strengthening_status: reached`
    - `p08 witness_strengthening_status: guarded_not_reached`
  - `python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service`
    - `p09 delegated_rng_service_status: reached`
- Problem 2 sample runs:
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample ...p07... --format pretty`
    - `static_gate: valid`
    - `entered_evaluation: true`
    - `terminal_outcome: success`
    - `steps_executed: 9`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample ...p08... --format pretty`
    - `static_gate: valid`
    - `terminal_outcome: success`
    - `steps_executed: 7`
    - `formal_hook_status: reached`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample ...p09... --format pretty`
    - `static_gate: valid`
    - `entered_evaluation: true`
    - `terminal_outcome: success`
    - `steps_executed: 11`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample ...p13... --format pretty`
    - `static_gate: underdeclared`
    - `entered_evaluation: false`
    - `steps_executed: 0`
    - `missing publication witness before handoff ...`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample ...p14... --format pretty`
    - `static_gate: malformed`
    - `entered_evaluation: false`
    - `steps_executed: 0`
    - `handoff appears before publish ...`
- Lean:
  - `source "$HOME/.elan/env" && lean --version`
    - `Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)`
  - `source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2LabelModel.lean && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean && lean samples/lean/foundations/CurrentL2ProofSkeleton.lean`
    - 出力なしで成功
  - `source "$HOME/.elan/env" && lean samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean`
    - `warning: declaration uses 'sorry'`
  - `source "$HOME/.elan/env" && lean samples/lean/current-l2/e5-underdeclared-lineage/e5-underdeclared-lineage.lean`
    - `warning: declaration uses 'sorry'` が 2 本
  - `source "$HOME/.elan/env" && python3 scripts/current_l2_lean_sample_sync.py`
    - foundation 4 本 `success: true`
    - current-l2 generated stubs `success: true` + `warning: declaration uses 'sorry'`
  - `python3 -m unittest scripts/tests/test_current_l2_lean_sample_sync.py`
    - `Ran 9 tests in 0.001s`
    - `OK`
- environment versions:
  - `python3 --version`
    - `Python 3.12.3`
  - `cargo --version`
    - `cargo 1.93.0 (083ac5135 2025-12-15)`

## 6. Evidence / findings

- Problem 1 / Problem 2 / Lean それぞれについて、概要版だけでは「どの行が何を宣言しているか」が読みにくい、という user 指摘に対し、detail docs で source code 全文と行単位説明を付けることで入口の不透明さを解消できた。
- Problem 1 では、authority miss / label-flow mismatch / capture-lifetime / simple cost という rejection 理由が互いに別であることを、sample ごとに分けて説明できるようになった。
- Problem 2 では、`underdeclared` と `malformed` の違いを `p13` / `p14` の negative pair と結び付けて読めるようになった。
- Lean では、foundation success と generated stub warning の差を、`warning: declaration uses 'sorry'` と `error: unsolved goals` の読み方まで含めて独立に説明できるようになった。
- `docs/research_abstract/README.md` から detail docs を直接辿れるようにした。
- `progress.md` に user-facing documentation hardening の進捗を反映した。

## 7. Changes in understanding

- 今回の task では規範判断そのものは変えていない。変えたのは「既にある current reading を、初学者が追える粒度まで展開する」部分である。
- Problem 1 の本質説明では、foundation proof と generated stub を明確に分けて書かないと、`Lean に通る artifact` と `本当に証明済みの補題` が混ざって読まれやすい。
- Problem 2 の説明では、`publication witness` を独立構文と誤読されないように、「publish の先行と helper / checker rule から読める根拠」として説明するのが重要だと分かった。

## 8. Open questions

- なし。今回スコープでは docs hardening を完了した。
- `plan/` 更新不要。
- `tasks.md` 更新不要。

## 9. Suggested next prompt

- `static_analysis_01_detail.md` / `order_01_detail.md` / `lean_01_detail.md` を読んだ上で、さらに「この節の説明をもっと短くしてほしい」「この sample の出力全文も別 appendix に欲しい」といった粒度調整を依頼してください。
