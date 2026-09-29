import MixedReferenceExecution
import MixedRequestAllocation

namespace MirroreaProofFirst.MixedReferenceAllocation

-- A single bound covers consumed IDs and every pending kind. The protected
-- classification is not the allocator source of truth.
abbrev Below (m : MixedReferenceExecution.Machine p a) (bound : Nat) : Prop := MixedRequestAllocation.Below m.store.core bound

theorem postCore_below (m : MixedReferenceStore.Machine p a) (principal id : Nat) (valid : MixedRequestAllocation.Below m.core id) :
    MixedRequestAllocation.Below (MixedReferenceMutation.postCore m principal id) (id+1) := by
  constructor
  · intro used member
    rcases List.mem_cons.mp member with rfl | old
    · exact Nat.lt_succ_self _
    · exact Nat.lt_trans (valid.1 used old) (Nat.lt_succ_self id)
  · intro ticket member
    exact Nat.lt_trans (valid.2 ticket member) (Nat.lt_succ_self id)

theorem manage_below (m : MixedReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : MixedCompositionCore.Raw) (result : MixedReferenceExecution.Machine p a × Option Nat)
    (valid : Below m id) (accepted : MixedReferenceExecution.manage m member place principal id raw = some result) :
    Below result.1 (id+1) :=
  MixedRequestAllocation.manage_below _ _ _ _ _ _ _ valid (MixedReferenceExecution.manage_core _ _ _ _ _ _ _ accepted)

theorem startPlain_below (m : MixedReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : MixedReferenceExecution.Machine p a × ReferenceExecution.Pending)
    (valid : Below m id) (accepted : MixedReferenceExecution.startPlain m member place principal id key argument = some result) :
    Below result.1 (id+1) :=
  MixedRequestAllocation.start_below _ _ _ _ _ _ _ _ valid (MixedReferenceExecution.startPlain_core _ _ _ _ _ _ _ _ accepted)

theorem startReference_below (m : MixedReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : MixedReferenceExecution.Machine p a × ReferenceExecution.Pending)
    (valid : Below m id) (accepted : MixedReferenceExecution.startReference m member place principal id key argument = some result) :
    Below result.1 (id+1) := by
  obtain ⟨target,coreRun⟩ := MixedReferenceExecution.startReference_core _ _ _ _ _ _ _ _ accepted
  exact MixedRequestAllocation.start_below _ _ _ _ _ _ _ _ valid coreRun

theorem finish_below (m : MixedReferenceExecution.Machine p a) (entry : ReferenceExecution.Pending) (value : Int)
    (result : MixedReferenceExecution.Machine p a) (bound : Nat) (valid : Below m bound)
    (accepted : MixedReferenceExecution.finish m entry value = some result) : Below result bound :=
  MixedRequestAllocation.finish_below _ _ _ _ _ valid (MixedReferenceExecution.finish_core_run _ _ _ _ accepted)

theorem acquire_below (m : MixedReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : MixedReferenceExecution.Machine p a × Nat)
    (valid : Below m id) (accepted : MixedReferenceExecution.acquire m member place principal id chain = some result) :
    Below result.1 (id+1) := by
  unfold Below
  rw [MixedReferenceExecution.acquire_core _ _ _ _ _ _ _ accepted]
  exact postCore_below _ _ _ valid

theorem reacquire_below (m : MixedReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : MixedReferenceExecution.Machine p a) (valid : Below m id)
    (accepted : MixedReferenceExecution.reacquire m member place principal id key = some result) : Below result (id+1) := by
  unfold Below
  rw [MixedReferenceExecution.reacquire_core _ _ _ _ _ _ _ accepted]
  exact postCore_below _ _ _ valid

theorem release_below (m : MixedReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : MixedReferenceExecution.Machine p a) (valid : Below m id)
    (accepted : MixedReferenceExecution.release m member place principal id key = some result) : Below result (id+1) := by
  unfold Below
  rw [MixedReferenceExecution.release_core _ _ _ _ _ _ _ accepted]
  exact postCore_below _ _ _ valid

