import MirroreaProofFirstReferenceSource
import MirroreaProofFirstReferenceOwnerControls

namespace MirroreaProofFirst.ReferenceSourceControls
open ReferenceSourceData ReferenceSource

-- Explicit independently supplied authority for this finite test realm. Source
-- compilation cannot issue these claims. These are IR tests, not parser E2E.
def authority : CurrentUse.Authority :=
  {ManagementEntry.Controls.view.authority with issued := [ManagementEntry.Controls.claim,
    {InvocationBoundary.Controls.invocationClaim with targets := List.range 32},
    {ReferenceAccessControls.accessClaim with targets := [1]},
    {ReferenceAccessControls.accessClaim with id := 41,targets := [0]},
    ReferenceOwnerControls.ownerClaim,ReferenceOwnerControls.holdClaim,
    {ReferenceOwnerControls.ownerClaim with id := 80,predicate := 60,actions := [15],targets := List.range 128}]}
def view : WorldProjection.AuthorityView 1 :=
  {InvocationBoundary.Controls.view with authority := authority}
def policy (action : Nat) : CurrentUse.Policy :=
  if action = 15 then ⟨90,1,0,.leaf ⟨4,60⟩⟩ else
  if action = 14 then ReferenceOwnerControls.holdPolicy else
  if action = 13 then ReferenceAccessControls.accessPolicy else
  if action ∈ [9,10,11,12] then ReferenceOwnerControls.ownerPolicy else ManagementEntry.Controls.policy
def initial : State 3 1 := ReferenceSource.initial 91 view policy
def chain : ChainSyntax :=
  ⟨"base",[⟨"extraOption","extra",some "score",.read,100⟩,⟨"baseOption","base",some "score",.read,100⟩],
    [some ⟨"extraOption","baseOption",true⟩]⟩
def program : List Located :=
  [⟨⟨"reference-ir-control",10⟩,.plain (.register "code" {InstancePrograms.Controls.original with contract := ⟨[0,1,2,3],1,100⟩} none)⟩,
   ⟨⟨"reference-ir-control",20⟩,.plain (.instantiate "base" "code" [0,1,2] none .top)⟩,
   ⟨⟨"reference-ir-control",30⟩,.plain (.instantiate "extra" "code" [0,2] none .top)⟩,
   ⟨⟨"reference-ir-control",40⟩,.acquire "route" chain⟩,
   ⟨⟨"reference-ir-control",50⟩,.alias "alias" "route"⟩,
   ⟨⟨"reference-ir-control",60⟩,.plain (.localValue "count" true (.integer 2))⟩,
   ⟨⟨"reference-ir-control",70⟩,.plain (.assign "count" (.add (.read "count") (.integer 1)))⟩,
   ⟨⟨"reference-ir-control",80⟩,.plain (.invoke "first" "alias" (.read "count"))⟩]
def built := run initial 0 0 7 program
#guard built.status = .ready
#guard lookup built.state.values "first" = some (.plain (.integer 10 false))
#guard lookup built.state.values "route" = lookup built.state.values "alias"
#guard built.state.machine.store.bindings.length = 1
#guard built.state.writes.length = 8
#guard built.state.machine.store.core.system.configuration.count = 2
#guard built.state.waiting.isNone
def invocation : Located := ⟨⟨"reference-ir-control",90⟩,.plain (.invoke "second" "alias" (.integer 2))⟩
def requested := advance built.state 0 0 7 invocation
#guard requested.status = .waiting
#guard lookup requested.state.values "second" = none
#guard requested.state.writes.length = built.state.writes.length
#guard (complete requested.state).status = .ready
#guard lookup (complete requested.state).state.values "second" = some (.plain (.integer 5 false))

def head (s : State 3 1) (ids : List Nat) : State 3 1 :=
  authorityHead s {s.machine.store.core.system.view with authority := {s.machine.store.core.system.view.authority with revoked := ids}}
