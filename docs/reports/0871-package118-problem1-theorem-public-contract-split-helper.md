# 0871 Package 118 Problem 1 theorem public-contract split helper

## 1. Title and identifier

- 0871 Package 118 Problem 1 theorem public-contract split helper

## 2. Objective

- Problem 1 theorem public-contract residual を typed residual / model-check residual から切り離して読むための
  narrow helper/doc cut を actualize し、
  representative `p06` と theorem-first pilot / Lean artifact 導線を
  split package 単位で読めるようにする。

## 3. Scope and assumptions

- current target は theorem public-contract split helper actualization に限定する。
- final public theorem contract、
  concrete theorem prover brand、
  final public verifier contract は今回の scope に入れない。
- typed residual と model-check residual は kept-separate として保持する。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md`
- `specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md`
- `specs/examples/590-current-l2-problem1-typed-source-principal-split-helper-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`

## 5. Actions taken

1. theorem public-contract split helper の current reading を確認した。
2. `samples/problem-bundles/problem1-typed-theorem-model-check.md` に
   `theorem public-contract split の入口` section を追加した。
3. split package status を更新し、
   theorem public-contract split を close 済みの helper/doc cut として読めるようにした。
4. guided sample tests と sample bundle doc tests に
   theorem split command / manifest / doc wording を追加した。
5. spec / snapshot / roadmap / traceability を Package 118 closeout reading に同期した。

## 6. Files changed

- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `specs/examples/591-current-l2-problem1-theorem-public-contract-split-helper-actualization.md`
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
python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract
python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - Problem 1 guide に theorem split helper section がなく失敗した。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract`
  - representative `p06`、bundle command、kept-separate residual、stop line を 1 画面で読めるようになった。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract --format json`
  - same reading が machine-readable manifest として返った。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
  - theorem split helper tests と Problem 1 guide updated doc tests を含めて通過した。

## 9. What changed in understanding

- theorem public-contract residual は typed residual や model-check residual と混ざりやすいが、
  `split` helper と Problem 1 bundle doc の専用 section を持たせると reopen point が読みやすい。
- theorem line は representative `p06` と theorem-first pilot / Lean artifact 導線だけでも
  current cut を十分に narrow に説明できる。

## 10. Open questions

- model-check public-contract split は still next package であり、
  今回は kept-separate として残した。
- final public theorem contract 自体は still later であり、
  split helper は reopen point の narrow 化に留めた。

## 11. Suggested next prompt

- Problem 1 theorem split helper が固まったので、
  model-check public-contract split を same style の narrow helper/doc package に落としてください。
