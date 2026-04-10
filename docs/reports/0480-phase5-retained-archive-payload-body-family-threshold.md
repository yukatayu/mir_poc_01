# Report 0480 — phase5 retained archive payload body family threshold

- Date: 2026-04-10
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen における retained archive payload body family comparison の docs-first threshold
- Decision levels touched: L2

## 1. Objective

`archive_retention_layout_ref` の次段として、
actual retained archive payload body family を theorem-line retained bridge にどこまで寄せるかを narrow に比較し、
retained payload materialization family と bless-side / update-side row split を同じ reopen に混ぜない current first choice を固定する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/181-current-l2-theorem-line-retained-archive-payload-ready-archive-retention-layout-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `tasks.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. `archive retention layout comparison` 直後の theorem-line retained bridge を読み直し、payload body family を独立 reopen として切れるかを確認した。
2. `specs/examples/182-current-l2-theorem-line-archive-retention-layout-ready-retained-archive-payload-body-family-threshold.md` を追加し、
   - layout family で止める案
   - `retained_archive_payload_body_family_ref` 1 本だけを足す案
   - payload body family と bless-side / update-side row split をまとめて入れる案
   を比較した。
3. current judgment を
   - `retained_archive_payload_body_family_ref` だけを足す
   - retained payload materialization family は still 後段
   - bless-side / update-side row split も still 後段
   に固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に追随させた。

## 4. Files changed

- `specs/examples/182-current-l2-theorem-line-archive-retention-layout-ready-retained-archive-payload-body-family-threshold.md`
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
git status --short --branch
sed -n '1,260p' specs/examples/182-current-l2-theorem-line-archive-retention-layout-ready-retained-archive-payload-body-family-threshold.md
rg -n "126\\.\\.\\.181|actual retained archive payload body family comparison|retained payload materialization family comparison" Documentation.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/13-heavy-future-workstreams.md plan/17-research-phases-and-autonomy-gates.md docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md progress.md tasks.md plan/90-source-traceability.md
date '+%Y-%m-%d %H:%M JST'
python3 scripts/validate_docs.py
git diff --check
```

主要 output:

- `git status --short --branch` → `Documentation.md`、`plan/11`、`plan/12`、`specs/00-document-map.md`、新規 `specs/examples/182...` の変更が作業開始時点で見えていた。
- `date '+%Y-%m-%d %H:%M JST'` → `2026-04-10 11:16 JST`
- `python3 scripts/validate_docs.py` → `Documentation scaffold looks complete.` / `Found 481 numbered report(s).`
- `git diff --check` → 無出力

## 6. Evidence / findings

- current theorem-line retained bridge の next narrow step は、
  **`retained_archive_payload_body_family_ref` を 1 本だけ足す retained-archive-payload-body-family-ready retained bridge**
  に留めるのが最小だった。
- これにより、
  - archive retention layout family
  - actual retained archive payload body family
  - retained payload materialization family
  - bless-side / update-side row split
  の 4 つを同じ reopen に混ぜずに済む。
- next promoted line は `retained payload materialization family comparison` に切り替えられる。

## 7. Changes in understanding

- actual retained archive payload body family comparison は、payload body family の actual materialization を theorem-side bridge に持ち込む task ではなく、
  **payload body family への symbolic bridge を 1 本だけ足す threshold comparison**
  と読むのが自然だと分かった。
- その結果、retained payload materialization family は次段の later reopen へ安全に送れる。

## 8. Open questions

- retained payload materialization family の最小 shape
- actual bless-side row と update-side row をどこで split するか
- actual retained archive payload body family を body materialization detail とどこで接続するか

## 9. Suggested next prompt

`retained payload materialization family comparison` を Phase 5 theorem-line の next later reopen として進め、`retained_archive_payload_body_family_ref` の次段で retained payload materialization family をどこまで retained bridge に寄せるかを docs-first で比較してください。
