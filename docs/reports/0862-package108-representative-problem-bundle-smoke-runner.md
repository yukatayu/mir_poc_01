# 0862 Package 108 representative problem bundle smoke runner

## 1. Title and identifier

- 0862 Package 108 representative problem bundle smoke runner

## 2. Objective

- Problem 1 / Problem 2 の representative sample bundle guide に書いた主要 command 群を repo-local helper で smoke 実行し、guide / helper / runnable evidence の 3 点が drift していないことを 1 コマンドずつで確認できるようにする。

## 3. Scope and assumptions

- target は representative line に限定する。
- exhaustive workflow automation、final public CLI / tutorial surface、final public parser / checker / runtime API は今回の scope に入れない。
- parser-side companion は `p06 / p07 / p08` representative slice に限定し、full `Program` lowering には広げない。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/580-current-l2-parser-side-representative-mapping-matrix-actualization.md`
- `specs/examples/581-current-l2-explained-representative-problem-sample-bundles.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`

## 5. Actions taken

1. `scripts/current_l2_guided_samples.py` に `smoke` subcommand と step builder / runner を actualize した。
2. Problem 1 / Problem 2 の representative smoke sequence を RED/GREEN で固定した。
3. actual smoke command を両問題で実行し、runtime / matrix / bundle / parser-side inspector / mapping が順に通ることを確認した。
4. closeout 用の spec / snapshot / traceability を Package 108 reading に同期した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `specs/examples/582-current-l2-representative-problem-bundle-smoke-runner-actualization.md`
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
python3 scripts/current_l2_guided_samples.py smoke problem1
python3 scripts/current_l2_guided_samples.py smoke problem2
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED 確認:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `build_problem_smoke_steps` / `run_problem_smoke` 未実装で失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - Problem 1 / Problem 2 の smoke step sequence と `main(["smoke", ...])` が通過した。
- manual run:
  - `python3 scripts/current_l2_guided_samples.py smoke problem1`
  - `runtime:p06 -> matrix:problem1 -> bundle:problem1 -> inspector:p06 -> mapping`
    が exit code 0 で完走した。
- manual run:
  - `python3 scripts/current_l2_guided_samples.py smoke problem2`
  - `runtime:p07 -> runtime:p08 -> matrix:problem2 -> bundle:problem2 -> inspector:p07 -> inspector:p08 -> mapping`
    が exit code 0 で完走した。

## 9. What changed in understanding

- representative bundle guide は README だけだと drift しやすいが、smoke command を 1 本ずつ持たせると current runnable floor をかなり読みやすく保てる。
- Problem 1 / Problem 2 とも、runtime / bundle / parser-side inspector / mapping を同じ helper に畳むことで「どこまで actualized 済みか」を機械的に確認しやすくなった。

## 10. Open questions

- Problem 1 / Problem 2 をまとめた compact smoke summary を追加するかは next package に残る。
- final public CLI / tutorial surface、aggregate CI contract、final public parser / checker / runtime API は still later。
- theorem/model-check / witness-provider public contract は mixed gate に残る。

## 11. Suggested next prompt

- representative smoke runner の上に、Problem 1 / Problem 2 をまとめて俯瞰できる compact smoke summary command を actualize し、repo-local verification の入口をさらに 1 本に絞ってください。
