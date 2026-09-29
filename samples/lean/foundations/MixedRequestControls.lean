import MixedRequestEmbedding
import MixedCatalogSuccessorControls
namespace MirroreaProofFirst.MixedRequestControls
open MixedRequestCore MixedManagementControls

def built := bothBuilt.map fun (s,_) => (⟨s,[],[]⟩ : Machine 3 1)
def pureStarted := built.bind fun m => startPure m 0 0 7 4 0 2
def origin : OwnerEffectService.Origin := ⟨"mixed-request-control",60,7,0,0⟩
-- Independent pending operations in the shared core. NOT a source sequence;
-- the owner argument below is a literal input, not a forged pure completion.
def bothPending := pureStarted.bind fun (m,t) => do
 let (next,saved) ← startOwner m 0 0 7 5 1 origin [.integer 5]
 return (next,t,saved)
#guard (bothPending.map fun (m,_,_) => m.pending.length) = some 2
#guard (pureStarted.bind fun (m,_) => startOwner m 0 0 7 4 1 origin [.integer 5]).isNone
#guard (bothPending.bind fun (m,_,_) => startPure m 0 0 7 5 0 2).isNone
#guard (bothPending.bind fun (m,_,_) => manage m 0 0 7 5 (.retire 0)).isNone
#guard (bothPending.bind fun (m,t,_) => finishPure m t 6).isNone

def pureCompleted := bothPending.bind fun (m,t,saved) => do
 let value ← InvocationBoundary.execute t
 let next ← finishPure m t value
 return (next,t,saved,value)
#guard (pureCompleted.map fun (_,_,_,value) => value) = some 5
#guard (pureCompleted.map fun (m,_,saved,_) => decide (m.pending = [.owner saved])) = some true
#guard (pureCompleted.map fun (m,_,_,_) => m.system.used.map CurrentUse.UseId.request) = some [4,3,2,1,0]
#guard (pureCompleted.bind fun (m,t,_,value) => finishPure m t value).isNone
#guard (pureCompleted.map fun (m,_,saved,_) =>
 decide ((authorityHead m {m.system.view with generation := 1}).pending = [.owner saved])) = some true
-- No owner ack/service replay/restore/cancellation/whole-source claim.
end MirroreaProofFirst.MixedRequestControls
