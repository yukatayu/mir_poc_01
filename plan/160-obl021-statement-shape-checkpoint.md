# plan/160 - OBL-021 statement-shape checkpoint

## Role and authority

This file is LAB repository memory. `mirrorea_canon/` remains the normative
source for theory, obligation status, Gates, phases, and process. This
checkpoint neither changes BND-001 nor proves, completes, promotes, or assigns
OBL-021.

The checkpoint closes the current bounded question about the **shape** of the
LAB OBL-021 statement. It is not a decision that the current draft is the final
determinism theorem, nor a request to move the theory ledger.

## Evidence boundary

The following records are all L3-open and `not-promoted`. They are separate
experiments over the LAB statement shape; their countermodels are not claimed
to describe one common model or one common input.

| Record | Retained evidence | What it establishes | What it does not establish |
| --- | --- | --- | --- |
| WRK-0002 | projection-vacuity countermodel | Empty result projections permit two distinct successful `Result` values under the draft. | That final Result equality is the correct target, or that Canon lacks a function-like contract. |
| WRK-0003 | total/unique projection plus component-equality countermodel | Per-projection totality/uniqueness and native component equality still do not force Result identity. | Which joint extensionality, adequacy bridge, or direct Result relation should be selected. |
| WRK-0004 | no-outcome countermodel | The draft alone permits a well-scoped input with neither success nor rejection. | The Canon location or exact formulation of an outcome-totality law. |
| WRK-0005 | conditional tagged-outcome relation | The draft and well-scopedness relate every pair in one fixed input's actual-outcome fiber. `OutcomeTotal` makes that fiber nonempty. | Native equality, global setoid/equivalence laws, quotient semantics, observational adequacy, a Diagnostic bridge, or Canon placement. |

The governing artifacts and append-only evidence commit identities are in the
four `mirrorea_canon/working/WRK-0002...0005` records. Reports 2299--2313 are
the operational evidence trail, not semantic authority.

## Current LAB reading

For a fixed well-scoped input, let the actual-outcome fiber contain tagged
values that satisfy `OutcomeOf`. The current draft gives:

```text
actual success x actual success -> SameElabResult x y
actual reject  x actual reject  -> SameDiagnostic x y
actual success x actual reject  -> impossible
```

Consequently, `SameOutcome` is all-pairs related on that fiber. The fiber can
be empty under the draft; experiment-local `OutcomeTotal` adds only a witness
of nonemptiness. On the restricted fiber, the all-pairs fact immediately gives
reflexivity, symmetry, and transitivity. It does not give any such global law
on every tagged `Outcome` value, nor identify `SameOutcome` with equality.

The concise decomposition is:

```text
total deterministic outcome reading
  = outcome existence
  + pairwise coherence on actual outcomes
  + adequacy to a deliberately selected equality/equivalence observation
```

Only the middle term is supplied by the LAB draft. The existence term is a
separate premise in WRK-0005 and absent in WRK-0004's model. The adequacy term
remains open after WRK-0002 and WRK-0003.

## Diagnostic and input limits

`SameDiagnostic` delegates solely to `EquivalentDiagnostic`. The checked LAB
source provides no bridge from that predicate to the canonical Diagnostic
fields, explanation soundness, or explanation completeness in theory/10.

Likewise, “fixed input” means the same Lean arguments `env`, `ctx`, `locus`,
and `item`. This checkpoint says nothing about canonicalized or extensionally
equivalent input snapshots.

## Decision surface

The updated `plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md`
is the future human/canon-facing decision surface. It now treats present fields
as **abstract comparison predicates**, not already-established equivalences,
and records that projection totality alone is insufficient without joint
adequacy/extensionality or a direct Result relation.

Research has resolved enough to state the following without further owner input:

- a fifth relation-law packaging theorem would only restate WRK-0005;
- a no-outcome countermodel and a result-identity countermodel are distinct
  gaps and must remain distinct in later reasoning;
- any later candidate must have an alternative outcome that changes a live
  decision branch.

Owner/canon action remains required for outcome-totality placement, Result
identity versus observational comparison/quotient, joint adequacy versus a
direct Result relation, Diagnostic comparison semantics, fixed-input identity,
artifact/wrapper identity, requested status, proof discharge, ledger movement,
and every Gate/Phase or public claim.

## Checkpoint acceptance

- [x] Four L3 results are separately manifested and retain their exact source
  evidence commits.
- [x] The positive conditional result is named and explained without `ExistsUnique`,
  global-law, or arbitrary-tagged-value overclaim.
- [x] Outcome existence, pairwise coherence, and Result adequacy are separated.
- [x] The future decision packet treats current predicates as comparisons and
  records the missing joint bridge.
- [x] No fifth theorem is scheduled merely to complete a local relation
  hierarchy.
- [x] No Canon theory, OBL status, theory ledger, Gate/Phase, conformance,
  implementation, or public state changed.

## Next research selection rule

No new WRK candidate is active at this checkpoint. A later standing-eligible
L3 candidate should be pre-registered only if its falsifier distinguishes a
choice that is otherwise live. Examples may compare a genuinely smaller joint
adequacy bridge or a diagnostic comparison boundary, but must not silently
select a canonical equality, totality placement, or diagnostic ABI.

Until then, retain this checkpoint and use `plan/143` for any owner/canon-facing
status or refinement discussion.
