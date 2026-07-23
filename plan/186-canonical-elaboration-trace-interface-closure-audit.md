# Plan 186 - Canonical elaboration/trace interface-closure audit

## Role and authority

This is a read-only LAB audit. `mirrorea_canon/` remains normative. It does
not amend a Core constructor, generated-edge carrier, event kind, transition
rule, obligation, proof status, Gate, Phase, conformance rule, or runtime.
Where a Canon choice is necessary, it records an escalation question rather
than selecting an answer.

## Audit rule

Each claim is classified only from the cited Canon wording:

- **closed**: the text supplies a single sufficient interface reading for this
  audit;
- **ambiguous**: more than one material reading remains and none may be chosen
  from LAB;
- **schema under-specified**: the carrier has required inhabitants but the text
  does not provide a closed formal grammar for every row kind;
- **outside scope / unaccepted**: an advisory finding has not been established
  by this restricted source comparison.

This is a source audit, not an L3 experiment. It opens no WRK because it has
no existing admitted executable lane or non-reserved binary importer.

## Scope

The audit examines four interfaces that must be readable before a future common
proof model can even transcribe the existing S0/S1/S2 boundary:

1. BND-001 elaboration outcomes;
2. Core value flow from a source read into a computed write value;
3. generated-edge output `G_e`;
4. the one-occurrence step discipline for remote service and admission.

It deliberately does not resolve the reviewer-raised request-envelope,
fallback, save/load, conformance, authority, or metadata questions. In
particular, the `theory/01`/`theory/02` mutual `depends_on` reference is not a
defect: `meta/style-guide` expressly permits mutual knowledge dependencies.

## Source matrix

| Interface | Canon anchors | Literal reading | Classification | Consequence without choosing a repair |
| --- | --- | --- | --- | --- |
| BND-001 outcome form | theory/03 BND-001; theory/10 Carrier; PROPOSAL-008 | The text says a well-scoped item "either produces" the full tuple or a Diagnostic, and separately states determinism. PROPOSAL-008 reserves the human interpretation of whether this wording is a proof-usable totality premise and its OBL placement. | **ambiguous at the owner-reserved proof boundary**, not a new contradiction | A future proof package must continue to obey PROPOSAL-008; this audit neither treats the wording as an accepted totality premise nor revises it. |
| Core value flow | theory/01 abstract syntax and [READ/WRITE] sketches; theory/03 SCN-02 worked shape; OPEN-011 | Surface permits runtime reads and expressions, but Core has `read(...)`, `write(..., v')`, `seq`, `cond(v,...)`, and `pure(v)` with no stated bind, result carrier, evaluation context, or rule connecting a runtime read to `v'`. The SCN-02 shape uses an unbound `v'`; OPEN-011 explicitly leaves the exact read-request reply/receipt carrier open. | **ambiguous / interface not closed** | A direct Core transcription of a read-modify-write must introduce an unstated evaluation device. Any such choice is a Core/elaboration decision and is not made here. |
| `G_e` | theory/01 unified judgment and [READ-LOCAL]; theory/03 BND-001, worked shape, THM-001 | [READ-LOCAL], the worked shape, and THM-001 explicitly place dependency edges in `G_e`; span carriage is stated for every `G_e` element. The parenthetical request/publish/observe/witness lists do not declare a closed grammar. | **closed for dependency membership; schema under-specified** | Dependency rows must remain in `G_e`. A later proof representation may need a full row schema, but this audit does not move dependency rows or infer a carrier mismatch. |
| Remote service and admission occurrence delta | theory/01 step shape and OPEN-010/011; theory/00 occurrence DAG; theory/04 causal generating family | Each labelled step appends zero or one occurrence. [E-SERVE] explicitly adds `served-occ` with `request ≺ serve`, but does not say whether that one occurrence is also the write/receive occurrence. [E-ADMIT] updates membership, grants, and witnesses; theory/04 fixes `admit_request ≺ verdict ≺ activation_cut`, but not the exact history mapping of those updates. Failure replies are explicitly requester receive occurrences; the exact read reply/receipt carrier remains OPEN-011. | **ambiguous / interface not closed** | A preservation proof cannot know the event identity for successful served mutation or the admission/grant/witness history mapping. Choosing composite versus decomposed events is owner/canon work. |

## Reconciled reading of prior LAB material

Plan 180 correctly separates pairwise coherence of the LAB OBL-021 draft from
outcome production. It does not override PROPOSAL-008, whose exact purpose is
to reserve the interpretation and OBL placement of BND-001. This audit
therefore does not accept the advisory shortcut that the BND sentence alone
autonomously settles that owner question.

Plan 182's "no contradiction identified" result was bounded to its then
reviewed cross-cutting claims. This audit confirms rather than contradicts its
generated-communication reading: dependency edges explicitly inhabit `G_e`.
Only the exact one-occurrence and Core value-flow interfaces are newly narrowed
here; this audit does not invalidate Plan 182 outside those loci.

## Escalation packet

The next Canon step needs a human decision or review before proof-model work
uses the remaining unclosed interfaces. The minimum questions are:

1. Which existing or new Core-level relation explains the value flow from a
   runtime `read` to a subsequent `write` without hiding evaluation outside
   the calculus, and how does this relate to the existing OPEN-011
   reply/receipt boundary?
2. Is a successful remote write represented by one composite occurrence or by
   multiple explicit steps, and how do admission/grant/witness effects map to
   the zero-or-one occurrence rule while preserving the already explicit
   request/serve and admit-request/verdict/activation order?
3. Separately, which existing PROPOSAL-008 option governs whether BND-001
   outcome wording is a proof premise and where it belongs in the ledger?

No answer is inferred. A subsequent proposal may present alternatives and
impact analysis, but it must not silently repair the four interfaces.

## Non-claims

This audit does not prove a Canon inconsistency globally; select a Core
evaluation model, `G_e` schema, event identity, Diagnostic API, OBL statement,
totality interpretation, request envelope, fallback relation, save/load model,
or any implementation behavior. It creates no evidence lane, Lean artifact,
countermodel, runtime result, workflow, Gate/Phase movement, or public claim.
