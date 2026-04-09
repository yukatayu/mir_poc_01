# Report 0382 — review phase5 proof obligation matrix and handoff artifact package

- Date: 2026-04-09T10:32:19.571874Z
- Author / agent: Codex
- Scope: current uncommitted docs-only Phase 5 package review。`specs/examples/127...`、`docs/research_abstract/phase5...`、mirror updates の semantic consistency / mirror drift / evidence overreach を current diff 限定で確認する
- Decision levels touched: review only / no normative edit

## 1. Objective

Current task として、current uncommitted diff のうち

- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`

を対象に、次を review する。

1. new judgment が `specs/examples/126...` と detached/static/shared-space 既存 evidence に整合するか
2. mirror files が completion を言い過ぎていないか、OPEN の残し方に drift がないか
3. Phase 5 を second inventory package close に動かした phase reading が coherent か

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
- `specs/examples/43-current-l2-complete-stable-static-reason-tranche.md`
- `specs/examples/44-current-l2-checked-reasons-coexistence-and-shrink-policy.md`
- `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md`
- `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md`
- `specs/examples/125-shared-space-control-plane-carrier-threshold.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0146-current-l2-detached-static-reason-code-mirror.md`
- `docs/reports/0379-phase5-small-decidable-core-and-proof-boundary-inventory.md`
- `docs/reports/0381-phase5-proof-obligation-matrix-and-handoff-artifact.md`

## 3. Actions taken

1. repo の required reading order を満たす範囲で top-level docs / core specs / relevant plan files を再読した。
2. `git diff` と `git status` で current uncommitted docs package を確認した。
3. `specs/examples/126...` と `specs/examples/127...` を突き合わせ、new matrix / handoff sketch が 4-way split を壊していないかを確認した。
4. detached static gate / `checked_reason_codes` / shared-space witness 系の既存 docs を再読し、`127` の example と terminology が source-backed boundary を越えていないかを確認した。
5. `Documentation.md`、`specs/00`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md` の updated wording を比較し、Phase 5 second package close の phase reading と OPEN の残し方を確認した。

## 4. Files changed

- `docs/reports/0382-review-phase5-proof-obligation-matrix-and-handoff-artifact-package.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-09 19:32 JST

$ python3 scripts/new_report.py --slug review-phase5-proof-obligation-matrix-and-handoff-artifact-package
/home/yukatayu/dev/mir_poc_01/docs/reports/0382-review-phase5-proof-obligation-matrix-and-handoff-artifact-package.md
```

Review 中には `git status --short`、`git diff --stat -- ...`、`git diff -- ...`、`sed -n`、`nl -ba`、`rg -n` を使って current diff と source anchor を確認した。

## 6. Evidence / findings

### Finding 1

`specs/examples/127...` は shared-space line で分けたはずの witness core と provider receipt attachment を、example row で再び曖昧にしている。`127` は protocol-side example を `obligation_kind = witness_receipt_consistency` と置きながら evidence を `room_profile` と `witness` のみで示しており、receipt 側参照を持たないまま receipt consistency を語っている。[specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md](/home/yukatayu/dev/mir_poc_01/specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md#L165) [specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md](/home/yukatayu/dev/mir_poc_01/specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md#L175) [specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md](/home/yukatayu/dev/mir_poc_01/specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md#L243)

This drifts from the existing shared-space cut, which explicitly keeps the witness core as `witness_kind + action_ref + draw_slot + draw_result`, and keeps provider receipt as an optional audit/receipt-side attachment outside the witness core.[specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md](/home/yukatayu/dev/mir_poc_01/specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md#L175) [specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md](/home/yukatayu/dev/mir_poc_01/specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md#L186) [specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md](/home/yukatayu/dev/mir_poc_01/specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md#L248) [specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md](/home/yukatayu/dev/mir_poc_01/specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md#L272) [specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md](/home/yukatayu/dev/mir_poc_01/specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md#L302)

### Finding 2

The mirror set now reads Phase 5 as a closed second inventory package, but the evidence trail for that close is incomplete. `progress.md` is timestamped to `2026-04-09 19:20 JST` and states second-package close, yet its work log still ends at the first-package `18:33 JST` entry and has no Phase 5 second-package close log line.[progress.md](/home/yukatayu/dev/mir_poc_01/progress.md#L3) [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md#L24) [progress.md](/home/yukatayu/dev/mir_poc_01/progress.md#L277)

`plan/90` has the same gap: the new addendum cites `127`, the research abstract, and the old first-package report `0379`, but not a completed report for the second package.[plan/90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md#L261) The only second-package report presently in the worktree is an empty template with no objective, inputs, actions, evidence, or open questions filled in.[docs/reports/0381-phase5-proof-obligation-matrix-and-handoff-artifact.md](/home/yukatayu/dev/mir_poc_01/docs/reports/0381-phase5-proof-obligation-matrix-and-handoff-artifact.md#L1) That makes the “second inventory package close” wording in mirrors materially ahead of the repo’s own documented evidence discipline.

## 7. Changes in understanding

- `127` 自体の大筋、すなわち「matrix を docs 正本に置き、handoff artifact は source evidence を参照する sketch に留める」という判断は、`126` と detached/static evidence の line には概ね整合している。
- ただし shared-space example では witness core と provider receipt attachment の separation を守る wording がまだ必要である。
- mirror の phase reading 自体はほぼ揃っているが、package close を支える report / progress log / traceability evidence が不足すると overclaim になる。

## 8. Open questions

- `127` の protocol-side example は `witness_validity` のような obligation 名へ落とすべきか、それとも provider receipt ref を evidence 側に明示して receipt consistency row として維持すべきか。
- Phase 5 second package close を source-backed close と読むなら、どの report を canonical package report として traceability に載せるか。
- `progress.md` の task-close log は second package close を 1 行で追記するだけで十分か、それとも package evidence の source file 名まで明記するか。

## 9. Suggested next prompt

Phase 5 package の review findings を反映してください。特に、`specs/examples/127...` で witness core と provider receipt attachment の separation を崩さない wording に直し、Phase 5 second inventory package close を支える report / traceability / progress log を current repo policy に合わせて補完してください。
