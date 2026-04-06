# Report 0261 — current L2 stage 3 request-local clause spillover first tranche

- Date: 2026-04-06
- Author / agent: Codex
- Decision levels touched: L2

## 1. Objective

`specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md` までで、
fixture-side `OptionDecl.admit` handoff を current phase では docs-only deferred に留めると整理したことを前提に、
stage 3 later branch の bare request-local `require` / `ensure` spillover pair を
helper-local malformed-source first tranche として actualize する。

## 2. Scope and assumptions

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- request-local clause spillover は bare line reject に留め、request attachment rule は導入しない。
- `PerformOn` / `PerformVia` parse、predicate fragment parse、public parser API は still later stage に残す。
- plan/ 更新済み。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md`
- `specs/examples/84-current-l2-stage3-admit-slot-carrier-and-compare-surface.md`
- `specs/examples/85-current-l2-stage3-admit-slot-first-tranche-actualization.md`
- `specs/examples/86-current-l2-stage3-admit-slot-malformed-source-comparison.md`
- `specs/examples/87-current-l2-stage3-admit-slot-malformed-source-first-tranche-actualization.md`
- `specs/examples/88-current-l2-stage3-admit-next-step-sequencing.md`
- `specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md`
- `specs/examples/90-current-l2-stage3-request-local-clause-spillover-comparison.md`
- `specs/examples/91-current-l2-stage3-request-local-clause-spillover-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json`
- `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json`

## 4. Actions taken

1. `specs/examples/90-current-l2-stage3-request-local-clause-spillover-comparison.md` を追加し、bare `require` / `ensure` line を helper-local malformed-source pair として扱う cut を比較した。
2. `specs/examples/91-current-l2-stage3-request-local-clause-spillover-first-tranche-actualization.md` を追加し、current tranche の actualized scope と non-goal を記録した。
3. `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs` に request-local clause spillover smoke 2 本を追加した。
4. `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs` に helper-local wording fragment 2 件を narrow に追加した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/11`、`plan/12`、`plan/90`、`progress.md` を更新した。

## 5. Files changed

- Added `specs/examples/90-current-l2-stage3-request-local-clause-spillover-comparison.md`
- Added `specs/examples/91-current-l2-stage3-request-local-clause-spillover-first-tranche-actualization.md`
- Updated `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- Updated `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- Updated `Documentation.md`
- Updated `specs/00-document-map.md`
- Updated `plan/07-parser-free-poc-stack.md`
- Updated `plan/11-roadmap-near-term.md`
- Updated `plan/12-open-problems-and-risks.md`
- Updated `plan/90-source-traceability.md`
- Updated `progress.md`
- Added `docs/reports/0261-current-l2-stage3-request-local-clause-spillover-first-tranche.md`

## 6. Evidence / outputs / test results

### Commands run

```bash
cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike
cargo test -p mir-ast
python3 scripts/validate_docs.py
git diff --check
```

### Expected helper-local wording fragments

- `request-local require clause is outside stage 3 admit-slot first tranche`
- `request-local ensure clause is outside stage 3 admit-slot first tranche`

## 7. What changed in understanding

- stage 3 later branch では、request head spillover の次に bare request-local clause spillover pair を actualize しても、request attachment rule を hidden に持ち込まずに済む。
- `e10` / `e11` の runtime anchor が request-local clause の存在理由を支え、current helper 側では bare line reject だけで later-stage boundary を十分示せる。
- stage 3 line の remaining issue は、request head + clause attachment multiline shape をどこまで docs-only に比較するか、または predicate fragment boundary の reopen 条件をどう切るか、へ絞られた。

## 8. Open questions

- request head + clause attachment multiline shape を stage 3 helper にどこまで持たせるか。
- predicate fragment boundary の reopen 条件を stage 3 line にどう接続するか。
- current private helper を public parser API へ昇格させる前提条件をどこまで narrow に切るか。

## 9. Suggested next prompt

`specs/examples/91-current-l2-stage3-request-local-clause-spillover-first-tranche-actualization.md` を前提に、次は request head + clause attachment multiline shape を docs-only comparison に持たせるべきか、それとも predicate fragment boundary の reopen 条件を先に整理すべきかを narrow に比較してください。
