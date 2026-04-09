# 0387 — Phase 5 theorem-line narrow actualization package

## Objective

Phase 5 later reopen candidate として、

- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md`

を追加し、

- mixed row default を壊さずに theorem line をどこまで narrow actualization できるか
- theorem-side `evidence_refs` をどの family まで stable に切ってよいか

を docs-first で整理する。

## Scope and assumptions

- `specs/examples/126...` から `129...` までの current Phase 5 package を前提にする。
- `theorem_prover_boundary` を first practical candidate に置く current judgmentは維持する。
- actual theorem handoff emitter、public checker API、actual path / URI based ref は固定しない。
- shared-space side line は protocol / runtime candidate に留め、premature に巻き込まない。

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
- `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. Phase 5 current package を読み直し、theorem line の later reopen を
   - mixed row default 維持
   - theorem-side first cut
   - stable `evidence_refs` family
   の 2 段比較に分けた。
2. `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md` を追加し、
   - mixed row only
   - docs-only theorem-side projection bundle
   - dedicated theorem artifact / emitter
   を比較して、docs-only projection bundle を current first choice に置いた。
3. `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md` を追加し、
   - free-form symbolic string
   - typed symbolic ref family
   - actual path / emitted artifact ref
   を比較して、typed symbolic `evidence_refs` family を current first choice に置いた。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に揃えた。

## Files changed

- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md`
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
date '+%Y-%m-%d %H:%M %Z'
df -h .
free -h
git status --short --branch
sed -n '1,260p' specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md
sed -n '1,280p' specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md
sed -n '1,260p' specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md
sed -n '1,240p' specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## Evidence / outputs / test results

- `specs/examples/130...` で、theorem line の current first cut は mixed row default を壊さない docs-only projection bundle だと固定した。
- `specs/examples/131...` で、theorem-side `evidence_refs` の current first choice は typed symbolic ref family だと固定した。
- `progress.md` と `tasks.md` は、Phase 5 を theorem-line later package close と読み替え、next later reopen を public checker migration threshold comparison に更新した。
- `python3 scripts/validate_docs.py` は成功した。
- `git diff --check` は無出力で通った。

## What changed in understanding

- theorem line の later reopen は、actual emitter へ進む前に
  - docs-only projection bundle
  - typed symbolic `evidence_refs` family
  を一段ずつ固定する方が自然だと分かった。
- actual path / URI based ref は theorem line の narrow cutではなく、detached path policy と bless flow が見える later reopen へ残す方が安全である。

## Open questions

- theorem-side projection bundle を public checker migration に寄せてよい threshold はどこか。
- typed symbolic `evidence_refs` family を boundary-specific carrier へ昇格させる concrete pressure を何とみなすか。
- actual theorem handoff emitter を later reopen に保ちながら、proof notebook / assistant との narrow bridge をどこまで docs-only に切れるか。

## Suggested next prompt

`Phase 5 の next later reopen candidate として、theorem-side projection bundle と typed symbolic evidence_refs family を前提に、public checker migration threshold をどこに置くのが最小かを docs-first で比較してください。`
