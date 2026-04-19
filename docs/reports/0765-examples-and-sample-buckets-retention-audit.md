# 0765 — examples and sample buckets retention audit

## Objective

`specs/examples/` と `samples/` の recent anchor 群について、
stale / superseded な dead entry が残っていないかを監査し、
安全に削除できるものがあれば narrow に除去し、
なければ retention reason を明記する。

## Scope and assumptions

- 対象は主に `specs/examples/45x...48x` と `samples/prototype/current-l2-*` である。
- numbered example docs は report / traceability / document map と結びついた時系列証跡でもある。
- traceability を壊す削除は行わない。

## Documents consulted

- `Documentation.md`
- `specs/00-document-map.md`
- `progress.md`
- `tasks.md`
- `plan/90-source-traceability.md`
- `specs/examples/450-current-l2-macro5-boundary-pilot-and-framing-closeout-threshold.md`
- `specs/examples/469-current-l2-near-end-closeout-after-actual-adoption-defaults.md`
- `specs/examples/487-current-l2-theorem-review-unit-transport-and-notebook-contract-actual-adoption.md`
- `samples/prototype/current-l2-order-handoff/*`
- `samples/prototype/current-l2-typed-proof-model-check/*`

## Actions taken

1. `specs/examples/45x...48x` の basename 参照数を repo 全体で集計した。
2. `samples/prototype/current-l2-order-handoff/*` と `samples/prototype/current-l2-typed-proof-model-check/*` の basename 参照数を集計した。
3. low-count 側の representative entry として `450`、`469`、`487` の参照元を spot check し、traceability / document map / report anchor の実参照を確認した。
4. numbered examples と prototype samples の双方について、safe dead entry に該当するものがあるかを判定した。

## Evidence / outputs / test results

- `for f in specs/examples/45*-*.md specs/examples/46*-*.md specs/examples/47*-*.md specs/examples/48*-*.md; do ...; done | sort -n`
  - self-only の entry は出なかった。
  - low-count entry でも、document map / traceability / report での外部参照があった。
- `for f in samples/prototype/current-l2-order-handoff/*.txt samples/prototype/current-l2-typed-proof-model-check/*.txt; do ...; done | sort -n`
  - prototype sample はすべて外部参照を持っていた。
- `rg -nF "...450..." .`
  - `specs/00-document-map.md`
  - `plan/90-source-traceability.md`
  - `docs/reports/0714-...`
- `rg -nF "...469..." .`
  - `specs/00-document-map.md`
  - `plan/90-source-traceability.md`
  - `docs/reports/0744-...`
- `rg -nF "...487..." .`
  - `specs/00-document-map.md`
  - `plan/90-source-traceability.md`
  - `docs/reports/0764-...`

## What changed in understanding

- recent numbered examples は compare / adoption / closeout の時系列証跡として機能しており、count が低いものでも dead file ではなかった。
- prototype samples も runner / helper / docs / reports で参照されており、current floor では削除より retention の方が正しい。

## Open questions

- 将来、numbered example docs の archival policy を別 bucket に切るか。
- traceability を保持したまま historical example を compress するか。

## Suggested next prompt

`model-check second-line か witness/provider/public-shape の later mixed gate を 1 本選び、actual adoption に寄せられる package があるかを再監査してください。`
