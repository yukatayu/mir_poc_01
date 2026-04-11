# Report 0564 — Phase 5 handoff witness row detail package

- Date: 2026-04-11 12:02 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen として `specs/examples/237...238` を追加し、authority transition line を actual handoff witness row detail まで narrow に actualize する。mirror sweep と current promoted line 更新を同じ package に含める。
- Decision levels touched: L2

## 1. Objective

- `retained_payload_body_materialization_theorem_export_witness_aware_handoff_family` の次段として、何を theorem-side retained bridge に足してよいかを narrow に比較する。
- actual handoff witness row detail を source-backed に切り、replay attachment / payload / carrier detail は still later に残す。
- `Documentation.md`、`plan/`、`progress.md`、`tasks.md`、research abstract の current promoted line を `238` snapshot に揃える。

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
- `specs/examples/235-current-l2-theorem-line-minimal-authority-handoff-epoch-ref-ready-witness-aware-handoff-family-comparison.md`
- `specs/examples/236-current-l2-theorem-line-witness-aware-handoff-family-ready-minimal-witness-aware-handoff-family-threshold.md`
- `specs/examples/237-current-l2-theorem-line-minimal-witness-aware-handoff-family-ready-handoff-witness-row-detail-comparison.md`
- `specs/examples/238-current-l2-theorem-line-handoff-witness-row-detail-ready-minimal-handoff-witness-row-detail-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 3. Actions taken

1. `specs/examples/237` で、symbolic witness-aware handoff family の次段では replay / payload / carrier detail より先に **actual handoff witness row detail** を切るのが自然だと比較した。
2. `specs/examples/238` で、handoff witness row detail の current first choice を `witness_aware_handoff_family_ref + witness_actor_ref + handoff_witness_kind` の minimal row に固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、Phase 5 research abstract を `specs/examples/126...238` と next promoted line に追随させた。
4. reviewer を 1 回だけ起動し、completion を長めに待つ。

## 4. Files changed

- `specs/examples/237-current-l2-theorem-line-minimal-witness-aware-handoff-family-ready-handoff-witness-row-detail-comparison.md`
- `specs/examples/238-current-l2-theorem-line-handoff-witness-row-detail-ready-minimal-handoff-witness-row-detail-threshold.md`
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
- `docs/reports/0564-phase5-handoff-witness-row-detail-package.md`

## 5. Commands run and exact outputs

### 5.1 Timestamp snapshot

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-11 12:02 JST
```

### 5.2 Fresh validation

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 564 numbered report(s).
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

- symbolic witness-aware handoff family の次段は、replay attachment / payload / carrier detail ではなく **actual handoff witness row detail** を切るのが自然である。
- minimal row は `witness_aware_handoff_family_ref + witness_actor_ref + handoff_witness_kind` で十分であり、replay attachment / payload / carrier detail は still later に残せる。
- したがって current theorem-side retained bridge の current first choice は、`witness_aware_handoff_family` → `handoff_witness_row` の順で narrow に伸ばす line である。

## 7. Changes in understanding

- authority transition line は、symbolic witness-aware handoff family の次に replay attachment を入れるより、**actual handoff witness row detail** を先に切る方が ratchet として安定する。
- current promoted line は `minimal-handoff-witness-row-detail-ready replay-attachment-ref comparison` と読むのが自然になった。

## 8. Open questions

- replay attachment ref を first reopen に置くべきか
- handoff payload / carrier detail を replay line より後段に残す順序でよいか
- `handoff_witness_kind` の canonical token set をどこまで current phase で固定してよいか
- reviewer は completion を返し、`Documentation.md` の section numbering drift と本 report の `PENDING` placeholder 除去を指摘した。両方とも反映済みで、semantic drift の指摘はなかった
- plan/ 更新不要か: **不要ではない**。今回の task で `plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90` を更新した
- progress.md 更新不要か: **不要ではない**。今回の task で `progress.md` を更新した
- tasks.md 更新不要か: **不要ではない**。今回の task で `tasks.md` を更新した

## 9. Suggested next prompt

`minimal-handoff-witness-row-detail-ready replay-attachment-ref comparison` を docs-first で進め、actual handoff witness row detail の次段として replay attachment ref をどこまで theorem-line retained bridge に寄せてよいかを比較してください。
