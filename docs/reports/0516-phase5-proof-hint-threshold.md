# 0516 — phase5 proof-hint threshold

## Objective

`specs/examples/200-current-l2-theorem-line-external-contract-payload-ready-proof-hint-threshold.md`
を追加し、Phase 5 theorem-line の current promoted line として

- consumer-specific external contract payload の次段で `proof_hint` をどこまで retained bridge に近づけるか
- `consumer_hint` や second consumer pressure をどこまで still 後段に残すか

を narrow に比較して、current first choice を固定する。

## Scope and assumptions

- `proof_notebook` first bridge だけを扱う。
- `consumer_hint` と second consumer pressure はまだ扱わない。
- theorem-line retained bridge を actual exported contract や proof assistant adapter contract に既成事実化しない。

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
- `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md`
- `specs/examples/199-current-l2-theorem-line-actual-external-contract-ready-consumer-specific-external-contract-payload-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0517-review-phase5-proof-hint-threshold-package.md`

## Actions taken

1. `specs/examples/200-current-l2-theorem-line-external-contract-payload-ready-proof-hint-threshold.md` を追加し、external contract payload の次段として
   - terminal cut
   - minimal `proof_hint` enrichment
   - `consumer_hint` / second consumer pressure 同時導入
   の 3 案を比較した。
2. current first choice を、`retained_payload_body_materialization_external_contract_proof_hint` だけを足す retained bridge として固定した。
3. mirror を `126...200` snapshot と `proof-hint-ready consumer-hint / second-consumer-pressure comparison` に更新した。

## Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/200-current-l2-theorem-line-external-contract-payload-ready-proof-hint-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0516-phase5-proof-hint-threshold.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- resource preflight
  - `2026-04-10 17:27 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 516 numbered report(s).`
- `git diff --check`
  - 無出力

## What changed in understanding

- minimal consumer-specific payload の次段では、`proof_hint` enrichment 自体は retained bridge に narrow に actualize してよい。
- ただし `consumer_hint` や second consumer pressure は同じ reopen に混ぜず、still 後段に残す方が line を保ちやすい。
- next promoted line は `proof-hint-ready consumer-hint / second-consumer-pressure comparison` と読むのが自然になった。

## Open questions

- `consumer_hint` enrichment をどの field で切るか
- `proof_hint` enrichment を retained bridge のまま維持するか richer payload へ actualize するか
- `proof_assistant_adapter` / `theorem_export_checker` pressure を later candidate のまま維持する concrete threshold を何と読むか

## Suggested next prompt

`specs/examples/200-current-l2-theorem-line-external-contract-payload-ready-proof-hint-threshold.md` を前提に、proof-hint-ready consumer-hint / second-consumer-pressure comparison を 3 案で比較し、current retained bridge に `consumer_hint` enrichment をどこまで近づけるかを docs-first で整理してください。
