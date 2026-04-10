# 0512 — phase5 actual external contract threshold

## Objective

`specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md`
を追加し、Phase 5 theorem-line の current promoted line として

- external-contract-facing handoff row の次段で actual external contract をどこまで retained bridge に近づけるか
- consumer-specific external contract payload をどこまで still 後段に残すか

を narrow に比較して、current first choice を固定する。

## Scope and assumptions

- `proof_notebook` first bridge だけを扱う。
- consumer-specific external contract payload はまだ扱わない。
- theorem-line retained bridge を actual exported contract や public checker contract に既成事実化しない。

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
- `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`
- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`
- `specs/examples/197-current-l2-theorem-line-boundary-specific-handoff-artifact-row-ready-external-contract-facing-handoff-row-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0513-review-phase5-actual-external-contract-threshold.md`

## Actions taken

1. `specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md` を追加し、external-contract-facing handoff row の次段として
   - terminal cut
   - minimal actual external contract
   - consumer-specific external contract payload 同時導入
   の 3 案を比較した。
2. current first choice を、`retained_payload_body_materialization_external_contract` だけを足す retained bridge として固定した。
3. mirror を `126...198` snapshot と `actual-external-contract-ready consumer-specific external contract payload comparison` に更新した。

## Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0512-phase5-actual-external-contract-threshold.md`
- `docs/reports/0513-review-phase5-actual-external-contract-threshold.md`

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
  - `2026-04-10 17:00 JST`
  - `df -h .` → `/dev/vda2 99G total / 92G used / 2.9G avail / 98%`
  - `free -h` → `Mem 960Mi total / 796Mi used / 76Mi free / 163Mi available`, `Swap 19Gi total / 2.2Gi used / 17Gi free`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 512 numbered report(s).`
- `git diff --check`
  - 無出力

## What changed in understanding

- external-contract-facing handoff row の次段では、actual external contract 自体は retained bridge に narrow に actualize してよい。
- ただし consumer-specific external contract payload は同じ reopen に混ぜず、still 後段に残す方が line を保ちやすい。
- next promoted line は `actual-external-contract-ready consumer-specific external contract payload comparison` と読むのが自然になった。

## Open questions

- consumer-specific external contract payload をどの field で切るか
- actual external contract を retained bridge のまま維持するか consumer-specific payload へ actualize するか
- `proof_assistant_adapter` / `theorem_export_checker` pressure を later candidate のまま維持する concrete threshold を何と読むか

## Suggested next prompt

`specs/examples/198-current-l2-theorem-line-external-contract-facing-handoff-row-ready-actual-external-contract-threshold.md` を前提に、actual-external-contract-ready consumer-specific external contract payload comparison を 3 案で比較し、current retained bridge に consumer-specific external contract payload をどこまで近づけるかを docs-first で整理してください。
