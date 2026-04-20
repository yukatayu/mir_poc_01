# 0866 Package 112 representative problem quickstart CLI mirror

## 1. Title and identifier

- 0866 Package 112 representative problem quickstart CLI mirror

## 2. Objective

- bundle doc 側の representative 4-step quickstart を
  `scripts/current_l2_guided_samples.py quickstart problem1|problem2`
  からも読めるようにし、sample-side quickstart と helper-side summary を揃える。

## 3. Scope and assumptions

- current target は quickstart CLI mirror に限定する。
- `smoke-all` は cross-problem aggregate entrypoint のまま残す。
- exhaustive tutorial surface、final public CLI / tutorial surface、
  final public parser / checker / runtime API は今回の scope に入れない。

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
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/README.md`

## 5. Actions taken

1. `scripts/current_l2_guided_samples.py` に
   `quickstart problem1|problem2` subcommand と quickstart renderer を追加した。
2. pretty / json の両方で、
   step title / command / expected_results を返す helper-side summary を actualize した。
3. guided sample test に quickstart renderer / subcommand の RED/GREEN を追加した。
4. `samples/problem-bundles/README.md` に quickstart CLI mirror command を追記した。
5. spec / snapshot / traceability を Package 112 closeout reading に同期した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/README.md`
- `specs/examples/586-current-l2-representative-problem-quickstart-cli-mirror-actualization.md`
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
python3 scripts/current_l2_guided_samples.py quickstart problem1
python3 scripts/current_l2_guided_samples.py quickstart problem2 --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED 確認:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - quickstart renderer / subcommand 未実装で失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - Problem 1 pretty quickstart、Problem 2 JSON quickstart、`main(["quickstart", ...])` が通過した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - sample bundle index から quickstart CLI mirror command を辿れるようになった。
- manual run:
  - `python3 scripts/current_l2_guided_samples.py quickstart problem1`
  - representative 4 ステップと `見るべき結果` を pretty summary で返した。
- manual run:
  - `python3 scripts/current_l2_guided_samples.py quickstart problem2 --format json`
  - step title / command / expected_results を JSON で返した。

## 9. What changed in understanding

- sample bundle doc 側に quickstart を置くだけでなく、
  helper-side summary にも mirror したほうが current representative line を確認しやすい。
- `smoke-all` と `quickstart problem1|problem2` を分けることで、
  aggregate regression と per-problem entrypoint を整理しやすくなった。

## 10. Open questions

- quickstart CLI mirror と sample bundle doc の parity check をどこまで automated にするかは next package に残る。
- exhaustive tutorial surface、final public CLI / tutorial surface、final public parser / checker / runtime API は still later。
- theorem/model-check / witness-provider public contract は mixed gate に残る。

## 11. Suggested next prompt

- quickstart CLI mirror と sample bundle doc の parity check を actualize し、
  command 側と sample 側の 4-step entrypoint が drift しないようにしてください。
