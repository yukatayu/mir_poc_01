# Report 0442 — phase5 concrete payload threshold

- Date: 2026-04-10 04:08 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen として、materialized-ready retained bridge の次段で `concrete_payload_ref` を current first choice に上げてよいかを docs-first に比較する
- Decision levels touched: L2 examples / non-normative roadmap and repository memory wording

## 1. Objective

`specs/examples/166...` の次段として、

- concrete payload family
- concrete transcript body
- actual serialized channel body

のうち、どこまでを theorem-line retained bridge に寄せてよいかを narrow comparison し、current first choice を 1 本に固定する。

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
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/163-current-l2-theorem-line-wording-ready-runtime-handoff-threshold.md`
- `specs/examples/164-current-l2-theorem-line-runtime-handoff-ready-invocation-receipt-threshold.md`
- `specs/examples/165-current-l2-theorem-line-invocation-receipt-ready-runtime-transcript-threshold.md`
- `specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 3. Actions taken

1. theorem-line retained bridge の latest stop line を `166...` まで見直し、次の reopen unit を `concrete_payload_ref` / transcript body / serialized channel body の三者分離として切り出した。
2. `specs/examples/167-current-l2-theorem-line-materialized-ready-concrete-payload-threshold.md` を追加し、`concrete_payload_ref` だけを current first choice に固定した。
3. mirror と phase snapshot を payload-ready retained bridge まで更新した。
4. review record 雛形を追加した。

## 4. Files changed

- `specs/examples/167-current-l2-theorem-line-materialized-ready-concrete-payload-threshold.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0442-phase5-concrete-payload-threshold.md`
- `docs/reports/0443-review-phase5-concrete-payload-threshold.md`

## 5. Commands run and exact outputs

```text
$ rg -n "concrete payload|transcript body|serialized channel body|concrete channel payload|payload body|channel body" specs/examples/12[6-9]* specs/examples/13* specs/examples/14* specs/examples/15* specs/examples/16[0-6]* docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md tasks.md progress.md plan/11-roadmap-near-term.md plan/13-heavy-future-workstreams.md
[matches inspected locally]
```

## 6. Evidence / findings

- `materialized_runtime_handoff_ref` の次段では、payload family だけを transcript / serialized body と分離して切るのが最小である。
- `concrete_payload_ref` を入れても、transcript formatting / serialized channel body policy は still theorem-line bridge の外に残せる。
- transcript body を同時に入れると、consumer rendering と transport serialization の境界が premature に混ざりやすい。
- `plan/ 更新済み`
- `progress.md 更新済み`
- `tasks.md 更新済み`

## 7. Changes in understanding

- Phase 5 later reopen の next minimal unit は「payload and transcript together」ではなく、「payload first, transcript second」である。
- `materialized_runtime_handoff_ref` と `concrete_payload_ref` の間には narrow bridge があり、ここを 1 task で切る価値がある。

## 8. Open questions

- `concrete_payload_ref` を payload family 単位に留めるか、consumer-specific payload attachment まで下ろすか。
- concrete transcript body を separate ref にするか、payload family attachment とみなすか。
- actual serialized channel body を transcript family の一部とみなすか、transport artifact family とみなすか。

## 9. Suggested next prompt

Phase 5 theorem-line later package の次段として、payload-ready retained bridge を前提に、concrete transcript body をどこまで theorem-line bridge に寄せるかを narrow comparison してください。

