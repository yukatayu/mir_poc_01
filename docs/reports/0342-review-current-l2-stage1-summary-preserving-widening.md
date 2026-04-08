# Report 0342 — review current L2 stage1 summary-preserving widening

- Date: 2026-04-08
- Author / agent: Codex
- Scope: `specs/examples/117` / `118`、`crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` widening、mirror 一式、`0341-current-l2-stage1-summary-preserving-widening.md` の final diff review
- Decision levels touched:
  - L2: current L2 parser boundary / first checker reconnect

## 1. Objective

stage 1 summary-preserving widening diff が current contract を越えていないかを最終確認し、finding の有無を記録する。

重視点は次の 4 点である。

1. `e18` / `e20` を current `Stage1ReconnectClusters` 3-bool summary contract のままで actualize する judgment が `45` / `73` / `114` / `115` / `116` と整合するか
2. helper contract widening を暗黙に起こしていないか
3. plan / progress / report の主張範囲が過剰でないか
4. docs / test drift がないか

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/114-current-l2-stage1-first-checker-reconnect-first-tranche-actualization.md`
- `specs/examples/115-current-l2-stage1-widening-vs-stage2-try-rollback-reconnect.md`
- `specs/examples/116-current-l2-stage2-try-rollback-reconnect-first-tranche-actualization.md`
- `specs/examples/117-current-l2-stage2-contrast-vs-stage1-summary-preserving-widening.md`
- `specs/examples/118-current-l2-stage1-summary-preserving-widening-actualization.md`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e18-malformed-missing-successor-option.json`
- `crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json`
- `crates/mir-ast/tests/fixtures/current-l2/e20-malformed-late-capability-strengthening.json`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0341-current-l2-stage1-summary-preserving-widening.md`

## 3. Actions taken

1. spec 117 / 118 の sequencing judgment と actualization cut を、45 / 73 / 114 / 115 / 116 の既存判断と突き合わせた。
2. `current_l2_stage1_parser_spike.rs` widening が support helper contract を変更していないことを diff と helper source で確認した。
3. `e18` / `e19` / `e20` fixture の `expected_static.checked_reason_codes` を見て、`e18` / `e20` だけが current 3-bool summary に自然に乗り、`e19` は `declared_target_mismatch` の別 pressure を持つことを確認した。
4. mirror / report / progress の主張が「summary-preserving widening まで actualize 済み」に留まり、`e19` や stage 2 runtime contrast を既成事実化していないことを確認した。

plan/ 更新不要。  
progress.md 更新不要。

## 4. Files changed

- `docs/reports/0342-review-current-l2-stage1-summary-preserving-widening.md`

## 5. Commands run and exact outputs

```text
$ cargo test -p mir-ast --test current_l2_stage1_parser_spike
running 14 tests
...
test stage1_parser_spike_matches_e18_fixture_subset ... ok
test stage1_parser_spike_matches_e20_fixture_subset ... ok
test stage1_parser_spike_marks_missing_option_reconnect_cluster_for_e18 ... ok
test stage1_parser_spike_marks_capability_reconnect_cluster_for_e20 ... ok
...
test result: ok. 14 passed; 0 failed

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 345 numbered report(s).

$ git diff --check
[no output]
```

## 6. Evidence / findings

- **No findings.**

確認結果の要点は次の通り。

- `e18` / `e20` を追加しても `Stage1ReconnectClusters` は既存 3-bool のままで済んでおり、support helper 側の field / compare surface / public boundary は広がっていない。
- `e18` は lineage annotation を持つため `same_lineage_floor = true` を保ちつつ、missing successor option により `missing_option_structure_floor = true` になる。これは fixture `e18` の `missing_successor_option` static stop と矛盾しない。
- `e20` は earlier edge を含む chain 全体を通して late capability strengthening を検出しており、`capability_strengthening_floor = true` で自然に要約できる。
- `e19` は fixture 側でも `declared_target_mismatch` を別 reason family として持つため、117 / 118 が still later に残す判断は 45 / 114 / 115 と整合する。
- plan / progress / report はいずれも「summary-preserving widening (`e18` / `e20`) まで actualize 済み」と述べるに留まり、helper redesign や stage 2 runtime-contrast widening を暗黙に完了扱いしていない。

## 7. Changes in understanding

- stage 1 reconnect family は、`e19` を除く widening であれば current 3-bool summary contract を保ったまま厚くできる、という understanding がさらに強くなった。
- したがって current next comparison は、やはり
  - `e19-malformed-target-mismatch` の summary contract
  - stage 2 `E21` / `E22` runtime-contrast reconnect threshold
  の 2 本に絞ってよい。

## 8. Open questions

- `e19-malformed-target-mismatch` を stage 1 reconnect summary にどう乗せるか
- stage 2 `E21` / `E22` runtime contrast を parser-side reconnect に mirror する threshold をどこに置くか

## 9. Suggested next prompt

```text
Phase 3 の next narrow comparison として、
1. e19-malformed-target-mismatch をどういう reconnect summary contract に乗せるか
2. stage 2 E21 / E22 runtime contrast を parser-side reconnect に mirror する threshold
を source-backed に比較してください。
```
