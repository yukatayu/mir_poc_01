import MirroreaProofFirstCohortCommitJournal
namespace MirroreaProofFirst.EntryAcquisition

-- Reference-profile local resource accounting. The native RLock depth query
-- reports this thread's intrinsic depth; another thread cannot mutate owned
-- gate state while this thread retains its RLock. Cleanup is uninterrupted.
-- `stage` is a proof index, not a Python flag whose store is assumed atomic
-- with acquire. Actual cleanup reads only depth, gate and the prior snapshot.
inductive Stage where
  | fresh | locked | noted | holding | closed
  deriving DecidableEq, BEq
structure Memory where
  depth : Nat
  gate : Bool
  snapshot : Option Bool
  stage : Stage
  retired : Bool
  deriving DecidableEq, BEq
inductive Action where
  -- `interrupt` records completed serialized retirement in this local account.
  -- It is NOT a physical store permitted to an interrupted non-owning waiter.
  -- Actual waiting-thread retirement obtains custody before changing the bit.
  | lock | note | acquireGate | nested | interrupt
  deriving DecidableEq, BEq

def initial (baseline : Nat) (parentGate retired : Bool) : Memory :=
  ⟨baseline,parentGate,none,.fresh,retired⟩

def advance (s : Memory) : Action → Option Memory
  | .lock => if decide (s.stage = .fresh) then some {s with depth:=s.depth+1,stage:=.locked} else none
  | .note => if decide (s.stage = .locked) then some {s with snapshot:=some s.gate,stage:=.noted} else none
  | .acquireGate => if decide (s.stage = .noted) && !s.gate then some {s with gate:=true,stage:=.holding} else none
  | .nested => if decide (s.stage = .holding) then some {s with depth:=s.depth+1} else none
  | .interrupt => some {s with retired:=true}

inductive Step : Memory → Action → Memory → Prop where
  | lock (stage : s.stage = .fresh) : Step s .lock {s with depth:=s.depth+1,stage:=.locked}
  | note (stage : s.stage = .locked) : Step s .note {s with snapshot:=some s.gate,stage:=.noted}
  | acquire (stage : s.stage = .noted) (free : s.gate = false) :
      Step s .acquireGate {s with gate:=true,stage:=.holding}
  | nested (stage : s.stage = .holding) : Step s .nested {s with depth:=s.depth+1}
  | interrupt : Step s .interrupt {s with retired:=true}

theorem advance_complete (step : Step s action next) : advance s action = some next := by
  cases step <;> simp_all [advance]
theorem advance_sound (checked : advance s action = some next) : Step s action next := by
  cases action with
  | lock =>
    simp only [advance] at checked
    split at checked
    · rename_i yes; cases Option.some.inj checked; exact .lock (by simpa using yes)
    · cases checked
  | note =>
    simp only [advance] at checked
    split at checked
    · rename_i yes; cases Option.some.inj checked; exact .note (by simpa using yes)
    · cases checked
  | acquireGate =>
    simp only [advance] at checked
    split at checked
    · rename_i yes
      have guard : s.stage = .noted ∧ s.gate = false := by simpa using yes
      cases Option.some.inj checked; exact .acquire guard.1 guard.2
    · cases checked
  | nested =>
    simp only [advance] at checked
    split at checked
    · rename_i yes; cases Option.some.inj checked; exact .nested (by simpa using yes)
    · cases checked
  | interrupt => cases Option.some.inj checked; exact .interrupt
theorem advance_exact : advance s action = some next ↔ Step s action next := ⟨advance_sound,advance_complete⟩

-- parentGate names the actual gate state when this attempt obtains exclusive
-- lock custody. This is not a stale observation made before waiting on a peer
-- thread's lock. The uncontended initial state can choose that actual value.
def Invariant (baseline : Nat) (parentGate : Bool) (s : Memory) : Prop :=
  match s.stage with
  | .fresh => s.depth = baseline ∧ s.gate = parentGate ∧ s.snapshot = none
  | .locked => baseline < s.depth ∧ s.gate = parentGate ∧ s.snapshot = none
  | .noted => baseline < s.depth ∧ s.gate = parentGate ∧ s.snapshot = some parentGate
  | .holding => baseline < s.depth ∧ parentGate = false ∧ s.gate = true ∧ s.snapshot = some false
  | .closed => s.depth = baseline ∧ s.gate = parentGate

theorem initial_valid : Invariant baseline parentGate (initial baseline parentGate retired) := by
  simp [Invariant,initial]

