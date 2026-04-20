# 0875 Package 122 representative problem reopen-map split closeout sync

## 1. Title and identifier

- 0875 Package 122 representative problem reopen-map split closeout sync

## 2. Objective

- representative problem reopen-map から stale な `next split packages` 表示を外し、
  split-package closeout 後の residual public-seam maintenance 読みへ同期する。

## 3. Scope and assumptions

- current target は `reopen-map` helper public surface の split closeout sync に限定する。
- split helper command 自体は separate helper として保持する。
- final public theorem/model-check/witness-provider contract、
  final public parser / checker / runtime API、
  exhaustive shared-space catalog は今回の scope に入れない。

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
- `specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md`
- `specs/examples/593-current-l2-problem2-source-wording-emitted-schema-split-helper-actualization.md`
- `specs/examples/594-current-l2-problem2-witness-provider-public-shape-split-helper-actualization.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`

## 5. Actions taken

1. `reopen-map problem1|problem2` の current reading を確認した。
2. public reopen-map manifest から stale な `split_packages` next-queue surface を外した。
3. `closed_split_packages` を manifest / pretty summary に actualize し、
   split-package closeout を helper public surface から読めるようにした。
4. focused helper tests を split closeout reading へ更新した。
5. snapshot / roadmap / traceability を Package 122 closeout reading に同期した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `specs/examples/595-current-l2-representative-problem-reopen-map-split-closeout-sync.md`
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
python3 scripts/current_l2_guided_samples.py reopen-map problem1
python3 scripts/current_l2_guided_samples.py reopen-map problem2
python3 scripts/current_l2_guided_samples.py reopen-map problem1 --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - reopen-map tests が `closed_split_packages` / `split package closeout` を見つけられず失敗した。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
  - Problem 1 reopen-map から split package closeout と remaining mixed gate を 1 画面で読めるようになった。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem2`
  - Problem 2 reopen-map から split package closeout と remaining mixed gate を 1 画面で読めるようになった。
- GREEN:
  - `python3 scripts/current_l2_guided_samples.py reopen-map problem1 --format json`
  - `closed_split_packages` を public manifest として受け取れるようになった。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - reopen-map helper tests を含めて通過した。

## 9. What changed in understanding

- reopen-map public surface は split helper queue を持ち続けるより、
  split-package closeout と remaining mixed gate を分けて見せたほうが current queue 読みが安定する。
- split helper command 自体は separate command として残したままでも、
  reopen-map 側から stale queue を外すことで drift を下げられる。

## 10. Open questions

- representative reopen-map の split closeout sync はここで一巡したが、
  remaining mixed gate compression 自体は still next package である。
- final public contract / parser / packaging / exhaustive catalog は still later に残る。

## 11. Suggested next prompt

- reopen-map split closeout sync が済んだので、
  remaining mixed gate と true user-spec residual を narrow に圧縮してください。
