# 181 - Preservation-proof prerequisite literature audit

## Role and authority

This is LAB repository memory for a literature-backed readiness audit. The
normative source remains `mirrorea_canon/`. It does not change Core syntax,
`WellFormed`, a transition rule, any OBL/THM row, a contract, a Gate, a Phase,
or an implementation claim. It is not an L3 working record and does not
authorize a proof command or a new evidence lane.

## Question

What minimum proof obligations normally accompany preservation for an
asynchronous owner-queue calculus with an occurrence DAG and hot-plug, and
which of them are already Canon facts versus future proof-interface choices?

## Canon baseline

`theory/01` defines a global OBL-020 target: every step preserves its five
named well-formedness clauses. The listed clauses are occurrence-DAG
acyclicity, grant lineage, publish ancestry, active-or-tombstoned store keys,
and monotone chain positions. The same chapter defines `Q` as per-locus
request queues and includes `[E-REQ]`, `[E-SERVE]`, and `[E-PATCH]` in the
small-step relation. Therefore a later proof may not replace OBL-020 with a
familywise fragment or omit successful patch activation.

THM-006 / OBL-019 are separate: rejected/deferred patch verdicts leave all
non-lifecycle state unchanged. They do not establish successful `[E-PATCH]`
preservation and are not a substitute for its OBL-020 case.

## Literature comparison

The comparison supports proof hygiene only. It does not import session types,
event structures, or a patch calculus into Mir.

- [Honda, Yoshida, and Carbone, *Multiparty Asynchronous Session Types*](https://mrg.cs.ox.ac.uk/publications/multiparty-asynchronous-session-types-jacm/jacm.pdf)
  models asynchronous communication with runtime message queues and proves
  safety/fidelity through a runtime typing discipline. Mir differs by retaining
  owner-directed requests with authority, witness, and failure data rather than
  adopting global protocols as source semantics.
- [Tirore, Bengtson, and Carbone, *Multiparty Asynchronous Session Types: A Mechanised Proof of Subject Reduction*](https://drops.dagstuhl.de/entities/document/10.4230/LIPIcs.ECOOP.2025.31)
  is a useful warning that the exact asynchronous admissibility relation is
  calculus-specific and must be stated, not assumed from a general slogan.
- [Lamport, *Time, Clocks, and the Ordering of Events in a Distributed System*](https://www.microsoft.com/en-us/research/publication/time-clocks-ordering-events-distributed-system/)
  anchors the happens-before comparison. Mir's causal generating family and
  authority/witness edges remain its own semantics.
- [Winskel, *Event Structures*](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-95.html)
  provides an event-structure comparison for preserving causality. It does not
  select an event carrier or insertion rule for Mir.

## Prerequisite catalog

| Area | Canon fact already fixed | Necessary proof-side fact | Still unselected |
| --- | --- | --- | --- |
| Request queue | `Q` stores owner queues; requests carry authority/witness/failure data; `[E-SERVE]` validates before serving. | A legal step must ensure queued requests are admissible enough that dequeue, validation, and service preserve the five named clauses. | Whether this is a component of a future proof carrier, a `RequestOK` relation, or a transition premise. |
| Occurrence DAG | `H` is acyclic; theory/04 fixes the causal generating family. | Each occurrence insertion needs a preservation argument, for example a fresh occurrence plus edges that cannot close a path, or an equivalent `Acyclic(H')` lemma. | Exact carrier, freshness representation, and safe-insertion presentation. |
| Owner seriality | Per-owner queue service is serial; cross-owner interleaving remains nondeterministic. | The formal transition relation must expose enough dispatch/order information for the affected rule case. | Scheduler representation and whether seriality is a relation invariant or a rule side condition. |
| Patch activation | `[E-PATCH]` activates only at a matching frontier; theory/08 fixes compatibility/frontier/patch-DAG requirements. | Successful activation must be one case of global preservation. A separate patch predicate is needed only for properties outside the present five clauses. | Placement and shape of any lifecycle/compatibility invariant in the proof interface. |

The right-hand column is a catalog of future proof-package decisions, not a
list of missing Core primitives. In particular, this document does not extend
Canon `WellFormed` with queue or patch fields.

## Current evidence disposition

The active Lean graph has no new source locus that both avoids prior evidence
and provides an immediate non-reserved branch under the current LAB priority
screen. OBL-020 family coverage is already WRK-0006; the direct Core/result
and outcome-totality boundaries are owner/canon matters; OBL-024/025 and the
foundations are either screened as duplicate/no-consumer or reserved mapping
work. This is a current LAB prioritization result, not an additional
ADR-0014 eligibility rule and not a closure of future research.

## Next integration point

After the required owner/canon proof-interface decisions, a common proof model
can use this catalog as a completeness check before proving the existing global
OBL-020 statement. Until then, do not add an abstract queue predicate, a toy
DAG model, a patch invariant, or more helper-local lattice lemmas merely to
manufacture a green Lean result.

## Non-claims

- No queue typing, event identity, safe insertion rule, scheduler, patch
  predicate, or final label system is selected.
- No new language primitive, runtime behavior, adapter, schema, helper,
  Make/CI surface, or public API is proposed.
- No OBL/THM is stated, proved, discharged, or moved in `theory/11`.
- No Gate/Phase, conformance, workflow, or public completion claim changes.
