# 1030. finite-index typing and lean formal skeleton stale-path cooling

## Objective

`specs/examples/566/567` と mirror docs に残っていた pre-clean-near-end prototype path / old Lean corpus path の stale current wording を冷やし、current front door を active clean-near-end typing / `samples/lean/clean-near-end`、historical appendix を archived prototype / `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/` に揃える。

## Scope and assumptions

- docs-first maintenance package として扱い、final typed calculus、final typed source principal、final public checker payload schema、final public theorem / verifier contract は claim しない。
- active source-side typing compare floor は `samples/clean-near-end/typing/04_capture_escape_rejected.mir` と `05_cost_bound_rejected.mir` を含む clean-near-end typing family に置く。
- `p15-typed-capture-escape-rejected` と `p16-typed-remote-call-budget-exceeded` は archived prototype anchor / archived old Lean appendix として保持し、active path へ戻さない。

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
- `specs/examples/566-current-l2-finite-index-first-layer-capture-lifetime-cost-actualization.md`
- `specs/examples/567-current-l2-lean-first-formal-skeleton-hardening-after-finite-index-widening.md`
- `samples/lean/README.md`
- `scripts/current_l2_lean_sample_sync.py`
- `scripts/tests/test_current_l2_lean_sample_sync.py`

## Actions taken

1. `specs/examples/566` を historical Phase 5 / Package 92 closeout memory として冷やし、`p15 / p16` を archived prototype anchor、current sample-side compare floor を clean-near-end typing `04 / 05` として切り分けた。
2. `specs/examples/567` を historical Phase 5 / Package 93 closeout memory として冷やし、`samples/lean/foundations/`、active `samples/lean/clean-near-end/`、archived `samples/lean/old/2026-04-22-pre-clean-near-end/current-l2/` の 3-way split を明示した。
3. `specs/12 D-151/D-152` を current clean-near-end compare floor / active generated stub front door / archived appendix split に同期した。
4. `specs/11` と `specs/00` の package summary / document-map mirror を同じ split に合わせて補正した。
5. `progress.md` と `tasks.md` を更新し、remaining stale-docs line を `specs/examples/522..529` の representative carry-over cluster へ狭めた。
6. reviewer 指摘を受けて `specs/11` の `566` row を historical queue anchor wording に冷やし、`tasks.md` に archived prototype anchor split を追記し、report の stale-token recheck scope を actual evidence scope に合わせた。

## Files changed

- `specs/examples/566-current-l2-finite-index-first-layer-capture-lifetime-cost-actualization.md`
- `specs/examples/567-current-l2-lean-first-formal-skeleton-hardening-after-finite-index-widening.md`
- `specs/12-decision-register.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/00-document-map.md`
- `progress.md`
- `tasks.md`

## Evidence / outputs / test results

- docs floor:
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
- typing compare floor:
  - `python3 scripts/clean_near_end_samples.py run typing --format json`
  - pass。`01_authorized_declassification` は `valid` / `success`、`02_unauthorized_declassification_rejected`・`03_label_flow_rejected`・`04_capture_escape_rejected`・`05_cost_bound_rejected` は expected malformed/static-stop families で返った。
- Lean sync:
  - `python3 scripts/current_l2_lean_sample_sync.py`
  - pass。`samples/lean/manifest.json` を出力し、working tree に追加 diff を残さなかった。
- targeted stale-path recheck:
  - `rg -n "samples/lean/current-l2|samples/prototype/current-l2-typed-proof-model-check" specs/examples/566-current-l2-finite-index-first-layer-capture-lifetime-cost-actualization.md specs/examples/567-current-l2-lean-first-formal-skeleton-hardening-after-finite-index-widening.md specs/00-document-map.md specs/11-roadmap-and-workstreams.md specs/12-decision-register.md`
  - exit `1`。primary mirror set で stale token zero match を確認した。
  - reviewer closeout 時に `progress.md` と `tasks.md` も追加確認し、same stale token zero match を確認した。report 本文は evidence command / next prompt で stale token を引用するため、この `rg` scope には含めていない。
- formatting / whitespace:
  - `git diff --check`

## What changed in understanding

- active `samples/current-l2/` source corpus と stale `samples/lean/current-l2/` old generated corpus を同列に扱ってはいけない。source corpus は still current だが、Lean corpus 側は `clean-near-end` と `old/.../current-l2` に明確に split 済みだった。
- `p15 / p16` は current semantic line の historical prototype anchor ではあるが、current execution / inspection / Lean sync front door は clean-near-end typing `04 / 05` と active `samples/lean/clean-near-end/` に移っている。
- `current_l2_lean_sample_sync.py` は active clean-near-end generated stub corpus only を再生成するため、old Lean corpus wording を current sync path として残すのは即座に drift になる。

## Open questions

- `specs/examples/522..529` の representative carry-over / IFC helper widening cluster を、どこまで current active clean-near-end / archived old corpus split に寄せ、どこまで historical helper/package memory に冷やすか。
- old Lean corpus wording を含む earlier examples のうち、current active lean front door (`samples/lean/README.md`) を mirror するものと pure historical package memory に留めるものの境界をどこで切るか。

## Suggested next prompt

`specs/examples/522..529` を対象に、source-side IFC / delegated-RNG / negative-pair representative carry-over docs に残る `samples/prototype/current-l2-*` と `samples/lean/current-l2/` stale wording を、active clean-near-end / archived prototype / archived old Lean corpus split に揃え、必要な mirror docs と progress/tasks を同期してほしい。

## plan/ update status

- `plan/ 更新不要`

## progress.md update status

- 更新した。

## tasks.md update status

- 更新した。

## samples_progress.md update status

- 更新不要。active runnable sample dashboard や command matrix は変えていない。

## Skipped validations and reasons

- full validation floor は未実行。今回の変更は docs-first stale-path cooling と active/archived path re-anchoringに限定し、implementation や sample taxonomy 自体は変更していないため、typing floor + Lean sync + docs floor に留めた。

## Commit / push status

- report authoring 時点では pending。
- package close commit / push はこの report 生成後に実施する。

## Sub-agent session close status

- docs_researcher session (`019dde69-1ef7-7241-bbba-399dc73993f3`): completed。`Package 92-93 finite-index typing stale-path cooling` を最小 coherent cluster として特定し、close 済み。
- reviewer session (`019dde71-2e47-7f21-90de-b75b80aef10e`): completed。3 件の wording / scope findings を回収したうえで close 済み。
