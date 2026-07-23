# plan/183 - Transparent cost-bound substitutability decision memory

## role and status

This is LAB repository memory for `PROPOSAL-011`. `mirrorea_canon/` remains
normative. The proposal is owner-pending and does not alter the current
Contract, overlay, OBL, runtime, or sample status.

## observed source relation

The Canon `Contract` tuple in `theory/02-types-effects-failures` has a
`cost_bound` field. Its transparent-overlay rule lists conditions that all must
hold, but the list does not name cost. The text does not say whether that list
is the complete substitutability predicate or how a cost-only change is
classified. `OPEN-013` describes an opaque bound and simple numeric comparison,
but it does not define a transparent-layer old/new bound relation.

`OBL-026` is consequently not ready to rely on an implicit cost rule. It stays
unchanged and open; this memo neither changes its statement nor supplies a
proof premise.

## evidence boundary

The active clean-near-end source declares three named CostBudget counters and a
pointwise law. Its cost-negative example requires one `remote_calls` unit and
offers zero, so the bounded LAB checker rejects it. The same source offers no
general Contract/layer evaluator, scalar-total policy, or final runtime cost
semantics. The older layer roadmap says both that cost degradation should reject
and that the final algebra is deferred. These are useful consistency signals,
not Canon authority.

## owner decision packet

`mirrorea_canon/meta/proposals/PROPOSAL-011-transparent-cost-bound-substitutability.md`
asks whether transparent overlays should preserve a bound under an
owner-selected interim comparison fragment (recommended), whether all changes
must be explicit updates until the algebra is selected, whether cost is
advisory, or whether interpretation should defer. The proposal deliberately
leaves patch-carrier scope and the final algebra for later decisions.

## next safe work

- Await the owner disposition before altering `theory/02` or treating a cost
  relation as a premise of OBL-026.
- A distinct ADR-0014 L3 countermodel may test only an existing LAB model's
  projection properties after its own committed pre-registration. It cannot
  decide this Contract rule or generalize a helper-local result into Canon.
- Keep unknown cost representations explicit. Do not infer that a scalar total
  preserves named resource limits or that Canon already defines an
  incomparability outcome.

## non-claims

This is not a cost-algebra choice, a new finite-index family, a Core/runtime
feature, a patch rule, an OBL/THM movement, a sample workflow update, or public
readiness evidence.
