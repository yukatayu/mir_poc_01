# 1032. order-handoff representative carry-over stale-path half cooling

## Objective

`specs/examples/525/527/528` と mirror docs に残っていた `samples/prototype/current-l2-order-handoff/` / `samples/lean/current-l2/` stale wording を冷やし、current front door を clean-near-end order-handoff `05/02/03` と active Lean docs、historical compare anchor を archived `p09/p13/p14` prototype / old Lean appendix に揃える。

## Scope and assumptions

- docs-first maintenance package として扱い、final public witness/provider/emitted-handoff contract、final source-surface wording、final public theorem contract は claim しない。
- current sample-side compare floor は `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir`、`02_missing_witness_rejected.mir`、`03_handoff_before_publication_rejected.mir` に置く。
- `p09/p13/p14` は archived compare anchors として保持し、active entrypoint へ戻さない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/525-current-l2-delegated-rng-provider-placement-representative-lean-sample-set-carryover.md`
- `specs/examples/527-current-l2-order-handoff-negative-static-stop-actualization.md`
- `specs/examples/528-current-l2-order-handoff-negative-pair-representative-lean-sample-set-carryover.md`
- `specs/examples/570-current-l2-authoritative-room-first-scenario-helper-summary-tightening.md`
- `samples/lean/README.md`
- `scripts/current_l2_lean_sample_sync.py`

## Actions taken

1. `specs/examples/525` を historical `p09` compare-anchor memory と current clean-near-end `05_delegated_rng_service` compare floor の split に書き換えた。
2. `specs/examples/527` を historical `p13/p14` negative compare-anchor memory と current clean-near-end `02/03` negative pair floor の split に書き換えた。
3. `specs/examples/528` を archived `p13/p14` carry-over anchor + old Lean appendix と active clean-near-end `02/03` + active Lean front door の split に書き換えた。
4. `specs/12 D-113/D-115/D-116`、`specs/11`、`specs/00` を同じ archived/current split に mirror した。
5. reviewer 指摘に従って `D-113`、`specs/11`、`specs/00` へ `05_delegated_rng_service` active Lean front door / archived old Lean appendix split を補った。
6. `progress.md` と `tasks.md` を更新し、remaining stale-docs line を `specs/examples/570` authoritative-room helper-summary current-surface cooling に狭めた。

## Files changed

- `specs/examples/525-current-l2-delegated-rng-provider-placement-representative-lean-sample-set-carryover.md`
- `specs/examples/527-current-l2-order-handoff-negative-static-stop-actualization.md`
- `specs/examples/528-current-l2-order-handoff-negative-pair-representative-lean-sample-set-carryover.md`
- `specs/12-decision-register.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/00-document-map.md`
- `progress.md`
- `tasks.md`

## Evidence / outputs / test results

- docs floor:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
- order-handoff compare floor:
  - `python3 scripts/clean_near_end_samples.py run order-handoff --format json`
  - pass。`05_delegated_rng_service` は `valid` / `success`、`02_missing_witness_rejected` と `03_handoff_before_publication_rejected` は expected malformed/static-stop families で返った。
- Lean sync:
  - `python3 scripts/current_l2_lean_sample_sync.py`
  - pass。`samples/lean/manifest.json` を出力し、working tree に追加 diff を残さなかった。
- targeted stale-path recheck:
  - `rg -n "samples/lean/current-l2|samples/prototype/current-l2-order-handoff|samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-order-handoff|samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/(p09|p13|p14)|samples/clean-near-end/order-handoff/(02_missing_witness_rejected|03_handoff_before_publication_rejected|05_delegated_rng_service)" specs/00-document-map.md specs/11-roadmap-and-workstreams.md specs/12-decision-register.md specs/examples/525-current-l2-delegated-rng-provider-placement-representative-lean-sample-set-carryover.md specs/examples/527-current-l2-order-handoff-negative-static-stop-actualization.md specs/examples/528-current-l2-order-handoff-negative-pair-representative-lean-sample-set-carryover.md`
  - pass。active clean-near-end floor と archived prototype / old Lean appendix split だけが残ることを確認した。
- reviewer follow-up:
  - reviewer が指摘した `D-113` / `specs/11` mirror の Lean split omission を補正した。
  - `specs/examples/525` leaf だけでなく、summary docs でも active `samples/lean/clean-near-end/05_delegated_rng_service/` と archived `samples/lean/old/.../p09.../` split を明示した。
- repo-wide remaining current-line drift probe:
  - `rg -n "samples/prototype/current-l2-order-handoff|samples/lean/current-l2/(p09-dice-delegated-rng-provider-placement|p13-dice-late-join-missing-publication-witness|p14-dice-late-join-handoff-before-publication)" .`
  - current docs maintenance の次 line が `specs/examples/570` helper-summary wording であることを確認した。`docs/reports/`、`docs/research_abstract/old/`、archived sample artifacts の historical references は維持。
- formatting / whitespace:
  - `git diff --check`

## What changed in understanding

- order-handoff representative carry-over cluster は一括 historical 化ではなく、`525` に current delegated-RNG compare floor、`527/528` に current negative-pair compare floor と active Lean front door が残る二層 split として扱うほうが正確だった。
- `p09/p13/p14` は current runnable entrypoint ではなく、pre-clean-near-end migration 前の compare-anchor / appendix memory として読む必要がある。
- repo-wide drift は old reports / archived research note と current docs mirror を分けて扱う必要があり、current docs maintenance の next line は `specs/examples/570` helper-summary current-surface wording に絞れる。

## Open questions

- `specs/examples/570` authoritative-room helper summary を、どこまで current clean-near-end / compatibility-front-door evidence と archived `p07/p08/p09/p13/p14` memory split に寄せるか。
- `specs/examples/526` は path drift 自体はないが、`570` cooling 後に helper-summary parity wording の追加同期が必要になるか。

## Suggested next prompt

`specs/examples/570` を対象に、authoritative-room helper-summary docs に残る old `run-source-sample samples/prototype/current-l2-order-handoff/...` current-surface wording を、active clean-near-end / compatibility-front-door evidence と archived compare-anchor memory split に揃え、必要な mirror docs と progress/tasks を同期してほしい。

## plan/ update status

- `plan/ 更新不要`

## progress.md update status

- 更新した。

## tasks.md update status

- 更新した。

## samples_progress.md update status

- 更新不要。active runnable sample dashboard は変えていない。

## Skipped validations and reasons

- full validation floor は未実行。今回の変更は docs-first stale-path cooling と active/archived path authority repair に限定し、implementation や sample taxonomy 自体は変更していないため、order-handoff family floor + Lean sync + docs floor に留めた。

## Commit / push status

- report authoring 時点では pending。
- package close commit / push は review と final local recheck の後に実施する。

## Sub-agent session close status

- reviewer session (`019dde88-d54d-7250-84a4-ae9a83b77a70`): completed。report missing timing gap 1 件と `D-113` / `specs/11` mirror 粒度 1 件を返し、後者を回収した。report missing 指摘は reviewer 実行時点で report 未作成だったため、report 生成後は解消済み。
