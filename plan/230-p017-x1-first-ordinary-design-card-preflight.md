# Plan 230: P017 X1 First Ordinary-Design Candidate-Card Preflight

## Role and authority

This is LAB design preparation after Plan 229. mirrorea_canon/ remains
normative. P017 selected only the X1 relation-state family for one V1/R1
cross-locus read. It has not selected a relation schema, field layout,
identity/equality discipline, transition, occurrence kind, causal generator,
failure row, persistence representation, source form, runtime, or public
contract.

This plan prepares two unselected candidate-native cards to expose dependencies
and stops. It is not a working/WRK record, executable experiment, positive
model, Canon amendment, or statement of conditional compatibility. A card may
conclude only open, Canon gap, out of scope, or preflight structurally complete;
neither card reaches the last status here.

The cards are not a shared interface. Their only shared material is exact Canon
constraints and review headings. No common Interaction, Pending, lifecycle,
identifier, restore function, or candidate data type is introduced for
comparison convenience.

## Authority cut and scope

DIRECT constraints are P017 X1, ADR-0014, theory/01--05 and theory/07 where a
row uses them, plus Plan 227's candidate-native C + H_K + D_K contract. Plan
229 closes further abstract L3 work and directs the next substantive work to
ordinary design preparation. WRK-0044 may be relied on only for its
explicit-premise five-pair static distinction account.

Scope is one V1/R1 cross-locus read. Owner-local reads, other
result-returning operations, writes, admission, fallback, syntax/elaboration,
transport, adapter, wire/API, serialization, provider, runtime, retry,
timeout, cancellation, fairness, delivery, termination, global exactly-once,
freshness beyond required M1 authority lineage, observation/export, proof,
OBL, conformance, Gate, Phase, and implementation are out of scope. The
observation gate is closed for both cards. That is no observation claim, not a
future prohibition on separately typed theory/07 projection.

## Common review classification

Every semantic noun, relation, state, transition, order, and restore fact is
classified as exactly one of:

- C: exact pinned Canon constraint;
- H_K: reversible candidate-local hypothesis;
- D_K: erasable candidate-local definition over C + H_K;
- OPEN: deliberately unresolved material;
- Canon gap: a required reserved choice; or
- out of scope.

An unclassified item is a hidden choice. Neither card may use source span,
payload equality, queue position, transport metadata, evaluator/proof side
tables, or an unlisted key as semantic residence or correlation.

## Candidate K0: external-rejection seed card

### K0 status

open. K0 is the smallest P017-permitted seed in which raw adapter delivery
rejection remains outside the semantic exchange transition system. It is not
selected because relation presentation, branch, acceptance/use, occurrence, and
persistence details remain unresolved.

### K0 classification

| Class | K0 content |
| --- | --- |
| C | P017 X1 requires an explicit relation-valued component anchored by current or admissibly restored request occurrence, dynamic domain, one requester-side pending binding per in-scope request, separate M1/provenance, owner branch, result/receipt/acceptance/use distinctions, existing theory/04 causal basis, no implicit observation, and save/load closure. Claims and provenance are not authority. |
| H_K0-R | A candidate-local semantic relation has a dynamic request-occurrence domain and one non-shared pending administrative binding per occurrence. Its mathematical presentation is not fixed and no public, global, or cross-load identity is assumed. |
| H_K0-B | An owner-side branch has outstanding and at-most-one typed terminal success or owner-service failure. Validation provenance refers to consulted grounds but is not authority. Failure representation and validation algorithm remain open. |
| H_K0-T | A raw rejected delivery candidate is outside semantic exchange. It leaves receipt-pending, owner-result availability, and accepted-use budget unchanged. It is not semantic receipt, requester failure, persisted state, adapter event, or whole-machine stutter claim. |
| H_K0-U | Only accepted success may enable one restricted use. Gamma/Delta disposition, consumption representation, and execution-wide argument remain open. |
| H_K0-C | Any future relied-on order must map to an existing theory/04 generator. K0 currently defines no occurrence, administrative transition, causal edge, or order. |
| H_K0-L | Every live exchange fact needs abstract restore correspondence and cut/channel closure without merge, duplicate, reset, revalidation, stale resurrection, or independent-load equality. No restore function, storage representation, Config, or SaveObject field is selected. |
| D_K0 | Candidate-local names may distinguish relation membership, pending binding, owner outstanding/success/failure, result availability, receipt-pending, acceptance, accepted/unconsumed, consumed, provenance reference, and correspondence. They are not a shared schema, total lifecycle classifier, source construct, or runtime object. |
| OPEN | Relation presentation; M1 validation; failure row; result carrier; receipt matching/transition; Gamma/Delta; occurrence accounting; exact theory/04 mapping; live-fact closure; restore relation; dynamic evidence. |

### K0 row and stop ledger

| P017 row | Current K0 account | Required future evidence or stop |
| --- | --- | --- |
| dynamic residence | H_K0-R keeps pending state semantic and non-shared in principle | stop for concrete schema, key, or hidden residence; later card states primitive versus uniquely derived facts |
| M1 and authority | H_K0-B keeps claims/provenance separate from authority | stop for validation algorithm, failure mapping, or authority primitive |
| branch and type | H_K0-B names separation only | stop if a failure member/row or owner-mutation rule is required |
| receipt and one-shot use | H_K0-T fixes external rejection; H_K0-U keeps use abstract | stop for Core, transition, or dynamic-failure selection; WRK-0044 is not execution-wide one-shot evidence |
| causal basis | no order claimed | stop if a proposed order lacks existing theory/04 generator or needs occurrence kind/new generator |
| observation | no observer claim | stop if storage/history becomes visibility or export |
| save/load | H_K0-L is only an obligation inventory | stop for global identity, restore function, or new Config/SaveObject surface |
| source boundary | no source claim | stop for correlation from source/payload/span/queue/history/transport or a G_e row |

