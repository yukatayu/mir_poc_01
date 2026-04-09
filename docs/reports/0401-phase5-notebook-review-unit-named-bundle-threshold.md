# 0401 — Phase 5 notebook review unit named-bundle threshold

- Date: 2026-04-09 22:38 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line の notebook review unit named-bundle threshold comparison と mirror 更新
- Decision levels touched: L1 / L2

## 1. Objective

Phase 5 theorem-line の next later reopen candidate として、

- review checklist / walkthrough unit を unnamed prose のまま維持する
- docs-only named review unit bundle に寄せる
- stronger notebook bridge sketch へ進める

のどこが current first cut として自然かを docs-first に比較し、
current first choice を固定する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md`
- `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md`
- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md`
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
- `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`
- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`
- `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md`
- `specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md`
- `specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md`
- `specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0399-phase5-concrete-notebook-workflow-pressure-comparison.md`
- `docs/reports/0400-review-phase5-concrete-notebook-workflow-pressure-comparison.md`

## 3. Actions taken

1. `specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md` を追加し、
   - unnamed prose 維持
   - docs-only named review unit bundle
   - stronger notebook bridge sketch
   を比較した。
2. current first choice を
   - review checklist / walkthrough unit は docs-only named review unit bundle に寄せる
   - stronger notebook bridge sketch は second step
   - actual emitted notebook artifact はさらに後段
   に固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に揃えた。
4. reviewer subagent を 1 回起動して長めに待ち、必要なら retry を 1 回行う closeout 方針を取った。

## 4. Files changed

- `specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md`
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

## 5. Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## 6. Evidence / findings

- `specs/examples/139...` で、review checklist / walkthrough pressure を受けた current first cut は stronger bridge sketch ではなく docs-only named review unit bundle に留める、と固定した。
- current named review unit の最小 shape は
  - `subject_ref`
  - `row(obligation_kind + evidence_refs + goal_text)`
  - `checklist`
  に留め、compare / bless policy と emitted artifact pressure は後段に残した。
- local validation は通っており、review closeout は `0402` に記録する。

## 7. Changes in understanding

- concrete notebook workflow pressure を認めた次段では、unnamed prose に留まるよりも docs-only named review unit bundle へ進む方が、later reopen の start point を明確にしやすい。
- ただし stronger notebook bridge sketch をこの段階で切るのは still 早く、compare / bless / retention pressure と混ざりやすい。

## 8. Open questions

- named review unit を stronger notebook bridge sketch へどこまで拡張するか
- actual theorem handoff emitter を later reopen に保てるか
- typed symbolic `evidence_refs` family を boundary-specific handoff artifact に昇格させる concrete pressure を何とみなすか
- `proof_assistant_adapter` consumer pressure を second practical candidate のまま維持する条件がいつ崩れるか

## 9. Suggested next prompt

`Phase 5 の next later reopen candidate として、docs-only named notebook review unit を stronger notebook bridge sketch へどこまで拡張するのが最小かを docs-first で比較してください。`
