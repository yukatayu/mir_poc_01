# 541 — current L2 IFC public-checker-entry-criteria threshold helper mirror

## 目的

`specs/examples/277-current-l2-minimal-public-checker-api-ready-public-checker-entry-criteria-comparison.md`
と
`specs/examples/278-current-l2-public-checker-entry-criteria-ready-minimal-public-checker-entry-criteria-threshold.md`
では、

- public checker comparison 専用の entry criteria を別段に切ること
- current minimum を
  `minimal API fixed + command-surface pressure present + comparison target narrowed + heavier boundary deferred`
  に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を actual command surface や shared output contract に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して
`actual_public_checker_entry_criteria_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- actual public checker command surface
- shared output contract
- parser-front public checker boundary
- emitted verifier handoff surface
- final public verifier contract

は still later に残す。

## 1. current question

`specs/examples/540` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_public_checker_api_sketch_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた public-checker entry criteria line を、
**actual command surface や parser-front public checker boundary に上げずに、
entry criteria minimum に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimal bundle は
   `public_checker_api_ref + entry_criteria_refs + family_facade_support_ref + family_facade_script_refs + smoke_command_refs + next_comparison_target_ref + deferred_boundary_refs`
   に留める
4. `public_checker_api_ref` は `public_checker_api_ready_sketch` に留める
5. `entry_criteria_refs` は
   `minimal_api_fixed`、
   `command_surface_pressure_present`、
   `comparison_target_narrowed`、
   `heavier_boundary_deferred`
   の 4 条件に留める
6. `family_facade_support_ref` は `scripts/current_l2_family_checker_support.py` に留める
7. `family_facade_script_refs` は
   `scripts/current_l2_same_lineage_checker.py`、
   `scripts/current_l2_missing_option_checker.py`、
   `scripts/current_l2_capability_checker.py`
   に留める
8. `smoke_command_refs` は
   `smoke-same-lineage-checker`、
   `smoke-missing-option-checker`、
   `smoke-capability-checker`
   に留める
9. `next_comparison_target_ref` は `public_checker_command_surface_comparison` に留める
10. `deferred_boundary_refs` は
    `shared_output_contract`、
    `parser_front_public_checker_boundary`、
    `verifier_handoff_surface`
    に留める
11. actual command surface / shared output contract / parser-front public checker boundary / emitted verifier handoff surface / final public verifier contract は still later に残す
12. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | public_checker_api_ref | next_comparison_target_ref | current reading |
|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `public_checker_api_ready_sketch` | `public_checker_command_surface_comparison` | IFC authority success sideでも public-checker entry criteria minimum を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `public_checker_api_ready_sketch` | `public_checker_command_surface_comparison` | authority miss negative sideでも同じ entry criteria minimum を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `public_checker_api_ready_sketch` | `public_checker_command_surface_comparison` | label-flow negative sideでも同じ entry criteria minimum を共有する |

current helper-local cut の source-backed pressure は次に留める。

1. `family_facade_support_ref = scripts/current_l2_family_checker_support.py`
2. `family_facade_script_refs = [same_lineage_checker, missing_option_checker, capability_checker]`
3. `smoke_command_refs = [smoke-same-lineage-checker, smoke-missing-option-checker, smoke-capability-checker]`

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_public_checker_entry_criteria_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_public_checker_entry_criteria_threshold_manifest,
  public_checker_api_ref = public_checker_api_ready_sketch,
  entry_criteria_refs = [
    public_checker_entry_criteria:minimal_api_fixed,
    public_checker_entry_criteria:command_surface_pressure_present,
    public_checker_entry_criteria:comparison_target_narrowed,
    public_checker_entry_criteria:heavier_boundary_deferred
  ],
  family_facade_support_ref = scripts/current_l2_family_checker_support.py,
  family_facade_script_refs = [...],
  smoke_command_refs = [...],
  next_comparison_target_ref = public_checker_command_surface_comparison,
  deferred_boundary_refs = [
    shared_output_contract,
    parser_front_public_checker_boundary,
    verifier_handoff_surface
  ],
  evidence_refs = [...],
  compare_floor_refs = [...],
  guard_refs = [...],
  kept_later_refs = [...],
  guard_reason = ...
}
```

重要なのは次の 5 点である。

1. これは `actual_public_checker_api_sketch_threshold` の次段にある helper-local threshold である
2. current checker-side package を public-looking reopen condition の minimum として束ねる line であり、actual command surface ではない
3. current summary は entry criteria minimum に留め、family facade command bundle や shared output contract はまだ actual field に上げない
4. current threshold は docs-first public-checker entry criteria reading の helper-local mirror であり、actual public checker command surface を意味しない
5. detached-loop smoke command は source-backed pressure として持つが、current package では public command surface ready sketch にはまだ昇格させない

## 5. why this is enough

- `specs/examples/277` により、public checker API minimal shape の次段として public checker comparison 専用の entry criteria を別段に切ってよい
- `specs/examples/278` により、その minimum は docs-only API fixed だけでなく、source-backed family-facade command-surface pressure と heavier boundary deferred を含めてよい
- current repo では `scripts/current_l2_family_checker_support.py`、3 family facade script、detached-loop smoke family が already source-backed に存在している
- `actual_public_checker_api_sketch_threshold` が helper-local actualization 済みであり、IFC trio を current checker-side bridge representative corpus に使ってよい

したがって current repo は、
actual command surface や shared output contract を still later に残したまま、
public-checker entry criteria minimum を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- checker-side docs-first bridge
  - `specs/examples/277`
  - `specs/examples/278`
  - `specs/examples/540`
- source-backed pressure
  - `scripts/current_l2_family_checker_support.py`
  - `scripts/current_l2_same_lineage_checker.py`
  - `scripts/current_l2_missing_option_checker.py`
  - `scripts/current_l2_capability_checker.py`
  - `scripts/current_l2_detached_loop.py`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_public_checker_entry_criteria_threshold` は helper-local / sample-local に留める
- current minimal bundle は entry criteria minimum で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- actual public checker command surface
- shared output contract
- parser-front public checker boundary
- emitted verifier handoff surface
- final public verifier contract

## 8. retained later

- actual public checker command surface
- shared output contract
- parser-front public checker boundary
- emitted verifier handoff surface
- final public verifier contract