theorem preserves (valid : Invariant baseline parentGate s) (step : Step s action next) :
    Invariant baseline parentGate next := by
  cases step with
  | lock stage =>
    simp only [Invariant,stage] at valid
    simp [Invariant,valid.1,valid.2.1,valid.2.2]
  | note stage =>
    simp only [Invariant,stage] at valid
    simp [Invariant,valid.1,valid.2.1]
  | acquire stage free =>
    simp only [Invariant,stage] at valid
    simp [Invariant,valid.1,←valid.2.1,free,valid.2.2]
  | nested stage =>
    simp only [Invariant,stage] at valid
    simp only [Invariant,stage]
    exact ⟨by omega,valid.2⟩
  | interrupt => cases stage : s.stage <;> simpa [Invariant,stage] using valid

-- Actual repeated releases, not an abstract atomic depth reset.
inductive ReleaseRuns (baseline : Nat) : Nat → Nat → Prop where
  | nil : ReleaseRuns baseline depth depth
  | release (owned : baseline < depth) : ReleaseRuns baseline (depth-1) after → ReleaseRuns baseline depth after

def drain (baseline : Nat) : Nat → Nat
  | 0 => 0
  | depth+1 => if baseline < depth+1 then drain baseline depth else depth+1

theorem drain_bounded (bounded : baseline ≤ depth) : drain baseline depth = baseline := by
  induction depth with
  | zero =>
    have zero : baseline = 0 := by omega
    subst baseline
    rfl
  | succ depth ih =>
    simp only [drain]
    split
    · exact ih (by omega)
    · omega

theorem drain_path : ReleaseRuns baseline depth (drain baseline depth) := by
  induction depth with
  | zero => exact .nil
  | succ depth ih =>
    simp only [drain]
    split
    · rename_i owned; exact .release owned (by simpa using ih)
    · exact .nil

def cleanup (baseline : Nat) (s : Memory) : Memory :=
  if baseline < s.depth then
    {s with depth:=drain baseline s.depth,gate:=(if s.snapshot == some false then false else s.gate),stage:=.closed}
  else {s with stage:=.closed}

theorem cleanup_restores (valid : Invariant baseline parentGate s) :
    (cleanup baseline s).depth = baseline ∧ (cleanup baseline s).gate = parentGate ∧
    (cleanup baseline s).retired = s.retired := by
  cases stage : s.stage <;> simp only [Invariant,stage] at valid
  case fresh => simp [cleanup,valid.1,valid.2.1]
  case locked => simp [cleanup,valid.1,valid.2.1,valid.2.2,drain_bounded (Nat.le_of_lt valid.1)]
  case noted => cases parentGate <;> simp_all [cleanup,drain_bounded (Nat.le_of_lt valid.1)]
  case holding => simp [cleanup,valid.1,valid.2.1,valid.2.2.2,drain_bounded (Nat.le_of_lt valid.1)]
  case closed => simp [cleanup,valid.1,valid.2]

theorem cleanup_preserves (valid : Invariant baseline parentGate s) :
    Invariant baseline parentGate (cleanup baseline s) := by
  have facts := cleanup_restores valid
  have terminal : (cleanup baseline s).stage = .closed := by unfold cleanup; split <;> rfl
  simp only [Invariant,terminal]
  exact ⟨facts.1,facts.2.1⟩

theorem failed_reentry_preserves_parent (valid : Invariant baseline true s) :
    (cleanup baseline s).gate = true := (cleanup_restores valid).2.1
theorem fresh_own_gate_released (valid : Invariant baseline false s) :
    (cleanup baseline s).gate = false := (cleanup_restores valid).2.1
theorem retired_cleanup (valid : Invariant baseline parentGate s) (retired : s.retired = true) :
    (cleanup baseline s).retired = true := (cleanup_restores valid).2.2.trans retired

-- Physical lock acquisition during a retired probe is possible; logical
-- admission is a distinct check and cannot be derived from gate ownership.
def admitted (s : Memory) : Bool := decide (s.stage = .holding) && !s.retired
theorem retired_no_admission (retired : s.retired = true) : admitted s = false := by simp [admitted,retired]

theorem cleanup_no_admission : admitted (cleanup baseline s) = false := by
  unfold cleanup
  split <;> simp [admitted]

#print axioms cleanup_preserves
#print axioms cleanup_no_admission
#print axioms advance_exact
#print axioms preserves
#print axioms drain_path
#print axioms cleanup_restores
#print axioms failed_reentry_preserves_parent
#print axioms retired_no_admission

