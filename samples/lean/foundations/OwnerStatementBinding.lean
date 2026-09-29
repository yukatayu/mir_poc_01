import OwnerStatementAcceptance
namespace MirroreaProofFirst.OwnerStatementBinding
open MixedOwnerContinuation MixedOwnerProgram

-- A waiting source occurrence is tied to the actual retained request, including
-- its activation and ordinal. Identity here does not confer authority.
def Payload (state : MixedOwnerSourceTrace.State p a) (activation ordinal control : Nat)
 (statement : MixedNamedOwnerSource.Assignment) : Prop :=
 ∃ held, MixedOwnerSourceIssue.ownerWaiting state.source.waiting = some held ∧
 held.statement = statement ∧
 held.saved.origin = ⟨statement.site.document,statement.site.byteOffset,activation,ordinal,control⟩

def Matches (s : Session p a) : Prop :=
 ∀ name ordinal control statement, s.status = .waiting →
 s.cursor.stopped = some (.write name ordinal control statement) →
 Payload s.state s.activation ordinal control statement

theorem issue_payload {s : Session p a} {control : Nat}
 (accepted : MixedOwnerSourceTrace.issue s.state s.program.fields s.program.labels member principal
   s.activation ordinal control name statement = some next) :
 Payload next s.activation ordinal control statement := by
 obtain ⟨source,held,run,rfl⟩ := MixedOwnerSourceTrace.issue_parts accepted
 obtain ⟨_,_,plan,key,place,_,_,_,_,_,_,waiting,_,_,_,same,_,_⟩ := MixedOwnerSourceIssue.issue_parts control run
 obtain ⟨_,_,_,_,origin,_,_,_⟩ := MixedOwnerSourceIssue.issue_payload control run
 exact ⟨held,by simp [MixedOwnerSourceIssue.ownerWaiting,waiting],same,origin⟩

theorem advance_write_payload {control : Nat}
 (waiting : (advance s member principal (.write name ordinal control statement)).status = .waiting) :
 Payload (advance s member principal (.write name ordinal control statement)).state s.activation ordinal control statement := by
 simp only [advance] at waiting ⊢
 split at waiting
 · cases waiting
 · rename_i admitted
   rw [if_neg admitted]
   cases run : MixedOwnerSourceTrace.issue s.state s.program.fields s.program.labels member principal s.activation ordinal control name statement with
   | none => simp [run] at waiting
   | some next => exact issue_payload run

theorem complete_wait_keeps
 (held : MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting = some saved)
 (waiting : (complete s).status = .waiting) : (complete s).state = s.state := by
 unfold complete at waiting ⊢
 simp only [held] at waiting ⊢
 split at waiting
 · rename_i empty; simp [empty]
 · rename_i nonempty
   cases run : MixedOwnerSourceTrace.receive s.state <;> simp [run] at waiting

theorem tick_matches (invariant : Matches s) : Matches (tick s member principal) := by
 intro name ordinal control statement waiting stopped
 cases phase : s.status with
 | failed reason => simp [tick,MixedSourceCursor.tick,phase] at waiting
 | ready =>
   cases rest : s.cursor.remaining with
   | nil => simp [tick,MixedSourceCursor.tick,phase,rest] at waiting
   | cons item tail =>
     have aw : (advance s member principal item).status = .waiting := by
       simpa only [tick,MixedSourceCursor.tick,phase,rest] using waiting
     have itemEq : item = .write name ordinal control statement := by
       simpa [tick,MixedSourceCursor.tick,phase,rest,aw] using stopped
     subst item
     simpa only [tick,MixedSourceCursor.tick,phase,rest] using advance_write_payload aw
 | waiting =>
   have cw : (complete s).status = .waiting := by simpa only [tick,MixedSourceCursor.tick,phase] using waiting
   have prior : s.cursor.stopped = some (.write name ordinal control statement) := by
     simpa [tick,MixedSourceCursor.tick,phase,cw] using stopped
   have bound := invariant name ordinal control statement phase prior
   obtain ⟨held,hw,hs,ho⟩ := bound
   have kept := complete_wait_keeps hw cw
   simpa only [tick,MixedSourceCursor.tick,phase,kept] using (show Payload s.state s.activation ordinal control statement from ⟨held,hw,hs,ho⟩)

-- Structural preservation helper: every use below must prove these concrete
-- equalities from the mutator; raw restored records are not admitted by it.
theorem retained (invariant : Matches s)
 (phase : next.status = s.status) (cursor : next.cursor = s.cursor)
 (activation : next.activation = s.activation)
 (waiting : MixedOwnerSourceIssue.ownerWaiting next.state.source.waiting =
   MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting) : Matches next := by
 intro name ordinal control statement hw hs
 obtain ⟨held,bound,same,origin⟩ := invariant name ordinal control statement (phase.symm.trans hw) (cursor.symm ▸ hs)
 exact ⟨held,waiting.trans bound,same,activation.symm ▸ origin⟩

