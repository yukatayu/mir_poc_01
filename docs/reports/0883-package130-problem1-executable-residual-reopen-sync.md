# Report 0883 — package130 Problem 1 executable residual reopen sync

- Date: 2026-04-20T10:27:53Z
- Author / agent: Codex
- Scope: Package 130 として Problem 1 residual mixed gate の reopen order を executable loop 側に再同期する
- Decision levels touched: L2

## 1. Objective

- Problem 1 の remaining mixed gate を docs-only lane としてではなく、すでに揃っている executable evidence
  `check-source-sample` と `emit-theorem problem1`
  を入口にして読み直せるようにする。
- `reopen-map problem1`、`lane problem1-final-public-seams`、Problem 1 sample bundle doc を揃え、
  typed / theorem / model-check residual の reopen order を stale wording なしで保つ。
- final typed source principal、final public theorem contract、final public checker artifact、final public verifier contract は still later に残す。

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
- `specs/examples/597-current-l2-problem1-final-public-seam-lane-helper-actualization.md`
- `specs/examples/600-current-l2-typed-checker-first-executable-slice-actualization.md`
- `specs/examples/601-current-l2-theorem-first-emitted-artifact-loop-hardening.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`

## 3. Actions taken

- `scripts/current_l2_guided_samples.py` の Problem 1 reopen map を更新し、
  `quickstart / smoke / matrix / bundle / parser companion inspector`
  に加えて
  `check-source-sample`
  と
  `emit-theorem problem1`
  を drift suppression 済みの executable floor として明示した。
- Problem 1 reopen map の entry commands を
  - `quickstart problem1`
  - `check-source-sample`
  - `emit-theorem problem1`
  - `lane problem1-final-public-seams`
  の順へ更新した。
- Problem 1 final-public-seam lane の entry commands を
  - `check-source-sample`
  - `emit-theorem problem1`
  - `reopen-map problem1`
  へ更新した。
- reopen guidance を typed / theorem / model-check residual ごとに executable evidence 起点へ寄せ直した。
- `samples/problem-bundles/problem1-typed-theorem-model-check.md` に、
  `reopen-map problem1`
  から
  `check-source-sample` → `emit-theorem problem1` → `lane problem1-final-public-seams`
  の順で読む current reopen order を追記した。
- `specs/examples/603-current-l2-problem1-executable-residual-reopen-sync.md`
  を追加し、current cut / recommendation / stop line を文書化した。
- `Documentation.md`、`plan/01`、`plan/11`、`plan/17`、`plan/18`、`plan/90`、`progress.md`、`tasks.md`、`specs/00`、`specs/11`、`specs/12`
  を Package 130 closeout 読みに同期した。

## 4. Files changed

- `docs/reports/0883-package130-problem1-executable-residual-reopen-sync.md`
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
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/603-current-l2-problem1-executable-residual-reopen-sync.md`

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
  - RED:
    - `AssertionError: 'emit-theorem problem1' not found in [...]`
  - GREEN:
    - `Ran 69 tests`
    - `OK`
- `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
  - GREEN:
    - `entry commands:`
    - `check-source-sample ... p10-typed-authorized-fingerprint-declassification.txt`
    - `emit-theorem problem1`
    - `lane problem1-final-public-seams`
- `python3 scripts/current_l2_guided_samples.py reopen-map problem1 --format json`
  - `problem_rows[0].entry_commands[1]` contains `check-source-sample`
  - `problem_rows[0].entry_commands[2]` contains `emit-theorem problem1`
- `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams --format json`
  - `entry_commands[0]` contains `check-source-sample`
  - `entry_commands[1]` contains `emit-theorem problem1`
  - `entry_commands[2] = python3 scripts/current_l2_guided_samples.py reopen-map problem1`

## 6. Evidence / findings

- Problem 1 residual mixed gate の next useful ratchet は、新しい theorem/model-check comparison を増やすことではなく、
  既存の executable loop を reopen lane の入口へ接続することだった。
- `reopen-map problem1` と `lane problem1-final-public-seams` に executable commands が入ったことで、
  typed / theorem / model-check residual の順序が
  `check-source-sample` → `emit-theorem problem1` → lane summary
  として素直に読めるようになった。
- これにより、Problem 1 current first line は
  representative bundle / emitted-artifact loop / reopen lane
  が別々に漂う状態から、
  repo-local executable reopen order を持つ状態へ一段締まった。

## 7. Changes in understanding

- Problem 1 final-public-seam residual は compare-floor 自体を増やすより、
  executable evidence から reopen lane へ入れるようにした方が drift を抑えやすい。
- `reopen-map problem1` は abstract map のままより、
  checker executable slice と theorem emitted-artifact loop を前段に出した方が current repo の到達点を誤読しにくい。

## 8. Open questions

- Problem 2 側でも `emit-scenario problem2` を final-public-seam lane / residual summary にどう接続するか。
- executable reopen order を踏まえた after-loop closeout snapshot をどこまで 1 package に圧縮するか。
- final public seam 群を reopen する前に、repo-local helper surface をどこで止めるか。

## 9. Suggested next prompt

- `Package 130 は閉じたので、次は Problem 2 executable residual reopen sync を進めてください。`
