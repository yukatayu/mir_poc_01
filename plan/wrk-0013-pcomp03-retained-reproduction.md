# WRK-0013 P-COMP-03 retained reproduction

**LAB evidence.** This memo records one fresh reproduction and its exact
retention boundary. `mirrorea_canon/` remains normative. The memo does not
unfreeze WRK-0012 or select a carrier, semantic result, repair, OBL, Gate,
Phase, conformance claim, or public workflow.

## Authority and retention cut

- The pre-registration is
  `mirrorea_canon/working/WRK-0013-pcomp03-retention-reproduction.md` at
  `3043140e6111de902826031ed520c3371993b8ad`.
- The fresh command ran after that pushed registration in a clean detached
  checkout at `ac8e1f3b90e5d33baf025a66b415ce09fa103713`. The only change after
  the registration before that checkout was reader-facing status synchronization.
- The two immutable inputs remain the direct-world `package.mir.json` leaves
  from `2242901a44d3feb7708f82ff535d91bff4fbe143`: positive
  `af09bf1cf56c341b6f91e7572b0f20c67e8f1b9942730270bdf753fae0da1fa3` and
  negative `220452b11ea7410f889833e05ee9519b884036bd74b708cd4f401ef1bc5c41b1`.
  They are W13 inputs, not W13 evidence artifacts.
- Existing CLI/Rust execution machinery was unchanged. Generated JSON, session
  directories, and shell trace remain disposable external output. The retained
  artifacts of this evidence package are only this memo, its index entry, and
  R-2353.

## Fresh reproduction

The exact registered W13 command first rechecked both input SHA-256 values,
then ran the two Product Alpha `check` / `run-local` paths and its JSON
assertions. It returned exit 0.

| Input | Check result | Run result | Fresh output SHA-256 |
| --- | --- | --- | --- |
| `control-flow/positive/direct-world/package.mir.json` | `accepted` | `mir_computation_claimed: true`; one `sum_to` history entry with `Int(5)` to `Int(15)` | check `b6c8f9ef48c6ff085dc70bdb0b0bd40247233c8475a0caf219297d2ae30288a3`; run `00c98cc2e18d3af6db8f55b0993e982007f30b4262f730cf29d62d341763682c` |
| `variables-scope/negative/direct-world/package.mir.json` | `accepted` | `run-local` exit 2; `status: error`; `diagnostic_code: MirCompute`; message contains `UnboundVariable: unbound variable \`y\`` | check `54b81849e874254516207902a9ade658e0ec23fb867f9ff6d57c8655519e8537`; run `85d91e999b3222a689b6631535a2ff56461f365f058c7b5c2791891c52f9d2a9` |

The positive and negative registered classifications therefore reproduced from
fresh output. This is not an additional direct-carrier claim: the retained
question is provenance plus the already documented unnumbered retention path.

## Retention check and stop line

The intended evidence delta is exactly this memo,
`plan/00-index.md`, and R-2353. Before its commit, `git diff --check`,
`python3 scripts/validate_docs.py`,
`python3 scripts/check_source_hierarchy.py`, and Canon index validation must
all pass without validator/source-hierarchy, sidecar, helper, schema, script,
CI/Make, Rust, runtime, CLI, public-interface, or unrelated-plan changes.

The evidence is not manifested in WRK-0013 until a later append-only working
record update names this memo's exact evidence commit and digest. If that
manifest or the unchanged validation route fails, W13 freezes rather than
repairing W13/W12 or treating this memo as a broader runtime result.

## Non-claims and reopen condition

This retained fresh reproduction does not establish general P-COMP-03 coverage,
direct textual `.mir`, helper/sidecar equivalence, rejection-phase equivalence,
language completeness, runtime correctness, a defect, a required repair, a
public API, Canon carrier, contract/conformance status, OBL/SCN, Gate/Phase,
or workflow readiness. A distinct future question is required for any such
interpretation.
