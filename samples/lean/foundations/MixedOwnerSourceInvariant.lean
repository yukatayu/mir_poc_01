import MixedOwnerSourceTrace
namespace MirroreaProofFirst.MixedOwnerSourceInvariant
open MixedOwnerSourceIssue

-- Preserve the real common execution/catalog/history invariant, the single
-- all-kind allocator bound, and correspondence of the protected pure wait.
-- Owner-slot correspondence is a separate property below, not inferred from
-- the empty pure projection or from arbitrary state construction.

theorem base_merge_idle (idle : ownerWaiting s.waiting = none) : base (merge s next) = next := by
 cases next with
 | mk machine values counter writes origins waiting =>
   cases waiting <;> simp [base,merge,idle,pureWaiting]

theorem base_merge_wait (same : next.waiting = (base s).waiting) : base (merge s next) = next := by
 cases sw : s.waiting with
 | none => exact base_merge_idle (by simp [ownerWaiting,sw])
 | some held =>
   cases held with
   | inl saved => exact base_merge_idle (by simp [ownerWaiting,sw])
   | inr saved =>
     have empty : next.waiting = none := by simpa [base,pureWaiting,sw] using same
     cases next
     simp_all [base,merge,ownerWaiting,pureWaiting]

theorem merge_valid (valid : Valid s) (projection : base (merge s next) = next)
 (step : MixedReferenceSourceTrace.Step (base s) next) : Valid (merge s next) := by
 have machine := MixedReferenceExecution.reached_preserves _ _ valid.1 (MixedReferenceSourceTrace.step_machine _ _ step)
 have bound := MixedReferenceSourceTrace.step_allocated (base s) next valid.2.1 step
 have pending := MixedReferenceSourceTrace.step_pending _ _ valid.2.2 step
 refine ⟨machine,bound,?_⟩
 rw [projection]; exact pending

theorem advance_valid (valid : Valid s) : Valid (advancePure s member place principal item).state := by
 unfold advancePure
 split
 · exact valid
 · rename_i notOwner
   have idle : ownerWaiting s.waiting = none := by cases w : ownerWaiting s.waiting <;> simp_all
   exact merge_valid valid (base_merge_idle idle) .advance

theorem complete_valid (valid : Valid s) : Valid (completePure s).state := by
 unfold completePure
 split
 · exact valid
 · rename_i notOwner
   have idle : ownerWaiting s.waiting = none := by cases w : ownerWaiting s.waiting <;> simp_all
   exact merge_valid valid (base_merge_idle idle) .complete

theorem cancel_valid (valid : Valid s) : Valid (cancelPure s member place principal).state := by
 unfold cancelPure
 split
 · exact valid
 · rename_i notOwner
   have idle : ownerWaiting s.waiting = none := by cases w : ownerWaiting s.waiting <;> simp_all
   exact merge_valid valid (base_merge_idle idle) .cancellation

theorem control_parts (accepted : controlInput s member place principal raw = some (next,key)) :
 ∃ pure, MixedReferenceSource.controlInput (base s) member place principal raw = some (pure,key) ∧ next = merge s pure := by
 unfold controlInput at accepted
 cases run : MixedReferenceSource.controlInput (base s) member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨pure,created⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   exact ⟨pure,rfl,rfl⟩

theorem control_valid (valid : Valid s)
 (accepted : controlInput s member place principal raw = some (next,key)) : Valid next := by
 obtain ⟨pure,run,rfl⟩ := control_parts accepted
 have wait := (MixedReferenceSource.controlInput_projects _ _ _ _ _ _ run).2.1
 exact merge_valid valid (base_merge_wait wait) (.control run)

theorem authority_parts (accepted : authorityHead s view = some next) :
 next = merge s (MixedReferenceSource.authorityHead (base s) view) := by
 unfold authorityHead at accepted
 cases run : MixedReferenceAuthority.install (base s) view with
 | none => simp [run] at accepted
 | some pure =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   have equal := (MixedReferenceAuthority.install_parts _ _ _ run).2
   exact accepted.symm.trans (congrArg (merge s) equal)

theorem authority_valid (valid : Valid s)
 (accepted : authorityHead s view = some next) : Valid next := by
 rw [authority_parts accepted]
 exact merge_valid valid (base_merge_wait rfl) .head

theorem issue_valid {control : Nat} (valid : Valid s)
 (accepted : issue s ctx labels member principal activation ordinal control name statement = some (next,waiting)) : Valid next := by
 obtain ⟨machine,bound,_,pure,wait⟩ := issue_preserves control valid.1 valid.2.1 accepted
 exact ⟨machine,bound,by simp [MixedReferenceSource.PendingAgrees,base,pureWaiting,wait,pure]⟩

