import OwnerStatementMetadataBinding
namespace MirroreaProofFirst.OwnerStatementHistory
open MixedOwnerContinuation MixedOwnerProgram

-- Every acknowledged source write has a real owner-history witness for the
-- same source activation/position. This is distinct from cursor partition.
def Valid (s : Session p a) : Prop :=
 ∀ name ordinal control source, .write name ordinal control source ∈ s.cursor.completed →
 ∃ written ∈ s.state.owner.history,
 written.pending.origin = ⟨source.site.document,source.site.byteOffset,s.activation,ordinal,control⟩

theorem advance_owner : (advance s member principal entry).state.owner = s.state.owner := by
 cases entry with
 | ordinary item => rfl
 | install name control source => simp only [advance]; split <;> rfl
 | write name ordinal control source =>
   simp only [advance]
   split
   · rfl
   · cases run : MixedOwnerSourceTrace.issue s.state s.program.fields s.program.labels member principal s.activation ordinal control name source with
     | none => rfl
     | some next => obtain ⟨_,_,_,rfl⟩ := MixedOwnerSourceTrace.issue_parts run; rfl

theorem complete_owner : (complete s).state.owner = s.state.owner := by
 unfold complete
 split
 · rfl
 · split
   · rfl
   · cases run : MixedOwnerSourceTrace.receive s.state with
     | none => rfl
     | some next => obtain ⟨_,_,_,_,_,rfl⟩ := MixedOwnerSourceTrace.receive_parts run; rfl

theorem tick_owner : (tick s member principal).state.owner = s.state.owner := by
 cases phase : s.status with
 | failed reason => simp [tick,MixedSourceCursor.tick,phase]
 | waiting => simpa only [tick,MixedSourceCursor.tick,phase] using complete_owner (s:=s)
 | ready =>
   cases rest : s.cursor.remaining with
   | nil => simp [tick,MixedSourceCursor.tick,phase,rest]
   | cons item tail => simpa only [tick,MixedSourceCursor.tick,phase,rest] using advance_owner (s:=s) (member:=member) (principal:=principal) (entry:=item)

theorem write_not_ready {control : Nat} :
 (advance s member principal (.write name ordinal control source)).status ≠ .ready := by
 simp only [advance]
 split
 · simp
 · split <;> simp

theorem tick_valid (valid : Valid s) (binding : OwnerStatementBinding.Matches s)
 (provenance : MixedOwnerSourceTrace.Provenance s.state) : Valid (tick s member principal) := by
 intro name ordinal control source present
 change ∃ written ∈ (tick s member principal).state.owner.history,
   written.pending.origin = ⟨source.site.document,source.site.byteOffset,s.activation,ordinal,control⟩
 rw [tick_owner]
 cases phase : s.status with
 | failed reason =>
   exact valid name ordinal control source (by simpa [tick,MixedSourceCursor.tick,phase] using present)
 | ready =>
   cases rest : s.cursor.remaining with
   | nil => exact valid name ordinal control source (by simpa [tick,MixedSourceCursor.tick,phase,rest] using present)
   | cons item tail =>
     by_cases ready : (advance s member principal item).status = .ready
     · have member : .write name ordinal control source ∈ s.cursor.completed ∨ .write name ordinal control source = item := by
         simpa [tick,MixedSourceCursor.tick,phase,rest,ready] using present
       rcases member with prior | rfl
       · exact valid name ordinal control source prior
       · exact False.elim (write_not_ready ready)
     · exact valid name ordinal control source (by simpa [tick,MixedSourceCursor.tick,phase,rest,ready] using present)
 | waiting =>
   by_cases ready : (complete s).status = .ready
   · have member : .write name ordinal control source ∈ s.cursor.completed ∨ s.cursor.stopped = some (.write name ordinal control source) := by
       simpa [tick,MixedSourceCursor.tick,phase,ready] using present
     rcases member with prior | stopped
     · exact valid name ordinal control source prior
     · obtain ⟨held,hw,_,origin⟩ := binding name ordinal control source phase stopped
       obtain ⟨written,member,same⟩ := (OwnerStatementAcceptance.owner_complete_committed provenance hw ready).1
       exact ⟨written,member,by rw [same,origin]⟩
   · exact valid name ordinal control source (by simpa [tick,MixedSourceCursor.tick,phase,ready] using present)

theorem retained {s next : Session p a} (valid : Valid s) (completed : next.cursor.completed = s.cursor.completed)
 (activation : next.activation = s.activation)
 (history : ∀ written ∈ s.state.owner.history, written ∈ next.state.owner.history) : Valid next := by
 intro name ordinal control source present
 obtain ⟨written,member,origin⟩ := valid name ordinal control source (completed ▸ present)
 exact ⟨written,history written member,activation.symm ▸ origin⟩

theorem empty_valid (empty : s.cursor.completed = []) : Valid s := by
 intro _ _ _ _ member
 simp [empty] at member

