# 0865 Package 111 representative problem bundle quickstart walkthrough hardening

## 1. Title and identifier

- 0865 Package 111 representative problem bundle quickstart walkthrough hardening

## 2. Objective

- `samples/problem-bundles/problem1|problem2` を quickstart として単独でも読めるようにし、
  representative sample の最短導線と expected reading を短く固定する。

## 3. Scope and assumptions

- current target は sample-side quickstart bundle hardening に限定する。
- exhaustive tutorial surface、final public onboarding、final public parser / checker / runtime API は今回の scope に入れない。
- parser companion inspector は final grammar adoption ではなく、
  thin experimental companion surface の parser-side carrier 確認として扱う。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/581-current-l2-explained-representative-problem-sample-bundles.md`
- `specs/examples/582-current-l2-representative-problem-bundle-smoke-runner-actualization.md`
- `specs/examples/583-current-l2-representative-problem-bundle-aggregate-smoke-summary-actualization.md`
- `specs/examples/584-current-l2-representative-problem-bundle-failure-focused-smoke-diagnostics-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`

## 5. Actions taken

1. Problem 1 sample bundle に `最短 quickstart` を追加し、
   `smoke problem1` → `matrix problem1` → `bundle problem1` →
   parser companion inspector の 4 段導線と `見るべき結果` を追記した。
2. Problem 2 sample bundle に `最短 quickstart` を追加し、
   `smoke problem2` → `matrix problem2` → `bundle problem2` →
   parser companion inspector の 4 段導線と `見るべき結果` を追記した。
3. `samples/problem-bundles/README.md` に
   bundle doc 冒頭の quickstart を index 側から案内する説明を追加した。
4. sample bundle test を更新し、
   quickstart / expected-reading / parser companion inspector 導線が drift しないようにした。
5. spec / snapshot / traceability を Package 111 closeout reading に同期した。

## 6. Files changed

- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `scripts/tests/test_problem_sample_bundles.py`
- `specs/examples/585-current-l2-representative-problem-bundle-quickstart-walkthrough-hardening.md`
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
python3 -m unittest scripts.tests.test_problem_sample_bundles
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED 確認:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - sample bundle doc に `最短 quickstart`、`見るべき結果`、parser companion inspector 導線がなく失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - Problem 1 / Problem 2 bundle doc と sample bundle index が quickstart reading を満たした。
- GREEN:
  - `python3 scripts/validate_docs.py`
  - doc scaffold / numbered report inventory は整合した。

## 9. What changed in understanding

- `samples/problem-bundles/` は sample index として置くだけでなく、
  representative sample の最短導線を持たせたほうが user-facing drift を減らせる。
- `smoke-all` と problem ごとの quickstart を分けることで、
  aggregate check と per-problem reading を競合させずに済んだ。

## 10. Open questions

- quickstart bundle を CLI 側にも mirror するかは next package に残る。
- exhaustive tutorial surface、broader sample catalog widening、final public onboarding flow は still later。
- theorem/model-check / witness-provider public contract は mixed gate に残る。

## 11. Suggested next prompt

- quickstart bundle の 4 段導線を CLI 側にも mirror し、
  `problem1|problem2` ごとの quickstart summary を command でも出せる helper package を actualize してください。