namespace Restoration
-- Local cleanup PC, separate from the acquisition invariant: after releasing
-- the gate, the original .holding predicate is deliberately no longer true.
-- This is an ownership-boundary path. Once the last RLock depth is released,
-- the endpoint records what THIS attempt relinquished, not a claim freezing
-- the live shared gate/retired fields against future environmental steps.
-- Successful uninterrupted cleanup applies to NORMAL and exceptional entry.
inductive Phase where
  | body | gate | depth | done
  deriving DecidableEq, BEq
structure State where
  memory : Memory
  phase : Phase
  deriving DecidableEq, BEq
inductive Action where
  | begin | releaseGate | skipGate | releaseDepth | finish
  deriving DecidableEq, BEq

def ownGate (baseline : Nat) (memory : Memory) : Bool :=
  decide (baseline < memory.depth) && (memory.snapshot == some false) && memory.gate

def advance (baseline : Nat) (s : State) : Action → Option State
  | .begin => if decide (s.phase = .body) then some {s with phase:=.gate} else none
  | .releaseGate => if decide (s.phase = .gate) && ownGate baseline s.memory then
      some ⟨{s.memory with gate:=false},.depth⟩ else none
  | .skipGate => if decide (s.phase = .gate) && !ownGate baseline s.memory then
      some {s with phase:=.depth} else none
  | .releaseDepth => if decide (s.phase = .depth) && decide (baseline < s.memory.depth) then
      some ⟨{s.memory with depth:=s.memory.depth-1},.depth⟩ else none
  | .finish => if decide (s.phase = .depth) && decide (s.memory.depth ≤ baseline) then
      some ⟨{s.memory with stage:=.closed},.done⟩ else none

inductive Step (baseline : Nat) : State → Action → State → Prop where
  | begin (phase : s.phase = .body) : Step baseline s .begin {s with phase:=.gate}
  | releaseGate (phase : s.phase = .gate) (owned : baseline < s.memory.depth)
      (snapshot : s.memory.snapshot = some false) (locked : s.memory.gate = true) :
      Step baseline s .releaseGate ⟨{s.memory with gate:=false},.depth⟩
  | skipGate (phase : s.phase = .gate) (skip : ownGate baseline s.memory = false) :
      Step baseline s .skipGate {s with phase:=.depth}
  | releaseDepth (phase : s.phase = .depth) (owned : baseline < s.memory.depth) :
      Step baseline s .releaseDepth ⟨{s.memory with depth:=s.memory.depth-1},.depth⟩
  | finish (phase : s.phase = .depth) (done : s.memory.depth ≤ baseline) :
      Step baseline s .finish ⟨{s.memory with stage:=.closed},.done⟩

theorem ownGate_exact : ownGate baseline memory = true ↔
    baseline < memory.depth ∧ memory.snapshot = some false ∧ memory.gate = true := by
  simp [ownGate,and_assoc]

theorem advance_complete (step : Step baseline s action next) : advance baseline s action = some next := by
  cases step <;> simp_all [advance,ownGate]

theorem advance_sound (checked : advance baseline s action = some next) : Step baseline s action next := by
  cases action with
  | begin =>
    simp only [advance] at checked
    split at checked
    · rename_i yes; cases Option.some.inj checked; exact .begin (by simpa using yes)
    · cases checked
  | releaseGate =>
    simp only [advance] at checked
    split at checked
    · rename_i yes
      have guard : s.phase = .gate ∧ baseline < s.memory.depth ∧ s.memory.snapshot = some false ∧ s.memory.gate = true := by
        simpa [ownGate,and_assoc] using yes
      cases Option.some.inj checked; exact .releaseGate guard.1 guard.2.1 guard.2.2.1 guard.2.2.2
    · cases checked
  | skipGate =>
    simp only [advance] at checked
    split at checked
    · rename_i yes
      have guard : s.phase = .gate ∧ ownGate baseline s.memory = false := by simpa using yes
      cases Option.some.inj checked; exact .skipGate guard.1 guard.2
    · cases checked
  | releaseDepth =>
    simp only [advance] at checked
    split at checked
    · rename_i yes
      have guard : s.phase = .depth ∧ baseline < s.memory.depth := by simpa using yes
      cases Option.some.inj checked; exact .releaseDepth guard.1 guard.2
    · cases checked
  | finish =>
    simp only [advance] at checked
    split at checked
    · rename_i yes
      have guard : s.phase = .depth ∧ s.memory.depth ≤ baseline := by simpa using yes
      cases Option.some.inj checked; exact .finish guard.1 guard.2
    · cases checked

