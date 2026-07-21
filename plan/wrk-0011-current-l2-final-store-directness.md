# WRK-0011 current-L2 final-store assertion directness

**LAB evidence.** This record classifies only the literal provenance of named
existing assertions. It does not make a Canon state-semantics, runtime
correctness, source/fixture equivalence, defect, coverage-requirement,
theorem/OBL, carrier, or repair claim.

## Authority and retention cut

- The pre-registration is
  `mirrorea_canon/working/WRK-0011-current-l2-final-store-directness.md` at
  `fa130a499cecca20c625663e4ad20872ef192d67`.
- The source-text inspection is pinned to
  `6297b9e6d60b8d4f02bd2efa744beb15648d9e53`. The `fa130a49` registration
  changes only the WRK/MAP/index/report operational metadata, not `crates/`,
  `scripts/`, or `samples/`.
- `crates/` tests and `scripts/` are unmodified, pinned execution/inspection
  machinery. They are not retained LAB inputs or artifacts. The retained
  artifact is this `plan/` file; generated outputs remain under `/tmp`.

## Directness rule

For this record only, `direct source-route final-store assertion` means an
existing equality assertion over the exact `run_report.final_place_store` from
the source-derived report. A source fixture name, host-plan dependency,
success/trace assertion, formal-hook identity, or a separate direct-evaluator
state assertion does not satisfy that rule.

## Authoritative reproduction

The registered command ran in the clean detached worktree
`/tmp/mirrorea-wrk0011-clean-20260722084732-2363026` at
`fa130a499cecca20c625663e4ad20872ef192d67`. Its disposable regression output
root was `/tmp/mirrorea-wrk0011-final-store-directness-authoritative.sLJtiZ`.
Neither directory is retained evidence.

The pinned `git grep` found no `final_place_store` reference in the registered
source-route test files:

- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`

All six named focused tests passed, each with one selected test:

1. `current_l2_source_lowering_matches_e21_fixture_and_try_atomic_cut_frontier`
2. `current_l2_source_lowering_matches_e22_fixture_and_nested_place_atomic_cut_mismatch`
3. `verification_ladder_marks_e21_as_runtime_and_formal_hook_reached`
4. `verification_ladder_marks_e22_as_runtime_and_formal_hook_reached`
5. `try_body_atomic_cut_updates_rollback_frontier_without_skipping_fallback`
6. `nested_place_atomic_cut_does_not_update_rollback_frontier`

The unchanged `current_l2_source_sample_regression.py regression` then passed
all 23 commands. This is runnability/compatibility evidence only, not a claim
about either expected store's meaning.

## Literal assertion matrix

| Subject | Exact registered source body | Literal assertion form | Classification |
| --- | --- | --- | --- |
| e21 source route | `current_l2_source_lowering_matches_e21_fixture_and_try_atomic_cut_frontier` and `verification_ladder_marks_e21_as_runtime_and_formal_hook_reached` | Reads `e21-try-atomic-cut-frontier.txt`, lowers/runs it, then asserts structural/static validity, entered evaluation, successful terminal outcome, and event sequence. The ladder separately builds a hook from fixture `run_bundle` and checks its subject identity. No exact `run_report.final_place_store` equality occurs. | source-route direct final-store assertion absent in the named bodies |
| e22 source route | `current_l2_source_lowering_matches_e22_fixture_and_nested_place_atomic_cut_mismatch` and `verification_ladder_marks_e22_as_runtime_and_formal_hook_reached` | Reads `e22-try-atomic-cut-place-mismatch.txt`, lowers/runs it, then asserts structural/static validity, entered evaluation, successful terminal outcome, and event sequence. The ladder separately builds a hook from fixture `run_bundle` and checks its subject identity. No exact `run_report.final_place_store` equality occurs. | source-route direct final-store assertion absent in the named bodies |
| e21 fixture/direct evaluator | `try_body_atomic_cut_updates_rollback_frontier_without_skipping_fallback` | Separately calls `run_direct_evaluator_with_plan` and directly compares `evaluator.state.place_store` with a two-entry `BTreeMap`. This is not the source-derived `RunReport` assertion in the rule above. | exact direct-evaluator state assertion present outside the source route |
| e22 fixture/direct evaluator | `nested_place_atomic_cut_does_not_update_rollback_frontier` | Separately calls `run_direct_evaluator_with_plan` and directly compares `evaluator.state.place_store` with a one-entry `BTreeMap`. This is not the source-derived `RunReport` assertion in the rule above. | exact direct-evaluator state assertion present outside the source route |

## Result and stop line

**Result class: literal assertion provenance.** In the registered named
source-route bodies, neither e21 nor e22 directly asserts the exact final
store carried by `RunReport`. The two named direct fixture/evaluator bodies do
directly assert their evaluator state's exact `PlaceStore` values.

This result stops here. It does not say whether either state is correct, what
Place/cut/rollback means, whether a source route ought to assert a store,
whether a test is sufficient, or whether any schema, helper, CLI, fixture,
test, runtime, theorem, or Canon document should change.

## Reopen condition

Only a separately pre-registered question may examine a different existing
assertion binding or any semantic/coverage/repair issue. It must not infer a
source-route equality from the direct-evaluator assertions recorded here.
