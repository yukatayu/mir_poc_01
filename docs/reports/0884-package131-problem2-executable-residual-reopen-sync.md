# Report 0884 — package131 Problem 2 executable residual reopen sync

- Date: 2026-04-20T10:34:14Z
- Author / agent: Codex
- Scope: Package 131 として Problem 2 residual mixed gate の reopen order を executable scenario loop 側に再同期する
- Decision levels touched: L2

## 1. Objective

- Problem 2 の remaining mixed gate を docs-only lane としてではなく、すでに揃っている executable evidence
  `emit-scenario problem2`
  を入口にして読み直せるようにする。
- `reopen-map problem2`、`lane problem2-final-public-seams`、`residuals`、Problem 2 sample bundle doc を揃え、
  source wording / emitted schema residual と witness-provider public-shape residual の reopen order を stale wording なしで保つ。
- final source-surface handoff wording、final public witness/provider/artifact contract、exhaustive shared-space catalog は still later に残す。

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
- `specs/examples/598-current-l2-problem2-final-public-seam-lane-helper-actualization.md`
- `specs/examples/602-current-l2-authoritative-room-runnable-scenario-loop-hardening.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`

## 3. Actions taken

- `scripts/current_l2_guided_samples.py` の Problem 2 reopen map を更新し、
  `quickstart / smoke / matrix / bundle / parser companion inspector`
  に加えて
  `emit-scenario problem2`
  を drift suppression 済みの executable floor として明示した。
- Problem 2 reopen map の entry commands を
  - `quickstart problem2`
  - `emit-scenario problem2`
  - `lane problem2-final-public-seams`
  - `residuals`
  の順へ更新した。
- Problem 2 final-public-seam lane の entry commands を
  - `emit-scenario problem2`
  - `reopen-map problem2`
  - `residuals`
  へ更新した。
- reopen guidance を source wording / emitted schema と witness-provider public-shape residual ごとに executable scenario evidence 起点へ寄せ直した。
- `samples/problem-bundles/problem2-order-handoff-shared-space.md` に、
  `reopen-map problem2`
  から
  `emit-scenario problem2` → `lane problem2-final-public-seams` → `residuals`
  の順で読む current reopen order を追記した。
- `specs/examples/604-current-l2-problem2-executable-residual-reopen-sync.md`
  を追加し、current cut / recommendation / stop line を文書化した。
- `Documentation.md`、`plan/01`、`plan/11`、`plan/17`、`plan/18`、`plan/90`、`progress.md`、`tasks.md`、`specs/00`、`specs/11`、`specs/12`
  を Package 131 closeout 読みに同期した。

## 4. Files changed

- `docs/reports/0884-package131-problem2-executable-residual-reopen-sync.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/604-current-l2-problem2-executable-residual-reopen-sync.md`

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
  - GREEN:
    - `Ran 69 tests`
    - `OK`
- `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
  - GREEN:
    - `entry commands:`
    - `emit-scenario problem2`
    - `lane problem2-final-public-seams`
    - `residuals`
- `python3 scripts/current_l2_guided_samples.py reopen-map problem2 --format json`
  - `problem_rows[0].entry_commands[1] = python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
  - `problem_rows[0].entry_commands[3] = python3 scripts/current_l2_guided_samples.py residuals`
- `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams --format json`
  - `entry_commands[0] = python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
  - `entry_commands[1] = python3 scripts/current_l2_guided_samples.py reopen-map problem2`
  - `entry_commands[2] = python3 scripts/current_l2_guided_samples.py residuals`

## 6. Evidence / findings

- Problem 2 residual mixed gate の next useful ratchet は、新しい shared-space catalog comparison を増やすことではなく、
  executable scenario loop を reopen lane の入口へ接続することだった。
- `reopen-map problem2` と `lane problem2-final-public-seams` に executable commands が入ったことで、
  source wording / emitted schema residual と witness-provider public-shape residual の順序が
  `emit-scenario problem2` → lane summary → residual summary
  として素直に読めるようになった。
- これにより、Problem 2 current first line は
  scenario loop / final-public-seam lane / residual summary
  が別々に漂う状態から、
  repo-local executable reopen order を持つ状態へ一段締まった。

## 7. Changes in understanding

- Problem 2 final-public-seam residual も Problem 1 と同じく、compare-floor 自体を増やすより、
  executable evidence から reopen lane へ入れるようにした方が drift を抑えやすい。
- `reopen-map problem2` は abstract map のままより、
  scenario loop を前段に出した方が current repo の到達点と stop line を誤読しにくい。

## 8. Open questions

- executable loop 群を踏まえた after-loop closeout snapshot をどこまで 1 package に圧縮するか。
- remaining mixed gate を near-end completion reading とどう切り分けて書くか。
- final public seam 群を reopen する前に、repo-local helper surface をどこで止めるか。

## 9. Suggested next prompt

- `Package 131 は閉じたので、次は after-loop closeout sync を進めてください。`
