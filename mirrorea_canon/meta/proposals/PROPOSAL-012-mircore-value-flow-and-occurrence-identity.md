---
id: meta/proposal-012
status: L3-open
maturity: draft
depends_on: [theory/01-mircore-v0, theory/03-elaboration, theory/04-ordering-and-cuts, theory/05-authority, adr/ADR-0002, adr/ADR-0003, adr/ADR-0005, adr/ADR-0014, meta/proposal-009]
summary: read の値フロー、成功 service、admission の occurrence identity を個別に owner へ問う。Core、OBL、Gate、runtime は変更しない。
open_items: []
---

# PROPOSAL-012 - MirCore value flow and occurrence identity

> Decision-request artifact only. This proposal records no owner answer and has
> no automatic repository effect.
>
> It does not add or select a Core constructor, a result/reply carrier, an
> operational rule, an occurrence schema, an OBL, a Gate, a Phase, a scenario,
> a runtime, a wire protocol, or a public contract.

## Target and authority boundary

`theory/01-mircore-v0` fixes `read` as a dependency, `write` as an occurrence,
and a zero-or-one-occurrence step discipline. Its existing abstract grammar and
rule sketches do not yet fix two interfaces needed by a future formal model:

1. how a value obtained by a runtime read is made available to a later computed
   write; and
2. which occurrence or occurrences represent successful remote service and the
   membership/grant/witness effects of admission.

Only the human owner may choose either semantic family. This proposal requests
four independently recordable dispositions. An answer authorizes only a later
design package; the ordinary Canon process remains required before changing
theory text, an ADR, `theory/11`, scenarios, Gates, Phases, or implementation.

`PROPOSAL-008` remains the exclusive decision record for BND-001 outcome
totality and ledger placement. `PROPOSAL-009` remains the separate static
THM-001 Core/write statement-interface decision. Neither is answered or
presupposed here.

## Current source reading

The Core grammar contains `read(ell, x[v].f)`, `write(ell, x[v].f, vprime)`,
`request`, `seq`, `cond`, and `pure`. `read` is a dependency rather than an
occurrence, while `write` requires a value. The SCN-02 worked shape has a
cross-locus read and an owner-directed write request with `vprime`, but fixes
neither a result carrier nor an evaluation relation for that value. `OPEN-011`
explicitly leaves the read reply/receipt carrier open.

Each operational step appends zero or one occurrence. `[E-SERVE]` records a
request-to-serve edge and says a successful owner serves as a write or
read-plus-reply. `[E-ADMIT]` updates membership and issues grants and
witnesses. The causal family already requires request-to-serve,
admit-request-to-verdict-to-activation-cut, grant-to-use, witness-create-to-use,
and membership-update-to-dependent-dispatch order. It does not select whether
the named roles are separate occurrence nodes or exact projections of one
composite node. `OPEN-010` currently reads a service failure reply at the
requester as a receive occurrence; this proposal neither closes nor changes
that open item.

The preserved constraints are: ordinary S0 source; explicit S1 request,
failure, authority, witness, and generated-edge obligations; S2 DAG acyclicity;
owner-serial state mutation; no ambient authority; and no collapse of
authentication, authorization, membership, capability, or witness into
transport identity. A domain `World` or `Game` remains S5 vocabulary, not a
Core concept.

## Question V - read-to-write value flow

> Which semantic family should a later design package use to expose a runtime
> read result to dependent Core computation?

| Option | Owner-level effect if selected | Required later design boundary | Immediate non-effect |
| --- | --- | --- | --- |
| V1 - restricted result-binding contract | A later package may make one-shot, locus-bound read-result use formal. An explicit administrative binding is the reference presentation. | It must distinguish owner-local read from explicit cross-locus read request; state a typed correlation/result/failure carrier, local binding scope, first-order pure value computation, held `Gamma`/`Delta` context, evaluation order, single evaluation, resumption, pending control state, cut/save/load treatment, and no dependent write after failure. | Does not select final spelling, generic `bind`, ANF grammar, type constructor, handler algebra, first-class continuation, wire message, or public API. |
| VD - defer | No proof model or implementation claim may transcribe a read-dependent computed write as if the current grammar supplied its value flow. | Other independent research may continue, but an attempted design that needs this relation stops for a successor decision. | Does not reject a future restricted result-binding contract. |

An evaluation-frame or machine-state presentation is not an independent owner
alternative: it can be investigated only after V1 as an explicit, equivalent
presentation of the selected restricted contract. It cannot retain the current
grammar without additional semantics. A
restricted administrative binding is distinct from a general continuation: its
continuation is syntactically delimited by the enclosing Core term, is not a
value, cannot be captured/stored/transmitted/re-entered, cannot duplicate linear
capabilities or effects, and has only the explicitly declared success and
failure paths. First-class or migratory continuations remain outside v0.

## Question R - successful read reply and receipt

> What makes a successful cross-locus read result available to exactly one
> requester-side pending computation?

| Option | Owner-level effect if selected | Required later design boundary | Immediate non-effect |
| --- | --- | --- | --- |
| R1 - explicit typed reply and receipt | A later package may model owner service result and requester receipt as separate, typed causal steps. The requester resumes only after its matching receipt. | It must state request/result correlation, result provenance, send/receive order, duplicate/stale/wrong-locus rejection, redaction, no raw-value history leak, and failure behavior. | Does not require a transport, delivery, fairness, retry, or public wire protocol. |
| R2 - abstract receipt with refinement | A later package may abstract delivery into a service-level result only with an explicit refinement/linearization relation that proves no separately observable or interleavable receipt fact was hidden. | It must still provide a unique requester-side pending-control transition and account for failure, save/load, and causality. | Does not make a hidden callback, future, or transport event into a semantic carrier. |
| RD - defer | OPEN-011 remains unresolved; a proof or implementation may not assume a successful-reply mapping. | A package needing result delivery stops for a successor decision. | Does not reject either future model. |

