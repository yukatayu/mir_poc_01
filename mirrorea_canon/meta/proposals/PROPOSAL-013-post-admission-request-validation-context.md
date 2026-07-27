---
id: meta/proposal-013
status: L3-open
maturity: draft
depends_on: [theory/01-mircore-v0, theory/05-authority, spec/04-core-ir, spec/05-runtime-semantics, adr/ADR-0003, adr/ADR-0005, adr/ADR-0012, adr/ADR-0014, meta/proposal-012]
summary: post-admission request の authority validation claims を request-local に置く M1 の owner disposition を記録する。Core、OBL、runtime、wire は変更しない。
open_items: []
---

# PROPOSAL-013 - Post-admission request validation context

> Decision-request artifact. The owner disposition is recorded below and has
> only the stated bounded design-package effect; it has no automatic repository
> effect.
>
> It does not add or select a Core constructor, a request field, a queue
> carrier, a generated-edge schema, an occurrence identity, an OBL, a Gate, a
> Phase, a runtime, a wire protocol, or a public contract.

## Target and authority boundary

The target is the minimum semantic validation claims by which a post-admission
owner-directed request can be validated at `[E-SERVE]` without treating a
transport session, a locus name, or a key as authority. The claims are checked
against authoritative state; they are not authority by themselves.

Only the human owner may select this representation boundary. The choice
connects the settled authority requirements to a later Core/queue proof or
runtime design; it does not choose an implementation encoding. This proposal
is non-duplicative and separately recordable from PROPOSAL-012: it concerns
validation-input provenance, not the value-flow, reply/receipt, served-write,
or admission-occurrence identity families requested there. Compatibility and
dependency with the `S` and `A` dispositions remain unresolved.

## Owner disposition

Recorded on 2026-07-28: **M1 accepted — request-local validation context.**

A later compatibility-reviewed design package may formalize request-associated
claims sufficient to identify principal, epoch, incarnation, and referenced
capability/witness provenance. The claims remain non-authoritative inputs that
the owner validates against authoritative membership, lineage, witness,
admission, visibility, and history facts. This disposition selects neither
field names nor an encoding, queue carrier, request-instance identity,
occurrence identity, transport session, wire envelope, or public interface.

The package must reject copied/replayed, stale, wrong-target, missing-witness,
and severed-lineage requests without store mutation, including two active
principals at one source locus. If it needs a hidden correlation, a new Core
primitive, or another unselected carrier, it stops for a successor decision.

## Current source reading

The displayed Core request is
`request(ell_src -> ell_own, op, values, caprefs, witrefs, failures)`. Its
settled design note names only capability references, witness references, and
the failure row. The runtime configuration queues this `request` form.

In contrast, `[LOCUS-BLOCK]` says that a non-owner block elaborates requests
carrying the origin principal, membership epoch, incarnation, capability refs,
witness refs, and spans. `[E-SERVE]` validates epoch, incarnation, capability
lineage, witnesses, and visibility. Theory 05 requires post-admission messages
to carry principal, epoch, incarnation, capability refs, and required witness
refs; a valid capability use must match its originating verdict, principal,
admitted role, target locus/world, epoch, incarnation, and (where required)
admission witness, as well as grant-policy version.

The L2 Core-IR exchange example has a request edge with `from`, `to`, `op`,
`caps`, `witnesses`, `fails`, and `span`; it does not establish where the
principal/epoch/incarnation validation context is represented or recovered.
Its illustrative `req.principal` use therefore cannot be treated as a closed
authority carrier. The runtime prose requires validation but likewise does not
choose a representation.

These statements preserve the intended anti-spoofing rule, but a later proof or
runtime package cannot silently choose whether the missing context is carried
by the request, recovered through an explicit non-transport relation, or left
outside the present formal model.

## Question presented

> Which representation family may a later owner-reviewed design package use
> to supply the validation claims for a post-admission request?

**M1**, request-local validation context; **M2**, an explicit non-transport
correlation to selected Canon authority facts; or **MD**, defer the
representation and prohibit packages from relying on an unselected recovery.

None of these alternatives identifies authentication with transport metadata,
turns a key or role claim into authority, or decides an event identity. A later
package still needs the ordinary Canon process before changing any rule or
carrier.