## Candidate K1: typed-requester-rejection delta card

### K1 status

Canon gap. K1 differs from K0 only at P017's other permitted rejection
treatment: a rejected delivery candidate is a separately typed requester-side
transition in the request's declared dynamic failure row. It is not selected
and does not inherit K0 definitions.

### K1 classification

| Class | K1 content |
| --- | --- |
| C | P017 permits typed requester-side rejection only when it is in the request's declared dynamic failure row with explicit terminal/nonterminal receipt-pending effect. It requires ordinary amendment if this needs a new failure member or closes OPEN-010. Requester rejection is not owner-service failure. |
| H_K1-R | A candidate-local relation has a dynamic request domain and non-shared pending binding, with no public/global identity or hidden residence. |
| H_K1-B | Owner branch and M1/provenance remain separate from requester-side treatment; no owner failure or mutation follows from rejection. |
| H_K1-T | A candidate-local typed requester rejection would have declared dynamic-row membership and explicit receipt-pending effect. The exact row/member is not assumed. |
| H_K1-U | Rejection must state whether receipt-pending remains eligible for a later candidate or becomes terminal, and cannot enable accepted use. Exact disposition remains unchosen. |
| H_K1-C | Any semantic rejection transition would need occurrence/administrative classification and theory/04 causal mapping. No such mapping is supplied. |
| H_K1-L | A selected rejection state is a live-fact/closure frontier if it survives load. Storage and restore form remain unchosen. |
| D_K1 | Candidate-local names can distinguish proposed rejection from owner failure, receipt, acceptance, and use. They do not create a Canon failure member, occurrence, transition, or persistence field. |
| Canon gap | No pinned declared dynamic failure row/member is selected for requester-side transition. Membership and terminality cannot be supplied by an erasable card without choosing reserved failure surface or closing OPEN-010. |

### K1 decisive stop

K1 stops before positive or conditional result. It cannot become structurally
complete until ordinary Canon design identifies a row-contained treatment
without inventing a member, collapsing it into owner failure, or silently
resolving OPEN-010. This is a contrast with K0, not a rejection of P017's
permitted future option.

## R/B/T/U/C/L dependency ledger

| Coordinate | K0 | K1 | Coupled boundary |
| --- | --- | --- | --- |
| R residence/reference | open | open | review with L; no incidental correlation or global identity |
| B owner branch/type/provenance | open | open | claims/provenance never become authority; owner failure stays distinct from K1 rejection |
| T receipt/rejection | external negative scope only | Canon gap | minimal K0/K1 delta; no delivery/fairness/wire claim |
| U restricted use | open | open | review with receipt effect and load; neither card proves one-shot execution behavior |
| C occurrence/causality | open | open | actual order waits for candidate-native fact/transition inventory and existing generator |
| L persistence/restore | open | open | live-frontier inventory depends on B/T/U/C; no function/equality/storage form assumed |

## Staged exploration after this preflight

1. Pin the Canon/LAB cut and transcribe P017 integration rows with Plan 227
   adversarial cases into a source-backed obligation map.
2. Explore candidate-native R/L skeletons together: domain, reference scope,
   pending binding, primitive/derived facts, live frontiers, and abstract
   correspondence. Eliminate a skeleton on hidden residence, identity, or
   schema dependency.
3. Add B to surviving R/L skeletons, then fork T explicitly at K0/K1. K1
   remains stopped until row-contained failure treatment exists.
4. Add U and C together: candidate-local acceptance/use transition,
   Gamma/Delta proposal, occurrence/administrative classification, and
   theory/04 mapping. No dynamic invariant is established here.
5. Reclose L against the resulting live-fact inventory. Do not claim no merge,
   duplicate, reset, revalidation, or stale resurrection without later dynamic
   evidence.
6. Run Plan 227 adversarial cases against each card separately, then compare.
   Classify results only as open, Canon gap, or out of scope.

## Dynamic-evidence cap and stop conditions

WRK-0044 does not establish transition existence, reachability, semantic
receipt, execution-wide one-shot behavior, restore functionality, merge
prevention, causal order, or runtime behavior. A card must mark those matters
not established until later registered dynamic work supplies a concrete claim,
consumer, falsifier, existing-lane execution plan, and no reserved dependency.

Stop an affected card when it requires hidden residence/identity; a
Core/authority/ownership/effect/failure/judgment/occurrence/causal/source/runtime
primitive; dynamic failure row; unmapped causal order; global restore identity,
function, schema, or persistence representation; observation/export; new
helper/evidence/CI lane; or theorem/OBL, implementation, Gate, Phase, or public
claim. A stop identifies ordinary Canon boundary, not a negative X1 decision.

## Preflight disposition

K0 is an explicit incomplete external-rejection seed. K1 is an explicit
typed-rejection contrast that stops at the unselected failure row. The
preflight exposes that ordinary comparison begins with R/L and B, while T is a
real design fork rather than a default inherited from WRK-0044.

Neither card is a selected relation model, proof, implementation plan, or Canon
proposal. A later evaluation can register only after it supplies concrete
candidate proposition, independent consumer, decisive falsifier, and
existing-lane plan meeting Plan 229 reopen conditions.

## Non-effects

This plan changes no Canon text, working record, Core, Config, SaveObject,
relation schema, identifier, transition, occurrence, causal family, failure
row, authority rule, observation surface, source grammar/elaboration, runtime,
adapter, wire/API, theorem/OBL, scenario, Gate, Phase, sample, implementation,
or public behavior.
