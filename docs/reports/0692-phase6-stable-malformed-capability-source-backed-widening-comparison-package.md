# Report 0692 — phase6 stable malformed capability source-backed widening comparison package

- Date: 2026-04-13T23:38:06+0900
- Author / agent: Codex
- Scope: `specs/examples/401...402` による stable malformed capability second source-backed widening actualization comparison close と、その mirror update
- Decision levels touched: L2

## 1. Objective

`e13/e20` pair judgment を崩さずに、capability family の source-backed widening actualization をどの first cut で扱うかを fixed する。

## 2. Scope and assumptions

- current package は docs-only actualization comparison に留める。
- `specs/examples/397...398` の pair judgment、helper-local capability compare、stage1 reconnect widen、fixture-static rows は current entry evidence として再利用してよいと仮定する。
- actual source sample / runner / ladder widening は次段 actual package に残す。
- `plan/09-helper-stack-and-responsibility-map.md` は今回 package では更新不要と判断する。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/48-current-l2-capability-third-checker-spike.md`
- `specs/examples/391...392`
- `specs/examples/397...400`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e13-malformed-capability-strengthening.json`
- `crates/mir-ast/tests/fixtures/current-l2/e20-malformed-late-capability-strengthening.json`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `scripts/current_l2_capability_checker.py`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`

## 4. Actions taken

1. `e13/e20` family の current evidence を helper-local checker、stage1 reconnect、fixture-static rows の 3 方向で再確認した。
2. missing-option first source-backed widening actualization を closest precedent として比較し、capability family の actualization comparison / threshold pair `401...402` を追加した。
3. current first choice を `e13/e20` source-authored static-stop pair first cut に固定し、implementation order だけを `e13` lead staging note に留める current cut を明記した。
4. snapshot / plan / abstract / FAQ / sample docs を current line `public operational CLI concrete shell actualization comparison` に合わせて更新した。
5. `plan/90-source-traceability.md` に今回 package の addendum を追加した。

## 5. Evidence / outputs / test results

- `e13` / `e20` は fixture-static family として `capability_strengthens` row を already 持つ。
- stage1 parser spike では `e13` / `e20` の source-text form と reconnect cluster summary が already ある。
- capability helper-local compare は `scripts/current_l2_capability_checker.py` と detached-loop smoke family で existing evidence を持つ。
- current judgment は次の通り。
  - actualization family は `e13/e20` source-authored static-stop pair に置く
  - entry evidence は helper-local capability compare、stage1 reconnect widen、fixture-static rows に置く
  - actualized surface は source sample / lowerer / runner / ladder / regression helper / fixture-static formal-hook smoke に留める
  - duplicate cluster、`TryFallback` / `AtomicCut` malformed-static broader family、theorem/model-check/public-checker wideningは kept-later に残す
- Commands run:
  - `python3 -m unittest scripts.tests.test_current_l2_capability_checker`
  - `cargo test -p mir-ast --test current_l2_stage1_parser_spike`
  - `cargo test -p mir-semantics --test current_l2_static_gate_support`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## 6. What changed in understanding

- capability family は helper-local compare 止まりではなく、missing-option family と同じ malformed source-backed widening line へ進める前提が already 揃っている。
- `e13/e20` pair judgment は comparison package の時点で十分強く、single-row lead へ縮める合理性は現状薄い。

## 7. Open questions

- capability source-backed widening actualization の implementation order を `e13` lead staged cut にするか
- capability widening close 後に duplicate cluster と malformed-static broader line をどう reopen するか
- capability widening actualization と public operational CLI actual shell のどちらを先に actual code にするか

## 8. Suggested next prompt

`tasks.md` 先頭の `public operational CLI concrete shell actualization comparison` を、そのまま次の package として自走してください。
