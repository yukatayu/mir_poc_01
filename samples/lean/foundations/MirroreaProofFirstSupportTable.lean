import MirroreaProofFirstSupport

namespace MirroreaProofFirst.SupportTable

-- Materialize each whole finite round once. This changes the executable
-- representation, not dependency admissibility or least-fixed-point meaning.
def rounds (s : Support.Snapshot n) : Nat → Vector Bool n
  | 0 => Vector.replicate n false
  | count+1 =>
      let previous := rounds s count
      Vector.ofFn (fun k => previous[k.val] ||
        (s.eligible k && Support.eval (fun j => previous[j.val]) (s.forms k)))

theorem rounds_exact (s : Support.Snapshot n) (count : Nat) :
    ∀ k : Fin n, (rounds s count)[k.val] = Support.rounds s.forms s.eligible count k := by
  induction count with
  | zero => intro k; simp [rounds,Support.rounds]
  | succ count ih =>
      intro k
      have same : (fun j : Fin n => (rounds s count)[j.val]) = Support.rounds s.forms s.eligible count := funext ih
      simp only [rounds,Vector.getElem_ofFn,Fin.eta,Support.rounds]
      rw [ih k,same]

def live (s : Support.Snapshot n) : Fin n → Bool :=
  let table := rounds s n
  fun k => table[k.val]

theorem live_exact (s : Support.Snapshot n) : live s = Support.snapshotLive s := by
  funext k
  simpa [live,Support.snapshotLive] using rounds_exact s n k

theorem grounded_exact (s : Support.Snapshot n) (k : Fin n) :
    live s k = true ↔ Support.Grounded s.forms (fun j => s.eligible j = true) k := by
  rw [live_exact]
  exact Support.snapshot_live_exact s k

#print axioms rounds_exact
#print axioms live_exact
#print axioms grounded_exact
end MirroreaProofFirst.SupportTable
