import MirroreaProofFirstOwnerCommitJournal
open MirroreaProofFirst OwnerCommitJournal
namespace OwnerStatementJournal
set_option maxHeartbeats 800000

-- Every input denotes ONE captured completed store of the current writer.
-- Authentic code/frame/occurrence custody is an independent physical premise;
-- this checker does not manufacture it from an equal value or a hash.
structure Observation (p a : Nat) where
  store : Store p a
  after : Memory p a

def storeEq : Store p a → Store p a → Bool
  | .credits x,.credits y | .revision x,.revision y => decide (x = y)
  | .image x,.image y => OwnerEndpoint.sameImage x y
  | .keys x,.keys y => decide (x = y)
  | .releaseLease,.releaseLease | .clearEntered,.clearEntered => true
  | _,_ => false

theorem storeEq_exact : storeEq left right = true ↔ left = right := by
  cases left <;> cases right <;> simp [storeEq,OwnerEndpoint.sameImage_exact]

def advance (before : Journal p a) (observed : Observation p a) : Option (Journal p a) :=
  match before with
  | ⟨memory,head::rest,false⟩ =>
    if storeEq observed.store head && memoryEq observed.after (write memory head) then
      some ⟨write memory head,rest,false⟩
    else none
  | _ => none

-- Independent rule: the exact next head, its one resulting state and the
-- corresponding observation. No future target/invariant is a guard.
inductive Step : Journal p a → Observation p a → Journal p a → Prop where
  | commit : Step ⟨memory,head::rest,false⟩ ⟨head,write memory head⟩ ⟨write memory head,rest,false⟩

theorem advance_exact : advance before observed = some after ↔ Step before observed after := by
  constructor
  · intro checked
    rcases before with ⟨memory,pending,stopped⟩
    cases pending with
    | nil => simp [advance] at checked
    | cons head rest =>
      cases stopped with
      | true => simp [advance] at checked
      | false =>
        simp only [advance] at checked
        split at checked
        · rename_i equal
          have facts : observed.store = head ∧ observed.after = write memory head := by
            simpa [storeEq_exact,memoryEq_exact] using equal
          rcases observed with ⟨submitted,value⟩
          have storeEqual : submitted = head := facts.1
          have valueEqual : value = write memory head := facts.2
          subst submitted
          subst value
          cases Option.some.inj checked
          exact .commit
        · cases checked
  · intro step
    cases step
    simp [advance,storeEq_exact,memoryEq_exact]

theorem step_projection (step : Step before observed after) : OwnerCommitJournal.Step before .commit after := by
  cases step; exact .commit

theorem step_observed (step : Step before observed after) : after.memory = observed.after := by
  cases step; rfl

theorem step_count (step : Step before observed after) : after.pending.length + 1 = before.pending.length := by
  cases step; rfl

theorem target_preserved (step : Step before observed after) : target after = target before :=
  preserves_target (step_projection step)

inductive Runs : Journal p a → List (Observation p a) → Journal p a → Prop where
  | nil : Runs before [] before
  | cons : Step before observed middle → Runs middle rest after → Runs before (observed::rest) after

def run (before : Journal p a) : List (Observation p a) → Option (Journal p a)
  | [] => some before
  | observed::rest => do
    let next ← advance before observed
    run next rest

theorem run_exact : run before observations = some after ↔ Runs before observations after := by
  induction observations generalizing before with
  | nil => simp only [run,Option.some.injEq]; exact ⟨fun equal => equal ▸ Runs.nil,fun path => by cases path; rfl⟩
  | cons observed rest ih =>
    constructor
    · intro checked
      obtain ⟨next,first,rest⟩ := Option.bind_eq_some_iff.mp checked
      exact .cons (advance_exact.mp first) (ih.mp rest)
    · intro path
      cases path with
      | cons first rest => simp [run,advance_exact.mpr first,ih.mpr rest]

theorem Runs.count (path : Runs before observations after) :
    observations.length + after.pending.length = before.pending.length := by
  induction path with
  | nil => simp
  | cons first rest ih =>
    have one := step_count first
    simp only [List.length_cons]; omega

def finish (journal : Journal p a) (observed : Memory p a) : Bool :=
  !journal.stopped && journal.pending.isEmpty && memoryEq journal.memory observed

theorem finish_exact : finish journal observed = true ↔
    journal.stopped = false ∧ journal.pending = [] ∧ journal.memory = observed := by
  simp [finish,memoryEq_exact,and_assoc]

theorem completion_count (path : Runs before observations after) (done : finish after final = true) :
    observations.length = before.pending.length := by
  have counts := path.count
  rw [(finish_exact.mp done).2.1] at counts
  simpa using counts

theorem missing_assignment_refused (memory : Memory p a) (head : Store p a)
    (rest : List (Store p a)) (observed : Memory p a) :
    finish ⟨memory,head::rest,false⟩ observed = false := by simp [finish]

theorem swapped_idempotent_refused (data : Data p a) :
    advance ⟨⟨data,none,false⟩,[.releaseLease,.clearEntered],false⟩
      ⟨.clearEntered,⟨data,none,false⟩⟩ = none := rfl

-- Arbitrary typed data, not a fixed decide fixture: both actual no-op
-- assignments are admitted, counted separately, and then allowed to finish.
theorem idempotent_pair_admitted (data : Data p a) :
    run ⟨⟨data,none,false⟩,[.releaseLease,.clearEntered],false⟩
      [⟨.releaseLease,⟨data,none,false⟩⟩,⟨.clearEntered,⟨data,none,false⟩⟩] =
      some ⟨⟨data,none,false⟩,[],false⟩ ∧
    finish ⟨⟨data,none,false⟩,[],false⟩ ⟨data,none,false⟩ = true := by
  simp [run,advance,write,storeEq,finish,memoryEq_exact]

#print axioms storeEq_exact
#print axioms advance_exact
#print axioms step_observed
#print axioms target_preserved
#print axioms run_exact
#print axioms Runs.count
#print axioms finish_exact
#print axioms completion_count
#print axioms missing_assignment_refused
#print axioms swapped_idempotent_refused
#print axioms idempotent_pair_admitted
end OwnerStatementJournal
