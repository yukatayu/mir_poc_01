import MirroreaProofFirstSharedHostJoin
import MirroreaProofFirstWorkOccurrence
import MirroreaProofFirstSourceFundingWork
import MirroreaProofFirstOwnerCommitJournal
import MirroreaProofFirstCohortCommitJournal
namespace MirroreaProofFirst.OwnerLeasePrefix
open OwnerCommitJournal

def localState (memory : Memory p a) : OwnerCreditCustody.Local :=
  ⟨memory.data.credits,memory.lease,memory.entered⟩

theorem send_matches_existing : maySend memory command = OwnerCreditCustody.maySend (localState memory) command := by
  cases memory.lease <;> rfl

def slotAvailable (memory : Memory p a) (capacity : Nat) (ticket : InvocationBoundary.Ticket) : Bool :=
  match memory.data.keys with
  | none => false
  | some keys => decide (OwnerOccurrence.key ticket ∉ keys) && decide (keys.length < capacity)

def ClaimAllowed (memory : Memory p a) (capacity : Nat) (ticket : InvocationBoundary.Ticket) : Prop :=
  memory.lease = none ∧ 2 ≤ memory.data.credits ∧
  ∃ keys, memory.data.keys = some keys ∧ OwnerOccurrence.key ticket ∉ keys ∧ keys.length < capacity

-- Exact selected writer claim guard, including actual retained reservation keys
-- and frozen capacity. Claim does not issue native reserve or source enter.
def claim (memory : Memory p a) (capacity : Nat) (ticket : InvocationBoundary.Ticket) : Option (Memory p a) :=
  if memory.lease.isNone && decide (2 ≤ memory.data.credits) && slotAvailable memory capacity ticket then
    some {memory with lease:=some ticket,entered:=false}
  else none

theorem claim_guard_exact :
    (memory.lease.isNone && decide (2 ≤ memory.data.credits) && slotAvailable memory capacity ticket) = true ↔
    ClaimAllowed memory capacity ticket := by
  cases keys : memory.data.keys <;> simp [slotAvailable,ClaimAllowed,keys,and_assoc]

theorem claim_exact : claim memory capacity ticket = some next ↔
    ClaimAllowed memory capacity ticket ∧ next = {memory with lease:=some ticket,entered:=false} := by
  unfold claim
  split
  · rename_i allowed
    simp [claim_guard_exact.mp allowed,eq_comm]
  · rename_i refused
    have no : ¬ClaimAllowed memory capacity ticket := fun yes => refused (claim_guard_exact.mpr yes)
    simp [no]

theorem claim_preserves (made : claim memory capacity ticket = some next) :
    next.data = memory.data ∧ OwnerCreditCustody.LocalInvariant (localState next) := by
  obtain ⟨allowed,rfl⟩ := claim_exact.mp made
  exact ⟨rfl,by simp [localState,OwnerCreditCustody.LocalInvariant,allowed.2.1]⟩

theorem claim_no_second (made : claim memory capacity ticket = some next) :
    claim next capacity other = none := by
  obtain ⟨_,rfl⟩ := claim_exact.mp made
  simp [claim]

-- This is the local notification check only. Its origin MUST be bound to the
-- actual matching successful source-enter reply in the surrounding native path.
def entered (memory : Memory p a) (ticket : InvocationBoundary.Ticket) : Option (Memory p a) :=
  if memory.lease == some ticket && !memory.entered then some {memory with entered:=true} else none

theorem entered_exact : entered memory ticket = some next ↔
    memory.lease = some ticket ∧ memory.entered = false ∧ next = {memory with entered:=true} := by
  unfold entered
  split
  · rename_i yes
    have guard : memory.lease = some ticket ∧ memory.entered = false := by simpa using yes
    simp [guard,eq_comm]
  · rename_i no
    have guard : ¬(memory.lease = some ticket ∧ memory.entered = false) := by simpa using no
    constructor
    · intro impossible; cases impossible
    · rintro ⟨held,inactive,_⟩; exact False.elim (guard ⟨held,inactive⟩)

theorem entered_preserves (valid : OwnerCreditCustody.LocalInvariant (localState memory))
    (notified : entered memory ticket = some next) :
    next.data = memory.data ∧ OwnerCreditCustody.LocalInvariant (localState next) := by
  obtain ⟨held,_,rfl⟩ := entered_exact.mp notified
  refine ⟨rfl,?_⟩
  have funded := valid.1 (by simp [localState,held])
  simpa [localState,OwnerCreditCustody.LocalInvariant,held] using funded

