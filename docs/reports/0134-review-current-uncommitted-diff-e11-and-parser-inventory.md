# Report 0134 — review current uncommitted diff e11 and parser inventory

- Date: 2026-04-05T08:27:39.372399Z
- Author / agent: Codex
- Scope: current uncommitted docs diff と、user 指定の e11 regression / test / report / mirror 一式の semantic correctness review
- Decision levels touched: L2 review only, no normative decision change

## 1. Objective

current uncommitted diff を semantic correctness と boundary discipline の観点で review する。

重点確認点は次の 4 つである。

1. `e11-perform-via-ensure-then-success` fixture と host plan
2. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` の coverage
3. `docs/reports/0126` と `0127` の e10 reviewer fix 後の accuracy
4. `specs/examples/02`、`specs/examples/06`、`plan/08`、`progress.md`、`plan/90` の整合

## 2. Inputs consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json`
10. `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.host-plan.json`
11. `crates/mir-semantics/src/lib.rs`
12. `crates/mir-semantics/src/harness.rs`
13. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
14. `docs/reports/0126-current-l2-e10-perform-on-ensure-failure-regression.md`
15. `docs/reports/0127-review-current-l2-e10-perform-on-ensure-failure-regression.md`
16. `docs/reports/0128-current-l2-e11-perform-via-ensure-then-success-regression.md`
17. `docs/reports/0129-review-current-l2-e11-perform-via-ensure-then-success-regression.md`
18. `specs/examples/02-current-l2-ast-fixture-schema.md`
19. `specs/examples/06-current-l2-interpreter-skeleton.md`
20. `plan/08-representative-programs-and-fixtures.md`
21. `progress.md`
22. `plan/90-source-traceability.md`
23. current uncommitted diff on `Documentation.md`、`specs/00-document-map.md`、`plan/06`、`plan/11`、`plan/12`、`plan/13`、`plan/90`、`progress.md`、`specs/examples/29`、`docs/reports/0132`、`docs/reports/0133`

## 3. Actions taken

1. `git status` と `git diff` で current uncommitted diff の範囲を確認した。
2. 指定された e11 fixture / host plan / interpreter test / reports / mirrors を `HEAD` の current state で照合した。
3. `PerformOn` / `PerformVia` の request-local `ensure` handling を code anchor で確認した。
4. `e10` と `e11` の focused regression test を fresh に実行した。
5. docs validation と `git diff --check` を fresh に実行した。
6. findings を priority 順に整理した。

## 4. Files changed

- `docs/reports/0134-review-current-uncommitted-diff-e11-and-parser-inventory.md`

`plan/` 更新不要。`progress.md` 更新不要。

## 5. Commands run and exact outputs

```text
git status --short --branch
## main...origin/main
 M Documentation.md
 M plan/06-surface-notation-status.md
 M plan/11-roadmap-near-term.md
 M plan/12-open-problems-and-risks.md
 M plan/13-heavy-future-workstreams.md
 M plan/90-source-traceability.md
 M progress.md
 M specs/00-document-map.md
?? docs/reports/0132-current-l2-first-parser-subset-inventory.md
?? docs/reports/0133-review-current-l2-first-parser-subset-inventory.md
?? specs/examples/29-current-l2-first-parser-subset-inventory.md
```

```text
cargo test -p mir-semantics perform_on_ensure_failure_returns_explicit_failure_without_non_admissible_metadata
...
running 1 test
test perform_on_ensure_failure_returns_explicit_failure_without_non_admissible_metadata ... ok
...
test result: ok. 1 passed; 0 failed
```

