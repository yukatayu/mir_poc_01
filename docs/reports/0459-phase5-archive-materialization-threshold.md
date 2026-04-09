# Report 0459 — phase5 archive materialization threshold

- Date: 2026-04-10 06:48 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen として、retained-file-body-ready retained bridge の次段で `archive_materialization_ref` を current first choice に上げてよいかを docs-first に比較する
- Decision levels touched: L2 examples / non-normative roadmap and repository memory wording

## 1. Objective

`specs/examples/172...` の次段として、

- actual archive materialization family
- actual archive body / bundle family

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
- `specs/examples/171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold.md`
- `specs/examples/172-current-l2-theorem-line-attachment-blob-ready-retained-file-body-threshold.md`
- `specs/examples/173-current-l2-theorem-line-retained-file-body-ready-archive-materialization-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. theorem-line retained bridge の latest stop line を `172...` まで見直し、次の reopen unit を `archive_materialization_ref` と actual archive body / bundle family の二者分離として切り出した。
2. `specs/examples/173-current-l2-theorem-line-retained-file-body-ready-archive-materialization-threshold.md` を追加し、`archive_materialization_ref` だけを current first choice に固定した。
3. mirror と phase snapshot を archive-materialization-ready retained bridge まで更新した。
4. review record 雛形を追加した。

## 4. Files changed

- `specs/examples/173-current-l2-theorem-line-retained-file-body-ready-archive-materialization-threshold.md`
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
- `docs/reports/0459-phase5-archive-materialization-threshold.md`
- `docs/reports/0460-review-phase5-archive-materialization-threshold.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-10 06:48 JST

$ date '+%Y-%m-%dT%H:%M'
2026-04-10T06:48

$ sed -n '1,260p' specs/examples/172-current-l2-theorem-line-attachment-blob-ready-retained-file-body-threshold.md
[existing theorem-line stop line inspected]
```

## 6. Evidence / findings

- `archive_materialization_ref` の次段でも、actual archive body / bundle family を一気に入れず archive materialization family だけを symbolic ref で切るのが最小である。
- `archive_materialization_ref` を入れても、actual archive body / bundle family は still theorem-line bridge の外に残せる。
- archive materialization と archive body / bundle family を同時に入れると、archive family と bless / retention / archive policy family の境界が premature に混ざりやすい。
- `plan/ 更新済み`
- `progress.md 更新済み`
- `tasks.md 更新済み`

## 7. Changes in understanding

- Phase 5 later reopen の next minimal unit は「archive materialization first, archive body / bundle second」である。
- `retained_file_body_ref` と actual archive body / bundle family の間にも narrow bridge があり、ここを 1 task で切る価値がある。

## 8. Open questions

- `archive_materialization_ref` を archive family の stable ceiling とみなすか、actual archive body / bundle family の first bridge field とみなすか。
- actual archive body / bundle family を symbolic ref 1 本に留めるか、archive body と archive bundle をさらに分けるか。
- `proof_assistant_adapter` pressure を archive body / bundle comparison より先に practical reopen へ上げる条件がどこまで続くか。

## 9. Suggested next prompt

Phase 5 theorem-line later package の次段として、archive-materialization-ready retained bridge を前提に、actual archive body / bundle family をどこまで theorem-line bridge に寄せるかを narrow comparison してください。
