import Std
namespace MirroreaProofFirst.OwnerStatementCounterBudget

-- Two concrete M8 queue counters. Costs are visible call-path counts, not a
-- model of memory allocation, trace bytes, local-runtime counters or transport.
structure Counters where
 occurrence : Nat
 trace : Nat
 deriving DecidableEq, Repr
structure Limits where
 occurrence : Nat
 trace : Nat
 deriving DecidableEq, Repr
structure Charge where
 occurrences : Nat
 traces : Nat
 deriving DecidableEq, Repr

-- The retained M8 leaf allocates one enqueue occurrence and one enqueue trace;
-- successful service has three more trace allocations, known failure at most
-- one. This host/source correspondence is an explicit implementation obligation.
def ownerLeaf : Charge := ⟨1,4⟩
def endAt (before : Counters) (charge : Charge) : Counters :=
 ⟨before.occurrence+charge.occurrences,before.trace+charge.traces⟩
def Fits (limits : Limits) (before : Counters) (charge : Charge) : Prop :=
 before.occurrence+charge.occurrences ≤ limits.occurrence ∧
 before.trace+charge.traces ≤ limits.trace

def reserve (limits : Limits) (before : Counters) (charge : Charge) : Option Counters :=
 if before.occurrence+charge.occurrences ≤ limits.occurrence ∧
   before.trace+charge.traces ≤ limits.trace then some (endAt before charge) else none

theorem reserve_exact : reserve limits before charge = some after ↔
 Fits limits before charge ∧ after = endAt before charge := by
 unfold reserve Fits
 split
 · rename_i room
   constructor
   · intro eq; exact ⟨room,(Option.some.inj eq).symm⟩
   · rintro ⟨_,rfl⟩; rfl
 · rename_i no
   constructor
   · intro impossible; cases impossible
   · rintro ⟨room,_⟩; exact False.elim (no room)

theorem reserve_complete (room : Fits limits before charge) :
 reserve limits before charge = some (endAt before charge) := reserve_exact.mpr ⟨room,rfl⟩

def Within (limits : Limits) (s : Counters) : Prop :=
 s.occurrence ≤ limits.occurrence ∧ s.trace ≤ limits.trace

theorem prefix_fits {spent : Charge} (room : Fits limits before charge)
 (occ : spent.occurrences ≤ charge.occurrences) (trace : spent.traces ≤ charge.traces) :
 Within limits (endAt before spent) := by
 unfold Fits at room
 simp only [Within,endAt]
 omega

-- Unsigned wrapping is specified explicitly so the resource proof cannot
-- silently assume mathematical integers are the final host representation.
def modulus (bits : Nat) : Nat := 2^bits
def maximum (bits : Nat) : Nat := modulus bits - 1
def Native (bits : Nat) (limits : Limits) : Prop :=
 limits.occurrence ≤ maximum bits ∧ limits.trace ≤ maximum bits

theorem prefix_no_wrap {spent : Charge} (native : Native bits limits) (room : Fits limits before charge)
 (occ : spent.occurrences ≤ charge.occurrences) (trace : spent.traces ≤ charge.traces) :
 (before.occurrence+spent.occurrences) % modulus bits = before.occurrence+spent.occurrences ∧
 (before.trace+spent.traces) % modulus bits = before.trace+spent.traces := by
 have within := prefix_fits room occ trace
 have positive : 0 < modulus bits := by exact Nat.two_pow_pos bits
 simp only [Within,endAt] at within
 unfold Native maximum at native
 constructor <;> apply Nat.mod_eq_of_lt <;> omega

-- Given the documented sequential path charge, each pre-increment in the real
-- enqueue/three-service-trace path has a representable successor.
theorem owner_leaf_trace_steps (native : Native bits limits)
 (room : Fits limits before ownerLeaf) (index : Nat) (step : index < 4) :
 before.trace+index+1 < modulus bits ∧
 (before.trace+index+1) % modulus bits = before.trace+index+1 := by
 have total := (prefix_no_wrap native room (spent:=⟨0,index+1⟩) (by change 0 ≤ 1; omega) (by dsimp [ownerLeaf];omega)).2
 have bound := (prefix_fits room (spent:=⟨0,index+1⟩) (by change 0 ≤ 1; omega) (by dsimp [ownerLeaf];omega)).2
 have positive : 0 < modulus bits := by exact Nat.two_pow_pos bits
 unfold Native maximum at native
 dsimp only [endAt] at bound
 exact ⟨by omega,by simpa [Nat.add_assoc] using total⟩

-- A reservation alone cannot survive other writers. Equality with the live
-- starting counters/exclusive queue access must be enforced by the custodian.
def current (expected live : Counters) : Bool := decide (expected = live)
theorem current_exact : current expected live = true ↔ expected = live := by simp [current]

namespace Controls
def limit : Limits := ⟨maximum 64,maximum 64⟩
#guard (reserve limit ⟨0,maximum 64-4⟩ ownerLeaf).isSome
#guard (reserve limit ⟨0,maximum 64-1⟩ ownerLeaf).isNone
#guard (reserve limit ⟨maximum 64,0⟩ ownerLeaf).isNone
#guard (reserve limit ⟨maximum 64-1,maximum 64-4⟩ ownerLeaf) = some ⟨maximum 64,maximum 64⟩
-- Reserving only the one enqueue trace admits the actual postcommit failure.
#guard (reserve limit ⟨0,maximum 64-1⟩ ⟨1,1⟩).isSome
example : ¬Fits limit ⟨0,maximum 64-1⟩ ownerLeaf := by unfold Fits ownerLeaf Controls.limit maximum modulus; decide
#guard current ⟨1,4⟩ ⟨1,5⟩ = false
end Controls

#print axioms reserve_exact
#print axioms reserve_complete
#print axioms prefix_fits
#print axioms prefix_no_wrap
#print axioms owner_leaf_trace_steps
#print axioms current_exact
end MirroreaProofFirst.OwnerStatementCounterBudget
