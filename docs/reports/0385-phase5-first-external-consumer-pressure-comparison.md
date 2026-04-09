# 0385 — Phase 5 first external consumer pressure comparison

## Objective

Phase 5 later reopen candidate の次段として、

- `theorem_prover_boundary`
- `protocol_verifier_boundary`
- `runtime_policy_boundary`

のどの系統が **first concrete external consumer pressure** として最も自然かを比較し、
Phase 5 current package の next practical reopen line を narrow に決める。

## Scope and assumptions

- `specs/examples/126...` から `128...` までの current Phase 5 package を前提にする。
- mixed row bundle を current docs-only default に維持する前提は崩さない。
- final theorem prover / protocol verifier / runtime policy schema や actual emitter は固定しない。

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
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. Phase 5 current package を読み直し、mixed row default を維持したまま first concrete consumer pressure を比べる比較軸を固定した。
2. `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md` を追加し、
   - theorem line を current first practical candidate
   - protocol line を second practical candidate
   - runtime policy line を later candidate
   とする current first choice を整理した。
3. fallback chain / static gate 側の theorem row と shared-space `room_action` 側の protocol row を対比し、現時点では theorem line の方が unresolved policy を巻き込みにくいことを practical example で示した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に揃えた。

## Files changed

- `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md`
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
sed -n '1,260p' specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md
sed -n '1,220p' docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
sed -n '1,120p' tasks.md
sed -n '18,65p' progress.md
sed -n '1,120p' plan/11-roadmap-near-term.md
sed -n '255,268p' plan/12-open-problems-and-risks.md
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## Evidence / outputs / test results

- `specs/examples/129...` で、first concrete external consumer pressure の current first practical candidate は `theorem_prover_boundary` だと固定した。
- `protocol_verifier_boundary` は second practical candidate、`runtime_policy_boundary` は later candidate と整理した。
- `progress.md` と `tasks.md` は Phase 5 を fourth inventory package close とし、next later reopen を theorem line narrow actualization comparison に更新した。

## What changed in understanding

- Phase 5 later reopen candidate は、単に「consumer pressure を待つ」だけではなく、
  **theorem line を first practical candidate に固定したうえで narrow actualization を考える**
  段階に進んだ。
- protocol / runtime line は current docs では重要だが、first actualization candidate に上げるには unresolved policy を巻き込みすぎる。

## Open questions

- theorem line を first actualization candidate にしたときの minimal dedicated row shape は何か。
- `evidence_refs` を theorem line で stable actual artifact ref へ寄せるなら、どの ref family が first candidate になるか。
- theorem line を actual consumer に寄せても mixed row default を維持できるか、それとも theorem-only split を先に切るべきか。

## Suggested next prompt

`Phase 5 の later reopen 候補として、theorem_prover_boundary を first practical candidate にしたうえで、mixed row default を壊さずに theorem-side narrow actualization をどこまで切るかを docs-first で比較してください。`
