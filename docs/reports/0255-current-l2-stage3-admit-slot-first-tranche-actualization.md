# Report 0255 — current L2 stage 3 admit-slot first tranche actualization

- Date: 2026-04-06
- Author / agent: Codex
- Decision levels touched: L2

## 1. Objective

`specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md` を前提に、
stage 3 request / admissibility cluster の最初の sub-cutである declaration-side `admit` attached slot を
actual parser spike としてどこまで narrow に actualize できるかを確認する。

具体的には、

- `decl_admit_slot` naming / compare surface の docs-only cut を先に固定する
- `crates/mir-ast/tests/support/` 配置の private helper を追加する
- `e3-option-admit-chain` 由来の option / chain subset compare と
  `decl_admit_slot.surface_text` retention smoke を TDD で通す

ところまでを current first tranche とする。

## 2. Scope and assumptions

- current L2 の core semantics、parser-free AST fixture schema、runtime semantics は変更しない。
- stage 1 parser spike の first tranche / malformed-source tranche は維持する。
- stage 3 first tranche は declaration-side `admit` attached slot だけに留める。
- fixture-side `OptionDecl.admit` predicate node への direct lowering は行わない。
- `PerformVia` / request-local `require` / `ensure` / predicate fragment parse は still later stage に残す。
- public parser API、typed parser diagnostics、fixture schema widening は導入しない。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
- `specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`
- `specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md`
- `specs/examples/82-current-l2-stage1-parser-spike-malformed-source-first-tranche-actualization.md`
- `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md`
- `specs/examples/84-current-l2-stage3-admit-slot-carrier-and-compare-surface.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`

## 4. Actions taken

1. `specs/examples/84-current-l2-stage3-admit-slot-carrier-and-compare-surface.md` を追加し、
   `decl_admit_slot` naming と compare surface を比較した。
2. docs-only judgment として、
   fixture-side `OptionDecl.admit` へ direct lowering せず、
   lowered fixture-subset compare と parser-side slot retention smoke を分ける cut を固定した。
3. `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs` と
   `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs` を追加し、TDD で RED を確認した。
4. success-side first tranche として、
   `option ... lease ... admit SLOT` を opaque attached slot として parse し、
   `e3` 由来 option / chain subset compare と `decl_admit_slot.surface_text` retention smoke を通した。
5. `specs/examples/85-current-l2-stage3-admit-slot-first-tranche-actualization.md` を追加し、
   current actualized scope と non-goal を明示した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90`、`progress.md` を更新した。

## 5. Files changed

- Added `specs/examples/84-current-l2-stage3-admit-slot-carrier-and-compare-surface.md`
- Added `specs/examples/85-current-l2-stage3-admit-slot-first-tranche-actualization.md`
- Added `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- Added `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- Updated `Documentation.md`
- Updated `specs/00-document-map.md`
- Updated `plan/07-parser-free-poc-stack.md`
- Updated `plan/09-helper-stack-and-responsibility-map.md`
- Updated `plan/11-roadmap-near-term.md`
- Updated `plan/12-open-problems-and-risks.md`
- Updated `plan/90-source-traceability.md`
- Updated `progress.md`
- Added `docs/reports/0255-current-l2-stage3-admit-slot-first-tranche-actualization.md`

## 6. Evidence / outputs / test results

### Commands run

```bash
cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike
cargo test -p mir-ast
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git diff --check
```

### RED

- `cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike`
  - initial stub state では
    - `stage3_admit_slot_parser_spike_matches_e3_fixture_structural_subset`
    - `stage3_admit_slot_parser_spike_keeps_decl_admit_slot_surface_text`
    がともに `"stage 3 admit-slot parser spike not implemented"` で失敗した。

### GREEN

- `cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike`
  - `running 2 tests`
  - `2 passed`
- `cargo test -p mir-ast`
  - unit 0 件
  - `current_l2_stage1_parser_spike.rs` 5 件 pass
  - `current_l2_stage3_admit_slot_spike.rs` 2 件 pass
  - doc-tests 0 件
- `cargo test -p mir-semantics`
  - unit 3 件 pass
  - detached support tests 12 件 pass
  - integration 46 件 pass
  - doc-tests 0 件
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 254 numbered report(s).`
- `git diff --check`
  - 無出力

### Direct evidence

- `stage3_admit_slot_parser_spike_matches_e3_fixture_structural_subset`
  - `e3-option-admit-chain.json` 由来の option / chain structural subset と一致した。
- `stage3_admit_slot_parser_spike_keeps_decl_admit_slot_surface_text`
  - `owner_writer.decl_admit_slot.surface_text == "owner_is(session_user)"`
  - `delegated_writer.decl_admit_slot.surface_text == "delegate_granted(session_user)"`
  が成立した。

## 7. What changed in understanding

- stage 3 admit-slot branch は、fixture-side `OptionDecl.admit` node を直接 compare しなくても、
  structural subset compare と parser-side slot retention smoke を分けることで narrow に actualize できる。
- `decl_admit_slot` を parser-side opaque carrier に留めれば、
  stage 3 first tranche を進めても request cluster や predicate parse を hidden に先食いしなくて済む。
- `crates/mir-ast/tests/support/` 配置の private helper という stage 1 line を stage 3 にも保てるため、
  public parser API や parser crate boundary をまだ固定せずに済む。

## 8. Open questions

- stage 3 admit-slot branch の malformed-source first tranche を helper-local にどこまで持たせるか。
- `PerformVia` / request-local clause spillover を stage 3 admit-slot branch と混ぜずに fail-closed で示す最小 pair は何か。
- fixture-side `OptionDecl.admit` node への handoff comparison を later stage にどう残すか。
- current private helper を public parser API へ昇格させる entry criteria をどこまで narrow に切るか。

plan/ 更新済み:
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`

## 9. Suggested next prompt

`specs/examples/85-current-l2-stage3-admit-slot-first-tranche-actualization.md` を前提に、次は stage 3 admit-slot branch の malformed-source first tranche を narrow に比較し、`PerformVia` / request-local clause spillover と混線しない最小 pair を source-backed に整理してください。
