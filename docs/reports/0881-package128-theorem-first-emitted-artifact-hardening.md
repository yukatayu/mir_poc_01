# Report 0881 — package128 theorem-first emitted-artifact hardening

- Date: 2026-04-20T10:02:00Z
- Author / agent: Codex
- Scope: Package 128 として `emit-theorem problem1` helper を追加し、Problem 1 theorem-first pilot を executable artifact loop として harden する
- Decision levels touched: L2

## 1. Objective

- theorem-first pilot を docs-only bundle で止めず、repo-local output dir に Lean bundle JSON を materialize する command まで actualize する。
- representative theorem line `p06 / p07 / p08` を、Problem 1 sample bundle から直接再実行できるようにする。
- final public theorem contract、concrete theorem prover brand、final public verifier contract を still later に残したまま、current emitted-artifact loop を narrow に固める。

## 2. Inputs consulted

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/508-current-l2-theorem-lean-first-nonproduction-stub-pilot-actualization.md`
- `specs/examples/509-current-l2-theorem-review-unit-to-lean-stub-repo-local-artifact-conformance-bridge.md`
- `specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `crates/mir-runtime/examples/current_l2_emit_theorem_lean_bundle.rs`

## 3. Actions taken

- `scripts/tests/test_current_l2_guided_samples.py` に RED として次を追加した。
  - `emit-theorem problem1` が bundle command surface に見えること
  - emitted-artifact row が `p06 / p07 / p08` を representative/support pair として返すこと
  - emitter command が prototype sample path と adjacent host-plan path を使うこと
  - `main(["emit-theorem", "problem1"])` が runtime renderer を通ること
- `scripts/current_l2_guided_samples.py` に次を追加した。
  - `ProblemTheoremEmitRow`
  - `emit-theorem problem1 [--format pretty|json] [--output-dir <path>]`
  - `target/current-l2-guided/problem1-theorem-pilot` default output dir
  - `current_l2_emit_theorem_lean_bundle` example を prototype sample path + host-plan path 付きで呼び出す helper
  - pretty/json summary renderer
- runtime 検証で、sample ID だけでは emitter example が prototype sample を解決できないこと、および host-plan 明示が必要なことを確認し、helper command builder を sample path + `.host-plan.json` 付きへ修正した。
- `samples/problem-bundles/problem1-typed-theorem-model-check.md` に emitted-artifact loop の execution step を追加した。
- `specs/examples/601-current-l2-theorem-first-emitted-artifact-loop-hardening.md`
  を追加し、current cut / recommendation / stop line を文書化した。
- `Documentation.md`、`plan/01`、`plan/11`、`plan/17`、`plan/18`、`plan/90`、`progress.md`、`tasks.md`、`specs/00`、`specs/11`、`specs/12`
  を Package 128 closeout 読みに同期した。

## 4. Files changed

- `docs/reports/0881-package128-theorem-first-emitted-artifact-hardening.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/601-current-l2-theorem-first-emitted-artifact-loop-hardening.md`

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - RED:
    - `module 'current_l2_guided_samples' has no attribute 'build_problem1_theorem_emit_rows'`
    - `module 'current_l2_guided_samples' has no attribute 'ProblemTheoremEmitRow'`
    - `emit-theorem problem1` not found in bundle text
  - GREEN:
    - `Ran 62 tests`
    - `OK`
- `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - RED:
    - `emit-theorem problem1` not found in Problem 1 sample bundle doc
  - GREEN:
    - `Ran 3 tests`
    - `OK`
- `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
  - first failure:
    - `source sample not found: p06-typed-proof-owner-handoff`
  - second failure:
    - `host plan did not cover all oracle calls`
  - final GREEN:
    - `output dir: target/current-l2-guided/problem1-theorem-pilot`
    - `p06-typed-proof-owner-handoff: representative theorem-first sample`
    - `p07-dice-late-join-visible-history: theorem-reached support sample`
    - `p08-dice-stale-reconnect-refresh: theorem-reached support sample`
- `python3 scripts/current_l2_guided_samples.py emit-theorem problem1 --format json`
  - `problem_id: problem1`
  - `rows[0].pilot_status: reached`
  - `rows[1].output_path: target/current-l2-guided/problem1-theorem-pilot/p07-dice-late-join-visible-history.lean-bundle.json`

## 6. Evidence / findings

- theorem-first pilot はすでに sample bundle / Lean corpus / bridge docs を持っていたため、next useful ratchet は public contract comparison の追加ではなく emitted artifact loop の repo-local materialization だった。
- `emit-theorem problem1` により、`p06` representative と `p07 / p08` theorem-reached support pair を 1 コマンドで bundle JSON に落とし直せるようになり、Problem 1 bundle doc の theorem-first line が「読むだけ」ではなくなった。
- 実 runtime で踏んだ 2 つの失敗
  - bare sample ID では emitter example が prototype sample を見つけない
  - adjacent host-plan を渡さないと oracle call coverage で止まる
  を command builder に吸収したことで、artifact loop の drift point を helper 側で潰せた。

## 7. Changes in understanding

- theorem-first emitted-artifact hardening の本質は新しい theorem schema 比較ではなく、既存 pilot を repo-local execution loop に下ろすことだった。
- Problem 1 current first line は `check-source-sample` focused checker slice と `emit-theorem problem1` emitted-artifact loop の 2 本が揃ったことで、typed/theorem bridge の runnable evidence が一段厚くなった。

## 8. Open questions

- authoritative-room first scenario 側でも emitted artifact / smoke / negative pair をどう 1 loop に束ねるか。
- theorem-side output を final public theorem contract へ進める前に、どこまで repo-local command surface を増やすか。
- concrete theorem prover brand や final proof object public schema を reopen する条件をどの package で narrow に書くか。

## 9. Suggested next prompt

- `Package 128 は閉じたので、次は authoritative-room runnable scenario hardening を進めてください。`
