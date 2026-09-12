import MirroreaProofFirstGraphValidation

namespace MirroreaProofFirst.GraphGrowth
open GraphValidation

-- Edges point from a dependent to its prerequisite. Different graph kinds
-- instantiate this construction independently; support is not this edge union.
def downstream (old : K → K → Prop) (fresh : N → N → Prop)
    (uses : N → K → Prop) : Sum K N → Sum K N → Prop
  | .inl a, .inl b => old a b
  | .inl _, .inr _ => False
  | .inr a, .inl b => uses a b
  | .inr a, .inr b => fresh a b

theorem path_from_old (h : Path (downstream old fresh uses) x y) :
    ∀ a, x = Sum.inl a → ∃ b, y = Sum.inl b ∧ Path old a b := by
  induction h with
  | refl => intro a hx; exact ⟨a,hx,.refl⟩
  | @step y z before edge ih =>
    intro a hx
    obtain ⟨b,hy,hp⟩ := ih a hx
    subst y
    cases z with
    | inl c => exact ⟨c,rfl,.step hp edge⟩
    | inr c => exact False.elim edge

theorem path_between_new (h : Path (downstream old fresh uses) x y) :
    ∀ a b, x = Sum.inr a → y = Sum.inr b → Path fresh a b := by
  induction h with
  | refl =>
    intro a b ha hb
    have : a = b := Sum.inr.inj (ha.symm.trans hb)
    subst b
    exact .refl
  | @step y z before edge ih =>
    intro a b hx hz
    subst z
    cases y with
    | inl c => exact False.elim edge
    | inr c => exact .step (ih a c hx rfl) edge

theorem downstream_acyclic (ho : Acyclic old) (hn : Acyclic fresh) :
    Acyclic (downstream old fresh uses) := by
  intro x y edge back
  cases x with
  | inl a =>
    cases y with
    | inl b =>
      obtain ⟨c,eq,hp⟩ := path_from_old back b rfl
      cases eq
      exact ho a b edge hp
    | inr b => exact False.elim edge
  | inr a =>
    cases y with
    | inl b =>
      obtain ⟨c,eq,_⟩ := path_from_old back b rfl
      cases eq
    | inr b => exact hn a b edge (path_between_new back b a rfl rfl)

theorem lift_old_path (h : Path old x y) :
    Path (downstream old fresh uses) (.inl x) (.inl y) := by
  induction h with
  | refl => exact .refl
  | step _ e ih => exact .step ih e

theorem lift_new_path (h : Path fresh x y) :
    Path (downstream old fresh uses) (.inr x) (.inr y) := by
  induction h with
  | refl => exact .refl
  | step _ e ih => exact .step ih e

-- Both component premises are necessary, and can be recovered from the result.
theorem downstream_acyclic_exact :
    Acyclic (downstream old fresh uses) ↔ Acyclic old ∧ Acyclic fresh := by
  constructor
  · intro h
    exact ⟨fun a b edge back => h (.inl a) (.inl b) edge (lift_old_path back),
      fun a b edge back => h (.inr a) (.inr b) edge (lift_new_path back)⟩
  · rintro ⟨ho,hn⟩
    exact downstream_acyclic ho hn

theorem each_kind_preserved (old : G → K → K → Prop)
    (fresh : G → N → N → Prop) (uses : G → N → K → Prop)
    (ho : ∀ g, Acyclic (old g)) (hn : ∀ g, Acyclic (fresh g)) :
    ∀ g, Acyclic (downstream (old g) (fresh g) (uses g)) :=
  fun g => downstream_acyclic (ho g) (hn g)

namespace Controls
-- Two individually acyclic graph kinds whose UNION is cyclic: requiring one
-- world-wide DAG would reject this lawful separated-graph shape.
def first (a b : Fin 2) : Bool := decide (a = 0 ∧ b = 1)
def second (a b : Fin 2) : Bool := decide (a = 1 ∧ b = 0)
#guard checkAcyclic first
#guard checkAcyclic second
#guard !checkAcyclic (fun a b => first a b || second a b)
-- Adding the reverse edge breaks the downstream precondition and acyclicity.
#guard !checkAdd first 1 0
#guard checkAdd first 0 1
end Controls

#print axioms path_from_old
#print axioms path_between_new
#print axioms downstream_acyclic
#print axioms downstream_acyclic_exact
#print axioms each_kind_preserved
end MirroreaProofFirst.GraphGrowth
