import MirroreaProofFirstDynamicIdentity

namespace MirroreaProofFirst.CurrentChoice
open CurrentUse

-- Internal current-choice dependency, NOT the static fallback-lineage checker.
-- Each candidate retains its actual full request. Authority is searched and
-- checked under the SAME current World; none is issued by this computation.
def offer (s : World n) (u : UseRequest n) : Option Evidence :=
  match authorize s.authority (s.policies u.operation.key) (currentContext s u) with
  | none => none
  | some e => if checkUse s u e then some e else none

theorem offer_sound (s : World n) (u : UseRequest n)
    (h : offer s u = some e) : CurrentUse s u ∧ checkUse s u e = true := by
  unfold offer at h
  split at h
  · cases h
  · split at h
    · cases h; exact ⟨checkUse_sound _ _ _ ‹_›,‹_›⟩
    · cases h

theorem offer_complete (s : World n) (u : UseRequest n) (h : CurrentUse s u) :
    ∃ e, offer s u = some e := by
  obtain ⟨e,ha,hc⟩ := authorize_use_complete s u h
  exact ⟨e,by simp [offer,ha,hc]⟩

theorem offer_none_iff (s : World n) (u : UseRequest n) :
    offer s u = none ↔ ¬ CurrentUse s u := by
  constructor
  · intro hn hu
    obtain ⟨e,he⟩ := offer_complete s u hu
    simp [hn] at he
  · intro h
    cases he : offer s u with
    | none => rfl
    | some e => exact False.elim (h (offer_sound s u he).1)

def pick (s : World n) : List (UseRequest n) → Option (Nat × UseRequest n × Evidence)
  | [] => none
  | u :: us => match offer s u with
    | some e => some (0,u,e)
    | none => (pick s us).map fun (j,v,e) => (j+1,v,e)

-- Independent least-current-option judgment: no call to pick/offer/checkUse.
inductive First (s : World n) : List (UseRequest n) → Nat → UseRequest n → Prop where
  | head : CurrentUse s u → First s (u :: us) 0 u
  | tail : ¬ CurrentUse s u → First s us j v → First s (u :: us) (j+1) v

theorem First.current (h : First s us j u) : CurrentUse s u := by
  induction h with
  | head hc => exact hc
  | tail _ _ ih => exact ih

theorem First.bound (h : First s us j u) : j < us.length := by
  induction h with
  | head _ => simp
  | tail _ _ ih => simp only [List.length_cons]; omega

theorem pick_sound (s : World n) (us : List (UseRequest n))
    (h : pick s us = some (j,u,e)) : First s us j u ∧ checkUse s u e = true := by
  induction us generalizing j u e with
  | nil => cases h
  | cons v vs ih =>
    unfold pick at h
    cases hv : offer s v with
    | some ev =>
      simp only [hv,Option.some.injEq,Prod.mk.injEq] at h
      rcases h with ⟨rfl,rfl,rfl⟩
      exact ⟨.head (offer_sound s v hv).1,(offer_sound s v hv).2⟩
    | none =>
      simp only [hv] at h
      cases hp : pick s vs with
      | none => simp [hp] at h
      | some row =>
        obtain ⟨k,w,ew⟩ := row
        simp only [hp,Option.map_some,Option.some.injEq,Prod.mk.injEq] at h
        rcases h with ⟨rfl,rfl,rfl⟩
        obtain ⟨hf,hc⟩ := ih hp
        exact ⟨.tail ((offer_none_iff s v).mp hv) hf,hc⟩

theorem pick_complete (h : First s us j u) : ∃ e, pick s us = some (j,u,e) := by
  induction h with
  | @head u us hu =>
    obtain ⟨e,he⟩ := offer_complete s u hu
    exact ⟨e,by simp [pick,he]⟩
  | @tail u us j v hu h ih =>
    obtain ⟨e,he⟩ := ih
    exact ⟨e,by simp [pick,(offer_none_iff s u).mpr hu,he]⟩

structure Cursor (n : Nat) where
  options : List (UseRequest n)
  position : Nat
  lineage : Nat

-- Semantic current-use resolution only. Missing presentation samples do not
-- invoke this function. An exhausted lineage stays at its exhausted position.
def resolve (s : World n) (c : Cursor n) : Cursor n × Option (UseRequest n × Evidence) :=
  match pick s (c.options.drop c.position) with
  | none => ({c with position := max c.position c.options.length},none)
  | some (j,u,e) => ({c with position := c.position+j},some (u,e))

