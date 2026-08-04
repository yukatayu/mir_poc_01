---
id: meta/proposal-021
status: L1-fixed
maturity: reviewed
depends_on: [meta/proposal-018, adr/ADR-0015, adr/ADR-0016]
summary: owner-approved M3 scopeで、有限 EvalPlan と owner transition / explicit receipt / designated materialization を採用する提案。
open_items: []
---

# PROPOSAL-021 — Evaluation / materialization calculus

## Owner disposition

Apply the owner-approved M3 scope in PROPOSAL-018 and ADR-0015. Adopt one
finite, typed `EvalPlan` that separately records semantic form, evaluation
site, trigger/clock, authority origin, and materialization. It is a Core
carrier and elaboration result, not M6 Surface grammar and not a public wire
format.

The selected M3 rule is: a same-owner mutable read-dependent write is an
owner-store `eval` served serially at that owner; its requester remains the
authority origin. An unannotated other-owner operand is rejected. The sole M3
alternative is an explicit `RemoteResult` request and receipt, which may be
used only after its request/serve/reply/receive chain has obtained a
target-labelled receipt; it supplies neither a common snapshot nor a
transaction. An authoritative designated evaluation is keyed by evaluator and
canonical input-producer frontier, then publishes one versioned value for an
explicit consumer step.

## Scope and non-effects

This authorizes the M3 Canon rules, finite Lean model, bounded executable
reference tests, SCN amendment/addition, proof-ledger evidence classification,
and ordinary report/review procedure. Consumer and provider are classified
here; maintained-relation DAGs, projection coherence, semantic/presentation
fallback, and their execution remain M4. It does not select Surface syntax,
public API/ABI/wire fields, transport/retry, distributed transaction, save/load
or patch algorithm, a final runtime, or an I1/conformance/deployment claim.
