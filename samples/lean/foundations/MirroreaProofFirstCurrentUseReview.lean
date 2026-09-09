import MirroreaProofFirstCurrentUse
open MirroreaProofFirst.CurrentUse
open MirroreaProofFirst.CurrentUse.Controls
namespace ReviewControls
-- Fixed countermodels to stronger interpretations, not general proofs.
def revised (i : Fin 4) (inc rev : Nat) : World 4 :=
  { world with records := fun k => if k = i then
    { world.records k with identity := { (world.records k).identity with incarnation := inc, revision := rev } }
    else world.records k }
def refreshed (s : World 4) : UseRequest 4 :=
  { request with member := ⟨7, 0, (s.records 0).identity⟩, locus := ⟨7, 1, (s.records 1).identity⟩, moduleHandle := ⟨7, 2, (s.records 2).identity⟩, operation := ⟨7, 3, (s.records 3).identity⟩ }
def freshCheck (s : World 4) (u : UseRequest 4) : Bool :=
  match authorize s.authority (s.policies u.operation.key) (currentContext s u) with
  | none => false
  | some e => checkUse s u e
#guard ([0,1,2,3] : List (Fin 4)).map (fun i => checkUse (revised i 1 1) request evidence) = [false,false,false,false]
#guard ([0,1,2,3] : List (Fin 4)).map (fun i => checkUse (revised i 1 1) (refreshed (revised i 1 1)) evidence) = [true,true,true,false]
#guard ([0,1,2,3] : List (Fin 4)).map (fun i => freshCheck (revised i 2 0) (refreshed (revised i 2 0))) = [false,true,true,true]
#guard ((revised 1 2 0).records 3).identity = (world.records 3).identity
#guard freshCheck recodedWorld request
#guard freshCheck world { request with arguments := [.boolean true] }
-- Global issued inventory is not caller possession/supplied-claim selection.
#guard (authorize auth policy (currentContext world request)).isSome
#guard (authorize { auth with issued := [] } policy (currentContext world request)).isNone
#guard (authorize { auth with issued := [{ claimA with targets := [2] }, { claimB with targets := [2] }] }
  policy (currentContext world request)).isNone

def orPolicy : Policy := { policy with expression := .either (.leaf ⟨10,11⟩) (.leaf ⟨20,22⟩) }
def leftEvidence : Evidence := { evidence with witness := .left (.leaf claimA) }
def rightEvidence : Evidence := { evidence with witness := .right (.leaf claimB) }
#guard revalidate auth orPolicy evidence.context rightEvidence
#guard produce auth evidence.context 0 orPolicy.expression = some leftEvidence.witness
#guard !revalidate { auth with revoked := [1] } orPolicy evidence.context leftEvidence
#guard produce { auth with revoked := [1] } evidence.context 0 orPolicy.expression = some rightEvidence.witness
#guard revalidate { auth with revoked := [1] } orPolicy evidence.context rightEvidence
-- An unchanged unused branch cannot establish immutable policy-version meaning.
#guard revalidate auth { orPolicy with expression := .either (.leaf ⟨10,11⟩) (.leaf ⟨99,99⟩) }
  evidence.context leftEvidence
-- Duplicate identifiers alias revocation even when exact issued records differ.
def aliasedB : Claim := { claimB with id := 1 }
def aliasedAuth : Authority := { auth with issued := [claimA, aliasedB] }
#guard (produce aliasedAuth evidence.context 0 policy.expression).isSome
#guard (produce { aliasedAuth with revoked := [1] } evidence.context 0 policy.expression).isNone

def admitted : Machine 4 := tryAdmit machine request evidence
def rejoined : Machine 4 := { admitted with world := freshWorld }
#guard (tryAdmit rejoined freshMemberRequest freshEvidence).decisions.length = 1
#guard (tryAdmit rejoined { freshMemberRequest with request := 10 }
  { freshEvidence with context := { freshEvidence.context with request := 10 } }).decisions.length = 2
#guard (tryAdmit admitted { request with arguments := [.boolean true] }
  { evidence with context := { evidence.context with arguments := [.boolean true] } }).decisions.length = 1
#guard (revoke admitted 2).decisions.length = 1
#guard !checkUse (revoke admitted 2).world request evidence
-- Uniqueness alone authenticates no initial history.
def fabricated : Machine 4 := ⟨{ world with authority := { auth with issued := [] } }, [⟨evidence.context⟩]⟩
example : UniqueDecisions fabricated := by simp [UniqueDecisions, fabricated]
#guard (run fabricated []).decisions.length = 1
#guard !checkUse fabricated.world request evidence
-- Generation alone is not the current support stamp.
#guard (retire machine 1).world.generation = world.generation
#guard !checkUse (retire machine 1).world request evidence
end ReviewControls
