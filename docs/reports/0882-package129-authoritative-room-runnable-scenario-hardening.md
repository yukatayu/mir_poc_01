# Report 0882 — package129 authoritative-room runnable scenario hardening

- Date: 2026-04-20T10:17:43Z
- Author / agent: Codex
- Scope: Package 129 として `emit-scenario problem2` helper を追加し、authoritative-room first scenario を repo-local runnable scenario loop として harden する
- Decision levels touched: L2

## 1. Objective

- authoritative-room first scenario を docs / bundle / matrix だけで止めず、repo-local output dir に `run-source-sample` JSON を materialize する command まで actualize する。
- representative pair `p07 / p08`、reserve route `p09`、negative static-stop pair `p13 / p14` を 1 本の scenario loop として再実行できるようにする。
- final source wording、final public witness/provider/artifact contract、exhaustive shared-space catalog を still later に残したまま、Problem 2 current first line の runnable loop を narrow に固める。

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
- `specs/examples/570-current-l2-authoritative-room-first-scenario-helper-summary-tightening.md`
- `specs/examples/571-current-l2-authoritative-room-reserve-strengthening-lane-tightening.md`
- `specs/examples/576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `samples/problem-bundles/README.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`

## 3. Actions taken

- `scripts/tests/test_current_l2_guided_samples.py` に RED として次を追加した。
  - `emit-scenario problem2` が bundle command surface に見えること
  - scenario row が representative / reserve / negative pair を別 status で返すこと
  - emitter command が prototype sample path を使うこと
  - `main(["emit-scenario", "problem2"])` が runtime renderer を通ること
- `scripts/tests/test_problem_sample_bundles.py` に RED として、Problem 2 bundle doc / sample-bundle index が `emit-scenario problem2` と output dir を案内することを追加した。
- `scripts/current_l2_guided_samples.py` に次を追加した。
  - `ProblemScenarioEmitRow`
  - `emit-scenario problem2 [--format pretty|json] [--output-dir <path>]`
  - `target/current-l2-guided/problem2-scenario-bundle` default output dir
  - representative pair `p07 / p08`、reserve route `p09`、negative pair `p13 / p14` を materialize する helper
  - pretty/json summary renderer
- `samples/problem-bundles/problem2-order-handoff-shared-space.md` に runnable scenario loop の execution step を追加した。
- `samples/problem-bundles/README.md` に `emit-scenario problem2` の bundle-level entrypoint を追加した。
- `specs/examples/602-current-l2-authoritative-room-runnable-scenario-loop-hardening.md`
  を追加し、current cut / recommendation / stop line を文書化した。
- `Documentation.md`、`plan/01`、`plan/11`、`plan/17`、`plan/18`、`plan/90`、`progress.md`、`tasks.md`、`specs/00`、`specs/11`、`specs/12`
  を Package 129 closeout 読みに同期した。

## 4. Files changed

- `docs/reports/0882-package129-authoritative-room-runnable-scenario-hardening.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/602-current-l2-authoritative-room-runnable-scenario-loop-hardening.md`

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - RED:
    - `module 'current_l2_guided_samples' has no attribute 'ProblemScenarioEmitRow'`
    - `emit-scenario problem2` not found in bundle text
  - GREEN:
    - `Ran 67 tests`
    - `OK`
- `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - RED:
    - `emit-scenario problem2` not found in Problem 2 sample bundle doc
  - GREEN:
    - `Ran 3 tests`
    - `OK`
- `python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
  - GREEN:
    - `output dir: target/current-l2-guided/problem2-scenario-bundle`
    - `p07-dice-late-join-visible-history: first-line representative`
    - `p08-dice-stale-reconnect-refresh: first-line representative`
    - `p09-dice-delegated-rng-provider-placement: reserve practical route`
    - `p13-dice-late-join-missing-publication-witness: negative static-stop`
    - `p14-dice-late-join-handoff-before-publication: negative static-stop`
- `python3 scripts/current_l2_guided_samples.py emit-scenario problem2 --format json`
  - `problem_id: problem2`
  - `rows[0].first_line_status: reached`
  - `rows[2].reserve_lane_status: reached`
  - `rows[3].terminal_outcome: none`

## 6. Evidence / findings

- Problem 2 current first line はすでに `matrix problem2` / `bundle problem2` / `surface_preview` に揃っていたため、next useful ratchet は final wording comparison の追加ではなく runnable scenario loop の materialization だった。
- `emit-scenario problem2` により、representative pair `p07 / p08`、reserve route `p09`、negative pair `p13 / p14` を 1 コマンドで repo-local output dir に落とし直せるようになり、authoritative-room first scenario の current reading が「読むだけ」ではなくなった。
- `p09` を first line へ premature に昇格させず reserve practical route のまま残し、`p13 / p14` を negative static-stop として並べたことで、first completion line / reserve lane / failure-focused corpus の区別を scenario output でも保てた。

## 7. Changes in understanding

- authoritative-room runnable scenario hardening の本質は、新しい fairness / replay comparison を増やすことではなく、既存 first default を representative / reserve / negative pair まで含めた repo-local loop に下ろすことだった。
- Problem 2 current first line は `smoke problem2`、`matrix problem2`、`bundle problem2`、`emit-scenario problem2` が揃ったことで、order/handoff/shared-space 側の runnable evidence が一段厚くなった。

## 8. Open questions

- Problem 1 / Problem 2 executable loops を踏まえた上で、remaining mixed gate をどの command surface から再開するのが最も drift しにくいか。
- final source wording や final public witness/provider/artifact contract を reopen する前に、どこまで repo-local helper surface を増やすか。
- repo-local once-through near-end line を閉じた後、packaging / FFI / host integration reserve をどの package 列で受けるか。

## 9. Suggested next prompt

- `Package 129 は閉じたので、次は executable loop を踏まえた residual mixed-gate reopen package を進めてください。`
