# Plan 224: P017 X1 M1 Adverse/Mutation Candidate Selection

## Role and authority

This is LAB candidate-selection memory for the post-WRK-0042 P017 X1 screen.
mirrorea_canon/ remains normative. The selected record is a reversible L3
pre-registration only; it neither supplies M1 validation semantics nor selects a
failure representation, mutation rule, attribution criterion, Core, Config,
SaveObject, terminal branch algebra, transition, theory/11 entry, scenario,
Gate, Phase, runtime, contract, or public behavior.

## Current cut and question

WRK-0040 retained five supplied-fixture collapse detectors. WRK-0041 retained
an owner-terminal-positive / owner-terminal-negative overlap detector. WRK-0042
retained an owner-terminal-negative / owner-mutation overlap detector. Plan 223
forbids extending that Boolean table mechanically: a successor needs a new
explicit source condition, an independent consumer, and a typed falsifier.

P013 requires copied/replayed, stale, wrong-target, missing-witness, and
severed-lineage requests to be rejected without store mutation. P017's M1 and
authority package row additionally names stale epoch/incarnation, wrong
principal/role/target, missing capability/witness, grant-policy mismatch,
severed provenance, visibility denial, and two active principals at one source
locus; each must fail closed with no owner mutation. Plan 220 retains this
separately as X-M1.

The question is not how an actual request becomes copied, stale, wrong-target,
or otherwise adverse. It is whether one source-named adverse-condition tag can
remain an opaque supplied fixture fact and still form a distinct negative oracle
with an equally supplied owner-mutation mark.

## Candidate screen

| Candidate | Independent consumer and falsifier | Disposition |
| --- | --- | --- |
| finite M1 adverse-condition tag / owner-mutation overlap | P013/P017's X-M1 consumer asks whether a supplied source-named adverse input is paired with mutation; neutral, adverse-only, mutation-only, and seeded overlap fixtures expose the pair | **selected**: one uniform AdverseTag family in the existing plan/ Lean lane |
| immediate scoped no-candidate | avoids a further table but leaves the distinct P013/P017 no-mutation consequence without its own input-condition negative oracle | not selected: the source condition, consumer, and typed falsifier are explicit and independent |
| finite owner-terminal-negative / owner-mutation overlap | starts from a terminal outcome label rather than an M1 input condition | duplicate: WRK-0042 already retains this different detector |
| generic validation failure, rejection, fail-closed, or owner-terminal-negative marker | either restates WRK-0042 or gives the label an unselected operational meaning | reserved or duplicate: selecting that meaning decides validation/failure semantics |
| one detector per stale, replay, wrong-target, or missing-witness name | preserves the same input condition, consumer, and falsifier shape | duplicate: one finite tag family is sufficient; do not multiply Boolean tables |
| authority recovery, claim-as-authority, success, mutation attribution, pending, receipt, typing, causality, linearity, load, or observation | needs a validation algorithm, authority/failure/mutation/transition representation, carrier, or projection | reserved or duplicate: these remain ordinary Canon design work |

The temporary Oracle review is advisory only. Its selection agrees with the
literal source distinction: the current Canon cut does not select a rule from a
supplied M1 adverse input tag to an owner-terminal-negative outcome. Treating
them as the same would silently select validation-to-failure semantics.

## Selected package

working/WRK-0043-p017-x1-m1-adverse-mutation-countermodel.md was committed
and pushed before any outcome source existed. Its later source may use exactly
one opaque fixture anchor, a finite AdverseTag copied only from the P013/P017
adverse-condition names, and an opaque owner-mutation mark. The source must
keep the tag vocabulary non-exhaustive and non-operational: it must not compare
epochs, query authority, inspect witnesses, infer history, or run validation.

The registered four controls are neutral, adverse-only for a supplied tag,
mutation-only, and their seeded overlap. The detector is the existential
overlap of one supplied tag and the supplied mutation mark. It must not contain
positive/negative terminal marks, validation-accepted/rejected, fail-closed, a
failure row, a store delta, a mutation function, a transition, or a causal edge.

## Execution order and stop rule

1. WRK-0043 is registered and pushed before outcome evidence.
2. Its one Markdown-held Lean block may be materialized only in the declared
   plan/ lane, then checked under lean --trust=0 with its matrix, axiom,
   vocabulary, allowlist, and diff checks.
3. A detector pass can show only the supplied labels distinguish the controls
   from a seeded overlap. It cannot establish a Mir execution, a validation
   result, rejection, fail-closed behavior, owner failure, mutation behavior,
   mutation attribution, or P017 implementation satisfaction.
4. After this package, stop the current fixture-only line. Do not create a
   record per tag, per conjunction permutation, or for a new control layout.
   Reopen only for a later Canon cut with a genuinely new explicit source
   condition, independent consumer, and typed falsifier.

Freeze rather than repair the source if it needs an actual adverse-condition
classifier, validation result, failure mapping, owner-terminal-negative fact,
mutation rule/attribution, identity, carrier, transition, persistence,
source syntax, runtime, transport, contract, OBL/THM, SCN, Gate, Phase, helper,
schema, CI/Make surface, or public interface.

## Non-effects

This plan does not turn the P013/P017 no-mutation requirements into a theorem,
OBL, proof of fail-closed behavior, implementation criterion, or selected
design. It selects one bounded LAB experiment only.