theorem advance_exact : advance baseline s action = some next ↔ Step baseline s action next :=
  ⟨advance_sound,advance_complete⟩

def Invariant (baseline : Nat) (parentGate : Bool) (s : State) : Prop :=
  match s.phase with
  | .body | .gate => EntryAcquisition.Invariant baseline parentGate s.memory
  | .depth => baseline ≤ s.memory.depth ∧ s.memory.gate = parentGate
  | .done => s.memory.depth = baseline ∧ s.memory.gate = parentGate ∧ s.memory.stage = .closed

theorem release_gate_exact (valid : EntryAcquisition.Invariant baseline parentGate memory)
    (owned : ownGate baseline memory = true) : parentGate = false ∧ baseline ≤ memory.depth := by
  have guard := ownGate_exact.mp owned
  cases stage : memory.stage <;> simp only [EntryAcquisition.Invariant,stage] at valid
  case fresh => omega
  case locked => rw [valid.2.2] at guard; simp at guard
  case noted => exact ⟨Option.some.inj (valid.2.2.symm.trans guard.2.1),by omega⟩
  case holding => exact ⟨valid.2.1,by omega⟩
  case closed => omega

theorem skip_gate_exact (valid : EntryAcquisition.Invariant baseline parentGate memory)
    (skip : ownGate baseline memory = false) : baseline ≤ memory.depth ∧ memory.gate = parentGate := by
  cases stage : memory.stage <;> simp only [EntryAcquisition.Invariant,stage] at valid
  case fresh => exact ⟨by omega,valid.2.1⟩
  case locked => exact ⟨by omega,valid.2.1⟩
  case noted => exact ⟨by omega,valid.2.1⟩
  case holding => simp [ownGate,valid.1,valid.2.2.1,valid.2.2.2] at skip
  case closed => exact ⟨by omega,valid.2⟩

theorem preserves (valid : Invariant baseline parentGate s) (step : Step baseline s action next) :
    Invariant baseline parentGate next := by
  cases step with
  | begin phase => simpa [Invariant,phase] using valid
  | releaseGate phase owned snapshot locked =>
    have before : EntryAcquisition.Invariant baseline parentGate s.memory := by simpa [Invariant,phase] using valid
    have facts := release_gate_exact before (ownGate_exact.mpr ⟨owned,snapshot,locked⟩)
    exact ⟨facts.2,facts.1.symm⟩
  | skipGate phase skip =>
    exact skip_gate_exact (by simpa [Invariant,phase] using valid) skip
  | releaseDepth phase owned =>
    have before : baseline ≤ s.memory.depth ∧ s.memory.gate = parentGate := by simpa [Invariant,phase] using valid
    exact ⟨by change baseline ≤ s.memory.depth-1; omega,before.2⟩
  | finish phase done =>
    have before : baseline ≤ s.memory.depth ∧ s.memory.gate = parentGate := by simpa [Invariant,phase] using valid
    exact ⟨by change s.memory.depth = baseline; omega,before.2,rfl⟩

theorem retired_frames (step : Step baseline s action next) : next.memory.retired = s.memory.retired := by
  cases step <;> rfl

def bodyEnabled (s : State) : Bool := (decide (s.phase = .body)) && EntryAcquisition.admitted s.memory

theorem no_resume (step : Step baseline s action next) : next.phase ≠ .body := by
  cases step <;> simp

theorem no_cleanup_body (step : Step baseline s action next) : bodyEnabled next = false := by
  simp [bodyEnabled,no_resume step]

#print axioms advance_exact
#print axioms preserves
#print axioms retired_frames
#print axioms no_cleanup_body

inductive Runs (baseline : Nat) (first : State) : State → Prop where
  | nil : Runs baseline first first
  | step : Runs baseline first s → Step baseline s action next → Runs baseline first next

theorem Runs.trans (left : Runs baseline first middle) (right : Runs baseline middle last) :
    Runs baseline first last := by
  induction right with
  | nil => exact left
  | step prior step ih => exact .step ih step

theorem runs_preserves (valid : Invariant baseline parentGate first) (path : Runs baseline first last) :
    Invariant baseline parentGate last := by
  induction path with
  | nil => exact valid
  | step prior step ih => exact preserves ih step

