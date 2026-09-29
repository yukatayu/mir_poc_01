import OwnerMetadataSession
namespace MirroreaProofFirst.OwnerMetadataSessionInvariant
open OwnerMetadataSession

def Joint (s : MixedOwnerSourceTrace.State p a) : Prop :=
 MixedOwnerSourceInvariant.Valid s.source ∧ MixedOwnerSourceInvariant.OwnerAgrees s.source ∧
 MixedOwnerAttemptQueue.Invariant s.owner ∧ MixedOwnerSourceTrace.Provenance s

theorem step_joint (valid : Joint s) (step : MixedOwnerSourceTrace.Step s next) : Joint next :=
 ⟨MixedOwnerSourceInvariant.step_valid valid.1 step,
  MixedOwnerSourceInvariant.step_owner_agrees valid.1 valid.2.1 step,
  MixedOwnerSourceTrace.step_owner_invariant valid.2.2.1 step,
  MixedOwnerSourceTrace.step_provenance valid.2.2.2 step⟩

def Moves (s next : MixedOwnerSourceTrace.State p a) : Prop := s = next ∨ MixedOwnerSourceTrace.Step s next
theorem moves_joint (valid : Joint s) (step : Moves s next) : Joint next := by
 rcases step with rfl | stepped
 · exact valid
 · exact step_joint valid stepped

theorem advance_moves : Moves s.state (MixedOwnerContinuation.advance s member principal item).state := by
 cases item with
 | ordinary item => exact Or.inr .pureAdvance
 | install name control source =>
   simp only [MixedOwnerContinuation.advance]
   split
   · exact Or.inl rfl
   · exact Or.inr .declaration
 | write name ordinal control source =>
   simp only [MixedOwnerContinuation.advance]
   split
   · exact Or.inl rfl
   · cases run : MixedOwnerSourceTrace.issue s.state s.program.fields s.program.labels member principal s.activation ordinal control name source with
     | none => exact Or.inl rfl
     | some next => exact Or.inr (.issue run)

theorem complete_moves : Moves s.state (MixedOwnerContinuation.complete s).state := by
 unfold MixedOwnerContinuation.complete
 cases waiting : MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting with
 | none => exact Or.inr .pureComplete
 | some held =>
   dsimp only
   split
   · exact Or.inl rfl
   · cases run : MixedOwnerSourceTrace.receive s.state with
     | none => exact Or.inl rfl
     | some next => exact Or.inr (.receive run)

theorem tick_moves : Moves s.state (MixedOwnerContinuation.tick s member principal).state := by
 cases phase : s.status with
 | failed reason => simp [MixedOwnerContinuation.tick,MixedSourceCursor.tick,phase,Moves]
 | waiting => simpa only [MixedOwnerContinuation.tick,MixedSourceCursor.tick,phase] using complete_moves (s:=s)
 | ready =>
   cases rest : s.cursor.remaining with
   | nil => simp [MixedOwnerContinuation.tick,MixedSourceCursor.tick,phase,rest,Moves]
   | cons item rest => simpa only [MixedOwnerContinuation.tick,MixedSourceCursor.tick,phase,rest] using advance_moves (s:=s) (member:=member) (principal:=principal) (item:=item)

theorem tick_joint (valid : Joint s.session.state) : Joint (tick s member principal).session.state := by
 have next := moves_joint valid (tick_moves (s:=s.session) (member:=member) (principal:=principal))
 unfold tick
 dsimp only
 split
 · exact next
 · split
   · split
     · split
       · exact next
       · exact valid
     · exact valid
   · split
     · exact valid
     · exact next

theorem transfer_joint (valid : Joint s.session.state) (accepted : transfer s = some next) : Joint next.session.state := by
 unfold transfer at accepted
 cases outer : MixedOwnerContinuation.transfer s.session with
 | none => simp [outer] at accepted
 | some session =>
   simp only [outer,Option.map_some,Option.some.injEq] at accepted
   subst next
   unfold MixedOwnerContinuation.transfer at outer
   cases run : MixedOwnerSourceTrace.transfer s.session.state with
   | none => simp [run] at outer
   | some trace =>
     simp only [run,Option.map_some,Option.some.injEq] at outer
     subst session
     exact step_joint valid (.transfer run)

theorem service_joint (valid : Joint s.session.state) : Joint (service s ops).session.state := by
 unfold service
 split
 · split
   · exact valid
   · split
     · exact step_joint valid .service
     · exact valid
 · exact valid

theorem metadata_joint (valid : Joint s.session.state) (consistent : OwnerMetadataRegistry.Consistent s.registry)
 (accepted : changeMetadata s member place principal site change = some next) :
 Joint next.session.state ∧ OwnerMetadataRegistry.Consistent next.registry := by
 unfold changeMetadata at accepted
 cases run : OwnerMetadataSource.changeMetadata s.session.state.source s.registry member place principal site change with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨source,registry⟩ := pair
   simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
   subst next
   have preserved := OwnerMetadataSource.preserves valid.1 valid.2.1 consistent run
   exact ⟨⟨preserved.1,preserved.2.1,valid.2.2⟩,preserved.2.2⟩

