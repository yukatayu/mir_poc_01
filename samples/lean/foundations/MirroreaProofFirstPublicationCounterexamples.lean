import MirroreaProofFirstPublicationControls
import MirroreaProofFirstPublicationPayload

namespace MirroreaProofFirst.PublicationCounterexamples
open Publication

-- Deliberately unsafe finite comparators, never admitted production transitions.
-- Each removes one exact guard/effect. These traces complement proof-script
-- mutation rejection and do not replace the arbitrary-history invariant.
inductive Fault where
  | unfrozenAck | missingParticipant | staleInstall | missingFence | wrongPublished | rejectUse

def faultyCheck (fault : Fault) (s : State 2) (action : Action 2) : Bool :=
  match fault,action with
  | .unfrozenAck,.acknowledge _ _ => true
  | .missingParticipant,.publish => decide (s.published < s.announced ∧ s.announced ≤ s.ack 0)
  | .staleInstall,.install _ r => s.certificates.contains r
  | .rejectUse,.use _ => false
  | _,_ => check s action

def faultyApply (fault : Fault) (s : State 2) (action : Action 2) : State 2 :=
  match fault,action with
  | .missingFence,.freeze i r => {s with frozen := (i,r) :: s.frozen}
  | .wrongPublished,.publish => {s with published := s.announced+1,certificates := s.announced :: s.certificates}
  | _,_ => apply s action

def faultyRun (fault : Fault) (actions : List (Action 2)) : Option (State 2) :=
  actions.foldl (fun state action => state.bind (fun s =>
    if faultyCheck fault s action then some (faultyApply fault s action) else none)) (some (initial 2 7))

def staleEnabled (state : Option (State 2)) (i : Fin 2) : Bool :=
  state.any (fun s => check s (.use i) && decide (s.installed i ≠ s.published))

def forgedAckTrace : List (Action 2) :=
  [.begin,.acknowledge 0 8,.acknowledge 1 8,.publish]
example : _root_.run (initial 2 7) forgedAckTrace = none := by decide
example : staleEnabled (faultyRun .unfrozenAck forgedAckTrace) 0 = true := by decide

def omittedParticipantTrace : List (Action 2) := [.begin,.freeze 0 8,.acknowledge 0 8,.publish]
example : _root_.run (initial 2 7) omittedParticipantTrace = none := by decide
example : staleEnabled (faultyRun .missingParticipant omittedParticipantTrace) 1 = true := by decide

def missingFenceTrace : List (Action 2) :=
  [.begin,.freeze 0 8,.freeze 1 8,.acknowledge 0 8,.acknowledge 1 8,.publish]
example : staleEnabled (_root_.run (initial 2 7) missingFenceTrace) 0 = false := by decide
example : staleEnabled (faultyRun .missingFence missingFenceTrace) 0 = true := by decide

def wrongPublicationTrace : List (Action 2) := missingFenceTrace ++ [.install 0 8]
example : staleEnabled (_root_.run (initial 2 7) wrongPublicationTrace) 0 = false := by decide
example : staleEnabled (faultyRun .wrongPublished wrongPublicationTrace) 0 = true := by decide

-- Removing the install floor alone admits a stale installation but does NOT
-- bypass the independent use fence. The concrete consequence is loss of an
-- otherwise enabled endpoint, not a claimed stale-use counterexample.
def oldInstallationTrace : List (Action 2) := _root_.round 8 ++ [.install 0 7]
example : _root_.run (initial 2 7) oldInstallationTrace = none := by decide
example : (faultyRun .staleInstall oldInstallationTrace).any
    (fun s => decide (s.installed 0 = 7 ∧ s.published = 8) && !check s (.use 0)) = true := by decide
example : (_root_.run (initial 2 7) (_root_.round 8)).any (fun s => check s (.use 0)) = true := by decide

example : (faultyRun .rejectUse (_root_.round 8)).isNone = true := by decide
example : (_root_.run (initial 2 7) (_root_.round 8)).isSome = true := by decide

-- A caller can save a validation before freezing and later act on that saved
-- value if it escapes the serialized gate. The ordinary gate model does not
-- authorize that detached continuation; its absence must be realized by code.
def detachedHeld : Nat := (initial 2 7).installed 0
example : (_root_.run (initial 2 7) (_root_.round 8)).any
    (fun s => decide (detachedHeld ≠ s.published)) = true := by decide

-- Equal numeric revisions do not detect forged or accidentally swapped image
-- content. This comparator breaks only cache origin, outside admitted actions.
def wrongPayload : PublicationPayload.State 2 Nat Unit :=
  {PublicationPayload.initial 2 7 10 with cached := fun _ => 99}
example : PublicationPayload.check (fun v (_ : Unit) => some (v+1)) wrongPayload (.use 0) = true := by decide
example : wrongPayload.cached 0 ≠ wrongPayload.current := by decide

#eval [staleEnabled (faultyRun .unfrozenAck forgedAckTrace) 0,
  staleEnabled (faultyRun .missingParticipant omittedParticipantTrace) 1,
  staleEnabled (faultyRun .missingFence missingFenceTrace) 0,
  staleEnabled (faultyRun .wrongPublished wrongPublicationTrace) 0]
end MirroreaProofFirst.PublicationCounterexamples
