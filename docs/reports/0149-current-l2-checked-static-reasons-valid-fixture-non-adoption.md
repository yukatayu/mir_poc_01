# Report 0149 — current L2 checked static reasons valid fixture non-adoption

- Date: 2026-04-05T10:48:41.000168Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC で `checked_reasons` を valid fixture へ `[]` として広げない current judgment を docs / plan / progress に反映する
- Decision levels touched: L2

## 1. Objective

`checked_reasons` は current L2 で additive optional bridge だが、
実際に machine-check 上の利得があるのは actual static gate wording が non-empty で安定している fixture に限られる。

本 task の目的は、

- `e4` / `e5` のような malformed / underdeclared fixture と
- `e1` / `e3` / `e6` / `e10` のような valid fixture

を分けて読み、
**valid fixture に `checked_reasons = []` を広く足す task は current L2 では優先しない**
という判断を source-backed に明記することである。

## 2. Scope and assumptions

- runtime semantics は変更しない
- actual compare source は引き続き `static_gate_detailed().reasons`
- `expected_static.reasons` は human-facing explanation に残す
- detached-side `reason_codes` mirror は helper-local / reference-only のまま
- current task は docs / plan / progress の refinement に限る

## 3. Inputs consulted

1. `README.md`
2. `Documentation.md`
3. `progress.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
10. `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
11. `plan/11-roadmap-near-term.md`
12. `plan/15-current-l2-fixture-authoring-template.md`
13. `plan/90-source-traceability.md`
14. `docs/reports/0146-current-l2-detached-static-reason-code-mirror.md`
15. `docs/reports/0147-current-l2-checked-static-reasons-fixture-adoption.md`
16. `crates/mir-semantics/src/harness.rs`
17. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
18. `crates/mir-ast/tests/fixtures/current-l2/e1-place-atomic-cut.json`
19. `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
20. `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
21. `crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json`
22. `crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json`
23. `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json`

## 4. Actions taken

1. current fixture corpus と test anchor を読み、`checked_reasons` actual adoption の有無を確認した。
2. mapper subagent に safe candidate / non-candidate の棚卸しをさせ、`e4` / `e5` が safe adoption candidate、`e1` / `e3` / `e6` / `e10` が current non-candidate だという partial finding を受けた。
3. `specs/examples/33-current-l2-checked-static-reasons-carrier.md` を更新し、actual static gate `reasons` が空な valid fixture では `checked_reasons = []` を無理に足さない current judgment を明記した。
4. `plan/11-roadmap-near-term.md`、`plan/15-current-l2-fixture-authoring-template.md`、`plan/90-source-traceability.md`、`progress.md` を更新し、near-term priority が valid fixture への空 carrier 拡大ではないことを明示した。
5. reviewer を 1 回だけ回し、report metadata と progress log の不足を補正した。

## 5. Files changed

- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0149-current-l2-checked-static-reasons-valid-fixture-non-adoption.md`

## 6. Commands run and exact outputs

```text
cargo test -p mir-semantics static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording -- --exact
running 1 test
test static_only_fixture_corpus_uses_checked_reasons_only_for_stable_actual_wording ... ok
```

```text
rg -n 'expected_static|checked_reasons' crates/mir-ast/tests/fixtures/current-l2/*.json
crates/mir-ast/tests/fixtures/current-l2/e1-place-atomic-cut.json:67:  "expected_static": {
crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json:50:  "expected_static": {
crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json:81:  "expected_static": {
crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json:92:  "expected_static": {
crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json:85:  "expected_static": {
crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json:56:  "expected_static": {
crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json:67:  "expected_static": {
crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json:73:  "expected_static": {
crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json:89:  "expected_static": {
crates/mir-ast/tests/fixtures/current-l2/e8-monotone-degradation-reject.json:101:  "expected_static": {
crates/mir-ast/tests/fixtures/current-l2/e9-monotone-degradation-success.json:101:  "expected_static": {
```

```text
sed -n '64,90p' crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json
sed -n '54,82p' crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json
[both fixtures contain checked_reasons]
```

## 7. Evidence / findings

- current corpus で `checked_reasons` actual adoption があるのは `e4` / `e5` であり、これは
  - static-only fixture
  - malformed / underdeclared
  - actual static gate wording が 1 本に安定
  - fixture-side `reasons` を explanation として残したい
  という条件を満たす。
- `e1` / `e3` / `e6` / `e10` は current scan 範囲では valid fixture であり、fixture-side `reasons` は explanatory prose である。
- これら valid fixture に `checked_reasons = []` を機械的に足しても、
  - current machine-check gain は小さい
  - explanation と bridge の境界が曖昧になる
  - authoring template で「空 carrier をとりあえず埋める」誤用を誘発しやすい
  ため、current L2 では優先しない方が自然である。

## 8. Changes in understanding

- `checked_reasons` adoption は「field を広く埋める」方向ではなく、
  **actual static wording が non-empty で stable な fixture にだけ narrow に置く**
  という運用が current L2 に合う。
- current valid fixture 群に対しては、`verdict` only machine-check と detached static gate artifact compare の併用で十分であり、
  空の `checked_reasons` を導入することは必須ではない。

## 9. Open questions

- valid fixture でも actual static gate `reasons` が non-empty になる future case をどう authoring guideline に書くか。
- `checked_reasons` をこれ以上広げる前に、fixture scaffold helper に何を入れて何を入れないかをどこまで自動化するか。
- detached-side `reason_codes` mirror と fixture-side `checked_reasons` bridge のどちらを次に強くするか。

## 10. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、fixture scaffold helper と authoring template が malformed / underdeclared static-only fixture を起こすとき、checked_reasons bridge の候補をどこまで assist してよいかを source-backed に比較し、auto-fill を避けつつ authoring friction を下げる最小 cut を整理してください。`
