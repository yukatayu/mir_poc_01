import OwnerMetadataSource
import OwnerMetadataStoreControls
import MixedOwnerContinuationControls
namespace MirroreaProofFirst.OwnerMetadataSourceControls
open OwnerMetadataManagementControls (key other bound attach)
open OwnerMetadataSource

def begun := MixedOwnerContinuation.launch 91 OwnerMetadataStoreControls.settings.view
 OwnerMetadataStoreControls.settings.controlPolicy MixedOwnerSourceTraceControls.initialStore
 MixedOwnerContinuationControls.program
def waiting := begun.map fun s => MixedOwnerContinuation.drive 7 s 0 7
#guard (waiting.map fun s => s.status) = some .waiting

def changed := waiting.bind fun session => do
 let source := session.state.source
 let metadata ← attach source.machine.store.core.system
 let (next,registry) ← changeMetadata source metadata.metadata 0 0 7 ⟨"shared-source.mir",45⟩ (.activate key 3)
 return (source,next,registry)
#guard changed.isSome
#guard (changed.map fun (before,next,_) => decide (next.nextRequest = before.nextRequest+1)) = some true
#guard (changed.map fun (before,next,_) => decide (next.waiting = before.waiting) && next.waiting.isSome) = some true
#guard (changed.map fun (before,next,_) => decide (next.values = before.values) && decide (next.writes = before.writes)) = some true
#guard (changed.map fun (before,next,_) => decide (next.machine.store.events.length = before.machine.store.events.length+1)) = some true
#guard (changed.map fun (before,next,_) => (next.origins.head?).map fun (o : ReferenceSource.Origin) => (o.kind,o.beforeCount,o.afterCount)) =
 changed.map fun (before,_,_) => some (ReferenceSource.MicroKind.control,before.machine.store.events.length,before.machine.store.events.length+1)
#guard (changed.map fun (_,next,registry) => OwnerMetadataManagement.usable
 (OwnerMetadataStore.state next.machine.store registry) 0 bound) = some true
#guard (changed.bind fun (_,next,registry) => changeMetadata next registry 0 0 7 ⟨"shared-source.mir",46⟩ (.activate other 3)).isSome

-- Refusal has no changed state to install and cannot fabricate a source origin.
def revoked := waiting.bind fun session => do
 let source ← MixedOwnerSourceIssue.authorityHead session.state.source
   {session.state.source.machine.store.core.system.view with authority :=
     {session.state.source.machine.store.core.system.view.authority with revoked := [93]}}
 let metadata ← attach source.machine.store.core.system
 changeMetadata source metadata.metadata 0 0 7 ⟨"shared-source.mir",45⟩ (.activate key 3)
#guard revoked.isNone

-- This reaches source control through the actual owner program's launch/drive;
-- it does not yet install a coupled metadata registry into the enclosing Session,
-- authenticate the schema, or recheck a bound generation in physical owner use.
end MirroreaProofFirst.OwnerMetadataSourceControls
