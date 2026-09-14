import MirroreaProofFirstSupportTable
import MirroreaProofFirstGraphValidation

namespace MirroreaProofFirst.GraphTable

def reachable (edge : Fin n → Fin n → Bool) (start target : Fin n) : Bool :=
  SupportTable.live {forms := GraphValidation.reachForms edge start,eligible := fun _ => true} target

theorem reachable_same (edge : Fin n → Fin n → Bool) (start target : Fin n) :
    reachable edge start target = GraphValidation.reachable edge start target := by
  simp [reachable,SupportTable.live_exact,Support.snapshotLive,GraphValidation.reachable]

def checkAcyclic (edge : Fin n → Fin n → Bool) : Bool :=
  (List.finRange n).all fun x => (List.finRange n).all fun y =>
    decide (edge x y = true → reachable edge y x ≠ true)

theorem check_same (edge : Fin n → Fin n → Bool) : checkAcyclic edge = GraphValidation.checkAcyclic edge := by
  simp [checkAcyclic,GraphValidation.checkAcyclic,reachable_same]

theorem check_exact (edge : Fin n → Fin n → Bool) :
    checkAcyclic edge = true ↔ GraphValidation.Acyclic (fun x y => edge x y = true) := by
  rw [check_same]
  exact GraphValidation.checkAcyclic_exact edge

#print axioms reachable_same
#print axioms check_same
#print axioms check_exact
end MirroreaProofFirst.GraphTable
