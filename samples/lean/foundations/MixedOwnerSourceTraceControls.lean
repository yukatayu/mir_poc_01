import MixedOwnerSourceInvariant
import MixedOwnerCompletionControls
namespace MirroreaProofFirst.MixedOwnerSourceTraceControls
open MixedOwnerSourceTrace

-- Bounded checked-IR composition through the actual entries. Its initial
-- generated binding remains a supplied fixture, not a source deployment claim.
def initialStore (key : Nat) : Option Int := if key=0 then some 10 else none
def begun := MixedOwnerSourceIssueControls.calculated.map fun result => initial result.state initialStore
def requested := begun.bind fun s => issue s MixedOwnerSourceIssueControls.fields MixedOwnerSourceIssueControls.labels
  0 7 12 1 0 "generated_assignment" MixedOwnerSourceIssueControls.statement
def transferred := requested.bind transfer
def served := transferred.map (service (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata)
def delivered := served.bind receive
#guard requested.isSome && transferred.isSome && served.isSome && delivered.isSome
#guard (transferred.map serviceReady) = some true
#guard (served.map fun s => s.owner.store 0) = some (some 15)
#guard (served.map fun s => (s.owner.history.length,s.owner.attempts.length,s.owner.queued.isNone,s.inbox.length)) = some (1,1,true,1)
#guard (served.map fun s => s.owner.history.map fun w => (w.oldValue,w.value,w.reads)) = some [(some 10,15,[(0,10)])]
#guard (delivered.map fun s => (s.source.waiting.isNone,s.source.machine.store.core.pending.isEmpty,s.source.nextRequest,s.inbox.isEmpty)) = some (true,true,8,true)
#guard (delivered.map fun s => (s.source.writes.length,s.owner.history.length,s.owner.attempts.length)) = some (1,1,1)
#guard (served.map fun s => (service (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata s).owner.history.length) = some 1
#guard (served.bind fun s => s.owner.history.head?.bind fun w => MixedOwnerAttemptQueue.enqueue s.owner w.pending).isNone
#guard (delivered.bind receive).isNone

-- ACTUAL commit first, then a checked head successor, then refused old ack.
-- Preserve the real owner store/history and terminal attempt independently.
def changed := served.bind fun s => authorityHead s
 {s.source.machine.store.core.system.view with generation := s.source.machine.store.core.system.view.generation+1}
#guard changed.isSome
#guard (changed.bind receive).isNone
#guard (changed.map fun s => (s.owner.store 0,s.owner.history.length,s.owner.attempts.length,s.inbox.length,s.source.waiting.isSome)) = some (some 15,1,1,1,true)
#guard (changed.map fun s => (service (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata s).owner.store 0) = some (some 15)
#guard (changed.bind fun s => s.owner.history.head?.bind fun w => MixedOwnerAttemptQueue.enqueue s.owner w.pending).isNone
#guard (changed.bind fun s => issue s MixedOwnerSourceIssueControls.fields MixedOwnerSourceIssueControls.labels
 0 7 12 2 0 "generated_assignment" MixedOwnerSourceIssueControls.statement).isNone

-- Ready boundary refusal is not an attempted arithmetic/authority outcome.
def revoked := transferred.bind fun s => authorityHead s
 {s.source.machine.store.core.system.view with
   generation := s.source.machine.store.core.system.view.generation+1,
   authority := {s.source.machine.store.core.system.view.authority with
     revoked := 81::s.source.machine.store.core.system.view.authority.revoked}}
def refused := revoked.map (service (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata)
#guard revoked.isSome
#guard (refused.map fun s => (s.owner.store 0,s.owner.history.length,s.owner.attempts.length,s.owner.queued.isNone,s.inbox.isEmpty)) = some (some 10,0,1,true,true)
#guard (refused.map fun s => s.owner.attempts.map Prod.snd) = some [.refused .authority]
#guard (refused.bind fun s => s.owner.attempts.head?.bind fun row => MixedOwnerAttemptQueue.enqueue s.owner row.1).isNone

def noMetadata := transferred.map (service (FallibleFlow.signed 63) (fun _ => none))
#guard (noMetadata.map fun s => (s.owner.store 0,s.owner.history.length,s.owner.attempts.length,s.owner.queued.isSome,s.inbox.length)) = some (some 10,0,0,true,0)
def laterMetadata := noMetadata.map (service (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata)
#guard (laterMetadata.map fun s => (s.owner.store 0,s.owner.history.length,s.owner.attempts.length)) = some (some 15,1,1)

-- Arithmetic overflow is a terminal actual attempt, with no fabricated ack.
-- Store injection here is an explicit test input before service, not an admitted
-- migration or same-instance recovery path. The starting value is bounded i64.
def maximum : Int := 9223372036854775807
def overflowInput := transferred.map fun s => {s with owner := {s.owner with store := fun k => if k=0 then some maximum else none}}
def overflow := overflowInput.map (service (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata)
#guard (overflow.map fun s => (s.owner.store 0,s.owner.history.length,s.owner.attempts.length,s.owner.queued.isNone,s.inbox.isEmpty)) = some (some maximum,0,1,true,true)
#guard (overflow.map fun s => s.owner.attempts.map Prod.snd) = some [.refused .arithmetic]
#guard (overflow.bind fun s => s.owner.attempts.head?.bind fun row => MixedOwnerAttemptQueue.enqueue s.owner row.1).isNone

-- Raw queue substitution with plausible old authority is blocked by the exact
-- retained source/core payload guard, before any attempt or owner change.
def substituted := transferred.map fun s => {s with owner := {s.owner with queued := s.owner.queued.map fun (saved : OwnerSavedPending.Saved) =>
 {saved with origin := {saved.origin with controlLabel := 1}}}}
#guard (substituted.map serviceReady) = some false
#guard (substituted.map fun s => (service (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata s).owner.store 0) = some (some 10)
#guard (substituted.map fun s => (service (FallibleFlow.signed 63) MixedOwnerMaterializationControls.metadata s).owner.attempts.length) = some 0

-- The composed request really is in the inductive entry relation; provenance
-- theorems do not rely merely on the tested numeric answer being fifteen.
theorem requested_rooted (start : begun = some first) (run : requested = some next) :
 Rooted first.source initialStore next := by
 have firstInitial : first = initial first.source initialStore := by
   unfold begun at start
   cases computed : MixedOwnerSourceIssueControls.calculated with
   | none => simp [computed] at start
   | some result =>
     simp only [computed,Option.map_some,Option.some.injEq] at start
     subst first; rfl
 have issued : issue first MixedOwnerSourceIssueControls.fields MixedOwnerSourceIssueControls.labels
     0 7 12 1 0 "generated_assignment" MixedOwnerSourceIssueControls.statement = some next := by
   simpa [requested,start] using run
 exact .step (firstInitial ▸ .initial) (.issue issued)

theorem delivered_rooted (start : begun = some first) (run : delivered = some next) :
 Rooted first.source initialStore next := by
 unfold delivered served transferred at run
 cases requestedRun : requested with
 | none => simp [requestedRun] at run
 | some issued =>
   simp only [requestedRun,Option.bind_some] at run
   cases transferRun : transfer issued with
   | none => simp [transferRun] at run
   | some queued =>
     simp only [transferRun,Option.map_some,Option.bind_some] at run
     exact .step (.step (.step (requested_rooted start requestedRun) (.transfer transferRun)) .service) (.receive run)

#print axioms requested_rooted
#print axioms delivered_rooted
end MirroreaProofFirst.MixedOwnerSourceTraceControls
