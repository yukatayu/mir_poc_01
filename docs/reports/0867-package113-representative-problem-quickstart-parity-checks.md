# 0867 Package 113 representative problem quickstart parity checks

## 1. Title and identifier

- 0867 Package 113 representative problem quickstart parity checks

## 2. Objective

- sample bundle doc と `quickstart problem1|problem2` helper の representative 4-step 導線が揃っているかを、
  focused parity check command と test で確認できるようにする。

## 3. Scope and assumptions

- current target は representative 4-step quickstart の parity check に限定する。
- exhaustive tutorial validation、final public CLI / tutorial surface、
  final public parser / checker / runtime API は今回の scope に入れない。
- current helper は repo-local drift suppression cut として読む。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/585-current-l2-representative-problem-bundle-quickstart-walkthrough-hardening.md`
- `specs/examples/586-current-l2-representative-problem-quickstart-cli-mirror-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/README.md`

## 5. Actions taken

1. `scripts/current_l2_guided_samples.py` に
   `quickstart-parity` subcommand と parity renderer を追加した。
2. representative 4-step quickstart の title / command が
   sample bundle doc に present かを problem ごとに集計する helper を actualize した。
3. shell block の multi-line 表記差を drift と誤認しないよう、
   command comparison を normalized shell text で行うようにした。
4. guided sample test に synced / mismatch / main command の RED/GREEN を追加した。
5. `samples/problem-bundles/README.md` に parity check command を追記し、
   spec / snapshot / traceability を Package 113 closeout reading に同期した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/README.md`
- `specs/examples/587-current-l2-representative-problem-quickstart-parity-checks-actualization.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`

## 7. Commands run

```bash
python3 -m unittest scripts.tests.test_current_l2_guided_samples
python3 -m unittest scripts.tests.test_problem_sample_bundles
python3 scripts/current_l2_guided_samples.py quickstart-parity
python3 scripts/current_l2_guided_samples.py quickstart-parity --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED 確認:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - quickstart parity helper 未実装で失敗した。
- RED 確認:
  - initial parity run
  - parser companion inspector command が multi-line shell block で書かれているため mismatch と誤認した。
- GREEN:
  - normalized shell text comparison を入れた後の `quickstart-parity`
  - Problem 1 / Problem 2 とも `synced` になった。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - synced / mismatch / main command の parity tests が通過した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - sample bundle index から parity check command を辿れるようになった。

## 9. What changed in understanding

- representative sample guide を docs と helper の両方で持つなら、
  narrow parity check を 1 本置いたほうが drift suppression がかなり楽になる。
- multi-line shell block はそのまま文字列一致させると誤検出が出るため、
  normalized shell text で見るのが current cut では十分だった。

## 10. Open questions

- parity check を CI hard gate にするかは still later。
- mixed-gate reopen-map refresh、broader sample catalog widening、final public onboarding flow は still later。
- theorem/model-check / witness-provider public contract は mixed gate に残る。

## 11. Suggested next prompt

- representative sample entrypoint が固まったので、
  Problem 1 / Problem 2 の mixed-gate reopen point を quickstart / bundle / matrix / smoke 現況に合わせて短く再整理してください。
