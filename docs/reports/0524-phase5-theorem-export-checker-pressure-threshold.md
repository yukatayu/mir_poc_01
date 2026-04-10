# 0524 — phase5 theorem-export-checker pressure threshold

## Objective

`specs/examples/204-current-l2-theorem-line-proof-assistant-adapter-contract-ready-theorem-export-checker-pressure-threshold.md`
を追加し、Phase 5 theorem-line の current promoted line として

- actual `proof_assistant_adapter` contract の次段で `theorem_export_checker` pressure をどこまで retained bridge に近づけるか
- actual checker-facing contract をどこまで still 後段に残すか

を narrow に比較して、current first choice を固定する。

## Scope and assumptions

- current `proof_notebook` first bridge を起点にする。
- first machine-facing checker candidate は `theorem_export_checker` に限る。
- actual checker-facing contract と exported checker payload はまだ扱わない。

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
- `specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md`
- `specs/examples/203-current-l2-theorem-line-second-consumer-pressure-ready-proof-assistant-adapter-contract-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. `specs/examples/204-current-l2-theorem-line-proof-assistant-adapter-contract-ready-theorem-export-checker-pressure-threshold.md` を追加し、proof-assistant-adapter-contract-ready retained bridge の次段として
   - terminal cut
   - symbolic `theorem_export_checker` pressure marker
   - actual checker-facing contract 同時導入
   の 3 案を比較した。
2. current first choice を、`retained_payload_body_materialization_theorem_export_checker_pressure` だけを足す retained bridge として固定した。
3. mirror を `126...204` snapshot と `theorem-export-checker-pressure-ready checker-facing contract comparison` に更新した。

## Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/204-current-l2-theorem-line-proof-assistant-adapter-contract-ready-theorem-export-checker-pressure-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0524-phase5-theorem-export-checker-pressure-threshold.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- timestamp snapshot
  - `2026-04-10 18:43 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 524 numbered report(s).`
- `git diff --check`
  - no output

## What changed in understanding

- actual `proof_assistant_adapter` contract の次段では、`theorem_export_checker` pressure 自体は retained bridge に narrow に actualize してよい。
- ただし actual checker-facing contract は同じ reopen に混ぜず、still 後段に残す方が line を保ちやすい。
- next promoted line は `theorem-export-checker-pressure-ready checker-facing contract comparison` と読むのが自然になった。

## Open questions

- actual checker-facing contract をどの field / row / consumer split で切るか
- `theorem_export_checker` pressure を retained bridge のまま維持するか checker-facing contract へ actualize するか
- exported checker payload pressure を later candidate のまま維持する concrete threshold を何と読むか

## Suggested next prompt

`specs/examples/204-current-l2-theorem-line-proof-assistant-adapter-contract-ready-theorem-export-checker-pressure-threshold.md` を前提に、theorem-export-checker-pressure-ready checker-facing contract comparison を 3 案で比較し、actual checker-facing contract を theorem-line retained bridge にどこまで近づけるかを docs-first で整理してください。