-- Every actual recursive release is represented; only the final PC transition
-- sets the terminal stage. No atomic reset to baseline is added to Step.
theorem drain_execution (memory : Memory) (bounded : baseline ≤ memory.depth) :
    Runs baseline ⟨memory,.depth⟩ ⟨{memory with depth:=baseline,stage:=.closed},.done⟩ := by
  generalize atDepth : memory.depth = depth at bounded
  induction depth generalizing memory with
  | zero =>
    have zero : baseline = 0 := by omega
    subst baseline
    have same : {memory with depth:=0,stage:=.closed} = {memory with stage:=.closed} := by
      cases memory; simp_all
    rw [same]
    exact .step .nil (.finish rfl (by change memory.depth ≤ _; omega))
  | succ depth ih =>
    by_cases owned : baseline < memory.depth
    · have smaller : baseline ≤ depth := by omega
      have step : Step baseline ⟨memory,.depth⟩ .releaseDepth
          ⟨{memory with depth:=depth},.depth⟩ := by
        simpa only [atDepth,Nat.add_sub_cancel] using
          (Step.releaseDepth (s:=⟨memory,.depth⟩) rfl owned)
      have tail := ih {memory with depth:=depth} rfl smaller
      exact (Runs.step Runs.nil step).trans tail
    · have equal : memory.depth = baseline := by omega
      have same : {memory with depth:=baseline,stage:=.closed} = {memory with stage:=.closed} := by
        cases memory; simp_all
      rw [same]
      exact .step .nil (.finish rfl (by change memory.depth ≤ _; omega))

def gateResult (baseline : Nat) (memory : Memory) : Memory :=
  if ownGate baseline memory then {memory with gate:=false} else memory

theorem gate_execution (memory : Memory) :
    Runs baseline ⟨memory,.body⟩ ⟨gateResult baseline memory,.depth⟩ := by
  have begin : Step baseline ⟨memory,.body⟩ .begin ⟨memory,.gate⟩ := .begin rfl
  by_cases owned : ownGate baseline memory = true
  · obtain ⟨depth,snapshot,locked⟩ := ownGate_exact.mp owned
    have released : Step baseline ⟨memory,.gate⟩ .releaseGate ⟨{memory with gate:=false},.depth⟩ :=
      .releaseGate rfl depth snapshot locked
    simpa [gateResult,owned] using (Runs.step Runs.nil begin).step released
  · have skipped : ownGate baseline memory = false := by cases flag : ownGate baseline memory <;> simp_all
    simpa [gateResult,skipped] using
      (Runs.step Runs.nil begin).step (Step.skipGate rfl skipped)

theorem cleanup_execution (valid : EntryAcquisition.Invariant baseline parentGate memory) :
    Runs baseline ⟨memory,.body⟩ ⟨EntryAcquisition.cleanup baseline memory,.done⟩ := by
  have gatePath := gate_execution (baseline:=baseline) memory
  have afterGate := runs_preserves (show Invariant baseline parentGate ⟨memory,.body⟩ from valid) gatePath
  have tail := drain_execution (gateResult baseline memory) afterGate.1
  have path := gatePath.trans tail
  have equal : {gateResult baseline memory with depth:=baseline,stage:=.closed} = EntryAcquisition.cleanup baseline memory := by
    have fields (x y : Memory) (depth : x.depth = y.depth) (gate : x.gate = y.gate)
        (snapshot : x.snapshot = y.snapshot) (stage : x.stage = y.stage) (retired : x.retired = y.retired) : x = y := by
      cases x; cases y; simp_all
    have endpoint := EntryAcquisition.cleanup_restores valid
    have snapshotSame : (gateResult baseline memory).snapshot = memory.snapshot := by
      unfold gateResult; split <;> rfl
    have retiredSame : (gateResult baseline memory).retired = memory.retired := by
      unfold gateResult; split <;> rfl
    have cleanupSnapshot : (EntryAcquisition.cleanup baseline memory).snapshot = memory.snapshot := by
      unfold EntryAcquisition.cleanup; split <;> rfl
    have cleanupStage : (EntryAcquisition.cleanup baseline memory).stage = .closed := by
      unfold EntryAcquisition.cleanup; split <;> rfl
    exact fields _ _ endpoint.1.symm (afterGate.2.trans endpoint.2.1.symm)
      (snapshotSame.trans cleanupSnapshot.symm) cleanupStage.symm (retiredSame.trans endpoint.2.2.symm)
  simpa only [equal] using path

#print axioms drain_execution
#print axioms gate_execution
#print axioms cleanup_execution
end Restoration
end MirroreaProofFirst.EntryAcquisition
