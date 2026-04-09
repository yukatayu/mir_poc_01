# 0422 — phase5 exchange rule threshold package

## Objective

Phase 5 theorem-line later package の次段として、adapter-linked retained bridge に `exchange_rule_ref` をどこまで current first choice として足してよいかを比較し、adapter-local validation をどこまで後段に残すかを docs-first で固定する。

## Scope and assumptions

- 対象は `proof_notebook` first bridge に限る。
- current 前提は `specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md` までである。
- current task では adapter-local validation rule、concrete file / blob exchange protocol、environment-specific consumer invocation surface は固定しない。
- `plan/` は relevant mirror を同 task で更新する。
- `progress.md` と `tasks.md` は current status / next later reopen の読みが変わるため更新する。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md`
- `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md`
- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md`
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
- `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`
- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`
- `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md`
- `specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md`
- `specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md`
- `specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md`
- `specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md`
- `specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md`
- `specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md`
- `specs/examples/142-current-l2-theorem-line-compare-ready-bridge-bless-decision-threshold.md`
- `specs/examples/143-current-l2-theorem-line-bless-ready-bridge-review-session-threshold.md`
- `specs/examples/144-current-l2-theorem-line-review-linked-bless-bridge-retained-notebook-threshold.md`
- `specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md`
- `specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md`
- `specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md`
- `specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md`
- `specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md`
- `specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md`
- `specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md`
- `specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. `specs/examples/153-current-l2-theorem-line-adapter-linked-exchange-rule-threshold.md` を追加し、
   - terminal cut 維持
   - `exchange_rule_ref` だけを足す案
   - `exchange_rule_ref + adapter_validation_ref` をまとめて足す案
   を比較した。
2. current judgment として、adapter-linked retained bridge に `exchange_rule_ref` だけを足し、adapter-local validation / concrete file-blob exchange は still 後段に残す cut を採用した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を mirror した。
4. next later reopen を `notebook exchange rule threshold` から `adapter-validation threshold` へ差し替えた。
5. reviewer を最後に 1 回だけ回す前提で package report を作成した。

## Files changed

- `specs/examples/153-current-l2-theorem-line-adapter-linked-exchange-rule-threshold.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0422-phase5-exchange-rule-threshold-package.md`

## Evidence / outputs / test results

- resource check は current session 冒頭の `df -h .` / `free -h` を継続参照した。
- timestamp:
  - `date '+%Y-%m-%d %H:%M %Z'` → `2026-04-10 01:15 JST`
- package close 後に実行予定:
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## What changed in understanding

- theorem-line later package は `consumer_adapter_ref` で terminal cut にするより、`exchange_rule_ref` までを symbolic ref として current first choice に含める方が自然である。
- ただし `exchange_rule_ref` を adapter-local validation と結びつけるのは still 早く、validation / environment-specific invocation surface / concrete file or blob exchange は次段の later reopen に残すべきである。
- したがって next later reopen の名前も `notebook exchange rule threshold` ではなく `adapter-validation threshold` に切り替えるのが正確である。

## Open questions

- `adapter_validation_ref` を theorem-line retained bridge にどこまで足すか。
- actual notebook exchange rule と adapter-local validation を同じ reopen package に置くべきか。
- `proof_assistant_adapter` pressure を notebook line より先に practical reopen に上げる条件があるか。
- `plan/` 更新不要ではない。今回の task では relevant mirror を更新した。
- `progress.md` 更新不要ではない。今回の task では current phase 読みと next step が変わるため更新した。
- `tasks.md` 更新不要ではない。今回の task では current task map を全体書き直しした。

## Suggested next prompt

Phase 5 theorem-line later reopen の次段として、`exchange_rule_ref` の上に `adapter_validation_ref` をどこまで足してよいか、actual notebook exchange rule / environment-specific invocation surface / concrete file-blob exchange protocol をどう分けるべきかを narrow に比較してください。
