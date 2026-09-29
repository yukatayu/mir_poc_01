import OwnerStatementProgram
namespace MirroreaProofFirst.OwnerStatementAdmission
open OwnerMetadataSession

-- A transition history is required; passing a set of state predicates is not
-- an admission certificate. Registry consistency is separately supplied, and
-- does not authenticate a schema or grant any invocation authority.
def Initial (realm : Nat) (view : WorldProjection.AuthorityView a)
 (policy : Nat → CurrentUse.Policy) (store : Nat → Option Int) (s : State p a) : Prop :=
 ∃ session registry addresses,
 MixedOwnerContinuation.Rooted realm view policy store session ∧
 OwnerMetadataRegistry.Consistent registry ∧ attach session registry addresses = some s

def Admitted (realm : Nat) (view : WorldProjection.AuthorityView a)
 (policy : Nat → CurrentUse.Policy) (store : Nat → Option Int) (s : State p a) : Prop :=
 ∃ initial, Initial realm view policy store initial ∧ Rooted initial s

def Facts (s : State p a) : Prop :=
 OwnerStatementProgram.Metadata.Compiled s ∧ OwnerStatementMetadataCursor.Valid s ∧
 OwnerStatementMetadataBinding.Valid s ∧ OwnerStatementMetadataHistory.Valid s ∧
 OwnerMetadataSessionInvariant.Valid s

theorem initial_facts (admitted : Initial realm view policy store s) : Facts s := by
 obtain ⟨session,registry,addresses,path,consistent,attached⟩ := admitted
 unfold attach at attached
 split at attached
 · simp only [Option.some.injEq] at attached
   subst s
   exact ⟨OwnerStatementProgram.rooted_compiled path,
     OwnerStatementSession.rooted_cursor path,OwnerStatementBinding.rooted_matches path,
     OwnerStatementHistory.rooted_history path, MixedOwnerContinuation.rooted_joint path,consistent⟩
 · cases attached

theorem admitted_facts (admitted : Admitted realm view policy store s) : Facts s := by
 obtain ⟨initial,entered,path⟩ := admitted
 obtain ⟨compiled,cursor,binding,history,joint⟩ := initial_facts entered
 exact ⟨OwnerStatementProgram.Metadata.rooted_compiled compiled path,
   OwnerStatementMetadataCursor.rooted_cursor cursor path,
   OwnerStatementMetadataBinding.rooted_binding binding path,
   OwnerStatementMetadataHistory.rooted_history history binding joint path,
   OwnerMetadataSessionInvariant.rooted_joint joint path⟩

theorem admitted_source_prefix (admitted : Admitted realm view policy store s) :
 (∃ suffix, OwnerStatementAcceptance.assignments s.session.program.items =
    OwnerStatementAcceptance.writes s.session.cursor.completed ++ suffix) ∧
 OwnerStatementHistory.Valid s.session := by
 obtain ⟨compiled,cursor,_,history,_⟩ := admitted_facts admitted
 obtain ⟨_,_,typed⟩ := compiled
 exact ⟨OwnerStatementAcceptance.write_prefix cursor.1 typed,history⟩

theorem admitted_owner_completion (admitted : Admitted realm view policy store s)
 (waiting : MixedOwnerSourceIssue.ownerWaiting s.session.state.source.waiting = some held)
 (ready : (MixedOwnerContinuation.complete s.session).status = .ready) :
 ∃ written ∈ s.session.state.owner.history, written.pending = held.saved :=
 (OwnerStatementAcceptance.owner_complete_committed
   (admitted_facts admitted).2.2.2.2.1.2.2.2 waiting ready).1

#print axioms initial_facts
#print axioms admitted_facts
#print axioms admitted_source_prefix
#print axioms admitted_owner_completion
end MirroreaProofFirst.OwnerStatementAdmission