def cancel (memory : Memory p a) : Option (Memory p a) :=
  if memory.entered then none else some {memory with lease:=none}

theorem cancel_preserves (cancelled : cancel memory = some next) :
    next.data = memory.data ∧ OwnerCreditCustody.LocalInvariant (localState next) := by
  unfold cancel at cancelled
  split at cancelled
  · cases cancelled
  · rename_i inactive
    cases Option.some.inj cancelled
    exact ⟨rfl,by simp [localState,OwnerCreditCustody.LocalInvariant,inactive]⟩

theorem active_no_cancel (active : memory.entered = true) : cancel memory = none := by simp [cancel,active]

-- The stable lease predicate is deliberately false at a real store prefix.
-- This general counterexample prevents reusing an atomic stable invariant at
-- every assignment. The outer held gate/phase must protect this private state.
theorem release_prefix_unstable (active : memory.entered = true) :
    ¬OwnerCreditCustody.LocalInvariant (localState (write memory .releaseLease)) := by
  intro stable
  have impossible := stable.2 (by simpa [localState,write] using active)
  simp [localState,write] at impossible

theorem release_prefix_local_send : maySend (write memory .releaseLease) command = true := rfl

theorem release_clear_path (memory : Memory p a) :
    Runs ⟨memory,[.releaseLease,.clearEntered],false⟩
      ⟨write (write memory .releaseLease) .clearEntered,[],false⟩ := discharge memory _

theorem released_stable :
    OwnerCreditCustody.LocalInvariant (localState (write (write memory .releaseLease) .clearEntered)) := by
  simp [OwnerCreditCustody.LocalInvariant,localState,write]

theorem completed_stable
    (binding : before.data = project state)
    (ran : OwnerEndpointBudget.transition assigned scope capacity state command = (next,reply))
    (path : Runs ⟨before,stores before command reply,false⟩ ⟨after,[],stopped⟩) :
    OwnerCreditCustody.LocalInvariant (localState after) := by
  obtain ⟨_,lease,entered⟩ := completed_exact binding ran path
  simp [OwnerCreditCustody.LocalInvariant,localState,lease,entered]

theorem held_gate_excludes_entry (held : outer.gate = true) :
    CohortCommitJournal.advance outer (.enter callId) = none := by
  rcases outer with ⟨memory,pending,call,gate,retired⟩
  cases gate <;> simp_all
  cases pending <;> cases call <;> cases retired <;> rfl

#print axioms send_matches_existing
#print axioms claim_guard_exact
#print axioms claim_exact
#print axioms claim_preserves
#print axioms claim_no_second
#print axioms entered_exact
#print axioms entered_preserves
#print axioms cancel_preserves
#print axioms active_no_cancel
#print axioms release_prefix_unstable
#print axioms release_prefix_local_send
#print axioms release_clear_path
#print axioms released_stable
#print axioms completed_stable
#print axioms held_gate_excludes_entry

-- Source replies are derived from the same full native history. In particular
-- cancellation is justified by a known semantic refusal, never by entered=false.
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

theorem known_reply (checked : SharedWireLifetime.known before request bytes = some result) :
    bytes = SharedWireLifetime.reply before request := by
  unfold SharedWireLifetime.known at checked
  split at checked
  · cases checked
  · split at checked
    · assumption
    · cases checked

theorem history_reply {before after : SharedWireLifetime.History base}
    (checked : SharedWireLifetime.History.knownStep before request bytes = some after) :
    bytes = SharedWireLifetime.reply before.current request := by
  unfold SharedWireLifetime.History.knownStep at checked
  split at checked
  · cases checked
  · rename_i result known
    exact known_reply known

theorem source_physics {before after : SharedWireLifetime.History base}
    (checked : SharedWireLifetime.History.knownStep before (.source (.inr (.inr request))) bytes = some after) :
    (SharedWireLifetime.native after.current).driver =
      (SourceFundingInput.execute assigned scope bootstrap (SharedWireLifetime.native before.current).driver request).1 ∧
    (SharedWireLifetime.native after.current).owners = (SharedWireLifetime.native before.current).owners ∧
    (SharedWireLifetime.native after.current).ordinal = (SharedWireLifetime.native before.current).ordinal+1 := by
  have native := SharedHostJoin.history_native checked
  rw [native]
  exact ⟨rfl,rfl,rfl⟩

