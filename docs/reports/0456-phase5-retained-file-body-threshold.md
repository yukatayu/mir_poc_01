# Report 0456 — phase5 retained file body threshold

- Date: 2026-04-10 06:31 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen として、attachment-blob-ready retained bridge の次段で `retained_file_body_ref` を current first choice に上げてよいかを docs-first に比較する
- Decision levels touched: L2 examples / non-normative roadmap and repository memory wording

## 1. Objective

`specs/examples/171...` の次段として、

- actual retained file body family
- actual archive materialization

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
- `specs/examples/170-current-l2-theorem-line-serialized-ready-emitted-attachment-body-threshold.md`
- `specs/examples/171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold.md`
- `specs/examples/172-current-l2-theorem-line-attachment-blob-ready-retained-file-body-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. theorem-line retained bridge の latest stop line を `171...` まで見直し、次の reopen unit を `retained_file_body_ref` と actual archive materialization の二者分離として切り出した。
2. `specs/examples/172-current-l2-theorem-line-attachment-blob-ready-retained-file-body-threshold.md` を追加し、`retained_file_body_ref` だけを current first choice に固定した。
3. mirror と phase snapshot を retained-file-body-ready retained bridge まで更新した。
4. review record 雛形を追加した。

## 4. Files changed

- `specs/examples/172-current-l2-theorem-line-attachment-blob-ready-retained-file-body-threshold.md`
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
- `docs/reports/0456-phase5-retained-file-body-threshold.md`
- `docs/reports/0457-review-phase5-retained-file-body-threshold.md`
- `docs/reports/0458-review-phase5-retained-file-body-package-consistency.md`

## 5. Commands run and exact outputs

```text
$ rg -n "171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold|172-current-l2-theorem-line-attachment-blob-ready-retained-file-body-threshold|emitted_attachment_blob_ref|retained_file_body_ref|actual archive materialization" Documentation.md specs/00-document-map.md docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/13-heavy-future-workstreams.md plan/17-research-phases-and-autonomy-gates.md progress.md tasks.md specs/examples/172-current-l2-theorem-line-attachment-blob-ready-retained-file-body-threshold.md
[matches inspected locally]

$ date '+%Y-%m-%d %H:%M %Z'
2026-04-10 06:31 JST
```

## 6. Evidence / findings

- `retained_file_body_ref` の次段でも、actual archive materialization を一気に入れず retained file body family だけを symbolic ref で切るのが最小である。
- `retained_file_body_ref` を入れても、actual archive materialization は still theorem-line bridge の外に残せる。
- archive materialization を同時に入れると、retained file body family と bless / retention / archive policy family の境界が premature に混ざりやすい。
- reviewer は normative theorem-line ratchet 自体には semantic finding を返さず、mirror drift と review traceability のみを指摘した。`plan/11`、`progress.md`、`docs/reports/0457...` は closeout 内で補正済みである。
- `plan/ 更新済み`
- `progress.md 更新済み`
- `tasks.md 更新済み`

## 7. Changes in understanding

- Phase 5 later reopen の next minimal unit は「retained file body first, archive materialization second」である。
- `emitted_attachment_blob_ref` と archive materialization の間にも narrow bridge があり、ここを 1 task で切る価値がある。

## 8. Open questions

- `retained_file_body_ref` を retained artifact family に寄せ続けるか、archive materialization line の first field とみなすか。
- actual archive materialization を symbolic `archive_materialization_ref` に留めるか、archive body / bundle family へさらに分けるか。
- retained file body family を `proof_assistant_adapter` pressure より先に保持する条件がどこまで続くか。

## 9. Suggested next prompt

Phase 5 theorem-line later package の次段として、retained-file-body-ready retained bridge を前提に、actual archive materialization をどこまで theorem-line bridge に寄せるかを narrow comparison してください。
