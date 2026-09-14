import SourceWorkerSupport
import MirroreaProofFirstPublicationInput

open MirroreaProofFirst
namespace SourcePublicationWorker
open OwnerCodecTree

def dispatch (p : Nat) : Codec (PublicationInput.Dispatch p) := iso
  (product (finite p) (product natural (product natural OwnerFullCodec.ticket)))
  (fun d => (d.endpoint,d.context.scopeId,d.context.revision,d.ticket))
  (fun (endpoint,scope,revision,ticket) => ⟨endpoint,⟨scope,revision⟩,ticket⟩)
  (by intro d; cases d; rfl) (by intro d; rfl)

structure PrivateOutput (p a : Nat) where
  source : SourceWorker.PrivateOutput p a
  published : Nat
  announced : Nat
  installed : Vector Nat p
  fence : Vector Nat p
  ack : Vector Nat p
  dispatch : Option (PublicationInput.Dispatch p)
  heldImage : Option (OwnerImage.Image p a)

def output (p a : Nat) : Codec (PrivateOutput p a) := iso
  (product (SourceWorker.output p a) (product natural (product natural
    (product (vector natural p) (product (vector natural p) (product (vector natural p)
      (product (optional (dispatch p)) (optional (OwnerFullCodec.image p a)))))))))
  (fun s => (s.source,s.published,s.announced,s.installed,s.fence,s.ack,s.dispatch,s.heldImage))
  (fun (source,published,announced,installed,fence,ack,dispatch,heldImage) =>
    ⟨source,published,announced,installed,fence,ack,dispatch,heldImage⟩)
  (by intro s; cases s; rfl) (by intro s; rfl)

def project (s : PublicationInput.State p a) : PrivateOutput p a :=
  ⟨SourceWorker.project s.publication.current,s.publication.barrier.published,s.publication.barrier.announced,
    Vector.ofFn s.publication.barrier.installed,Vector.ofFn s.publication.barrier.fence,Vector.ofFn s.publication.barrier.ack,
    s.dispatch,s.dispatch >>= fun d => (s.publication.held d.endpoint).map Prod.snd⟩

def reply (p a : Nat) := product boolean (optional (output p a))
def exchange (assigned : SourceInput.Assignment p a) (scopeId : Nat) (seed : SourceInput.Bootstrap a)
    (s : Option (PublicationInput.State p a)) (input : PublicationInput.Input p a) :=
  let (next,accepted) := PublicationInput.transition assigned scopeId seed s input
  (next,(accepted,next.map project))

theorem exchange_preserves (valid : PublicationInput.Invariant assigned scopeId seed s) :
    PublicationInput.Invariant assigned scopeId seed (exchange assigned scopeId seed s input).1 :=
  PublicationInput.transition_preserves valid

theorem reply_bytes (message : Bool × Option (PrivateOutput p a)) :
    OwnerPacketCodec.decode (reply p a) (OwnerPacketCodec.encode (reply p a) message) = some message :=
  OwnerPacketCodec.roundtrip _ _

#print axioms dispatch
#print axioms output
#print axioms exchange_preserves
#print axioms reply_bytes


end SourcePublicationWorker
