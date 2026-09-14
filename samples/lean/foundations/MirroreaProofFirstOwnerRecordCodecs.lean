import MirroreaProofFirstOwnerCodeCodec

namespace MirroreaProofFirst.OwnerRecordCodecs
open OwnerCodecTree

-- Full record isomorphisms; neither authority rows nor context fields are
-- projected away. These are data codecs, not authority or currentness checks.
def scalar : Codec CurrentUse.Scalar := iso (sum integer boolean)
  (fun s => match s with | .integer i => .inl i | .boolean b => .inr b)
  (fun s => match s with | .inl i => .integer i | .inr b => .boolean b)
  (by intro s; cases s <;> rfl) (by intro s; cases s <;> rfl)

def recordKind : Codec CurrentUse.RecordKind where
  encode := fun k => .natural (match k with | .member => 0 | .locus => 1 | .module => 2 | .operation => 3)
  decode := fun tree => match tree with
    | .natural 0 => some .member
    | .natural 1 => some .locus
    | .natural 2 => some .module
    | .natural 3 => some .operation
    | _ => none
  roundtrip := by intro k; cases k <;> rfl
  canonical := by
    intro tree k decoded
    split at decoded <;> cases decoded <;> rfl

def need : Codec CurrentUse.Need := iso
  (product (natural) (natural))
  (fun r => (r.issuer,r.predicate))
  (fun (x0,x1) => ⟨x0,x1⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def claim : Codec CurrentUse.Claim := iso
  (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (list natural) (product (list natural) (natural)))))))))))
  (fun r => (r.id,r.issuer,r.epoch,r.principal,r.member,r.memberIncarnation,r.instanceId,r.predicate,r.actions,r.targets,r.label))
  (fun (x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10) => ⟨x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def context : Codec CurrentUse.Context := iso
  (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (natural) (product (list scalar) (product (natural) (product (natural) (natural)))))))))))))))))
  (fun r => (r.principal,r.member,r.memberIncarnation,r.locus,r.locusIncarnation,r.moduleKey,r.moduleIncarnation,r.action,r.target,r.targetIncarnation,r.targetRevision,r.instanceId,r.request,r.arguments,r.code,r.contract,r.generation))
  (fun (x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16) => ⟨x0,x1,x2,x3,x4,x5,x6,x7,x8,x9,x10,x11,x12,x13,x14,x15,x16⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def member : Codec WorldProjection.Member := iso
  (product (natural) (product (natural) (product (natural) (boolean))))
  (fun r => (r.principal,r.incarnation,r.revision,r.enabled))
  (fun (x0,x1,x2,x3) => ⟨x0,x1,x2,x3⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def identity : Codec CurrentUse.RecordIdentity := iso
  (product (recordKind) (product (natural) (natural)))
  (fun r => (r.kind,r.incarnation,r.revision))
  (fun (x0,x1,x2) => ⟨x0,x1,x2⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

def authority : Codec AuthorityImage.Image := iso
  (product (list claim) (product (list natural) (list (product natural (optional natural)))))
  (fun r => (r.issued,r.revoked,r.epochs))
  (fun (x0,x1,x2) => ⟨x0,x1,x2⟩)
  (by intro r; cases r; rfl) (by intro r; rfl)

#print axioms need
#print axioms claim
#print axioms context
#print axioms member
#print axioms identity
#print axioms authority
end MirroreaProofFirst.OwnerRecordCodecs
