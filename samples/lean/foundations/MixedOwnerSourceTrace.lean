import MixedOwnerSourceDeclaration
import MixedOwnerAttemptQueue
namespace MirroreaProofFirst.MixedOwnerSourceTrace

structure Reply where
 pending : OwnerSavedPending.Saved
 evidence : CurrentUse.Evidence
 deriving DecidableEq

def reply (written : MixedOwnerAttemptQueue.Record) : Reply := ⟨written.pending,written.serviceEvidence⟩

-- Distinct source issue, transfer, guarded owner service and caller ack steps.
-- ONE source/common Execution/catalog and one owner attempt log; no pure-side
-- second scheduler or independently supplied expected result. The mathematical
-- common catalog/view is not a requirement for a permanent physical coordinator.
structure State (p a : Nat) where
 source : MixedOwnerSourceIssue.State p a
 owner : MixedOwnerAttemptQueue.State
 outbox : List OwnerSavedPending.Saved
 inbox : List Reply

def issue (s : State p a) (ctx : MixedNamedOwnerSource.Fields) (labels : List (String × Nat))
 (member : Fin a) (principal activation ordinal control : Nat) (operationName : String)
 (statement : MixedNamedOwnerSource.Assignment) : Option (State p a) := do
 let (next,waiting) ← MixedOwnerSourceIssue.issue s.source ctx labels member principal activation ordinal control operationName statement
 return {s with source := next,outbox := s.outbox++[waiting.saved]}

def declareOwner (s : State p a) (ctx : MixedNamedOwnerSource.Fields) (labels : List (String × Nat))
 (control : Nat) (member : Fin a) (place : Fin p) (principal : Nat) (name : String)
 (source : MixedNamedOwnerSource.Assignment) : State p a :=
 {s with source := (MixedOwnerSourceDeclaration.declareOwner s.source ctx labels control member place principal name source).state}

def transfer (s : State p a) : Option (State p a) := do
 let saved::rest := s.outbox | none
 let owner ← MixedOwnerAttemptQueue.enqueue s.owner saved
 return {s with owner := owner,outbox := rest}

def serviceAdmitted (ops : FallibleFlow.Arithmetic) (current : Nat → Option (Nat × Nat)) (s : State p a) : State p a :=
 let (owner,result) := MixedOwnerAttemptQueue.advance ops s.source.machine.store.core.system.configuration.state
   s.source.machine.store.core.system.view current s.owner
 {s with owner := owner,inbox := match result with
   | some (.committed written) => s.inbox++[reply written]
   | _ => s.inbox}

-- Exact current payload custody is checked at the enclosing service entry.
-- This is independent of source elaboration, current authorization, arithmetic
-- and replay checks; none of those can substitute for this equality boundary.
def serviceReady (s : State p a) : Bool :=
 match MixedOwnerSourceIssue.ownerWaiting s.source.waiting,s.owner.queued with
 | some waiting,some saved => decide (waiting.saved = saved ∧ .owner saved ∈ s.source.machine.store.core.pending)
 | _,_ => false

def service (ops : FallibleFlow.Arithmetic) (current : Nat → Option (Nat × Nat)) (s : State p a) : State p a :=
 if serviceReady s then serviceAdmitted ops current s else s

theorem service_ready_exact : serviceReady s = true ↔
 ∃ waiting saved, s.source.waiting = some (.inr waiting) ∧ s.owner.queued = some saved ∧
 waiting.saved = saved ∧ .owner saved ∈ s.source.machine.store.core.pending := by
 cases sw : s.source.waiting with
 | none => simp [serviceReady,MixedOwnerSourceIssue.ownerWaiting,sw]
 | some held =>
   cases held with
   | inl pure => simp [serviceReady,MixedOwnerSourceIssue.ownerWaiting,sw]
   | inr waiting =>
     cases queued : s.owner.queued <;> simp [serviceReady,MixedOwnerSourceIssue.ownerWaiting,sw,queued]

def receive (s : State p a) : Option (State p a) := do
 let received::rest := s.inbox | none
 let source ← MixedOwnerSourceIssue.acknowledge s.source received.pending received.evidence
 return {s with source := source,inbox := rest}

