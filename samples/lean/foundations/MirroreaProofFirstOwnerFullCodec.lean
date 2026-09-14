import MirroreaProofFirstOwnerTreeCodecs
import MirroreaProofFirstOwnerEvaluator

namespace MirroreaProofFirst.OwnerFullCodec
open OwnerCodecTree

-- Exact structural serialization for the whole finite owner input and ticket.
-- No omitted claim/context/policy/support rows, no byte/parser/currentness claim.

def policy  : Codec (CurrentUse.Policy) := iso
  (product (natural) (product (natural) (product (natural) (OwnerTreeCodecs.policy))))
  (fun r => (r.id,r.version,r.label,r.expression))
  (fun (x0,x1,x2,x3) => ⟨x0,x1,x2,x3⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def evidence  : Codec (CurrentUse.Evidence) := iso
  (product (natural) (product (natural) (product (OwnerRecordCodecs.context) (OwnerTreeCodecs.witness))))
  (fun r => (r.policy,r.version,r.context,r.witness))
  (fun (x0,x1,x2,x3) => ⟨x0,x1,x2,x3⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def instanceData (d p n : Nat) : Codec (InstanceState.Instance d p n) := iso
  (product (finite d) (product (OwnerCodeCodec.contract) (product (natural) (product (natural) (product (boolean) (product (list (finite p)) (product (optional (finite n)) (OwnerTreeCodecs.formula (finite n)))))))))
  (fun r => (r.definition,r.interface,r.owner,r.revision,r.enabled,r.placements,r.parent,r.dependencies))
  (fun (x0,x1,x2,x3,x4,x5,x6,x7) => ⟨x0,x1,x2,x3,x4,x5,x6,x7⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def state (d p n : Nat) : Codec (OwnerImage.StateImage d p n) := iso
  (product (natural) (product (vector natural p) (product (vector OwnerCodeCodec.definition d) (product (vector (optional (finite d)) d) (product (vector (instanceData d p n) n) (vector boolean p))))))
  (fun r => (r.realm,r.incarnations,r.definitions,r.predecessors,r.instances,r.participating))
  (fun (x0,x1,x2,x3,x4,x5) => ⟨x0,x1,x2,x3,x4,x5⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def view (a p n : Nat) : Codec (OwnerImage.ViewImage a p n) := iso
  (product (natural) (product (natural) (product (vector OwnerRecordCodecs.member a) (product (OwnerRecordCodecs.authority) (vector (vector policy p) n)))))
  (fun r => (r.realm,r.generation,r.members,r.authority,r.policies))
  (fun (x0,x1,x2,x3,x4) => ⟨x0,x1,x2,x3,x4⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def ticket  : Codec (InvocationBoundary.Ticket) := iso
  (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (integer) (product (OwnerRecordCodecs.identity) (product (OwnerRecordCodecs.identity) (product (OwnerRecordCodecs.identity) (product (OwnerRecordCodecs.identity) (product (OwnerCodeCodec.definition) (product (natural) (product (natural) (evidence)))))))))))))))
  (fun r => (r.realm,r.member,r.key,r.place,r.principal,r.id,r.argument,r.memberIdentity,r.locusIdentity,r.moduleIdentity,r.operationIdentity,r.definition,r.arithmeticProfile,r.contractTheoryVersion,r.evidence))
  (fun (x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14) => ⟨x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def image (p a : Nat) : Codec (OwnerImage.Image p a) := iso
  (dependent natural (fun d => dependent natural (fun n => product (state d p n) (view a p n))))
  (fun value => ⟨value.definitions,⟨value.count,(value.state,value.view)⟩⟩)
  (fun value => ⟨value.1,value.2.1,value.2.2.1,value.2.2.2⟩)
  (by intro value; cases value; rfl)
  (by intro value; rcases value with ⟨d,⟨n,s,v⟩⟩; rfl)

def request (p a : Nat) := product (image p a) ticket

-- Mathematical evaluator correspondence uses the very same full decoded value.
-- Serialization is lossless; semantic rejection remains an evaluator decision.
theorem roundtrip_evaluator (assigned : OwnerEvaluator.Assignment p)
    (input : OwnerImage.Image p a × InvocationBoundary.Ticket) :
    ((request p a).decode ((request p a).encode input)).map
      (fun pair => OwnerEvaluator.run assigned pair.1 pair.2) =
        some (OwnerEvaluator.run assigned input.1 input.2) := by
  rw [(request p a).roundtrip]
  rfl

theorem decoded_exact (wire : Tree) (input : OwnerImage.Image p a × InvocationBoundary.Ticket) :
    (request p a).decode wire = some input ↔ wire = (request p a).encode input :=
  decode_exact _ _ _

#print axioms image
#print axioms ticket
#print axioms roundtrip_evaluator
#print axioms decoded_exact
end MirroreaProofFirst.OwnerFullCodec
