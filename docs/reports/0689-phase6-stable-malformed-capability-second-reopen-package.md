# Report 0689 — phase6 stable malformed capability second reopen package

- Date: 2026-04-13T21:25:00+0900
- Author / agent: Codex
- Scope: `specs/examples/397...398` による stable malformed capability second reopen actualization comparison close と、その mirror update
- Decision levels touched: L2

## 1. Objective

missing-option first source-backed widening close を巻き戻さず、capability family (`e13/e20`) をどの reopen cut から next malformed-side actualization へ送るかを固定する。

## 2. Scope and assumptions

- current package は docs-only comparison に留める。
- helper-local capability compare、stage1 reconnect widen、fixture-static capability rows は current evidence として再利用してよいと仮定する。
- source-authored `e13/e20` sample、runner accepted set widening、ladder wideningは次段に残す。

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
- `specs/examples/369...370`
- `specs/examples/395...396`
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

## 4. Actions taken

1. capability family の current evidence を helper-local checker、stage1 reconnect、fixture-static rows の 3 方向で整理した。
2. `e13/e20` pair judgment を維持したまま source-backed widening first に送る comparison / threshold pair `397...398` を追加した。
3. snapshot / plan / abstract / FAQ / sample docs を current line `public operational CLI concrete shell naming comparison` に合わせて更新した。
4. `plan/90-source-traceability.md` に今回 package の addendum を追加した。

## 5. Evidence / outputs / test results

- `e13` / `e20` は fixture-static family として `capability_strengthens` row を already 持つ。
- capability helper-local compare は `scripts/current_l2_capability_checker.py` と detached-loop smoke で existing evidence を持つ。
- stage1 parser reconnect widen では `e13` / `e20` の source-text form が already accepted であり、`e17` 型の source-surface blocker は current evidence からは見えていない。
- current judgment は次の通り。
  - capability family current judgment は `e13/e20` pair のまま保つ
  - helper-local capability compare、stage1 reconnect widen、fixture-static rows を entry evidence に使う
  - next malformed-side actualization mode は source-backed widening first に置く
  - `e13` lead は implementation staging note に留める
- Commands run:
  - `python3 -m unittest scripts/tests/test_current_l2_capability_checker.py`
  - `cargo test -p mir-ast --test current_l2_stage1_parser_spike`
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## 6. What changed in understanding

- capability family は missing-option line と違って current source-authored sample が未実装なだけであり、helper-local / parser-side / fixture-static evidence は already 揃っている。
- そのため current package は helper-only stop ではなく、`e13/e20` pair judgment を維持したまま source-backed widening first を next malformed-side actualization に送るのが自然である。

## 7. Open questions

- capability source-backed widening actualization を `e13/e20` pair 一括にするか、`e13` lead staged cut にするか
- duplicate cluster と `TryFallback` / `AtomicCut` malformed-static broader line を capability widening の後にどう reopen するか

## 8. Suggested next prompt

`tasks.md` 先頭の `public operational CLI concrete shell naming comparison` を、そのまま次の package として自走してください。
