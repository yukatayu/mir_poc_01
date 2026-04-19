# Report 0811 — Package 58 theorem/model-check helper preview widening

## Objective

Package 58 の remaining line として、
`run-source-sample` helper summary に theorem-side result-object preview と model-check side public-checker preview を追加し、
representative sample 上で reached / guard split を machine-check 可能にする。

## Scope and assumptions

- final public theorem result object、consumer-shaped theorem payload public contract、concrete theorem prover brand、proof object public schemaは固定しない。
- final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract は固定しない。
- widening は helper-local operational CLI diagnostics に留める。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/491-current-l2-theorem-result-object-preview-and-proof-object-schema-reserve-actualization.md`
- `specs/examples/492-current-l2-model-check-public-checker-artifact-preview-and-verifier-handoff-reserve-actualization.md`
- `specs/examples/500-current-l2-theorem-result-object-route-actual-adoption.md`
- `specs/examples/501-current-l2-model-check-checker-artifact-route-actual-adoption.md`
- `specs/examples/529-current-l2-ifc-typed-checker-hint-preview-actualization.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`

## Actions taken

1. `crates/mir-runtime/src/current_l2_cli.rs` に
   - `theorem_result_object_preview`
   - `model_check_public_checker_preview`
   の helper-local summary を追加した。
2. theorem-side quartet `e5 / p06 / p07 / p08` reached、
   model-check-side quartet `e5 / p06 / p07 / p09` reached を sample-local guard で切り分けた。
3. `crates/mir-runtime/tests/current_l2_operational_cli.rs` を更新し、
   - `p06` で theorem/model-check preview 両方 reached
   - `p08` で theorem reached / model-check guarded
   - `p09` で theorem guarded / model-check reached
   を固定した。
4. `specs/examples/530` を追加し、
   helper preview widening の current recommendation / retained alternatives / stop line を記録した。
5. snapshot / roadmap / traceability 文書群を更新し、
   Package 58 の remaining line を closeout sync 主体へ寄せた。

## Evidence / outputs / test results

- `cargo test -p mir-runtime --test current_l2_operational_cli`
  - 16 tests passed
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt --format json`
  - theorem result-object preview `reached`
  - model-check public-checker preview `guarded_not_reached`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt --format json`
  - theorem result-object preview `guarded_not_reached`
  - model-check public-checker preview `reached`

## What changed in understanding

- representative Lean sample set actual Lean execution reached 後の next widening は、
  corpus をさらに増やすことよりも helper-local diagnostics を representative quartet に結びつける方が stop line を狭めやすい。
- theorem-side quartet と model-check-side quartet は同一ではなく、
  `p08` と `p09` の非対称 reached / guard を visible にした方が current boundary を誤読しにくい。
- Package 58 の remaining line は、
  broader theorem-side export そのものよりも helper/CLI mirror と closeout sync に比重が移った。

## Open questions

- theorem result-object preview を CLI helper mirror の次に theorem result-object route actual adoption とどう再接続するか。
- model-check public-checker preview を CLI helper mirror の次に checker-artifact route actual adoption とどう再接続するか。
- Package 59 で residual mixed/user-spec wording をどこまで再圧縮するか。

## Suggested next prompt

`specs/examples/530` と `docs/reports/0811` を前提に、Package 59 near-end closeout sync として `progress.md` / `tasks.md` / `plan/11` / `plan/17` / `plan/18` / `plan/90` の stale wording を洗い出し、Package 58 close 後の current queue と residual mixed gates を全面同期してください。`