theorem resolve_monotone (s : World n) (c : Cursor n) :
    c.position ≤ (resolve s c).1.position := by
  unfold resolve
  split
  · exact Nat.le_max_left _ _
  · exact Nat.le_add_right _ _

theorem resolve_lineage (s : World n) (c : Cursor n) :
    (resolve s c).1.lineage = c.lineage := by
  unfold resolve; split <;> rfl

theorem resolve_current (s : World n) (c : Cursor n)
    (h : (resolve s c).2 = some (u,e)) : CurrentUse s u ∧ checkUse s u e = true := by
  unfold resolve at h
  split at h
  · cases h
  · cases h
    exact ⟨(pick_sound s _ ‹_›).1.current,(pick_sound s _ ‹_›).2⟩

theorem resolve_bound (s : World n) (c : Cursor n) (bound : c.position ≤ c.options.length) :
    (resolve s c).1.position ≤ c.options.length := by
  unfold resolve
  split
  · simp; omega
  · have h := (pick_sound s _ ‹_›).1.bound
    simp only [List.length_drop] at h
    simp; omega

-- Current logical auth source is retained by pure addition; this is not a
-- theorem that an externally supplied stale authority context is fresh.
theorem grow_offer (s : World n) (a : IdentityGrowth.Addition n m) (u : UseRequest n) :
    offer (IdentityGrowth.grow s a) (IdentityGrowth.mapRequest m u) = offer s u := by
  unfold offer
  rw [IdentityGrowth.grow_context]
  simp only [IdentityGrowth.mapRequest,IdentityGrowth.mapHandle,IdentityGrowth.grow_policy]
  change (match authorize s.authority (s.policies u.operation.key) (currentContext s u) with
    | none => none
    | some e => if checkUse (IdentityGrowth.grow s a) (IdentityGrowth.mapRequest m u) e
        then some e else none) = _
  simp only [IdentityGrowth.grow_checkUse]

theorem grow_pick (s : World n) (a : IdentityGrowth.Addition n m) (us : List (UseRequest n)) :
    pick (IdentityGrowth.grow s a) (us.map (IdentityGrowth.mapRequest m)) =
      (pick s us).map (fun (j,u,e) => (j,IdentityGrowth.mapRequest m u,e)) := by
  induction us with
  | nil => rfl
  | cons u us ih =>
    simp only [List.map,pick,grow_offer]
    cases ho : offer s u with
    | some e => rfl
    | none =>
      simp only [ih]
      cases hp : pick s us with
      | none => rfl
      | some row => obtain ⟨j,v,e⟩ := row; rfl

def mapCursor (m : Nat) (c : Cursor n) : Cursor (n+m) :=
  ⟨c.options.map (IdentityGrowth.mapRequest m),c.position,c.lineage⟩

theorem grow_resolve_position (s : World n) (a : IdentityGrowth.Addition n m) (c : Cursor n) :
    (resolve (IdentityGrowth.grow s a) (mapCursor m c)).1.position = (resolve s c).1.position := by
  simp only [resolve,mapCursor,←List.map_drop,grow_pick,List.length_map]
  cases hp : pick s (c.options.drop c.position) with
  | none => rfl
  | some row => obtain ⟨j,v,e⟩ := row; rfl

namespace Controls
open CurrentUse.Controls
def bad : UseRequest 4 := {request with locus := handle 2}
def cursor : Cursor 4 := ⟨[bad,request],0,12⟩
#guard (resolve world cursor).1.position = 1
#guard (resolve world cursor).2.isSome
#guard (resolve retiredWorld (resolve world cursor).1).1.position = 2
-- A later environment in which the target is eligible again does not rewind
-- the same exhausted cursor; this is not a same-instance restore theorem.
#guard (resolve world (resolve retiredWorld (resolve world cursor).1).1).2.isNone
#guard (resolve world cursor).1.lineage = 12
end Controls

#print axioms offer_sound
#print axioms offer_complete
#print axioms pick_sound
#print axioms pick_complete
#print axioms resolve_monotone
#print axioms resolve_lineage
#print axioms resolve_bound
#print axioms resolve_current
#print axioms grow_offer
#print axioms grow_pick
#print axioms grow_resolve_position
end MirroreaProofFirst.CurrentChoice
