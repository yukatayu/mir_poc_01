# Report 0453 — phase5 emitted attachment blob threshold

- Date: 2026-04-10 05:53 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen として、attachment-body-ready retained bridge の次段で `emitted_attachment_blob_ref` を current first choice に上げてよいかを docs-first に比較する
- Decision levels touched: L2 examples / non-normative roadmap and repository memory wording

## 1. Objective

`specs/examples/170...` の次段として、

- actual emitted attachment blob family
- actual retained file body / archive materialization

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
- `specs/examples/169-current-l2-theorem-line-transcript-body-ready-serialized-channel-body-threshold.md`
- `specs/examples/170-current-l2-theorem-line-serialized-ready-emitted-attachment-body-threshold.md`
- `specs/examples/171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. theorem-line retained bridge の latest stop line を `170...` まで見直し、次の reopen unit を `emitted_attachment_blob_ref` と actual retained file materialization の二者分離として切り出した。
2. `specs/examples/171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold.md` を追加し、`emitted_attachment_blob_ref` だけを current first choice に固定した。
3. mirror と phase snapshot を attachment-blob-ready retained bridge まで更新した。
4. review record 雛形を追加した。

## 4. Files changed

- `specs/examples/171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold.md`
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
- `docs/reports/0453-phase5-emitted-attachment-blob-threshold.md`
- `docs/reports/0454-review-phase5-emitted-attachment-blob-threshold.md`

## 5. Commands run and exact outputs

```text
$ rg -n "170-current-l2-theorem-line-serialized-ready-emitted-attachment-body-threshold|171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold|emitted_attachment_body_ref|emitted_attachment_blob_ref|actual retained file body / archive materialization" Documentation.md specs/00-document-map.md docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/13-heavy-future-workstreams.md plan/17-research-phases-and-autonomy-gates.md progress.md tasks.md specs/examples/171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold.md
[matches inspected locally]
```

## 6. Evidence / findings

- `emitted_attachment_body_ref` の次段でも、actual retained file materialization を一気に入れず、emitted attachment blob family だけを symbolic ref で切るのが最小である。
- `emitted_attachment_blob_ref` を入れても、actual retained file body / archive materialization は still theorem-line bridge の外に残せる。
- retained materialization を同時に入れると、emitted blob family と retention/archive family の境界が premature に混ざりやすい。
- `plan/ 更新済み`
- `progress.md 更新済み`
- `tasks.md 更新済み`

## 7. Changes in understanding

- Phase 5 later reopen の next minimal unit は「emitted attachment blob first, retained file materialization second」である。
- `emitted_attachment_body_ref` と retained file materialization の間にも narrow bridge があり、ここを 1 task で切る価値がある。

## 8. Open questions

- `emitted_attachment_blob_ref` を transport artifact family に寄せるか、emitted-artifact family に寄せるか。
- actual retained file body と archive materialization を同じ reopen で扱うか。
- attachment blob family を `proof_assistant_adapter` pressure より先に保持する条件がどこまで続くか。

## 9. Suggested next prompt

Phase 5 theorem-line later package の次段として、attachment-blob-ready retained bridge を前提に、actual retained file body / archive materialization をどこまで theorem-line bridge に寄せるかを narrow comparison してください。
