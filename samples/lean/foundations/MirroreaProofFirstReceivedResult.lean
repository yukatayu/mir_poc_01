import MirroreaProofFirstReferenceSession

namespace MirroreaProofFirst.ReceivedResult
open ReferenceSource ReferenceContinuation

-- A delivered value enters explicitly. This entry never obtains its value by
-- resuming the reference evaluator. Existing finish still verifies its value
-- and current protection; authenticated carrier provenance remains a separate
-- physical prerequisite, not authority manufactured by these mathematical data.
def accept (s : ReferenceSource.State p a) (entry : ReferenceExecution.Pending) (value : Int) :
    Option (ReferenceSource.State p a) := do
  let saved ← s.waiting
  if saved.entry != entry then none else do
    let machine ← ReferenceExecution.finish s.machine entry value
    let next := ReferenceSource.mark s machine (some saved.site) .result 0
    return {ReferenceSource.write next saved.site saved.machineBefore saved.inputs saved.name
      (.plain (.integer value false)) false with waiting := none}

theorem accept_parts {s next : ReferenceSource.State p a} (accepted : accept s entry value = some next) :
    ∃ saved machine, s.waiting = some saved ∧ saved.entry = entry ∧
      ReferenceExecution.finish s.machine entry value = some machine ∧
      next = {ReferenceSource.write
        (ReferenceSource.mark s machine (some saved.site) .result 0)
        saved.site saved.machineBefore saved.inputs saved.name (.plain (.integer value false)) false
        with waiting := none} := by
  unfold accept at accepted
  cases waiting : s.waiting with
  | none => simp [waiting] at accepted
  | some saved =>
      simp only [waiting,Option.bind_eq_bind,Option.bind_some] at accepted
      split at accepted
      · cases accepted
      · rename_i same
        have equal : saved.entry = entry := by simpa using same
        cases finished : ReferenceExecution.finish s.machine entry value with
        | none => simp [finished] at accepted
        | some machine =>
            simp only [finished,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
            exact ⟨saved,machine,rfl,equal,rfl,accepted.symm⟩

theorem accepted_value (accepted : accept s entry value = some next) :
    entry ∈ s.machine.pending ∧ ReferenceExecution.Protected s.machine.store entry ∧
      InvocationBoundary.ResultMeaning s.machine.store.core.system.configuration.state
        s.machine.store.core.system.view entry.ticket value := by
  obtain ⟨_,machine,_,_,finished,_⟩ := accept_parts accepted
  exact ⟨(ReferenceExecution.finish_parts _ _ _ _ finished).1,
    ReferenceExecution.finish_meaning _ _ _ _ finished⟩

theorem accept_sound (accepted : accept s entry value = some next) :
    ReferenceSource.complete s = ⟨next,.ready⟩ := by
  obtain ⟨saved,machine,waiting,equal,finished,rfl⟩ := accept_parts accepted
  have execution := (ReferenceExecution.finish_meaning _ _ _ _ finished).2.2
  have computed : InvocationBoundary.execute entry.ticket = some value :=
    (InstancePrograms.Machine.run_exact _ _ _).mpr execution
  simp [ReferenceSource.complete,waiting,ReferenceExecution.resume,equal,computed,finished]

theorem accept_complete (completed : ReferenceSource.complete s = ⟨next,.ready⟩) :
    ∃ entry value, accept s entry value = some next := by
  cases waiting : s.waiting with
  | none => simp [ReferenceSource.complete,waiting] at completed
  | some saved =>
      cases computed : InvocationBoundary.execute saved.entry.ticket with
      | none => simp [ReferenceSource.complete,waiting,ReferenceExecution.resume,computed] at completed
      | some value =>
          cases finished : ReferenceExecution.finish s.machine saved.entry value with
          | none => simp [ReferenceSource.complete,waiting,ReferenceExecution.resume,computed,finished] at completed
          | some machine =>
              simp only [ReferenceSource.complete,waiting,ReferenceExecution.resume,computed,
                Option.bind_eq_bind,Option.bind_some,Option.pure_def] at completed
              simp only [finished,Option.bind_some] at completed
              cases completed
              exact ⟨saved.entry,value,by simp [accept,waiting,finished]⟩

theorem accept_exact : (∃ entry value, accept s entry value = some next) ↔
    ReferenceSource.complete s = ⟨next,.ready⟩ :=
  ⟨fun ⟨_,_,accepted⟩ => accept_sound accepted,accept_complete⟩

theorem double_accept_rejected (accepted : accept s entry value = some next)
    (other : ReferenceExecution.Pending) (result : Int) : accept next other result = none := by
  obtain ⟨_,_,_,_,_,rfl⟩ := accept_parts accepted
  rfl

theorem wrong_entry_rejected (waiting : s.waiting = some saved) (different : saved.entry ≠ entry) :
    accept s entry value = none := by
  simp [accept,waiting,different]

-- A malformed/delayed reply returns none and does not fail the source program.
-- The transport must retain its actual typed rejection observation separately.
def arrive (s : Session p a) (entry : ReferenceExecution.Pending) (value : Int) : Option (Session p a) :=
  if s.status = .waiting then do
    let state ← accept s.state entry value
    return {s with
      state := state,status := .ready,completed := s.completed ++ s.stopped.toList,stopped := none}
  else none

theorem arrive_parts (accepted : arrive s entry value = some next) :
    s.status = .waiting ∧ ∃ state, accept s.state entry value = some state ∧
      next = {s with
        state := state,status := .ready,completed := s.completed ++ s.stopped.toList,stopped := none} := by
  unfold arrive at accepted
  split at accepted
  · rename_i waiting
    cases received : accept s.state entry value with
    | none => simp [received] at accepted
    | some state =>
        simp only [received,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
        exact ⟨waiting,state,rfl,accepted.symm⟩
  · cases accepted

theorem arrive_tick {s next : Session p a} (accepted : arrive s entry value = some next) (member : Fin a) (principal : Nat) :
    next = ReferenceContinuation.tick s member principal := by
  obtain ⟨waiting,state,received,rfl⟩ := arrive_parts accepted
  have completed := accept_sound received
  simp [ReferenceContinuation.tick,waiting,completed]

theorem arrive_rooted {s next : Session p a} (root : ReferenceSession.Rooted realm view policy s)
    (accepted : arrive s entry value = some next) (member : Fin a) (principal : Nat) :
    ReferenceSession.Rooted realm view policy next := by
  rw [arrive_tick accepted member principal]
  exact .step root .tick

-- The physical source scheduler may advance ready source only. A waiting
-- source is resumed solely through explicit arrival, not reference tick's
-- convenience branch that locally produces the result.
def advanceReady (s : Session p a) (member : Fin a) (principal : Nat) : Option (Session p a) :=
  if s.status = .ready then some (ReferenceContinuation.tick s member principal) else none

theorem waiting_cannot_advance (s : Session p a) (member : Fin a) (principal : Nat)
    (waiting : s.status = .waiting) : advanceReady s member principal = none := by
  simp [advanceReady,waiting]

#print axioms accepted_value
#print axioms accept_exact
#print axioms double_accept_rejected
#print axioms wrong_entry_rejected
#print axioms arrive_tick
#print axioms arrive_rooted
#print axioms waiting_cannot_advance
end MirroreaProofFirst.ReceivedResult