## Alternatives

| Option | Owner-level effect if selected | Required later verification boundary | Immediate non-effect |
| --- | --- | --- | --- |
| M1 - request-local validation context | A later package may formalize request-local validation claims sufficient to identify the principal, epoch, incarnation, referenced capability and required witness provenance. These claims are not authority; the presentation need not freeze field names. | It must compare the claims with authoritative membership, capability-lineage, witness, admission, and history facts; account for every Theory 05 capability-lineage component and `[E-SERVE]` visibility; reject stale, replayed, wrong-target, and severed-provenance uses; preserve owner seriality; and keep the context separate from transport identity. | Does not choose a Core syntax spelling, JSON field, queue implementation, wire envelope, occurrence kind, reply/receipt, or public API. |
| M2 - explicit non-transport correlation | A later package may retain the displayed request shape only with an explicit semantic relation that recovers the required validation claims from selected Canon membership, capability-lineage, witness, admission, and history facts without ambient lookup or transport identity. | It must state the relation's domain, uniqueness/freshness rule, save/load and revocation behavior, relation to later admission/service identity decisions, and why two active principals at one source locus cannot be confused. If this requires a new request-instance identity, queue carrier, event identity, or hidden side relation, it stops for another owner decision. | Does not silently use a queue-local side table, event identity, transport session, or helper-local envelope as a semantic fact. |
| MD - defer | No proof-facing or runtime design package may claim a closed, Canon-grounded account of validation-input provenance through a request representation until this boundary is selected. | Unrelated research may continue, but a package needing that closed provenance account stops for a successor decision. | Does not suspend the existing `[E-SERVE]` validation/fail-closed requirement, reject either future representation family, or weaken the existing anti-spoofing invariant. |

## Evidence and verification boundary

The decision evidence is the read-only Canon source at `497b8b0d`:
`theory/01-mircore-v0`, `theory/05-authority`, `spec/04-core-ir`,
`spec/05-runtime-semantics`, ADR-0003, ADR-0005, ADR-0014, and PROPOSAL-012.
LAB Plan 192 records the literal comparison and the prior-audit check that this
is not a replay of the already separated value-flow or occurrence-identity
questions.

Before a later package relies on M1 or M2, it must account for every
Theory 05 capability-lineage component: originating verdict, principal,
admitted role, target locus/world, membership epoch, member incarnation,
required admission witness, and grant-policy version. It must also account for
the required witnesses and visibility checked by `[E-SERVE]`. The following
are minimum adverse cases, not an exhaustive definition of validation: two
simultaneous active principals at one source locus, an owner-mediated operation,
stale membership, wrong target, missing capability/witness, and replayed or
severed lineage; each validation failure leaves the store unchanged. Discovery
that the selected form requires a new Core primitive, generated-edge schema,
event identity, wire contract, OBL change, Gate/Phase action, or public
interface stops the package for the ordinary Canon process.

## Requested owner output

Record `M1 accepted`, `M2 accepted`, `MD deferred`, or `return for
clarification`. This proposal ranks no alternative. A later design package may
compare the alternatives only after an owner disposition, while preserving the
project rule that transport is not authority. This is not a selection by the
proposal.

Recorded output on 2026-07-28: `M1 accepted`.

An acceptance authorizes only a later design package. It requires the ordinary
Canon process before editing theory/spec text, an ADR, `theory/11`, scenarios,
Gates, Phases, an implementation, or a public contract.

## Non-effects

This proposal does not:

- change request, queue, Config, `G_e`, generated-edge, admission, capability,
  witness, membership, epoch, incarnation, or `[E-SERVE]` semantics;
- choose a Core AST/IR field, JSON schema, queue/local-store representation,
  lookup key, transport/session field, wire envelope, event identity, or
  persistence encoding;
- decide PROPOSAL-012's `V`, `R`, `S`, or `A` families, their compatibility or
  dependency, BND-001 outcome totality, or the OBL-001 Core/write statement
  interface;
- add, rename, discharge, or change the status, target, wording, or Lean
  target of any THM or OBL;
- change scenarios, conformance, runtime behavior, helper/schema/CI surfaces,
  implementation readiness, or public readiness; or
- make a role claim, key, locus name, provider, package, signature, or
  transport identity into authority.
