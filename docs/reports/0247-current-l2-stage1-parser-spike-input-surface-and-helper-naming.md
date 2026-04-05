# Report 0247 — current L2 stage 1 parser spike input surface and helper naming

- Date: 2026-04-05T22:42:09.313003Z
- Author / agent: Codex
- Scope: docs-only comparison for stage 1 parser spike の input surface、`decl_guard_slot` internal carrier、private helper naming
- Decision levels touched: L2

## 1. Objective

`specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md` の次段として、
actual stage 1 parser spike の actual implementation へ入る前に、
input surface、`decl_guard_slot` internal carrier、private helper family naming をどこまで narrow に決めるのが最小かを比較する。

## 2. Scope and assumptions

- scope は stage 1 parser spike の implementation 直前 cut に限定する。
- current L2 semantics、final parser grammar、final parser API、final text fixture policy は変更しない。
- placement は `crates/mir-ast/tests/support/`、compare surface は lowered fixture-subset compare を前提とする。
- `e4` / `e7` が current stage 1 smoke family である current judgment を維持する。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

## 4. Actions taken

1. spec 74 / 75 / 76 / 78 を読み直し、stage 1 parser spike の implementation 前に残っている narrow choice を
   input surface / internal carrier / helper naming の 3 点へ絞った。
2. input surface について、
   - current JSON fixture reverse generation
   - test inline string
   - dedicated text fixture file
   の 3 案を比較した。
3. `decl_guard_slot` internal carrier について、
   - bare `String`
   - borrowed span / slice
   - dedicated wrapper + owned `surface_text`
   の 3 案を比較した。
4. private helper family naming について、
   - `stage1_parser_spike_support`
   - `current_l2_stage1_parser_spike_support`
   - `current_l2_decl_chain_parser_support`
   の 3 案を比較した。
5. `specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md` を追加し、current judgment を整理した。
6. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を mirror 更新した。

## 5. Evidence / outputs / test results

- Files changed:
  - Added: `specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`
  - Added: `docs/reports/0247-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`
  - Modified: `Documentation.md`
  - Modified: `specs/00-document-map.md`
  - Modified: `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
  - Modified: `plan/11-roadmap-near-term.md`
  - Modified: `plan/12-open-problems-and-risks.md`
  - Modified: `plan/90-source-traceability.md`
  - Modified: `progress.md`
- Commands run:
  - `python3 scripts/new_report.py --slug current-l2-stage1-parser-spike-input-surface-and-helper-naming`
    - Output: `/home/yukatayu/dev/mir_poc_01/docs/reports/0247-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`
  - `find crates/mir-ast -maxdepth 3 -type f | sort`
    - Output:
      - `crates/mir-ast/Cargo.toml`
      - `crates/mir-ast/src/lib.rs`
  - local file reads:
    - `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
    - `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
    - `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
    - `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
    - `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
    - `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`
- Findings:
  - input surface は test inline string が最も narrow であり、current fixture corpus を compare target anchor に残せる。
  - `decl_guard_slot` internal carrier は dedicated wrapper + owned `surface_text` が、opaque distinction と implementation simplicity を両立する。
  - private helper family naming は `current_l2_stage1_parser_spike_support` が current repo の non-production helper naming と最も整合する。
- Test results:
  - 実行系 test はなし。docs-only comparison のため file inspection と mirror 更新のみを行った。

## 6. What changed in understanding

- stage 1 parser spike の docs-only boundary は、actual implementation 前の last narrow cut までかなり揃った。
- current next step では、parser grammar や public API を決めずに、`tests/support` 内 private helper と lowered fixture-subset compare の actualization に進める。
- `decl_guard_slot` は bare text へ潰すより dedicated wrapper にした方が、later widening point を保ちやすい。
- actual helper surface で narrow に認めるべき bridge は、program-level ではなく option-level structural transfer に留める方が spec 76 と整合する。

## 7. Open questions

- actual Rust type 名を `Stage1DeclGuardSlot` / `CurrentL2Stage1DeclGuardSlot` のどちらへ寄せるか。
- inline text を dedicated text fixture file へ移す trigger をどこに置くか。
- stage 1 helper compare を `e4` / `e7` 固定の smoke に留めるか、shared compare utility を同時に切るか。

## 8. plan/ update note

- `plan/` 更新あり: `plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`

## 9. Suggested next prompt

`specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md` を前提に、non-production の actual stage 1 parser spike を `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs` に実装し、inline text input を parse して option-level bridge を通し、`e4` / `e7` の lowered fixture-subset compare smoke を通してください。