theorem transfer_matches (invariant : Matches s) (accepted : transfer s = some next) : Matches next := by
 unfold transfer at accepted
 cases run : MixedOwnerSourceTrace.transfer s.state with
 | none => simp [run] at accepted
 | some state =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   obtain ⟨_,_,_,_,_,rfl⟩ := MixedOwnerSourceTrace.transfer_parts run
   exact retained invariant rfl rfl rfl rfl

theorem service_matches (invariant : Matches s) : Matches (service s ops metadata) := by
 refine retained (next:=service s ops metadata) invariant rfl rfl rfl ?_
 simp only [service,MixedOwnerSourceTrace.service]
 split <;> rfl

theorem authority_matches (invariant : Matches s) (accepted : authorityHead s view = some next) : Matches next := by
 unfold authorityHead at accepted
 cases run : MixedOwnerSourceTrace.authorityHead s.state view with
 | none => simp [run] at accepted
 | some state =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   unfold MixedOwnerSourceTrace.authorityHead at run
   cases head : MixedOwnerSourceIssue.authorityHead s.state.source view with
   | none => simp [head] at run
   | some source =>
     simp only [head,Option.map_some,Option.some.injEq] at run
     subst state
     refine retained (next:={s with state := {s.state with source := source}}) invariant rfl rfl rfl ?_
     simp only [MixedOwnerSourceInvariant.authority_parts head]
     exact MixedOwnerSourceInvariant.merge_owner_wait

theorem control_matches (invariant : Matches s)
 (accepted : controlInput s member place principal raw = some (next,key)) : Matches next := by
 unfold controlInput at accepted
 cases run : MixedOwnerSourceTrace.controlInput s.state member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨state,created⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   unfold MixedOwnerSourceTrace.controlInput at run
   cases input : MixedOwnerSourceIssue.controlInput s.state.source member place principal raw with
   | none => simp [input] at run
   | some pair =>
     obtain ⟨source,created⟩ := pair
     simp only [input,Option.map_some,Option.some.injEq,Prod.mk.injEq] at run
     obtain ⟨rfl,rfl⟩ := run
     obtain ⟨pure,_,rfl⟩ := MixedOwnerSourceInvariant.control_parts input
     exact retained invariant rfl rfl rfl MixedOwnerSourceInvariant.merge_owner_wait

theorem ready_matches (ready : s.status = .ready) : Matches s := by
 intro _ _ _ _ waiting _
 simp [ready] at waiting

theorem cancel_matches (invariant : Matches s) : Matches (cancel s member principal) := by
 simp only [cancel]
 split
 · intro _ _ _ _ waiting _; cases waiting
 · exact invariant

theorem continued_matches (accepted : continueWith s program = some next) : Matches next := by
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
       exact ready_matches rfl
 · cases accepted

theorem replaced_matches (accepted : replaceResidual s program = some next) : Matches next := by
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
       exact ready_matches rfl
 · cases accepted

theorem rooted_matches (path : Rooted realm view policy store s) : Matches s := by
 induction path with
 | launch accepted => obtain ⟨_,_,_,rfl⟩ := launch_parts accepted; exact ready_matches rfl
 | tick _ ih => exact tick_matches ih
 | transfer _ accepted ih => exact transfer_matches ih accepted
 | service _ ih => exact service_matches ih
 | authority _ accepted ih => exact authority_matches ih accepted
 | control _ accepted ih => exact control_matches ih accepted
 | cancel _ ih => exact cancel_matches ih
 | continued _ accepted _ => exact continued_matches accepted
 | replaced _ accepted _ => exact replaced_matches accepted

-- Actual successful completion of this exact source occurrence has an owner
-- history witness with the same invocation/ordinal/control/site, not just name.
theorem rooted_occurrence_committed {control : Nat}
 (path : Rooted realm view policy store s)
 (waiting : s.status = .waiting)
 (stopped : s.cursor.stopped = some (.write name ordinal control statement))
 (ready : (complete s).status = .ready) :
 ∃ written ∈ s.state.owner.history,
 written.pending.origin = ⟨statement.site.document,statement.site.byteOffset,s.activation,ordinal,control⟩ := by
 obtain ⟨held,hw,_,origin⟩ := rooted_matches path name ordinal control statement waiting stopped
 obtain ⟨written,member,same⟩ := OwnerStatementAcceptance.rooted_owner_complete_committed path hw ready
 exact ⟨written,member,by rw [same,origin]⟩

#print axioms transfer_matches
#print axioms service_matches
#print axioms authority_matches
#print axioms control_matches
#print axioms ready_matches
#print axioms cancel_matches
#print axioms continued_matches
#print axioms replaced_matches
#print axioms rooted_matches
#print axioms rooted_occurrence_committed
#print axioms issue_payload
#print axioms advance_write_payload
#print axioms complete_wait_keeps
#print axioms tick_matches
#print axioms retained
end MirroreaProofFirst.OwnerStatementBinding
