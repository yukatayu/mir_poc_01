# Report 0257 — current L2 stage 3 admit-slot malformed-source first tranche

- Date: 2026-04-06
- Author / agent: Codex
- Decision levels touched: L2

## 1. Objective

stage 3 admit-slot branch の success-side first tranche を前提に、
helper-local malformed-source smoke をどこまで narrow に足すべきかを比較し、
最小 malformed pair を TDD で actualize する。

## 2. Scope and assumptions

- current L2 の core semantics、parser-free AST fixture schema、runtime semantics は変更しない。
- stage 3 success-side first tranche は維持する。
- malformed-source smoke は helper-local substring compare に留める。
- typed parser error carrier、public diagnostics API、request-local `require` / `ensure` spillover は still later stage に残す。

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
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`

## 4. Actions taken

1. `specs/examples/86-current-l2-stage3-admit-slot-malformed-source-comparison.md` を追加し、最小 malformed pair を比較した。
2. current judgment として、
   - `admit` payload 欠落
   - `PerformVia` request-head spillover
   の 2 本だけを helper-local malformed-source smoke に採った。
3. `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs` に RED の malformed-source smoke 2 本を追加した。
4. RED として targeted test を回し、generic wording では落ちることを確認した。
5. `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs` に helper-local wording fragment 2 件を narrow に追加した。
6. `specs/examples/87-current-l2-stage3-admit-slot-malformed-source-first-tranche-actualization.md` を追加し、actualized malformed-source pair と non-goal を記録した。
7. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/11`、`plan/12`、`plan/90`、`progress.md` を更新した。

## 5. Files changed

- Added `specs/examples/86-current-l2-stage3-admit-slot-malformed-source-comparison.md`
- Added `specs/examples/87-current-l2-stage3-admit-slot-malformed-source-first-tranche-actualization.md`
- Updated `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- Updated `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- Updated `Documentation.md`
- Updated `specs/00-document-map.md`
- Updated `plan/07-parser-free-poc-stack.md`
- Updated `plan/11-roadmap-near-term.md`
- Updated `plan/12-open-problems-and-risks.md`
- Updated `plan/90-source-traceability.md`
- Updated `progress.md`
- Added `docs/reports/0257-current-l2-stage3-admit-slot-malformed-source-first-tranche.md`

## 6. Evidence / outputs / test results

### Commands run

```bash
cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike
```

### RED

- `stage3_admit_slot_parser_spike_rejects_missing_admit_slot_payload`
  - got `line 2: unsupported option declaration ... admit`
- `stage3_admit_slot_parser_spike_rejects_request_head_spillover`
  - got `line 4: unsupported stage 3 admit-slot input \`perform write_profile via profile_ref\``

### GREEN

- `cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike`
  - `running 4 tests`
  - `4 passed`

## 7. What changed in understanding

- stage 3 admit-slot branch でも、accepted-cluster malformed 1 件と later-stage spillover 1 件の pair が最小である。
- later-stage spillover としては request-local clause 単体より `PerformVia` request head の方が、`e3` contrast に近く boundary が読みやすい。
- typed parser diagnostics へ進まなくても、helper-local wording fragment 2 件だけで stage 3 boundary を十分示せる。

## 8. Open questions

- request-local `require` / `ensure` spillover を stage 3 admit-slot branch の later malformed pair にどこまで持たせるか。
- fixture-side `OptionDecl.admit` node への handoff comparison を later stage にどう残すか。
- current private helper を public parser API へ昇格させる entry criteria をどこまで narrow に切るか。

plan/ 更新済み:
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`

## 9. Suggested next prompt

`specs/examples/87-current-l2-stage3-admit-slot-malformed-source-first-tranche-actualization.md` を前提に、次は stage 3 admit-slot branch の request-local clause spillover と fixture-side `admit` node handoff comparison のどちらを先に進めるべきかを narrow に比較してください。
