# 0162 — review current L2 target-mismatch tranche

## Objective

`0161` で行った `declared_target_mismatch` static-only tranche について、
stable malformed cluster の actualization が existing helper boundary を壊していないかを reviewer で確認し、
指摘があれば task 内で収束させる。

## Scope and assumptions

- review 対象は `e19-malformed-target-mismatch` fixture 追加と、それに伴う tests / docs / plan / progress の更新に限る。
- runtime semantics、parser grammar、failure family は変更していない。
- reviewer は 1 回起動し、180s wait + 1 retry を行った。

## Documents consulted

1. `docs/reports/0161-static-only-target-mismatch-tranche.md`
2. `specs/examples/00-representative-mir-programs.md`
3. `specs/examples/02-current-l2-ast-fixture-schema.md`
4. `plan/01-status-at-a-glance.md`
5. `plan/08-representative-programs-and-fixtures.md`
6. `plan/11-roadmap-near-term.md`
7. `plan/15-current-l2-fixture-authoring-template.md`
8. `plan/90-source-traceability.md`
9. `progress.md`
10. `crates/mir-semantics/src/lib.rs`
11. `crates/mir-semantics/src/harness.rs`
12. `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
13. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
14. `crates/mir-semantics/tests/current_l2_static_gate_support.rs`

## Actions taken

1. reviewer agent `Lovelace` を起動し、working tree diff と主要更新ファイルを review させた。
2. reviewer からは 2 件の hygiene finding が返った。
   - `plan/90-source-traceability.md` が今回の tranche (`0161` / `0162`) まで伸びていなかった
   - `0161` の files changed / report count が final tree とずれていた
3. 上記 2 件を task 内で修正した。
4. reviewer の semantic assessment を確認し、コード側には no findings だったことを記録した。

## Files changed

- `plan/90-source-traceability.md`
- `docs/reports/0161-static-only-target-mismatch-tranche.md`
- `docs/reports/0162-review-current-l2-target-mismatch-tranche.md`

## Evidence / outputs / test results

- reviewer summary:
  - Medium: `plan/90` の traceability drift
  - Medium: `0161` の report evidence drift
  - semantic / helper-boundary / count-matrix / regression については no findings
- reviewer が確認したもの:
  - `e19` の actual wording は `crates/mir-semantics/src/lib.rs` の checker 実装と一致
  - `reason_codes` は detached non-core mirror に留まる
  - selection / profile / count matrix は coherent
- post-fix local verify:
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## What changed in understanding

- `declared_target_mismatch` tranche 自体の semantic boundary は妥当で、review でも実質的な問題は出なかった。
- 今回の修正点は traceability と report hygiene に限られた。
- same-lineage static stable cluster の actual corpus 化は、current docs / tests / detached loop / plan mirror の整合を保ったまま進められることが補強された。

## Open questions

- current stable cluster 一巡後に、typed carrier の first-class actualization を先に始めるか、fixture authoring 実地反復をもう少し積むか。
- duplicate declaration cluster を display-only actual wording のままどこまで維持するか。

## Suggested next prompt

```text
current L2 parser-free PoC を前提に、static gate helper-local `reason_codes` mirror を first-class typed carrier に昇格させる最小 migration cut を narrow に actualize してください。
current exact-compare core と `checked_reasons` bridge を壊さず、non-production helper / detached artifact / tests / docs / plan / progress を一緒に更新し、review と push まで進めてください。
```
