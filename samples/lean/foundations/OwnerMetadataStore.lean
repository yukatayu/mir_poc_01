import OwnerMetadataManagement
import MixedReferenceMutation
import MixedReferenceAllocation
import MixedReferenceOrigins
namespace MirroreaProofFirst.OwnerMetadataStore
open OwnerMetadataRegistry
namespace M
abbrev State := OwnerMetadataManagement.State
end M

def state (m : MixedReferenceStore.Machine p a) (registry : Registry) : M.State p a :=
 ⟨m.core.system,registry⟩

-- Same actual core, pending list, reference bindings and history as ordinary
-- operations. The new typed occurrence is emitted ONLY on admitted mutation.
-- The registry is an enclosing retained component, not a second runtime.
def record (m : MixedReferenceStore.Machine p a) (principal requestId : Nat)
 (context : OwnerMetadataManagement.Context) : MixedReferenceStore.Machine p a :=
 { m with
   core := MixedReferenceMutation.postCore m principal requestId
   history := m.history ++ [MixedReferenceStore.frame (MixedReferenceMutation.postCore m principal requestId)]
   events := .metadata context :: m.events }

def commit (m : MixedReferenceStore.Machine p a) (registry : Registry) (member : Fin a) (place : Fin p)
 (principal requestId : Nat) (change : Change) (evidence : OwnerMetadataManagement.Evidence) :
 Option (MixedReferenceStore.Machine p a × Registry) := do
 if !MixedRequestCore.fresh m.core (MixedManagementEntry.useId m.core.system principal requestId) then none else do
 let next ← OwnerMetadataManagement.commit (state m registry) member place principal requestId change evidence
 return (record m principal requestId (OwnerMetadataManagement.context (state m registry) member place principal requestId change),next.metadata)

