# 0869 Package 115/116 representative problem split-package map refresh

## 1. Title and identifier

- 0869 Package 115/116 representative problem split-package map refresh

## 2. Objective

- `reopen-map` aggregate helper を踏まえて、
  Problem 1 / Problem 2 の remaining mixed gate を
  next split package 単位へ narrow に戻し、
  helper / sample bundle / snapshot docs の 3 点で同じ読みへ揃える。

## 3. Scope and assumptions

- current target は split-package map refresh に限定する。
- final public theorem/model-check/witness-provider contract、
  final public parser / checker / runtime API、
  exhaustive shared-space catalog は今回の scope に入れない。
- current helper は repo-local reopen-order hardening cut として読む。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/588-current-l2-representative-problem-mixed-gate-reopen-map-refresh.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`

## 5. Actions taken

1. `scripts/current_l2_guided_samples.py reopen-map` に optional `problem_id` filter を追加した。
2. Problem 1 row に
   `typed source principal split` /
   `theorem public-contract split` /
   `model-check public-contract split`
   を actualize した。
3. Problem 2 row に
   `source wording / emitted schema split` /
   `witness-provider public-shape split`
   を actualize した。
4. Problem 1 / Problem 2 の sample bundle doc に
   `次の split package` section を追加した。
5. guided sample tests と sample bundle doc tests に
   problem-local reopen-map filter / split-package wording を追加した。
6. spec / snapshot / roadmap / traceability を Package 115/116 closeout reading に同期した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md`
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
python3 scripts/current_l2_guided_samples.py reopen-map problem1
python3 scripts/current_l2_guided_samples.py reopen-map problem1 --format json
python3 scripts/current_l2_guided_samples.py reopen-map problem2
python3 scripts/current_l2_guided_samples.py reopen-map problem2 --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `reopen-map problem1` filter と `split_packages` が未実装で失敗した。
- RED:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - Problem 1 guide に `次の split package` section がなく失敗した。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
  - Problem 1 remaining mixed gate と 3 本の split package が pretty summary で返った。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
  - Problem 2 remaining mixed gate と 2 本の split package が pretty summary で返った。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - problem-local reopen-map filter / split-package tests が通過した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - Problem 1 / Problem 2 guide から split package を辿れるようになった。

## 9. What changed in understanding

- aggregate reopen map を置いた後でも、
  next package 名まで narrow に落としておかないと
  queue drift は再発しやすい。
- Problem 1 と Problem 2 は mixed gate の shape が違うので、
  split package の粒度も分けたほうが自然だった。

## 10. Open questions

- Problem 1 typed / theorem / model-check split のうち、
  どこを次に actual helper / docs package へ落とすかは still open だが、
  現在は separate package 名まで narrowed した。
- Problem 2 source wording / emitted schema split と
  witness-provider public-shape split のどちらを先に実装強化するかは still open だが、
  両方とも package 名まで narrowed した。
- final modal foundation / source marker は still later。

## 11. Suggested next prompt

- split-package map が固まったので、
  Problem 1 typed source principal split と
  Problem 2 source wording / emitted schema split の
 どちらかを actual helper / docs package に落としてください。
