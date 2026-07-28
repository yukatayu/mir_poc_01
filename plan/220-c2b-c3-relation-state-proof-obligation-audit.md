# Plan 220: C2-B/C3 Relation-State Proof-Obligation Audit

## Role and authority

This is a **LAB conditional audit** of Plan 219 option A. It does not accept
that option, define its relation, create a Core transition, or claim a proof.
Its purpose is to prevent a later normal Canon proposal from converting a
compact-looking configuration component into an implicit proof or a hidden
runtime carrier. `mirrorea_canon/` remains normative.

The audit is not an ADR-0014 `working/` record: it retains no theorem,
countermodel, helper, schema, source rule, implementation, OBL status, Gate,
Phase, or public behavior. It records the obligation/falsifier shape that a
selected design must state before any formal model can rely on it.

## Authority cut and question

Review cut: `59221216ceeef0223614080d71eb0859b247138b`.

The relevant fixed/bounded inputs are theory/01--05, theory/07,
ADR-0014, P012 V1/R1, P013 M1, and Plans 209, 210, 215--219. They require
explicit request/result correlation, typed dynamic failures, causal ordering,
authority validation, admissible save/load, redaction, and a one-shot result
use direction, but do not select a carrier.

Question: if a normal Canon proposal chooses a relation-valued exchange state
anchored by the current-history request occurrence, what must it define and
prove so that the proposal is neither an under-specified history projection nor
an overclaimed exactly-once/freshness/transport model?

## Corrected conditional reading of option A

Plan 219 survives this audit only under the following reading:

1. `q` identifies a request occurrence **within one current or restored
   history**, not a source, wire, session, or cross-load global identity.
2. The exchange relation contains an explicit pending disposition. A terminal
   owner outcome is *at most one* per `q`; an outstanding `q` is not forced to
   have an outcome.
3. P012 R1 requires distinct typed owner-result and requester-receipt causal
   steps. The audit only calls a receipt transition semantic once it meets the
   chosen acceptance predicate; raw adapter delivery remains an implementation
   refinement, not an implicit at-most-once property.
4. A result/provenance record and an observer-facing redacted projection are
   distinct. No raw result is made public merely by being stored in exchange
   state or history.
5. Save/load preserves a declared relation correspondence and causal/channel
   closure, not occurrence equality across unrelated restored configurations.

These are constraints on a later proposal, not a definition of its fields,
arity, final event names, storage representation, syntax, or protocol.

## Required definition and proof obligations

| ID | A later selected design must state | What must be established | Decisive falsifier |
| --- | --- | --- | --- |
| X-DEF | exchange relation, dynamic domain, pending/terminal/accepted/consumed-or-failed dispositions, and which facts are stored versus derived | no semantic state is carried only by a proof relation, evaluator side table, queue position, span, or transport metadata | two equal-payload requests share an effective pending/result state without a selected relation explaining why |
| X-M1 | request-associated non-authoritative M1 claims and separately named authoritative validation grounds | copied, stale, wrong-target, missing-witness, or severed-lineage claims fail closed without owner mutation | validation succeeds by ambient/transport recovery or by treating a claim as authority |
| X-BRANCH | owner success/failure terminal branches and pending disposition | a branch is exclusive when terminal; owner failure has no owner-store mutation; absence of receipt is not failure | one `q` supports both accepted success and failure, or missing delivery is treated as failure |
| X-TYPE | result type, operation effect row, and all selected dynamic failure rows | no ill-typed result reaches consumption; receipt rejection is either not a semantic transition or is declared in the request failure row; ambiguity is static Diagnostic | a runtime rejection is outside the row, or an ambiguous source form becomes a hidden runtime path |
| X-CAUSAL | request-to-service, owner-outcome/reply-to-receipt, accepted-receipt-to-consumption/dependency edges or selected equivalent projections | every claimed causal edge is represented without a cycle; owner service remains serial and the later action is not credited with a result it did not consume | a receipt or dependent occurrence exists without the required predecessor, or a selected edge closes a cycle |
| X-LINEAR | exact `Gamma`/`Delta` disposition on success and failure, and the zero-occurrence consumption transition | at most one **accepted consumption** per branch in one trace, including after load; no statement about raw delivery multiplicity follows | an accepted receipt can resume twice, or a consumed branch becomes enabled after load |
| X-OBS | provenance and redaction projection from result state to observer-visible data | raw result/provenance is not exposed as history/telemetry merely because exchange state exists; observer authority remains separate | an unauthorized observer learns the unredacted value from `X`, history, or debug output |
| X-LOAD | complete `SaveObject` placement, cut/channel closure, restore correspondence, and post-load live-state predicate | every saved exchange fact retains all required predecessors or saved in-flight/channel fact; stale authority is not resurrected; recorded service is not silently replaced by revalidation against current state | save after owner result and before receipt loses/duplicates the branch, reruns a consumption, or validates against different post-load grounds |
| X-SCOPE | explicit exclusions for retry, timeout, cancellation, fairness, delivery, global exactly-once, freshness, and read-modify-write atomicity | no proof or conformance test quietly relies on an excluded property | a claimed safety result uses eventual delivery, global identity, snapshot freshness, or atomic fusion as an unstated premise |
| X-ELAB | a later admitted source-convenience fragment and full explicit elaboration record | one checked complete artifact or one Diagnostic, preserving every selected fact, generated edge, row, authority obligation, provenance/redaction, dependency, and load fact | two semantically different explicit records elaborate from the same admitted omission, or an inferred fact depends on incidental source data |

