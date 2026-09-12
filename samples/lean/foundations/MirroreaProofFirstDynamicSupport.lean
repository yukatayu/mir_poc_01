import MirroreaProofFirstSupport

/- Nonproduction W3 candidate. Indices are not semantic identities. This first
   dependency isolates support under finite domain growth, not lifecycle/auth. -/
namespace MirroreaProofFirst.Growth
open Support

def mapFormula (embed : K → L) : Formula K → Formula L
 | .top => .top
 | .bottom => .bottom
 | .ref k => .ref (embed k)
 | .both p q => .both (mapFormula embed p) (mapFormula embed q)
 | .either p q => .either (mapFormula embed p) (mapFormula embed q)

theorem eval_map (embed : K → L) (available : L → Bool) (f : Formula K) :
 eval available (mapFormula embed f) = eval (fun k => available (embed k)) f := by
 induction f with
 | top => rfl
 | bottom => rfl
 | ref k => rfl
 | both p q hp hq => simp [mapFormula,eval,hp,hq]
 | either p q hp hq => simp [mapFormula,eval,hp,hq]

-- Closed old dependencies and unchanged old eligibility are input conditions,
-- not an assumed equality of derived live sets or checker verdicts.
theorem rounds_embedding (embed : K → L)
 (oldForms : K → Formula K) (newForms : L → Formula L)
 (oldEligible : K → Bool) (newEligible : L → Bool)
 (forms : ∀ k, newForms (embed k) = mapFormula embed (oldForms k))
 (eligible : ∀ k, newEligible (embed k) = oldEligible k) (fuel : Nat) :
 ∀ k, rounds newForms newEligible fuel (embed k) = rounds oldForms oldEligible fuel k := by
 induction fuel with
 | zero => intro k; rfl
 | succ fuel ih =>
   intro k
   simp only [rounds,forms,eligible,eval_map,ih]

theorem grounded_embedding (embed : K → L)
 (oldForms : K → Formula K) (newForms : L → Formula L)
 (oldEligible : K → Bool) (newEligible : L → Bool)
 (forms : ∀ k, newForms (embed k) = mapFormula embed (oldForms k))
 (eligible : ∀ k, newEligible (embed k) = oldEligible k) (k : K) :
 Grounded newForms (fun j => newEligible j = true) (embed k) ↔
 Grounded oldForms (fun j => oldEligible j = true) k := by
 rw [←rounds_iff_derivation,←rounds_iff_derivation]
 simp only [rounds_embedding embed oldForms newForms oldEligible newEligible forms eligible]

-- The abstract support lemma does not need injectivity. Actual domain extension
-- DOES preserve distinct slots; identity correspondence is still a later boundary.
def left (n m : Nat) (k : Fin n) : Fin (n+m) := ⟨k.val, by omega⟩

theorem left_injective (n m : Nat) : Function.Injective (left n m) := by
 intro a b eq
 apply Fin.ext
 exact congrArg (fun x : Fin (n+m) => x.val) eq

def append (old : Snapshot n) (moreForms : Fin m → Formula (Fin (n+m)))
 (moreEligible : Fin m → Bool) : Snapshot (n+m) where
 forms k := if h : k.val < n then mapFormula (left n m) (old.forms ⟨k.val,h⟩)
            else moreForms ⟨k.val-n,by omega⟩
 eligible k := if h : k.val < n then old.eligible ⟨k.val,h⟩
               else moreEligible ⟨k.val-n,by omega⟩

theorem append_forms (old : Snapshot n) (forms : Fin m → Formula (Fin (n+m)))
 (eligible : Fin m → Bool) (k : Fin n) :
 (append old forms eligible).forms (left n m k) = mapFormula (left n m) (old.forms k) := by
 simp [append,left,k.isLt]

theorem append_eligible (old : Snapshot n) (forms : Fin m → Formula (Fin (n+m)))
 (eligible : Fin m → Bool) (k : Fin n) :
 (append old forms eligible).eligible (left n m k) = old.eligible k := by
 simp [append,left,k.isLt]

theorem append_preserves_grounded (old : Snapshot n)
 (forms : Fin m → Formula (Fin (n+m))) (eligible : Fin m → Bool) (k : Fin n) :
 Grounded (append old forms eligible).forms
   (fun j => (append old forms eligible).eligible j = true) (left n m k) ↔
 Grounded old.forms (fun j => old.eligible j = true) k :=
 grounded_embedding (left n m) _ _ _ _ (append_forms old forms eligible)
   (append_eligible old forms eligible) k

theorem append_preserves_computed_live (old : Snapshot n)
 (forms : Fin m → Formula (Fin (n+m))) (eligible : Fin m → Bool) (k : Fin n) :
 snapshotLive (append old forms eligible) (left n m k) = snapshotLive old k := by
 apply Bool.eq_iff_iff.mpr
 rw [snapshot_live_exact,snapshot_live_exact]
 exact append_preserves_grounded old forms eligible k

theorem append_certificate_checked (old : Snapshot n)
 (forms : Fin m → Formula (Fin (n+m))) (eligible : Fin m → Bool) :
 let out := append old forms eligible
 checkSnapshot out (snapshotLive out) (snapshotRank out) = true :=
 snapshot_certificate_accepted _

theorem append_checked_old_exact (old : Snapshot n)
 (forms : Fin m → Formula (Fin (n+m))) (eligible : Fin m → Bool)
 (live : Fin (n+m) → Bool) (rank : Fin (n+m) → Nat)
 (checked : checkSnapshot (append old forms eligible) live rank = true) (k : Fin n) :
 live (left n m k) = true ↔ Grounded old.forms (fun j => old.eligible j = true) k :=
 ((snapshot_checked_sound _ _ _ checked).1 _).trans (append_preserves_grounded old forms eligible k)

namespace Controls
-- Pure support examples, not deployments: three old nodes and two NEW slots.
def base : Snapshot 3 := ⟨fun _ => .top,fun _ => true⟩
def rootedForms (k : Fin 2) : Formula (Fin 5) :=
 if k.val = 0 then .either (.ref 4) (.ref 0) else .ref 3
def rootlessForms (k : Fin 2) : Formula (Fin 5) :=
 if k.val = 0 then .ref 4 else .ref 3
def rooted := append base rootedForms (fun _ => true)
def rootless := append base rootlessForms (fun _ => true)
#guard (List.finRange 5).all (snapshotLive rooted)
#guard snapshotLive rootless 0 && snapshotLive rootless 1 && snapshotLive rootless 2
#guard !(snapshotLive rootless 3) && !(snapshotLive rootless 4)
-- Omitting the new derivable nodes is not a valid full-domain certificate.
#guard !checkSnapshot rooted (fun k => decide (k.val < 3)) (fun _ => 0)
-- The two old-domain hypotheses cannot be omitted.
def retiredOld : Snapshot 3 := {base with eligible := fun _ => false}
def cyclicOld : Snapshot 3 := {base with forms := fun k => .ref k}
#guard snapshotLive base 0 && !snapshotLive retiredOld 0
#guard snapshotLive base 0 && !snapshotLive cyclicOld 0
end Controls

#print axioms eval_map
#print axioms rounds_embedding
#print axioms grounded_embedding
#print axioms left_injective
#print axioms append_preserves_grounded
#print axioms append_preserves_computed_live
#print axioms append_certificate_checked
#print axioms append_checked_old_exact
end MirroreaProofFirst.Growth
