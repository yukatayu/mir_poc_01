# WRK-0046 P017 X1 K0 q-fiber U/L lifting

## Evidence role

This is the sole Markdown-held Lean source declared by `working/WRK-0046`.
It is a candidate-local conditional examination of the pre-registered K0
external-rejection U/L ledger. It does not define or choose a Mir relation
schema, identity, key, transition, persistence representation, restore
function, receipt, authority rule, runtime, transport, or public interface.

The generic theorem uses opaque state and q carriers, opaque `AcceptedSuccess`
and `Spent` predicates, and an arbitrary four-place relation. A line has one
*designated* restore bridge; no functionality or disjointness from ordinary
edges is assumed for the relation. The fixture later supplies a universal
Prop-valued relation only to show that the registered premise bundles are
inhabited. It does not make a claim about an actual restore relation.

The source proves only a finite, supplied-line conditional lemma. It neither
proves P017 X1 for every admissible restored continuation nor selects the
residence, representation, or final classification of `Spent`.

## Lean source

```lean
set_option autoImplicit false

universe u v w

namespace WRK0046

structure Endpoint (State : Type u) (Q : Type v) where
  state : State
  q : Q

def HoldsAt {State : Type u} {Q : Type v}
    (P : State -> Q -> Prop) (e : Endpoint State Q) : Prop :=
  P e.state e.q

abbrev FourEndpointRestore (State : Type u) (Q : Type v) :=
  State -> Q -> State -> Q -> Prop

/-- A finite sequence whose internal edges are ordinary experimental edges. -/
inductive OrdinarySegment (E : Type w) where
  | terminal (e : E)
  | edge (src : E) (counted : Bool) (rest : OrdinarySegment E)

namespace OrdinarySegment

def start {E : Type w} : OrdinarySegment E -> E
  | .terminal e => e
  | .edge src _ _ => src

def finish {E : Type w} : OrdinarySegment E -> E
  | .terminal e => e
  | .edge _ _ rest => finish rest

/--
The first conjunct is local ordinary-edge `Spent` preservation. The second
conjunct gives the guard and effect of a counted consume edge. No uniqueness
or global at-most-one premise occurs here.
-/
def Valid {State : Type u} {Q : Type v}
    (Accepted Spent : State -> Q -> Prop) :
    OrdinarySegment (Endpoint State Q) -> Prop
  | .terminal _ => True
  | .edge src counted rest =>
      (HoldsAt Spent src -> HoldsAt Spent (start rest)) /\
      (counted = true ->
        HoldsAt Accepted src /\
        Not (HoldsAt Spent src) /\
        HoldsAt Spent (start rest)) /\
      Valid Accepted Spent rest

def HasConsume {E : Type w} : OrdinarySegment E -> Prop
  | .terminal _ => False
  | .edge _ counted rest =>
      counted = true \/ HasConsume rest

/-- A structural witness for two distinct counted edges in one segment. -/
def HasTwoConsumes {E : Type w} : OrdinarySegment E -> Prop
  | .terminal _ => False
  | .edge _ counted rest =>
      (counted = true /\ HasConsume rest) \/ HasTwoConsumes rest

theorem spent_finish
    {State : Type u} {Q : Type v}
    (Accepted Spent : State -> Q -> Prop) :
    forall seg : OrdinarySegment (Endpoint State Q),
      Valid Accepted Spent seg ->
      HoldsAt Spent (start seg) ->
      HoldsAt Spent (finish seg)
  | .terminal _, _, hStart =>
      hStart
  | .edge _ _ rest, hValid, hStart =>
      spent_finish Accepted Spent rest hValid.2.2 (hValid.1 hStart)

theorem no_consume_of_spent_start
    {State : Type u} {Q : Type v}
    (Accepted Spent : State -> Q -> Prop) :
    forall seg : OrdinarySegment (Endpoint State Q),
      Valid Accepted Spent seg ->
      HoldsAt Spent (start seg) ->
      Not (HasConsume seg)
  | .terminal _, _, _, hHas =>
      False.elim hHas
  | .edge _ _ rest, hValid, hStart, hHas => by
      cases hHas with
      | inl hHere =>
          exact (hValid.2.1 hHere).2.1 hStart
      | inr hLater =>
          exact no_consume_of_spent_start Accepted Spent rest
            hValid.2.2 (hValid.1 hStart) hLater

theorem not_two_consumes
    {State : Type u} {Q : Type v}
    (Accepted Spent : State -> Q -> Prop) :
    forall seg : OrdinarySegment (Endpoint State Q),
      Valid Accepted Spent seg ->
      Not (HasTwoConsumes seg)
  | .terminal _, _, hTwo =>
      False.elim hTwo
  | .edge _ _ rest, hValid, hTwo => by
      cases hTwo with
      | inl hHereAndLater =>
          exact no_consume_of_spent_start Accepted Spent rest
            hValid.2.2
            (hValid.2.1 hHereAndLater.1).2.2
            hHereAndLater.2
      | inr hTailTwo =>
          exact not_two_consumes Accepted Spent rest hValid.2.2 hTailTwo

theorem spent_finish_of_has_consume
    {State : Type u} {Q : Type v}
    (Accepted Spent : State -> Q -> Prop) :
    forall seg : OrdinarySegment (Endpoint State Q),
      Valid Accepted Spent seg ->
      HasConsume seg ->
      HoldsAt Spent (finish seg)
  | .terminal _, _, hHas =>
      False.elim hHas
  | .edge _ _ rest, hValid, hHas => by
      cases hHas with
      | inl hHere =>
          exact spent_finish Accepted Spent rest
            hValid.2.2 (hValid.2.1 hHere).2.2
      | inr hLater =>
          exact spent_finish_of_has_consume Accepted Spent rest hValid.2.2 hLater

end OrdinarySegment

/--
All internal edges of `before` and `after` are ordinary. The one designated
restore position is the bridge from `finish before` to `start after`.
-/
structure OneRestoreLine (State : Type u) (Q : Type v) where
  before : OrdinarySegment (Endpoint State Q)
  after : OrdinarySegment (Endpoint State Q)

namespace OneRestoreLine

def restorePre {State : Type u} {Q : Type v}
    (line : OneRestoreLine State Q) : Endpoint State Q :=
  OrdinarySegment.finish line.before

def restorePost {State : Type u} {Q : Type v}
    (line : OneRestoreLine State Q) : Endpoint State Q :=
  OrdinarySegment.start line.after

def HasTwoConsumes {State : Type u} {Q : Type v}
    (line : OneRestoreLine State Q) : Prop :=
  OrdinarySegment.HasTwoConsumes line.before \/
  (OrdinarySegment.HasConsume line.before /\
    OrdinarySegment.HasConsume line.after) \/
  OrdinarySegment.HasTwoConsumes line.after

end OneRestoreLine

def RestoreAt {State : Type u} {Q : Type v}
    (Restore : FourEndpointRestore State Q)
    (pre post : Endpoint State Q) : Prop :=
  Restore pre.state pre.q post.state post.q

/-- This preservation obligation is local to one supplied endpoint pair. -/
def PreservesSpentAt {State : Type u} {Q : Type v}
    (Restore : FourEndpointRestore State Q)
    (Spent : State -> Q -> Prop)
    (pre post : Endpoint State Q) : Prop :=
  RestoreAt Restore pre post ->
  HoldsAt Spent pre ->
  HoldsAt Spent post

/-- A0: two counted consume edges on this supplied finite line are impossible. -/
theorem a0_no_two_consumes
    {State : Type u} {Q : Type v}
    (line : OneRestoreLine State Q)
    (Accepted Spent : State -> Q -> Prop)
    (Restore : FourEndpointRestore State Q)
    (hBefore : OrdinarySegment.Valid Accepted Spent line.before)
    (hAfter : OrdinarySegment.Valid Accepted Spent line.after)
    (hRestore : RestoreAt Restore
      (OneRestoreLine.restorePre line)
      (OneRestoreLine.restorePost line))
    (hPreserve : PreservesSpentAt Restore Spent
      (OneRestoreLine.restorePre line)
      (OneRestoreLine.restorePost line)) :
    Not (OneRestoreLine.HasTwoConsumes line) := by
  intro hTwo
  cases hTwo with
  | inl hBeforeTwo =>
      exact OrdinarySegment.not_two_consumes
        Accepted Spent line.before hBefore hBeforeTwo
  | inr hCrossOrAfter =>
      cases hCrossOrAfter with
      | inl hCross =>
          have hPreSpent := OrdinarySegment.spent_finish_of_has_consume
            Accepted Spent line.before hBefore hCross.1
          have hPostSpent := hPreserve hRestore hPreSpent
          exact OrdinarySegment.no_consume_of_spent_start
            Accepted Spent line.after hAfter hPostSpent hCross.2
      | inr hAfterTwo =>
          exact OrdinarySegment.not_two_consumes
            Accepted Spent line.after hAfter hAfterTwo

/--
A genuine A0 falsifier must supply all A0 premises, including restore
preservation, and still supply two consumes. The extra witnesses retain the
registered nonvacuity obligations even though the kernel theorem is stronger.
-/
theorem no_true_a0_falsifier
    {State : Type u} {Q : Type v}
    (line : OneRestoreLine State Q)
    (Accepted Spent : State -> Q -> Prop)
    (Restore : FourEndpointRestore State Q)
    (hBefore : OrdinarySegment.Valid Accepted Spent line.before)
    (hAfter : OrdinarySegment.Valid Accepted Spent line.after)
    (hRestore : RestoreAt Restore
      (OneRestoreLine.restorePre line)
      (OneRestoreLine.restorePost line))
    (hPreserve : PreservesSpentAt Restore Spent
      (OneRestoreLine.restorePre line)
      (OneRestoreLine.restorePost line))
    (_hPreConsume : OrdinarySegment.HasConsume line.before)
    (_hPostAccepted : HoldsAt Accepted (OneRestoreLine.restorePost line))
    (hTwo : OneRestoreLine.HasTwoConsumes line) : False :=
  a0_no_two_consumes line Accepted Spent Restore
    hBefore hAfter hRestore hPreserve hTwo

namespace Fixture

inductive FixtureState where
  | s0 | s1 | s2 | s3

inductive FixtureQ where
  | qPre | qPost

def e0 : Endpoint FixtureState FixtureQ := ⟨.s0, .qPre⟩
def e1 : Endpoint FixtureState FixtureQ := ⟨.s1, .qPre⟩
def e2 : Endpoint FixtureState FixtureQ := ⟨.s2, .qPost⟩
def e3 : Endpoint FixtureState FixtureQ := ⟨.s3, .qPost⟩

/-- A universal test relation with no functionality premise or theorem. -/
def FixtureRestore : FourEndpointRestore FixtureState FixtureQ :=
  fun _ _ _ _ => True

inductive FixtureAccepted : FixtureState -> FixtureQ -> Prop where
  | at_e0 : FixtureAccepted .s0 .qPre
  | at_e2 : FixtureAccepted .s2 .qPost

inductive FixtureSpentA0 : FixtureState -> FixtureQ -> Prop where
  | at_e1 : FixtureSpentA0 .s1 .qPre
  | at_e2 : FixtureSpentA0 .s2 .qPost
  | at_e3 : FixtureSpentA0 .s3 .qPost

inductive FixtureSpentA1 : FixtureState -> FixtureQ -> Prop where
  | at_e1 : FixtureSpentA1 .s1 .qPre
  | at_e3 : FixtureSpentA1 .s3 .qPost

def before : OrdinarySegment (Endpoint FixtureState FixtureQ) :=
  .edge e0 true (.terminal e1)

def afterA0 : OrdinarySegment (Endpoint FixtureState FixtureQ) :=
  .edge e2 false (.terminal e3)

def afterA1 : OrdinarySegment (Endpoint FixtureState FixtureQ) :=
  .edge e2 true (.terminal e3)

def lineA0 : OneRestoreLine FixtureState FixtureQ :=
  ⟨before, afterA0⟩

def lineA1 : OneRestoreLine FixtureState FixtureQ :=
  ⟨before, afterA1⟩

theorem fixture_restore : RestoreAt FixtureRestore e1 e2 :=
  True.intro

theorem before_valid_a0 :
    OrdinarySegment.Valid FixtureAccepted FixtureSpentA0 before := by
  constructor
  · intro h
    cases h
  constructor
  · intro _
    exact ⟨FixtureAccepted.at_e0, (by intro h; cases h), FixtureSpentA0.at_e1⟩
  · exact True.intro

theorem after_valid_a0 :
    OrdinarySegment.Valid FixtureAccepted FixtureSpentA0 afterA0 := by
  constructor
  · intro _
    exact FixtureSpentA0.at_e3
  constructor
  · intro h
    cases h
  · exact True.intro

theorem before_valid_a1 :
    OrdinarySegment.Valid FixtureAccepted FixtureSpentA1 before := by
  constructor
  · intro h
    cases h
  constructor
  · intro _
    exact ⟨FixtureAccepted.at_e0, (by intro h; cases h), FixtureSpentA1.at_e1⟩
  · exact True.intro

theorem after_valid_a1 :
    OrdinarySegment.Valid FixtureAccepted FixtureSpentA1 afterA1 := by
  constructor
  · intro h
    cases h
  constructor
  · intro _
    exact ⟨FixtureAccepted.at_e2, (by intro h; cases h), FixtureSpentA1.at_e3⟩
  · exact True.intro

theorem before_has_consume : OrdinarySegment.HasConsume before :=
  Or.inl rfl

theorem after_a1_has_consume : OrdinarySegment.HasConsume afterA1 :=
  Or.inl rfl

theorem a0_pre_spent : HoldsAt FixtureSpentA0 e1 :=
  FixtureSpentA0.at_e1

theorem a0_post_spent : HoldsAt FixtureSpentA0 e2 :=
  FixtureSpentA0.at_e2

theorem post_accepted : HoldsAt FixtureAccepted e2 :=
  FixtureAccepted.at_e2

theorem restore_preserves_a0 :
    PreservesSpentAt FixtureRestore FixtureSpentA0 e1 e2 := by
  intro _ _
  exact FixtureSpentA0.at_e2

/-- A closed A0 premise-satisfiability/nonvacuity fixture. -/
theorem a0_nonvacuity :
    OrdinarySegment.Valid FixtureAccepted FixtureSpentA0 before /\
    OrdinarySegment.Valid FixtureAccepted FixtureSpentA0 afterA0 /\
    OrdinarySegment.HasConsume before /\
    RestoreAt FixtureRestore e1 e2 /\
    HoldsAt FixtureAccepted e2 /\
    HoldsAt FixtureSpentA0 e1 /\
    HoldsAt FixtureSpentA0 e2 /\
    PreservesSpentAt FixtureRestore FixtureSpentA0 e1 e2 :=
  ⟨before_valid_a0, after_valid_a0, before_has_consume, fixture_restore,
    post_accepted, a0_pre_spent, a0_post_spent, restore_preserves_a0⟩

theorem a0_fixture_no_two :
    Not (OneRestoreLine.HasTwoConsumes lineA0) :=
  a0_no_two_consumes lineA0 FixtureAccepted FixtureSpentA0 FixtureRestore
    before_valid_a0 after_valid_a0 fixture_restore restore_preserves_a0

theorem a1_post_unspent : Not (HoldsAt FixtureSpentA1 e2) := by
  intro h
  cases h

theorem a1_two_consumes : OneRestoreLine.HasTwoConsumes lineA1 :=
  Or.inr (Or.inl ⟨before_has_consume, after_a1_has_consume⟩)

/--
The A1 control removes restore preservation only. Its exact adverse trace is
closed, while non-preservation is not an additional A1 premise.
-/
theorem a1_adverse_control :
    OrdinarySegment.Valid FixtureAccepted FixtureSpentA1 before /\
    OrdinarySegment.Valid FixtureAccepted FixtureSpentA1 afterA1 /\
    RestoreAt FixtureRestore e1 e2 /\
    OrdinarySegment.HasConsume before /\
    OrdinarySegment.HasConsume afterA1 /\
    HoldsAt FixtureAccepted e2 /\
    Not (HoldsAt FixtureSpentA1 e2) /\
    OneRestoreLine.HasTwoConsumes lineA1 :=
  ⟨before_valid_a1, after_valid_a1, fixture_restore, before_has_consume,
    after_a1_has_consume, post_accepted, a1_post_unspent, a1_two_consumes⟩

/-- A derived property of the A1 fixture, not an A1 premise. -/
theorem a1_not_restore_preservation :
    Not (PreservesSpentAt FixtureRestore FixtureSpentA1 e1 e2) := by
  intro hPreserve
  exact a1_post_unspent (hPreserve fixture_restore FixtureSpentA1.at_e1)

#print axioms Endpoint
#print axioms HoldsAt
#print axioms FourEndpointRestore
#print axioms OrdinarySegment
#print axioms OrdinarySegment.start
#print axioms OrdinarySegment.finish
#print axioms OrdinarySegment.Valid
#print axioms OrdinarySegment.HasConsume
#print axioms OrdinarySegment.HasTwoConsumes
#print axioms OrdinarySegment.spent_finish
#print axioms OrdinarySegment.no_consume_of_spent_start
#print axioms OrdinarySegment.not_two_consumes
#print axioms OrdinarySegment.spent_finish_of_has_consume
#print axioms OneRestoreLine
#print axioms OneRestoreLine.restorePre
#print axioms OneRestoreLine.restorePost
#print axioms OneRestoreLine.HasTwoConsumes
#print axioms RestoreAt
#print axioms PreservesSpentAt
#print axioms a0_no_two_consumes
#print axioms no_true_a0_falsifier
#print axioms Fixture.FixtureState
#print axioms Fixture.FixtureQ
#print axioms Fixture.e0
#print axioms Fixture.e1
#print axioms Fixture.e2
#print axioms Fixture.e3
#print axioms Fixture.FixtureRestore
#print axioms Fixture.FixtureAccepted
#print axioms Fixture.FixtureSpentA0
#print axioms Fixture.FixtureSpentA1
#print axioms Fixture.before
#print axioms Fixture.afterA0
#print axioms Fixture.afterA1
#print axioms Fixture.lineA0
#print axioms Fixture.lineA1
#print axioms Fixture.fixture_restore
#print axioms Fixture.before_valid_a0
#print axioms Fixture.after_valid_a0
#print axioms Fixture.before_valid_a1
#print axioms Fixture.after_valid_a1
#print axioms Fixture.before_has_consume
#print axioms Fixture.after_a1_has_consume
#print axioms Fixture.a0_pre_spent
#print axioms Fixture.a0_post_spent
#print axioms Fixture.post_accepted
#print axioms Fixture.restore_preserves_a0
#print axioms Fixture.a0_nonvacuity
#print axioms Fixture.a0_fixture_no_two
#print axioms Fixture.a1_post_unspent
#print axioms Fixture.a1_two_consumes
#print axioms Fixture.a1_adverse_control
#print axioms Fixture.a1_not_restore_preservation

end Fixture
end WRK0046
```
