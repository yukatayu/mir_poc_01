# 0510 — phase5 external-contract-facing handoff row threshold

## Objective

`specs/examples/197-current-l2-theorem-line-boundary-specific-handoff-artifact-row-ready-external-contract-facing-handoff-row-threshold.md`
を追加し、Phase 5 theorem-line の current promoted line として

- boundary-specific handoff artifact row の次段で external-contract-facing handoff row をどこまで retained bridge に近づけるか
- actual external contract をどこまで still 後段に残すか

を narrow に比較して、current first choice を固定する。

## Scope and assumptions

- `proof_notebook` first bridge だけを扱う。
- actual external contract はまだ扱わない。
- theorem-line retained bridge を actual external contract に既成事実化しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md`
- `specs/examples/196-current-l2-theorem-line-actual-handoff-pair-shape-ready-boundary-specific-handoff-artifact-row-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. `specs/examples/197-current-l2-theorem-line-boundary-specific-handoff-artifact-row-ready-external-contract-facing-handoff-row-threshold.md` を追加し、boundary-specific handoff artifact row の次段として
   - terminal cut
   - minimal external-facing handoff row
   - actual external contract 同時導入
   の 3 案を比較した。
2. current first choice を、`retained_payload_body_materialization_external_handoff_row` だけを足す retained bridge として固定した。
3. mirror を `126...197` snapshot と `external-contract-facing-handoff-row-ready actual external contract comparison` に更新した。

## Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/197-current-l2-theorem-line-boundary-specific-handoff-artifact-row-ready-external-contract-facing-handoff-row-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0510-phase5-external-contract-facing-handoff-row-threshold.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
df -h .
free -h
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- resource preflight
  - `2026-04-10 16:50 JST`
  - `df -h .` → `/dev/vda2 99G total / 92G used / 2.9G avail / 98%`
  - `free -h` → `Mem 960Mi total / 781Mi used / 78Mi free / 178Mi available`, `Swap 19Gi total / 2.2Gi used / 17Gi free`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 510 numbered report(s).`
- `git diff --check`
  - 無出力

## What changed in understanding

- boundary-specific handoff artifact row の次段では、external-contract-facing handoff row 自体は retained bridge に narrow に actualize してよい。
- ただし actual external contract は同じ reopen に混ぜず、still 後段に残す方が line を保ちやすい。
- next promoted line は `external-contract-facing-handoff-row-ready actual external contract comparison` と読むのが自然になった。

## Open questions

- actual external contract をどの field で切るか
- external-facing handoff row を retained bridge のまま維持するか actual external contract へ actualize するか
- actual external contract へ actualize する concrete pressure を何と読むか

## Suggested next prompt

`specs/examples/197-current-l2-theorem-line-boundary-specific-handoff-artifact-row-ready-external-contract-facing-handoff-row-threshold.md` を前提に、external-contract-facing-handoff-row-ready actual external contract comparison を 3 案で比較し、current retained bridge に actual external contract をどこまで近づけるかを docs-first で整理してください。