theorem commit_parts (accepted : commit m registry member place principal requestId change evidence = some result) :
 MixedRequestCore.fresh m.core (MixedManagementEntry.useId m.core.system principal requestId) = true ∧
 ∃ next, OwnerMetadataManagement.commit (state m registry) member place principal requestId change evidence = some next ∧
 result = (record m principal requestId (OwnerMetadataManagement.context (state m registry) member place principal requestId change),next.metadata) := by
 unfold commit at accepted
 split at accepted
 · cases accepted
 · rename_i gate
   have fresh : MixedRequestCore.fresh m.core (MixedManagementEntry.useId m.core.system principal requestId) = true := by simpa using gate
   cases done : OwnerMetadataManagement.commit (state m registry) member place principal requestId change evidence with
   | none => simp [done] at accepted
   | some next =>
     simp only [done,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
     exact ⟨fresh,next,rfl,accepted.symm⟩

-- Existing ID update is definitionally the exact admitted management result.
theorem commit_system (accepted : commit m registry member place principal requestId change evidence = some result) :
 ∃ next, OwnerMetadataManagement.commit (state m registry) member place principal requestId change evidence = some next ∧
 result.1.core.system = next.management ∧ result.2 = next.metadata := by
 obtain ⟨_,next,done,rfl⟩ := commit_parts accepted
 obtain ⟨_,_,_,slot,_,equal⟩ := OwnerMetadataManagement.commit_parts done
 subst next
 exact ⟨_,done,rfl,rfl⟩

theorem commit_complete
 (fresh : MixedRequestCore.fresh m.core (MixedManagementEntry.useId m.core.system principal requestId) = true)
 (allowed : OwnerMetadataManagement.Allowed (state m registry) member place principal requestId change)
 (rule : Prepares registry change next) :
 ∃ evidence result, commit m registry member place principal requestId change evidence = some result := by
 obtain ⟨evidence,done⟩ := OwnerMetadataManagement.commit_complete
   (MixedRequestCore.fresh_exact.mp fresh).1 allowed rule
 exact ⟨evidence,(record m principal requestId (OwnerMetadataManagement.context (state m registry) member place principal requestId change),
  (OwnerMetadataManagement.after (state m registry) principal requestId change next).metadata),by simp [commit,fresh,done]⟩

theorem record_preserves {context : OwnerMetadataManagement.Context} (valid : MixedReferenceCoherence.Invariant m)
 (fresh : MixedRequestCore.fresh m.core (MixedManagementEntry.useId m.core.system principal requestId) = true) :
 MixedReferenceCoherence.Invariant (record m principal requestId context) := by
 refine ⟨⟨MixedReferenceMutation.postCore_invariant _ _ _ valid.1.1 fresh,
   valid.1.2.1,valid.1.2.2.1,⟨m.history,rfl⟩,?_⟩,?_⟩
 · simpa [record,MixedReferenceStore.coreEvents,MixedReferenceMutation.postCore] using valid.1.2.2.2.2
 · intro key binding atKey
   exact MixedReferenceCoherence.stored_extend _ _ _ _ _ _ (valid.2 key binding atKey) (by simp [record])

theorem commit_preserves (valid : MixedReferenceCoherence.Invariant m)
 (metadata : Consistent registry)
 (accepted : commit m registry member place principal requestId change evidence = some result) :
 MixedReferenceCoherence.Invariant result.1 ∧ Consistent result.2 := by
 obtain ⟨fresh,next,done,rfl⟩ := commit_parts accepted
 refine ⟨record_preserves valid fresh,?_⟩
 obtain ⟨_,_,_,slot,rule,equal⟩ := OwnerMetadataManagement.commit_parts done
 subst next
 exact OwnerMetadataManagement.prepared_update_consistent metadata rule

theorem commit_origins (valid : MixedReferenceOrigins.Invariant m)
 (accepted : commit m registry member place principal requestId change evidence = some result) :
 MixedReferenceOrigins.Invariant result.1 := by
 obtain ⟨_,_,_,rfl⟩ := commit_parts accepted
 exact MixedReferenceOrigins.same_rows m _ _ rfl rfl valid

theorem commit_clock (count : m.history.length = m.events.length+1)
 (accepted : commit m registry member place principal requestId change evidence = some result) :
 result.1.history.length = result.1.events.length+1 := by
 obtain ⟨_,_,_,rfl⟩ := commit_parts accepted
 simp only [record,List.length_append,List.length_cons,List.length_nil]
 omega

theorem commit_below (bound : MixedRequestAllocation.Below m.core requestId)
 (accepted : commit m registry member place principal requestId change evidence = some result) :
 MixedRequestAllocation.Below result.1.core (requestId+1) := by
 obtain ⟨_,_,_,rfl⟩ := commit_parts accepted
 exact MixedReferenceAllocation.postCore_below _ _ _ bound

theorem pending_refused
 (pending : MixedManagementEntry.useId m.core.system principal requestId ∈ m.core.pending.map MixedRequestCore.pendingId) :
 commit m registry member place principal requestId change evidence = none := by
 simp [commit,MixedRequestCore.fresh,pending]

theorem commit_no_repeat (accepted : commit m registry member place principal requestId change evidence = some result) :
 commit result.1 result.2 otherMember otherPlace principal requestId other otherEvidence = none := by
 obtain ⟨_,_,_,rfl⟩ := commit_parts accepted
 simp [commit,MixedRequestCore.fresh,record,MixedReferenceMutation.postCore,MixedManagementEntry.useId]

theorem commit_event (accepted : commit m registry member place principal requestId change evidence = some result) :
 result.1.events = .metadata (OwnerMetadataManagement.context (state m registry) member place principal requestId change) :: m.events ∧
 evidence.context = OwnerMetadataManagement.context (state m registry) member place principal requestId change := by
 obtain ⟨_,next,done,rfl⟩ := commit_parts accepted
 exact ⟨rfl,(OwnerMetadataManagement.commit_parts done).2.2.1⟩

-- Existing execution retains its actual pending classification; this mutation
-- cannot erase a queued pure/owner request or manufacture its result.
def execute (m : MixedReferenceExecution.Machine p a) (registry : Registry)
 (member : Fin a) (place : Fin p) (principal requestId : Nat) (change : Change) :
 Option (MixedReferenceExecution.Machine p a × Registry) := do
 let evidence ← OwnerMetadataManagement.authorize (state m.store registry) member place principal requestId change
 let (store,next) ← commit m.store registry member place principal requestId change evidence
 return ({m with store := store},next)

theorem execute_parts (accepted : execute m registry member place principal requestId change = some result) :
 ∃ evidence store next, commit m.store registry member place principal requestId change evidence = some (store,next) ∧
 result = ({m with store := store},next) := by
 unfold execute at accepted
 cases issued : OwnerMetadataManagement.authorize (state m.store registry) member place principal requestId change with
 | none => simp [issued] at accepted
 | some evidence =>
   simp only [issued,Option.bind_eq_bind,Option.bind_some] at accepted
   cases done : commit m.store registry member place principal requestId change evidence with
   | none => simp [done] at accepted
   | some pair =>
     obtain ⟨store,next⟩ := pair
     simp only [done,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
     exact ⟨evidence,store,next,done,accepted.symm⟩

theorem execute_preserves (valid : MixedReferenceExecution.Invariant m) (metadata : Consistent registry)
 (accepted : execute m registry member place principal requestId change = some result) :
 MixedReferenceExecution.Invariant result.1 ∧ Consistent result.2 := by
 obtain ⟨evidence,store,next,done,rfl⟩ := execute_parts accepted
 have preserved := commit_preserves valid.1 metadata done
 refine ⟨⟨preserved.1,?_⟩,preserved.2⟩
 obtain ⟨_,_,_,equal⟩ := commit_parts done
 have stores : store = record m.store principal requestId (OwnerMetadataManagement.context (state m.store registry) member place principal requestId change) := congrArg Prod.fst equal
 change _ = MixedPendingProjection.tickets store.core.pending
 rw [stores]
 exact valid.2

theorem execute_below (bound : MixedReferenceAllocation.Below m requestId)
 (accepted : execute m registry member place principal requestId change = some result) :
 MixedReferenceAllocation.Below result.1 (requestId+1) := by
 obtain ⟨_,_,_,done,rfl⟩ := execute_parts accepted
 exact commit_below bound done

theorem execute_pending (accepted : execute m registry member place principal requestId change = some result) :
 result.1.pending = m.pending ∧ result.1.store.core.pending = m.store.core.pending := by
 obtain ⟨_,_,_,done,rfl⟩ := execute_parts accepted
 obtain ⟨_,_,_,equal⟩ := commit_parts done
 have core := congrArg (fun pair => pair.1.core.pending) equal
 exact ⟨rfl,core⟩

theorem execute_complete
 (fresh : MixedRequestCore.fresh m.store.core (MixedManagementEntry.useId m.store.core.system principal requestId) = true)
 (allowed : OwnerMetadataManagement.Allowed (state m.store registry) member place principal requestId change)
 (rule : Prepares registry change next) :
 ∃ result, execute m registry member place principal requestId change = some result := by
 obtain ⟨evidence,issued⟩ := OwnerMetadataManagement.authorize_complete allowed
 have checked := OwnerMetadataManagement.authorize_sound issued
 have unused : OwnerMetadataManagement.useId (state m.store registry) principal requestId ∉ m.store.core.system.used :=
   (MixedRequestCore.fresh_exact.mp fresh).1
 have prepared : prepare (state m.store registry).metadata change = some next := prepare_exact.mpr rule
 have admitted : OwnerMetadataManagement.commit (state m.store registry) member place principal requestId change evidence =
   some (OwnerMetadataManagement.after (state m.store registry) principal requestId change next) := by
   simp [OwnerMetadataManagement.commit,checked,prepared]
   exact unused
 let nextStore := record m.store principal requestId
   (OwnerMetadataManagement.context (state m.store registry) member place principal requestId change)
 let nextMachine : MixedReferenceExecution.Machine _ _ := {m with store := nextStore}
 refine ⟨(nextMachine,(OwnerMetadataManagement.after (state m.store registry) principal requestId change next).metadata),?_⟩
 simp [execute,issued,commit,fresh,admitted,nextMachine,nextStore]

#print axioms execute_below
#print axioms execute_pending
#print axioms execute_complete

#print axioms commit_parts
#print axioms commit_system
#print axioms commit_complete
#print axioms commit_preserves
#print axioms commit_origins
#print axioms commit_clock
#print axioms commit_below
#print axioms pending_refused
#print axioms commit_no_repeat
#print axioms commit_event
#print axioms execute_preserves
end MirroreaProofFirst.OwnerMetadataStore
