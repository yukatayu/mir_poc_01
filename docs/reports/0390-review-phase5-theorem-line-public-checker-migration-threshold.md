# 0390 — review: Phase 5 theorem-line public checker migration threshold

## Objective

`specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
と mirror 更新が、

- `specs/examples/126...` から `131...` までの current Phase 5 judgment
- mixed row default と theorem-side projection bundle の cut
- `progress.md` / `tasks.md` / `plan/11` / `plan/17` の Phase 5 reading

を壊していないかを確認する。

## Scope and assumptions

- review 対象は docs-first package 全体である。
- current session では reviewer-capable subagent tool が露出していないため、
  local evidence fallback を残す。

## Documents consulted

- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md`
- `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md`
- `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`
- `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md`
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
- `docs/reports/0389-phase5-theorem-line-public-checker-migration-threshold.md`

## Actions taken

1. reviewer-capable subagent tool の露出有無を確認し、current session では使えないことを再確認した。
2. fallback として、Phase 5 package と mirror 更新の focused diff inspection を行った。
3. `validate_docs` と `git diff --check` の結果を再確認し、shared-space side line の premature spillover と Phase 5 reading の不一致を点検した。

## Evidence / outputs / test results

- local diff inspection では、`specs/examples/132...` は theorem-side projection bundle を docs-only bridge に留めており、`specs/examples/130...` と `131...` の current first choiceを weaken していない。
- `plan/11`、`plan/17`、`progress.md`、`tasks.md` は、Phase 5 を theorem-line later package close と読み、next later reopen を minimum contract row comparison に揃えている。
- `plan/12` は public checker migration 問いを threshold 問題として保持し、actual emitter や path policy の premature 固定に寄っていない。
- `plan/90` には `0389` / `0390` と `specs/examples/132...` の provenance を追加済みであり、traceability には空きがない。
- `python3 scripts/validate_docs.py` は成功した。
- `git diff --check` は無出力で通った。

## What changed in understanding

- Phase 5 の current checkpoint では、public checker migration も still later reopen に残す方が repo の docs-first discipline と整合する。
- reviewer-capable subagent tool が current session に無い場合でも、focused diff inspection と validation evidence で close 可能である。

## Open questions

- reviewer-capable subagent tool が見えない session での標準 review fallback をどこまで短文化するか。
- theorem-side minimum contract row comparison に進むとき、review checklist に `obligation_kind` subset drift を追加する必要があるか。

## Suggested next prompt

`Phase 5 の next later reopen candidate として、concrete theorem consumer bridge に必要な minimum contract rows をどこまで docs-only で切るのが最小かを比較してください。`