theorem acknowledge_valid (valid : Valid s)
 (accepted : acknowledge s received evidence = some next) : Valid next := by
 have kept := acknowledge_preserves valid.1 valid.2.1 accepted
 obtain ⟨waiting,atWait,run,equal⟩ := acknowledge_parts accepted
 have empty : s.machine.pending = [] := by
   simpa [MixedReferenceSource.PendingAgrees,base,pureWaiting,atWait] using valid.2.2
 have classification := (MixedReferenceExecution.finishOwner_parts _ run).2
 exact ⟨kept.1,kept.2.1,by simp [MixedReferenceSource.PendingAgrees,base,pureWaiting,kept.2.2.1,classification,empty]⟩

theorem step_valid (valid : Valid s.source) (transition : MixedOwnerSourceTrace.Step s next) : Valid next.source := by
 cases transition with
 | issue accepted =>
   obtain ⟨_,_,run,rfl⟩ := MixedOwnerSourceTrace.issue_parts accepted
   exact issue_valid valid run
 | transfer accepted => obtain ⟨_,_,_,_,_,rfl⟩ := MixedOwnerSourceTrace.transfer_parts accepted; exact valid
 | service => unfold MixedOwnerSourceTrace.service; split <;> exact valid
 | receive accepted =>
   obtain ⟨_,_,_,_,run,rfl⟩ := MixedOwnerSourceTrace.receive_parts accepted
   exact acknowledge_valid valid run
 | declaration =>
   exact MixedOwnerSourceDeclaration.declaration_valid valid
 | pureAdvance => exact advance_valid valid
 | pureComplete => exact complete_valid valid
 | pureCancel => exact cancel_valid valid
 | control accepted =>
   rename_i member place principal raw key
   unfold MixedOwnerSourceTrace.controlInput at accepted
   cases run : controlInput s.source member place principal raw with
   | none => simp [run] at accepted
   | some pair =>
     obtain ⟨source,key⟩ := pair
     simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
     obtain ⟨rfl,rfl⟩ := accepted
     exact control_valid valid run
 | authority accepted =>
   rename_i view
   unfold MixedOwnerSourceTrace.authorityHead at accepted
   cases run : authorityHead s.source view with
   | none => simp [run] at accepted
   | some source =>
     simp only [run,Option.map_some,Option.some.injEq] at accepted
     subst next
     exact authority_valid valid run


theorem merge_owner_wait : ownerWaiting (merge s next).waiting = ownerWaiting s.waiting := by
 simp only [merge]
 cases sw : ownerWaiting s.waiting with
 | none => cases nw : next.waiting <;> simp [ownerWaiting]
 | some saved => rfl

theorem merge_owner_agrees (valid : OwnerAgrees s)
 (pending : next.machine.store.core.pending = s.machine.store.core.pending) : OwnerAgrees (merge s next) := by
 intro waiting atWait
 have old : ownerWaiting s.waiting = some waiting := by rw [←merge_owner_wait (next:=next),atWait]; rfl
 have sourceWait : s.waiting = some (.inr waiting) := by
   cases sw : s.waiting with
   | none => simp [ownerWaiting,sw] at old
   | some held => cases held <;> simpa [ownerWaiting,sw] using old
 exact pending.trans (valid waiting sourceWait)

theorem merge_no_owner (idle : ownerWaiting s.waiting = none) : OwnerAgrees (merge s next) := by
 intro waiting atWait
 have old := merge_owner_wait (s:=s) (next:=next)
 rw [idle,atWait] at old
 cases old

theorem advance_owner_agrees (valid : OwnerAgrees s) : OwnerAgrees (advancePure s member place principal item).state := by
 unfold advancePure
 split
 · exact valid
 · rename_i notOwner
   apply merge_no_owner
   cases waiting : ownerWaiting s.waiting <;> simp_all

theorem complete_owner_agrees (valid : OwnerAgrees s) : OwnerAgrees (completePure s).state := by
 unfold completePure
 split
 · exact valid
 · rename_i notOwner
   apply merge_no_owner
   cases waiting : ownerWaiting s.waiting <;> simp_all

theorem cancel_owner_agrees (valid : OwnerAgrees s) : OwnerAgrees (cancelPure s member place principal).state := by
 unfold cancelPure
 split
 · exact valid
 · rename_i notOwner
   apply merge_no_owner
   cases waiting : ownerWaiting s.waiting <;> simp_all

