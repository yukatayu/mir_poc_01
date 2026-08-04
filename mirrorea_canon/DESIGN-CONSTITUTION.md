---
id: root/design-constitution
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, adr/ADR-0015]
summary: Mir Theory v0 / deterministic I1+ の横断的な設計判断規則。詳細 carrier、grammar、algorithm を固定せず、後続 theory/spec/implementation を拘束する。
open_items: []
---

# Design Constitution — Mir Theory v0 / I1+

This Constitution is the compact decision filter for the owner-approved Mir
Theory v0 and deterministic I1+ reference-system program. It is subordinate to
the North Star and ADRs, and constrains later theory, specification,
implementation, and evidence. It is not a grammar, Core schema, wire/API
contract, runtime algorithm, proof catalogue, lifecycle verdict, or product
claim.

## C1. Authority, scope, and decision procedure

Canon is the only normative source; LAB is evidence and implementation. The
v0 outcome is one ordinary `.mir` source whose checked meaning can be parsed,
statically checked, elaborated into typed Core/obligations/generated
communication, run by a deterministic single-process logical multi-locus
reference runtime, traced, saved/loaded, patched, projected, and diagnosed.

For an ordinary design question, decide in this order: (1) semantic meaning
and owner, (2) authority origin, (3) evaluation site and trigger, (4)
communication/effect/failure/information release, (5) materialization,
frontier, version, and lineage. The only outcomes are `allowed`, `allowed only
with an explicit semantic boundary`, `deferred to a named later milestone`,
`prohibited by v0`, or `owner escalation required`. Ambiguity never becomes an
implicit default.

## C2. Ordinary Surface and semantic strata

Surface expresses ordinary declarations, state, assignment, expressions,
references, relations, handlers, and bounded policy annotations. It does not
make send/receive machinery, occurrence identifiers, message envelopes,
transport retry, witness carriers, proof terms, or an unbounded `Event` the
ordinary programming model. Surface intent, Core operation/request, runtime
occurrence, domain event, and observation row are distinct.

The exact syntax, accepted-form inventory, and Surface-to-Core totality are
deferred to M6 after the shared meaning is closed. A domain event may remain an
explicit handler input; it is never a synonym for a Core request or occurrence.
`World`, `Room`, `Avatar`, and `Game` remain domain/library vocabulary, never
Core primitives.

## C3. Meaning precedes communication

Communication interfaces, per-locus programs, and generated edges are
projections of checked source meaning. They preserve owner, dependencies,
authority obligations, effect/failure rows, visibility, lineage, and source
spans. No elaboration, optimization, transport, provider, or cache may create
hidden communication, authority, effect, failure, information release, or
transaction.

## C4. Evaluation is a coordinate, not authority

Semantic form (`value | state | relation | computation`), evaluation site
(`owner | locus | designated evaluator | consumer | provider`), trigger/clock,
authority origin, and materialization (`local only | store | publish value |
publish relation | adapter stream | persist`) are independent semantic
information.
Evaluation at a locus never grants that locus's ambient authority to the
requester. Authentication evidence identifies a principal; admission and a
validated capability lineage authorize a mutation.

The deterministic default is: owner-side mutation first; owner-private
calculation stays owner-side; a safe pure relation may remain late at a
consumer; an authoritative decision uses a designated evaluator; declared
handler/provider boundaries host external effects or nondeterminism. If these
rules do not select one meaning, checking requires the smallest annotation or
returns a Diagnostic. It never infers multi-owner atomicity.

## C5. Owner mutation and explicit cross-owner boundaries

An owner serializes every mutation of its state. If the mutable reads and write
of a read-modify-write expression have that owner, the owner evaluates the
whole bounded transition: two accepted attacks against `hp = 100` yield
`100 → 90 → 80`, not two requester-side reads followed by stale blind writes.
The requester's authority origin remains explicit.

When an operand belongs to another owner, the program must elaborate an
explicit remote result/receipt dependency or be rejected as outside the v0
ordinary fragment. Same-owner seriality is not a distributed snapshot, lock,
transaction, or exactly-once guarantee.

## C6. Designated materialization

A designated evaluator makes an authoritative semantic decision only when its
identity/site, policy, trigger, input frontier, result version, authority
evidence, and observation policy are bound in Core consequence. Consumers may
interpolate or coordinate-convert for presentation, but do not silently
re-evaluate that semantic decision. Evaluating a result does not transfer
authority over its inputs; stale or mismatched results never become current
authoritative state.

## C7. Relations and late projection

