# 549 — current L2 IFC phase2-parser-free-poc-closeout threshold helper mirror

## 目的

`specs/examples/293-current-l2-phase1-semantics-closeout-ready-phase2-parser-free-poc-closeout-comparison.md`
と
`specs/examples/294-current-l2-phase2-parser-free-poc-closeout-ready-minimal-phase2-parser-free-poc-closeout-threshold.md`
では、

- current L2 parser-free PoC closeout を narrow closeout bundle で読むこと
- current minimum を
  `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs`
  に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を final retention/path policy や public exporter API に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して
`actual_phase2_parser_free_poc_closeout_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- reference update / bless workflow
- final retention/path policy
- public exporter API
- production host interface

は still later に残す。

## 1. current question

`specs/examples/548` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_phase1_semantics_closeout_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた Phase 2 parser-free PoC closeout line を、
**reference update / bless workflow や public exporter API に上げずに、
compile gate + helper boundary + detached loop policy minimum に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimum は
   `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs`
   に留める
4. `closeout_kind` は `parser_free_companion_baseline` に留める
5. compile gate refs は
   `interpreter_regression_suite`
   `detached_loop_unit_suite`
   `detached_example_compile_gate`
   `runtime_smoke_fixture_gate`
   `single_fixture_aggregate_compare_gate`
   `static_gate_checker_smoke_gate`
   に留める
6. helper boundary refs は
   `bundle_runtime_path`
   `aggregate_compare_convenience`
   `static_gate_checker_smoke_family`
   `display_only_authoring_assists`
   に留める
7. detached loop policy refs は
   `compare_only_non_production_helper`
   `target_current_l2_detached_default_candidate`
   `diff_exit_code_one_is_informational`
   に留める
8. `next_comparison_target_ref` は `phase4_shared_space_self_driven_closeout_comparison` に留める
9. reference update / bless workflow / final retention path policy / public exporter API / production host interface は still later に残す
10. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | closeout kind | compile gate | helper boundary | detached loop policy | current reading |
|---|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `parser_free_companion_baseline` | interpreter / detached loop / smoke gate baseline | bundle / aggregate / static-gate-side checker / authoring assist split | compare-only non-production helper + default detached target candidate | IFC authority success sideでも Phase 2 parser-free closeout minimum を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `parser_free_companion_baseline` | interpreter / detached loop / smoke gate baseline | bundle / aggregate / static-gate-side checker / authoring assist split | compare-only non-production helper + default detached target candidate | authority miss negative sideでも同じ closeout minimum を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `parser_free_companion_baseline` | interpreter / detached loop / smoke gate baseline | bundle / aggregate / static-gate-side checker / authoring assist split | compare-only non-production helper + default detached target candidate | label-flow negative sideでも同じ closeout minimum を共有する |

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_phase2_parser_free_poc_closeout_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = phase2_parser_free_poc_closeout_threshold_manifest,
  closeout_kind = parser_free_companion_baseline,
  compile_gate_refs = [
    interpreter_regression_suite,
    detached_loop_unit_suite,
    detached_example_compile_gate,
    runtime_smoke_fixture_gate,
    single_fixture_aggregate_compare_gate,
    static_gate_checker_smoke_gate
  ],
  helper_boundary_refs = [
    bundle_runtime_path,
    aggregate_compare_convenience,
    static_gate_checker_smoke_family,
    display_only_authoring_assists
  ],
  detached_loop_policy_refs = [
    compare_only_non_production_helper,
    target_current_l2_detached_default_candidate,
    diff_exit_code_one_is_informational
  ],
  next_comparison_target_ref = phase4_shared_space_self_driven_closeout_comparison,
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 5 点である。

1. これは `actual_phase1_semantics_closeout_threshold` の次段にある helper-local threshold である
2. current parser-free closeout を compile gate + helper boundary + detached loop policy minimum に留める line であり、reference update / bless workflow や public exporter API ではない
3. current summary は reference update / bless workflow、final retention/path policy、public exporter API、production host interface を still later に残す
4. current threshold は docs-first Phase 2 closeout reading の helper-local mirror であり、Phase 4 shared-space self-driven closeout そのものではない
5. current next promoted line は `phase4_shared_space_self_driven_closeout_comparison` であり、運用 workflow / public API finalization ではない

## 5. why this is enough

- `specs/examples/293` により、Phase 2 closeout は narrow closeout bundle でよい
- `specs/examples/294` により、その minimum は `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs + retained_later_refs` まででよい
- current repo では `actual_phase1_semantics_closeout_threshold` が helper-local actualization 済みであり、その次段を compile gate / helper boundary / detached loop policy minimum に留める条件が揃っている
- current closeout line の immediate pressure は Phase 4 shared-space self-driven closeout comparison へ進むことであり、reference update / bless workflow や public exporter API の adoption ではない

したがって current repo は、
reference update / bless workflow、final retention/path policy、public exporter API、production host interface 群を still later に残したまま、
phase2 parser-free PoC closeout ready sketch を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- phase2 closeout-side docs-first bridge
  - `specs/examples/293`
  - `specs/examples/294`
  - `specs/examples/548`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_phase2_parser_free_poc_closeout_threshold` は helper-local / sample-local に留める
- current minimum は compile gate + helper boundary + detached loop policy で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- reference update / bless workflow
- final retention/path policy
- public exporter API
- production host interface

## 8. retained later

- reference update / bless workflow
- final retention/path policy
- public exporter API
- production host interface
