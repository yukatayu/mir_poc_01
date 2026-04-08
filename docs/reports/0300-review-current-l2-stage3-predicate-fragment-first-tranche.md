# 0300 — review current L2 stage 3 predicate fragment first tranche

## Objective

`0299-current-l2-stage3-predicate-fragment-first-tranche.md` と対応する spec / plan / code change が、
current Phase 3 line、first checker cut inventory、request attachment deferred line を壊していないかを確認する。

## Scope and assumptions

- current L2 core semantics は変更しない。
- review は stage 3 predicate fragment branch とその mirror / helper-local actualization に限定する。

## Documents consulted

- `docs/reports/0299-current-l2-stage3-predicate-fragment-first-tranche.md`
- `specs/examples/93-current-l2-stage3-predicate-fragment-boundary-comparison.md`
- `specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs`

## Actions taken

1. reviewer subagent を 1 回だけ起動した。
2. 1 回目の long wait は timeout したが、repo ルールどおり retry は 1 回だけに留めた。
3. 2 回目の long wait で reviewer completion を取得し、その finding を local evidence と照合した。

## Evidence / outputs / test results

### reviewer handling

- reviewer subagent は 1 回だけ起動した。
- 1 回目の long wait は timeout した。
- 2 回目の long wait で completion を取得した。

### fresh evidence

```text
$ cargo test -p mir-ast --test current_l2_stage3_predicate_fragment_spike
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```text
$ cargo test -p mir-ast
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 300 numbered report(s).
```

```text
$ git diff --check
[no output]
```

### findings

- **No substantive finding**
  - [93-current-l2-stage3-predicate-fragment-boundary-comparison.md](/home/yukatayu/dev/mir_poc_01/specs/examples/93-current-l2-stage3-predicate-fragment-boundary-comparison.md) は、predicate fragment boundary を declaration-side only compare や opaque retention へ寄せず、shared isolated helper に切ることで first checker cut の `minimal predicate fragment well-formedness` と素直に接続している。
  - [94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md](/home/yukatayu/dev/mir_poc_01/specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md) と [current_l2_stage3_predicate_fragment_spike_support.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs) / [current_l2_stage3_predicate_fragment_spike.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs) は、isolated fragment string only / fixture-side subset compare only に留まっており、request head attachment や multiline suite を hidden に持ち込んでいない。
  - mirror 側の [Documentation.md](/home/yukatayu/dev/mir_poc_01/Documentation.md)、[00-document-map.md](/home/yukatayu/dev/mir_poc_01/specs/00-document-map.md)、[07-parser-free-poc-stack.md](/home/yukatayu/dev/mir_poc_01/plan/07-parser-free-poc-stack.md)、[11-roadmap-near-term.md](/home/yukatayu/dev/mir_poc_01/plan/11-roadmap-near-term.md)、[12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md)、[90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md)、[progress.md](/home/yukatayu/dev/mir_poc_01/progress.md) も、この cut を private/test-only first tranche として mirror しているだけで、Phase 3 mainline を壊す drift は見当たらない。

### residual risk

- reviewer が挙げた residual risk は次の 2 点である。
  - `collect_performs` は current working-set fixture には十分だが、`body` 再帰と `perform_index` 依存なので later tranche で incidental break の余地がある。
  - helper-local fragment parser は grouping を surface retention しないため、malformed-source や diagnostics wording を扱う段で compare contract の明文化をもう一段足した方が安全である。

## What changed in understanding

- reviewer completion により、今回の cut が
  - shared floor
  - request attachment deferred
  - private/test-only helper
  の 3 点を守っていることを第三者視点でも確認できた。

## Open questions

- predicate fragment helper の malformed-source pair を先に actualize するか、それとも request head + clause attachment multiline shape の docs-only comparison を先に開くか。

## Suggested next prompt

`Phase 3 の次段として、predicate fragment helper の malformed-source pair を先に actualize するか、それとも request head + clause attachment multiline shape の docs-only comparison を先に開くかを比較してください。`
