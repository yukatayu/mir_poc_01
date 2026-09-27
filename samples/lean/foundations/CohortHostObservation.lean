import CohortHostPrefix
open MirroreaProofFirst
namespace CohortHostObservation
open CohortCommitJournal

-- Runtime sets have no observable insertion order. Every member is nevertheless
-- a complete typed value; matching a hash or a count is not this relation.
def SameSet (left right : List α) : Prop := left ⊆ right ∧ right ⊆ left
def setEq [DecidableEq α] (left right : List α) : Bool :=
  left.all (fun value => decide (value ∈ right)) && right.all (fun value => decide (value ∈ left))
theorem setEq_exact [DecidableEq α] {left right : List α} : setEq left right = true ↔ SameSet left right := by
  simp [setEq,SameSet,List.subset_def,List.all_eq_true]
theorem SameSet.membership {left right : List α} (same : SameSet left right) (value : α) : value ∈ left ↔ value ∈ right :=
  ⟨fun member => same.1 member,fun member => same.2 member⟩

def paymentKey (value : PaidHeadPhase.Pending p) :=
  (value.freeze,value.target,value.revision,value.sourceOrdinal)
def paymentEq (left right : Option (PaidHeadPhase.Pending p)) : Bool :=
  left.map paymentKey == right.map paymentKey
theorem paymentEq_exact : paymentEq left right = true ↔ left = right := by
  cases left with
  | none => cases right <;> simp [paymentEq]
  | some left =>
    cases right with
    | none => simp [paymentEq]
    | some right =>
      cases left; cases right
      simp [paymentEq,paymentKey]

def Matches (expected observed : Memory p a) : Prop :=
  expected.bootstrapped = observed.bootstrapped ∧ expected.snapshot = observed.snapshot ∧
  SameSet expected.initialized observed.initialized ∧ SameSet expected.freezes observed.freezes ∧
  SameSet expected.installs observed.installs ∧ SameSet expected.produced observed.produced ∧
  expected.pendingPayment = observed.pendingPayment

def memoryEq (expected observed : Memory p a) : Bool :=
  decide (expected.bootstrapped = observed.bootstrapped) &&
  SourceEntryCapture.snapshotEq expected.snapshot observed.snapshot &&
  setEq expected.initialized observed.initialized && setEq expected.freezes observed.freezes &&
  setEq expected.installs observed.installs && setEq expected.produced observed.produced &&
  paymentEq expected.pendingPayment observed.pendingPayment

theorem memoryEq_exact : memoryEq expected observed = true ↔ Matches expected observed := by
  simp [memoryEq,Matches,SourceEntryCapture.snapshotEq_exact,setEq_exact,paymentEq_exact,and_assoc]

theorem snapshot_refused (different : expected.snapshot ≠ observed.snapshot) : memoryEq expected observed = false := by
  cases checked : memoryEq expected observed with
  | false => rfl
  | true => exact False.elim (different (memoryEq_exact.mp checked).2.1)

theorem payment_refused (different : expected.pendingPayment ≠ observed.pendingPayment) : memoryEq expected observed = false := by
  cases checked : memoryEq expected observed with
  | false => rfl
  | true => exact False.elim (different (memoryEq_exact.mp checked).2.2.2.2.2.2)

theorem unrecorded_install_refused (new : value ∈ observed.installs) (absent : value ∉ expected.installs) :
    memoryEq expected observed = false := by
  cases checked : memoryEq expected observed with
  | false => rfl
  | true => exact False.elim (absent (((memoryEq_exact.mp checked).2.2.2.2.1).2 new))

theorem self_accepts (memory : Memory p a) : memoryEq memory memory = true := by
  apply memoryEq_exact.mpr
  simp [Matches,SameSet]

#print axioms memoryEq_exact
#print axioms snapshot_refused
#print axioms payment_refused
#print axioms unrecorded_install_refused
#print axioms self_accepts
end CohortHostObservation
