# Plan 218: C2-B/C3 First Candidate-Card Source Preflight

## Role

This LAB preflight applies Plan 217's method to the smallest possible first
card: cross-locus read through the existing Canon `request` form. It does not
create a hypothesis delta, working record, carrier, or candidate selection.

## Pinned sources and question

Authority cut: `87c05c44b8eb40b7801691795e8bdf06db12eb85`.
Read-only anchors are ADR-0014, theory/01--06, P012, P013, `OPEN-010`,
`OPEN-011`, and Plan 217. The question is whether a `CANON-NATIVE` candidate
card can state every Plan 217 row without adding a request-instance identity,
reply/receipt carrier, pending carrier, or hidden relation.

## Source-ledger result

| Card fact or row | Canon-grounded material | Preflight classification |
| --- | --- | --- |
| `emit` | `request` is a Core occurrence; `[E-REQ]` queues it; owner seriality and DAG are fixed | `CANON-NATIVE` for emission/step boundary |
| M1 validation | M1 permits later request-associated claims; theory/01 and theory/05 name validation facts and fail-closed service | `OPEN`: representation/recovery is expressly unselected |
| type/failure | `request` declares a failure row and `[READ-CROSS]` / `[E-SERVE]` give containment/fail-closed constraints | `CANON-NATIVE` for this conditional row |
| `owner-outcome` | `[E-SERVE]` has pass read+reply or explicit failure occurrence | `OPEN`: exact success reply and requester-observed failure carrier remain open |
| `reply`, `receipt`, `consume` | P012 R1/V1 direction distinguishes typed reply/receipt and restricted result use | `CARRIER-GAP`: `OPEN-011` leaves exact reply/receipt carrier open; no Canon-native linkage supplies consumption |
| `load-frontier` | theory/04 gives full SaveObject/load admissibility and no stale resurrection | `OPEN`: no selected relation connects a loaded request to reply/receipt/use observations |
| authority | theory/05 supplies lineage checks and no transport authority | `CANON-NATIVE` for negative/fail-closed constraints; `OPEN` for request-context representation |
| fallback | theory/06 fixes monotone chain law | `OUT-OF-SCOPE` for this card; no fallback access is claimed |
| ergonomics | P008 remains a later boundary | `NOT-FORECLOSED`, not a satisfaction result |

## Result and stop line

No first candidate card can receive `CONDITIONALLY-SATISFIES`. The minimum
existing card is useful only as a source ledger with `OPEN` and `CARRIER-GAP`
rows. Supplying a link from request to M1 representation, reply, receipt,
acceptance, consumption, or post-load reconstruction would choose a core,
authority, occurrence, or persistence carrier and crosses ADR-0014's reserved
boundary. A helper, queue field, transport/session token, proof relation, or
comparison identifier cannot repair the gap.

## Reopen trigger and non-effects

Reopen only after a Canon/owner design package selects the required semantic
residence, or after an ADR-0014-eligible pre-registered result can remain a
literal transcription/countermodel/conditional lemma without adding a carrier.
This preflight changes no Canon text, L3 record, syntax, runtime, helper, test,
OBL, Gate, Phase, or implementation status.