None of these rows is discharged by this audit. In particular, `X-LINEAR` and
`X-LOAD` cannot be proved from the current occurrence DAG alone, and `X-OBS`
cannot be proved merely by storing provenance.

## Cross-check against the three alternatives

| Alternative | Audit result | Reason |
| --- | --- | --- |
| A: explicit relation state | conditionally viable as a **decision envelope**, not as a proved semantics | it has a named location for every obligation, but each row above still needs a selected definition and proof/falsifier |
| B: history-only projection | inadequate at the current cut | it has no location for `X-DEF`, zero-occurrence consumption, redaction record, or restore correspondence without reintroducing relation state invisibly |
| C: nominal identity | reserve only | it can locate facts, but adds freshness, equality, non-reuse, persistence, and replay obligations without present evidence that A cannot satisfy the matrix |

This does not prove Family B impossible or Family C wrong. It only prevents a
future proposal from claiming that B is simpler while it leaves one of the
required facts unlocated, or that C is necessary before an A falsifier exists.

## Falsifier scenarios required before reliance

The normal proposal must make a selected result explicit for each scenario:

1. two active requests share payload and M1 claims;
2. a copied or stale request claims valid authority;
3. owner success is saved before requester receipt;
4. accepted receipt is saved before zero-occurrence consumption;
5. owner failure, no receipt, and requester-side receipt rejection are distinct;
6. a duplicate or wrong-locus candidate receipt arrives;
7. result observation is attempted by an unauthorized or redacted observer;
8. the same source site emits two dynamic requests;
9. an owner-local read occurs; and
10. fallback access is either excluded or satisfies its separate lineage law.

A conformance schedule may witness these scenarios after a design exists; it
cannot be the missing relation, proof, or delivery assumption itself.

## Stop line and next autonomous work

No Lean theorem, runtime experiment, Core rule, or `working/` record follows
from this audit. A precise formal statement would immediately choose the
relation's carrier, branch algebra, restore relation, and receipt transition,
which are reserved normal Canon decisions.

After an owner/Canon choice, this matrix yields a concrete existing-lane
research queue: pre-register one narrow countermodel or conditional lemma at a
time, with a stated row, alternative, falsifier, non-effects, and rollback
trigger. The first formal model must cover only the selected scope; it must not
promote a passing fragment to OBL-020, delivery, or product completion.

## Non-effects

This audit changes no Canon document, decision level, Core constructor,
configuration schema, SaveObject, history relation, failure family, authority
rule, diagnostic catalog, source grammar, elaboration rule, runtime, adapter,
wire/API contract, theorem/OBL, scenario, Gate, Phase, sample, implementation,
or public claim.