def controlInput (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
 (raw : MixedCompositionCore.Raw) : Option (State p a × Option Nat) :=
 (MixedOwnerSourceIssue.controlInput s.source member place principal raw).map fun (source,key) => ({s with source := source},key)

def authorityHead (s : State p a) (view : WorldProjection.AuthorityView a) : Option (State p a) :=
 (MixedOwnerSourceIssue.authorityHead s.source view).map fun source => {s with source := source}

def initial (source : MixedOwnerSourceIssue.State p a) (store : Nat → Option Int) : State p a :=
 ⟨source,MixedOwnerAttemptQueue.initial store,[],[]⟩

-- This relation intentionally excludes arbitrary image/packet/raw-state import.
-- A later network/restore entry must prove the same carried-origin invariants.
-- It is not yet the entire typed Mir program/Session driver or receipt custody.
inductive Step : State p a → State p a → Prop where
 | issue {control : Nat} : issue s ctx labels member principal activation ordinal control operationName statement = some next → Step s next
 | declaration {control : Nat} : Step s (declareOwner s ctx labels control member place principal name source)
 | transfer : transfer s = some next → Step s next
 | service : Step s (service ops current s)
 | receive : receive s = some next → Step s next
 | pureAdvance : Step s {s with source := (MixedOwnerSourceIssue.advancePure s.source member place principal item).state}
 | pureComplete : Step s {s with source := (MixedOwnerSourceIssue.completePure s.source).state}
 | pureCancel : Step s {s with source := (MixedOwnerSourceIssue.cancelPure s.source member place principal).state}
 | control : controlInput s member place principal raw = some (next,key) → Step s next
 | authority : authorityHead s view = some next → Step s next
inductive Rooted (source : MixedOwnerSourceIssue.State p a) (store : Nat → Option Int) : State p a → Prop where
 | initial : Rooted source store (initial source store)
 | step : Rooted source store s → Step s next → Rooted source store next

theorem issue_parts {control : Nat} (accepted : issue s ctx labels member principal activation ordinal control operationName statement = some next) :
 ∃ source waiting, MixedOwnerSourceIssue.issue s.source ctx labels member principal activation ordinal control operationName statement = some (source,waiting) ∧
 next = {s with source := source,outbox := s.outbox++[waiting.saved]} := by
 unfold issue at accepted
 cases run : MixedOwnerSourceIssue.issue s.source ctx labels member principal activation ordinal control operationName statement with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨source,waiting⟩ := pair
   simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
   exact ⟨source,waiting,rfl,accepted.symm⟩

theorem transfer_parts (accepted : transfer s = some next) :
 ∃ saved rest owner, s.outbox = saved::rest ∧ MixedOwnerAttemptQueue.enqueue s.owner saved = some owner ∧
 next = {s with owner := owner,outbox := rest} := by
 unfold transfer at accepted
 cases box : s.outbox with
 | nil => simp [box] at accepted
 | cons saved rest =>
   simp only [box] at accepted
   cases run : MixedOwnerAttemptQueue.enqueue s.owner saved with
   | none => simp [run] at accepted
   | some owner =>
     simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
     exact ⟨saved,rest,owner,rfl,run,accepted.symm⟩

theorem receive_parts (accepted : receive s = some next) :
 ∃ received rest source, s.inbox = received::rest ∧
 MixedOwnerSourceIssue.acknowledge s.source received.pending received.evidence = some source ∧
 next = {s with source := source,inbox := rest} := by
 unfold receive at accepted
 cases box : s.inbox with
 | nil => simp [box] at accepted
 | cons received rest =>
   simp only [box] at accepted
   cases run : MixedOwnerSourceIssue.acknowledge s.source received.pending received.evidence with
   | none => simp [run] at accepted
   | some source =>
     simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
     exact ⟨received,rest,source,rfl,run,accepted.symm⟩

theorem control_parts (accepted : controlInput s member place principal raw = some (next,key)) :
 ∃ source, next = {s with source := source} := by
 unfold controlInput at accepted
 cases run : MixedOwnerSourceIssue.controlInput s.source member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨source,created⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   exact ⟨source,accepted.1.symm⟩

theorem authority_parts (accepted : authorityHead s view = some next) :
 ∃ source, next = {s with source := source} := by
 unfold authorityHead at accepted
 cases run : MixedOwnerSourceIssue.authorityHead s.source view with
 | none => simp [run] at accepted
 | some source =>
   simp only [run,Option.map_some,Option.some.injEq] at accepted
   exact ⟨source,accepted.symm⟩

def Issued (source : MixedOwnerSourceIssue.State p a) (store : Nat → Option Int) (saved : OwnerSavedPending.Saved) : Prop :=
 ∃ before ctx labels member principal activation ordinal control name statement next waiting,
 Rooted source store before ∧
 MixedOwnerSourceIssue.issue before.source ctx labels member principal activation ordinal control name statement = some (next,waiting) ∧ waiting.saved = saved

def Carried (source : MixedOwnerSourceIssue.State p a) (store : Nat → Option Int) (s : State p a) : Prop :=
 (∀ saved ∈ s.outbox, Issued source store saved) ∧
 (∀ saved, s.owner.queued = some saved → Issued source store saved) ∧
 (∀ saved result, (saved,result) ∈ s.owner.attempts → Issued source store saved)

theorem rooted_carried (path : Rooted source store s) : Carried source store s := by
 induction path with
 | initial => simp [Carried,initial,MixedOwnerAttemptQueue.initial]
 | @step s next prior step ih =>
   cases step with
   | issue accepted =>
     obtain ⟨after,waiting,run,rfl⟩ := issue_parts accepted
     refine ⟨?_,ih.2.1,ih.2.2⟩
     intro saved present
     rcases List.mem_append.mp present with old | fresh
     · exact ih.1 saved old
     · have same : saved = waiting.saved := List.mem_singleton.mp fresh
       exact ⟨s,_,_,_,_,_,_,_,_,_,after,waiting,prior,run,same.symm⟩
   | transfer accepted =>
     obtain ⟨saved,rest,owner,box,enqueued,rfl⟩ := transfer_parts accepted
     obtain ⟨_,_,rfl⟩ := MixedOwnerAttemptQueue.enqueue_exact.mp enqueued
     refine ⟨?_,?_,ih.2.2⟩
     · intro other present
       exact ih.1 other (by rw [box]; exact List.mem_cons_of_mem _ present)
     · intro other same
       cases same
       exact ih.1 saved (by simp [box])
   | @service ops current =>
     by_cases allowed : serviceReady s = true
     ·
       cases run : (MixedOwnerAttemptQueue.advance ops s.source.machine.store.core.system.configuration.state
         s.source.machine.store.core.system.view current s.owner).2 with
       | none =>
         have retained := MixedOwnerAttemptQueue.not_attempted_retains run
         simpa [service,allowed,serviceAdmitted,run,retained] using ih
       | some answer =>
         obtain ⟨saved,_,_,queued,_,_,_,empty,attempts,_⟩ := MixedOwnerAttemptQueue.advance_parts run
         refine ⟨by simpa only [service,allowed,ite_true,serviceAdmitted] using ih.1,?_,?_⟩
         · simp [service,allowed,serviceAdmitted,empty]
         · intro other result present
           simp only [service,allowed,ite_true,serviceAdmitted] at present
           change (other,result) ∈ (MixedOwnerAttemptQueue.advance ops _ _ current s.owner).1.attempts at present
           rw [attempts] at present
           rcases List.mem_append.mp present with old | fresh
           · exact ih.2.2 other result old
           · have same : other = saved := congrArg Prod.fst (List.mem_singleton.mp fresh)
             subst other
             exact ih.2.1 saved queued
     · simpa [service,allowed] using ih
   | receive accepted => obtain ⟨_,_,_,_,_,rfl⟩ := receive_parts accepted; exact ih
   | declaration => exact ih
   | pureAdvance => exact ih
   | pureComplete => exact ih
   | pureCancel => exact ih
   | control accepted => obtain ⟨_,rfl⟩ := control_parts accepted; exact ih
   | authority accepted => obtain ⟨_,rfl⟩ := authority_parts accepted; exact ih

def Provenance (s : State p a) : Prop :=
 ∀ received ∈ s.inbox, ∃ written ∈ s.owner.history, received = reply written

theorem service_inbox_parts (member : received ∈ (service ops current s).inbox) :
 received ∈ s.inbox ∨ ∃ written, serviceReady s = true ∧
 (MixedOwnerAttemptQueue.advance ops s.source.machine.store.core.system.configuration.state
   s.source.machine.store.core.system.view current s.owner).2 = some (.committed written) ∧
 received = reply written := by
 by_cases allowed : serviceReady s = true
 · cases run : (MixedOwnerAttemptQueue.advance ops s.source.machine.store.core.system.configuration.state
     s.source.machine.store.core.system.view current s.owner).2 with
   | none => exact Or.inl (by simpa [service,allowed,serviceAdmitted,run] using member)
   | some answer =>
     cases answer with
     | refused why => exact Or.inl (by simpa [service,allowed,serviceAdmitted,run] using member)
     | committed written =>
       have parts : received ∈ s.inbox ∨ received = reply written := by simpa [service,allowed,serviceAdmitted,run] using member
       exact parts.imp_right (fun equal => ⟨written,allowed,rfl,equal⟩)
 · exact Or.inl (by simpa [service,allowed] using member)

theorem service_provenance (valid : Provenance s) : Provenance (service ops current s) := by
 by_cases ready : serviceReady s = true
 ·
  intro received member
  rcases service_inbox_parts member with old | ⟨written,allowed,run,equal⟩
  · obtain ⟨written,present,equal⟩ := valid received old
    obtain ⟨suffix,history⟩ := MixedOwnerAttemptQueue.history_retained
      (ops:=ops) (catalog:=s.source.machine.store.core.system.configuration.state)
      (view:=s.source.machine.store.core.system.view) (current:=current) (s:=s.owner)
    refine ⟨written,?_,equal⟩
    simp only [service,ready,ite_true,serviceAdmitted]
    change written ∈ (MixedOwnerAttemptQueue.advance ops _ _ current s.owner).1.history
    rw [history]; exact List.mem_append_left _ present
  · obtain ⟨_,_,_,_,_,_,_,_,history⟩ := MixedOwnerAttemptQueue.actual_commit run
    refine ⟨written,?_,equal⟩
    simp only [service,ready,ite_true,serviceAdmitted]
    change written ∈ (MixedOwnerAttemptQueue.advance ops _ _ current s.owner).1.history
    rw [history]; simp
 · simpa [service,ready] using valid

theorem step_provenance (valid : Provenance s) (transition : Step s next) : Provenance next := by
 cases transition with
 | issue accepted => obtain ⟨_,_,_,rfl⟩ := issue_parts accepted; exact valid
 | transfer accepted =>
   obtain ⟨_,_,_,_,queued,rfl⟩ := transfer_parts accepted
   obtain ⟨_,_,rfl⟩ := MixedOwnerAttemptQueue.enqueue_exact.mp queued
   exact valid
 | service => exact service_provenance valid
 | receive accepted =>
   obtain ⟨head,rest,_,box,_,rfl⟩ := receive_parts accepted
   intro received member
   exact valid received (by rw [box]; exact List.mem_cons_of_mem head member)
 | declaration => exact valid
 | pureAdvance => exact valid
 | pureComplete => exact valid
 | pureCancel => exact valid
 | control accepted => obtain ⟨_,rfl⟩ := control_parts accepted; exact valid
 | authority accepted => obtain ⟨_,rfl⟩ := authority_parts accepted; exact valid

theorem rooted_provenance (path : Rooted source store s) : Provenance s := by
 induction path with
 | initial => simp [Provenance,initial]
 | step _ transition ih => exact step_provenance ih transition

-- This predicate names an ACTUAL guarded service at a rooted prefix. It is
-- derived below, never a Boolean admission oracle or an assumed history row.
def Generated (source : MixedOwnerSourceIssue.State p a) (store : Nat → Option Int)
 (received : Reply) : Prop :=
 ∃ before ops current written,
 Rooted source store before ∧ serviceReady before = true ∧
 (MixedOwnerAttemptQueue.advance ops before.source.machine.store.core.system.configuration.state
   before.source.machine.store.core.system.view current before.owner).2 = some (.committed written) ∧
 received = reply written

theorem rooted_generated (path : Rooted source store s) :
 ∀ received ∈ s.inbox, Generated source store received := by
 induction path with
 | initial => simp [initial]
 | @step s next prior transition ih =>
   cases transition with
   | issue accepted => obtain ⟨_,_,_,rfl⟩ := issue_parts accepted; exact ih
   | transfer accepted => obtain ⟨_,_,_,_,_,rfl⟩ := transfer_parts accepted; exact ih
   | @service ops current =>
     intro received member
     rcases service_inbox_parts member with old | ⟨written,allowed,run,equal⟩
     · exact ih received old
     · exact ⟨s,ops,current,written,prior,allowed,run,equal⟩
   | receive accepted =>
     obtain ⟨head,_,_,box,_,rfl⟩ := receive_parts accepted
     intro received member
     exact ih received (by rw [box]; exact List.mem_cons_of_mem head member)
   | declaration => exact ih
   | pureAdvance => exact ih
   | pureComplete => exact ih
   | pureCancel => exact ih
   | control accepted => obtain ⟨_,rfl⟩ := control_parts accepted; exact ih
   | authority accepted => obtain ⟨_,rfl⟩ := authority_parts accepted; exact ih

theorem accepted_receipt_has_actual_write (path : Rooted source store s)
 (accepted : receive s = some next) :
 ∃ written ∈ s.owner.history,
 MixedOwnerSourceIssue.acknowledge s.source written.pending written.serviceEvidence = some next.source ∧
 next.owner = s.owner ∧ Generated source store (reply written) ∧ Issued source store written.pending := by
 obtain ⟨received,rest,after,box,ack,rfl⟩ := receive_parts accepted
 have present : received ∈ s.inbox := by simp [box]
 obtain ⟨written,history,equal⟩ := rooted_provenance path received present
 have generated := rooted_generated path received present
 have actualGenerated : Generated source store (reply written) := equal ▸ generated
 have retainedGenerated := actualGenerated
 obtain ⟨before,ops,current,actual,prior,allowed,run,replyEq⟩ := actualGenerated
 obtain ⟨saved,_,_,queued,_,_,_,same,_⟩ := MixedOwnerAttemptQueue.actual_commit run
 have pendingEq : written.pending = actual.pending := congrArg Reply.pending replyEq
 have issued := (rooted_carried prior).2.1 saved queued
 refine ⟨written,history,?_,rfl,retainedGenerated,?_⟩
 · simpa only [equal,reply] using ack
 · rw [pendingEq,same]; exact issued

theorem step_owner_invariant (valid : MixedOwnerAttemptQueue.Invariant s.owner)
 (transition : Step s next) : MixedOwnerAttemptQueue.Invariant next.owner := by
 cases transition with
 | issue accepted => obtain ⟨_,_,_,rfl⟩ := issue_parts accepted; exact valid
 | transfer accepted =>
   obtain ⟨_,_,_,_,enqueued,rfl⟩ := transfer_parts accepted
   exact MixedOwnerAttemptQueue.enqueue_preserves valid enqueued
 | service =>
   unfold service
   split
   · exact MixedOwnerAttemptQueue.advance_preserves valid
   · exact valid
 | receive accepted => obtain ⟨_,_,_,_,_,rfl⟩ := receive_parts accepted; exact valid
 | declaration => exact valid
 | pureAdvance => exact valid
 | pureComplete => exact valid
 | pureCancel => exact valid
 | control accepted => obtain ⟨_,rfl⟩ := control_parts accepted; exact valid
 | authority accepted => obtain ⟨_,rfl⟩ := authority_parts accepted; exact valid

theorem rooted_owner_invariant (path : Rooted source store s) : MixedOwnerAttemptQueue.Invariant s.owner := by
 induction path with
 | initial => exact MixedOwnerAttemptQueue.initial_invariant
 | step _ transition ih => exact step_owner_invariant ih transition

-- A rejected acknowledgment is not an optional outer transaction which could
-- discard an already returned commit. The service state is retained, with its
-- completed attempt; this theorem claims no eventual continuation or repair.
theorem service_commit_retained (ready : serviceReady s = true)
 (committed : (MixedOwnerAttemptQueue.advance ops s.source.machine.store.core.system.configuration.state
   s.source.machine.store.core.system.view current s.owner).2 = some (.committed written)) :
 (service ops current s).owner.history = s.owner.history++[written] ∧
 (service ops current s).owner.queued = none ∧
 MixedOwnerAttemptQueue.enqueue (service ops current s).owner written.pending = none := by
 obtain ⟨saved,_,_,queued,_,_,_,same,history⟩ := MixedOwnerAttemptQueue.actual_commit committed
 obtain ⟨_,_,_,_,_,_,_,empty,_⟩ := MixedOwnerAttemptQueue.advance_parts committed
 simp only [service,ready,ite_true]
 exact ⟨history,empty,MixedOwnerAttemptQueue.completed_blocks_same_id committed queued (by rw [same])⟩

#print axioms service_ready_exact
#print axioms service_inbox_parts
#print axioms service_provenance
#print axioms rooted_provenance
#print axioms rooted_generated
#print axioms accepted_receipt_has_actual_write
#print axioms rooted_owner_invariant
#print axioms service_commit_retained
#print axioms issue_parts
#print axioms transfer_parts
#print axioms receive_parts
#print axioms rooted_carried
end MirroreaProofFirst.MixedOwnerSourceTrace