theorem control_owner_agrees (valid : OwnerAgrees s)
 (accepted : controlInput s member place principal raw = some (next,key)) : OwnerAgrees next := by
 obtain ⟨pure,run,rfl⟩ := control_parts accepted
 apply merge_owner_agrees valid
 unfold MixedReferenceSource.controlInput at run
 cases managed : MixedReferenceExecution.manage s.machine member place principal s.nextRequest raw with
 | none => simp [base,managed] at run
 | some pair =>
   obtain ⟨machine,created⟩ := pair
   simp only [base,managed,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at run
   obtain ⟨rfl,rfl⟩ := run
   have core := MixedReferenceExecution.manage_core _ _ _ _ _ _ _ managed
   obtain ⟨_,_,_,_,_,_,equal⟩ := MixedRequestCore.manage_parts core
   exact congrArg (fun r => r.1.pending) equal

theorem authority_owner_agrees (valid : OwnerAgrees s)
 (accepted : authorityHead s view = some next) : OwnerAgrees next := by
 rw [authority_parts accepted]
 exact merge_owner_agrees valid rfl

theorem issue_owner_agrees {control : Nat} (valid : Valid s)
 (accepted : issue s ctx labels member principal activation ordinal control name statement = some (next,waiting)) : OwnerAgrees next := by
 obtain ⟨_,_,pending,_,wait⟩ := issue_preserves control valid.1 valid.2.1 accepted
 intro other atWait
 have same : other = waiting := Sum.inr.inj (Option.some.inj (atWait.symm.trans wait))
 simpa [same] using pending

theorem acknowledge_owner_agrees (accepted : acknowledge s received evidence = some next) : OwnerAgrees next := by
 obtain ⟨_,_,_,equal⟩ := acknowledge_parts accepted
 rw [equal]; intro waiting atWait; cases atWait

theorem step_owner_agrees (valid : Valid s.source) (agrees : OwnerAgrees s.source)
 (transition : MixedOwnerSourceTrace.Step s next) : OwnerAgrees next.source := by
 cases transition with
 | issue accepted =>
   obtain ⟨_,_,run,rfl⟩ := MixedOwnerSourceTrace.issue_parts accepted
   exact issue_owner_agrees valid run
 | transfer accepted => obtain ⟨_,_,_,_,_,rfl⟩ := MixedOwnerSourceTrace.transfer_parts accepted; exact agrees
 | service => unfold MixedOwnerSourceTrace.service; split <;> exact agrees
 | receive accepted =>
   obtain ⟨_,_,_,_,run,rfl⟩ := MixedOwnerSourceTrace.receive_parts accepted
   exact acknowledge_owner_agrees run
 | declaration => exact (MixedOwnerSourceDeclaration.declaration_preserves valid agrees).2
 | pureAdvance => exact advance_owner_agrees agrees
 | pureComplete => exact complete_owner_agrees agrees
 | pureCancel => exact cancel_owner_agrees agrees
 | control accepted =>
   rename_i member place principal raw key
   unfold MixedOwnerSourceTrace.controlInput at accepted
   cases run : controlInput s.source member place principal raw with
   | none => simp [run] at accepted
   | some pair =>
     obtain ⟨source,key⟩ := pair
     simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
     obtain ⟨rfl,rfl⟩ := accepted
     exact control_owner_agrees agrees run
 | authority accepted =>
   rename_i view
   unfold MixedOwnerSourceTrace.authorityHead at accepted
   cases run : authorityHead s.source view with
   | none => simp [run] at accepted
   | some source =>
     simp only [run,Option.map_some,Option.some.injEq] at accepted
     subst next
     exact authority_owner_agrees agrees run

theorem rooted_valid (initialValid : Valid source) (path : MixedOwnerSourceTrace.Rooted source store s) : Valid s.source := by
 induction path with
 | initial => exact initialValid
 | step _ transition ih => exact step_valid ih transition

theorem rooted_joint (initialValid : Valid source) (initialOwner : OwnerAgrees source)
 (path : MixedOwnerSourceTrace.Rooted source store s) : Valid s.source ∧ OwnerAgrees s.source := by
 induction path with
 | initial => exact ⟨initialValid,initialOwner⟩
 | step _ transition ih => exact ⟨step_valid ih.1 transition,step_owner_agrees ih.1 ih.2 transition⟩

#print axioms base_merge_idle
#print axioms base_merge_wait
#print axioms advance_valid
#print axioms complete_valid
#print axioms cancel_valid
#print axioms control_valid
#print axioms authority_valid
#print axioms issue_valid
#print axioms acknowledge_valid
#print axioms rooted_valid
#print axioms rooted_joint
end MirroreaProofFirst.MixedOwnerSourceInvariant
