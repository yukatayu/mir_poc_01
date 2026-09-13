import MirroreaProofFirstReferenceCancellationBoundary
import MirroreaProofFirstReferenceExecutionControls

namespace MirroreaProofFirst.ReferenceCancellationControls
open ReferenceCancellationBoundary

def claim : CurrentUse.Claim := {ReferenceOwnerControls.ownerClaim with id := 80,predicate := 60,actions := [15],targets := [101]}
def policy : CurrentUse.Policy := ⟨90,1,0,.leaf ⟨4,60⟩⟩
-- Explicit environment grants for the second binding used by the deadline
-- control below. The original claims authorize binding0 only.
def secondOwner : CurrentUse.Claim := {ReferenceOwnerControls.ownerClaim with id := 61,targets := [1]}
def secondHold : CurrentUse.Claim := {ReferenceOwnerControls.holdClaim with id := 71,targets := [1]}
def supplied (s : ManagementEntry.System 3 1) : ManagementEntry.System 3 1 :=
  {s with
    controlPolicy := (fun action => if action = 15 then policy else s.controlPolicy action),
    view := {s.view with authority := {s.view.authority with issued := claim :: secondOwner :: secondHold :: s.view.authority.issued}}}
def pair := ReferenceExecutionControls.requested.map fun (m,t) => (supplied m.store.core.system,t.ticket)
def permitted := pair.bind fun (s,t) => (authorize s 2 t 0 2 7 102).map fun permit => (s,t,permit)
#guard pair.isSome
#guard permitted.isSome
#guard (permitted.map fun (s,t,permit) => check s 2 t 0 2 7 102 permit) = some true
#guard (permitted.map fun (s,t,permit) => check s 3 t 0 2 7 102 permit) = some false
#guard (permitted.map fun (s,t,permit) => check s 2 {t with argument := 3} 0 2 7 102 permit) = some false
#guard (permitted.map fun (s,t,permit) => check s 2 {t with arithmeticProfile := 2} 0 2 7 102 permit) = some false
#guard (permitted.map fun (s,t,permit) => check s 2 {t with contractTheoryVersion := 2} 0 2 7 102 permit) = some false
#guard (permitted.map fun (s,t,permit) => check s 2 {t with id := 103} 0 2 7 102 permit) = some false
#guard (permitted.map fun (s,t,permit) => check {s with serial := s.serial+1} 2 t 0 2 7 102 permit) = some false
#guard (permitted.map fun (s,t,permit) => check s 2 t 0 2 8 102 permit) = some false
#guard (permitted.map fun (s,t,permit) => check s 2 t 0 0 7 102 permit) = some false

def revoked (s : ManagementEntry.System 3 1) (ids : List Nat) :=
  {s with view := {s.view with authority := {s.view.authority with revoked := ids}}}
-- Current cancellation is independent of invocation, reference, holding and
-- release permission. No proof or old invocation ticket manufactures claim80.
#guard (pair.bind fun (s,t) => authorize (revoked s [80]) 2 t 0 2 7 102).isNone
#guard (pair.bind fun (s,t) => authorize (revoked s [10,40,60,70]) 2 t 0 2 7 102).isSome
#guard (ReferenceExecutionControls.requested.bind fun (m,t) => authorize m.store.core.system 2 t.ticket 0 2 7 102).isNone
#guard (pair.bind fun (s,t) => authorize {s with view := {s.view with members := fun k => {s.view.members k with incarnation := 2}}} 2 t 0 2 7 102).isNone

-- Component initial authority fixture, followed only by actual engine entries.
def initial : ReferenceExecution.Machine 3 1 :=
  let old := ReferenceExecutionControls.initial
  let core := {old.store.core with system := supplied old.store.core.system}
  {old with store := {old.store with core := core,history := [ReferenceStore.frame core]}}
def requested := (ReferenceExecution.acquire initial 0 2 7 100 FallbackStaticControls.chain).bind fun (m,key) =>
  ReferenceExecution.startReference m 0 2 7 101 key 2
