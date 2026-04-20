# 0861 Package 107 explained representative problem sample bundles

## 1. Title and identifier

- 0861 Package 107 explained representative problem sample bundles

## 2. Objective

- 二大問題の representative sample を `samples/` 側の簡潔な日本語 guide に actualize し、runner / Lean artifact / parser companion / guided helper を 1 本の sample bundle 導線へ揃える。

## 3. Scope and assumptions

- current target は representative line と supporting / reserve / negative sample の最小 bundle に限定する。
- final public tutorial surface、exhaustive sample catalog、final public parser / checker / runtime API は今回の scope に入れない。

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
- `specs/examples/576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md`
- `specs/examples/580-current-l2-parser-side-representative-mapping-matrix-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `tasks.md`
- `samples/current-l2/README.md`
- `samples/prototype/README.md`

## 5. Actions taken

1. `samples/problem-bundles/README.md` と Problem 1 / Problem 2 の per-problem guide を新設した。
2. `scripts/current_l2_guided_samples.py bundle problem1|problem2` に `sample_bundle_doc` を actualize し、helper 側から `samples/` guide を辿れるようにした。
3. `samples/current-l2/README.md` と `samples/prototype/README.md` に explained bundle への導線を追加した。
4. closeout 用の spec / snapshot / traceability を Package 107 reading に同期した。

## 6. Files changed

- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `samples/current-l2/README.md`
- `samples/prototype/README.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `specs/examples/581-current-l2-explained-representative-problem-sample-bundles.md`
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
python3 -m unittest scripts.tests.test_current_l2_guided_samples
python3 scripts/current_l2_guided_samples.py bundle problem1
python3 scripts/current_l2_guided_samples.py bundle problem2
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED 確認:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - bundle guide files未作成で失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - Problem 1 / Problem 2 guide の file existence と key refs が通過した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `bundle problem1|problem2` に `sample_bundle_doc` が出ることを含めて通過した。
- manual check:
  - `python3 scripts/current_l2_guided_samples.py bundle problem1`
  - `python3 scripts/current_l2_guided_samples.py bundle problem2`
  - helper output から `samples/problem-bundles/*.md` へ辿れることを確認した。

## 9. What changed in understanding

- user-facing sample guide を `docs/` 側へ増やすより、`samples/` 側に置いて helper から辿れるようにした方が drift が少ない。
- representative sample bundle は new runnable artifact を増やすより、existing corrected prototype / Lean / parser-side evidence を同じ日本語 guide にまとめる cut が適切だった。

## 10. Open questions

- bundle command 群を end-to-end smoke 実行まで自動化するかは次 package に残る。
- final public tutorial surface と exhaustive sample catalog は still later。
- theorem/model-check / witness-provider final public contract は mixed gate に残る。

## 11. Suggested next prompt

- Problem 1 / Problem 2 representative bundle の smoke runner を actualize し、guide に書いた主要 command 群を repo-local helper でまとめて検証してください。