```text
cargo test -p mir-semantics perform_via_ensure_failure_can_continue_to_later_success
...
running 1 test
test perform_via_ensure_failure_can_continue_to_later_success ... ok
...
test result: ok. 1 passed; 0 failed
```

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 134 numbered report(s).
```

```text
git diff --check
```

無出力。

## 6. Evidence / findings

## Finding 1 — low — `docs/reports/0132` の docs validation evidence count が current uncommitted tree とずれている

- file: `docs/reports/0132-current-l2-first-parser-subset-inventory.md`
- 現在の uncommitted tree では `0132` と `0133` が同時に存在し、fresh な `python3 scripts/validate_docs.py` は `Found 133 numbered report(s).` を返す。
- しかし `0132` の evidence section には `Found 132 numbered report(s).` と書かれている。
- これは semantic regression ではないが、同一 uncommitted diff を review する読者にとっては reproducible evidence として 1 件ずれる。
- 0132 を「0133 追加前に実行した historical evidence」として残すのは可能だが、その場合は wording で time-of-task であることを補足した方が誤読を減らせる。

## Requested focus areas — no blocking findings

### (1) e11 fixture / host plan

- `e11-perform-via-ensure-then-success.json` は、
  - earlier option `delegated_writer`
  - later option `backup_writer`
  - request-local `ensure`
  - success-side effect preview
  - later same-lineage continuation
  を最小で表している。
- host plan sidecar も、
  - both options で `request-require = satisfied`
  - earlier `request-ensure = unsatisfied`
  - later `request-ensure = satisfied`
  - both options の effect verdict は `success`
  という narrow setup で、issue の核心だけを支えている。
- `lib.rs` の `eval_perform_via` は `ensure_ok == false` で `PerformFailure` を記録し、later candidate があれば `continue` し、commit は apply していない。fixture と実装の読みは一致している。

### (2) interpreter test coverage

- `perform_via_ensure_failure_can_continue_to_later_success()` は
  - `terminal_outcome == Success`
  - event sequence `PerformFailure, PerformSuccess`
  - formal `non_admissible_metadata` empty
  - narrative explanation empty
  - final `place_store == {"profile_doc": ["write_profile@backup_writer"]}`
  を押さえている。
- 最後の `place_store` assertion により、earlier delegated preview が破棄され、later success だけが commit されることは current PoC の observable state では十分 machine-check されている。

### (3) docs/reports/0126 and 0127 after the e10 reviewer fix

- `0126` は reviewer fix 後の current state と整合している。
  - focused regression が `DirectStyleEvaluator` の empty `place_store` まで確認すること
  - suite count 36
  - report numbering 127
  は current historical context と矛盾しない。
- `0127` も reviewer finding と follow-up が妥当で、overclaim は解消済みである。

### (4) consistency of `specs/examples/02`, `specs/examples/06`, `plan/08`, `progress.md`, and `plan/90`

- `specs/examples/02` は `e10` / `e11` の fixture catalog を current semantics に沿って記述している。
- `specs/examples/06` は
  - `e10` を direct-target ensure failure
  - `e11` を via-chain ensure continuation
  として分け、`tentative commit` suppression を含めて mirror できている。
- `plan/08` の fixture catalog / prose summary もこの reading に一致している。
- `progress.md` は e10 reviewer fix 後の current understanding と矛盾していない。
- `plan/90` も `plan/08` の根拠として `0126` / `0127` / `0128` / `0129` を列挙しており、traceability drift は見当たらない。

## 7. Changes in understanding

- 指定された e11 / e10 ensure regression と mirror 群については、current `HEAD` に semantic regression や boundary violationは見つからなかった。
- current uncommitted diff の本体は parser inventory docs であり、requested focus area の code / fixture / test には未コミット変更が入っていない。
- この review で実際に見つかったのは、parser inventory report の evidence count drift 候補 1 件だけである。

## 8. Open questions

- `docs/reports/0132-current-l2-first-parser-subset-inventory.md` の evidence count を current tree に合わせて更新するか、それとも「0133 追加前の historical evidence」と明示して残すか。

## 9. Suggested next prompt

`0132 current L2 first parser subset inventory report の evidence count drift を historical note として残すか、current tree の validation count に揃えるかを narrow に決めてください。`
