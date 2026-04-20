# 0863 Package 109 representative problem bundle aggregate smoke summary

## 1. Title and identifier

- 0863 Package 109 representative problem bundle aggregate smoke summary

## 2. Objective

- `smoke problem1` / `smoke problem2` をまとめて compact に俯瞰できる `smoke-all` helper を actualize し、representative sample bundle verification の入口を 1 本にする。

## 3. Scope and assumptions

- current target は representative 2 問題の aggregate smoke summary に限定する。
- exhaustive workflow automation、aggregate CI contract、final public CLI / tutorial surface、final public parser / checker / runtime API は今回の scope に入れない。
- per-problem `smoke` command は残し、aggregate 側は compact summary 読みだけに留める。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/582-current-l2-representative-problem-bundle-smoke-runner-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/README.md`

## 5. Actions taken

1. `scripts/current_l2_guided_samples.py` に `smoke-all` subcommand と aggregate smoke summary renderer を actualize した。
2. aggregate smoke summary の RED/GREEN test を追加し、Problem 1 / Problem 2 の compact row shape を固定した。
3. actual `smoke-all` / `smoke-all --format json` を実行し、両問題が passed summary を返すことを確認した。
4. `samples/problem-bundles/README.md` に `smoke-all` command を追記し、sample bundle index の shortest aggregate entrypoint を同期した。
5. closeout 用の spec / snapshot / traceability を Package 109 reading に同期した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/README.md`
- `specs/examples/583-current-l2-representative-problem-bundle-aggregate-smoke-summary-actualization.md`
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
python3 scripts/current_l2_guided_samples.py smoke-all
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED 確認:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - aggregate smoke summary helper 未実装で失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `build_problem_smoke_aggregate_rows` と `main(["smoke-all"])` が通過した。
- RED 確認:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - `samples/problem-bundles/README.md` に `smoke-all` command が未記載で失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - sample bundle index から `smoke-all` を辿れるようになった。
- manual run:
  - `python3 scripts/current_l2_guided_samples.py smoke-all`
  - `problem1: passed (5/5 steps)`、`problem2: passed (7/7 steps)` の compact summary を返した。
- manual run:
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - problem id、status、step_count、successful_steps、failed_step、smoke command、sample bundle doc、step labels を JSON で返した。

## 9. What changed in understanding

- representative 2 問題の verification entrypoint は、per-problem smoke を保持したまま `smoke-all` の compact summary を足すのが最も自然だった。
- `samples/problem-bundles/README.md` に aggregate command を置くことで、helper command と sample-side guide の導線がずれにくくなった。

## 10. Open questions

- aggregate failure 時に failed step と captured output をどこまで compact に surfacing するかは next package に残る。
- exhaustive workflow automation、aggregate CI contract、final public CLI / tutorial surface は still later。
- theorem/model-check / witness-provider public contract は mixed gate に残る。

## 11. Suggested next prompt

- `smoke-all` failure 時に failed step と captured output を compact に追える aggregate diagnostics hardening を actualize し、repo-local verification loop をもう一段詰めてください。
