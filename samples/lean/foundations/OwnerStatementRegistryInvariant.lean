import OwnerStatementRegistrySelection
namespace MirroreaProofFirst.OwnerStatementRegistryInvariant
open OwnerStatementRegistrySelection

-- These are derived safety facts, not a substitute for a reachable admission
-- history or a permission to execute a supplied serialized State.
structure Facts (s : MixedOwnerContinuation.Session p a) : Prop where
 compiled : OwnerStatementProgram.Compiled s
 cursor : OwnerStatementSession.Valid s
 binding : OwnerStatementBinding.Matches s
 history : OwnerStatementHistory.Valid s
 joint : OwnerMetadataSessionInvariant.Joint s.state
 retained : OwnerStatementRetainedHistory.Valid s.state.owner

def Consistent (bank : Bank p) : Prop :=
 ∀ place registry, bank place = some registry → OwnerMetadataRegistry.Consistent registry

def Valid (s : State p a) : Prop := Facts s.session ∧ Consistent s.bank

theorem metadata_tick (facts : Facts s.session) :
 Facts (OwnerMetadataSession.tick s member principal).session := by
 exact ⟨OwnerStatementProgram.Metadata.tick_valid facts.compiled,
 OwnerStatementMetadataCursor.tick_valid facts.cursor,
 OwnerStatementMetadataBinding.tick_valid facts.binding,
 OwnerStatementMetadataHistory.tick_valid facts.history facts.binding facts.joint.2.2.2,
 OwnerMetadataSessionInvariant.tick_joint facts.joint,
 OwnerStatementRetainedHistory.metadata_tick facts.retained⟩

theorem metadata_service (facts : Facts s.session) :
 Facts (OwnerMetadataSession.service s ops).session := by
 exact ⟨OwnerStatementProgram.Metadata.service_valid facts.compiled,
 OwnerStatementMetadataCursor.service_valid facts.cursor,
 OwnerStatementMetadataBinding.service_valid facts.binding,
 OwnerStatementMetadataHistory.service_valid facts.history,
 OwnerMetadataSessionInvariant.service_joint facts.joint,
 OwnerStatementRetainedHistory.metadata_service facts.retained⟩

theorem entry_service (facts : Facts s.session) :
 Facts (OwnerStatementEntryGate.execute s candidate ops).session := by
 rcases OwnerStatementEntryGate.execute_route (live:=s) (candidate:=candidate) (ops:=ops) with same | ⟨_,same⟩
 · rw [same]; exact facts
 · rw [same]; exact metadata_service facts

theorem base_tick (facts : Facts s) : Facts (MixedOwnerContinuation.tick s member principal) := by
 refine ⟨facts.compiled,OwnerStatementSession.tick_valid facts.cursor,
 OwnerStatementBinding.tick_matches facts.binding,
 OwnerStatementHistory.tick_valid facts.history facts.binding facts.joint.2.2.2,
 OwnerMetadataSessionInvariant.moves_joint facts.joint OwnerMetadataSessionInvariant.tick_moves,?_⟩
 rw [OwnerStatementHistory.tick_owner]
 exact facts.retained

theorem refused_facts (facts : Facts s.session) : Facts (refused s candidate).session := by
 refine ⟨facts.compiled,⟨facts.cursor.1,True.intro⟩,?_,facts.history,facts.joint,facts.retained⟩
 intro _ _ _ _ waiting _
 cases waiting

theorem tick_valid (valid : Valid s) : Valid (tick s member principal) := by
 refine ⟨?_,by simpa only [tick_bank] using valid.2⟩
 unfold tick
 dsimp only
 split
 · exact base_tick valid.1
 · split
   · exact refused_facts valid.1
   · exact metadata_tick valid.1

theorem service_valid (valid : Valid s) : Valid (service s ops) := by
 refine ⟨?_,by simpa only [service_bank] using valid.2⟩
 rcases service_route (s:=s) (ops:=ops) with same | ⟨_,_,_,_,_,_,_,_,same⟩
 · rw [same]; exact valid.1
 · rw [same]; exact entry_service valid.1

theorem transfer_valid (valid : Valid s) (accepted : transfer s = some next) : Valid next := by
 unfold transfer at accepted
 cases run : MixedOwnerContinuation.transfer s.session with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   refine ⟨⟨OwnerStatementProgram.transfer_compiled valid.1.compiled run,
     OwnerStatementSession.transfer_valid valid.1.cursor run,
     OwnerStatementBinding.transfer_matches valid.1.binding run,
     OwnerStatementHistory.transfer_valid valid.1.history run,?_,?_⟩,valid.2⟩
   all_goals
     unfold MixedOwnerContinuation.transfer at run
     cases stepped : MixedOwnerSourceTrace.transfer s.session.state with
     | none => simp [stepped] at run
     | some trace =>
       simp only [stepped,Option.map_some,Option.some.injEq] at run
       subst session
       first
       | exact OwnerMetadataSessionInvariant.step_joint valid.1.joint (.transfer stepped)
       | exact OwnerStatementRetainedHistory.trace_step_valid valid.1.retained (.transfer stepped)

