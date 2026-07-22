# 182 - Canon Core minimality and proof-interface audit

## Role and authority

This is LAB repository memory for a read-only audit. `mirrorea_canon/` remains
the sole normative source. This document selects no Core primitive, grammar,
carrier, relation, transition premise, theorem status, Gate, Phase, contract,
or public claim. It is not a working record and does not authorize an outcome
command or a new evidence lane.

## Question

Does the current Canon retain a small, non-domain-specific Core while keeping
the proof prerequisites for explicit communication, authority, ordering,
fallback, observation, and hot-plug visible enough to avoid proof laundering
or implicit semantics?

## Audit basis

The audit reads theory/00 through theory/11, the root/MAP hierarchy, and
ADR-0001 through ADR-0007, ADR-0010, and ADR-0014. A local configuration and
step-rule map was compared with an independent temporary Oracle review. The
review classifies a statement as a Canon contradiction, a proof-interface
prerequisite, an already tracked OPEN/L2 item, or no issue. It does not decide
among future representations.

## Result: no contradiction identified in the bounded audit

The audit did not identify a conflict among the reviewed cross-cutting claims
below. In particular:

- Surface remains ordinary while cross-locus effects become explicit Core rows
  through elaboration; no domain `World` / `Game` primitive is smuggled in.
- `O { ... }` is not ambient authority; non-owner bodies elaborate to
  owner-directed requests carrying caller evidence.
- role claims, keys, loci, transport, packages, providers, and signatures do
  not create authority; only validated grant lineage does.
- occurrence DAG, existence DAG, admission graph, and patch DAG remain
  distinct; fallback does not act as rollback and `atomic_cut` is not a fence.
- observation remains a typed information effect; patch activation remains
  admission/frontier bound and rejected/deferred patches do not mutate runtime
  configuration outside lifecycle rows.
- `theory/11` remains the only proof-status ledger. LAB Lean material and
  working records do not discharge a theorem or move a Gate.

## Proof-interface catalog

The following are requirements for a later common proof model, not reasons to
add Core vocabulary. Each is already anchored in existing Canon carriers.

| Area | Existing Canon anchor | Proof-side interface still needed | Non-claim |
| --- | --- | --- | --- |
| Elaboration determinism | unified judgment, OBL-021 | Equality/extensionality conditions for all judgment inputs and all outputs `(c, A, mu, epsilon, phi, C, O, G_e)`. | Does not choose a new elaborator, grammar, or outcome-totality rule. |
| Generated communication | BND-001 / THM-001 / OBL-004 | Correspondence between generated request/publish/observe/witness rows and runtime communication effects. | Does not add transport semantics or make a runtime artifact a Core fact. |
| Step preservation | OBL-020, plan/181 | Queue admissibility, safe occurrence insertion, owner dispatch, and successful patch activation coverage. | Does not enlarge Canon `WellFormed`. |
| Fallback monotonicity | THM-002 / OBL-005..008 | Relation tying chain instance, selected index, lineage, and reacquire epoch/witness boundary. | Does not add a fallback primitive or treat fallback as rollback. |
| Save/load | THM-003 / OBL-009..014 | Reconstruction relation from serialized provenance/state to the five global well-formedness clauses and explicit stale-rejection lemmas. | Does not select storage, replication, or durable distributed realization. |
| Observation | THM-005 / OBL-017/018 | Low-equivalence and exported-row equivalence for the declared observer-safe view. | Does not finalize the label lattice or create an untyped debug channel. |

`theory/09` stream non-influence/coherence and `theory/10` explanation
properties remain explicitly L2-working / open obligations. They are not
silently upgraded by this audit.

## Current source-locus disposition

No new L3 record is selected. The audit does not override the existing
source-locus and T1/T2 identity screens:

- OBL-001 still reaches the owner/canon direct-Core versus Result/write
  interface decision.
- OBL-020 already has family/global and proof-prerequisite LAB evidence; a
  common proof model must make transition coverage explicit.
- OBL-021 still needs the unselected outcome/adequacy boundary.
- For this audit's current LAB prioritization, the remaining catalog items have
  no identified immediate consumer or narrowly scoped non-reserved question.
  Authorization for any L3 record remains governed by ADR-0014's standing
  eligibility predicate.

This is a current LAB prioritization result, not an ADR-0014 restriction or a
claim that future L3 research is closed.

## Next integration point

When owner/canon proof-interface decisions make a common model admissible,
use this catalog together with plan/180 and plan/181 as a completeness check.
Do not manufacture progress by adding toy predicates, duplicate countermodels,
or a broad Core extension before an existing consumer requires one.

## Non-claims

- No theorem, OBL, or proof status changes in `theory/11`.
- No formal proof, Lean outcome, L3 working record, or source artifact is
  created by this audit.
- No Core/Config/Step/WellFormed, authority, effect, failure, transport,
  observation, patch, grammar, runtime, adapter, schema, or API change.
- No Gate/Phase, scenario, conformance, implementation, or public-completion
  claim changes.
