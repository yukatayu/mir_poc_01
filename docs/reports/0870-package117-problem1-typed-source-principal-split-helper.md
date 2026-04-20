# 0870 Package 117 Problem 1 typed source principal split helper

## 1. Title and identifier

- 0870 Package 117 Problem 1 typed source principal split helper

## 2. Objective

- Problem 1 typed residual を theorem/model-check public-contract residual から切り離して読むための
  narrow helper/doc cut を actualize し、
  representative `p06` と supporting `p10 / p11 / p12 / p15 / p16` の役割差を
  split package 単位で読めるようにする。

## 3. Scope and assumptions

- current target は typed source principal split helper actualization に限定する。
- stronger typed source principal 自体、
  final typed calculus、
  final public verifier contract は今回の scope に入れない。
- theorem/model-check residual は kept-separate として保持する。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/566-current-l2-first-strong-typing-finite-index-layer-actualization.md`
- `specs/examples/567-current-l2-lean-first-formal-skeleton-hardening-after-finite-index-widening.md`
- `specs/examples/573-current-l2-problem1-public-seam-residual-bundle-matrix.md`
- `specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`

## 5. Actions taken

1. `scripts/current_l2_guided_samples.py split` command を追加した。
2. Problem 1 `typed-source-principal` manifest を追加し、
   representative / supporting sample、kept-separate residual、stop line を actualize した。
3. `samples/problem-bundles/problem1-typed-theorem-model-check.md` に
   `typed source principal split の入口` section を追加した。
4. guided sample tests と sample bundle doc tests に
   split helper command / manifest / doc wording を追加した。
5. spec / snapshot / roadmap / traceability を Package 117 closeout reading に同期した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `specs/examples/590-current-l2-problem1-typed-source-principal-split-helper-actualization.md`
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
python3 -m unittest scripts.tests.test_current_l2_guided_samples
python3 -m unittest scripts.tests.test_problem_sample_bundles
python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal
python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - split helper function / command が未実装で失敗した。
- RED:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - Problem 1 guide に typed split helper section がなく失敗した。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal`
  - representative / supporting sample、kept-separate residual、stop line を 1 画面で読めるようになった。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal --format json`
  - same reading が machine-readable manifest として返った。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - split helper tests を含めて通過した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - Problem 1 guide から typed split helper を辿れるようになった。

## 9. What changed in understanding

- Problem 1 residual を 3 本へ split した後は、
  typed residual だけを独立 helper にしておくと
  theorem/model-check residual との混線を防ぎやすい。
- typed residual は representative `p06` だけでは不足で、
  `p10 / p11 / p12 / p15 / p16` を supporting set として明示したほうが読みやすかった。

## 10. Open questions

- theorem public-contract split と model-check public-contract split は still next package であり、
  今回は kept-separate として残した。
- stronger typed source principal actual adoption 自体は still later であり、
  split helper は reopen point の narrow 化に留めた。

## 11. Suggested next prompt

- Problem 1 typed split helper が固まったので、
  theorem public-contract split か model-check public-contract split を
  same style の narrow helper/doc package に落としてください。
