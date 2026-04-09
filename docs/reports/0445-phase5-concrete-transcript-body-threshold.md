# Report 0445 — phase5 concrete transcript body threshold

- Date: 2026-04-10 04:40 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen として、payload-ready retained bridge の次段で `concrete_transcript_body_ref` を current first choice に上げてよいかを docs-first に比較する
- Decision levels touched: L2 examples / non-normative roadmap and repository memory wording

## 1. Objective

`specs/examples/167...` の次段として、

- concrete transcript body family
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
- `specs/examples/165-current-l2-theorem-line-invocation-receipt-ready-runtime-transcript-threshold.md`
- `specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md`
- `specs/examples/167-current-l2-theorem-line-materialized-ready-concrete-payload-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `plan/00-index.md`

## 3. Actions taken

1. theorem-line retained bridge の latest stop line を `167...` まで見直し、次の reopen unit を `concrete_transcript_body_ref` と actual serialized channel body の二者分離として切り出した。
2. `specs/examples/168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md` を追加し、`concrete_transcript_body_ref` だけを current first choice に固定した。
3. mirror と phase snapshot を transcript-body-ready retained bridge まで更新した。
4. review record 雛形を追加した。

## 4. Files changed

- `specs/examples/168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md`
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
- `docs/reports/0445-phase5-concrete-transcript-body-threshold.md`
- `docs/reports/0446-review-phase5-concrete-transcript-body-threshold.md`

## 5. Commands run and exact outputs

```text
$ rg -n "concrete transcript body|serialized channel body|payload_ready|transcript_body_ready" specs/examples/16[5-8]* docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md plan/11-roadmap-near-term.md plan/13-heavy-future-workstreams.md progress.md tasks.md
specs/examples/168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md:6:- payload-ready retained bridge に concrete transcript body family をどこまで足すか
specs/examples/168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md:7:- actual serialized channel body をどこまで後段に残すか
docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md:196:**`concrete_transcript_body_ref`**
plan/13-heavy-future-workstreams.md:197:- したがって current next pressure は low-level memory-order の即時導入ではなく、actual serialized channel body comparison を narrow に比較することである。
progress.md:24:- **Phase 5 small decidable core / proof / async-control boundary** は、`specs/examples/126...` から `specs/examples/168...` までで theorem-line later package の current first choice を段階的に伸ばし、現時点では `concrete_transcript_body_ref` までを symbolic retained bridge に寄せられるところまで整理した。したがって current package は theorem-line later package close とみなし、next later reopen は **actual serialized channel body comparison** の comparison に寄せてよい。
tasks.md:27:| 2 | Phase 5 checkpoint 後半 / later reopen 候補 | actual serialized channel body comparison | `proof_notebook` first bridge に `concrete_transcript_body_ref` までを足したうえで、actual serialized channel body をどこまで theorem-side bridge に近づけるかを比べる | 中〜重 | 0〜2 task | 一部自走可能 | `specs/examples/126...` から `168...` までで current theorem-line package は close。`proof_assistant_adapter` pressure は second practical candidate に残す |
```

## 6. Evidence / findings

- `concrete_payload_ref` の次段では、transcript body family だけを serialized channel body と分離して切るのが最小である。
- `concrete_transcript_body_ref` を入れても、actual serialized channel body は still theorem-line bridge の外に残せる。
- serialized body を同時に入れると、transcript rendering と transport / attachment body の境界が premature に混ざりやすい。
- `plan/ 更新済み`
- `progress.md 更新済み`
- `tasks.md 更新済み`

## 7. Changes in understanding

- Phase 5 later reopen の next minimal unit は「transcript body と serialized body together」ではなく、「transcript body first, serialized body second」である。
- `concrete_payload_ref` と `concrete_transcript_body_ref` の間にも narrow bridge があり、ここを 1 task で切る価値がある。

## 8. Open questions

- `concrete_transcript_body_ref` を transcript rendering body に留めるか、consumer-specific formatted view まで下ろすか。
- actual serialized channel body を transcript family の一部とみなすか、transport artifact family とみなすか。
- serialized channel body reopen のときに emitted notebook attachment body を同じ family に入れるか。

## 9. Suggested next prompt

Phase 5 theorem-line later package の次段として、transcript-body-ready retained bridge を前提に、actual serialized channel body をどこまで theorem-line bridge に寄せるかを narrow comparison してください。
