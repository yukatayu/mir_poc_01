import OwnerMetadataSession
import OwnerMetadataSourceControls
namespace MirroreaProofFirst.OwnerMetadataSessionControls
open OwnerMetadataManagementControls (key other)
open OwnerMetadataSession

-- Actual ordinary checked-IR program, at the drained prefix before its first
-- owner assignment. Initial schema attachment remains explicit, not claimed
-- as source-level metadata construction or trusted issuer implementation.
def prepared := OwnerMetadataSourceControls.begun.map fun s => MixedOwnerContinuation.drive 6 s 0 7
#guard (prepared.map MixedOwnerContinuation.drained) = some true
def attached := prepared.bind fun session => do
 let metadata ← OwnerMetadataManagementControls.attach session.state.source.machine.store.core.system
 attach session metadata.metadata [key]
def activated := attached.bind fun s => changeMetadata s 0 0 7 ⟨"metadata-session.mir",1⟩ (.activate key 0)
def requested := activated.map fun s => tick s 0 7
#guard activated.isSome && requested.isSome
#guard (requested.map fun s => s.session.status) = some .waiting
#guard (requested.map fun s => s.packet.isSome) = some true
#guard (requested.map fun s => s.session.state.outbox.length) = some 1
def transferred := requested.bind transfer
def served := transferred.map fun s => service s (FallibleFlow.signed 63)
def acknowledged := served.map fun s => tick s 0 7
def finished := acknowledged.map fun s => drive 2 s 0 7
#guard (served.map fun s => s.session.state.owner.store 0) = some (some 15)
#guard (served.map fun s => (s.session.state.owner.history.length,s.session.state.inbox.length)) = some (1,1)
#guard (acknowledged.map fun s => (s.session.status,s.packet.isNone)) = some (.ready,true)
#guard (finished.map fun s => ReferenceSourceData.lookup s.session.state.source.values "later") =
 some (some (.plain (.integer 2 false)))

def recreated := transferred.bind fun s => do
 let s ← changeMetadata s 0 0 7 ⟨"metadata-session.mir",2⟩ (.retire key)
 changeMetadata s 0 0 7 ⟨"metadata-session.mir",3⟩ (.activate key 0)
def staleService := recreated.map fun s => service s (FallibleFlow.signed 63)
#guard (staleService.map fun s => (s.session.state.owner.store 0,s.session.state.owner.history.length,
 s.session.state.owner.queued.isSome,s.session.state.inbox.length)) = some (some 10,0,true,0)
#guard (staleService.map fun s => (tick s 0 7).session.status) = some .waiting

-- Deliberate RAW state injection demonstrates why a value-only packet check
-- cannot establish origin. The admitted entry relation forbids this state by
-- empty_packet_not_admitted; physical sealing/import validation is still owed.
def rawForged := recreated.map fun s => {s with packet := s.packet.map fun (packet : OwnerMetadataPending.Packet) => {packet with fields := []}}
#guard (rawForged.map fun s => (service s (FallibleFlow.signed 63)).session.state.owner.store 0) = some (some 15)

def unrelated := transferred.bind fun s => changeMetadata s 0 0 7 ⟨"metadata-session.mir",4⟩ (.activate other 0)
#guard (unrelated.map fun s => (service s (FallibleFlow.signed 63)).session.state.owner.store 0) = some (some 15)

-- Missing metadata refuses before publishing tentative request/event. Earlier
-- source computations remain; this does not roll back an actual owner write.
def refused := attached.map fun s => tick s 0 7
#guard (refused.map fun s => s.session.status) = some (.failed .rejected)
#guard (refused.map fun s => s.session.state.outbox.length) = some 0
#guard (refused.map fun s => s.session.state.source.machine.store.events.length) =
 attached.map fun s => s.session.state.source.machine.store.events.length
#guard (refused.map fun s => ReferenceSourceData.lookup s.session.state.source.values "amount") =
 some (some (.plain (.integer 5 false)))

theorem requested_rooted (start : attached = some initial) (run : requested = some next) : Rooted initial next := by
 unfold requested activated at run
 simp only [start,Option.bind_some] at run
 cases changed : changeMetadata initial 0 0 7 ⟨"metadata-session.mir",1⟩ (.activate key 0) with
 | none => simp [changed] at run
 | some state =>
   simp only [changed,Option.map_some,Option.some.injEq] at run
   subst next
   exact .tick (.metadata .initial changed)

theorem served_rooted (start : attached = some initial) (run : served = some next) : Rooted initial next := by
 unfold served transferred at run
 cases issued : requested with
 | none => simp [issued] at run
 | some state =>
   simp only [issued,Option.bind_some] at run
   cases queued : transfer state with
   | none => simp [queued] at run
   | some transferred =>
     simp only [queued,Option.map_some,Option.some.injEq] at run
     subst next
     exact .service (.transfer (requested_rooted start issued) queued)

theorem finished_rooted (start : attached = some initial) (run : finished = some next) : Rooted initial next := by
 unfold finished acknowledged at run
 cases used : served with
 | none => simp [used] at run
 | some state =>
   simp only [used,Option.map_some,Option.some.injEq] at run
   subst next
   exact drive_rooted (.tick (served_rooted start used))

#print axioms requested_rooted
#print axioms served_rooted
#print axioms finished_rooted
end MirroreaProofFirst.OwnerMetadataSessionControls
