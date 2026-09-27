import MirroreaProofFirstSourceInput
import ParsedQualified

namespace MirroreaProofFirst.SourceInput.Controls

def seed : Bootstrap 1 :=
  ⟨⟨91,0,Vector.ofFn ReferenceSourceControls.view.members,
      AuthorityImage.capture ReferenceSourceControls.authority,
      ⟨InvocationBoundary.Controls.invocationPolicy,[]⟩⟩,
    ⟨ManagementEntry.Controls.policy,
      [(15,ReferenceSourceControls.policy 15),(14,ReferenceSourceControls.policy 14),
       (13,ReferenceSourceControls.policy 13),(12,ReferenceSourceControls.policy 12),
       (11,ReferenceSourceControls.policy 11),(10,ReferenceSourceControls.policy 10),
       (9,ReferenceSourceControls.policy 9)]⟩⟩
def assigned : Assignment 3 1 := ⟨91,⟨0,0,7⟩⟩

-- Exact finite rule meaning, not a finite collection of example checks.
theorem controls_same (action : Nat) : lookupRules seed.control action = ReferenceSourceControls.policy action := by
  by_cases h15 : action = 15
  · subst action; rfl
  by_cases h14 : action = 14
  · subst action; rfl
  by_cases h13 : action = 13
  · subst action; rfl
  by_cases h12 : action = 12
  · subst action; rfl
  by_cases h11 : action = 11
  · subst action; rfl
  by_cases h10 : action = 10
  · subst action; rfl
  by_cases h9 : action = 9
  · subst action; rfl
  simp [lookupRules,seed,ReferenceSourceControls.policy,List.find?_cons,eq_comm,h15,h14,h13,h12,h11,h10,h9]

def step (s : QualifiedCustody.State 3 1) (command : Command 3 1) : IO (QualifiedCustody.State 3 1) := do
  let bytes := OwnerPacketCodec.encode (input 3 1) (.step command)
  let some decoded := OwnerPacketCodec.decodeAt (input 3 1) 256 bytes |
    throw (IO.userError "source input bytes")
  let (some next,true) := transition assigned seed (some s) decoded |
    throw (IO.userError "source input step rejected")
  return next

def check : IO Unit := do
  let seedBytes := OwnerPacketCodec.encode (bootstrap 1) seed
  let some decodedSeed := OwnerPacketCodec.decodeAt (bootstrap 1) 256 seedBytes |
    throw (IO.userError "bootstrap byte roundtrip")
  let launchBytes := OwnerPacketCodec.encode (input 3 1) (.launch _root_.program)
  let some decoded := OwnerPacketCodec.decodeAt (input 3 1) 256 launchBytes |
    throw (IO.userError "actual source byte launch")
  let (some launched,true) := transition assigned decodedSeed none decoded |
    throw (IO.userError "actual source failed initial admission")
  unless !(transition assigned seed (some launched) decoded).2 do
    throw (IO.userError "populated source reinitialized")
  unless (launch {assigned with identity := ⟨2,0,7⟩} seed _root_.program).isNone &&
      (launch {assigned with realm := 92} seed _root_.program).isNone &&
      (launch {assigned with identity := ⟨0,0,8⟩} seed _root_.program).isNone do
    throw (IO.userError "assigned source identity realm or member lost")
  let mut current := launched
  for _ in [:8] do current ← step current .tick
  let waiting ← step current .tick
  let some saved := waiting.privateState.session.state.source.waiting |
    throw (IO.userError "actual source missing pending")
  unless (evaluate waiting .tick).isNone do throw (IO.userError "waiting source silently computed result")
  -- The old current-owner image has no rows at launch. Restoring its policies
  -- as the future source policy silently removes permission for later records.
  let emptyCaptured := OwnerImage.captureView (p:=3) (n:=0) (view seed.head)
  let oldFuture := (OwnerImage.restoreView emptyCaptured).policies 0 2
  unless oldFuture == WorldProjection.inactivePolicy &&
      oldFuture != (view seed.head).policies 0 2 do
    throw (IO.userError "current-image/future-policy counterexample missing")
  let left ← step current (.control 0 0 7 (.leave 0))
  let rejoined ← step left (.control 0 1 7 (.join 0))
  unless (evaluate rejoined .tick).isNone do throw (IO.userError "rejoined activation resumed")
  let cancelled ← step waiting .cancel
  let replacement ← step cancelled (.replace ⟨0,[]⟩)
  let continued ← step replacement (.continueWith ⟨0,[]⟩)
  unless continued.privateState.session.superseded.length == 2 do
    throw (IO.userError "replace/continue archive missing")
  let refreshed ← step waiting (.head {seed.head with generation := 1})
  unless (evaluate refreshed (.receive saved.entry.ticket 10)).isNone do
    throw (IO.userError "old authority-generation receipt was silently refreshed")
  current ← step waiting (.receive saved.entry.ticket 10)
  let mut modelReplies := 1
  for _ in [:16] do
    if current.privateState.session.remaining.isEmpty then break
    current ← step current .tick
    if current.privateState.session.status == .waiting then
      let some awaiting := current.privateState.session.state.source.waiting |
        throw (IO.userError "waiting source has no pending")
      let some result := OwnerProjection.serve
          (OwnerProjection.project current.privateState.session.state.source) 2 awaiting.entry.ticket |
        throw (IO.userError "model owner rejected")
      current ← step current (.receive awaiting.entry.ticket result)
      modelReplies := modelReplies+1
  unless current.privateState.session.completed == _root_.program.items && modelReplies == 4 &&
      current.privateState.session.state.source.writes.length == 17 do
    throw (IO.userError "actual source model did not complete")
  IO.FS.writeBinFile "source-bootstrap-actual.bin" ⟨seedBytes.toArray⟩
  IO.FS.writeBinFile "source-launch-actual.bin" ⟨launchBytes.toArray⟩
  IO.println s!"SOURCE_INPUT_MODEL_OK actual17source/4modelresults/all7entries; bootstrapBytes={seedBytes.length},launchBytes={launchBytes.length}; separate seed/identity, no populated reinit, exact future policies; no native source/transport/authentication guarantee"

#print axioms controls_same
#eval check
end MirroreaProofFirst.SourceInput.Controls
