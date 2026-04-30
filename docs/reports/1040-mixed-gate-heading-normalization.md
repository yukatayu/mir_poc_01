# 1040 — mixed-gate heading normalization

## Objective

active `specs/examples/478`、`479`、`480`、`484..497`、`530`、`568` に残っていた mixed-gate / coupled-later `## next line` drift を、historical reopen memory / current kept-later mixed-gate status / historical queue anchor へ読み分け直し、current repo-level queue authority と分離する。

## Scope and assumptions

- scope は docs-only maintenance closeout に限る。
- `478/479/480` は heading と body の reopen wording を touch する。
- `484..497` は body の `remaining mixed gate` / `kept-later` readingを維持したまま、heading を `current kept-later mixed-gate status` へ揃える。
- `530`、`568` は historical queue anchor heading に揃える。
- current repo-level queue authority は `progress.md` / `tasks.md` を正とする。
- 規範判断、runtime behavior、sample taxonomy、`plan/` は変更しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `specs/examples/478-current-l2-model-check-second-line-concretization.md`
- `specs/examples/479-current-l2-theorem-discharge-actual-format-probe.md`
- `specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md`
- `specs/examples/484-current-l2-order-handoff-surface-artifact-threshold-default.md`
- `specs/examples/485-current-l2-theorem-contract-shape-threshold-default.md`
- `specs/examples/486-current-l2-theorem-transport-public-contract-coupled-later-gate.md`
- `specs/examples/487-current-l2-theorem-review-unit-transport-and-notebook-contract-actual-adoption.md`
- `specs/examples/488-current-l2-model-check-row-local-property-and-checker-boundary-actual-adoption.md`
- `specs/examples/489-current-l2-witness-provider-artifact-public-shape-actual-adoption.md`
- `specs/examples/490-current-l2-order-handoff-surface-actual-adoption.md`
- `specs/examples/491-current-l2-theorem-result-object-preview-and-proof-object-schema-reserve-actualization.md`
- `specs/examples/492-current-l2-model-check-public-checker-artifact-preview-and-verifier-handoff-reserve-actualization.md`
- `specs/examples/493-current-l2-witness-provider-public-contract-and-emitted-handoff-coupled-later-gate.md`
- `specs/examples/494-current-l2-theorem-proof-object-schema-and-prover-brand-coupled-later-gate.md`
- `specs/examples/495-current-l2-model-check-tool-brand-and-verifier-handoff-coupled-later-gate.md`
- `specs/examples/496-current-l2-order-handoff-source-wording-and-emitted-artifact-coupled-later-gate.md`
- `specs/examples/497-current-l2-theorem-result-object-and-payload-public-contract-coupled-later-gate.md`
- `specs/examples/530-current-l2-theorem-and-model-check-helper-preview-widening.md`
- `specs/examples/568-current-l2-theorem-model-check-bridge-carrier-reconnect-after-finite-index-widening.md`

## Actions taken

1. Audited the remaining mixed-gate / coupled-later family after `1039`.
2. Rewrote `478/479/480` so their reopen wording is explicitly historical queue memory and defers live authority to `progress.md` / `tasks.md`.
3. Renamed `## next line` headings in `484..497` to `## current kept-later mixed-gate status` so the heading matches the existing body semantics.
4. Renamed `## next line` in `530` / `568` to `## historical queue anchor`.
5. Mirrored the closeout in `progress.md` and `tasks.md`.
6. Added this report.

## Evidence / outputs / test results

- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- after `1039`, the remaining active-doc drift had compressed to mixed-gate and queue-anchor headings plus a small reopen-memory wording pocket in `478/479/480`.
- the coupled-later family already read as `remaining mixed gate` in-body, so the accurate fix was to align headings to current kept-later status rather than forcing those sections into historical queue memory.

## Open questions

- if a later audit still finds stale wording, it is likely outside the queue-authority family and should be treated as a separate maintenance lane.

## Suggested next prompt

`U1` 待ちのまま自走を続けるなら、active docs の one-pass stale wording audit をもう一度行い、queue-authority family に残件がないかを確認する。

## Reviewer findings and follow-up

- reviewer `Goodall` first flagged that `484..497` were better read as current kept-later mixed-gate status than historical queue memory, and also pointed out the missing reviewer section / placeholder command in this report.
- follow-up: kept `478/479/480` as historical reopen memory, changed `484..497` headings to `current kept-later mixed-gate status`, and replaced the placeholder command with the actual targeted `rg` invocation.
- `Goodall` follow-up review then reported no remaining scoped queue-authority or overclaim issue in `478/479/480/484..497/530/568`; only the report-discipline gap remained, which this patch closes.

## Commands run

