---
id: theory/08-patch-hotplug
status: L1-fixed
maturity: draft
depends_on: [theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority, adr/ADR-0006]
summary: patch pipeline、互換 carrier、frontier 束縛 activation、否定例、THM-006。
open_items: [OPEN-021]
---

# 08 — Source patch hot-plug

## Pipeline (ADR-0006)

```text
patch.mir → parse → typecheck → elaborate → compatibility → capability/
admission → HotPlugRequest → HotPlugVerdict → activation_cut → runtime
mutation → devtools trace
```

Rejected/deferred verdicts mutate nothing but lifecycle rows. No direct eval,
even in development. Default evolution mode is downstream addition; API
shadowing is forbidden; overlays must be compatibility-preserving
(theory/02 layer laws).

## Compatibility carrier

A patch declares: provided_surfaces, required_capabilities, effect_row,
failure_row, observation/redaction/retention policy, state_additions,
state_migrations, save_load_interaction, rollback_replay_cut_policy,
checked_membership_epoch, checked_member_incarnations, required
membership/capability witness refs. Generated communication introduced by a
patch obeys the same elaboration contract (theory/03).

## Frontier-bound activation

Admission validates against a frontier F = (membership epoch, incarnations,
witness refs). `activate(P, F)` fires only if the live frontier still matches
F; drift ⇒ reject or defer — never silent activation against a different
participant set. If the activation cut is included in a save cut, the request,
verdict, and F must be in the cut too (Consistent closure, theory/04).

## Negative axioms (each is a required reject)

A patch may not: write private state without declared capability; introduce
undeclared failures; alter an already-finalized atomic_cut prefix; grant
itself authority (self-grant, SCN-09 negative); import unresolved/incompatible
sources; silently weaken redaction/retention.

## THM-006 — Rejection no-mutation

```text
If HotPlugVerdict(P) ∈ {rejected, deferred}, the runtime configuration after
the verdict differs from before only in patch lifecycle rows: H gains only
patchreq/patchverdict occurrences; Q, S, M, G, W, L are unchanged.
```

(OBL-019.) Patch DAG: inter-patch dependencies form a DAG (an avatar patch
over a world system on loci {A, C}); activation order respects it.

OPEN-021: durable migration and distributed activation ordering — later gates
(PHASE-I6); v0 fixes only the single-session pipeline semantics.
