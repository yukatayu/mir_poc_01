# Report 0463 — phase5 archive bundle threshold

- Date: 2026-04-10 07:40 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later package の次段として、archive-body-ready retained bridge に actual archive bundle family をどこまで足すかを docs-first で比較する
- Decision levels touched: L2 docs-only boundary refinement

## 1. Objective

`specs/examples/174-current-l2-theorem-line-archive-materialization-ready-archive-body-bundle-threshold.md`
までで `archive_body_ref` を current first choice に置いたあと、
actual archive bundle family をどこまで theorem-line bridge に寄せるべきかを narrow に比較し、
next later reopen をどこに置くかを固定する。

## 2. Scope and assumptions

- `proof_notebook` first bridge だけを扱う。
- current theorem-line later package の bridge family を前提にする。
- actual archive bundle manifest / member family、archive retention layout、bless/update policy は今回 fixed しない。

## 3. Documents consulted

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
- `specs/examples/173-current-l2-theorem-line-retained-file-body-ready-archive-materialization-threshold.md`
- `specs/examples/174-current-l2-theorem-line-archive-materialization-ready-archive-body-bundle-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 4. Actions taken

1. 174 package の review finding を反映し、archive-body-ready package の next reopen wording を actual archive bundle comparison に揃えた。
2. actual archive bundle family の current comparison を 3 案で整理した。
3. `archive_bundle_ref` のみを bridge に足し、manifest / member family は still 後段に残す current first choice を `specs/examples/175...` に固定した。
4. top-level mirror、Phase 5 summary、task map、traceability を current snapshot に更新した。

## 5. Files changed

- `Documentation.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0462-review-phase5-archive-body-bundle-threshold.md`
- `docs/reports/0463-phase5-archive-bundle-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/174-current-l2-theorem-line-archive-materialization-ready-archive-body-bundle-threshold.md`
- `specs/examples/175-current-l2-theorem-line-archive-body-ready-archive-bundle-threshold.md`
- `tasks.md`

## 6. Commands run

```text
$ date '+%Y-%m-%d %H:%M:%S %Z'
2026-04-10 07:40:43 JST

$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   92G  2.6G  98% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       743Mi        87Mi       1.1Mi       284Mi       216Mi
Swap:           19Gi       2.2Gi        17Gi
```

## 7. Evidence / outputs / test results

- `specs/examples/175...` で current first choice を `archive_bundle_ref` only に固定した。
- next later reopen は actual archive bundle manifest / member family comparison に更新した。
- 174 package の stale mirror も同 task で補正した。

## 8. What changed in understanding

- actual archive bundle family までは theorem-line bridge に symbolic ref で寄せても、actual archive bundle manifest / member family は still 後段に残せる。
- archive bundle family 自体と manifest/member family を別 reopen に分ける方が narrow ratchet を維持しやすい。

## 9. Open questions

- actual archive bundle manifest / member family の最小 shape
- archive bundle manifest と archive bundle member family を同じ reopen で扱うかどうか
- archive retention layout / bless/update policy への接続

## 10. Suggested next prompt

Phase 5 theorem-line later package の current first choice を前提に、actual archive bundle manifest / member family をどこまで theorem-line bridge に寄せるかを narrow comparison してください。
