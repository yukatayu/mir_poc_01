# Report 0476 — phase5 retained archive payload threshold

- Date: 2026-04-10
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen における retained archive payload comparison の docs-first threshold
- Decision levels touched: L2

## 1. Objective

`archive_bless_update_policy_ref` の次段として、
retained archive payload を theorem-line retained bridge にどこまで寄せるかを narrow に比較し、
archive retention layout と actual bless-side / update-side row split を同じ reopen に混ぜない current first choice を固定する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/178-current-l2-theorem-line-archive-member-family-ready-archive-member-body-compare-threshold.md`
- `specs/examples/179-current-l2-theorem-line-archive-member-body-compare-ready-archive-bless-update-policy-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `tasks.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

## 3. Actions taken

1. theorem-line retained bridge の直前判断 (`178` / `179`) を読み直し、retained archive payload を同じ reopen で扱う最小比較軸を抽出した。
2. `specs/examples/180-current-l2-theorem-line-archive-bless-update-policy-ready-retained-archive-payload-threshold.md` を追加し、
   - terminal cut
   - `retained_archive_payload_ref` 1 本だけを足す案
   - payload body / bless-side payload / update-side payload までまとめて入れる案
   を比較した。
3. current judgment を
   - `retained_archive_payload_ref` だけを足す
   - archive retention layout は still 後段
   - bless-side / update-side row split も still 後段
   に固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に追随させた。

## 4. Files changed

- `specs/examples/180-current-l2-theorem-line-archive-bless-update-policy-ready-retained-archive-payload-threshold.md`
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
df -h .
free -h
git status --short --branch
sed -n '1,220p' README.md
sed -n '1,240p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,220p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/examples/178-current-l2-theorem-line-archive-member-family-ready-archive-member-body-compare-threshold.md
sed -n '1,260p' specs/examples/179-current-l2-theorem-line-archive-member-body-compare-ready-archive-bless-update-policy-threshold.md
sed -n '1,260p' docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
sed -n '1,260p' tasks.md
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/validate_docs.py
git diff --check
```

主要 output:

- `df -h .` → `/dev/vda2 99G / 92G used / 2.5G avail / 98%`
- `free -h` → `960Mi total / 791Mi used / 76Mi free / 168Mi available`
- `git status --short --branch` → `## main...origin/main`
- `date` → `2026-04-10 10:13 JST`

## 6. Evidence / findings

- current theorem-line retained bridge の next narrow step は、retained payload 本体や retention layout を actual row へ割ることではなく、
  **`retained_archive_payload_ref` を 1 本だけ足す retained-archive-payload-ready retained bridge**
  に留めるのが最小だった。
- これにより、
  - retained archive payload family への return path
  - archive retention layout
  - actual bless-side / update-side row split
  の 3 つを同じ reopen に混ぜずに済む。
- next promoted line は `archive retention layout comparison` に切り替えられる。

## 7. Changes in understanding

- retained archive payload comparison は、payload 本体を theorem-side bridge に actualize する task ではなく、
  **retained payload family への symbolic bridge を 1 本だけ足す threshold comparison**
  と読むのが自然だと分かった。
- その結果、archive retention layout は Phase 5 later reopen の次段へ安全に送れる。

## 8. Open questions

- archive retention layout の最小 shape
- actual bless-side row と update-side row をどこで split するか
- actual retained archive payload body family をどこで theorem-line bridge と接続するか
- `proof_assistant_adapter` pressure が current second practical candidate のままでよい条件

## 9. Suggested next prompt

`archive retention layout comparison` を Phase 5 theorem-line の next later reopen として進め、`retained_archive_payload_ref` の次段で archive retention layout をどこまで retained bridge に寄せるかを docs-first で比較し、必要な mirror を同じ task で更新してください。
