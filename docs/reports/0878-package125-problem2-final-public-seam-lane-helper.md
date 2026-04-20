# 0878 Package 125 Problem 2 final-public-seam lane helper

## 1. Title and identifier

- 0878 Package 125 Problem 2 final-public-seam lane helper

## 2. Objective

- `residuals` summary と Package 124 Problem 1 lane helper の次段として、
  `scripts/current_l2_guided_samples.py lane problem2-final-public-seams`
  を current package として close し、Problem 2 の final wording / witness-provider reopen order を独立 lane として読めるようにする。

## 3. Scope and assumptions

- current target は Problem 2 final-public-seam lane helper の doc / snapshot integration に限定する。
- helper 本体は Package 124 で generic `lane` cut を得ているため、Package 125 では Problem 2 entrypoint と queue closeout を主眼にする。
- final source-surface handoff wording、
  final public witness/provider/artifact contract、
  exhaustive shared-space catalog、
  final public parser / checker / runtime API は今回の scope に入れない。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/593-current-l2-problem2-source-wording-emitted-schema-split-helper-actualization.md`
- `specs/examples/594-current-l2-problem2-witness-provider-public-shape-split-helper-actualization.md`
- `specs/examples/596-current-l2-remaining-residual-lane-summary-actualization.md`
- `specs/examples/597-current-l2-problem1-final-public-seam-lane-helper-actualization.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`

## 5. Actions taken

1. Problem 2 final-public-seam lane の期待値をテストへ追加し、bundle doc に lane entrypoint が無い RED を確認した。
2. Problem 2 bundle doc に `lane problem2-final-public-seams` entrypoint を追記した。
3. snapshot / roadmap / traceability を Package 125 closeout reading に同期し、current live queue を Package 126 と later mixed/user-spec residual へ narrow に戻した。

## 6. Files changed

- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `specs/examples/598-current-l2-problem2-final-public-seam-lane-helper-actualization.md`
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
python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles
python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams
python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
  - Problem 2 bundle doc が `lane problem2-final-public-seams` entrypoint をまだ案内しておらず失敗した。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams`
  - Problem 2 final-public-seam lane の focus / entry commands / component order / stop line を 1 画面で読める。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams --format json`
  - machine-readable に `component_order` / `stop_line` / `anchor_refs` を受け取れる。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples scripts.tests.test_problem_sample_bundles`
  - Problem 2 lane helper の pretty / JSON / doc entrypoint を含めて通過した。

## 9. What changed in understanding

- Problem 2 でも `residuals` summary だけでは source wording / witness-provider の reopen order が 1 段粗い。
- `lane problem2-final-public-seams` を bundle doc 側から直接辿れるようにすると、Problem 2 mixed gate を syntax-modality lane や true user-spec residual と混ぜずに保ちやすい。

## 10. Open questions

- syntax-modality final-marker lane helper は still next package に残る。
- final source-surface handoff wording は still mixed gate である。
- final public witness/provider/artifact contract と exhaustive shared-space catalog は still mixed/user-spec residual である。

## 11. Suggested next prompt

- Problem 2 lane helper が入ったので、次は syntax-modality final-marker lane helper を actualize してください。
