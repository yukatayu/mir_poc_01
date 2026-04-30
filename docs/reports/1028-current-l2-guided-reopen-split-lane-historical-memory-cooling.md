# 1028. current-l2 guided reopen-split-lane historical-memory cooling

## Objective

`scripts/current_l2_guided_samples.py` の current active compatibility front door が
`list / smoke-all / closeout` の 3 コマンドだけである事実に合わせて、
`reopen-map` / `split ...` / `residuals` / `lane ...` cluster と、
それに依存していた archived problem-bundle doc mirror を
historical helper memory / historical queue memory へ冷やす。

## Scope and assumptions

- docs-first maintenance package に限定する。
- final public parser / checker / runtime API、final public CLI / tutorial surface、
  final public theorem/model-check/witness-provider contract、
  final public source wording、final public verifier contract は claim しない。
- `specs/examples/588..599` と `specs/12 D-173..184` は
  current active command surface ではなく、
  2026-04-22 clean-sample migration 前の historical helper-local memory として扱う。
- `specs/examples/600` / `D-185` と archived problem-bundle / quickstart / matrix / bundle cluster は、
  次 package に残す。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/588-current-l2-representative-problem-mixed-gate-reopen-map-refresh.md`
- `specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md`
- `specs/examples/590-current-l2-problem1-typed-source-principal-split-helper-actualization.md`
- `specs/examples/591-current-l2-problem1-theorem-public-contract-split-helper-actualization.md`
- `specs/examples/592-current-l2-problem1-model-check-public-contract-split-helper-actualization.md`
- `specs/examples/593-current-l2-problem2-source-wording-emitted-schema-split-helper-actualization.md`
- `specs/examples/594-current-l2-problem2-witness-provider-public-shape-split-helper-actualization.md`
- `specs/examples/595-current-l2-representative-problem-reopen-map-split-closeout-sync.md`
- `specs/examples/596-current-l2-remaining-residual-lane-summary-actualization.md`
- `specs/examples/597-current-l2-problem1-final-public-seam-lane-helper-actualization.md`
- `specs/examples/598-current-l2-problem2-final-public-seam-lane-helper-actualization.md`
- `specs/examples/599-current-l2-syntax-modality-final-marker-lane-helper-actualization.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1028-current-l2-guided-reopen-split-lane-historical-memory-cooling.md`
- `scripts/current_l2_guided_samples.py`
- `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/README.md`
- `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/problem2-order-handoff-shared-space.md`

## Actions taken

1. `scripts/current_l2_guided_samples.py` を再確認し、current active compatibility front door が `list / smoke-all / closeout` のみであることを baseline とした。
2. exploratory command として `reopen-map problem1`、`residuals --format json`、`lane problem1-final-public-seams --format json`、`lane syntax-modality-final-marker --format json`、5 本の `split ...` command を実行し、すべて migration note + `supported compatibility commands: list, smoke-all, closeout` で exit 2 になることを確認した。
3. `specs/examples/588..594` を historical reopen-map / split helper memory に冷やし、archived `samples/old/2026-04-22-pre-clean-near-end/problem-bundles/...` path を historical doc memory として明示した。
4. `specs/examples/595..599` を historical reopen-map split-closeout / residual-summary / lane helper memory に冷やし、queue wording は historical closeout queue memory として `progress.md` / `tasks.md` authority に戻した。
5. `specs/12 D-173..184` を historical helper memory judgment に更新し、current active compatibility front door を `list|smoke-all|closeout` に固定した。
6. `specs/11` と `specs/00` の `588..599` mirror を historical helper memory / historical closeout queue memory wording に同期した。
7. `progress.md` と `tasks.md` に current maintenance snapshot と recent log を追記し、残件を `specs/examples/600` / `D-185` と older bundle/quickstart/matrix mirror に圧縮した。
8. reviewer sub-agent の low findings に従い、report の consulted-docs baseline と files-changed self-description を補った。

## Files changed

- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/588-current-l2-representative-problem-mixed-gate-reopen-map-refresh.md`
- `specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md`
- `specs/examples/590-current-l2-problem1-typed-source-principal-split-helper-actualization.md`
- `specs/examples/591-current-l2-problem1-theorem-public-contract-split-helper-actualization.md`
- `specs/examples/592-current-l2-problem1-model-check-public-contract-split-helper-actualization.md`
- `specs/examples/593-current-l2-problem2-source-wording-emitted-schema-split-helper-actualization.md`
- `specs/examples/594-current-l2-problem2-witness-provider-public-shape-split-helper-actualization.md`
- `specs/examples/595-current-l2-representative-problem-reopen-map-split-closeout-sync.md`
- `specs/examples/596-current-l2-remaining-residual-lane-summary-actualization.md`
- `specs/examples/597-current-l2-problem1-final-public-seam-lane-helper-actualization.md`
- `specs/examples/598-current-l2-problem2-final-public-seam-lane-helper-actualization.md`
- `specs/examples/599-current-l2-syntax-modality-final-marker-lane-helper-actualization.md`
- `progress.md`
- `tasks.md`

