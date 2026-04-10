# 0526 — phase5 theorem-export-checker contract threshold

## Objective

`specs/examples/205-current-l2-theorem-line-theorem-export-checker-pressure-ready-checker-facing-contract-threshold.md`
を追加し、Phase 5 theorem-line の current promoted line として

- symbolic `theorem_export_checker` pressure の次段で actual checker-facing contract をどこまで retained bridge に近づけるか
- exported checker payload をどこまで still 後段に残すか

を narrow に比較して、current first choice を固定する。

## Scope and assumptions

- current `proof_notebook` first bridge を起点にする。
- first machine-facing checker candidate は `theorem_export_checker` に限る。
- exported checker payload と checker result materialization family はまだ扱わない。

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
- `specs/examples/203-current-l2-theorem-line-second-consumer-pressure-ready-proof-assistant-adapter-contract-threshold.md`
- `specs/examples/204-current-l2-theorem-line-proof-assistant-adapter-contract-ready-theorem-export-checker-pressure-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. `specs/examples/205-current-l2-theorem-line-theorem-export-checker-pressure-ready-checker-facing-contract-threshold.md` を追加し、theorem-export-checker-pressure-ready retained bridge の次段として
   - terminal cut
   - minimal actual checker-facing contract
   - exported checker payload 同時導入
   の 3 案を比較した。
2. current first choice を、`retained_payload_body_materialization_theorem_export_checker_contract` だけを足す retained bridge として固定した。
3. mirror を `126...205` snapshot と `theorem-export-checker-contract-ready exported-checker-payload-pressure comparison` に更新した。

## Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/205-current-l2-theorem-line-theorem-export-checker-pressure-ready-checker-facing-contract-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0526-phase5-theorem-export-checker-contract-threshold.md`

## Commands run and exact outputs

```bash
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/new_report.py --slug phase5-theorem-export-checker-contract-threshold
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / findings

- timestamp snapshot
  - `2026-04-10 20:11 JST`
- `python3 scripts/new_report.py --slug phase5-theorem-export-checker-contract-threshold`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0526-phase5-theorem-export-checker-contract-threshold.md`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 527 numbered report(s).`
- `git diff --check`
  - no output

## Changes in understanding

- symbolic `theorem_export_checker` pressure の次段では、actual checker-facing contract 自体は retained bridge に narrow に actualize してよい。
- ただし exported checker payload は同じ reopen に混ぜず、still 後段に残す方が line を保ちやすい。
- next promoted line は `theorem-export-checker-contract-ready exported-checker-payload-pressure comparison` と読むのが自然になった。

## Open questions

- exported checker payload をどの field / row / payload family で切るか
- `theorem_export_checker` contract を retained bridge のまま維持するか exported checker payload family へ actualize するか
- exported checker payload pressure の concrete threshold を何とみなすか

## Suggested next prompt

`specs/examples/205-current-l2-theorem-line-theorem-export-checker-pressure-ready-checker-facing-contract-threshold.md` を前提に、theorem-export-checker-contract-ready exported-checker-payload-pressure comparison を 3 案で比較し、exported checker payload を theorem-line retained bridge にどこまで近づけるかを docs-first で整理してください。
