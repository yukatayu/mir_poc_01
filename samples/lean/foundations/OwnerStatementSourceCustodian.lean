import OwnerStatementLiveCustodian
import OwnerStatementCompletion
namespace MirroreaProofFirst.OwnerStatementSourceCustodian
open OwnerStatementLiveCustodian
namespace Bank
export OwnerStatementRegistrySelection (tick transfer continueWith Rooted)
end Bank

-- This is the direct source-driver composition on the existing owning state.
-- A free slot may prepare a source step, but cannot bypass a waiting owner's
-- actual typed acknowledgment. Transfer publishes its real issued request.
def progress (s : State p a) (member : Fin a) (principal : Nat) : Option (State p a) :=
 if s.held = none ∧ MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting = none then
 some {s with live := Bank.tick s.live member principal} else none

def transfer (s : State p a) : Option (State p a) := do
 if s.held.isSome then none else do
 let live ← Bank.transfer s.live
 return {s with live := live}

-- New intentional invocation, only through the existing fully-drained source
-- continuation. Preserve the actual owner, history, counters and registry.
def invokeAgain (s : State p a) : Option (State p a) := do
 if s.held.isSome then none else do
 let live ← Bank.continueWith s.live s.live.session.program
 return {s with live := live}

theorem invokeAgain_exact : invokeAgain s = some next ↔ s.held = none ∧
 ∃ live, Bank.continueWith s.live s.live.session.program = some live ∧ next = {s with live := live} := by
 unfold invokeAgain
 cases held : s.held with
 | some held => simp
 | none =>
   simp only [Option.isSome_none,Bool.false_eq_true,↓reduceIte,Option.bind_eq_bind]
   cases run : Bank.continueWith s.live s.live.session.program <;> simp [eq_comm]

theorem invokeAgain_keeps (run : invokeAgain s = some next) :
 next.live.session.state = s.live.session.state ∧
 next.live.session.activation = s.live.session.activation+1 ∧
 next.nextSerial = s.nextSerial ∧ next.dispatched = s.dispatched := by
 obtain ⟨_,live,continued,rfl⟩ := invokeAgain_exact.mp run
 unfold OwnerStatementRegistrySelection.continueWith at continued
 cases step : MixedOwnerContinuation.continueWith s.live.session s.live.session.program with
 | none => simp [step] at continued
 | some session =>
   simp only [step,Option.map_some,Option.some.injEq] at continued
   subst live
   have kept := MixedOwnerContinuation.continued_keeps step
   exact ⟨kept.1,kept.2.1,rfl,rfl⟩

def count (s : MixedOwnerContinuation.Session p a) : Nat :=
 (OwnerStatementPosition.writes s.cursor.completed).length

-- Consuming a report is not mere custody release. It performs the original
-- typed source tick and publishes only an actual one-write accepted prefix.
-- This is a pure candidate check; it performs no extra owner effect.
def consume (s : State p a) (ticket : Ticket) (member : Fin a) (principal : Nat) : Option (State p a) := do
 if ticket.designation ≠ s.designation ∨ s.held ≠ some ⟨ticket,.reported⟩ then none else do
 let waiting ← MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting
 if waiting.saved ≠ ticket.saved then none else
 let live := Bank.tick s.live member principal
 if count live.session = count s.live.session + 1 then some {s with live := live,held := none} else none

def Consumed (s next : State p a) (ticket : Ticket) (member : Fin a) (principal : Nat) : Prop :=
 ticket.designation = s.designation ∧ s.held = some ⟨ticket,.reported⟩ ∧
 ∃ waiting, MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting = some waiting ∧
 waiting.saved = ticket.saved ∧
 count (Bank.tick s.live member principal).session = count s.live.session + 1 ∧
 next = {s with live := Bank.tick s.live member principal,held := none}

theorem progress_exact : progress s member principal = some next ↔
 s.held = none ∧ MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting = none ∧
 next = {s with live := Bank.tick s.live member principal} := by
 unfold progress
 split <;> simp_all [eq_comm]

theorem transfer_exact : transfer s = some next ↔ s.held = none ∧
 ∃ live, Bank.transfer s.live = some live ∧ next = {s with live := live} := by
 unfold transfer
 cases held : s.held with
 | some held => simp
 | none =>
   simp only [Option.isSome_none,Bool.false_eq_true,↓reduceIte,Option.bind_eq_bind]
   cases run : Bank.transfer s.live <;> simp [eq_comm]

