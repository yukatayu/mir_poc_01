# 0389 — Phase 5 theorem-line public checker migration threshold

## Objective

`specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
を追加し、

- theorem-side projection bundle を current phase で public checker migration に寄せるか
- それとも concrete theorem consumer pressure が出るまで docs-only bridge に留めるか

を docs-first で比較する。

## Scope and assumptions

- `specs/examples/126...` から `131...` までの current Phase 5 package を前提にする。
- theorem-side projection bundle と typed symbolic `evidence_refs` family の current cutは維持する。
- actual theorem handoff emitter、public checker API、actual path / URI ref は固定しない。

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
- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. theorem-side later reopen package を読み直し、projection bundle と typed symbolic `evidence_refs` family が current first cut だと再確認した。
2. `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md` を追加し、
   - docs-only bridge 維持
   - public checker migration
   - actual theorem emitter
   の 3 案を比較した。
3. actual theorem consumer がまだ repo 内で source-backed でないことを根拠に、current phase では docs-only bridge 維持を current first choice に置いた。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に揃えた。

## Files changed

- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
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
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
git diff --stat
```

## Evidence / outputs / test results

- `specs/examples/132...` で、current phase の theorem-side projection bundle は docs-only bridge に留め、public checker migration は concrete theorem consumer pressure が出たときだけ reopen する threshold を固定した。
- `progress.md` と `tasks.md` は、Phase 5 を theorem-line later package close と読みつつ、next later reopen を minimum contract row comparison に更新した。
- `python3 scripts/validate_docs.py` は成功した。
- `git diff --check` は無出力で通った。

## What changed in understanding

- theorem line の later reopen は、public checker migration へ進む前に docs-only bridge として止める方が current phase に自然だと分かった。
- 次の reopen は API actualization ではなく、concrete theorem consumer に渡す minimum contract row を docs-only でどこまで書くかに絞るのが最小である。

## Open questions

- minimum contract row を docs-only で書くとき、`obligation_kind` family を theorem-side subset にどこまで絞るべきか。
- actual theorem consumer pressure の concrete signal を何とみなすか。
- emitted theorem artifact を later reopen に保つための最小条件をどこに置くか。

## Suggested next prompt

`Phase 5 の next later reopen candidate として、concrete theorem consumer bridge に必要な minimum contract rows をどこまで docs-only で切るのが最小かを比較してください。`
