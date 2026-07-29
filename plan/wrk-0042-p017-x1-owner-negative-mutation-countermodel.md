# WRK-0042 - P017 X1 owner-negative/mutation countermodel evidence

This is **LAB** evidence for `working/WRK-0042` after its registration cut. It
is a finite negative oracle, not an owner-failure or owner-mutation model. The
single anchor and all four labels are supplied fixtures only: they do not define
a Mir request, identity, value, failure member, mutation rule, attribution
criterion, persistence key, saved object, protocol, or runtime carrier.

`neutral` supplies neither mark; `negativeOnly` and `mutationOnly` supply one
mark each; `both` supplies both. `NEGATIVE_MUTATION_OVERLAP` classifies only the
seeded `both` fixture. The marks stand only for the pre-registered
owner-terminal-negative and owner-mutation labels. They do not supply a failure
type, dynamic row, branch state, transition, receipt, storage mutation,
attribution fact, or reachability fact about a Mir execution.

The sole fenced source is extracted to a disposable file before compiling. Its
fixture-local constructors and predicates are not a stable module, schema,
validator input, public API, or runtime interface.

## Outcome Lean source

```lean
namespace P017X1NegativeMutationLab

inductive Fixture where
  | neutral
  | negativeOnly
  | mutationOnly
  | both

inductive Anchor where
  | q

def NegativeMark : Anchor -> Fixture -> Prop
  | .q, .neutral => False
  | .q, .negativeOnly => True
  | .q, .mutationOnly => False
  | .q, .both => True

def MutationMark : Anchor -> Fixture -> Prop
  | .q, .neutral => False
  | .q, .negativeOnly => False
  | .q, .mutationOnly => True
  | .q, .both => True

def NEGATIVE_MUTATION_OVERLAP : Anchor -> Fixture -> Prop
  | .q, .neutral => False
  | .q, .negativeOnly => False
  | .q, .mutationOnly => False
  | .q, .both => True

theorem neutral_has_no_marks :
    ¬ NegativeMark .q .neutral /\ ¬ MutationMark .q .neutral /\
      ¬ NEGATIVE_MUTATION_OVERLAP .q .neutral := by
  exact ⟨fun impossible => impossible, fun impossible => impossible,
    fun impossible => impossible⟩

theorem negative_only_has_one_mark :
    NegativeMark .q .negativeOnly /\ ¬ MutationMark .q .negativeOnly /\
      ¬ NEGATIVE_MUTATION_OVERLAP .q .negativeOnly := by
  exact ⟨True.intro, fun impossible => impossible,
    fun impossible => impossible⟩

theorem mutation_only_has_one_mark :
    ¬ NegativeMark .q .mutationOnly /\ MutationMark .q .mutationOnly /\
      ¬ NEGATIVE_MUTATION_OVERLAP .q .mutationOnly := by
  exact ⟨fun impossible => impossible, True.intro,
    fun impossible => impossible⟩

theorem both_detects_overlap :
    NegativeMark .q .both /\ MutationMark .q .both /\
      NEGATIVE_MUTATION_OVERLAP .q .both := by
  exact ⟨True.intro, True.intro, True.intro⟩

end P017X1NegativeMutationLab
```

## Bound of the result

If this source compiles, it establishes only that this supplied four-fixture
detector table separates the three neutral/singleton labels from a seeded
simultaneous pair. It does not establish an owner failure, owner mutation,
attribution, terminal branch relation, state reachability, delivery, fairness,
termination, typed failure behavior, a receipt rule, consumption representation,
saved-state behavior, authority enforcement, observation policy, a theorem/OBL,
or implementation readiness.
