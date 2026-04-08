# Report 0341 — current L2 stage1 summary-preserving widening

- Date: 2026-04-08
- Author / agent: Codex
- Scope: Phase 3 checker-side reconnect の next widening として、stage 2 runtime-contrast widening より先に stage 1 summary-preserving widening (`e18` / `e20`) を actualize する。
- Decision levels touched:
  - L2: current L2 parser boundary / first checker reconnect sequencing
  - L2: current L2 stage1 summary-preserving widening cut

## 1. Objective

- stage 2 `E21` / `E22` runtime contrast widening と、stage 1 `e18` / `e20` summary-preserving widening のどちらを next narrow step に置くべきかを source-backed に比較する。
- 自然なら、current `Stage1ReconnectClusters` contract を広げずに `e18` / `e20` を actual evidence に追加する。
- docs / plan / progress mirror を更新する。

## 2. Inputs consulted

- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/114-current-l2-stage1-first-checker-reconnect-first-tranche-actualization.md`
- `specs/examples/115-current-l2-stage1-widening-vs-stage2-try-rollback-reconnect.md`
- `specs/examples/116-current-l2-stage2-try-rollback-reconnect-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e18-malformed-missing-successor-option.json`
- `crates/mir-ast/tests/fixtures/current-l2/e20-malformed-late-capability-strengthening.json`

## 3. Actions taken

1. stage 2 runtime-contrast widening と stage 1 summary-preserving widening を比較し、next narrow step を stage 1 `e18` / `e20` widening に置く judgement を `specs/examples/117...` に追加した。
2. current `Stage1ReconnectClusters` contract を広げずに `e18` / `e20` を actualize する cut を `specs/examples/118...` に追加した。
3. `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` に
   - `e18` fixture subset compare
   - `e20` fixture subset compare
   - `e18` summary compare
   - `e20` summary compare
   を追加した。
4. `plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90`、`progress.md` を更新した。

## 4. Files changed

- Added: `specs/examples/117-current-l2-stage2-contrast-vs-stage1-summary-preserving-widening.md`
- Added: `specs/examples/118-current-l2-stage1-summary-preserving-widening-actualization.md`
- Added: `docs/reports/0341-current-l2-stage1-summary-preserving-widening.md`
- Added: `docs/reports/0342-review-current-l2-stage1-summary-preserving-widening.md`
- Modified: `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- Modified: `Documentation.md`
- Modified: `specs/00-document-map.md`
- Modified: `plan/07-parser-free-poc-stack.md`
- Modified: `plan/09-helper-stack-and-responsibility-map.md`
- Modified: `plan/11-roadmap-near-term.md`
- Modified: `plan/12-open-problems-and-risks.md`
- Modified: `plan/90-source-traceability.md`
- Modified: `progress.md`

## 5. Commands run and exact outputs

### focused stage1 widening evidence

```text
$ cargo test -p mir-ast --test current_l2_stage1_parser_spike
running 14 tests
...
test stage1_parser_spike_matches_e18_fixture_subset ... ok
test stage1_parser_spike_matches_e20_fixture_subset ... ok
test stage1_parser_spike_marks_missing_option_reconnect_cluster_for_e18 ... ok
test stage1_parser_spike_marks_capability_reconnect_cluster_for_e20 ... ok

test result: ok. 14 passed; 0 failed
```

### full verification

```text
$ cargo test -p mir-ast
test result: ok. 14 passed; 0 failed   # current_l2_stage1_parser_spike
test result: ok. 3 passed; 0 failed    # current_l2_stage2_try_rollback_spike
test result: ok. 6 passed; 0 failed    # current_l2_stage3_admit_slot_spike
test result: ok. 8 passed; 0 failed    # current_l2_stage3_multiline_attachment_spike
test result: ok. 7 passed; 0 failed    # current_l2_stage3_predicate_fragment_spike
test result: ok. 11 passed; 0 failed   # current_l2_stage3_request_clause_suite_spike

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 345 numbered report(s).

$ git diff --check
[no output]
```

## 6. Evidence / findings

- current next widening は stage 2 `E21` / `E22` contrast より先に stage 1 `e18` / `e20` が自然である。
  - `E21` / `E22` の差は runtime / proof boundary により近く、current `checked_try_rollback_structural_*` compare surface では new reconnect evidence が薄い。
  - `e18` / `e20` は current `Stage1ReconnectClusters` 3-bool contract をそのまま使える。
- helper support の widening は不要だった。existing stage 1 parser spike support は already generic enough であり、focused tests 追加だけで current tranche を actualize できた。
- `e19` は still later に残すのが自然である。

## 7. Changes in understanding

- stage 1 reconnect family は `e19` を除けば current summary contract のまま 1 段厚くできる。
- したがって next question は
  - `e19` の summary contract
  - stage 2 `E21` / `E22` contrast threshold
  の比較に絞れる。

## 8. Open questions

- `e19-malformed-target-mismatch` を current reconnect summary contract にどう乗せるか
- stage 2 `E21` / `E22` runtime contrast を parser-side reconnect に mirror する threshold

## 9. Suggested next prompt

```text
Phase 3 の next narrow comparison として、
1. e19-malformed-target-mismatch をどういう reconnect summary contract に乗せるか
2. stage 2 E21 / E22 runtime contrast を parser-side reconnect に mirror する threshold
を source-backed に比較してください。
```
