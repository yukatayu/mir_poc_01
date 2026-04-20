# 0873 Package 120 Problem 2 source wording / emitted schema split helper

## 1. Title and identifier

- 0873 Package 120 Problem 2 source wording / emitted schema split helper

## 2. Objective

- Problem 2 source wording / emitted schema residual を witness-provider public-shape residual から切り離して読むための
  narrow helper/doc cut を actualize し、
  representative `p07 / p08` と supporting `p13 / p14` の役割差を
  split package 単位で読めるようにする。

## 3. Scope and assumptions

- current target は source wording / emitted schema split helper actualization に限定する。
- final source-surface handoff wording、
  final emitted-artifact schema、
  final public parser / checker / runtime API は今回の scope に入れない。
- witness-provider public-shape residual は kept-separate として保持する。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/569-current-l2-order-handoff-source-surface-artifact-route-tightening.md`
- `specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`

## 5. Actions taken

1. source wording / emitted schema split helper の current reading を確認した。
2. `samples/problem-bundles/problem2-order-handoff-shared-space.md` に
   `source wording / emitted schema split の入口` section を追加した。
3. split package status を更新し、
   source wording / emitted schema split を close 済みの helper/doc cut として読めるようにした。
4. guided sample tests と sample bundle doc tests に
   source wording split command / manifest / doc wording を追加した。
5. spec / snapshot / roadmap / traceability を Package 120 closeout reading に同期した。

## 6. Files changed

- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `specs/examples/593-current-l2-problem2-source-wording-emitted-schema-split-helper-actualization.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`

## 7. Commands run

```bash
python3 -m unittest scripts.tests.test_problem_sample_bundles
python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles
python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema
python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - Problem 2 guide に source wording split helper section がなく失敗した。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema`
  - representative/supporting sample set、kept-separate residual、stop line を 1 画面で読めるようになった。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema --format json`
  - same reading が machine-readable manifest として返った。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
  - source wording split helper tests と Problem 2 guide updated doc tests を含めて通過した。

## 9. What changed in understanding

- source wording / emitted schema residual は witness/provider public-shape residual と同じ段で並べると混線しやすいが、
  `split` helper と Problem 2 bundle doc の専用 section を持たせると reopen point が読みやすい。
- `p07 / p08` representative pair だけでなく `p13 / p14` negative pair を supporting set として明示したほうが、
  edge-row principal / stage-block secondary / emitted schema residual の current cut を説明しやすい。

## 10. Open questions

- Problem 2 の source wording split package closeout 自体はここで一巡したが、
  final source-surface handoff wording、final emitted-artifact schema、final public parser / checker / runtime API は still later である。
- Problem 2 witness-provider public-shape residual は still next package であり、
  今回は untouched に残した。

## 11. Suggested next prompt

- Problem 2 source wording split が揃ったので、
  Problem 2 witness-provider public-shape split を same style の narrow helper/doc package に落としてください。