- `rg -n '^## next line|next reopen line は|current package を close した後|current self-driven|historical closeout queue memory|current queue authority|kept-later|remaining mixed gate' specs/examples/478-current-l2-model-check-second-line-concretization.md specs/examples/479-current-l2-theorem-discharge-actual-format-probe.md specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md specs/examples/484-current-l2-order-handoff-surface-artifact-threshold-default.md specs/examples/485-current-l2-theorem-contract-shape-threshold-default.md specs/examples/486-current-l2-theorem-transport-public-contract-coupled-later-gate.md specs/examples/487-current-l2-theorem-review-unit-transport-and-notebook-contract-actual-adoption.md specs/examples/488-current-l2-model-check-row-local-property-and-checker-boundary-actual-adoption.md specs/examples/489-current-l2-witness-provider-artifact-public-shape-actual-adoption.md specs/examples/490-current-l2-order-handoff-surface-actual-adoption.md specs/examples/491-current-l2-theorem-result-object-preview-and-proof-object-schema-reserve-actualization.md specs/examples/492-current-l2-model-check-public-checker-artifact-preview-and-verifier-handoff-reserve-actualization.md specs/examples/493-current-l2-witness-provider-public-contract-and-emitted-handoff-coupled-later-gate.md specs/examples/494-current-l2-theorem-proof-object-schema-and-prover-brand-coupled-later-gate.md specs/examples/495-current-l2-model-check-tool-brand-and-verifier-handoff-coupled-later-gate.md specs/examples/496-current-l2-order-handoff-source-wording-and-emitted-artifact-coupled-later-gate.md specs/examples/497-current-l2-theorem-result-object-and-payload-public-contract-coupled-later-gate.md specs/examples/530-current-l2-theorem-and-model-check-helper-preview-widening.md specs/examples/568-current-l2-theorem-model-check-bridge-carrier-reconnect-after-finite-index-widening.md`
- `rg -n '^## next line$' specs/examples/478-current-l2-model-check-second-line-concretization.md specs/examples/479-current-l2-theorem-discharge-actual-format-probe.md specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md specs/examples/484-current-l2-order-handoff-surface-artifact-threshold-default.md specs/examples/485-current-l2-theorem-contract-shape-threshold-default.md specs/examples/486-current-l2-theorem-transport-public-contract-coupled-later-gate.md specs/examples/487-current-l2-theorem-review-unit-transport-and-notebook-contract-actual-adoption.md specs/examples/488-current-l2-model-check-row-local-property-and-checker-boundary-actual-adoption.md specs/examples/489-current-l2-witness-provider-artifact-public-shape-actual-adoption.md specs/examples/490-current-l2-order-handoff-surface-actual-adoption.md specs/examples/491-current-l2-theorem-result-object-preview-and-proof-object-schema-reserve-actualization.md specs/examples/492-current-l2-model-check-public-checker-artifact-preview-and-verifier-handoff-reserve-actualization.md specs/examples/493-current-l2-witness-provider-public-contract-and-emitted-handoff-coupled-later-gate.md specs/examples/494-current-l2-theorem-proof-object-schema-and-prover-brand-coupled-later-gate.md specs/examples/495-current-l2-model-check-tool-brand-and-verifier-handoff-coupled-later-gate.md specs/examples/496-current-l2-order-handoff-source-wording-and-emitted-artifact-coupled-later-gate.md specs/examples/497-current-l2-theorem-result-object-and-payload-public-contract-coupled-later-gate.md specs/examples/530-current-l2-theorem-and-model-check-helper-preview-widening.md specs/examples/568-current-l2-theorem-model-check-bridge-carrier-reconnect-after-finite-index-widening.md`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## Files changed

- `specs/examples/478-current-l2-model-check-second-line-concretization.md`
- `specs/examples/479-current-l2-theorem-discharge-actual-format-probe.md`
- `specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md`
- `specs/examples/484-current-l2-order-handoff-surface-artifact-threshold-default.md`
- `specs/examples/485-current-l2-theorem-contract-shape-threshold-default.md`
- `specs/examples/486-current-l2-theorem-transport-public-contract-coupled-later-gate.md`
- `specs/examples/487-current-l2-theorem-review-unit-transport-and-notebook-contract-actual-adoption.md`
- `specs/examples/488-current-l2-model-check-row-local-property-and-checker-boundary-actual-adoption.md`
- `specs/examples/489-current-l2-witness-provider-artifact-public-shape-actual-adoption.md`
- `specs/examples/490-current-l2-order-handoff-surface-actual-adoption.md`
- `specs/examples/491-current-l2-theorem-result-object-preview-and-proof-object-schema-reserve-actualization.md`
- `specs/examples/492-current-l2-model-check-public-checker-artifact-preview-and-verifier-handoff-reserve-actualization.md`
- `specs/examples/493-current-l2-witness-provider-public-contract-and-emitted-handoff-coupled-later-gate.md`
- `specs/examples/494-current-l2-theorem-proof-object-schema-and-prover-brand-coupled-later-gate.md`
- `specs/examples/495-current-l2-model-check-tool-brand-and-verifier-handoff-coupled-later-gate.md`
- `specs/examples/496-current-l2-order-handoff-source-wording-and-emitted-artifact-coupled-later-gate.md`
- `specs/examples/497-current-l2-theorem-result-object-and-payload-public-contract-coupled-later-gate.md`
- `specs/examples/530-current-l2-theorem-and-model-check-helper-preview-widening.md`
- `specs/examples/568-current-l2-theorem-model-check-bridge-carrier-reconnect-after-finite-index-widening.md`
- `progress.md`
- `tasks.md`
- `docs/reports/1040-mixed-gate-heading-normalization.md`

## plan/ 更新の有無

- 更新不要

## progress.md 更新の有無

- 更新した

## tasks.md 更新の有無

- 更新した

## samples_progress.md 更新の有無

- 更新不要

## skipped validations and reasons

- sample / cargo / full validation floor は未実行。今回は active example wording と snapshot docs だけの maintenance closeout であり、source hierarchy / docs scaffold / diff check を focused validation とした。

## remaining user decision blockers

- `U1` actual commitment:
  packaging / installed binary target、host integration target、first shipped public surface scope、final shared-space operational catalog breadth は引き続き user-facing decision を要する。

## commit / push status

- report authoring時点では未実行。same-package closeout で commit / push を行う。

## sub-agent session close status

- reviewer `Goodall` を使用し、findings を取り込んだ後に session を close した。
