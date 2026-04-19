# 542 — current L2 IFC public-checker-command-surface threshold helper mirror

## 目的

`specs/examples/279-current-l2-minimal-public-checker-entry-criteria-ready-public-checker-command-surface-comparison.md`
と
`specs/examples/280-current-l2-public-checker-command-surface-ready-minimal-public-checker-command-surface-threshold.md`
では、

- public checker command surface comparison の first candidate を existing family facade pattern に置くこと
- current minimum を
  `command_surface_kind + family_facade_command_refs + public_checker_api_ref`
  に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を shared output contract や parser-front public checker boundary に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して
`actual_public_checker_command_surface_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- detached-loop `smoke-*` wrapper の public surface 昇格
- generic shared public checker entry
- shared output contract
- parser-front public checker boundary
- emitted verifier handoff surface
- final public verifier contract

は still later に残す。

## 1. current question

`specs/examples/541` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_public_checker_entry_criteria_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた public-checker command surface line を、
**shared output contract や parser-front public checker boundary に上げずに、
minimal command-surface ready sketch に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimal bundle は
   `command_surface_kind + family_facade_command_refs + public_checker_api_ref`
   に留める
4. `command_surface_kind` は `family_facade_checker_commands` に留める
5. `family_facade_command_refs` は
   `same_lineage_checker_command`、
   `missing_option_checker_command`、
   `capability_checker_command`
   に留める
6. `public_checker_api_ref` は `public_checker_api_ready_sketch` に留める
7. `next_comparison_target_ref` は `shared_output_contract_comparison` に留める
8. detached-loop `smoke-*` wrapper は source-backed evidence として保持するが、current shape の first-class field には上げない
9. generic shared public checker entry / shared output contract / parser-front public checker boundary / emitted verifier handoff surface / final public verifier contract は still later に残す
10. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | command_surface_kind | public_checker_api_ref | current reading |
|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `family_facade_checker_commands` | `public_checker_api_ready_sketch` | IFC authority success sideでも minimal command-surface ready sketch を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `family_facade_checker_commands` | `public_checker_api_ready_sketch` | authority miss negative sideでも同じ command-surface ready sketch を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `family_facade_checker_commands` | `public_checker_api_ready_sketch` | label-flow negative sideでも同じ command-surface ready sketch を共有する |

current helper-local cut の family facade command ref は次に留める。

1. `same_lineage_checker_command`
2. `missing_option_checker_command`
3. `capability_checker_command`

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_public_checker_command_surface_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_public_checker_command_surface_threshold_manifest,
  command_surface_kind = family_facade_checker_commands,
  family_facade_command_refs = [
    same_lineage_checker_command,
    missing_option_checker_command,
    capability_checker_command
  ],
  public_checker_api_ref = public_checker_api_ready_sketch,
  next_comparison_target_ref = shared_output_contract_comparison,
  deferred_surface_refs = [
    detached_loop_smoke_same_lineage_checker,
    detached_loop_smoke_missing_option_checker,
    detached_loop_smoke_capability_checker,
    generic_shared_public_checker_entry,
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

1. これは `actual_public_checker_entry_criteria_threshold` の次段にある helper-local threshold である
2. current checker-side package を command-surface ready sketch の minimum として束ねる line であり、shared output contract ではない
3. current summary は family facade bundle に留め、detached-loop `smoke-*` wrapper や generic shared public checker entry は still later に残す
4. current threshold は docs-first public-checker command surface reading の helper-local mirror であり、final public command surface を意味しない
5. current next promoted line は `shared_output_contract_comparison` であり、parser-front public checker boundary ではない

## 5. why this is enough

- `specs/examples/279` により、public checker command surface comparison の first candidate は existing family facade pattern でよい
- `specs/examples/280` により、その minimum は `command_surface_kind + family_facade_command_refs + public_checker_api_ref` まででよい
- current repo では family facade script 群と detached-loop `smoke-*` family が already source-backed に存在している
- `actual_public_checker_entry_criteria_threshold` が helper-local actualization 済みであり、IFC trio を current checker-side representative corpus に使ってよい

したがって current repo は、
shared output contract や parser-front public checker boundary を still later に残したまま、
public-checker command surface ready sketch を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- checker-side docs-first bridge
  - `specs/examples/279`
  - `specs/examples/280`
  - `specs/examples/541`
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

- `actual_public_checker_command_surface_threshold` は helper-local / sample-local に留める
- current minimal bundle は family facade command bundle で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- detached-loop `smoke-*` wrapper の public surface 昇格
- generic shared public checker entry
- shared output contract
- parser-front public checker boundary
- emitted verifier handoff surface
- final public verifier contract

## 8. retained later

- detached-loop `smoke-*` wrapper の public surface 昇格
- generic shared public checker entry
- shared output contract
- parser-front public checker boundary
- emitted verifier handoff surface
- final public verifier contract
