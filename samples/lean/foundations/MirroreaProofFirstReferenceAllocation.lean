import MirroreaProofFirstReferenceExecution
import MirroreaProofFirstSourceAllocation

namespace MirroreaProofFirst.ReferenceAllocation

-- Reuse the existing pure core request-bound predicate, without executing the
-- old source adapter. A single internal allocator bounds BOTH consumed and
-- outstanding ids. Finishing a request does not allocate a second request id.
abbrev Below (m : ReferenceExecution.Machine p a) (bound : Nat) : Prop := SourceAllocation.Below m.store.core bound

theorem postCore_below (m : ReferenceStore.Machine p a) (principal id : Nat) (valid : SourceAllocation.Below m.core id) :
    SourceAllocation.Below (ReferenceMutation.postCore m principal id) (id+1) := by
  constructor
  · intro used member
    rcases List.mem_cons.mp member with rfl | old
    · exact Nat.lt_succ_self _
    · exact Nat.lt_trans (valid.1 used old) (Nat.lt_succ_self id)
  · intro ticket member
    exact Nat.lt_trans (valid.2 ticket member) (Nat.lt_succ_self id)

theorem manage_below (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : CompositionCore.Raw) (result : ReferenceExecution.Machine p a × Option Nat)
    (valid : Below m id) (accepted : ReferenceExecution.manage m member place principal id raw = some result) :
    Below result.1 (id+1) :=
  SourceAllocation.manage_below _ _ _ _ _ _ _ valid (ReferenceExecution.manage_core _ _ _ _ _ _ _ accepted)

theorem startPlain_below (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : ReferenceExecution.Machine p a × ReferenceExecution.Pending)
    (valid : Below m id) (accepted : ReferenceExecution.startPlain m member place principal id key argument = some result) :
    Below result.1 (id+1) :=
  SourceAllocation.start_below _ _ _ _ _ _ _ _ valid (ReferenceExecution.startPlain_core _ _ _ _ _ _ _ _ accepted)

theorem startReference_below (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : ReferenceExecution.Machine p a × ReferenceExecution.Pending)
    (valid : Below m id) (accepted : ReferenceExecution.startReference m member place principal id key argument = some result) :
    Below result.1 (id+1) := by
  obtain ⟨target,coreRun⟩ := ReferenceExecution.startReference_core _ _ _ _ _ _ _ _ accepted
  exact SourceAllocation.start_below _ _ _ _ _ _ _ _ valid coreRun

theorem finish_below (m : ReferenceExecution.Machine p a) (entry : ReferenceExecution.Pending) (value : Int)
    (result : ReferenceExecution.Machine p a) (bound : Nat) (valid : Below m bound)
    (accepted : ReferenceExecution.finish m entry value = some result) : Below result bound :=
  SourceAllocation.finish_below _ _ _ _ _ valid (ReferenceExecution.finish_core_run _ _ _ _ accepted)

theorem acquire_below (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : ReferenceExecution.Machine p a × Nat)
    (valid : Below m id) (accepted : ReferenceExecution.acquire m member place principal id chain = some result) :
    Below result.1 (id+1) := by
  unfold Below
  rw [ReferenceExecution.acquire_core _ _ _ _ _ _ _ accepted]
  exact postCore_below _ _ _ valid

theorem reacquire_below (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : ReferenceExecution.Machine p a) (valid : Below m id)
    (accepted : ReferenceExecution.reacquire m member place principal id key = some result) : Below result (id+1) := by
  unfold Below
  rw [ReferenceExecution.reacquire_core _ _ _ _ _ _ _ accepted]
  exact postCore_below _ _ _ valid

theorem release_below (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : ReferenceExecution.Machine p a) (valid : Below m id)
    (accepted : ReferenceExecution.release m member place principal id key = some result) : Below result (id+1) := by
  unfold Below
  rw [ReferenceExecution.release_core _ _ _ _ _ _ _ accepted]
  exact postCore_below _ _ _ valid

theorem normalize_consumed_below (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (valid : Below m id) (consumed : (ReferenceExecution.normalize m member place principal id key).consumed = true) :
    Below (ReferenceExecution.normalize m member place principal id key).state (id+1) := by
  unfold Below
  rw [ReferenceExecution.normalize_consumed_core _ _ _ _ _ _ consumed]
  exact postCore_below _ _ _ valid

theorem head_below (m : ReferenceExecution.Machine p a) (view : WorldProjection.AuthorityView a) (bound : Nat)
    (valid : Below m bound) : Below (ReferenceExecution.authorityHead m view) bound := valid

theorem initial_below (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) :
    Below (ReferenceExecution.initial (p:=p) realm view policy) 0 := by
  constructor <;> intro item member <;> cases member

theorem resume_below (m : ReferenceExecution.Machine p a) (entry : ReferenceExecution.Pending)
    (result : ReferenceExecution.Machine p a × Int) (bound : Nat) (valid : Below m bound)
    (accepted : ReferenceExecution.resume m entry = some result) : Below result.1 bound := by
  unfold ReferenceExecution.resume at accepted
  cases evaluated : InvocationBoundary.execute entry.ticket with
  | none => simp [evaluated] at accepted
  | some value =>
      simp only [evaluated,Option.bind_eq_bind,Option.bind_some] at accepted
      cases completed : ReferenceExecution.finish m entry value with
      | none => simp [completed] at accepted
      | some next =>
          simp only [completed,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
          subst result
          exact finish_below _ _ _ _ _ valid completed

theorem fresh_bound (m : ReferenceExecution.Machine p a) (bound realm principal : Nat) (valid : Below m bound) :
    CompositionMachine.fresh m.store.core ⟨realm,principal,bound⟩ = true := by
  apply (CompositionMachine.fresh_exact _ _).mpr
  constructor
  · intro member
    exact Nat.lt_irrefl bound (valid.1 _ member)
  · intro member
    obtain ⟨ticket,atPending,same⟩ := List.mem_map.mp member
    have idEq := congrArg CurrentUse.UseId.request same
    have less := valid.2 ticket atPending
    change ticket.id = bound at idEq
    omega

theorem cancel_below (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry : ReferenceExecution.Pending) (result : ReferenceExecution.Machine p a) (valid : Below m id)
    (accepted : ReferenceExecution.cancel m member place principal id entry = some result) : Below result (id+1) := by
  obtain ⟨permit,run⟩ := ReferenceExecution.cancel_core _ _ _ _ _ _ _ accepted
  obtain ⟨pending,_,_,_,equal⟩ := CompositionMachine.cancel_parts _ _ _ _ _ _ _ _ run
  unfold Below SourceAllocation.Below
  rw [equal]
  constructor
  · intro used atUsed
    rcases List.mem_cons.mp atUsed with rfl | old
    · exact Nat.lt_trans (valid.2 entry.ticket pending) (Nat.lt_succ_self id)
    · rcases List.mem_cons.mp old with rfl | earlier
      · exact Nat.lt_succ_self id
      · exact Nat.lt_trans (valid.1 used earlier) (Nat.lt_succ_self id)
  · intro ticket atPending
    exact Nat.lt_trans (valid.2 ticket (List.mem_filter.mp atPending).1) (Nat.lt_succ_self id)

#print axioms cancel_below
#print axioms postCore_below
#print axioms manage_below
#print axioms startPlain_below
#print axioms startReference_below
#print axioms finish_below
#print axioms acquire_below
#print axioms reacquire_below
#print axioms release_below
#print axioms normalize_consumed_below
#print axioms head_below
#print axioms initial_below
#print axioms resume_below
#print axioms fresh_bound
end MirroreaProofFirst.ReferenceAllocation
