import OwnerStatementMetadataHistory
namespace MirroreaProofFirst.OwnerStatementProgram
open MixedOwnerContinuation MixedOwnerProgram

-- Independently stated compiler relation is retained with the SAME program
-- and entries; current values may evolve without reinterpreting that relation.
def Compiled (s : Session p a) : Prop :=
 ∃ before after, Lowers p s.program.fields s.program.labels s.program.control 0
   (reserved before s.program) before s.program.items s.entries after

theorem continued_compiled (accepted : continueWith s program = some next) :
 Compiled next := by
 unfold continueWith at accepted
 split at accepted
 · change (if !drained s then none else do
     let (entries,_) ← MixedOwnerProgram.compile (ReferenceSourceData.environment s.state.source.values) (resumedProgram s program)
     pure {s with
       program := resumedProgram s program
       entries := entries
       cursor := ⟨[],entries,none⟩
       status := .ready
       activation := s.activation+1
       control := (resumedProgram s program).control
       superseded := archive s::s.superseded}) = some next at accepted
   split at accepted
   · cases accepted
   · cases compiled : MixedOwnerProgram.compile (ReferenceSourceData.environment s.state.source.values) (resumedProgram s program) with
     | none => simp [compiled] at accepted
     | some pair =>
       obtain ⟨entries,env⟩ := pair
       simp only [compiled,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
       subst next
       exact ⟨_,_,lower_sound compiled⟩
 · cases accepted

theorem replaced_compiled (accepted : replaceResidual s program = some next) : Compiled next := by
 unfold replaceResidual at accepted
 split at accepted
 · change (if !drained s then none else do
     let (entries,_) ← MixedOwnerProgram.compile (ReferenceSourceData.environment s.state.source.values) (resumedProgram s program)
     pure {s with
       program := resumedProgram s program
       entries := entries
       cursor := ⟨[],entries,none⟩
       status := .ready
       activation := s.activation+1
       control := (resumedProgram s program).control
       superseded := archive s::s.superseded}) = some next at accepted
   split at accepted
   · cases accepted
   · cases compiled : MixedOwnerProgram.compile (ReferenceSourceData.environment s.state.source.values) (resumedProgram s program) with
     | none => simp [compiled] at accepted
     | some pair =>
       obtain ⟨entries,env⟩ := pair
       simp only [compiled,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
       subst next
       exact ⟨_,_,lower_sound compiled⟩
 · cases accepted

theorem transfer_compiled (compiled : Compiled s) (accepted : transfer s = some next) : Compiled next := by
 unfold transfer at accepted
 cases run : MixedOwnerSourceTrace.transfer s.state with
 | none => simp [run] at accepted
 | some state => simp only [run,Option.map_some,Option.some.injEq] at accepted; subst next; exact compiled

theorem authority_compiled (compiled : Compiled s) (accepted : authorityHead s view = some next) : Compiled next := by
 unfold authorityHead at accepted
 cases run : MixedOwnerSourceTrace.authorityHead s.state view with
 | none => simp [run] at accepted
 | some state => simp only [run,Option.map_some,Option.some.injEq] at accepted; subst next; exact compiled

theorem control_compiled (compiled : Compiled s) (accepted : controlInput s member place principal raw = some (next,key)) : Compiled next := by
 unfold controlInput at accepted
 cases run : MixedOwnerSourceTrace.controlInput s.state member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨state,key⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   exact compiled

theorem cancel_compiled (compiled : Compiled s) : Compiled (cancel s member principal) := by
 simp only [cancel]
 split
 · exact compiled
 · exact compiled

theorem rooted_compiled (path : Rooted realm view policy store s) : Compiled s := by
 induction path with
 | launch accepted => obtain ⟨_,_,typed,rfl⟩ := launch_parts accepted; exact ⟨[],_,typed⟩
 | tick _ ih => exact ih
 | transfer _ accepted ih => exact transfer_compiled ih accepted
 | service _ ih => exact ih
 | authority _ accepted ih => exact authority_compiled ih accepted
 | control _ accepted ih => exact control_compiled ih accepted
 | cancel _ ih => exact cancel_compiled ih
 | continued _ accepted _ => exact continued_compiled accepted
 | replaced _ accepted _ => exact replaced_compiled accepted

theorem rooted_source_prefix (path : Rooted realm view policy store s) :
 (∃ suffix, OwnerStatementAcceptance.assignments s.program.items =
    OwnerStatementAcceptance.writes s.cursor.completed ++ suffix) ∧
 OwnerStatementHistory.Valid s := by
 obtain ⟨_,_,compiled⟩ := rooted_compiled path
 exact ⟨OwnerStatementAcceptance.write_prefix (OwnerStatementSession.rooted_cursor path).1 compiled,
   OwnerStatementHistory.rooted_history path⟩

namespace Metadata
open OwnerMetadataSession
def Compiled (s : State p a) : Prop := OwnerStatementProgram.Compiled s.session

theorem refused_valid (valid : Compiled s) : Compiled (refused s candidate) :=
 valid

theorem tick_valid (valid : Compiled s) : Compiled (OwnerMetadataSession.tick s member principal) := by
 have candidate : OwnerStatementProgram.Compiled (MixedOwnerContinuation.tick s.session member principal) := valid
 unfold OwnerMetadataSession.tick
 dsimp only
 split
 · exact candidate
 · split
   · split
     · split
       · exact candidate
       · exact refused_valid valid
     · exact refused_valid valid
   · split
     · exact refused_valid valid
     · exact candidate

theorem transfer_valid (valid : Compiled s) (accepted : OwnerMetadataSession.transfer s = some next) : Compiled next := by
 unfold OwnerMetadataSession.transfer at accepted
 cases run : MixedOwnerContinuation.transfer s.session with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   exact OwnerStatementProgram.transfer_compiled valid run

theorem service_valid (valid : Compiled s) : Compiled (OwnerMetadataSession.service s ops) := by
 rcases service_route (s:=s) (ops:=ops) with same | ⟨_,_,_,_,_,_,same⟩
 · rw [same]; exact valid
 · change OwnerStatementProgram.Compiled (OwnerMetadataSession.service s ops).session
   rw [same]
   exact valid

theorem metadata_valid (valid : Compiled s)
 (accepted : changeMetadata s member place principal site change = some next) : Compiled next := by
 unfold changeMetadata at accepted
 cases run : OwnerMetadataSource.changeMetadata s.session.state.source s.registry member place principal site change with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨source,registry⟩ := pair
   simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
   subst next
   exact valid

theorem authority_valid (valid : Compiled s) (accepted : OwnerMetadataSession.authorityHead s view = some next) : Compiled next := by
 unfold OwnerMetadataSession.authorityHead at accepted
 cases run : MixedOwnerContinuation.authorityHead s.session view with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   exact OwnerStatementProgram.authority_compiled valid run

theorem control_valid (valid : Compiled s)
 (accepted : OwnerMetadataSession.controlInput s member place principal raw = some (next,key)) : Compiled next := by
 unfold OwnerMetadataSession.controlInput at accepted
 cases run : MixedOwnerContinuation.controlInput s.session member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨session,created⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   exact OwnerStatementProgram.control_compiled valid run

theorem continued_valid (accepted : OwnerMetadataSession.continueWith s program = some next) : Compiled next := by
 unfold OwnerMetadataSession.continueWith at accepted
 cases run : MixedOwnerContinuation.continueWith s.session program with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   exact OwnerStatementProgram.continued_compiled run

theorem replaced_valid (accepted : OwnerMetadataSession.replaceResidual s program = some next) : Compiled next := by
 unfold OwnerMetadataSession.replaceResidual at accepted
 cases run : MixedOwnerContinuation.replaceResidual s.session program with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   exact OwnerStatementProgram.replaced_compiled run

theorem attach_valid (valid : OwnerStatementProgram.Compiled session)
 (accepted : attach session registry addresses = some initial) : Compiled initial := by
 unfold attach at accepted
 split at accepted
 · simp only [Option.some.injEq] at accepted
   subst initial
   exact valid
 · cases accepted

-- Every current metadata-attached Session entry, including refusal that keeps
-- the prior cursor instead of publishing the tentative owner request.
theorem rooted_compiled (initialValid : Compiled initial) (path : OwnerMetadataSession.Rooted initial s) : Compiled s := by
 induction path with
 | initial => exact initialValid
 | tick _ ih => exact tick_valid ih
 | transfer _ accepted ih => exact transfer_valid ih accepted
 | service _ ih => exact service_valid ih
 | metadata _ accepted ih => exact metadata_valid ih accepted
 | authority _ accepted ih => exact authority_valid ih accepted
 | control _ accepted ih => exact control_valid ih accepted
 | continued _ accepted _ => exact continued_valid accepted
 | replaced _ accepted _ => exact replaced_valid accepted
 | cancel _ ih => exact OwnerStatementProgram.cancel_compiled ih

theorem rooted_source_prefix
 (initialCompiled : Compiled initial)
 (initialCursor : OwnerStatementMetadataCursor.Valid initial)
 (initialBinding : OwnerStatementMetadataBinding.Valid initial)
 (initialHistory : OwnerStatementMetadataHistory.Valid initial)
 (initialJoint : OwnerMetadataSessionInvariant.Valid initial)
 (path : OwnerMetadataSession.Rooted initial s) :
 (∃ suffix, OwnerStatementAcceptance.assignments s.session.program.items =
   OwnerStatementAcceptance.writes s.session.cursor.completed ++ suffix) ∧
 OwnerStatementHistory.Valid s.session := by
 obtain ⟨_,_,compiled⟩ := rooted_compiled initialCompiled path
 exact ⟨OwnerStatementAcceptance.write_prefix (OwnerStatementMetadataCursor.rooted_cursor initialCursor path).1 compiled,
   OwnerStatementMetadataHistory.rooted_history initialHistory initialBinding initialJoint path⟩

#print axioms rooted_compiled
#print axioms rooted_source_prefix
end Metadata

#print axioms continued_compiled
#print axioms replaced_compiled
#print axioms transfer_compiled
#print axioms authority_compiled
#print axioms control_compiled
#print axioms cancel_compiled
#print axioms rooted_compiled
#print axioms rooted_source_prefix
end MirroreaProofFirst.OwnerStatementProgram
