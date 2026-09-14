import MirroreaProofFirstOwnerEndpoint
import SourceWorkerSupport

open MirroreaProofFirst
namespace OwnerEndpointWorker
open OwnerCodecTree

def command (p a : Nat) : Codec (OwnerEndpoint.Command p a) := iso
  (sum (OwnerReservationWorker.commandCodec p a) natural)
  (fun c => match c with | .owner command => .inl command | .freeze revision => .inr revision)
  (fun c => match c with | .inl command => .owner command | .inr revision => .freeze revision)
  (by intro c; cases c <;> rfl) (by intro c; cases c <;> rfl)

#print axioms command


end OwnerEndpointWorker
