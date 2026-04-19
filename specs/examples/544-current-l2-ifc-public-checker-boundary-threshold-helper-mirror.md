# 544 — current L2 IFC public-checker-boundary threshold helper mirror

## 目的

`specs/examples/283-current-l2-minimal-shared-output-contract-ready-public-checker-boundary-comparison.md`
と
`specs/examples/284-current-l2-public-checker-boundary-ready-minimal-public-checker-boundary-threshold.md`
では、

- public checker boundary comparison の first candidate を docs-only parser-front boundary に置くこと
- current minimum を
  `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref`
  に留めること

までは docs-first に整理済みである。

この文書の目的は、その compare-floor を verifier handoff surface や final parser grammar に上げることではなく、
**source-side IFC trio `p10 / p11 / p12` に対して
`actual_public_checker_boundary_threshold` として actualize する current cut**
を固定することにある。

ここで actualize するのは `mir-current-l2 run-source-sample` の helper-local summary だけであり、

- final parser grammar
- query token / `checker_subject` public naming
- generic shared public checker entry
- detached loop wrapper path line の public surface 昇格
- verifier handoff surface
- final public verifier contract

は still later に残す。

## 1. current question

`specs/examples/543` により source-side IFC trio `p10 / p11 / p12` は
sample-local `actual_shared_output_contract_threshold` まで actualize 済みである。

その次段として、
docs-only comparison に留まっていた public-checker-boundary line を、
**verifier handoff surface や final parser grammar に上げずに、
minimal public-checker-boundary ready sketch に限って operational CLI へ narrow に mirror してよいか**
が current question である。

## 2. current first line

current recommendation は次である。

1. helper-local threshold に留める
2. current source-side actualization 対象は `p10 / p11 / p12` だけに絞る
3. current minimal bundle は
   `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref`
   に留める
4. `boundary_kind` は `docs_only_parser_front_checker_boundary` に留める
5. `public_checker_command_surface_ref` は `public_checker_command_surface_ready_sketch` に留める
6. `shared_output_contract_ref` は `shared_output_contract_ready_sketch` に留める
7. `next_comparison_target_ref` は `verifier_handoff_surface_comparison` に留める
8. final parser grammar / query token naming / generic shared public checker entry / detached loop wrapper path line / verifier handoff surface / final public verifier contract は still later に残す
9. `p06` や order-handoff sample など、現行 IFC trio の外側は guard-only に留める

## 3. actualized helper reading

| sample | status | boundary_kind | command ref | output ref | current reading |
|---|---|---|---|---|---|
| `p10-typed-authorized-fingerprint-declassification` | `reached` | `docs_only_parser_front_checker_boundary` | `public_checker_command_surface_ready_sketch` | `shared_output_contract_ready_sketch` | IFC authority success sideでも public-checker-boundary minimum を helper-local summary に actualize する |
| `p11-typed-unauthorized-fingerprint-release` | `reached` | `docs_only_parser_front_checker_boundary` | `public_checker_command_surface_ready_sketch` | `shared_output_contract_ready_sketch` | authority miss negative sideでも同じ public-checker-boundary ready sketch を共有する |
| `p12-typed-classified-fingerprint-publication-block` | `reached` | `docs_only_parser_front_checker_boundary` | `public_checker_command_surface_ready_sketch` | `shared_output_contract_ready_sketch` | label-flow negative sideでも同じ public-checker-boundary ready sketch を共有する |

current helper-local cut の deferred surface ref は次に留める。

1. `final_parser_grammar`
2. `query_token_and_checker_subject_public_naming`
3. `generic_shared_public_checker_entry`
4. `detached_loop_wrapper_path_line`
5. `verifier_handoff_surface`

## 4. helper summary shape

current helper-local summary では、次の shape まで actualize してよい。

```text
actual_public_checker_boundary_threshold = {
  status = reached | guarded_not_reached,
  threshold_kind = checker_adjacent_public_checker_boundary_threshold_manifest,
  boundary_kind = docs_only_parser_front_checker_boundary,
  public_checker_command_surface_ref = public_checker_command_surface_ready_sketch,
  shared_output_contract_ref = shared_output_contract_ready_sketch,
  next_comparison_target_ref = verifier_handoff_surface_comparison,
  deferred_surface_refs = [
    final_parser_grammar,
    query_token_and_checker_subject_public_naming,
    generic_shared_public_checker_entry,
    detached_loop_wrapper_path_line,
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

1. これは `actual_shared_output_contract_threshold` の次段にある helper-local threshold である
2. current checker-side package を public-checker-boundary ready sketch の minimum として束ねる line であり、verifier handoff surface ではない
3. current summary は docs-only parser-front relation に留め、final parser grammar や query token public naming は still later に残す
4. current threshold は docs-first public-checker-boundary reading の helper-local mirror であり、final public parser/checker boundary を意味しない
5. current next promoted line は `verifier_handoff_surface_comparison` であり、actual emitted verifier handoff surface ではない

## 5. why this is enough

- `specs/examples/283` により、public checker boundary comparison の first candidate は docs-only parser-front boundary でよい
- `specs/examples/284` により、その minimum は `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` まででよい
- current repo では `actual_public_checker_command_surface_threshold` と `actual_shared_output_contract_threshold` が helper-local actualization 済みであり、その接点を narrow に mirror する条件が揃っている
- current checker-side line の immediate pressure は verifier handoff surface comparison へ進むことであり、final parser grammar や generic shared entry の adoption ではない

したがって current repo は、
verifier handoff surface や final parser grammar を still later に残したまま、
public-checker-boundary ready sketch を
helper-local operational CLI へ narrow に mirror してよい。

## 6. evidence

- checker-side docs-first bridge
  - `specs/examples/283`
  - `specs/examples/284`
  - `specs/examples/543`
- operational CLI actualization
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 7. stop line

この package の stop line は次である。

- `actual_public_checker_boundary_threshold` は helper-local / sample-local に留める
- current minimal bundle は docs-only parser-front boundary bundle で止める
- current IFC trio の outside は guard-only に保つ

この package は次を意味しない。

- final parser grammar
- query token / `checker_subject` public naming
- generic shared public checker entry
- detached loop wrapper path line の public surface 昇格
- verifier handoff surface
- final public verifier contract

## 8. retained later

- final parser grammar
- query token / `checker_subject` public naming
- generic shared public checker entry
- detached loop wrapper path line の public surface 昇格
- verifier handoff surface
- final public verifier contract