R1 is the current recommendation because an owner-side service and a
requester-side result are cross-locus and can otherwise differ in failure,
interleaving, and observability. R2 remains available only with the stated
refinement boundary; it is not an implicit shortcut.

## Question S - successful remote write service identity

> Which occurrence family should represent a successful remote owner write?

| Option | Owner-level effect if selected | Required later design boundary | Immediate non-effect |
| --- | --- | --- | --- |
| SW1 - atomic served-write occurrence | A later package may represent validation and one owner mutation by one typed `ServedWrite` occurrence carrying named service and mutation facets. | It must state the projection-to-DAG rule, make the same node satisfy the state-mutation predicate, preserve validated capability/witness lineage, and leave independently observable reply/receipt facts to R. | Does not permit an opaque batch delta, erase request/failure occurrences, or merge transport identity with authority. |
| SW2 - decomposed service and write | A later package may represent validation/service and a later mutation by distinct occurrences. | It must state the pending authorized-operation state, causal edges, intermediate observability, revocation/epoch/time-of-check protection, and failure/no-mutation behavior. | Does not require a queue, delivery, fairness, retry, cryptographic, or transport model. |
| SWD - defer | The current `[E-SERVE]` wording remains a rule sketch only; no proof or implementation may assume one particular event identity. | Any package requiring service-event identity stops for a successor decision. | Does not reject future decomposition or composite representation. |

The zero-or-one rule applies per primitive semantic step, not to an unexamined
multi-action batch. A composite option is acceptable only if its projections
remain explicit enough to construct the existing causal relations and to audit
authority/failure behavior. SW1 is the current recommendation: it retains
owner-serial atomic mutation without introducing a separately schedulable
authorized-operation state. It is not selected by this proposal.

## Question A - admission occurrence identity

> Which occurrence family should represent verdict, membership update, grant
> issuance, and witness creation at admission?

| Option | Owner-level effect if selected | Required later design boundary | Immediate non-effect |
| --- | --- | --- | --- |
| A1 - decomposed admission occurrences | A later package may use distinct primitive occurrences for the verdict, membership transition, grant creation, and witness creation. | It must state their causal order, partial-failure/rejection behavior, intermediate visibility, grant/witness identifiers, and why dependent dispatch cannot bypass the required frontier. | Does not select an admission protocol, identity provider, persistence mechanism, or lifecycle API. |
| A2 - composite admission/verdict occurrence | A later package may use one composite admission occurrence with named projections for verdict, membership change, grants, and witnesses. | It must state how every projected grant/witness/membership fact is referred to by later causal edges, how rejection produces no unauthorized grant or membership change, and how load/rollback preserves lineage. | Does not collapse grant lineage into membership, transport, or a role claim. |
| AD - defer | The current `[E-ADMIT]` wording remains a rule sketch only; no proof or implementation may assume a chosen history mapping. | Any package requiring that mapping stops for a successor decision. | Does not reject future decomposed or composite representation. |

The current recommendation is A2 when the verdict atomically establishes the
membership and the finite named grant/witness set. It remains conditional: any
separately failing, observable, or schedulable issuance phase needs A1 instead.

## Dependency and verification order

`V` and `R` govern read-result service. `S` governs served writes and is
otherwise independent of `R`; `A` is independently answerable. The four
dispositions may be mixed; this
proposal does not require all to be accepted at once.

Before any resulting Canon amendment, a later design package must test its
chosen family against SCN-02; owner-serial service; request-to-serve,
grant-to-use, witness-create-to-use, membership-update-to-dispatch, and
admit-request-to-verdict-to-activation causal relations; S2 DAG acyclicity;
failure atomicity/no mutation; linear capability non-duplication; and the
distinction between a semantic reply carrier and a transport/public contract.
Existing LAB elaborators, admission reports, runtimes, and helper DAGs are not
tie-breaking evidence because they do not implement the disputed shared model.

The literature comparison recorded in `LAB:plan/187` is decision support only:
effect-calculus bind, ANF, and evaluation-context work explain established
value-flow mechanisms; distributed process calculi explain the distinction
between an abstract atomic service and observable asynchronous protocol phases.
It supplies no Mir-specific semantic choice.

## Requested owner output

Record a separate disposition for each question:

```text
V = V1 | VD
R = R1 | R2 | RD
S = SW1 | SW2 | SWD
A = A1 | A2 | AD
```

An owner may return `clarify <question>` for any item. A non-defer disposition
authorizes only the corresponding bounded design/comparison package; it does
not itself edit a rule, admit an executable lane, alter a theorem, or claim
implementation readiness.

## Non-effects

This proposal does not:

- change `read` dependency status, `G_e` dependency membership, the current
  static Core/write THM-001 wording, BND-001 totality, or any OPEN item;
- add `bind`, `let`, an evaluation context, a continuation, a result/reply
  carrier, a new event kind, a history schema, a queue rule, or a scheduler;
- select final syntax, result representation, receipt payload, durability, or
  public/wire message;
- change authority, membership, capability, witness, visibility, failure,
  cut/save/load, patch, conformance, provider, or transport semantics;
- create a WRK, helper, schema, CI target, sample, runtime behavior, proof,
  OBL status, Gate/Phase movement, or public-completion claim; or
- supersede PROPOSAL-008, PROPOSAL-009, PROPOSAL-010, or PROPOSAL-011.
