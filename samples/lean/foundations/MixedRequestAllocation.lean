import MixedReferenceExecution
namespace MirroreaProofFirst.MixedRequestAllocation
open MixedRequestCore
-- A single allocator bounds ALL consumed IDs and both pending kinds. The bound
-- is sufficient for freshness across every realm/principal; it is not an ID wire format.
def Below (m : Machine p a) (bound : Nat) : Prop :=
 (∀ id ∈ m.system.used, id.request < bound) ∧
 (∀ entry ∈ m.pending, (pendingId entry).request < bound)

theorem manage_below (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
 (raw : MixedCompositionCore.Raw) (next : Machine p a × Option Nat) (valid : Below m id)
 (accepted : manage m member place principal id raw = some next) : Below next.1 (id+1) := by
 obtain ⟨_,_,cfg,created,_,_,rfl⟩ := manage_parts accepted
 constructor
 · intro request member
   rcases List.mem_cons.mp member with rfl | old
   · exact Nat.lt_succ_self id
   · exact Nat.lt_trans (valid.1 request old) (Nat.lt_succ_self id)
 · intro t member
   exact Nat.lt_trans (valid.2 t member) (Nat.lt_succ_self id)

theorem prepare_id (s : MixedInstanceState.State d p n) (v : WorldProjection.AuthorityView a)
 (member : Fin a) (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int)
 (t : InvocationBoundary.Ticket)
 (accepted : MixedPureInvocation.prepare s v member key place principal id arg = some t) : t.id = id := by
 unfold MixedPureInvocation.prepare at accepted
 cases kind : MixedCatalogService.operationDefinition s key with
 | owner defn => simp [kind] at accepted
 | pure defn =>
   simp only [kind] at accepted
   cases e : CurrentUse.authorize (MixedCatalogService.world s v).authority
     ((MixedCatalogService.world s v).policies (MixedPureInvocation.request s v member key place principal id arg).operation.key)
     (CurrentUse.currentContext (MixedCatalogService.world s v) (MixedPureInvocation.request s v member key place principal id arg)) with
   | none => simp [e] at accepted
   | some evidence =>
     simp only [e,Option.bind_eq_bind,Option.bind_some] at accepted
     split at accepted
     · cases accepted; rfl
     · cases accepted

theorem start_id (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) (arg : Int)
 (next : Machine p a × InvocationBoundary.Ticket)
 (accepted : startPure m member place principal id key arg = some next) : next.2.id = id := by
 unfold startPure at accepted
 cases hi : CompositionCore.index m.system.configuration.count key with
 | none => simp [hi] at accepted
 | some unit =>
   simp only [hi,Option.bind_eq_bind,Option.bind_some] at accepted
   cases ht : MixedPureInvocation.prepare m.system.configuration.state m.system.view member unit place principal id arg with
   | none => simp [ht] at accepted
   | some t =>
     simp only [ht,Option.bind_some] at accepted
     split at accepted
     · cases accepted; exact prepare_id _ _ _ _ _ _ _ _ _ ht
     · cases accepted

theorem enqueue_below (m : Machine p a) (bound : Nat) (entry : Pending)
 (valid : Below m bound) (id : (pendingId entry).request = bound) : Below (enqueue m entry) (bound+1) := by
 constructor
 · intro used atUsed
   exact Nat.lt_trans (valid.1 used atUsed) (Nat.lt_succ_self bound)
 · intro t atPending
   rcases List.mem_cons.mp atPending with rfl | old
   · rw [id]; exact Nat.lt_succ_self bound
   · exact Nat.lt_trans (valid.2 t old) (Nat.lt_succ_self bound)

theorem start_below (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) (arg : Int)
 (next : Machine p a × InvocationBoundary.Ticket) (valid : Below m id)
 (accepted : startPure m member place principal id key arg = some next) : Below next.1 (id+1) := by
 have exactId := start_id _ _ _ _ _ _ _ _ accepted
 obtain ⟨_,_,eq⟩ := startPure_parts accepted
 rw [eq]
 exact enqueue_below _ _ _ valid exactId

theorem startOwner_id (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
 (origin : OwnerEffectService.Origin) (args : List CurrentUse.Scalar) (next : Machine p a × OwnerSavedPending.Saved)
 (accepted : startOwner m member place principal id key origin args = some next) :
 pendingId (.owner next.2) = ⟨m.system.configuration.state.realm,principal,id⟩ := by
 unfold startOwner at accepted
 cases hi : CompositionCore.index m.system.configuration.count key with
 | none => simp [hi] at accepted
 | some unit =>
   simp only [hi,Option.bind_eq_bind,Option.bind_some] at accepted
   cases hp : prepareOwner m.system.configuration.state m.system.view member unit place principal id origin args with
   | none => simp [hp] at accepted
   | some saved =>
     simp only [hp,Option.bind_some] at accepted
     split at accepted
     · cases accepted; exact prepareOwner_id hp
     · cases accepted

theorem startOwner_below (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
 (origin : OwnerEffectService.Origin) (args : List CurrentUse.Scalar) (next : Machine p a × OwnerSavedPending.Saved)
 (valid : Below m id) (accepted : startOwner m member place principal id key origin args = some next) :
 Below next.1 (id+1) := by
 have exactId := congrArg CurrentUse.UseId.request (startOwner_id _ _ _ _ _ _ _ _ _ accepted)
 obtain ⟨_,eq⟩ := startOwner_parts accepted
 rw [eq]
 exact enqueue_below _ _ _ valid exactId

theorem finish_below (m : Machine p a) (t : InvocationBoundary.Ticket) (value : Int)
 (next : Machine p a) (bound : Nat) (valid : Below m bound)
 (accepted : finishPure m t value = some next) : Below next bound := by
 obtain ⟨pending,_,_,rfl⟩ := finishPure_parts accepted
 constructor
 · intro id member
   rcases List.mem_cons.mp member with rfl | old
   · exact valid.2 (.pure t) pending
   · exact valid.1 id old
 · intro ticket member
   exact valid.2 ticket (List.mem_filter.mp member).1

theorem finishOwner_below (m : Machine p a) (valid : Below m bound)
 (accepted : finishOwner m saved received evidence = some next) : Below next bound := by
 obtain ⟨present,_,_,_,_,_,_,rfl⟩ := finishOwner_parts accepted
 constructor
 · intro id member
   rcases List.mem_cons.mp member with rfl | old
   · exact valid.2 (.owner saved) present
   · exact valid.1 id old
 · intro entry member
   exact valid.2 entry (List.mem_filter.mp member).1
#print axioms finishOwner_below
#print axioms prepare_id
#print axioms startOwner_below
#print axioms finish_below
end MirroreaProofFirst.MixedRequestAllocation
