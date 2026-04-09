# Report 0451 — phase5 emitted attachment body threshold

- Date: 2026-04-10 05:19 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen として、serialized-ready retained bridge の次段で `emitted_attachment_body_ref` を current first choice に上げてよいかを docs-first に比較する
- Decision levels touched: L2 examples / non-normative roadmap and repository memory wording

## 1. Objective

`specs/examples/169...` の次段として、

- actual emitted attachment body family
- actual emitted attachment blob / retained file materialization

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
- `specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md`
- `specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md`
- `specs/examples/168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md`
- `specs/examples/169-current-l2-theorem-line-transcript-body-ready-serialized-channel-body-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. theorem-line retained bridge の latest stop line を `169...` まで見直し、次の reopen unit を `emitted_attachment_body_ref` と actual emitted attachment blob / retained file materialization の二者分離として切り出した。
2. `specs/examples/170-current-l2-theorem-line-serialized-ready-emitted-attachment-body-threshold.md` を追加し、`emitted_attachment_body_ref` だけを current first choice に固定した。
3. mirror と phase snapshot を attachment-body-ready retained bridge まで更新した。
4. review record 雛形を追加した。

## 4. Files changed

- `specs/examples/170-current-l2-theorem-line-serialized-ready-emitted-attachment-body-threshold.md`
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
- `docs/reports/0451-phase5-emitted-attachment-body-threshold.md`
- `docs/reports/0452-review-phase5-emitted-attachment-body-threshold.md`

## 5. Commands run and exact outputs

```text
$ rg -n "attachment blob|file body|blob body|attachment body|serialized_channel_body_ref|emitted_artifact_ref|proof_notebook_bridge" specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md specs/examples/166-current-l2-theorem-line-transcript-ready-materialized-runtime-handoff-threshold.md specs/examples/168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md specs/examples/169-current-l2-theorem-line-transcript-body-ready-serialized-channel-body-threshold.md docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md tasks.md progress.md
specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md:174:`emitted_artifact_ref` は、
specs/examples/168-current-l2-theorem-line-payload-ready-concrete-transcript-threshold.md:174:- emitted notebook attachment blob
specs/examples/169-current-l2-theorem-line-transcript-body-ready-serialized-channel-body-threshold.md:169:## なぜ emitted attachment blob / file body をまだ入れないか
docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md:209:- actual emitted attachment blob / file body をどこまで theorem-line bridge に寄せるか
tasks.md:27:| 2 | Phase 5 checkpoint 後半 / later reopen 候補 | actual emitted attachment blob / file body comparison | ...
```

## 6. Evidence / findings

- `serialized_channel_body_ref` の次段でも、actual blob / file materialization を一気に入れず、emitted attachment body family だけを symbolic ref で切るのが最小である。
- `emitted_attachment_body_ref` を入れても、actual emitted attachment blob payload / retained file materialization は still theorem-line bridge の外に残せる。
- actual materialization を同時に入れると、attachment body family と actual blob/file materialization の境界が premature に混ざりやすい。
- `plan/ 更新済み`
- `progress.md 更新済み`
- `tasks.md 更新済み`

## 7. Changes in understanding

- Phase 5 later reopen の next minimal unit は「emitted attachment body first, actual blob/file materialization second」である。
- `serialized_channel_body_ref` と actual materialization の間にも narrow bridge があり、ここを 1 task で切る価値がある。

## 8. Open questions

- `emitted_attachment_body_ref` を transport artifact family に寄せるか、retained artifact family に寄せるか。
- actual emitted blob payload と retained file body / archive materialization を同じ reopen で扱うか。
- attachment body family を `proof_assistant_adapter` pressure より先に保持する条件がどこまで続くか。

## 9. Suggested next prompt

Phase 5 theorem-line later package の次段として、attachment-body-ready retained bridge を前提に、actual emitted attachment blob / file materialization をどこまで theorem-line bridge に寄せるかを narrow comparison してください。
