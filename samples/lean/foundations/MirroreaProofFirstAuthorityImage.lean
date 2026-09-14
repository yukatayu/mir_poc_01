import MirroreaProofFirstCurrentUse

namespace MirroreaProofFirst.AuthorityImage
open CurrentUse

-- A finite representation obligation for the owner-input boundary. This is
-- NOT permission to disclose claims, proof of an authenticated image, or a wire
-- codec. An arbitrary decoded Image is not an admitted authority successor.
structure Image where
  issued : List Claim
  revoked : List Nat
  epochs : List (Nat × Option Nat)
  deriving DecidableEq, Repr

def lookupEpoch : List (Nat × Option Nat) → Nat → Option Nat
  | [],_ => none
  | (issuer,value) :: rest,key => if key = issuer then value else lookupEpoch rest key

def capture (a : Authority) : Image :=
  ⟨a.issued,a.revoked,a.issued.map fun claim => (claim.issuer,a.epochs claim.issuer)⟩

def restore (image : Image) : Authority :=
  ⟨image.issued,image.revoked,lookupEpoch image.epochs⟩

theorem captured_epoch (issued : List Claim) (epochs : Nat → Option Nat) (claim : Claim)
    (present : claim ∈ issued) :
    lookupEpoch (issued.map fun c => (c.issuer,epochs c.issuer)) claim.issuer = epochs claim.issuer := by
  induction issued with
  | nil => simp at present
  | cons first rest ih =>
      by_cases same : claim.issuer = first.issuer
      · simp [lookupEpoch,same]
      · have tail : claim ∈ rest := by
          rcases List.mem_cons.mp present with equal | tail
          · exact False.elim (same (congrArg Claim.issuer equal))
          · exact tail
        simpa [lookupEpoch,same] using ih tail

-- Only epochs of ACTUAL issued claims can affect a leaf judgment. No default
-- epoch for an unissued/forged witness can turn it into an issued claim.
theorem claim_roundtrip (a : Authority) (ctx : Context) (label : Nat) (need : Need) (claim : Claim) :
    ValidClaim (restore (capture a)) ctx label need claim ↔ ValidClaim a ctx label need claim := by
  by_cases present : claim ∈ a.issued
  · have epoch := captured_epoch a.issued a.epochs claim present
    simp [ValidClaim,restore,capture,present,epoch]
  · simp [ValidClaim,restore,capture,present]

theorem checkClaim_roundtrip (a : Authority) (ctx : Context) (label : Nat) (need : Need) (claim : Claim) :
    checkClaim (restore (capture a)) ctx label need claim = checkClaim a ctx label need claim := by
  apply Bool.eq_iff_iff.mpr
  rw [checkClaim_exact,checkClaim_exact,claim_roundtrip]

theorem witness_roundtrip (a : Authority) (ctx : Context) (label : Nat) (policy : PolicyExpr) (witness : Witness) :
    checkWitness (restore (capture a)) ctx label policy witness = checkWitness a ctx label policy witness := by
  induction policy generalizing witness with
  | leaf need => cases witness <;> simp [checkWitness,checkClaim_roundtrip]
  | both left right ihl ihr => cases witness <;> simp [checkWitness,ihl,ihr]
  | either left right ihl ihr => cases witness <;> simp [checkWitness,ihl,ihr]

theorem producer_roundtrip (a : Authority) (ctx : Context) (label : Nat) (policy : PolicyExpr) :
    produce (restore (capture a)) ctx label policy = produce a ctx label policy := by
  induction policy with
  | leaf need =>
      have same : checkClaim (restore (capture a)) ctx label need = checkClaim a ctx label need :=
        funext (checkClaim_roundtrip a ctx label need)
      simp only [produce,same]
      rfl
  | both left right ihl ihr => simp [produce,ihl,ihr]
  | either left right ihl ihr => simp [produce,ihl,ihr]

theorem finite_rows (a : Authority) : (capture a).epochs.length = a.issued.length := by simp [capture]

#print axioms captured_epoch
#print axioms claim_roundtrip
#print axioms checkClaim_roundtrip
#print axioms witness_roundtrip
#print axioms producer_roundtrip
#print axioms finite_rows
end MirroreaProofFirst.AuthorityImage
