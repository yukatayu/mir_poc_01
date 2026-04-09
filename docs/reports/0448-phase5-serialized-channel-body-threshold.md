# Report 0448 — phase5 serialized channel body threshold

- Date: 2026-04-10 04:46 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen として、transcript-body-ready retained bridge の次段で `serialized_channel_body_ref` を current first choice に上げてよいかを docs-first に比較する
- Decision levels touched: L2 examples / non-normative roadmap and repository memory wording

## 1. Objective

`specs/examples/168...` の次段として、

- actual serialized channel body family
- actual emitted attachment blob / file body

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
- `specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md`
- `specs/examples/167-current-l2-theorem-line-materialized-ready-concrete-payload-threshold.md`
- `specs/examples/168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. theorem-line retained bridge の latest stop line を `168...` まで見直し、次の reopen unit を `serialized_channel_body_ref` と actual emitted attachment blob / file body の二者分離として切り出した。
2. `specs/examples/169-current-l2-theorem-line-transcript-body-ready-serialized-channel-body-threshold.md` を追加し、`serialized_channel_body_ref` だけを current first choice に固定した。
3. mirror と phase snapshot を serialized-ready retained bridge まで更新した。
4. review record 雛形を追加した。

## 4. Files changed

- `specs/examples/169-current-l2-theorem-line-transcript-body-ready-serialized-channel-body-threshold.md`
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
- `docs/reports/0448-phase5-serialized-channel-body-threshold.md`
- `docs/reports/0449-review-phase5-serialized-channel-body-threshold.md`

## 5. Commands run and exact outputs

```text
$ rg -n "serialized channel body|attachment blob|file body|serialized_ready" specs/examples/16[6-9]* docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md plan/11-roadmap-near-term.md plan/13-heavy-future-workstreams.md progress.md tasks.md
specs/examples/168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md:7:- actual serialized channel body をどこまで後段に残すか
specs/examples/169-current-l2-theorem-line-transcript-body-ready-serialized-channel-body-threshold.md:7:- transcript-body-ready retained bridge に actual serialized channel body family をどこまで足すか
docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md:196:**`concrete_transcript_body_ref`**
plan/11-roadmap-near-term.md:8:... `specs/examples/168...` で `concrete_transcript_body_ref` までを current first choice に置くところまで進んだ ...
plan/13-heavy-future-workstreams.md:182:- ... `concrete_transcript_body_ref` までは symbolic retained bridge に段階的に足し、actual serialized channel body は later reopen に残す ...
progress.md:56:2. Phase 5 current package は checkpoint close として維持し、actual serialized channel body comparison が必要になったときだけ later pressure で reopen する
tasks.md:27:| 2 | Phase 5 checkpoint 後半 / later reopen 候補 | actual serialized channel body comparison | ...
```

## 6. Evidence / findings

- `concrete_transcript_body_ref` の次段では、serialized channel body family だけを emitted attachment blob / file body と分離して切るのが最小である。
- `serialized_channel_body_ref` を入れても、actual emitted attachment blob / file body は still theorem-line bridge の外に残せる。
- emitted blob / file body を同時に入れると、transport serialization body と retained artifact body の境界が premature に混ざりやすい。
- `plan/ 更新済み`
- `progress.md 更新済み`
- `tasks.md 更新済み`

## 7. Changes in understanding

- Phase 5 later reopen の next minimal unit は「serialized body と emitted blob/file body together」ではなく、「serialized body first, emitted blob/file body second」である。
- `concrete_transcript_body_ref` と `serialized_channel_body_ref` の間にも narrow bridge があり、ここを 1 task で切る価値がある。

## 8. Open questions

- `serialized_channel_body_ref` を transport-facing channel body に留めるか、consumer envelope body まで下ろすか。
- actual emitted attachment blob / file body を transport artifact family とみなすか、retained artifact family とみなすか。
- emitted notebook attachment body と retained file body を同じ reopen で扱うか。

## 9. Suggested next prompt

Phase 5 theorem-line later package の次段として、serialized-ready retained bridge を前提に、actual emitted attachment blob / file body をどこまで theorem-line bridge に寄せるかを narrow comparison してください。
