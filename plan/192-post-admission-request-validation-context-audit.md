# Plan 192 - Post-admission request validation-context audit

## Role and authority

This is LAB repository memory for a read-only source audit at clean
`497b8b0d`. `mirrorea_canon/` remains normative. The associated
PROPOSAL-013 is a decision request, not a Canon answer. This plan opens no
`WRK-####`, adds no Core/request/queue carrier, changes no theorem or
obligation, and moves no Gate, Phase, conformance, implementation, or public
status.

## Question

Do the existing Core request, authority, runtime, and Core-IR descriptions
state where `[E-SERVE]` obtains the principal, epoch, and incarnation that it
must validate for a post-admission request? If not, is this already resolved
by a prior proposal or a standing-eligible L3 experiment?

## Literal source comparison

| Canon locus | Stated fact | What it does not state |
| --- | --- | --- |
| theory/01 Core syntax and Config | `request` has source/destination locus, operation, values, capability refs, witness refs, and failure row; `Q` queues requests. | A request principal, membership epoch, incarnation, or a relation that recovers them. |
| theory/01 `[LOCUS-BLOCK]` | A non-owner body elaborates requests carrying origin principal, epoch, incarnation, capability/witness refs, and spans. | The closed Core/queue/generated-edge carrier or recovery relation for those values. |
| theory/01 `[E-SERVE]` | Serving validates epoch, incarnation, capability lineage, witnesses, and visibility before a no-mutation failure or service. | How the dequeued request supplies the values used for validation. |
| theory/05 authority | Post-admission messages carry principal, epoch, incarnation, capability refs, and required witness refs; capability lineage validates all named components. | Whether those values live in a Core request, a queue relation, a generated-edge row, or another semantic carrier. |
| spec/04 Core-IR exchange form | The illustrative request edge records endpoints, operation, capability refs, witnesses, failures, and span; the example transition uses `req.principal`. | A closed request-context schema or a proof that `req.principal` is recoverable. |
| spec/05 runtime semantics | The request lifecycle validates epoch, incarnation, capability lineage, witnesses, and visibility. | The representation or semantic provenance of each validation input. |

## Prior-work and duplication check

- Plan 180 records the broader statement-identity boundary but does not select
  a request-validation representation.
- Plan 186 deliberately excludes request-envelope interpretation while it
  separates Core value flow and service/admission occurrence identity.
- PROPOSAL-012 asks about value flow, read receipt, served-write identity, and
  admission identity. It explicitly does not create a request/result carrier
  or let LAB envelopes tie-break a shared semantics; separate recordability is
  not compatibility or causal isolation.
- Plan 191 correctly prevents a speculative L3 successor: an experiment that
  chooses a request field, lookup, correlation, or helper envelope would repair
  the gap by selecting the very owner-reserved interface under review.

This is not a replay of a frozen WRK and not proof that the Canon is
inconsistent. It is a distinct owner decision surface: preserve enough
post-admission validation context without using transport identity as authority.

## Research disposition

No L3 experiment is selected. A finite counterexample would have to assume a
request equality or recovery function, and a positive run would have to choose
a representation. Neither outcome can establish the intended Canon reading
without crossing the Core/authority boundary.

The appropriate next action is PROPOSAL-013. Its alternatives are deliberately
representation families rather than field names: request-local validation
claims (M1),
an explicit non-transport correlation (M2), or defer (MD). The proposal leaves
PROPOSAL-012 compatibility/dependency, BND-001 totality, direct-Core OBL-001
work, and occurrence identity unresolved rather than treating them as
independent.

## Advisory review and correction

A temporary Oracle foundation review independently found no eligible next L3
package and recommended a broad premise-closure matrix. Its claims were checked
against Plans 180, 181, 182, and 186 before use. Mutual `depends_on` references
are permitted knowledge dependencies under the Canon style guide, and `G_e`
dependency membership is already settled by the Canon text; neither is carried
forward as a new defect. The useful non-duplicative finding is the request
validation-context boundary recorded here.

The same review noted that Plan 191's binary consumer condition is a LAB
selection heuristic, not an additional ADR-0014 eligibility rule. This plan
does not change Plan 191's source-cut disposition or select an L3 record; it
only avoids treating that heuristic as a Canon prohibition.

## Reopen condition

After an owner disposition, re-screen under ADR-0014 only if an existing
documented LAB lane supplies an exact non-duplicate question, a reproducible
command, a permitted evidence path, and an adverse branch that does not choose
another reserved relation. Do not reuse an ignored helper-local envelope or
modify a frozen WRK as evidence.

## Non-claims

- No request, queue, authority, membership, witness, transport, or runtime
  carrier is selected.
- No PROPOSAL-012 compatibility or dependency is asserted.
- No anti-spoofing invariant is weakened or proved.
- No THM/OBL, `theory/11` status, proof, Gate, Phase, conformance,
  implementation, or public claim changes.
