import MirroreaProofFirstSourceCompletion

namespace MirroreaProofFirst.SavedInvocation
open InstanceState WorldProjection CompositionCore InvocationBoundary

theorem growth_preserves_saved (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (definition : Fin d) (owner : Nat) (placements : List (Fin p))
    (parent : Option (Fin n)) (dependencies : Support.Formula (Fin n))
    (checked : InvocationBoundary.check s v t = true) :
    InvocationBoundary.check (instantiate s definition owner placements parent dependencies) v t = true := by
  obtain ⟨member,key,place,atKey⟩ := check_parts _ _ _ checked
  have cur := checkAt_sound _ _ _ _ _ _ atKey
  have memberEq := cur.1
  have keyEq := cur.2.1
  have placeEq := cur.2.2.1
  have grownIndex : index (n+1) key.val = some (Growth.left n 1 key) := index_roundtrip (Growth.left n 1 key)
  simp only [InvocationBoundary.check,memberEq,keyEq,placeEq,index_roundtrip,grownIndex]
  rw [grown_checkAt]
  exact atKey

namespace Controls
open CompositionMachine
def initial : Machine 3 1 :=
  {CompositionMachine.Controls.initial with
    system := {CompositionMachine.Controls.initial.system with
      view := {InvocationBoundary.Controls.view with
        authority := {InvocationBoundary.Controls.view.authority with
          issued := [ManagementEntry.Controls.claim,InvocationBoundary.Controls.invocationClaim]}}}}
def pending := start initial 0 2 7 20 1 2
#guard pending.isSome
-- A real authorized operation extends the domain while the OLD ticket remains.
def added := pending.bind fun (m,t) =>
  (manage m 0 0 7 21 (.instantiate 0 7 [0,2] none .top)).map fun (next,_) => (next,t)
#guard (added.map fun (m,_) => m.system.configuration.count) = some 3
#guard (added.bind fun (m,t) => resume m t).map Prod.snd = some 5
#guard (pending.bind fun (m,t) =>
  (manage m 0 0 7 21 (.register InstancePrograms.Controls.replacement none)).bind fun (next,_) => resume next t).map Prod.snd = some 5
#guard (pending.bind fun (m,t) =>
  (manage m 0 0 7 21 (.retire 0)).bind fun (next,_) => resume next t).map Prod.snd = some 5
#guard (pending.bind fun (m,t) =>
  (manage m 0 0 7 21 (.replace 0 1)).bind fun (next,_) => resume next t).map Prod.snd = some 5
#guard (pending.bind fun (m,t) =>
  (manage m 0 0 7 21 (.replace 1 1)).bind fun (next,_) => resume next t).isNone
#guard (pending.bind fun (m,t) =>
  (manage m 0 0 7 21 (.retire 1)).bind fun (next,_) => resume next t).isNone

def alternative : CurrentUse.Claim := {InvocationBoundary.Controls.invocationClaim with id := 11,predicate := 24}
def twoBranches : Machine 3 1 :=
  {initial with system := {initial.system with view := {initial.system.view with
    authority := {initial.system.view.authority with issued := [InvocationBoundary.Controls.invocationClaim,alternative]}
    policies := fun _ _ => {InvocationBoundary.Controls.invocationPolicy with
      expression := .either (.leaf ⟨4,23⟩) (.leaf ⟨4,24⟩)}}}}
def branchPending := start twoBranches 0 2 7 20 1 2
#guard (branchPending.map fun (_,t) => t.evidence.witness) =
  some (.left (.leaf InvocationBoundary.Controls.invocationClaim))
def revokeLeft (m : Machine 3 1) := authorityHead m
  {m.system.view with authority := {m.system.view.authority with revoked := [10]}}
#guard (branchPending.bind fun (m,t) => resume (revokeLeft m) t).isNone
-- The later branch can authorize a NEW call, not repair the OLD submitted tree.
def freshRight := branchPending.bind fun (m,_) => start (revokeLeft m) 0 2 7 22 1 2
#guard (freshRight.map fun (_,t) => t.evidence.witness) = some (.right (.leaf alternative))
#guard (freshRight.bind fun (m,t) => resume m t).map Prod.snd = some 5
end Controls
#print axioms growth_preserves_saved
end MirroreaProofFirst.SavedInvocation
