# Report 0337 — current L2 first checker reconnect family and first tranche

- Date: 2026-04-08T09:29:11Z
- Author / agent: Codex
- Scope: Phase 3 checker-side reconnect の first family selection と、stage 1 chain / declaration structural floor family の helper-local / test-only first tranche actualization
- Decision levels touched: L2

## 1. Objective

request contract subset family freeze と Phase 3 resume side-selection の後続として、

1. existing parser-boundary evidence family のうち、どれを first checker cut inventory へ最初に reconnect するか
2. その chosen family をどの first tranche で actualize するか

を source-backed に整理し、可能な最小 code evidence まで actualize する。

## 2. Scope and assumptions

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- public parser API、public checker API、final parser grammar は still later に残す。
- current task の actualization は `crates/mir-ast/tests/support/` と test file に留める。
- `e17-malformed-missing-predecessor-option` は current stage 1 surface の structural predecessor 制約により first tranche へ入れない。

## 3. Documents consulted

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
- `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
- `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
- `specs/examples/48-current-l2-capability-third-checker-spike.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md`
- `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
- `specs/examples/112-current-l2-phase3-resume-side-selection.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/fixtures/current-l2/`

## 4. Actions taken

1. reconnect family 候補を `stage1 chain/declaration structural floor`、`try/rollback structural floor`、`request/clause attachment family` の 3 案で比較した。
2. `specs/examples/113-current-l2-first-checker-reconnect-family-selection.md` を追加し、stage 1 family を first reconnect family とする judgment を固定した。
3. stage 1 family の first tranche 候補を比較し、existing `e4` / `e7` working set を維持しつつ `e13` と `e16` を追加する representative-per-new-family cut を `specs/examples/114-current-l2-stage1-first-checker-reconnect-first-tranche-actualization.md` に固定した。
4. TDD として `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` に reconnect summary test と `e13` / `e16` fixture-subset compare test を先に追加し、missing symbol compile failure を確認した。
5. `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs` に `Stage1ReconnectClusters` と `summarize_stage1_reconnect_clusters()` を追加し、stage 1 lowered subset が same-lineage / missing-option / capability family のどれに触れているかを helper-local / test-only summary として actualize した。
6. top-level docs / roadmap / traceability / progress mirror を更新した。

## 5. Files changed

- `specs/examples/113-current-l2-first-checker-reconnect-family-selection.md`
- `specs/examples/114-current-l2-stage1-first-checker-reconnect-first-tranche-actualization.md`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0337-current-l2-first-checker-reconnect-family-and-first-tranche.md`

## 6. Commands run

```bash
date -u '+%Y-%m-%dT%H:%M:%SZ'
date '+%Y-%m-%d %H:%M:%S %Z'

cargo test -p mir-ast --test current_l2_stage1_parser_spike

cargo test -p mir-ast --test current_l2_stage1_parser_spike

cargo test -p mir-ast

python3 scripts/validate_docs.py
git diff --check
```

## 7. Evidence / outputs / test results

### red step

最初の `cargo test -p mir-ast --test current_l2_stage1_parser_spike` は compile fail した。

```text
error[E0432]: unresolved imports `...::Stage1ReconnectClusters`, `...::summarize_stage1_reconnect_clusters`
```

これは test 側が新しい reconnect summary helper を要求しており、support 側がまだ未実装だったことを示す。

### green step

support helper 追加後の focused test は pass した。

```text
running 10 tests
...
test stage1_parser_spike_marks_capability_reconnect_cluster_for_e13 ... ok
test stage1_parser_spike_marks_missing_option_reconnect_cluster_for_e16 ... ok
test stage1_parser_spike_marks_same_lineage_reconnect_cluster_for_e4 ... ok
test stage1_parser_spike_matches_e13_fixture_subset ... ok
test stage1_parser_spike_matches_e16_fixture_subset ... ok
...
test result: ok. 10 passed; 0 failed
```

### full verification

- `cargo test -p mir-ast` は pass
- `python3 scripts/validate_docs.py` は `Documentation scaffold looks complete.` / `Found 338 numbered report(s).`
- `git diff --check` は無出力

## 8. What changed in understanding

- Phase 3 の checker-side reconnect は、`specs/examples/45...` の checker baseline と `specs/examples/73...` の parser staging の両方に最も近い stage 1 chain / declaration structural floor familyから始めるのが自然だと確定した。
- first tranche では full family coverage を急がず、`e13` と `e16` を representative anchor として足す cut が最小であると分かった。
- `e17-malformed-missing-predecessor-option` は current stage 1 surface で structural predecessor が chain progression に固定されるため、same family でも first tranche からは外すのが自然だと明確になった。

## 9. Open questions

- stage 1 reconnect family を `e18` / `e19` / `e20` まで widening してから stage 2 `try` / rollback reconnect へ進むべきか。
- `e17-malformed-missing-predecessor-option` を current stage 1 accepted cluster の外に残したまま進める cut が十分か。
- reconnect summary helper を later task で checker-side exact compare へ近づける必要があるか。

## 10. Suggested next prompt

`specs/examples/113-current-l2-first-checker-reconnect-family-selection.md` と `specs/examples/114-current-l2-stage1-first-checker-reconnect-first-tranche-actualization.md` を前提に、stage 1 reconnect family を `e18` / `e19` / `e20` まで widening するか、それとも stage 2 の `try` / rollback structural floor reconnect へ進むかを narrow に比較してください。
