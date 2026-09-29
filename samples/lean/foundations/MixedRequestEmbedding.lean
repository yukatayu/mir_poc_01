import MixedRequestCore
namespace MirroreaProofFirst.MixedRequestEmbedding
open MixedRequestCore

def event : CompositionMachine.Occurrence → Occurrence
 | .management ctx created => .management (MixedManagementEmbedding.context ctx) created
 | .requested t => .requested (.pure t)
 | .result t value => .pureResult t value
 | .cancelled permit => .cancelled permit
 | .authorityHead generation => .authorityHead generation

def machine (m : CompositionMachine.Machine p a) : Machine p a :=
 ⟨MixedManagementEmbedding.system m.system,m.pending.map Pending.pure,m.events.map event⟩
def result (r : CompositionMachine.Machine p a × β) := (machine r.1,r.2)

theorem mapped_ids (ts : List InvocationBoundary.Ticket) :
 (ts.map Pending.pure).map pendingId = ts.map CompositionMachine.ticketId := by
 simp [List.map_map,pendingId]

theorem fresh_exact (m : CompositionMachine.Machine p a) (id : CurrentUse.UseId) :
 fresh (machine m) id = CompositionMachine.fresh m id := by
 simp only [fresh,CompositionMachine.fresh,machine,MixedManagementEmbedding.system,mapped_ids]

theorem enqueue_exact (m : CompositionMachine.Machine p a) (t : InvocationBoundary.Ticket) :
 enqueue (machine m) (.pure t) = machine (CompositionMachine.enqueue m t) := rfl

theorem manage_exact (m : CompositionMachine.Machine p a) (member : Fin a) (place : Fin p)
 (principal id : Nat) (raw : CompositionCore.Raw) :
 manage (machine m) member place principal id (MixedCoreEmbedding.raw raw) =
 (CompositionMachine.manage m member place principal id raw).map result := by
 simp only [manage,CompositionMachine.manage]
 have used : MixedManagementEntry.useId (machine m).system principal id = ManagementEntry.useId m.system principal id := rfl
 simp only [used,fresh_exact]
 split
 · rfl
 · have cfg : (machine m).system = MixedManagementEmbedding.system m.system := rfl
   simp only [cfg,MixedManagementEmbedding.perform_exact]
   cases h : ManagementEntry.perform m.system member place principal id raw with
   | none => rfl
   | some pair =>
     cases pair
     simp only [Option.map_some,Option.bind_eq_bind,Option.bind_some,
       MixedManagementEmbedding.result,MixedManagementEmbedding.current_exact]
     rfl

theorem start_exact (m : CompositionMachine.Machine p a) (member : Fin a) (place : Fin p)
 (principal id key : Nat) (arg : Int) :
 startPure (machine m) member place principal id key arg =
 (CompositionMachine.start m member place principal id key arg).map result := by
 unfold startPure CompositionMachine.start
 have sameIndex : CompositionCore.index (machine m).system.configuration.count key = CompositionCore.index m.system.configuration.count key := rfl
 rw [sameIndex]
 cases indexed : CompositionCore.index m.system.configuration.count key with
 | none => rfl
 | some unit =>
   simp only [Option.bind_eq_bind,Option.bind_some]
   have prepare : MixedPureInvocation.prepare (machine m).system.configuration.state (machine m).system.view member unit place principal id arg =
     InvocationBoundary.prepare m.system.configuration.state m.system.view member unit place principal id arg :=
       MixedPureInvocation.pure_prepare _ _ _ _ _ _ _ _
   dsimp only [Option.bind]
   rw [prepare]
   cases prepared : InvocationBoundary.prepare m.system.configuration.state m.system.view member unit place principal id arg with
   | none => rfl
   | some t =>
     simp only [pendingId,fresh_exact,enqueue_exact]
     split <;> rfl

theorem finishCheck_exact (m : CompositionMachine.Machine p a) (t : InvocationBoundary.Ticket) (value : Int) :
 finishCheck (machine m) t value = CompositionMachine.finishCheck m t value := by
 simp only [finishCheck,CompositionMachine.finishCheck,machine,MixedManagementEmbedding.system,MixedCoreEmbedding.config,
   MixedPureInvocation.pure_result]
 have member : (m.pending.map Pending.pure).contains (.pure t) = m.pending.contains t := by
   simp
 rw [member]

theorem consume_exact (m : CompositionMachine.Machine p a) (t : InvocationBoundary.Ticket) (value : Int) :
 consumePure (machine m) t value = machine (CompositionMachine.consume m t value) := by
 simp only [consumePure,CompositionMachine.consume,machine,MixedManagementEmbedding.system,
  List.filter_map,List.map_cons,event,pendingId,Function.comp_def]
 rfl

theorem finish_exact (m : CompositionMachine.Machine p a) (t : InvocationBoundary.Ticket) (value : Int) :
 finishPure (machine m) t value = (CompositionMachine.finish m t value).map machine := by
 simp only [finishPure,CompositionMachine.finish,finishCheck_exact,consume_exact]
 split <;> rfl

theorem head_exact (m : CompositionMachine.Machine p a) (view : WorldProjection.AuthorityView a) :
 authorityHead (machine m) view = machine (CompositionMachine.authorityHead m view) := rfl

theorem invariant_exact (m : CompositionMachine.Machine p a) : Invariant (machine m) ↔ CompositionMachine.Invariant m := by
 simp only [Invariant,CompositionMachine.Invariant,machine,MixedManagementEntry.Invariant,ManagementEntry.Invariant,
  MixedManagementEmbedding.system,MixedCompositionCore.Invariant,CompositionCore.Invariant,MixedCoreEmbedding.config,
  MixedCatalogEmbedding.valid_exact,mapped_ids]
 simp only [List.mem_map,forall_exists_index,and_imp,forall_apply_eq_imp_iff₂,pendingId]

#print axioms manage_exact
#print axioms start_exact
#print axioms finish_exact
#print axioms head_exact
#print axioms invariant_exact
end MirroreaProofFirst.MixedRequestEmbedding
