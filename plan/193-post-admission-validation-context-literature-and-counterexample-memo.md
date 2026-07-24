# Plan 193 - Post-admission validation-context literature and adversarial memo

## Role and authority

This is LAB decision-support evidence for PROPOSAL-013 at clean `cfc246c8`.
`mirrorea_canon/` remains normative. It selects neither M1, M2, nor MD; it
does not amend a Canon rule, carrier, request instance, queue, wire protocol,
transport, runtime, OBL, Gate, Phase, or public contract. The external works
below are contrasts, not imported semantics.

## Question

Can primary literature on authorization and provenance select a representation
family for the post-admission validation claims required by `[LOCUS-BLOCK]`,
`[E-SERVE]`, and theory/05? If not, how can it contrastively pressure-test the
already fixed Canon requirement that those claims be semantically associated
with a request and checked against authoritative state?

## Canon boundary retained

Theory/01 and theory/05 require post-admission requests/messages to carry
principal, epoch, incarnation, capability and witness references. Those
request-associated claims are then checked against the authority conditions:
Theory/05 requires an originating verdict, admitted role, target locus/world,
required admission witness, and grant-policy version for a valid capability
use. `[E-SERVE]` additionally checks required witnesses and visibility, failing
closed without a store mutation. A transport session, locus name, key, role
claim, or a copied reference is not authority.

PROPOSAL-013 asks only where validation claims are retained or recovered. It is
separately recordable from PROPOSAL-012's value-flow and occurrence families;
their compatibility and dependency remain unresolved. This memo must not use a
literature analogy to infer a request field, an aggregate authority record, or
an event/request-instance identity.

## Primary-source contrasts

| Source | Direct observation | Limited use here | Non-transfer to Mir |
| --- | --- | --- | --- |
| [Macaroons (2014)](https://research.google/pubs/macaroons-cookies-with-contextual-caveats-for-decentralized-authorization-in-the-cloud/) | The authors describe macaroons as bearer credentials with contextual caveats. | A credential can carry contextual restrictions while still being a bearer artifact. | Mir theory/05 explicitly says a capref is not a bearer token; no macaroon, caveat, signature, or credential representation is selected. |
| [Souilah, Francalanza, Sassone (2009)](https://www.usenix.org/legacy/event/tapp09/tech/full_papers/souilah/souilah.pdf) | Their distributed provenance calculus motivates provenance annotations, observes that hand-carried origin tags can be forged, and places provenance tracking in a trusted middleware tier. | Provenance useful for a consumer must be backed by a defined tracking/validation relation, not by an unverified payload claim. | Mir does not adopt their pi-calculus, middleware, identity semantics, annotation grammar, or provenance visibility policy. |
| [Hu (2010)](https://arxiv.org/abs/1006.0880) | The paper studies provenance-aware authorization logic and argues that provenance management is useful for authorization/auditing. | Authorization provenance is a distinct concern from a bare policy answer. | It does not choose Mir's lineage carrier, proof interface, or request representation. |
| [Zanzibar (2019)](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/) | The published system reports authorization decisions respecting causal ordering of user actions amid ACL/object changes. | Freshness and ordering are substantive validation concerns, not a reason to infer authority from delivery metadata. | It supplies no request-local or correlation representation for Mir and no permission to import a service/ACL architecture. |

The sources do not establish an exhaustive taxonomy or choose M1/M2/MD. Under
the above Canon, they are used solely to pressure-test three explicit risks:
mistaking carried claims for authority, leaving an owner-selected M2 relation
implicit, and treating freshness/revocation as ambient delivery behavior. The
terms of those requirements come from Canon, not the sources. The only
literature-level result is bounded non-entailment: none of these works justifies
a Mir-specific representation choice.

## Conditional adversarial cases

| Case | Conditional setup | Consequence for a later M1/M2 package | Not a conclusion |
| --- | --- | --- | --- |
| A1 - complete-claim copy or replay | Principal B presents complete-looking claims copied from principal A or A's previous incarnation; source locus and transport may be the same. | Presence and syntactic completeness cannot imply acceptance. M1 must compare claims with authoritative membership, capability-lineage, witness, admission, and history facts. | Does not require cryptography, a trusted transport, or a particular request field. |
| A2 - same-locus correlation alias | Two active principals at one source locus issue semantically distinct requests. | An M2 relation may not associate claims by source locus, transport session, apparent location, or another ambient non-authority property. If it needs an unselected request-instance, queue-entry, event, or side relation, stop for another owner decision. | Does not prove M2 impossible or select an occurrence identity. |
| A3 - leave/rejoin plus save/load | A request or putative association predates leave/revocation; the principal later rejoins with a new epoch/incarnation, and load/rollback makes the old state available. | Both M1 and any owner-selected M2 need current-state/freshness behavior that rejects the stale or replayed use without store mutation; M2 must state save/load behavior. | Does not select clock, latency, retry, transport, persistence encoding, snapshot schema, or a load theorem. |
| A4 - one-component lineage mismatch | Hold visible request data constant while varying exactly one originating verdict, admitted role, target, required admission witness, or grant-policy version. | A later package must account for every theory/05 lineage component; it cannot collapse authority validation to principal/epoch/incarnation. | Does not turn any component into a request-local field. |
| A5 - fresh but unauthorized | A proposed freshness/correlation condition is current, but the capability/witness is missing, target is wrong, or lineage is invalid. | Freshness cannot substitute for authority validation; the request fails closed. | Does not import Zanzibar's zookie, snapshot, or ACL representation. |
| A6 - owner-mediated positive control | A valid owner-directed operation is owner-mediated rather than relying on a bearer-credential-style path. | Comparison must preserve the existing owner-mediated alternative and not assume every valid request is capability-possession authorization. | Does not choose authorization encoding or reply/occurrence semantics. |
| A7 - bearer import | A design argues that possession of a capability reference alone authorizes the request. | Reject the argument: theory/05 requires lineage matching and rejects copied/replayed/severed references. | Does not rule out all external capability research; it rejects only this incompatible import. |

These are proof-facing adverse branches, not executable counterexamples. No
command, helper, model, Lean theorem, or sample is introduced because any such
artifact would have to select the request equality/recovery domain under review.

## Comparative result

M1 is compatible with the literal `carrying` wording only if its values remain
non-authoritative validation claims and are checked against the full settled
lineage. M2 remains conditionally possible only if it can state a relation over
already selected Canon facts without smuggling in a new request-instance,
queue, event, or hidden side carrier. MD leaves existing validation requirements
in force, but prevents a later package from claiming a closed provenance
account.

This is a compatibility screen, not a ranking. It does not show that M1 is
superior, that M2 is implementable, or that MD is required. The owner decision
remains PROPOSAL-013 M1/M2/MD.

## Reopen condition

After an owner disposition, re-screen under ADR-0014 only for an existing LAB
lane with an exact source locus, non-duplicate question, reproducible command,
permitted evidence path, and adverse branch that does not decide another
reserved relation. Do not convert this memo, its URLs, or its conditional cases
into a Core/IR/wire schema, a transport/authentication rule, or an L3 record.

## Non-claims

- No external work is adopted as Mir semantics or evidence of conformance.
- No request field, request-instance identity, queue entry, event identity,
  aggregate authority record, correlation key, cryptographic primitive, or
  trusted middleware is selected.
- No authority, membership, witness, capability, visibility, save/load,
  occurrence, OBL, Gate, Phase, runtime, sample, or public claim changes.