def lostWaiting := head requested.state [40]
#guard (complete lostWaiting).status = .failed .rejected
#guard (complete (head lostWaiting [])).status = .failed .rejected
#guard (complete lostWaiting).state.writes = requested.state.writes
#guard (complete lostWaiting).state.machine.pending = requested.state.machine.pending
#guard (advance requested.state 0 0 7 invocation).status = .failed .awaiting
#guard (run requested.state 0 0 7 []).status = .waiting

-- Static error precedes normalization: an integer argument cannot read a
-- reference, and a rejected static statement consumes no id or binding event.
def lost := head built.state [40]
def illTyped : Located := ⟨⟨"reference-ir-control",91⟩,.plain (.invoke "bad" "alias" (.read "route"))⟩
def illTypedResult := advance lost 0 0 7 illTyped
#guard illTypedResult.status = .failed .staticType
#guard illTypedResult.state.nextRequest = lost.nextRequest
#guard illTypedResult.state.machine.store.bindings = lost.machine.store.bindings
#guard illTypedResult.state.origins = lost.origins

-- Here normalization genuinely commits, then invocation authority fails.
-- State/id/origin survive the failed SOURCE statement; no output write appears.
def bothLost := head built.state [40,10]
def failedCall := advance bothLost 0 0 7 invocation
#guard failedCall.status = .failed .rejected
#guard failedCall.state.nextRequest = bothLost.nextRequest+1
#guard failedCall.state.machine.store.events.length = bothLost.machine.store.events.length+1
#guard failedCall.state.writes = bothLost.writes
#guard lookup failedCall.state.values "second" = none
#guard (failedCall.state.origins.head?).map Origin.kind = some .normalize
#guard ((ReferenceMutation.lookup failedCall.state.machine.store 0).bind fun b => b.selected.map ReferenceSelection.Choice.index) = some 1
def exhausted := advance (head built.state [40,41]) 0 0 7 invocation
#guard exhausted.status = .failed (.normalization .exhausted)
#guard exhausted.state.nextRequest = built.state.nextRequest+1
#guard exhausted.state.writes = built.state.writes

def reacquired := advance built.state 0 0 7 ⟨⟨"reference-ir-control",100⟩,.reacquire "fresh" "alias"⟩
#guard reacquired.status = .ready
#guard lookup reacquired.state.values "alias" = lookup built.state.values "alias"
#guard ((ReferenceMutation.lookup reacquired.state.machine.store 0).map fun b => b.request.epoch) = some 1
def released := advance reacquired.state 0 0 7 ⟨⟨"reference-ir-control",110⟩,.release "done" "route"⟩
#guard released.status = .ready
#guard (ReferenceMutation.lookup released.state.machine.store 0).isNone
#guard (advance released.state 0 0 7 invocation).status = .failed (.normalization .absent)

-- Losing only B does NOT remove a reader still participating at A/C. This
-- positive control prevents accidental all-placement interpretation of support.
def readerLost := controlInput requested.state 0 0 7 (.leave 1)
#guard readerLost.isSome
#guard (readerLost.map fun (s,_) => (complete s).status) = some .ready
def readerRecovered := readerLost.bind fun (s,_) => controlInput s 0 0 7 (.join 1)
#guard readerRecovered.isSome
#guard (readerRecovered.map fun (s,_) => (complete s).status) = some .ready
#guard (readerRecovered.map fun (s,_) => s.waiting) = some requested.state.waiting

-- The source explicitly constructs a B-only holder with base as its lifetime
-- ancestor. Now B exit really invalidates the reader, while selected extra at A
-- remains usable for a plain invocation. No example-specific runtime branch.
def holderProgram : List Located := program.take 3 ++
  [⟨⟨"reference-ir-control",35⟩,.plain (.instantiate "holder" "code" [1] (some "base") .top)⟩,
   ⟨⟨"reference-ir-control",40⟩,.acquire "route" {chain with reader := "holder"}⟩,⟨⟨"reference-ir-control",50⟩,.alias "alias" "route"⟩]
