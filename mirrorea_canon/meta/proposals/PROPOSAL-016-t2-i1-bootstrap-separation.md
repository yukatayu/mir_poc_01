---
id: meta/proposal-016
status: L3-open
maturity: draft
depends_on: [plan/00-gates, plan/01-phases, spec/06-conformance, theory/11-metatheory-ledger, adr/ADR-0014]
summary: narrow T2 と separate I1-readiness/bootstrap、C-static formal entry の owner disposition を記録する。現行 Phase は動かさない。
open_items: []
---

# PROPOSAL-016 - T2/I1 bootstrap separation

> Decision-request artifact. The owner disposition is recorded below. It
> authorizes a later lifecycle/profile wording package only; it is not a Gate
> exit, Phase entry, production implementation authorization, or conformance
> result.

## Target and authority boundary

Current Canon describes T2 in terms of proof skeletons and G5 statements, while
`spec/06` calls C-static 10/10 formal I1 entry and the phase table also retains
C-static with C-runtime/carrier-freeze I1 exit evidence. The sources do not yet
define a profile that distinguishes permission to begin a bounded
single-process implementation from formal I1 entry and exit.

## Owner disposition

Recorded on 2026-07-28:

1. **Narrow T2 plus separate I1 readiness.** T2 remains the proof-skeleton/G5
   stage. A later, separately accepted I1-readiness/bootstrap record must bind
   all-SCN implementation scope and the implementation-facing semantic
   interface; T2 is not silently overloaded with that work.
2. **Explicit bootstrap, then C-static formal entry.** A future bounded
   bootstrap authorization may permit work on the selected deterministic
   single-process reference implementation. C-static 10/10 remains the formal
   I1 entry milestone and remains required evidence at I1 exit alongside
   C-runtime 10/10 and carrier freeze.

## Required follow-up boundary

Before any lifecycle amendment or implementation authorization, the later
profile package must bind: the frozen SCN-01..10 positive/negative corpus; the
deterministic profile and report meanings; the selected statement-level
semantics for elaboration, authority, fallback, patch, and cut/save/load; the
explicit evidence class of every open OBL consumed or deferred; and the narrow
production-moratorium exception, if any. It must reconcile the C-static
entry/exit wording in `plan/01-phases` and `spec/06` through the ordinary Canon
process.

## Non-effects

This disposition does not:

- move T0, G0, T1, T2, or I1; accept a profile; alter an SCN; or change an OBL
  status, target, wording, or proof;
- narrow I1 below all ten frozen scenarios, select a parser/checker/runtime
  architecture, freeze carrier fields, or authorize a public implementation;
- decide G4/G6/G7 or OBL-003/027 classification; those must be explicit in the
  later readiness profile; or
- authorize a new helper, schema, CI surface, Make target, or evidence lane.
