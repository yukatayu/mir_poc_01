# 0868 Package 114 representative problem mixed-gate reopen map refresh

## 1. Title and identifier

- 0868 Package 114 representative problem mixed-gate reopen map refresh

## 2. Objective

- representative sample の quickstart / matrix / bundle / smoke floor を踏まえて、
  Problem 1 / Problem 2 の remaining mixed gate と true user-spec residual を
  helper / sample bundle / snapshot docs の 3 点で同じ読みへ揃える。

## 3. Scope and assumptions

- current target は reopen point の refresh に限定する。
- final public theorem/model-check/witness-provider contract、
  final public parser / checker / runtime API、
  exhaustive shared-space catalog は今回の scope に入れない。
- current helper は repo-local queue drift suppression cut として読む。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/573-current-l2-problem1-public-seam-residual-bundle-matrix.md`
- `specs/examples/574-current-l2-problem2-public-shape-residual-bundle-matrix.md`
- `specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md`
- `specs/examples/576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md`
- `specs/examples/587-current-l2-representative-problem-quickstart-parity-checks-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`
- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`

## 5. Actions taken

1. `scripts/current_l2_guided_samples.py` に `reopen-map` subcommand を追加し、
   Problem 1 / Problem 2 の current floor、remaining mixed gate、
   global true user-spec residual を 1 コマンドで見られるようにした。
2. `samples/problem-bundles/README.md` に `reopen-map` command を追加し、
   sample bundle index から reopen point を辿れるようにした。
3. Problem 1 / Problem 2 の bundle doc に
   `現在の mixed gate 再開点` section を追加し、
   problem-local mixed gate と global true user-spec residual を切り分けて書いた。
4. guided sample tests と sample bundle doc tests に
   reopen-map text / json / main command / doc wording を追加した。
5. spec / snapshot / roadmap / traceability を Package 114 closeout reading に同期した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `scripts/tests/test_problem_sample_bundles.py`
- `samples/problem-bundles/README.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `specs/examples/588-current-l2-representative-problem-mixed-gate-reopen-map-refresh.md`
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
python3 scripts/current_l2_guided_samples.py reopen-map
python3 scripts/current_l2_guided_samples.py reopen-map --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `render_problem_reopen_map*` と `reopen-map` main command が未実装で失敗した。
- RED:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - bundle docs に `現在の mixed gate 再開点` と `reopen-map` command がなく失敗した。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py reopen-map`
  - Problem 1 / Problem 2 の current floor、remaining mixed gate、global true user-spec residual が pretty summary で返った。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py reopen-map --format json`
  - `problem_rows` と `true_user_spec_residuals` を JSON で返した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - reopen-map text / json / main command tests が通過した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - sample bundle index と Problem 1 / Problem 2 guide から reopen point を辿れるようになった。

## 9. What changed in understanding

- representative sample floor が固まった後の queue drift は、
  「何が mixed gate で、どの command から reopen するか」が
  helper / sample bundle / snapshot docs でずれると再発しやすい。
- problem-local mixed gate と global true user-spec residual を分けて書くと、
  自走可能部分と user decision が必要な部分の境界がかなり読みやすくなる。

## 10. Open questions

- Problem 1 側では theorem public contract と model-check public contract の split を、
  さらに narrow package に落とす余地がある。
- Problem 2 側では source wording / emitted schema と witness/provider public shape の split を、
  さらに narrow package に落とす余地がある。
- final modal foundation / source marker は still later。

## 11. Suggested next prompt

- representative reopen map が固まったので、
  Problem 1 theorem/model-check mixed gate と
  Problem 2 order-handoff/public-shape mixed gate を
  separate package に split してください。
