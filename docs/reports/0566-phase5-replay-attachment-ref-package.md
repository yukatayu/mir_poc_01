# Report 0566 — Phase 5 replay attachment ref package

- Date: 2026-04-11 12:23 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen として `specs/examples/239...240` を追加し、authority handoff line を symbolic replay attachment ref まで narrow に actualize する。mirror sweep と current promoted line 更新を同じ package に含める。
- Decision levels touched: L2

## 1. Objective

- `retained_payload_body_materialization_theorem_export_handoff_witness_row` の次段として、何を theorem-side retained bridge に足してよいかを narrow に比較する。
- symbolic replay attachment ref を source-backed に切り、handoff payload / carrier detail は still later に残す。
- `Documentation.md`、`plan/`、`progress.md`、`tasks.md`、research abstract の current promoted line を `240` snapshot に揃える。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/237-current-l2-theorem-line-minimal-witness-aware-handoff-family-ready-handoff-witness-row-detail-comparison.md`
- `specs/examples/238-current-l2-theorem-line-handoff-witness-row-detail-ready-minimal-handoff-witness-row-detail-threshold.md`
- `specs/examples/239-current-l2-theorem-line-minimal-handoff-witness-row-detail-ready-replay-attachment-ref-comparison.md`
- `specs/examples/240-current-l2-theorem-line-replay-attachment-ref-ready-minimal-replay-attachment-ref-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 3. Actions taken

1. `specs/examples/239` で、actual handoff witness row detail の次段では handoff payload / carrier detail より先に **symbolic replay attachment ref** を切るのが自然だと比較した。
2. `specs/examples/240` で、replay attachment ref の current first choice を `handoff_witness_row_ref + replay_attachment_ref` の minimal row に固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、Phase 5 research abstract を `specs/examples/126...240` と next promoted line に追随させた。
4. reviewer を 1 回だけ起動し、completion を長めに待った。

## 4. Files changed

- `specs/examples/239-current-l2-theorem-line-minimal-handoff-witness-row-detail-ready-replay-attachment-ref-comparison.md`
- `specs/examples/240-current-l2-theorem-line-replay-attachment-ref-ready-minimal-replay-attachment-ref-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0566-phase5-replay-attachment-ref-package.md`
- `docs/reports/0567-review-phase5-replay-attachment-ref-package.md`

## 5. Commands run and exact outputs

### 5.1 Timestamp snapshot

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-11 12:23 JST
```

### 5.2 Fresh validation

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 566 numbered report(s).
```

```text
$ git diff --check
<no output>
```

### 5.3 Review

```text
$ reviewer subagent
completed
```

## 6. Evidence / findings

- actual handoff witness row の次段は、handoff payload / carrier detail ではなく **symbolic replay attachment ref** を切るのが自然である。
- minimal row は `handoff_witness_row_ref + replay_attachment_ref` で十分であり、payload / carrier detail は still later に残せる。
- したがって current theorem-side retained bridge の current first choice は、`handoff_witness_row` → `handoff_replay_attachment_ref` の順で narrow に伸ばす line である。

## 7. Changes in understanding

- authority handoff line は、actual handoff witness row の次に handoff payload を入れるより、**symbolic replay attachment ref** を先に切る方が ratchet として安定する。
- current promoted line は `minimal-replay-attachment-ref-ready handoff-payload-ref comparison` と読むのが自然になった。

## 8. Open questions

- handoff payload ref を first reopen に置くべきか
- handoff carrier detail を payload line より後段に残す順序でよいか
- `replay_attachment_ref` の canonical token set をどこまで current phase で固定してよいか
- plan/ 更新不要か: **不要ではない**。今回の task で `plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90` を更新した
- progress.md 更新不要か: **不要ではない**。今回の task で `progress.md` を更新した
- tasks.md 更新不要か: **不要ではない**。今回の task で `tasks.md` を更新した

## 9. Suggested next prompt

`minimal-replay-attachment-ref-ready handoff-payload-ref comparison` を docs-first で進め、symbolic replay attachment ref の次段として handoff payload ref をどこまで theorem-line retained bridge に寄せてよいかを比較してください。
