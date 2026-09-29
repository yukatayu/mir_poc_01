import OwnerMetadataStore
import OwnerMetadataManagementControls
import MixedExecutionControls
namespace MirroreaProofFirst.OwnerMetadataStoreControls
open OwnerMetadataManagementControls (key other declaration bound attach withMetadata)
open MixedReferenceExecution

-- Fixture authority is supplied ONCE at empty initial construction. Actual
-- register/instantiate/acquire/start entries produce every subsequent machine.
def settings := withMetadata {MixedManagementControls.initial with
 view := MixedExecutionControls.view,controlPolicy := MixedStoreControls.policy}
def initial : Machine 3 1 := MixedReferenceExecution.initial 91 settings.view settings.controlPolicy

def configure (m : Machine p a) (member : Fin a) (place : Fin p) (principal requestId : Nat) :
 List MixedCompositionCore.Raw → Option (Machine p a)
 | [] => some m
 | action::rest => do
   let (next,_) ← manage m member place principal requestId action
   configure next member place principal (requestId+1) rest

theorem configure_valid (valid : Invariant m)
 (accepted : configure m member place principal requestId actions = some result) : Invariant result := by
 induction actions generalizing m requestId with
 | nil => cases accepted; exact valid
 | cons action rest ih =>
   cases done : manage m member place principal requestId action with
   | none => simp [configure,done] at accepted
   | some pair =>
     obtain ⟨next,created⟩ := pair
     exact ih (MixedReferenceExecution.manage_preserves _ _ _ _ _ _ _ valid done)
       (by simpa [configure,done] using accepted)

def actions : List MixedCompositionCore.Raw :=
 [MixedManagementControls.pureCode,.instantiate 0 7 [0] none .top,
 MixedManagementControls.ownerCode,.instantiate 1 7 [0] none .top,.instantiate 0 7 [0] none .top]
def configured := configure initial 0 0 7 0 actions
def built := configured.bind fun m => acquire m 0 0 7 5 MixedReferenceControls.chain
def bothPending := built.bind fun (m,_) => do
 let (m,pure) ← startReference m 0 0 7 6 0 2
 let (m,owner) ← startOwner m 0 0 7 7 1 MixedRequestControls.origin [.integer 5]
 let metadata ← attach m.store.core.system
 return (m,pure,owner,metadata.metadata)
#guard bothPending.isSome

-- The lower reviewed management-only function admits these IDs; the real
-- shared store boundary refuses both kinds of pending identifier.
#guard (bothPending.bind fun (m,_,_,r) =>
 OwnerMetadataManagement.perform (OwnerMetadataStore.state m.store r) 0 0 7 6 (.activate key 3)).isSome
#guard (bothPending.bind fun (m,_,_,r) => OwnerMetadataStore.execute m r 0 0 7 6 (.activate key 3)).isNone
#guard (bothPending.bind fun (m,_,_,r) => OwnerMetadataStore.execute m r 0 0 7 7 (.activate key 3)).isNone

def metadataChanged := bothPending.bind fun (m,pure,owner,r) => do
 let (next,r) ← OwnerMetadataStore.execute m r 0 0 7 8 (.activate key 3)
 return (next,pure,owner,r)
#guard metadataChanged.isSome
#guard (metadataChanged.map fun (m,pure,owner,_) => decide (m.pending = [pure]) &&
 decide (m.store.core.pending = [.owner owner,.pure pure.ticket])) = some true
#guard (metadataChanged.map fun (m,_,_,_) => m.store.events.length) =
 bothPending.map fun (m,_,_,_) => m.store.events.length+1
#guard (metadataChanged.map fun (m,_,_,_) => m.store.history.length) =
 bothPending.map fun (m,_,_,_) => m.store.history.length+1
#guard (metadataChanged.map fun (m,_,_,_) => m.store.core.events.length) =
 bothPending.map fun (m,_,_,_) => m.store.core.events.length
#guard (metadataChanged.map fun (m,_,_,r) =>
 OwnerMetadataManagement.usable (OwnerMetadataStore.state m.store r) 0 bound) = some true
#guard (metadataChanged.bind fun (m,_,_,_) => reacquire m 0 0 7 8 0).isNone
#guard (metadataChanged.bind fun (m,_,_,_) => reacquire m 0 0 7 9 0).isSome
#guard (metadataChanged.bind fun (m,pure,_,_) => resume m pure |>.map fun (_,value) => value) = some 5
#guard (metadataChanged.bind fun (m,pure,owner,_) => resume m pure |>.map fun (next,_) =>
 decide (next.store.core.pending = [.owner owner])) = some true
#guard (metadataChanged.bind fun (m,_,_,r) => OwnerMetadataStore.execute m r 0 0 7 8 (.activate other 3)).isNone

def metadataRetired := metadataChanged.bind fun (m,pure,owner,r) => do
 let (next,r) ← OwnerMetadataStore.execute m r 0 0 7 9 (.retire key)
 return (next,pure,owner,r)
def metadataReturned := metadataRetired.bind fun (m,pure,owner,r) => do
 let (next,r) ← OwnerMetadataStore.execute m r 0 0 7 10 (.activate key 3)
 return (next,pure,owner,r)
#guard metadataRetired.isSome && metadataReturned.isSome
#guard (metadataReturned.map fun (m,_,_,r) => OwnerMetadataManagement.usable (OwnerMetadataStore.state m.store r) 0 bound) = some false
-- Binding predicates alone are not the owner service carrier. This file does
-- not claim the unchanged saved owner request consumes generation evidence.

theorem configured_valid (done : configured = some result) : Invariant result :=
 configure_valid (initial_invariant _ _ _) done

theorem built_valid (done : built = some result) : Invariant result.1 := by
 unfold built at done
 cases prior : configured with
 | none => simp [prior] at done
 | some machine => exact acquire_preserves _ _ _ _ _ _ _ (configured_valid prior) (by simpa [prior] using done)

#print axioms configure_valid
#print axioms configured_valid
#print axioms built_valid
end MirroreaProofFirst.OwnerMetadataStoreControls