## Evidence / outputs / test results

### Exploratory failures that changed the reading

- `python3 scripts/current_l2_guided_samples.py reopen-map problem1`
  - exit 2
  - stderr:
    `current_l2_guided_samples.py now forwards to the clean near-end active suite. Legacy problem1/problem2 bundle commands were retired with the 2026-04-22 clean-sample migration.`
    `supported compatibility commands: list, smoke-all, closeout`
- `python3 scripts/current_l2_guided_samples.py residuals --format json`
  - exit 2 with the same migration note
- `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams --format json`
  - exit 2 with the same migration note
- `python3 scripts/current_l2_guided_samples.py lane syntax-modality-final-marker --format json`
  - exit 2 with the same migration note
- `python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal`
  - exit 2 with the same migration note
- `python3 scripts/current_l2_guided_samples.py split problem1 theorem-public-contract`
  - exit 2 with the same migration note
- `python3 scripts/current_l2_guided_samples.py split problem1 model-check-public-contract`
  - exit 2 with the same migration note
- `python3 scripts/current_l2_guided_samples.py split problem2 source-wording-emitted-schema`
  - exit 2 with the same migration note
- `python3 scripts/current_l2_guided_samples.py split problem2 witness-provider-public-shape`
  - exit 2 with the same migration note

### Focused validation run

- `python3 scripts/current_l2_guided_samples.py list`
  - pass
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - pass
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
  - pass
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass (`Found 1025 numbered report(s).` before this report)
- `git diff --check`
  - pass

### Closeout rerun after report addition

- `python3 scripts/validate_docs.py`
  - pass (`Found 1026 numbered report(s).`)
- `git diff --check`
  - pass

## What changed in understanding

- emit-family を historical memory に落としただけでは足りず、
  `reopen-map` / `split` / `residuals` / `lane` cluster 全体も、
  live wrapper が受けない以上は current active helper surface として書けない。
- archived problem-bundle docs を current path のまま参照すると、
  helper command retirementだけでなく path hierarchy も誤読させる。
- package-local queue memory は retired helper docs の中に残ってもよいが、
  live roadmap authority として読ませないために `progress.md` / `tasks.md` へ戻す必要がある。

## Open questions

- `specs/examples/600` / `D-185` は `check-source-sample` 自体が still current である一方、
  Problem 1 bundle quickstart path は archived sample-bundle memory に残っているため、
  mixed current/historical wording の整理が必要。
- older `matrix` / `bundle` / `quickstart` cluster と `samples/problem-bundles/...` mirror には、
  archived path と retired helper surface の温度差が残っている。

## Suggested next prompt

`current_l2_guided_samples.py` の archived problem-bundle / quickstart / matrix / bundle cluster を次 package で冷やし、`specs/examples/600` / `D-185` と `specs/examples/581..587`、`specs/11` / `specs/00` / `specs/12 D-166..172` の mirror wording を current wrapper reading に同期してください。

## plan/ updates

- 更新不要

## progress.md updates

- 更新した

## tasks.md updates

- 更新した

## samples_progress.md updates

- 更新不要

## Skipped validations and reasons

- full validation floor は未実行。
  - 理由: docs-only maintenance package であり、current wrapper front door / source hierarchy / docs scaffold / focused helper negative-evidence で今回の claim を十分に裏づけられるため。
- cargo tests と sample-specific runtime checks は未実行。
  - 理由: runnable sample implementation / taxonomy / progress % 自体は今回変更していないため。

## Commit / push status

- report作成時点: 未commit / 未push

## Sub-agent session close status

- `Banach` (`019dde2d-b0ee-7ca0-9352-6e900feef516`)
  - completed を受領後に close 済み
- `Boyle` (`019dde3b-f58d-7470-b74e-17491d780345`)
  - low findings を受領後に修正を反映し、その後 close 済み
