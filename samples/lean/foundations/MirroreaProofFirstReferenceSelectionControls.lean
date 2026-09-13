import MirroreaProofFirstReferenceSelection
import MirroreaProofFirstReferenceAccessControls

namespace MirroreaProofFirst.ReferenceSelectionControls
open ReferenceSelection ReferenceAccessControls
#guard (search system accessPolicy request 0).map Choice.index = some 0
#guard (search system accessPolicy request 1).map Choice.index = some 1
#guard (search system accessPolicy request 2).isNone
#guard (search revoked accessPolicy request 0).isNone
def retired : ManagementEntry.System 3 1 :=
  {system with configuration := CompositionCore.config (InstanceState.retire FallbackStaticControls.state 1)}
#guard (search retired accessPolicy request 0).map Choice.index = some 1
#guard (search retired accessPolicy request 1).map Choice.index = some 1
-- Recovery of the earlier instance does not change a strictly-later search.
#guard (search system accessPolicy request 1).map Choice.index = some 1
def primaryExpired := {request with chain := {request.chain with options :=
  [{FallbackStaticControls.primary with leaseUntil := 0},FallbackStaticControls.terminal]}}
#guard (search system accessPolicy primaryExpired 0).map Choice.index = some 1

-- A normalization occurrence advances the local serial. Checking the suffix
-- only before that commit can promise a choice already expired afterwards.
def expiringSuffix := {request with chain := {request.chain with options :=
  [FallbackStaticControls.primary,{FallbackStaticControls.terminal with leaseUntil := 1}]}}
#guard (search system accessPolicy expiringSuffix 1).map Choice.index = some 1
#guard (search {system with serial := system.serial+1} accessPolicy expiringSuffix 1).isNone

-- Observable A/B discriminator over three declared options and actual checked
-- definitions. This compares selection cuts; it is not a completed B executor.
def dependencyB := InstanceState.instantiate InstanceState.Controls.empty 0 7 [1] none .top
def primaryA := InstanceState.instantiate dependencyB 0 7 [0] none .top
def laterA := InstanceState.instantiate primaryA 0 7 [0] none (.ref 0)
def readerA := InstanceState.instantiate laterA 1 7 [0] none .top
def timingChain : FallbackStatic.Chain :=
  ⟨3,[{FallbackStaticControls.primary with leaseUntil := 1},
      {FallbackStaticControls.terminal with name := "later",target := 2},
      {FallbackStaticControls.terminal with target := 3}],
    [some ⟨"extra","later",true⟩,some ⟨"later","base",true⟩]⟩
def timingRequest := {request with chain := timingChain,place := 0}
def timingView : WorldProjection.AuthorityView 1 :=
  {system.view with authority := {system.view.authority with issued :=
    [{accessClaim with targets := [0,1,2,3]},ManagementEntry.Controls.claim,
     {InvocationBoundary.Controls.invocationClaim with targets := [5,9,13,17]}]}}
def timingMachine : CompositionMachine.Machine 3 1 :=
  ⟨{system with configuration := CompositionCore.config readerA,view := timingView},[],[]⟩
def timingLeft := CompositionMachine.manage timingMachine 0 0 7 30 (.leave 1)
def timingRejoined := timingLeft.bind fun (m,_) => CompositionMachine.manage m 0 0 7 31 (.join 1)
def eager := timingLeft.bind fun (m,_) => search {m.system with serial := m.system.serial+1} accessPolicy timingRequest 1
def lazyChoice := timingRejoined.bind fun (m,_) => search {m.system with serial := m.system.serial+1} accessPolicy timingRequest 1
#guard (eager.map Choice.index) = some 2
#guard (lazyChoice.map Choice.index) = some 1
def delivered (choice : Choice) : Option Int := do
  let (m,_) ← timingRejoined
  let current := {m.system with serial := m.system.serial+1}
  if !ReferenceAccess.check current accessPolicy (atIndex timingRequest choice.index) choice.guard then none else do
    let key ← CompositionCore.index current.configuration.count choice.guard.context.targetCapture.key
    let ticket ← InvocationBoundary.prepare current.configuration.state current.view 0 key 0 7 32 2
    if !InvocationBoundary.check current.configuration.state current.view ticket then none else InvocationBoundary.execute ticket
#guard (eager.bind delivered) = some 6
#guard (lazyChoice.bind delivered) = some 5
end MirroreaProofFirst.ReferenceSelectionControls
