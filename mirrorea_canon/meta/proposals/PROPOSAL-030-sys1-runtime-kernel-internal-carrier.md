---
id: meta/proposal-030
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, adr/ADR-0025, adr/ADR-0026, arch/04-runtime-carriers, theory/13-evaluation-materialization, spec/05-runtime-semantics]
summary: SYS-1のcrate-private semantic runtime kernelと、owner request及びdesignated remote-inputのbounded internal lifecycle carrierを固定する提案。
open_items: []
---

# PROPOSAL-030 — SYS-1 runtime kernel and bounded internal carrier

## Owner disposition and selected capability

Under ADR-0026, accept the smallest SYS-1 separation that gives SYS-2 and
SYS-3 a typed runtime consumer without making the M10 conformance shell the
systems architecture.

The production path for ordinary `run_source` and a generic checked
`OwnerEvent` is a crate-private semantic runtime kernel. It consumes one exact
checked program plus a sealed M9 execution seam, owns the admitted M8 local
runtime while serving owner requests, and permits that M8 runtime to be
extracted after kernel service. M10 profile selection, evidence predicates,
correspondence verification, release manifests/anchors, and CLI rendering are
outside the kernel and do not flow into it.

This claim is intentionally narrower than “the whole M10 facade uses the
kernel.” The specialized historical SCN-04, SCN-09, SCN-10, and route-patch
runners retain their existing M10 regression-only paths. They are not SYS-1
kernel acceptance evidence and are not generalized here.

## Selected internal lifecycle contract

Two source-derived lifecycle families are admitted:

```text
checked owner operation:
  request -> serve -> reply -> receive/receipt

checked designated remote-input dependency:
  request -> source-owner serve -> reply -> receive/receipt
          -> designated-evaluator consume
```

Both use a kernel-issued request identity distinct from queue position and
retain concrete ordered occurrence references. Both return either a typed
success or a failure from the checked declared failure row. Their internal
carrier retains the applicable subset of:

```text
request identity; operation identity; origin principal and locus;
target owner, or source owner and target evaluator;
request/serve/reply/receive occurrences;
exact checked source reference and Core reference;
effect row and declared failure row;
visibility class and redaction policy;
membership reference, epoch, and incarnation;
capability and witness references;
input frontier, producer release tuple, receipt identity,
and explicit consumption key/state where applicable.
```

The producer release tuple and evaluator authority are non-interchangeable.
The designated remote-input path is the first bounded effect-handler-like
instance: a typed checked remote read is requested at its admitted source
owner and returns a typed result/failure for explicit evaluator consumption.
It is not a generic provider registry and does not collapse transport,
authorization, projection, persistence, or semantic ownership into one
handler abstraction.

## Authority and failure discipline

The M9 seam is the only production source of admitted membership,
capability, witness, owner, and producer-release lineage for this kernel. A
schedule/caller supplies operation arguments and the exact checked origin
locus; it cannot choose owner/evaluator, provenance, authority, release
label, or request identity. A receipt reports a completed lifecycle; it does
not grant authority, transfer ownership, or authorize a later mutation.

Carrier validation completes before any M8 enqueue or semantic occurrence is
created. Source-free, wrong-source/Core, wrong-target, wrong-origin, stale,
missing-authority, duplicate, and out-of-order carriers fail closed. Duplicate
rejection is single-assignment validation, not a hidden retry or an
exactly-once delivery guarantee. The contract defines no retry protocol.

## Primary falsifiers and acceptance evidence

The contract is false if any tested invalid/duplicate/stale/wrong target or
wrong source carrier mutates state; a restricted value escapes redaction; an
evaluator capability substitutes for the producer release; pre-admission
failure creates a kernel occurrence or leaves work in M8; queue order changes
request identity/result alignment; the caller can change the checked origin
locus; or a malformed checked factory panics instead of returning typed
diagnostics.

The selected source cut is
`94e3707c7bc98d4a0764c51f13a12b1dae1968c6`. Its evidence is
`runtime-monitored`: 13 focused SYS-1 tests, ordinary source and generic
checked-owner integration, unchanged M10 regression groups, changed-crate and
workspace validation, and two independent reviews. No Lean proof, model
check, general correspondence theorem, or broad scheduler claim is added.
Legacy public M8 receipt builders remain M8/LAB fixture APIs and are not the
I2 internal carrier selected here.

## OPEN-030 resolution and residual

OPEN-030 is resolved only for the preceding I2-internal bounded contract.
The carrier remains crate-private and provisional. Architecture/04 remains
L2-working: OPEN-026 field-name/IR exchange questions, OPEN-027 external
delivery observability, and the broader internal carrier freeze still block
broad PHASE-I1 acceptance. A final public API, ABI, JSON schema, or wire form
is deliberately not selected.

The current production admission is an immutable snapshot of final M9 facts.
Therefore revoke-after-enqueue and revoke-after-serve visibility is not
claimed by SYS-1. Mapping revocation/publication visibility to ST/OW ordering
is the direct SYS-2 blocker; a stale use observed after the required
revocation edge reopens this decision.

## Non-effects

This proposal does not change ordinary Surface, add low-level memory-order
syntax, define a generic effect/provider registry, move relation/projection,
save/load, patch, or specialized M10 runners into the kernel, select real
transport, freeze a public contract, claim exactly-once, change the proof
ledger, exit broad PHASE-I1, accept official I2 entry/exit, or change theory
lifecycle T1.
