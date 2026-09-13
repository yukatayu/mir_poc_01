import MirroreaProofFirstReferenceExecution
import MirroreaProofFirstReferenceMutationControls

namespace MirroreaProofFirst.ReferenceExecutionControls
open ReferenceExecution

-- Component controls use the existing actual instance cut. Ordinary-source
-- creation and source continuation are a separate, still required consumer.
def initial : Machine 3 1 := ⟨ReferenceMutationControls.initial,[]⟩
def acquired := acquire initial 0 2 7 100 FallbackStaticControls.chain
def requested := acquired.bind fun (m,key) => startReference m 0 2 7 101 key 2
#guard acquired.isSome
#guard requested.isSome
#guard (requested.bind fun (m,t) => resume m t).map Prod.snd = some 5
#guard (requested.bind fun (m,t) => finish m t 6).isNone
#guard (requested.bind fun (m,t) => finishPlain m t.ticket 5).isNone
#guard (requested.bind fun (m,t) => finish m {t with binding := none} 5).isNone
#guard (requested.bind fun (m,t) => (resume m t).bind fun (m,_) => resume m t).isNone
#guard (requested.bind fun (m,_) => startPlain m 0 2 7 101 1 2).isNone
#guard (requested.bind fun (m,_) => release m 0 2 7 101 0).isNone

def head (m : Machine 3 1) (ids : List Nat) :=
  authorityHead m {m.store.core.system.view with authority := {m.store.core.system.view.authority with revoked := ids}}
def lost := requested.map fun (m,t) => (head m [40],t)
#guard (lost.map fun (m,t) => InvocationBoundary.resultCheck m.store.core.system.configuration.state m.store.core.system.view t.ticket 5) = some true
#guard (lost.bind fun (m,t) => resume m t).isNone
-- The old plain helper alone would wrongly accept. Actual wrapper rejects and
-- stores classification of all core pending, so caller erasure cannot bypass it.
#guard (lost.bind fun (m,t) => ReferenceStore.finishPlain m.store t.ticket 5).isSome
#guard (lost.bind fun (m,t) => finishPlain m t.ticket 5).isNone
def restored := lost.map fun (m,t) => (head m [],t)
#guard (restored.map fun (m,t) => t.binding.map fun binding => binding.selected.map fun choice =>
  ReferenceAccess.check m.store.core.system (m.store.core.system.controlPolicy 13)
    (ReferenceSelection.atIndex binding.request choice.index) choice.guard) = some (some (some true))
#guard (restored.bind fun (m,t) => resume m t).isNone
#guard (restored.map fun (m,t) => m.pending.contains t) = some true

def explicitReacquire := requested.bind fun (m,t) => (reacquire m 0 2 7 102 0).map fun next => (next,t)
#guard explicitReacquire.isSome
#guard (explicitReacquire.map fun (m,t) => InvocationBoundary.resultCheck m.store.core.system.configuration.state m.store.core.system.view t.ticket 5) = some true
#guard (explicitReacquire.bind fun (m,t) => resume m t).isNone
#guard (explicitReacquire.bind fun (m,_) => (startReference m 0 2 7 103 0 2).bind fun (m,t) => resume m t).map Prod.snd = some 5

def released := requested.bind fun (m,t) => (release m 0 2 7 102 0).map fun next => (next,t)
#guard released.isSome
#guard (released.bind fun (m,t) => resume m t).isNone

def degraded := lost.map fun (m,t) => ((normalize m 0 2 7 102 0).state,t)
#guard (degraded.bind fun (m,t) => resume m t).isNone
#guard (degraded.bind fun (m,_) => (startReference m 0 2 7 103 0 2).bind fun (m,t) => resume m t).map Prod.snd = some 5

def invokeLost := requested.map fun (m,t) => (head m [10],t)
#guard (invokeLost.map fun (m,t) => protectionCheck m.store t) = some true
#guard (invokeLost.bind fun (m,t) => resume m t).isNone

def plain := startPlain initial 0 2 7 100 1 2
#guard (plain.bind fun (m,t) => finishPlain m t.ticket 5).isSome
-- All entries act on the same pending/core stores. New requests after a
-- successful completion get a fresh id; the consumed id cannot be reused.
#guard (requested.bind fun (m,t) => (resume m t).bind fun (next,_) => startReference next 0 2 7 101 0 2).isNone
#guard (requested.bind fun (m,t) => (resume m t).bind fun (next,_) => startReference next 0 2 7 102 0 2).isSome
end MirroreaProofFirst.ReferenceExecutionControls