def cancelled := requested.bind fun (m,t) => (ReferenceExecution.cancel m 0 2 7 102 t).map fun next => (next,t)
#guard requested.isSome
#guard cancelled.isSome
#guard (cancelled.map fun (m,_) => m.pending.length) = some 0
#guard (cancelled.map fun (m,_) => m.store.core.pending.length) = some 0
#guard (cancelled.map fun (m,_) => m.store.core.system.used.map CurrentUse.UseId.request) = some [101,102,100]
#guard (cancelled.bind fun (m,t) => ReferenceExecution.resume m t).isNone
#guard (cancelled.bind fun (m,t) => ReferenceExecution.cancel m 0 2 7 103 t).isNone
#guard (cancelled.bind fun (m,_) => ReferenceExecution.startReference m 0 2 7 101 0 2).isNone
#guard (cancelled.bind fun (m,_) => ReferenceExecution.startReference m 0 2 7 102 0 2).isNone
#guard (cancelled.bind fun (m,_) => (ReferenceExecution.startReference m 0 2 7 103 0 2).bind fun (m,t) => ReferenceExecution.resume m t).map Prod.snd = some 5
#guard (cancelled.map fun (m,_) => m.store.bindings.length) = some 1
#guard (cancelled.map fun (m,_) => match m.store.events.head? with | some (.cancellation _) => true | _ => false) = some true
#guard (cancelled.map fun (m,_) => match m.store.core.events.head? with | some (.cancelled _) => true | _ => false) = some true
#guard (requested.bind fun (m,t) => ReferenceExecution.cancel m 0 2 7 101 t).isNone
#guard (requested.bind fun (m,t) => ReferenceExecution.cancel m 0 2 7 100 t).isNone
#guard (requested.bind fun (m,t) => ReferenceExecution.cancel m 0 2 8 102 t).isNone
#guard (requested.bind fun (m,t) => ReferenceExecution.cancel m 0 0 7 102 t).isNone
#guard (requested.bind fun (m,t) => ReferenceExecution.cancel m 0 2 7 102 {t with binding := none}).isNone
#guard (requested.bind fun (m,t) => ReferenceExecution.cancel (ReferenceExecutionControls.head m [80]) 0 2 7 102 t).isNone
#guard (requested.bind fun (m,t) => ReferenceExecution.cancel (ReferenceExecutionControls.head m [10,40,60,70]) 0 2 7 102 t).isSome
#guard (requested.bind fun (m,t) => (ReferenceExecution.release m 0 2 7 102 0).bind fun next => ReferenceExecution.cancel next 0 2 7 103 t).isSome
#guard (requested.bind fun (m,t) => (ReferenceExecution.reacquire m 0 2 7 102 0).bind fun next => ReferenceExecution.cancel next 0 2 7 103 t).isSome
#guard (requested.bind fun (m,t) => (ReferenceExecution.resume m t).bind fun (next,_) => ReferenceExecution.cancel next 0 2 7 102 t).isNone
-- Cancelling one ticket preserves the other pending payload and current rows.
#guard (requested.bind fun (m,t) => (ReferenceExecution.startPlain m 0 2 7 102 1 2).bind fun (m,other) =>
  (ReferenceExecution.cancel m 0 2 7 103 t).map fun next => next.pending == [other]) = some true
#guard (requested.bind fun (m,t) => (ReferenceCancellation.authorize m.store.core.system m.store.core.events.length m.store.events.length t 0 2 7 102).map fun permit =>
  ReferenceCancellation.check m.store.core.system m.store.core.events.length m.store.events.length {t with binding := none} 0 2 7 102 permit) = some false
-- Exact serial boundary: records survive C, their deadline need not.
def twoRequests := requested.bind fun (m,first) =>
  let shortChain := {FallbackStaticControls.chain with options := FallbackStaticControls.chain.options.map (fun option => {option with leaseUntil := m.store.core.system.serial+2})}
  (ReferenceExecution.acquire m 0 2 7 103 shortChain).bind fun (m,key) =>
    (ReferenceExecution.startReference m 0 2 7 104 key 2).map fun (m,other) => (m,first,other)
#guard twoRequests.isSome
#guard (twoRequests.map fun (m,_,other) => ReferenceExecution.protectionCheck m.store other) = some true
#guard (twoRequests.bind fun (m,_,other) => ReferenceExecution.resume m other).isSome
def expiredByCancel := twoRequests.bind fun (m,first,other) =>
  (ReferenceExecution.cancel m 0 2 7 105 first).map fun next => (m,next,other)
#guard expiredByCancel.isSome
#guard (expiredByCancel.map fun (old,next,other) => next.pending == [other] && next.store.bindings == old.store.bindings) = some true
#guard (expiredByCancel.map fun (_,next,other) => ReferenceExecution.protectionCheck next.store other) = some false
#guard (expiredByCancel.bind fun (_,next,other) => ReferenceExecution.resume next other).isNone
-- Reacquisition changes lineage, not the absolute deadlines of this chain.
#guard (expiredByCancel.bind fun (_,next,_) => ReferenceExecution.reacquire next 0 2 7 106 1).isNone

end MirroreaProofFirst.ReferenceCancellationControls
