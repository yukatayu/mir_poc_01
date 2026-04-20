# 0876 Package 123 remaining residual lane summary

## 1. Title and identifier

- 0876 Package 123 remaining residual lane summary

## 2. Objective

- split-package closeout 後に残る mixed gate と true user-spec residual を
  `scripts/current_l2_guided_samples.py residuals`
  で圧縮し、next reopen order を helper / docs / snapshot から同じ読みで辿れるようにする。

## 3. Scope and assumptions

- current target は residual lane summary の helper-local actualization と snapshot 同期に限定する。
- final public theorem/model-check/witness-provider contract、
  final parser / checker / runtime API、
  exhaustive shared-space catalog、
  packaging / FFI / engine adapter は今回の scope に入れない。
- representative sample bundle / reopen-map / split helper は保持し、その上位に residual lane summary を足す。

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
- `specs/examples/588-current-l2-representative-problem-mixed-gate-reopen-map-refresh.md`
- `specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md`
- `specs/examples/595-current-l2-representative-problem-reopen-map-split-closeout-sync.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`

## 5. Actions taken

1. `reopen-map` closeout 後に remaining mixed gate と true user-spec residual がどこでまだ冗長かを再監査した。
2. `scripts/current_l2_guided_samples.py residuals` を追加し、
   Problem 1 final public seam、
   Problem 2 final public seam、
   syntax/modality final marker
   の 3 lane と true user-spec residual を 1 枚に圧縮した。
3. focused unit test を追加し、pretty/json 両 surface と CLI entrypoint を固定した。
4. sample bundle docs に `residuals` entrypoint を追記し、problem-local reopen と global residual lane summary を往復しやすくした。
5. snapshot / roadmap / traceability を Package 123 closeout reading に同期し、次 self-driven queue を Problem 1 / Problem 2 / syntax-modality の residual lane package へ再構成した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `specs/examples/596-current-l2-remaining-residual-lane-summary-actualization.md`
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
python3 scripts/current_l2_guided_samples.py residuals
python3 scripts/current_l2_guided_samples.py residuals --format json
python3 -m unittest scripts.tests.test_problem_sample_bundles
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `render_remaining_residual_lane_summary*` と `main(["residuals"])` が未実装で失敗した。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py residuals`
  - Problem 1 / Problem 2 / syntax-modality の remaining mixed-gate lane と true user-spec residual が 1 画面で読めるようになった。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py residuals --format json`
  - machine-readable に `mixed_gate_lanes` / `true_user_spec_residuals` / `recommended_order` を受け取れるようになった。
- RED:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - sample bundle docs が `residuals` entrypoint をまだ案内しておらず失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - residual helper を含めて通過した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - bundle index / problem bundle docs が `residuals` entrypoint を含む current reading に同期した。

## 9. What changed in understanding

- split-package closeout 後は `reopen-map` だけでは problem-local reopen point と global residual lane がまだ少し混ざる。
- `residuals` helper を別に置くと、
  Problem 1 final public seam、
  Problem 2 final public seam、
  syntax/modality final marker、
  true user-spec residual
  を queue drift なしで読み分けやすい。
- current self-driven queue は queue zero ではなく、
  residual lane summary closeout の後に
  Problem 1 / Problem 2 / syntax-modality の follow-up lane へ narrow に再構成する方が自然である。

## 10. Open questions

- final public theorem/model-check/witness-provider contract は still mixed gate である。
- final parser / checker / runtime API、packaging / FFI / engine adapter、exhaustive shared-space catalog は still later に残る。
- syntax/modality final marker line は current residual lane summary までであり、final calculus adoption ではない。

## 11. Suggested next prompt

- residual lane summary が入ったので、Problem 1 / Problem 2 / syntax-modality の next residual lane package を順に actualize してください。
