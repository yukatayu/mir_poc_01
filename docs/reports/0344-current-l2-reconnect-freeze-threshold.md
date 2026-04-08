# Report 0344 — current L2 reconnect freeze threshold

- Date: 2026-04-08
- Author / agent: Codex
- Scope: Phase 3 checker-side reconnect lineについて、`e19` widening・stage 2 `E21` / `E22` runtime contrast widening・current tranche freeze の 3 案を比較し、current reconnect subline を一区切りで freeze する judgment を mirror まで反映する。
- Decision levels touched:
  - L2: current L2 parser boundary / first checker reconnect threshold
  - L2: Phase 3 closeout sequencing

## 1. Objective

- `e19-malformed-target-mismatch` を stage 1 reconnect summary に混ぜるべきか、stage 2 `E21` / `E22` contrast を parser-side reconnect へ mirror するべきか、それとも reconnect subline 自体を current tranche で freeze するべきかを source-backed に比較する。
- reconnect threshold judgment を `specs/examples/119...` と plan / progress mirror に反映する。
- Phase 3 closeout sweep へ進む current stop line を top-level docs で辿れるようにする。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/40-current-l2-first-typed-static-reason-family-selection.md`
- `specs/examples/42-current-l2-second-typed-static-reason-family-actualization.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `specs/examples/114-current-l2-stage1-first-checker-reconnect-first-tranche-actualization.md`
- `specs/examples/115-current-l2-stage1-widening-vs-stage2-try-rollback-reconnect.md`
- `specs/examples/116-current-l2-stage2-try-rollback-reconnect-first-tranche-actualization.md`
- `specs/examples/117-current-l2-stage2-contrast-vs-stage1-summary-preserving-widening.md`
- `specs/examples/118-current-l2-stage1-summary-preserving-widening-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json`
- `crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.json`
- `crates/mir-ast/tests/fixtures/current-l2/e22-try-atomic-cut-place-mismatch.json`
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. `e19` が current reconnect line の natural widening candidate かどうかを再確認し、`expected_static.checked_reason_codes` / detached `reason_codes` で already actualize 済みの declared target edge pair family であることを code / spec anchor まで再確認した。
2. `E21` / `E22` が current parser-side reconnect line に自然に乗るかを再確認し、contrast の本体が nested `place` / `place_anchor == current_place` / restore scope であり、current `checked_try_rollback_structural_*` compare surface では still runtime / proof boundary に寄ることを整理した。
3. 3 案比較の結果を `specs/examples/119-current-l2-reconnect-freeze-threshold.md` に追加した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/11`、`plan/12`、`plan/17`、`plan/90`、`progress.md` を更新し、current reconnect subline を freeze した上で Phase 3 closeout sweep へ進む current reading を揃えた。

## 4. Files changed

- Added: `specs/examples/119-current-l2-reconnect-freeze-threshold.md`
- Added: `docs/reports/0344-current-l2-reconnect-freeze-threshold.md`
- Modified: `Documentation.md`
- Modified: `specs/00-document-map.md`
- Modified: `plan/07-parser-free-poc-stack.md`
- Modified: `plan/11-roadmap-near-term.md`
- Modified: `plan/12-open-problems-and-risks.md`
- Modified: `plan/17-research-phases-and-autonomy-gates.md`
- Modified: `plan/90-source-traceability.md`
- Modified: `progress.md`

## 5. Commands run and exact outputs

- `cargo test -p mir-ast`
  - `current_l2_stage1_parser_spike`: 14 passed
  - `current_l2_stage2_try_rollback_spike`: 3 passed
  - `current_l2_stage3_admit_slot_spike`: 6 passed
  - `current_l2_stage3_multiline_attachment_spike`: 8 passed
  - `current_l2_stage3_predicate_fragment_spike`: 7 passed
  - `current_l2_stage3_request_clause_suite_spike`: 11 passed
  - doc-tests: 0 failed
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 348 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- `e19-malformed-target-mismatch` は reconnect line にとって新しい structural floor evidence というより、already actualize 済みの declared target edge pair family の typed static reason anchor である。
- stage 2 `E21` / `E22` は runtime representative として強いが、current parser-side reconnect line では contrast を自然に受ける dedicated carrier をまだ持たない。
- したがって current reconnect subline は、
  - stage 1 same-lineage / missing-option / capability representative widening
  - stage 2 malformed pair + valid one-shot `atomic_cut` smoke
  までで一区切りとして freeze するのが最も clean である。

## 7. Changes in understanding

- current reconnect line は「coverage をさらに広げるべき line」ではなく、「private staged spike として十分 source-backed になった line」と読める。
- Phase 3 の next work は reconnect widening ではなく、closeout sweep と current state summary へ移せる。

## 8. Open questions

- final parser grammar をどの subset から actual parser へ上げるか
- public checker API をどこで切るか
- `E21` / `E22` runtime contrast を将来どの layer で mirror するか
- `e19` を reconnect line に再度重ねる必要が将来本当にあるか

## 9. Suggested next prompt

```text
Phase 3 closeout sweep として、parser boundary / first checker cut の current staged line を top-level docs / plan / progress / reports で再点検し、closeout summary と stop line を整理してください。
```
