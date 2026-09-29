import OwnerStatementCursor
import MixedOwnerContinuation
namespace MirroreaProofFirst.OwnerStatementSession
open MixedOwnerContinuation

def Valid (s : Session p a) : Prop :=
 OwnerStatementCursor.Partition s.entries s.cursor ∧ OwnerStatementCursor.Shape s.status s.cursor

theorem tick_valid (valid : Valid s) : Valid (tick s member principal) := by
 exact ⟨OwnerStatementCursor.tick_partition valid.1 valid.2,OwnerStatementCursor.tick_shape valid.2⟩

theorem continued_cursor (accepted : continueWith s program = some next) :
 Valid next := by
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
       exact ⟨OwnerStatementCursor.initial_partition,OwnerStatementCursor.initial_shape⟩
 · cases accepted

theorem replaced_cursor (accepted : replaceResidual s program = some next) : Valid next := by
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
       exact ⟨OwnerStatementCursor.initial_partition,OwnerStatementCursor.initial_shape⟩
 · cases accepted

theorem transfer_valid (valid : Valid s) (accepted : transfer s = some next) : Valid next := by
 unfold transfer at accepted
 cases run : MixedOwnerSourceTrace.transfer s.state with
 | none => simp [run] at accepted
 | some state => simp only [run,Option.map_some,Option.some.injEq] at accepted; subst next; exact valid

theorem authority_valid (valid : Valid s) (accepted : authorityHead s view = some next) : Valid next := by
 unfold authorityHead at accepted
 cases run : MixedOwnerSourceTrace.authorityHead s.state view with
 | none => simp [run] at accepted
 | some state => simp only [run,Option.map_some,Option.some.injEq] at accepted; subst next; exact valid

theorem control_valid (valid : Valid s) (accepted : controlInput s member place principal raw = some (next,key)) : Valid next := by
 unfold controlInput at accepted
 cases run : MixedOwnerSourceTrace.controlInput s.state member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨state,key⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   exact valid

theorem cancel_valid (valid : Valid s) : Valid (cancel s member principal) := by
 simp only [cancel]
 split
 · exact ⟨valid.1,True.intro⟩
 · exact valid

theorem failed_tick_keeps (failed : s.status = .failed reason) : tick s member principal = s := by
 cases s
 simp_all [tick,MixedSourceCursor.tick,attemptedControl]

theorem cancel_keeps_owner : (cancel s member principal).state.owner = s.state.owner := by
 simp only [cancel]
 split <;> rfl

theorem continued_keeps_owner (accepted : continueWith s program = some next) :
 next.state.owner = s.state.owner := congrArg (fun state => state.owner) (continued_keeps accepted).1

theorem replaced_keeps_owner (accepted : replaceResidual s program = some next) :
 next.state.owner = s.state.owner := congrArg (fun state => state.owner) (replaced_keeps accepted).1

-- All existing admitted Session entry/mutator cases, not only successful tick.
-- Raw populated-image construction and concrete Rust persistence are excluded.
theorem rooted_cursor (path : Rooted realm view policy store s) : Valid s := by
 induction path with
 | launch accepted =>
   obtain ⟨_,_,_,rfl⟩ := launch_parts accepted
   exact ⟨OwnerStatementCursor.initial_partition,OwnerStatementCursor.initial_shape⟩
 | tick _ ih => exact tick_valid ih
 | transfer _ accepted ih => exact transfer_valid ih accepted
 | service _ ih => exact ih
 | authority _ accepted ih => exact authority_valid ih accepted
 | control _ accepted ih => exact control_valid ih accepted
 | cancel _ ih => exact cancel_valid ih
 | continued _ accepted _ => exact continued_cursor accepted
 | replaced _ accepted _ => exact replaced_cursor accepted

-- Existing actual modeled owner-history invariant and cursor claim coexist on
-- the SAME Session; an accepted source prefix is not the committed owner log.
theorem rooted_cursor_and_owner (path : Rooted realm view policy store s) :
 Valid s ∧ MixedOwnerAttemptQueue.Invariant s.state.owner ∧ MixedOwnerSourceTrace.Provenance s.state := by
 have joint := rooted_joint path
 exact ⟨rooted_cursor path,joint.2.2.1,joint.2.2.2⟩

#print axioms failed_tick_keeps
#print axioms cancel_keeps_owner
#print axioms continued_keeps_owner
#print axioms replaced_keeps_owner
#print axioms tick_valid
#print axioms continued_cursor
#print axioms replaced_cursor
#print axioms transfer_valid
#print axioms authority_valid
#print axioms control_valid
#print axioms cancel_valid
#print axioms rooted_cursor
#print axioms rooted_cursor_and_owner
end MirroreaProofFirst.OwnerStatementSession