theorem source_reply {before after : SharedWireLifetime.History base}
    (checked : SharedWireLifetime.History.knownStep before (.source (.inr (.inr request))) bytes = some after) :
    bytes = OwnerPacketCodec.encode (PublicationCapacityDriver.reply p a)
      ((SourceFundingInput.execute assigned scope bootstrap (SharedWireLifetime.native before.current).driver request).2,
       (SharedWireLifetime.native after.current).driver.source.map SourcePublicationWorker.project) := by
  rw [history_reply checked,(source_physics checked).1]
  rfl

theorem source_refused_frames {before after : SharedWireLifetime.History base}
    (checked : SharedWireLifetime.History.knownStep before (.source (.inr (.inr request))) bytes = some after)
    (refused : (SourceFundingInput.execute assigned scope bootstrap
      (SharedWireLifetime.native before.current).driver request).2 ≠ .accepted) :
    (SharedWireLifetime.native after.current).driver = (SharedWireLifetime.native before.current).driver ∧
    (SharedWireLifetime.native after.current).owners = (SharedWireLifetime.native before.current).owners := by
  have valid := ((SharedFundedDriver.invariant before.current) _ rfl).1
  exact ⟨(source_physics checked).1.trans (SourceFundingWork.known_refusal_frames valid refused),
    (source_physics checked).2.1⟩

#print axioms known_reply
#print axioms history_reply
#print axioms source_physics
#print axioms source_reply
#print axioms source_refused_frames

-- A successful wire entry derives the dispatch from the actual pre-source
-- waiting ticket. The local lease flag is not a premise of this derivation.
theorem source_accepted_dispatch {before after : SharedWireLifetime.History base}
    {target : Fin p} {vector : Vector (Fin 513) p} {source : PublicationInput.State p a}
    {ticket : InvocationBoundary.Ticket}
    (checked : SharedWireLifetime.History.knownStep before
      (.source (.inr (.inr (vector,.step (.enter target))))) bytes = some after)
    (present : (SharedWireLifetime.native before.current).driver.source = some source)
    (waiting : (SourcePublicationWorker.project source).source.pending = some ticket)
    (accepted : (SourceFundingInput.execute assigned scope bootstrap
      (SharedWireLifetime.native before.current).driver (vector,.step (.enter target))).2 = .accepted) :
    ∃ enteredSource,
      (SharedWireLifetime.native after.current).driver.source = some enteredSource ∧
      enteredSource.dispatch = some ⟨target,⟨scope,source.publication.barrier.installed target⟩,ticket⟩ := by
  have valid := ((SharedFundedDriver.invariant before.current) _ rfl).1
  have executed : SourceFundingInput.execute assigned scope bootstrap
      (SharedWireLifetime.native before.current).driver (vector,.step (.enter target)) =
      ((SharedWireLifetime.native after.current).driver,.accepted) := by
    apply Prod.ext
    · exact (source_physics checked).1.symm
    · exact accepted
  obtain ⟨enteredSource,atSource,semantic⟩ := WorkOccurrence.accepted_semantic valid present executed
  obtain ⟨_,saved,atWaiting,_,dispatch⟩ := PublicationInput.dispatch_created semantic
  have exactTicket : saved.entry.ticket = ticket := by
    simpa [SourcePublicationWorker.project,SourceWorker.project,atWaiting] using waiting
  exact ⟨enteredSource,atSource,by simpa only [exactTicket] using dispatch⟩

-- The same known bytes decode to this actual status and full typed projection;
-- a host-supplied success bit or projection equality is not an alternate input.
theorem source_decoded_reply {before after : SharedWireLifetime.History base}
    (checked : SharedWireLifetime.History.knownStep before (.source (.inr (.inr request))) bytes = some after) :
    OwnerPacketCodec.decode (PublicationCapacityDriver.reply p a) bytes = some
      ((SourceFundingInput.execute assigned scope bootstrap (SharedWireLifetime.native before.current).driver request).2,
       (SharedWireLifetime.native after.current).driver.source.map SourcePublicationWorker.project) := by
  rw [source_reply checked]
  exact OwnerPacketCodec.roundtrip _ _

#print axioms source_accepted_dispatch
#print axioms source_decoded_reply
end MirroreaProofFirst.OwnerLeasePrefix
