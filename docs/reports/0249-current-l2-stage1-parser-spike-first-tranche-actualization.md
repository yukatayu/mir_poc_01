# Report 0249 — current L2 stage 1 parser spike first tranche actualization

- Date: 2026-04-06
- Author / agent: Codex
- Decision levels touched: L2

## 1. Objective

`specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md` と
`specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`
で固定した docs-only judgment を、current repo の non-production / private parser spike として
どこまで actualize できるかを確認する。

具体的には、

- `crates/mir-ast/tests/support/` 配置の private helper
- inline text input
- lowered fixture-subset compare
- `decl_guard_slot.surface_text` retention

を `e4` / `e7` の two-fixture pair で実証する。

## 2. Scope and assumptions

- current L2 の core semantics、parser-free AST fixture schema、runtime semantics は変更しない。
- final parser grammar、public parser API、predicate fragment parse は扱わない。
- compare surface は parser-side raw AST snapshot でも full fixture compare でもなく、lowered fixture-subset compare に留める。
- current helper は test-only / private / non-production であり、production parser API の既成事実化は行わない。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md`
- `specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md`
- `specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `crates/mir-ast/Cargo.toml`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

## 4. Actions taken

1. `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` を追加し、先に RED の integration test を書いた。
2. `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs` を追加し、
   line-oriented な stage 1 parser spike helper、fixture subset loader、thin lowering bridge を実装した。
3. `crates/mir-ast/Cargo.toml` に local `dev-dependencies` として `serde_json` を追加した。
4. `specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md` を追加し、first tranche の actualized scope と non-goal を明示した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90`、`progress.md` を更新した。

## 5. Files changed

- Added `specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md`
- Added `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- Added `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- Updated `crates/mir-ast/Cargo.toml`
- Updated `Documentation.md`
- Updated `specs/00-document-map.md`
- Updated `plan/07-parser-free-poc-stack.md`
- Updated `plan/09-helper-stack-and-responsibility-map.md`
- Updated `plan/11-roadmap-near-term.md`
- Updated `plan/12-open-problems-and-risks.md`
- Updated `plan/90-source-traceability.md`
- Updated `progress.md`
- Updated `docs/reports/0249-current-l2-stage1-parser-spike-first-tranche-actualization.md`

## 6. Evidence / outputs / test results

### Commands run

```bash
cargo test -p mir-ast
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git diff --check
```

### Outputs

- `cargo test -p mir-ast`
  - unit 0 件
  - `tests/current_l2_stage1_parser_spike.rs` 3 件 pass
- `cargo test -p mir-semantics`
  - unit 3 件 pass
  - integration 58 件相当 pass
  - doc-tests 0 件
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 249 numbered report(s).`
- `git diff --check`
  - 無出力

### Direct evidence

- `stage1_parser_spike_matches_e4_fixture_subset`
  - `e4` 相当 inline text が existing fixture subset と一致した
- `stage1_parser_spike_matches_e7_fixture_subset`
  - `e7` 相当 inline text が existing fixture subset と一致した
- `stage1_parser_spike_keeps_decl_guard_slot_surface_text`
  - `writer.decl_guard_slot.surface_text == "expired"` が成立した

## 7. What changed in understanding

- docs-only で止めていた stage 1 parser spike の first tranche は、current repo で actual code anchor を持てると確認できた。
- ただし actualized のは declaration / chain structural floor だけであり、predicate fragment parse や request / admissibility cluster を前倒ししてよいという意味ではない。
- current helper は `mir-ast` test support に留めることで、parser boundary 側の実証を進めても parser-free PoC helper stack 本体を汚さずに済むことが確認できた。

## 8. Open questions

- `e3-option-admit-chain` を stage 2 parser spike へいつ送るか。
- malformed-source smoke を parser helper 自身へどこまで持たせるか。
- current private helper を actual parser crate / module へ昇格させる entry criteria をどこまで narrow に切るか。
- final parser grammar、public parser API、span-aware carrier をどの順で actual workstream に乗せるか。

plan/ 更新済み:
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`

## 9. Suggested next prompt

`specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md` を前提に、次は stage 1 parser spike の malformed-source smoke を parser helper 自身へどこまで持たせるべきかを narrow に比較し、`e4` / `e7` first tranche との責務境界を source-backed に整理してください。