theorem consume_exact : consume s ticket member principal = some next ↔ Consumed s next ticket member principal := by
 unfold consume Consumed
 by_cases designation : ticket.designation = s.designation
 · by_cases held : s.held = some ⟨ticket,.reported⟩
   · simp only [designation,held,ne_eq,not_true_eq_false,or_self,↓reduceIte,Option.bind_eq_bind,true_and]
     cases waiting : MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting with
     | none => simp
     | some value =>
       simp only [Option.bind_some,Option.some.injEq]
       by_cases same : value.saved = ticket.saved
       · simp [same,eq_comm]
       · simp [same]
   · simp [designation,held]
 · simp [designation]

theorem bank_tick_route :
 (Bank.tick s member principal).session = MixedOwnerContinuation.tick s.session member principal ∨
 (Bank.tick s member principal).session.cursor = s.session.cursor := by
 unfold OwnerStatementRegistrySelection.tick
 dsimp only
 split
 · exact Or.inl rfl
 · split
   · exact Or.inr rfl
   · rcases OwnerStatementCompletion.metadata_tick_route (s:=OwnerStatementRegistrySelection.view s _) (member:=member) (principal:=principal) with same | refused
     · exact Or.inl same
     · rw [refused]; exact Or.inr rfl

theorem bank_tick_delta (facts : OwnerStatementRegistryInvariant.Facts s.session) :
 OwnerStatementPosition.writes (Bank.tick s member principal).session.cursor.completed =
 OwnerStatementPosition.writes s.session.cursor.completed ∨
 OwnerStatementCompletion.Accepted s.session (Bank.tick s member principal).session := by
 rcases bank_tick_route (s:=s) (member:=member) (principal:=principal) with same | unchanged
 · rw [same]; exact OwnerStatementCompletion.tick_delta facts.binding facts.joint.2.2.2
 · exact Or.inl (by rw [unchanged])

-- Actual accepted completion carries original stopped statement/full saved
-- request, consumed inbox reply, real owner history, and unchanged owner.
theorem consume_sound (facts : OwnerStatementRegistryInvariant.Facts s.live.session)
 (run : consume s ticket member principal = some next) :
 OwnerStatementCompletion.Accepted s.live.session next.live.session := by
 obtain ⟨_,_,_,_,_,growth,rfl⟩ := consume_exact.mp run
 rcases bank_tick_delta (s:=s.live) (member:=member) (principal:=principal) facts with unchanged | accepted
 · have equal := congrArg List.length unchanged
   simp only [count] at growth
   omega
 · exact accepted

theorem consume_complete (designation : ticket.designation = s.designation)
 (held : s.held = some ⟨ticket,.reported⟩)
 (waitingHeld : MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting = some waiting)
 (same : waiting.saved = ticket.saved)
 (original : OwnerStatementCompletion.Accepted s.live.session (Bank.tick s.live member principal).session) :
 consume s ticket member principal = some {s with live := Bank.tick s.live member principal,held := none} := by
 obtain ⟨_,_,projection⟩ := OwnerStatementCompletion.accepted_projection original
 apply consume_exact.mpr
 refine ⟨designation,held,_,waitingHeld,same,?_,rfl⟩
 simp [count,projection]

theorem consumed_prefix (facts : OwnerStatementRegistryInvariant.Facts s.live.session)
 (run : consume s ticket member principal = some next) :
 ∃ ordinal source, OwnerStatementPosition.writes next.live.session.cursor.completed =
 OwnerStatementPosition.writes s.live.session.cursor.completed ++ [(ordinal,source)] :=
 OwnerStatementCompletion.accepted_projection (consume_sound facts run)

theorem no_unreported_consume (held : s.held = some ⟨ticket,.unreported⟩) :
 consume s other member principal = none := by
 simp [consume,held,Held.mk.injEq]

theorem no_repeat_consume (run : consume s ticket member principal = some next) :
 consume next other member principal = none := by
 obtain ⟨_,_,_,_,_,_,rfl⟩ := consume_exact.mp run
 simp [consume]

-- Only source progress, real request transfer, guarded original service,
-- reporting and typed completion are enabled by this static invocation cut.
-- Initial schema/authentic activation/control mutation and recovery are NOT
-- silently granted by this path. Existing bank reachability is retained.
inductive Path (capacity : Nat) (initial : State p a) : State p a → Prop where
 | initial : Path capacity initial initial
 | progress : Path capacity initial s → progress s member principal = some next → Path capacity initial next
 | transfer : Path capacity initial s → transfer s = some next → Path capacity initial next
 | stage : Path capacity initial s → stage capacity s = some (next,ticket) → Path capacity initial next
 | execute : Path capacity initial s → execute s ticket ops = some next → Path capacity initial next
 | report : Path capacity initial s → report s ticket = some next → Path capacity initial next
 | consume : Path capacity initial s → consume s ticket member principal = some next → Path capacity initial next
 | invokeAgain : Path capacity initial s → invokeAgain s = some next → Path capacity initial next