theorem normalize_consumed_below (m : MixedReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (valid : Below m id) (consumed : (MixedReferenceExecution.normalize m member place principal id key).consumed = true) :
    Below (MixedReferenceExecution.normalize m member place principal id key).state (id+1) := by
  unfold Below
  rw [MixedReferenceExecution.normalize_consumed_core _ _ _ _ _ _ consumed]
  exact postCore_below _ _ _ valid

theorem head_below (m : MixedReferenceExecution.Machine p a) (view : WorldProjection.AuthorityView a) (bound : Nat)
    (valid : Below m bound) : Below (MixedReferenceExecution.authorityHead m view) bound := valid

theorem initial_below (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) :
    Below (MixedReferenceExecution.initial (p:=p) realm view policy) 0 := by
  constructor <;> intro item member <;> cases member

theorem resume_below (m : MixedReferenceExecution.Machine p a) (entry : ReferenceExecution.Pending)
    (result : MixedReferenceExecution.Machine p a × Int) (bound : Nat) (valid : Below m bound)
    (accepted : MixedReferenceExecution.resume m entry = some result) : Below result.1 bound := by
  unfold MixedReferenceExecution.resume at accepted
  cases evaluated : InvocationBoundary.execute entry.ticket with
  | none => simp [evaluated] at accepted
  | some value =>
      simp only [evaluated,Option.bind_eq_bind,Option.bind_some] at accepted
      cases completed : MixedReferenceExecution.finish m entry value with
      | none => simp [completed] at accepted
      | some next =>
          simp only [completed,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
          subst result
          exact finish_below _ _ _ _ _ valid completed

theorem fresh_bound (m : MixedReferenceExecution.Machine p a) (bound realm principal : Nat) (valid : Below m bound) :
    MixedRequestCore.fresh m.store.core ⟨realm,principal,bound⟩ = true := by
  apply MixedRequestCore.fresh_exact.mpr
  constructor
  · intro member
    exact Nat.lt_irrefl bound (valid.1 _ member)
  · intro member
    obtain ⟨ticket,atPending,same⟩ := List.mem_map.mp member
    have idEq := congrArg CurrentUse.UseId.request same
    have less := valid.2 ticket atPending
    change (MixedRequestCore.pendingId ticket).request = bound at idEq
    omega

theorem cancel_below (m : MixedReferenceExecution.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry : ReferenceExecution.Pending) (result : MixedReferenceExecution.Machine p a) (valid : Below m id)
    (accepted : MixedReferenceExecution.cancel m member place principal id entry = some result) : Below result (id+1) := by
  obtain ⟨permit,run⟩ := MixedReferenceExecution.cancel_core _ _ _ _ _ _ _ accepted
  obtain ⟨pending,_,_,_,equal⟩ := MixedCancelEntry.cancel_parts _ _ _ _ _ _ _ _ run
  unfold Below MixedRequestAllocation.Below
  rw [equal]
  constructor
  · intro used atUsed
    rcases List.mem_cons.mp atUsed with rfl | old
    · exact Nat.lt_trans (valid.2 (.pure entry.ticket) pending) (Nat.lt_succ_self id)
    · rcases List.mem_cons.mp old with rfl | earlier
      · exact Nat.lt_succ_self id
      · exact Nat.lt_trans (valid.1 used earlier) (Nat.lt_succ_self id)
  · intro ticket atPending
    exact Nat.lt_trans (valid.2 ticket (List.mem_filter.mp atPending).1) (Nat.lt_succ_self id)

theorem finishOwner_below (m : MixedReferenceExecution.Machine p a) (valid : Below m bound)
 (accepted : MixedReferenceExecution.finishOwner m saved received evidence = some next) : Below next bound := by
 obtain ⟨run,_⟩ := MixedReferenceExecution.finishOwner_parts _ accepted
 obtain ⟨core,coreRun,eq⟩ := MixedReferenceStore.finishOwner_parts _ run
 unfold Below
 rw [eq]
 exact MixedRequestAllocation.finishOwner_below _ valid coreRun
#print axioms finishOwner_below
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
end MirroreaProofFirst.MixedReferenceAllocation
