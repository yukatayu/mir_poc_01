# Report 0478 — phase5 archive retention layout threshold

- Date: 2026-04-10
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen における archive retention layout comparison の docs-first threshold
- Decision levels touched: L2

## 1. Objective

`retained_archive_payload_ref` の次段として、
archive retention layout を theorem-line retained bridge にどこまで寄せるかを narrow に比較し、
actual retained archive payload body family と bless-side / update-side row split を同じ reopen に混ぜない current first choice を固定する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/180-current-l2-theorem-line-archive-bless-update-policy-ready-retained-archive-payload-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `tasks.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. `retained archive payload comparison` 直後の theorem-line retained bridge を読み直し、layout family を独立 reopen として切れるかを確認した。
2. `specs/examples/181-current-l2-theorem-line-retained-archive-payload-ready-archive-retention-layout-threshold.md` を追加し、
   - terminal cut
   - `archive_retention_layout_ref` 1 本だけを足す案
   - layout family と actual retained payload body family をまとめて入れる案
   を比較した。
3. current judgment を
   - `archive_retention_layout_ref` だけを足す
   - actual retained archive payload body family は still 後段
   - bless-side / update-side row split も still 後段
   に固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に追随させた。

## 4. Files changed

- `specs/examples/181-current-l2-theorem-line-retained-archive-payload-ready-archive-retention-layout-threshold.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 5. Commands run and exact outputs

```bash
sed -n '1,240p' specs/examples/180-current-l2-theorem-line-archive-bless-update-policy-ready-retained-archive-payload-threshold.md
sed -n '1,220p' tasks.md
sed -n '1,220p' progress.md
python3 scripts/validate_docs.py
git diff --check
```

主要 output:

- `python3 scripts/validate_docs.py` → `Documentation scaffold looks complete.` / `Found 477 numbered report(s).`（review 前の local validation）
- `git diff --check` → 無出力

## 6. Evidence / findings

- current theorem-line retained bridge の next narrow step は、actual payload body family や bless-side / update-side split を入れることではなく、
  **`archive_retention_layout_ref` を 1 本だけ足す archive-retention-layout-ready retained bridge**
  に留めるのが最小だった。
- これにより、
  - archive retention layout family
  - actual retained archive payload body family
  - bless-side / update-side row split
  の 3 つを同じ reopen に混ぜずに済む。
- next promoted line は `actual retained archive payload body family comparison` に切り替えられる。

## 7. Changes in understanding

- archive retention layout comparison は、layout 本体を theorem-side bridge に actualize する task ではなく、
  **layout family への symbolic bridge を 1 本だけ足す threshold comparison**
  と読むのが自然だと分かった。
- その結果、actual retained payload body family は次段の later reopen へ安全に送れる。

## 8. Open questions

- actual retained archive payload body family の最小 shape
- actual bless-side row と update-side row をどこで split するか
- archive retention layout family を actual retained payload materialization family とどこで接続するか

## 9. Suggested next prompt

`actual retained archive payload body family comparison` を Phase 5 theorem-line の next later reopen として進め、`archive_retention_layout_ref` の次段で actual retained archive payload body family をどこまで retained bridge に寄せるかを docs-first で比較してください。
