import OwnerStatementSession
import OwnerMetadataSession
namespace MirroreaProofFirst.OwnerStatementMetadataCursor
open OwnerMetadataSession

def Valid (s : State p a) : Prop := OwnerStatementSession.Valid s.session

theorem refused_valid (valid : Valid s) : Valid (refused s candidate) :=
 ⟨valid.1,True.intro⟩

theorem tick_valid (valid : Valid s) : Valid (tick s member principal) := by
 have candidate := OwnerStatementSession.tick_valid (s:=s.session) (member:=member) (principal:=principal) valid
 unfold tick
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

theorem transfer_valid (valid : Valid s) (accepted : transfer s = some next) : Valid next := by
 unfold transfer at accepted
 cases run : MixedOwnerContinuation.transfer s.session with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   exact OwnerStatementSession.transfer_valid valid run

theorem service_valid (valid : Valid s) : Valid (service s ops) := by
 rcases service_route (s:=s) (ops:=ops) with same | ⟨_,_,_,_,_,_,same⟩
 · rw [same]; exact valid
 · change OwnerStatementSession.Valid (service s ops).session
   rw [same]
   exact valid

theorem metadata_valid (valid : Valid s)
 (accepted : changeMetadata s member place principal site change = some next) : Valid next := by
 unfold changeMetadata at accepted
 cases run : OwnerMetadataSource.changeMetadata s.session.state.source s.registry member place principal site change with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨source,registry⟩ := pair
   simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
   subst next
   exact valid

theorem authority_valid (valid : Valid s) (accepted : authorityHead s view = some next) : Valid next := by
 unfold authorityHead at accepted
 cases run : MixedOwnerContinuation.authorityHead s.session view with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   exact OwnerStatementSession.authority_valid valid run

theorem control_valid (valid : Valid s)
 (accepted : controlInput s member place principal raw = some (next,key)) : Valid next := by
 unfold controlInput at accepted
 cases run : MixedOwnerContinuation.controlInput s.session member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨session,created⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   exact OwnerStatementSession.control_valid valid run

theorem continued_valid (accepted : continueWith s program = some next) : Valid next := by
 unfold continueWith at accepted
 cases run : MixedOwnerContinuation.continueWith s.session program with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   exact OwnerStatementSession.continued_cursor run

theorem replaced_valid (accepted : replaceResidual s program = some next) : Valid next := by
 unfold replaceResidual at accepted
 cases run : MixedOwnerContinuation.replaceResidual s.session program with
 | none => simp [run] at accepted
 | some session =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   subst next
   exact OwnerStatementSession.replaced_cursor run

theorem attach_valid (valid : OwnerStatementSession.Valid session)
 (accepted : attach session registry addresses = some initial) : Valid initial := by
 unfold attach at accepted
 split at accepted
 · simp only [Option.some.injEq] at accepted
   subst initial
   exact valid
 · cases accepted

-- Every current metadata-attached Session entry, including refusal that keeps
-- the prior cursor instead of publishing the tentative owner request.
theorem rooted_cursor (initialValid : Valid initial) (path : Rooted initial s) : Valid s := by
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
 | cancel _ ih => exact OwnerStatementSession.cancel_valid ih

#print axioms refused_valid
#print axioms tick_valid
#print axioms transfer_valid
#print axioms service_valid
#print axioms metadata_valid
#print axioms authority_valid
#print axioms control_valid
#print axioms continued_valid
#print axioms replaced_valid
#print axioms attach_valid
#print axioms rooted_cursor
end MirroreaProofFirst.OwnerStatementMetadataCursor
