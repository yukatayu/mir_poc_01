# WRK-0041 - P017 X1 owner-terminal exclusivity countermodel evidence

This is **LAB** evidence for `working/WRK-0041` after its registration cut. It
is a finite negative oracle, not a terminal-branch model. The single anchor
and all four labels are supplied fixtures only: they do not define a Mir
request, identity, value, failure member, persistence key, saved object,
protocol, or runtime carrier.

`clear` supplies neither mark; `positiveOnly` and `negativeOnly` supply one
mark each; `both` supplies both. `OVERLAP` classifies only the seeded `both`
fixture. The marks stand only for the pre-registered owner-terminal-positive
and owner-terminal-negative labels. They do not supply a result type, dynamic
failure row, outcome constructor, state, transition, receipt, mutation, or
reachability fact about a Mir execution.

The sole fenced source is extracted to a disposable file before compiling. Its
fixture-local constructors and predicates are not a stable module, schema,
validator input, public API, or runtime interface.

## Outcome Lean source

```lean
namespace P017X1TerminalExclusiveLab

inductive Fixture where
  | clear
  | positiveOnly
  | negativeOnly
  | both

inductive Anchor where
  | q

def PositiveMark : Anchor -> Fixture -> Prop
  | .q, .clear => False
  | .q, .positiveOnly => True
  | .q, .negativeOnly => False
  | .q, .both => True

def NegativeMark : Anchor -> Fixture -> Prop
  | .q, .clear => False
  | .q, .positiveOnly => False
  | .q, .negativeOnly => True
  | .q, .both => True

def OVERLAP : Anchor -> Fixture -> Prop
  | .q, .clear => False
  | .q, .positiveOnly => False
  | .q, .negativeOnly => False
  | .q, .both => True

theorem clear_has_no_marks :
    ¬ PositiveMark .q .clear /\ ¬ NegativeMark .q .clear /\
      ¬ OVERLAP .q .clear := by
  exact ⟨fun impossible => impossible, fun impossible => impossible,
    fun impossible => impossible⟩

theorem positive_only_has_one_mark :
    PositiveMark .q .positiveOnly /\ ¬ NegativeMark .q .positiveOnly /\
      ¬ OVERLAP .q .positiveOnly := by
  exact ⟨True.intro, fun impossible => impossible,
    fun impossible => impossible⟩

theorem negative_only_has_one_mark :
    ¬ PositiveMark .q .negativeOnly /\ NegativeMark .q .negativeOnly /\
      ¬ OVERLAP .q .negativeOnly := by
  exact ⟨fun impossible => impossible, True.intro,
    fun impossible => impossible⟩

theorem both_detects_overlap :
    PositiveMark .q .both /\ NegativeMark .q .both /\ OVERLAP .q .both := by
  exact ⟨True.intro, True.intro, True.intro⟩

end P017X1TerminalExclusiveLab
```

## Bound of the result

If this source compiles, it establishes only that this supplied four-fixture
detector table separates the three all-clear/singleton labels from a seeded
simultaneous pair. It does not establish a positive terminal-branch relation,
state reachability, delivery, fairness, termination, typed failure behavior,
owner mutation, a receipt rule, consumption representation, saved-state
behavior, authority enforcement, observation policy, a theorem/OBL, or
implementation readiness.