A maintained reference/transform/fallback relation is semantic source of truth
when it defines a value. Preserve the relation DAG and factor shared structure
as long as possible. A consumer may locally evaluate the projected relation in
its presentation context, but never becomes its semantic owner and need not
receive a per-frame absolute-value stream.

The Constitution neither requires nor selects a derived-value stream. M4 may
admit one only at an explicit relation-, authority-, and policy-preserving
boundary; it cannot silently replace the maintained relation.

For example, B may own `bird follows A.shoulder, fallback B.shoulder`; C uses
the same admitted A/B presentation samples to derive the bird locally. This
must preserve the finite-fragment law `project then evaluate ≃ evaluate then
project`, including declared redaction and approximation. Ownership, authority,
information-flow policy, semantic lifetime, capability validity, patch
compatibility, authoritative fallback, and mutation are never pushed down to a
provider or renderer.

## C8. Two fallback domains

Semantic fallback responds only to semantic invalidation: existence,
membership/incarnation, lease, authority, or relation-lineage loss. It records
the applicable occurrence/frontier and advances monotonically on one lineage.
Returning to an earlier option requires explicit fresh reacquire with a new
witness/epoch.

Presentation fallback responds to a consumer-local sample gap, latency budget,
temporary packet loss, renderer limitation, interpolation, prediction, or LOD.
It may not alter semantic state, authority, lineage, fallback position, or
occurrence history. A stale anchor sample is not admissible for semantic use.

## C9. Extensions, authority, and observation

Credential/authentication evidence → principal claim → admission/policy
decision → capability grant → validated use is the authority chain. Role,
key, locus, session, transport, provider, package, runtime kind, and signature
alone are not authority.

Runtime policy layers transform typed Contracts. A non-transparent change to a
precondition, failure row, capability requirement, observation/redaction, or
retention requires an explicit ContractUpdate, admission, and activation cut.
Verification is separate: its modules map Judgments/Obligations to Evidence,
Diagnostic, or ResidualObligation. Neither kind of extension may mint
authority, permit an undeclared effect, erase failure, redefine Core, or weaken
information flow.

Observation, devtools, telemetry, and visualization are typed information
effects with observer principal, label, redaction, retention, authority, source
span, and reason/proof references. A derived relation cannot weaken any input
visibility or redaction policy.

## C10. Persistence and evolution

Save/load is a consistent-cut operation, not a byte copy. It includes the
relevant owner state, occurrences/queues, membership/incarnation,
capability/witness provenance, fallback/relation lineage, evaluator result
versions, receipt/consumption state, patch lifecycle, and verification
invalidation state. Load validates before mutation and never resurrects stale
membership, witness, lease, consumed result, revoked capability, old lineage,
or rejected patch state.

Patch is not `eval`: parse → check → elaborate → compatibility →
capability/admission → verdict → activation cut → runtime mutation → trace.
Rejected/deferred patches change only lifecycle rows. v0 covers bounded
single-session patching, not durable distributed migration or activation
ordering.

## C11. Assurance and finite scope

The assurance lines remain separate: finite/decidable static checking, bounded
model checking, and proof. Mir source is not a proof language. The v0 fragment
uses finite orders/lattices, powersets, lifetime preorders, capture inclusion,
finite keyspaces, simple resource bounds, effect/failure rows, and
capability/visibility/fallback constraints. Stronger claims become explicit
ResidualObligations; bounded evidence is never a general proof.

M3--M5 define the shared formal model and exact proof inventory; M6--M10
provide the bounded syntax, checker, runtime, extensions, and conformance.
This Constitution makes no proof-status, SCN pass, Gate/Phase, or I1
authorization claim.

## C12. Priority, deferral, and escalation

Choose between valid internal designs in this order: (1) meaning preservation;
(2) authority, privacy, and safety; (3) ordinary simple Surface; (4) no hidden
communication/failure/effect; (5) smaller orthogonal Core; (6) finite
decidability; (7) modular proofability; (8) conservative extensibility; (9)
implementation simplicity; (10) performance. Compare only the current design
and one smallest viable alternative; close the question once the accepted
choice has a positive case, a representative falsifier, and an extension path.

Existing deferred scope is not a new permanent non-goal. Escalate only to
change the North Star; weaken authority/privacy/redaction/no-stale-resurrection;
promote domain vocabulary into Core; make a declared v0 non-goal mandatory;
irreversibly freeze a final public API/ABI/wire; start deployment/publication;
choose tied irreversible alternatives; or risk user data/secrets.