theorem authority_joint (valid : Joint s.session.state) (accepted : authorityHead s view = some next) : Joint next.session.state := by
 unfold authorityHead at accepted
 cases outer : MixedOwnerContinuation.authorityHead s.session view with
 | none => simp [outer] at accepted
 | some session =>
   simp only [outer,Option.map_some,Option.some.injEq] at accepted
   subst next
   unfold MixedOwnerContinuation.authorityHead at outer
   cases run : MixedOwnerSourceTrace.authorityHead s.session.state view with
   | none => simp [run] at outer
   | some trace =>
     simp only [run,Option.map_some,Option.some.injEq] at outer
     subst session
     exact step_joint valid (.authority run)

theorem control_joint (valid : Joint s.session.state)
 (accepted : controlInput s member place principal raw = some (next,key)) : Joint next.session.state := by
 unfold controlInput at accepted
 cases outer : MixedOwnerContinuation.controlInput s.session member place principal raw with
 | none => simp [outer] at accepted
 | some pair =>
   obtain ⟨session,created⟩ := pair
   simp only [outer,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   unfold MixedOwnerContinuation.controlInput at outer
   cases run : MixedOwnerSourceTrace.controlInput s.session.state member place principal raw with
   | none => simp [run] at outer
   | some pair =>
     obtain ⟨trace,key⟩ := pair
     simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at outer
     obtain ⟨rfl,rfl⟩ := outer
     exact step_joint valid (.control run)

theorem continued_joint (valid : Joint s.session.state) (accepted : continueWith s program = some next) : Joint next.session.state := by
 unfold continueWith at accepted
 cases run : MixedOwnerContinuation.continueWith s.session program with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   simpa only [(MixedOwnerContinuation.continued_keeps run).1] using valid

theorem replaced_joint (valid : Joint s.session.state) (accepted : replaceResidual s program = some next) : Joint next.session.state := by
 unfold replaceResidual at accepted
 cases run : MixedOwnerContinuation.replaceResidual s.session program with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   simpa only [(MixedOwnerContinuation.replaced_keeps run).1] using valid

theorem cancel_joint (valid : Joint s.session.state) : Joint (cancel s member principal).session.state := by
 unfold cancel MixedOwnerContinuation.cancel
 dsimp only
 split
 · exact step_joint valid .pureCancel
 · exact valid

theorem mapped_registry {s : State p a} {input : Option (MixedOwnerContinuation.Session p a)}
 (accepted : (input.map fun session => {s with session := session}) = some next) : next.registry = s.registry := by
 cases input with
 | none => cases accepted
 | some session => cases accepted; rfl

theorem tick_registry : (tick s member principal).registry = s.registry := by
 unfold tick
 dsimp only
 split
 · rfl
 · split
   · split
     · split <;> rfl
     · rfl
   · split <;> rfl

theorem service_registry : (service s ops).registry = s.registry := by
 unfold service
 split
 · split
   · rfl
   · split <;> rfl
 · rfl

theorem control_registry (accepted : controlInput s member place principal raw = some (next,key)) :
 next.registry = s.registry := by
 unfold controlInput at accepted
 cases run : MixedOwnerContinuation.controlInput s.session member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨session,created⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   rfl

def Valid (s : State p a) : Prop := Joint s.session.state ∧ OwnerMetadataRegistry.Consistent s.registry

theorem rooted_joint (valid : Valid initial) (path : Rooted initial s) : Valid s := by
 induction path with
 | initial => exact valid
 | tick _ ih => exact ⟨tick_joint ih.1,by simpa only [tick_registry] using ih.2⟩
 | transfer _ accepted ih =>
   exact ⟨transfer_joint ih.1 accepted,by simpa only [mapped_registry accepted] using ih.2⟩
 | service _ ih => exact ⟨service_joint ih.1,by simpa only [service_registry] using ih.2⟩
 | metadata _ accepted ih => exact metadata_joint ih.1 ih.2 accepted
 | authority _ accepted ih =>
   exact ⟨authority_joint ih.1 accepted,by simpa only [mapped_registry accepted] using ih.2⟩
 | control _ accepted ih =>
   exact ⟨control_joint ih.1 accepted,by simpa only [control_registry accepted] using ih.2⟩
 | continued _ accepted ih =>
   exact ⟨continued_joint ih.1 accepted,by simpa only [mapped_registry accepted] using ih.2⟩
 | replaced _ accepted ih =>
   exact ⟨replaced_joint ih.1 accepted,by simpa only [mapped_registry accepted] using ih.2⟩
 | cancel _ ih => exact ⟨cancel_joint ih.1,ih.2⟩

#print axioms tick_moves
#print axioms tick_joint
#print axioms transfer_joint
#print axioms service_joint
#print axioms metadata_joint
#print axioms authority_joint
#print axioms control_joint
#print axioms continued_joint
#print axioms replaced_joint
#print axioms cancel_joint
#print axioms rooted_joint
end MirroreaProofFirst.OwnerMetadataSessionInvariant
