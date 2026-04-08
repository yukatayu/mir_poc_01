# Report 0339 — current L2 stage2 try/rollback reconnect

- Date: 2026-04-08
- Author / agent: Codex
- Scope: Phase 3 checker-side reconnect の next step として、stage 1 widening と stage 2 `try` / rollback reconnect を比較し、stage 2 first tranche を private helper / test-only actual evidence まで actualize する。
- Decision levels touched:
  - L2: current L2 parser boundary / first checker reconnect sequencing
  - L2: current L2 stage 2 try/rollback reconnect first tranche cut

## 1. Objective

- stage 1 reconnect family の remaining widening (`e18` / `e19` / `e20`) と、stage 2 `try` / rollback structural floor reconnect のどちらを先に取るのが自然かを source-backed に比較する。
- 自然なら、そのまま stage 2 reconnect の first tranche を helper-local / test-only actual evidence まで下ろす。
- docs / plan / progress mirror を current phase line に合わせて更新する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md`
- `specs/examples/65-current-l2-try-rollback-ast-helper-first-tranche-cut.md`
- `specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/113-current-l2-first-checker-reconnect-family-selection.md`
- `specs/examples/114-current-l2-stage1-first-checker-reconnect-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e18-malformed-missing-successor-option.json`
- `crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json`
- `crates/mir-ast/tests/fixtures/current-l2/e20-malformed-late-capability-strengthening.json`
- `crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json`
- `crates/mir-ast/tests/fixtures/current-l2/e24-malformed-atomic-cut-fallback-placement.json`
- `scripts/current_l2_try_rollback_structural_checker.py`
- `scripts/tests/test_current_l2_try_rollback_structural_checker.py`

## 3. Actions taken

1. stage 1 widening と stage 2 reconnect の比較を整理し、next narrow step を stage 2 `try` / rollback structural floor に置く judgement を新規 spec に追加した。
2. stage 2 reconnect first tranche の actualization cut を新規 spec に整理した。
3. TDD red として `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs` を追加し、missing support file で compile fail を確認した。
4. green として `crates/mir-ast/tests/support/current_l2_stage2_try_rollback_spike_support.rs` を追加し、
   - single `try { ... } fallback { ... }` block parse
   - `atomic_cut` vs opaque `Other` statement head split
   - helper-local `Stage2TryRollbackStructuralSummary`
   - fixture-side `checked_try_rollback_structural_*` loader
   を実装した。
5. focused tests で `e23` / `e24` expectation compare と valid one-shot `atomic_cut` in try body の `no_findings` smoke を通した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90`、`progress.md` を更新した。

## 4. Files changed

- Added: `specs/examples/115-current-l2-stage1-widening-vs-stage2-try-rollback-reconnect.md`
- Added: `specs/examples/116-current-l2-stage2-try-rollback-reconnect-first-tranche-actualization.md`
- Added: `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- Added: `crates/mir-ast/tests/support/current_l2_stage2_try_rollback_spike_support.rs`
- Added: `docs/reports/0339-current-l2-stage2-try-rollback-reconnect.md`
- Added: `docs/reports/0340-review-current-l2-stage2-try-rollback-reconnect.md`
- Modified: `Documentation.md`
- Modified: `specs/00-document-map.md`
- Modified: `plan/07-parser-free-poc-stack.md`
- Modified: `plan/09-helper-stack-and-responsibility-map.md`
- Modified: `plan/11-roadmap-near-term.md`
- Modified: `plan/12-open-problems-and-risks.md`
- Modified: `plan/90-source-traceability.md`
- Modified: `progress.md`

## 5. Commands run and exact outputs

### resource check

```text
$ date '+%Y-%m-%d %H:%M:%S %Z'
2026-04-08 18:15:05 JST

$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   89G  5.8G  94% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       760Mi        93Mi       2.0Mi       106Mi       199Mi
Swap:           19Gi       1.4Gi        18Gi
```

### TDD red

```text
$ cargo test -p mir-ast --test current_l2_stage2_try_rollback_spike
error: couldn't read `crates/mir-ast/tests/support/current_l2_stage2_try_rollback_spike_support.rs`: No such file or directory (os error 2)
```

### focused green

```text
$ cargo test -p mir-ast --test current_l2_stage2_try_rollback_spike
running 3 tests
test stage2_try_rollback_spike_marks_no_findings_for_atomic_cut_in_try_body ... ok
test stage2_try_rollback_spike_matches_e23_fixture_expectation ... ok
test stage2_try_rollback_spike_matches_e24_fixture_expectation ... ok

test result: ok. 3 passed; 0 failed
```

### full verification

```text
$ cargo test -p mir-ast
test result: ok. 10 passed; 0 failed   # current_l2_stage1_parser_spike
test result: ok. 3 passed; 0 failed    # current_l2_stage2_try_rollback_spike
test result: ok. 6 passed; 0 failed    # current_l2_stage3_admit_slot_spike
test result: ok. 8 passed; 0 failed    # current_l2_stage3_multiline_attachment_spike
test result: ok. 7 passed; 0 failed    # current_l2_stage3_predicate_fragment_spike
test result: ok. 11 passed; 0 failed   # current_l2_stage3_request_clause_suite_spike

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 340 numbered report(s).

$ git diff --check
[no output]
```

## 6. Evidence / findings

- stage 1 widening より先に stage 2 reconnect を取る方が自然である。
  - stage 1 first tranche は already `same_lineage` / `missing_option_structure` / `capability_strengthening` の 3 family representative を持つ。
  - `e19` は current `Stage1ReconnectClusters` summary を押し広げやすく、stage 1 widening は next narrow step として少し重い。
  - stage 2 側には `checked_try_rollback_structural_*` dedicated contract と `e23` / `e24` pair が already ある。
- stage 2 reconnect first tranche は parser-side private helper + fixture-side checked field exact compare で十分に切れた。
- valid one-shot `atomic_cut` in try body の `no_findings` smoke を first tranche に含めることで、`atomic_cut` が ordinary statement として try body に置ける structural floor を narrow に示せた。
- nested `place`、`place_anchor == current_place` gate、restore scope は current tranche でも still later に残している。

## 7. Changes in understanding

- Phase 3 checker-side reconnect の next step は stage 1 widening ではなく stage 2 `try` / rollback structural floor が自然である。
- stage 2 reconnect first tranche は、parser-side private helper で block shape を parse しつつ、compare surface 自体は existing `checked_try_rollback_structural_*` に合わせる cut が最小である。
- current next narrow step は、
  - stage 2 reconnect family を `E21` / `E22` runtime contrast まで widening するか
  - stage 1 reconnect family の remaining widening (`e18` / `e19` / `e20`) に戻るか
  の比較である。

## 8. Open questions

- stage 2 reconnect の次段は `E21` / `E22` contrast まで広げるべきか。
- stage 1 remaining widening のうち、`e19` をどういう reconnect summary contract に載せるのが自然か。
- stage 2 private helper で nested `place` を accepted cluster に入れる threshold はどこか。
- runtime / proof boundary に残している `place_anchor == current_place` gate と restore scope を parser-side reconnect にどこまで mirror すべきか。

## 9. Suggested next prompt

```text
Phase 3 の next narrow step として、
1. stage 2 reconnect family を E21 / E22 runtime contrast まで widening する案
2. stage 1 reconnect family の remaining widening (e18 / e19 / e20) に戻る案
を source-backed に比較し、自然なら chosen tranche を helper-local / test-only actual evidence まで actualize してください。
```
