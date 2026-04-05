# Report 0151 — checked reasons assist reconciliation

- Date: 2026-04-05T23:10:00+09:00
- Author / agent: Codex
- Scope: current L2 `checked_reasons` display-only assist の existing spec / scaffold follow-up reminder / progress snapshot / traceability を整合させる
- Decision levels touched: L2

## 1. Objective

- 既存の `specs/examples/36-current-l2-checked-reasons-authoring-assist.md` と current helper behavior の factual trail を整合させる。
- static-only scaffold が detached static gate assist への follow-up reminder を出す current behavior を spec / test / report で一致させる。
- `progress.md`、`plan/90-source-traceability.md`、report evidence を current task close に合わせて更新する。

## 2. Scope and assumptions

- current L2 semantics、parser grammar、failure family、machine-check policy は変えない。
- `checked_reasons` assist は display-only のまま維持し、fixture JSON の auto-fill は導入しない。
- current task-start dirty state には、assist 周辺の途中差分と report stub が含まれていた。
- `plan/` 本体の semantics / roadmap 判断は変えないが、traceability mirror は current task の report を反映するため更新する。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `progress.md`
- `specs/examples/27-current-l2-fixture-scaffold-helper.md`
- `specs/examples/32-current-l2-static-gate-artifact-loop.md`
- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
- `scripts/current_l2_scaffold_fixture.py`
- `scripts/current_l2_checked_reasons_assist.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_scaffold_fixture.py`
- `scripts/tests/test_current_l2_checked_reasons_assist.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`

## 4. Actions taken

1. current repo の docs / progress / helper state を再読し、display-only assist judgment が既に `Documentation.md`、`specs/00-document-map.md`、`plan/` に mirror されていることを確認した。
2. `specs/examples/36-current-l2-checked-reasons-authoring-assist.md` の current text と `scripts/current_l2_scaffold_fixture.py` の current behavior を照合し、static-only scaffold follow-up reminder の位置づけを既存 spec 側へ補足した。
3. `specs/examples/27-current-l2-fixture-scaffold-helper.md` を更新し、static-only scaffold で follow-up reminder を stderr に出してよい current cut と、copyable command のための shell-safe quoting 要件を追記した。
4. `scripts/tests/test_current_l2_scaffold_fixture.py` に quoted-path case を追加して RED を確認し、その後 `scripts/current_l2_scaffold_fixture.py` を最小修正して reminder command が shell-safe quoting を使うようにした。runtime scaffold では reminder を出さないことも維持した。
5. `progress.md` の最終更新行と作業ログ（簡潔）を current factual trail に合わせて更新した。
6. `plan/90-source-traceability.md` に current report `0151` を追加し、`plan/07` と `plan/15` の traceability mirror を current worktree と一致させた。
7. code mapper subagent を 180s x 2 回待ったが completion が返らず、その後の遅延 notification は local inspection と整合したため primary evidence は local reading とした。

## 5. Files changed

- `specs/examples/27-current-l2-fixture-scaffold-helper.md`
- `specs/examples/36-current-l2-checked-reasons-authoring-assist.md`
- `scripts/current_l2_scaffold_fixture.py`
- `scripts/tests/test_current_l2_scaffold_fixture.py`
- `progress.md`
- `plan/90-source-traceability.md`
- `docs/reports/0151-current-l2-checked-reasons-assist-reconciliation.md`
- `docs/reports/0152-review-current-l2-checked-reasons-assist-reconciliation.md`

## 6. Commands run and exact outputs

```text
$ python3 -m unittest scripts.tests.test_current_l2_checked_reasons_assist scripts.tests.test_current_l2_static_gate_loop scripts.tests.test_current_l2_scaffold_fixture
...static gate artifact: /tmp/tmpd0ia9rwk/artifacts/static-gates/left-run/left.static-gate.json
reference static gate artifact: /tmp/tmpd0ia9rwk/artifacts/static-gates/right-run/right.static-gate.json
..static gate artifact: /tmp/tmpl4pwfcui/artifacts/static-gates/assist-run/left.static-gate.json
.......
----------------------------------------------------------------------
Ran 12 tests in 0.008s
OK
```

```text
$ python3 scripts/current_l2_detached_loop.py suggest-checked-reasons crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --run-label suggest-e4 --overwrite
static gate artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/static-gates/suggest-e4/e4-malformed-lineage.static-gate.json
fixture: crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json
artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/static-gates/suggest-e4/e4-malformed-lineage.static-gate.json
current expected_static.checked_reasons: ["lineage assertion does not describe primary -> mirror"]
current expected_static.checked_reasons already matches actual static gate reasons
```

```text
$ python3 scripts/current_l2_detached_loop.py suggest-checked-reasons crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --run-label suggest-e3 --overwrite
static gate artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/static-gates/suggest-e3/e3-option-admit-chain.static-gate.json
fixture: crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json
artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/static-gates/suggest-e3/e3-option-admit-chain.static-gate.json
current expected_static.checked_reasons: absent
actual static gate reasons are empty; no checked_reasons suggestion
```

```text
$ cargo test -p mir-semantics
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.03s
...
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
...
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 152 numbered report(s).
```

```text
$ git diff --check
(no output)
```

## 7. Evidence / outputs / test results

- repo 内の docs / plan / progress / helper 参照は、display-only assist を既に前提にしており、今回 task の主眼は current spec / helper / report / traceability の factual trail を揃えることだった。
- `scripts/current_l2_scaffold_fixture.py` は既に static-only scaffold follow-up reminder を持っていたため、今回 task ではその current behavior を spec と test へ整合させるのが主眼になった。
- quoted-path case を test で先に落とし、reminder command が shell-safe quoting を使うように最小修正した。
- `e4` では current `checked_reasons` と actual wording が一致しており、assist は no-op で十分だった。
- `e3` では actual static gate `reasons` が空で、assist が no-suggestion で止まる current non-adoption policy と整合した。

## 8. What changed in understanding

- `checked_reasons` assist の current settled judgment は既に repo-wide に mirror されており、今回の主要課題は current helper behavior / report / progress / traceability の整合だった。
- scaffold helper の follow-up reminder は semantics inference ではなく、display-only assist への operational hint として扱えば current L2 boundary を壊さない。
- `plan/` 自体の current understanding は変えていないが、report 追加に伴う `plan/90-source-traceability.md` の mirror 更新は必要だった。

## 9. Open questions

- `suggest-checked-reasons` を detached validation loop の default smoke flow に寄せるか。
- helper-local `reason_codes` mirror を assist 表示へ将来どこまで使うか。
- `checked_reasons` を fixture-side typed row へ昇格させる timing。
- code mapper subagent は timeout で completion が得られなかった。reviewer は別途 1 回だけ待つ。
- `plan/90-source-traceability.md` 以外の `plan/` へ current semantics / roadmap mirror 変更を広げる必要があるか。

## 10. Suggested next prompt

- `checked_reasons` display-only assist を 2〜3 本の新しい static-only malformed / underdeclared fixture authoring で実地に回し、どこで still-manual friction が残るかを narrow に棚卸ししてください。
