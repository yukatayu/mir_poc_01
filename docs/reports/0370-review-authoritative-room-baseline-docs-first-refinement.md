# 0370 — review-authoritative-room-baseline-docs-first-refinement

## Objective

`0369` の authoritative room baseline docs-first refinement に対する reviewer 指摘と反映内容を記録する。

## Scope and assumptions

- review 対象は uncommitted diff 全体
- 主眼は
  - mainline shift の mirror 一貫性
  - authoritative baseline に policy option を混ぜていないか
  - provenance / report hygiene
  である

## Documents consulted

- `docs/reports/0369-authoritative-room-baseline-docs-first-refinement.md`
- `specs/examples/121-shared-space-authoritative-room-baseline.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `progress.md`
- `plan/90-source-traceability.md`
- reviewer feedback from agent `019d70ec-ba99-7282-9555-cfd0d8d7095c`

## Actions taken

1. `plan/17` の stale numbering / stale wording を修正し、detached loop baseline close と authoritative baseline closeout を区別した。
2. `progress.md` の `次に進めるべき task` を current mainline に合わせて更新した。
3. `specs/examples/121...` で `delegated_rng_service` を fairness-source 軸の practical candidate であって overall next strengthening ではないと明記した。
4. `docs/reports/0369...` の evidence section を placeholder から実行結果へ置き換えた。
5. review 記録本体としてこの report を追加し、`plan/90` の provenance 参照先を実体と一致させた。

## Files changed

- `plan/17-research-phases-and-autonomy-gates.md`
- `progress.md`
- `specs/examples/121-shared-space-authoritative-room-baseline.md`
- `docs/reports/0369-authoritative-room-baseline-docs-first-refinement.md`
- `docs/reports/0370-review-authoritative-room-baseline-docs-first-refinement.md`

## Evidence / outputs / test results

Reviewer findings summary:

1. mainline shift mirror mismatch
2. `delegated_rng_service` wording overclaimed sequencing
3. provenance referred to missing review file
4. report evidence section still had placeholder

All four were addressed in the same change set.

## What changed in understanding

- authoritative room baseline 自体は十分 narrow だが、mainline shift と fairness-source sequencing の wording は mirror 側で崩れやすい
- shared-space line では、provider placement と witness/trust strengthening を別軸に保つことを baseline 文書側でも繰り返し明記した方が安全である

## Open questions

- `auditable authority witness` をどの task で reopen するか
- `delegated_rng_service` と epoch / incarnation split を同じ catalog comparison で扱うか、別 task に分けるか

## Suggested next prompt

`consistency / fairness / causal metadata catalog comparison` を current mainline として進め、authoritative room baseline から次に reopen する strengthening axis を 1 本選んでください。まずは provider placement と witness requirement を別軸で保ったまま、authoritative room と append-friendly room の両方にまたがる working subset を整理してください。