theorem metadata_valid (valid : Valid s)
 (accepted : changeMetadata s member place principal site change = some next) : Valid next := by
 unfold changeMetadata at accepted
 cases found : s.bank place with
 | none => simp [found] at accepted
 | some registry =>
   simp only [found,Option.bind_eq_bind,Option.bind_some] at accepted
   cases run : OwnerMetadataSession.changeMetadata (view s registry) member place principal site change with
   | none => simp [run] at accepted
   | some result =>
     simp only [run,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
     subst next
     have joint := OwnerMetadataSessionInvariant.metadata_joint valid.1.joint (valid.2 _ _ found) run
     refine ⟨⟨OwnerStatementProgram.Metadata.metadata_valid (s:=view s registry) valid.1.compiled run,
       OwnerStatementMetadataCursor.metadata_valid (s:=view s registry) valid.1.cursor run,
       OwnerStatementMetadataBinding.metadata_valid (s:=view s registry) valid.1.binding run,
       OwnerStatementMetadataHistory.metadata_valid (s:=view s registry) valid.1.history run,joint.1,
       OwnerStatementRetainedHistory.metadata_change valid.1.retained run⟩,?_⟩
     intro other metadata kept
     by_cases same : other = place
     · subst other
       simp only [set_same,Option.some.injEq] at kept
       subst metadata
       exact joint.2
     · exact valid.2 _ _ ((set_other same).symm.trans kept)

theorem authority_valid (valid : Valid s) (accepted : authorityHead s authority = some next) : Valid next := by
 unfold authorityHead at accepted
 cases run : MixedOwnerContinuation.authorityHead s.session authority with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   refine ⟨⟨OwnerStatementProgram.authority_compiled valid.1.compiled run,OwnerStatementSession.authority_valid valid.1.cursor run,OwnerStatementBinding.authority_matches valid.1.binding run,OwnerStatementHistory.authority_valid valid.1.history run,?_,?_⟩,valid.2⟩
   all_goals
     unfold MixedOwnerContinuation.authorityHead at run
     cases stepped : MixedOwnerSourceTrace.authorityHead s.session.state authority with
     | none => simp [stepped] at run
     | some trace =>
       simp only [stepped,Option.map_some,Option.some.injEq] at run
       subst session
       first
       | exact OwnerMetadataSessionInvariant.step_joint valid.1.joint (.authority stepped)
       | exact OwnerStatementRetainedHistory.trace_step_valid valid.1.retained (.authority stepped)

theorem continued_valid (valid : Valid s) (accepted : continueWith s program = some next) : Valid next := by
 unfold continueWith at accepted
 cases run : MixedOwnerContinuation.continueWith s.session program with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   refine ⟨⟨OwnerStatementProgram.continued_compiled run,OwnerStatementSession.continued_cursor run,OwnerStatementBinding.continued_matches run,OwnerStatementHistory.continued_valid run,?_,?_⟩,valid.2⟩
   · rw [(MixedOwnerContinuation.continued_keeps run).1]; exact valid.1.joint
   · rw [(MixedOwnerContinuation.continued_keeps run).1]; exact valid.1.retained

theorem replaced_valid (valid : Valid s) (accepted : replaceResidual s program = some next) : Valid next := by
 unfold replaceResidual at accepted
 cases run : MixedOwnerContinuation.replaceResidual s.session program with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   refine ⟨⟨OwnerStatementProgram.replaced_compiled run,OwnerStatementSession.replaced_cursor run,OwnerStatementBinding.replaced_matches run,OwnerStatementHistory.replaced_valid run,?_,?_⟩,valid.2⟩
   · rw [(MixedOwnerContinuation.replaced_keeps run).1]; exact valid.1.joint
   · rw [(MixedOwnerContinuation.replaced_keeps run).1]; exact valid.1.retained

theorem control_valid (valid : Valid s)
 (accepted : controlInput s member place principal raw = some (next,key)) : Valid next := by
 unfold controlInput at accepted
 cases run : MixedOwnerContinuation.controlInput s.session member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨session,created⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   refine ⟨⟨OwnerStatementProgram.control_compiled valid.1.compiled run,
     OwnerStatementSession.control_valid valid.1.cursor run,
     OwnerStatementBinding.control_matches valid.1.binding run,
     OwnerStatementHistory.control_valid valid.1.history run,?_,?_⟩,valid.2⟩
   all_goals
     unfold MixedOwnerContinuation.controlInput at run
     cases stepped : MixedOwnerSourceTrace.controlInput s.session.state member place principal raw with
     | none => simp [stepped] at run
     | some pair =>
       obtain ⟨trace,created⟩ := pair
       simp only [stepped,Option.map_some,Option.some.injEq,Prod.mk.injEq] at run
       obtain ⟨rfl,rfl⟩ := run
       first
       | exact OwnerMetadataSessionInvariant.step_joint valid.1.joint (.control stepped)
       | exact OwnerStatementRetainedHistory.trace_step_valid valid.1.retained (.control stepped)

theorem cancel_valid (valid : Valid s) : Valid (cancel s member principal) := by
 refine ⟨⟨OwnerStatementProgram.cancel_compiled valid.1.compiled,
   OwnerStatementSession.cancel_valid valid.1.cursor,
   OwnerStatementBinding.cancel_matches valid.1.binding,
   OwnerStatementHistory.cancel_valid valid.1.history,?_,?_⟩,valid.2⟩
 all_goals
   unfold cancel MixedOwnerContinuation.cancel
   dsimp only
   split
   · first
     | exact OwnerMetadataSessionInvariant.step_joint valid.1.joint .pureCancel
     | exact valid.1.retained
   · first
     | exact valid.1.joint
     | exact valid.1.retained

theorem install_valid (valid : Valid s) (accepted : install s place registry = some next) : Valid next := by
 unfold install at accepted
 split at accepted
 · rename_i checked
   simp only [Bool.and_eq_true,Option.isNone_iff_eq_none,List.isEmpty_iff,beq_iff_eq] at checked
   simp only [Option.some.injEq] at accepted
   subst next
   refine ⟨valid.1,?_⟩
   intro other metadata kept
   by_cases same : other = place
   · subst other
     simp only [set_same,Option.some.injEq] at kept
     subst metadata
     exact OwnerMetadataRegistry.empty_consistent checked.1.1.1.2
   · exact valid.2 _ _ ((set_other same).symm.trans kept)
 · cases accepted

theorem rooted_valid (valid : Valid initial) (path : Rooted initial s) : Valid s := by
 induction path with
 | initial => exact valid
 | tick _ ih => exact tick_valid ih
 | transfer _ run ih => exact transfer_valid ih run
 | service _ ih => exact service_valid ih
 | metadata _ run ih => exact metadata_valid ih run
 | authority _ run ih => exact authority_valid ih run
 | control _ run ih => exact control_valid ih run
 | continued _ run ih => exact continued_valid ih run
 | replaced _ run ih => exact replaced_valid ih run
 | cancel _ ih => exact cancel_valid ih
 | install _ run ih => exact install_valid ih run

-- The base source is freshly launched, not a supplied state that
-- merely passes Facts. No registry exists until explicit checked installation.
def Initial (realm : Nat) (authority : WorldProjection.AuthorityView a)
 (policy : Nat → CurrentUse.Policy) (store : Nat → Option Int) (s : State p a) : Prop :=
 (∃ program, MixedOwnerContinuation.launch realm authority policy store program = some s.session) ∧
 s.bank = (fun _ => none) ∧ s.packet = none

def Admitted (realm : Nat) (authority : WorldProjection.AuthorityView a)
 (policy : Nat → CurrentUse.Policy) (store : Nat → Option Int) (s : State p a) : Prop :=
 ∃ initial, Initial realm authority policy store initial ∧ Rooted initial s

theorem initial_valid (entered : Initial realm authority policy store s) : Valid s := by
 obtain ⟨⟨program,launched⟩,empty,_⟩ := entered
 have path := MixedOwnerContinuation.Rooted.launch launched
 refine ⟨⟨OwnerStatementProgram.rooted_compiled path,
 OwnerStatementSession.rooted_cursor path,OwnerStatementBinding.rooted_matches path,
 OwnerStatementHistory.rooted_history path,MixedOwnerContinuation.rooted_joint path,
 OwnerStatementRetainedHistory.trace_rooted (MixedOwnerContinuation.rooted_source path)⟩,?_⟩
 intro place registry held
 rw [empty] at held
 cases held

theorem admitted_valid (entered : Admitted realm authority policy store s) : Valid s := by
 obtain ⟨initial,entered,path⟩ := entered
 exact rooted_valid (initial_valid entered) path

theorem admitted_source_prefix (entered : Admitted realm authority policy store s) :
 (∃ suffix, OwnerStatementAcceptance.assignments s.session.program.items =
   OwnerStatementAcceptance.writes s.session.cursor.completed ++ suffix) ∧
 OwnerStatementHistory.Valid s.session ∧ OwnerStatementRetainedHistory.Valid s.session.state.owner := by
 have facts := (admitted_valid entered).1
 obtain ⟨_,_,compiled⟩ := facts.compiled
 exact ⟨OwnerStatementAcceptance.write_prefix facts.cursor.1 compiled,facts.history,facts.retained⟩

#print axioms rooted_valid
#print axioms initial_valid
#print axioms admitted_valid
#print axioms admitted_source_prefix
#print axioms tick_valid
#print axioms transfer_valid
#print axioms service_valid
#print axioms metadata_valid
#print axioms authority_valid
#print axioms continued_valid
#print axioms replaced_valid
#print axioms control_valid
#print axioms cancel_valid
#print axioms install_valid
end MirroreaProofFirst.OwnerStatementRegistryInvariant
