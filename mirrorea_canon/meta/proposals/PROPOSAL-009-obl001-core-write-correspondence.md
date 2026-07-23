---
id: meta/proposal-009
status: L3-open
maturity: draft
depends_on: [theory/01-mircore-v0, theory/03-elaboration, theory/11-metatheory-ledger, adr/ADR-0014, plan/02-operating-model]
summary: THM-001 / OBL-001 の proof-facing Core/write statement interface を owner に問う。Core、OBL、Gate、proof status は変更しない。
open_items: []
---

# PROPOSAL-009 - OBL-001 Core/write correspondence boundary review

> Decision-request artifact only. This proposal records no owner answer and has
> no automatic repository effect.
>
> It does not change THM-001, a Core constructor, a runtime rule, an OBL,
> `theory/11` status, a Gate, a Phase, a contract, an implementation, or a
> public claim.

## Target and Authority Boundary

The target is the proof-facing statement interface for the existing `OBL-001`
target (`MirCore.Elab.Soundness (stmt)`). The Canon THM-001 wording in
`theory/03-elaboration` already ranges over every write in elaborated Core
`c`; this proposal asks how a later formalization package may expose that
existing statement domain.

Only the human owner may select this boundary or revise the Canon statement.
This proposal asks neither for a Lean artifact nor for a proof. It does not
make a LAB `Result`, `GeneratedWrite`, or countermodel canonical.

## Current Source Reading

`theory/01-mircore-v0` has distinct Core constructors for owner-local `write`
and owner-directed `request`, including requests whose operation is `write`.
It also has `seq` and `cond`; a Surface assignment may include simple or
compound assignment syntax. `theory/03-elaboration` requires every write in
the elaborated `c` to be owner-local or an explicit owner-directed request,
with the stated obligation, failure-row, dependency-edge, and source-span
properties.

This is a static elaboration property. It is not a claim about a runtime
occurrence in `H`, a queue item in `Q`, or a store mutation in `S`. In
particular, a cross-locus request is not a direct remote-store mutation; the
owner later validates and serves it or records an explicit fail-closed outcome.

At LAB source cut `1630dd32`, WRK-0007 demonstrates only that the current
experiment-local `Result` / `GeneratedWrite` draft does not establish coverage
of every Core write. It does not refute THM-001 or rule out a future explicit
correspondence. `LAB:plan/180` records the resulting unselected statement
boundary. Neither artifact selects a Core representation, a write-occurrence
relation, or a final proof interface.

## Question Presented

> Which statement interface should govern a later owner-reviewed OBL-001
> formalization package?
>
> **A**, state THM-001's existing every-write clause directly over Core `c`;
> **B**, permit a Result/output-level statement only with an explicit
> Core/write correspondence that derives the existing Core clause; or **C**,
> defer the interface choice and prohibit proof-facing reliance on either
> route.

The alternatives describe the final statement boundary, not mutually exclusive
implementation techniques. A direct statement may use proof-local traversals or
lemmas. A bridge route still has to account for the Canon Core clause; it does
not replace it with an opaque output predicate.

## Alternatives

| Option | Owner-level effect if selected | Required later verification boundary | Immediate non-effect |
| --- | --- | --- | --- |
| A - direct Core `c` | A later OBL-001 statement packages the every-write property directly over the elaborated Core term. | The package exposes how owner-local `write`, request-carried `op = write`, `seq` / `cond`, and the relevant Surface assignment forms are in or out of its statement domain. It preserves THM-001's disjunction: owner-local writes are local, while an owner-directed request carries the stated authority, failure, edge, and span properties. | Does not choose a Core AST/IR representation, traversal, occurrence relation, equality, Lean API, proof decomposition, or runtime behavior. |
| B - explicit Result/output bridge | A later OBL-001 package may state a Result/output view together with a correspondence sufficient to derive the Core clause. | The package states a non-opaque direction from every relevant Core write to the output classification and handles extra and duplicate output entries explicitly. It preserves the same disjunction: owner-local writes are classified as local, while each covered owner-directed request carries its `C union O` authority placement, failure row, `G_e` edge record, and source span. | Does not identify an experiment-local Result with Core, choose its container/order/equality, define a public serialization contract, or revise THM-001. |
| C - defer | No proof-facing OBL-001 package may rely on a direct statement of the Core clause or an output/Core correspondence until this decision is reopened. | Unrelated standing-eligible L3 research may continue under ADR-0014, but no draft may present the existing Result predicate as the missing correspondence. | Does not decide that either route is invalid, permanently forbid a bridge, or alter current WRK evidence. |

## Evidence and Verification Boundary

The decision evidence is the read-only Canon source at `1630dd32`:
`theory/01-mircore-v0`, `theory/03-elaboration`, `theory/11-metatheory-ledger`,
and `adr/ADR-0014`; and the LAB evidence `WRK-0007` and `LAB:plan/180` at that
cut. LAB evidence identifies a statement-shape gap only. It neither supplies a
canonical carrier nor selects an alternative.

Before a later package relies on A or B, it must make the static Core domain
and its relation to any proof-local output view inspectable, distinguish it
from runtime `H` / `Q` / `S`, and demonstrate the stated coverage and
non-regression conditions. Discovery that this requires a new Core primitive,
request meaning, assignment semantics, outcome-totality premise, ledger
change, Gate/Phase change, or public contract stops the package for the
ordinary Canon process.

Outcome totality remains independent: PROPOSAL-008 is the only current owner
decision surface for BND-001 outcome-totality interpretation and future
OBL-021 placement. This proposal cannot settle or rely on it.

## Requested Owner Output

Record `A accepted`, `B accepted`, `C deferred`, or `return for clarification`
with the disputed boundary. An acceptance authorizes only the corresponding
later design package. It requires the ordinary Canon process before changing
`theory/03-elaboration`, `theory/11-metatheory-ledger`, an ADR, a Gate, a
Phase, or a proof-facing artifact.

## Non-effects

This proposal does not:

- add, rename, discharge, or change the status, target, wording, or Lean
  target of any OBL or THM;
- define `CoreWrite`, request-carried write membership, branch normalization,
  assignment coverage, a Result carrier, equality, enumeration order, or a
  Core/result identity relation;
- change the distinction between static `c` and runtime occurrences, queues,
  stores, failure handling, owner-serial execution, authority, or transport;
- change existing LAB statement drafts, WRK-0007 evidence, or any L3 status;
- claim elaboration soundness, a proof, no-hidden communication, parser or
  runtime correctness, conformance, a Gate/Phase transition, or public
  readiness; or
- supersede PROPOSAL-003 or PROPOSAL-008, or introduce a language primitive,
  helper family, schema, CI surface, evidence lane, implementation, or public
  interface.
