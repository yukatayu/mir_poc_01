# WRK-0043 P017 X1 M1 adverse/mutation countermodel

## Evidence role

This is the single Markdown-held Lean source declared by WRK-0043. It is LAB
countermodel evidence only. Its anchor, adverse tags, fixtures, and mutation
mark are supplied finite test vocabulary; none denotes a Mir request, actual
validation result, rejection, failure, owner, store update, identity, carrier,
transition, persistence field, source construct, runtime event, or public
interface.

## Registered finite matrix

| Fixture | Supplied adverse tag | Supplied mutation mark | Detector |
| --- | --- | --- | --- |
| neutral | no | no | false |
| adverse-only(tag) | yes | no | false |
| mutation-only | no | yes | false |
| overlap(tag) | yes | yes | true |

AdverseTag copies the finite P013/P017 condition names only. It is
non-exhaustive test vocabulary. The source does not determine whether a request
has any tag, run validation, or map a tag to a terminal outcome.

## Lean source

```lean
namespace P017X1M1AdverseMutationLab

inductive Anchor where
  | q

inductive AdverseTag where
  | copiedOrReplayed
  | staleEpochOrIncarnation
  | wrongPrincipalRoleOrTarget
  | missingCapabilityOrWitness
  | grantPolicyMismatch
  | severedProvenance
  | visibilityDenied
  | sameLocusActivePrincipals

inductive Fixture where
  | neutral
  | adverseOnly (tag : AdverseTag)
  | mutationOnly
  | overlap (tag : AdverseTag)

inductive AdverseMark : Anchor -> Fixture -> AdverseTag -> Prop where
  | adverseOnly (tag : AdverseTag) : AdverseMark .q (.adverseOnly tag) tag
  | overlap (tag : AdverseTag) : AdverseMark .q (.overlap tag) tag

inductive MutationMark : Anchor -> Fixture -> Prop where
  | mutationOnly : MutationMark .q .mutationOnly
  | overlap (tag : AdverseTag) : MutationMark .q (.overlap tag)

def M1_ADVERSE_MUTATION_OVERLAP (anchor : Anchor) (fixture : Fixture) : Prop :=
  Exists fun tag => AdverseMark anchor fixture tag ∧ MutationMark anchor fixture

theorem neutral_is_clear :
    ¬ M1_ADVERSE_MUTATION_OVERLAP .q .neutral := by
  intro detected
  rcases detected with ⟨tag, adverse, mutation⟩
  cases adverse

theorem adverse_only_is_clear (tag : AdverseTag) :
    ¬ M1_ADVERSE_MUTATION_OVERLAP .q (.adverseOnly tag) := by
  intro detected
  rcases detected with ⟨found, adverse, mutation⟩
  cases mutation

theorem mutation_only_is_clear :
    ¬ M1_ADVERSE_MUTATION_OVERLAP .q .mutationOnly := by
  intro detected
  rcases detected with ⟨tag, adverse, mutation⟩
  cases adverse

theorem overlap_is_detected (tag : AdverseTag) :
    M1_ADVERSE_MUTATION_OVERLAP .q (.overlap tag) := by
  exact ⟨tag, .overlap tag, .overlap tag⟩

end P017X1M1AdverseMutationLab
```

## Non-claims

A pass distinguishes the four supplied fixture forms only. It neither proves
fail-closed behavior nor supplies validation/rejection semantics, a typed
failure, a mutation rule or attribution, a relation model, a Core/Config rule,
an OBL/THM result, scenario conformance, implementation readiness, or public
behavior.