theorem transfer_valid (valid : Valid s) (accepted : transfer s = some next) : Valid next := by
 unfold transfer at accepted
 cases run : MixedOwnerSourceTrace.transfer s.state with
 | none => simp [run] at accepted
 | some state =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   obtain ⟨_,_,_,_,enqueued,rfl⟩ := MixedOwnerSourceTrace.transfer_parts run
   obtain ⟨_,_,rfl⟩ := MixedOwnerAttemptQueue.enqueue_exact.mp enqueued
   exact retained valid rfl rfl (fun _ member => member)

theorem service_valid (valid : Valid s) : Valid (service s ops metadata) := by
 refine retained (next:=service s ops metadata) valid rfl rfl ?_
 intro written member
 simp only [service,MixedOwnerSourceTrace.service]
 split
 · change written ∈ (MixedOwnerAttemptQueue.advance ops _ _ metadata s.state.owner).1.history
   obtain ⟨suffix,history⟩ := MixedOwnerAttemptQueue.history_retained (ops:=ops)
     (catalog:=s.state.source.machine.store.core.system.configuration.state)
     (view:=s.state.source.machine.store.core.system.view) (current:=metadata) (s:=s.state.owner)
   rw [history]
   exact List.mem_append_left _ member
 · exact member

theorem authority_valid (valid : Valid s) (accepted : authorityHead s view = some next) : Valid next := by
 unfold authorityHead at accepted
 cases run : MixedOwnerSourceTrace.authorityHead s.state view with
 | none => simp [run] at accepted
 | some state =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   obtain ⟨_,rfl⟩ := MixedOwnerSourceTrace.authority_parts run
   exact retained valid rfl rfl (fun _ member => member)

theorem control_valid (valid : Valid s)
 (accepted : controlInput s member place principal raw = some (next,key)) : Valid next := by
 unfold controlInput at accepted
 cases run : MixedOwnerSourceTrace.controlInput s.state member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨state,created⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   obtain ⟨_,rfl⟩ := MixedOwnerSourceTrace.control_parts run
   exact retained valid rfl rfl (fun _ member => member)

theorem cancel_valid (valid : Valid s) : Valid (cancel s member principal) := by
 simp only [cancel]
 split
 · exact retained valid rfl rfl (fun _ member => member)
 · exact valid

theorem continued_valid (accepted : continueWith s program = some next) : Valid next := by
 unfold continueWith at accepted
 split at accepted
 · change (if !drained s then none else do
     let (entries,_) ← MixedOwnerProgram.compile (ReferenceSourceData.environment s.state.source.values) (resumedProgram s program)
     pure {s with
       program := resumedProgram s program,entries := entries,cursor := ⟨[],entries,none⟩,
       status := .ready,activation := s.activation+1,control := (resumedProgram s program).control,
       superseded := archive s::s.superseded}) = some next at accepted
   split at accepted
   · cases accepted
   · cases compiled : MixedOwnerProgram.compile (ReferenceSourceData.environment s.state.source.values) (resumedProgram s program) with
     | none => simp [compiled] at accepted
     | some pair =>
       obtain ⟨entries,env⟩ := pair
       simp only [compiled,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
       subst next
       exact empty_valid rfl
 · cases accepted

theorem replaced_valid (accepted : replaceResidual s program = some next) : Valid next := by
 unfold replaceResidual at accepted
 split at accepted
 · change (if !drained s then none else do
     let (entries,_) ← MixedOwnerProgram.compile (ReferenceSourceData.environment s.state.source.values) (resumedProgram s program)
     pure {s with
       program := resumedProgram s program,entries := entries,cursor := ⟨[],entries,none⟩,
       status := .ready,activation := s.activation+1,control := (resumedProgram s program).control,
       superseded := archive s::s.superseded}) = some next at accepted
   split at accepted
   · cases accepted
   · cases compiled : MixedOwnerProgram.compile (ReferenceSourceData.environment s.state.source.values) (resumedProgram s program) with
     | none => simp [compiled] at accepted
     | some pair =>
       obtain ⟨entries,env⟩ := pair
       simp only [compiled,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
       subst next
       exact empty_valid rfl
 · cases accepted

theorem rooted_history (path : Rooted realm view policy store s) : Valid s := by
 induction path with
 | launch accepted => obtain ⟨_,_,_,rfl⟩ := launch_parts accepted; exact empty_valid rfl
 | tick prior ih => exact tick_valid ih (OwnerStatementBinding.rooted_matches prior) (rooted_joint prior).2.2.2
 | transfer _ accepted ih => exact transfer_valid ih accepted
 | service _ ih => exact service_valid ih
 | authority _ accepted ih => exact authority_valid ih accepted
 | control _ accepted ih => exact control_valid ih accepted
 | cancel _ ih => exact cancel_valid ih
 | continued _ accepted _ => exact continued_valid accepted
 | replaced _ accepted _ => exact replaced_valid accepted

#print axioms transfer_valid
#print axioms service_valid
#print axioms authority_valid
#print axioms control_valid
#print axioms cancel_valid
#print axioms continued_valid
#print axioms replaced_valid
#print axioms rooted_history
#print axioms advance_owner
#print axioms complete_owner
#print axioms tick_owner
#print axioms write_not_ready
#print axioms tick_valid
#print axioms retained
#print axioms empty_valid
end MirroreaProofFirst.OwnerStatementHistory
