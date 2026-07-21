# plan/167 - P-COMP-03 rejection-phase cross-carrier audit

## Role and authority

This is LAB repository memory. `mirrorea_canon/` remains authoritative for
language direction, static-versus-dynamic semantics, effects, failures,
contracts, Gates, Phases, conformance, and implementation authority. This
audit neither establishes Canon conformance nor changes the meaning of
`Reject`.

## Question

For each negative P-COMP-03 case, what is the first rejection phase in its
checked-in helper fixture and in the closed Rust semantic registry, and how is
the latter carried by the Product Alpha session runtime?

## Reproduced boundary

On 2026-07-22, the computational matrix and `check-all` still passed all 15
rows: seven accepted, five helper-labelled `runtime_rejection`, and three
package-check rejections. The five P-COMP-03 fixture inputs are
`computational_helper_row` packages with `helper_package_runtime` as their
execution surface. Their executable path reads the manifest and calls Python
`_evaluate_helper_compute`; it does not call Product Alpha schema checking,
the Rust typechecker, or the Rust evaluator.

The separate Product Alpha runtime tests construct valid `world` packages and
use the closed `mir-semantics::computational_core` registry. That route calls
`typecheck_module` before `eval_function`; `eval_function` itself starts by
typechecking again before it can enter `eval_function_impl`.

## Phase and carrier table

| Case | Checked-in fixture's observed path | Closed semantic registry's first rejecting phase | Product Alpha carrier in constructed-package test |
| --- | --- | --- | --- |
| scope / use before declaration | Python helper returns `runtime_rejection` with `unbound variable` detail | static typecheck: `UnboundVariable` | `MirCompute` at `<runtime_input.mir_compute>` |
| arrays / out of bounds | Python helper returns `runtime_rejection` with `out of bounds` detail | evaluation: `OutOfBounds` after typecheck succeeds | `MirCompute` at `<runtime_input.mir_compute>` |
| Vec3 / unknown field | Python helper returns `runtime_rejection` with `unknown field` detail | static typecheck: `UnknownField` | `MirCompute` at `<runtime_input.mir_compute>` |
| control flow / non-Bool condition | Python helper returns `runtime_rejection` with `condition must be Bool` detail | static typecheck: `TypeMismatch` | `MirCompute` at `<runtime_input.mir_compute>` |
| imports / missing `add_one` | Python helper returns `runtime_rejection` with missing-import detail | static typecheck: `UnknownFunction` | `MirCompute` at `<runtime_input.mir_compute>` |

The correspondence is by shared module/function identity and separately
asserted rejection detail. It is not a claim that the checked-in helper
fixture and the Rust Product Alpha route are one implementation path.

## Correction to prior LAB record

Report 2327 correctly distinguished fixture helper execution from constructed
package runtime tests, but its phrase that all five negative P-COMP-03 modules
reject "before evaluator execution" was too broad. The precise result is four
static typecheck rejections and one evaluation-time bounds rejection. This
record supersedes that phrase; report 2327 remains immutable historical task
evidence.

The helper's `terminal_outcome: runtime_rejection` is a matrix comparison
category, not a proof of Rust evaluator reachability or a Canon dynamic-failure
classification. Product Alpha's current `MirCompute` wrapper likewise
collapses the five cases and exposes no public rejection-phase carrier.

## Consequence and stop line

This audit strengthens only the accuracy of LAB evidence classification:

- current helper fixtures are not direct Product Alpha phase evidence;
- constructed-package tests show a four-static / one-evaluation split in the
  closed registry; and
- neither route determines how a future Mir implementation must expose static
  diagnostics, dynamic failures, or `Reject`.

No helper, fixture schema, package schema, Rust runtime, CLI, CI, Make target,
Canon file, Gate, Phase, OBL, or conformance record changes in this package.
A future public or fixture-visible phase carrier would require a separately
scoped design and boundary review; it is not selected here.

## References

- `mirrorea_canon/theory/01-mircore-v0.md`
- `mirrorea_canon/theory/02-types-effects-failures.md`
- `mirrorea_canon/spec/03-static-semantics.md`
- `scripts/mir_computational_samples.py`
- `crates/mir-semantics/src/computational_core/typecheck.rs`
- `crates/mir-semantics/src/computational_core/eval.rs`
- `crates/mir-runtime/src/product_alpha1/session.rs`
- `plan/53-mir-computational-core-roadmap.md`
- `plan/166-mir-computational-baseline-directness-audit.md`
- `docs/reports/2327-mir-computational-baseline-directness-audit.md`