def holderBuilt := run initial 0 0 7 holderProgram
def holderWaiting := advance holderBuilt.state 0 0 7 invocation
#guard holderBuilt.status = .ready
#guard holderWaiting.status = .waiting
def holderLost := controlInput holderWaiting.state 0 0 7 (.leave 1)
#guard holderLost.isSome
#guard (holderLost.map fun (s,_) => (complete s).status) = some (.failed .rejected)
def holderRecovered := holderLost.bind fun (s,_) => controlInput s 0 0 7 (.join 1)
#guard holderRecovered.isSome
#guard (holderRecovered.map fun (s,_) => (complete s).status) = some (.failed .rejected)

-- Unrelated real domain growth can preserve a pending call when its saved
-- permission/lease remains valid; blanket invalidation is not the profile.
def registeredWhileWaiting := controlInput requested.state 0 0 7
  (.register {InstancePrograms.Controls.original with contract := ⟨[0,1,2,3],1,100⟩} none)
def grewWhileWaiting := registeredWhileWaiting.bind fun (s,created) =>
  created.bind fun key => controlInput s 0 0 7 (.instantiate key 7 [0] none .top)
#guard grewWhileWaiting.isSome
#guard (grewWhileWaiting.map fun (s,_) => s.machine.store.core.system.configuration.count) = some 3
#guard (grewWhileWaiting.map fun (s,_) => (complete s).status) = some .ready
#guard (grewWhileWaiting.map fun (s,_) => lookup (complete s).state.values "second") = some (some (.plain (.integer 5 false)))
-- Oracle F1: an arbitrary helper Plan is not checked source execution.
-- The contrast is actual execution, not a fabricated expected trace. The
-- admitted advance path is tied to the independently proved elaboration.
def referencePermissionLost := head built.state [40,41]
def forgedPlanCall := executePlan referencePermissionLost 0 0 7 invocation
  (.call "second" (.instance 1) 2)
#guard forgedPlanCall.status = .waiting
#guard (complete forgedPlanCall.state).status = .ready
#guard lookup (complete forgedPlanCall.state).state.values "second" = some (.plain (.integer 5 false))
#guard (advance referencePermissionLost 0 0 7 invocation).status = .failed (.normalization .exhausted)
#guard lookup (advance referencePermissionLost 0 0 7 invocation).state.values "second" = none

-- Sentinel projection cannot make reference names fresh, or usable as instance,
-- definition, parent/support, or integer names through checked source entry.
#guard (advance built.state 0 0 7 ⟨⟨"reference-ir-control",120⟩,.plain (.localValue "route" false (.integer 1))⟩).status = .failed .staticType
#guard (advance built.state 0 0 7 ⟨⟨"reference-ir-control",121⟩,.plain (.retire "badRetire" "route")⟩).status = .failed .staticType
#guard (advance built.state 0 0 7 ⟨⟨"reference-ir-control",122⟩,.plain (.instantiate "badDef" "route" [0] none .top)⟩).status = .failed .staticType
#guard (advance built.state 0 0 7 ⟨⟨"reference-ir-control",123⟩,.plain (.instantiate "badParent" "code" [0] (some "route") .top)⟩).status = .failed .staticType
#guard (advance built.state 0 0 7 ⟨⟨"reference-ir-control",124⟩,.plain (.instantiate "badSupport" "code" [0] none (.ref "route"))⟩).status = .failed .staticType

-- Oracle F4 discriminator: CURRENT candidate separates mutation action rights
-- from selected access rights. This control records the unresolved Canon14
-- continuing-owner mapping; it is not normative acceptance of this behavior.
def ownerMutationLost := head built.state [60]
#guard (run ownerMutationLost 0 0 7 [invocation]).status = .ready
#guard (advance ownerMutationLost 0 0 7 ⟨⟨"reference-ir-control",125⟩,.release "noOwnerRelease" "route"⟩).status = .failed .rejected

end MirroreaProofFirst.ReferenceSourceControls
