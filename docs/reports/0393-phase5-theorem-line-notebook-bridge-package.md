# 0393 — Phase 5 theorem-line notebook bridge package

## Objective

Phase 5 の current later reopen を一段進めて、

- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`
- `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md`

を追加し、

- theorem-side minimum contract row core を最初に受ける concrete consumer class を何に置くか
- `proof_notebook` first bridge の attachment family をどこまで lightweight に保つか

を docs-first に整理する。

## Scope and assumptions

- `specs/examples/126...` から `133...` までの current Phase 5 package を前提にする。
- minimum contract row core は `obligation_kind + evidence_refs` の current cutを維持する。
- public checker API、actual theorem handoff emitter、solver-specific script format は固定しない。
- proof notebook / proof assistant / theorem export checker のうち、first practical consumer class だけを narrow に決める。

## Documents consulted

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
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0391-phase5-theorem-line-minimum-contract-row-comparison.md`
- `docs/reports/0392-review-phase5-theorem-line-minimum-contract-row-comparison.md`

## Actions taken

1. Phase 5 current theorem-line package を読み直し、`133...` で minimum contract row core が固定された後に残る問いを
   - first practical consumer class
   - notebook first bridge の lightweight attachment
   に分解した。
2. `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md` を追加し、
   - `proof_notebook`
   - `proof_assistant_adapter`
   - `theorem_export_checker`
   を比較して、`proof_notebook` を current first practical consumer class に置いた。
3. `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md` を追加し、
   - no attachment
   - `goal_text`
   - `goal_text + proof_hint`
   - `goal_text + proof_hint + consumer_hint`
   を比較して、current lightweight attachment を `goal_text` に固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に揃えた。

## Files changed

- `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`
- `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md`
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

## Commands run

```bash
rg -n "proof notebook|proof assistant|theorem consumer|goal_text|proof_hint|consumer_hint" specs plan docs/reports docs/research_abstract
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `specs/examples/134...` で、current first practical consumer class は `proof_notebook`、`proof_assistant_adapter` は second practical candidate、`theorem_export_checker` は later candidate と固定した。
- `specs/examples/135...` で、`proof_notebook` first bridge の current lightweight attachment は `goal_text` に留め、`proof_hint` / `consumer_hint` は later reopen に残す cut を固定した。
- `progress.md` と `tasks.md` は、Phase 5 theorem-line package を `135...` までで close と読み、next later reopen を notebook bridge artifact / stable contract threshold に更新した。

## What changed in understanding

- minimum contract row core の次で最も自然なのは、proof assistant adapter でも public checker でもなく、`proof_notebook` という lightweight consumer class である。
- row core の次の attachment も `goal_text` までに留めれば、solver-specific schema を current phase へ押し込まずに済む。

## Open questions

- notebook bridge artifact を docs-only のまま維持するか、stable contract sketch に進めるか
- typed symbolic `evidence_refs` family をいつ stable contract に昇格させるか
- actual theorem handoff emitter を later reopen に保てるか

## Suggested next prompt

`Phase 5 の next later reopen candidate として、proof_notebook first bridge と goal_text attachment を前提に、notebook bridge artifact / stable contract threshold をどこに置くのが最小かを docs-first で比較してください。`
