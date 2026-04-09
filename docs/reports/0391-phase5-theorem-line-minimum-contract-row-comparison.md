# 0391 — Phase 5 theorem-line minimum contract row comparison

## Objective

`specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`
を追加し、

- theorem-side projection bundle を concrete theorem consumer bridge に渡すなら
  minimum contract row をどこまで docs-only で切るか
- どの field を row core に残し、どの field を consumer-specific attachment に残すか

を current Phase 5 later reopen の次段として整理する。

あわせて、stale になっていた
`docs/reports/0386-review-phase5-first-external-consumer-pressure-comparison.md`
を actual repo state に合わせて補正する。

## Scope and assumptions

- `specs/examples/126...` から `132...` までの current Phase 5 package を前提にする。
- mixed row default、theorem-side projection bundle、typed symbolic `evidence_refs` family の current cut は維持する。
- public checker API、actual theorem handoff emitter、final theorem prover input schema は固定しない。
- concrete theorem consumer class 自体は今回まだ固定しない。

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
- `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0385-phase5-first-external-consumer-pressure-comparison.md`
- `docs/reports/0386-review-phase5-first-external-consumer-pressure-comparison.md`
- `docs/reports/0387-phase5-theorem-line-narrow-actualization-package.md`
- `docs/reports/0388-review-phase5-theorem-line-narrow-actualization-package.md`
- `docs/reports/0389-phase5-theorem-line-public-checker-migration-threshold.md`
- `docs/reports/0390-review-phase5-theorem-line-public-checker-migration-threshold.md`

## Actions taken

1. Phase 5 current theorem-line package を読み直し、minimum contract row comparison が
   - projection bundle の後段
   - public checker migration の前段
   に位置することを再確認した。
2. `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md` を追加し、
   - projection bundle をそのまま bridge row とみなす案
   - minimum contract row core を `obligation_kind + evidence_refs` に絞る案
   - consumer-specific field まで row に入れる案
   を比較した。
3. current first choice を、envelope は `subject_kind + subject_ref + contract_rows[]`、
   row core は `obligation_kind + evidence_refs` に留める docs-only minimum cut に固定した。
4. `goal_text` / `proof_hint` / `consumer_hint` は consumer-specific attachment として later reopen に残す判断を mirror に反映した。
5. stale になっていた `docs/reports/0386-review-phase5-first-external-consumer-pressure-comparison.md` を、actual repo state と review scope に合わせて補正した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に揃えた。

## Files changed

- `docs/reports/0386-review-phase5-first-external-consumer-pressure-comparison.md`
- `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`
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
date '+%Y-%m-%d %H:%M:%S %Z'
df -h .
free -h
git status --short --branch
git log --oneline -n 10
sed -n '1,220p' docs/reports/0386-review-phase5-first-external-consumer-pressure-comparison.md
sed -n '1,260p' specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md
sed -n '1,260p' specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md
sed -n '1,260p' specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md
sed -n '1,260p' specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md
rg -n "minimum contract row|theorem consumer|attachment family" specs/examples plan docs/reports docs/research_abstract
```

## Evidence / outputs / test results

- `specs/examples/133...` で、current minimum contract bridge envelope は
  - `subject_kind`
  - `subject_ref`
  - `contract_rows[]`
  に留め、row core は
  - `obligation_kind`
  - `evidence_refs`
  に留める current first choice を固定した。
- `goal_text` / `proof_hint` / `consumer_hint` は row core ではなく later consumer-specific attachment に残す cut を明記した。
- `progress.md` と `tasks.md` は、Phase 5 theorem-line package を `133...` までで close と読み、next later reopen を concrete theorem consumer class / attachment family comparison に更新した。

## What changed in understanding

- theorem-side docs-only bridge を concrete consumer 側へ一歩進めても、row core 自体はかなり小さく保てる。
- current phase で本当に未決なのは row core ではなく、**どの theorem consumer class を first bridge に置くか** と **consumer-specific attachment family をどこまで先に書くか** である。

## Open questions

- concrete theorem consumer class を first bridge に何で置くのが自然か
- `goal_text` / `proof_hint` / `consumer_hint` の attachment family を docs-only でどこまで先に比べるべきか
- typed symbolic `evidence_refs` family を stable contract にいつ昇格させるべきか
- actual theorem handoff emitter を later reopen に保てるか

## Suggested next prompt

`Phase 5 の next later reopen candidate として、theorem-side minimum contract row core を前提に、concrete theorem consumer class と consumer-specific attachment family をどこまで docs-only で比べるのが最小かを比較してください。`