theorem path_trans (before : Path capacity initial middle) (after : Path capacity middle final) : Path capacity initial final := by
 induction after with
 | initial => exact before
 | progress _ run ih => exact .progress ih run
 | transfer _ run ih => exact .transfer ih run
 | stage _ run ih => exact .stage ih run
 | execute _ run ih => exact .execute ih run
 | report _ run ih => exact .report ih run
 | consume _ run ih => exact .consume ih run
 | invokeAgain _ run ih => exact .invokeAgain ih run

theorem path_refines (path : Path capacity initial s) : Bank.Rooted initial.live s.live := by
 induction path with
 | initial => exact .initial
 | progress _ run ih => obtain ⟨_,_,rfl⟩ := progress_exact.mp run; exact .tick ih
 | transfer _ run ih => obtain ⟨_,_,transferred,rfl⟩ := transfer_exact.mp run; exact .transfer ih transferred
 | stage _ run ih => obtain ⟨_,_,_,_,_,rfl⟩ := stage_exact.mp run; exact ih
 | execute _ run ih => obtain ⟨_,rfl⟩ := execute_exact.mp run; exact .service ih
 | report _ run ih => obtain ⟨_,rfl⟩ := report_exact.mp run; exact ih
 | consume _ run ih => obtain ⟨_,_,_,_,_,_,rfl⟩ := consume_exact.mp run; exact .tick ih
 | invokeAgain _ run ih => obtain ⟨_,_,continued,rfl⟩ := invokeAgain_exact.mp run; exact .continued ih continued

theorem path_admitted (initialAdmitted : OwnerStatementRegistryInvariant.Admitted realm authority policy store initial.live)
 (path : Path capacity initial s) : OwnerStatementRegistryInvariant.Admitted realm authority policy store s.live := by
 obtain ⟨origin,entered,prior⟩ := initialAdmitted
 exact ⟨origin,entered,OwnerStatementRegistrySelection.rooted_trans prior (path_refines path)⟩

theorem consume_bounded (bounded : Bounded capacity s)
 (run : consume s ticket member principal = some next) : Bounded capacity next := by
 obtain ⟨_,held,_,_,_,_,rfl⟩ := consume_exact.mp run
 simpa only [Bounded,reserved,held] using bounded

theorem consume_serials (valid : Serials s)
 (run : consume s ticket member principal = some next) : Serials next := by
 obtain ⟨_,_,_,_,_,_,rfl⟩ := consume_exact.mp run
 exact ⟨valid.1,valid.2.1,by intro held impossible; cases impossible⟩

theorem path_bounded (initialBounded : Bounded capacity initial) (path : Path capacity initial s) : Bounded capacity s := by
 induction path with
 | initial => exact initialBounded
 | progress _ run ih => obtain ⟨_,_,rfl⟩ := progress_exact.mp run; exact ih
 | transfer _ run ih => obtain ⟨_,_,_,rfl⟩ := transfer_exact.mp run; exact ih
 | stage _ run ih => exact stage_bounded ih run
 | execute _ run ih => exact execute_bounded ih run
 | report _ run ih => exact report_bounded ih run
 | consume _ run ih => exact consume_bounded ih run
 | invokeAgain _ run ih => obtain ⟨_,_,_,rfl⟩ := invokeAgain_exact.mp run; exact ih

theorem path_serials (initialValid : Serials initial) (path : Path capacity initial s) : Serials s := by
 induction path with
 | initial => exact initialValid
 | progress _ run ih => obtain ⟨_,_,rfl⟩ := progress_exact.mp run; exact ih
 | transfer _ run ih => obtain ⟨_,_,_,rfl⟩ := transfer_exact.mp run; exact ih
 | stage _ run ih => exact stage_serials ih run
 | execute _ run ih => exact execute_serials ih run
 | report _ run ih => exact report_serials ih run
 | consume _ run ih => exact consume_serials ih run
 | invokeAgain _ run ih => obtain ⟨_,_,_,rfl⟩ := invokeAgain_exact.mp run; exact ih

#print axioms invokeAgain_exact
#print axioms invokeAgain_keeps
#print axioms consume_exact
#print axioms consume_sound
#print axioms consume_complete
#print axioms path_refines
#print axioms path_admitted
#print axioms path_bounded
#print axioms path_serials
end MirroreaProofFirst.OwnerStatementSourceCustodian
