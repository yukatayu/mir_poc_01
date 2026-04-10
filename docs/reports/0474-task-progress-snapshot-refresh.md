# 0474 — task / progress snapshot refresh

## Objective

- `tasks.md` と `progress.md` を current snapshot として全面見直しし、冗長な列挙を圧縮する。
- user から提示された blocker 方針を current recommendation に反映する。
- 必要なら maintenance rule mirror も補正し、今後の snapshot 運用を読みやすく保つ。

## Scope and assumptions

- 規範判断の正本は `specs/` と `plan/` に残し、この task では snapshot 文書と maintenance rule の整理だけを扱う。
- shared-space blocker については、user が示した方針を **current recommendation** として反映するが、未決事項を決定済みと装わない。
- Phase 5 theorem-line package の current promoted line は、直前までの判断どおり `retained archive payload comparison` と読む。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `docs/reports/0475-review-docs-only-snapshot-rules.md`
- `AGENTS.md`

## Actions taken

1. `tasks.md` を全面書き直し、次を current snapshot として再構成した。
   - current reading
   - 次に自走で進める順番と rough estimate
   - 自走 task package
   - current blocker / open question
2. `tasks.md` の blocker recommendation に、user が示した current preference を反映した。
   - activation rule: `authority-ack` baseline + overlay policy option
   - authority placement: `single room authority` baseline + replicated / relaxed later option
   - fairness / RNG: `authority_rng` baseline + `delegated_rng_service` / `auditable_authority_witness` の別軸比較
   - control-plane carrier: membership default + lightweight reopen cut
   - parser / checker boundary: companion notation 維持から narrow public 化へ
3. `progress.md` を全面書き直し、次を短い snapshot に整理した。
   - current summary
   - phase table
   - mainline / readiness
   - chapter-level rough progress
   - next recommended task
   - recent work log
4. `AGENTS.md` と `plan/91-maintenance-rules.md` に、snapshot 文書の recent-log / concise-summary 運用を追記した。
5. reviewer finding に合わせて `plan/11-roadmap-near-term.md` と `plan/17-research-phases-and-autonomy-gates.md` を current promoted line に追随させた。

## Files changed

- `AGENTS.md`
- `plan/91-maintenance-rules.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `tasks.md`
- `progress.md`
- `docs/reports/0474-task-progress-snapshot-refresh.md`
- `docs/reports/0475-review-docs-only-snapshot-rules.md`

## Commands run

```text
df -h .
free -h
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,320p' tasks.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,220p' specs/09-invariants-and-constraints.md
sed -n '1,260p' plan/11-roadmap-near-term.md
sed -n '1,260p' plan/12-open-problems-and-risks.md
sed -n '1,260p' plan/16-shared-space-membership-and-example-boundary.md
sed -n '1,260p' plan/17-research-phases-and-autonomy-gates.md
sed -n '1,220p' plan/90-source-traceability.md
sed -n '1,260p' plan/91-maintenance-rules.md
rg -n "^## |^### |^#### " tasks.md
rg -n "^## |^### " progress.md
date '+%Y-%m-%d %H:%M %Z'
git status --short --branch
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## Evidence / outputs / test results

- resource snapshot at task start:
  - `df -h .` → `/dev/vda2` 99G total / 92G used / 2.5G avail / 98%
  - `free -h` → 960Mi memory / 214Mi available / swap 19Gi
- task-start dirty state:
  - `git status --short --branch` → `## main...origin/main` with modifications only after this task began
- snapshot rewrite evidence:
  - `tasks.md` は current promoted line / checkpoint / blocker recommendation が一目で読める構成に圧縮した
  - `progress.md` は current summary / phase / rough progress / recent log の snapshot に整理した
- validation:
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## What changed in understanding

- `tasks.md` は theorem-line chain や historical checkpoint を exhaustively 再列挙するより、**current checkpoint / current promoted line / next reopen point** を前に出す方が current repo の運用に合う。
- `progress.md` も archive ではなく snapshot なので、**recent log** と短い current reading を優先する方が drift を抑えやすい。
- user の blocker 見解は、現行の plan memory と矛盾せず、そのまま current recommendation に寄せられる。

## Open questions

- `retained archive payload comparison` をどこまで theorem-line retained bridge に寄せるかは未決。
- shared-space final catalog は引き続き OPEN であり、working subset を final catalog と装わない。
- `progress.md` の recent log を何件程度残すのが最も運用しやすいかは、今後の task cadence を見ながら調整余地がある。

## plan / progress / tasks update

- `plan/` は `plan/91-maintenance-rules.md` を更新した。
- `progress.md` は更新した。
- `tasks.md` は更新した。
- reviewer は completion まで待ち、finding 2 件を反映した。

## Suggested next prompt

`tasks.md` の current promoted line どおり、`retained archive payload comparison` を Phase 5 theorem-line later reopen として docs-first に比較し、必要な mirror と snapshot を同じ task で更新してください。
