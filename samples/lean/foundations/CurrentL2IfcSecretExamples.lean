/-!
current-l2 IFC secret examples fragment

This file keeps the first concrete IFC examples for Package 56 in one self-contained
Lean artifact. The goal is not the final public typed surface. The goal is to make the
secret-key valid/invalid reading executable at the proof-fragment level.
-/

namespace CurrentL2IfcSecretExamples

inductive SecurityLabel where
  | low
  | high
deriving DecidableEq, Repr

open SecurityLabel

def flowsTo : SecurityLabel → SecurityLabel → Prop
  | low, _ => True
  | high, high => True
  | high, low => False

def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=
  hasAuthority = true ∨ flowsTo fromLabel toLabel

structure Labeled (label : SecurityLabel) (α : Type) where
  value : α

abbrev SecretKey := Labeled high String
abbrev SecretFingerprint := Labeled high String
abbrev PublicFingerprint := Labeled low String

def fingerprint (key : SecretKey) : SecretFingerprint :=
  { value := "fp:" ++ key.value }

def declassify
    (hasAuthority : Bool)
    {fromLabel toLabel : SecurityLabel}
    (_proof : CanDeclassify hasAuthority fromLabel toLabel)
    (value : Labeled fromLabel α) :
    Labeled toLabel α :=
  { value := value.value }

theorem declassify_preserves_value
    (hasAuthority : Bool)
    {fromLabel toLabel : SecurityLabel}
    (proof : CanDeclassify hasAuthority fromLabel toLabel)
    (value : Labeled fromLabel α) :
    (declassify hasAuthority proof value).value = value.value := by
  rfl

theorem no_secret_release_without_authority :
    ¬ CanDeclassify false high low := by
  simp [CanDeclassify, flowsTo]

theorem authorized_secret_release_is_available :
    CanDeclassify true high low := by
  simp [CanDeclassify]

theorem low_to_low_release_without_authority_is_available :
    CanDeclassify false low low := by
  simp [CanDeclassify, flowsTo]

def publicAuditNote : Labeled low String :=
  { value := "audit-ok" }

def unchangedPublicAuditNote : Labeled low String :=
  declassify false low_to_low_release_without_authority_is_available publicAuditNote

theorem unchanged_public_audit_note_keeps_payload :
    unchangedPublicAuditNote.value = "audit-ok" := by
  simpa [unchangedPublicAuditNote, publicAuditNote] using
    declassify_preserves_value
      false
      low_to_low_release_without_authority_is_available
      publicAuditNote

def liveSecretKey : SecretKey :=
  { value := "sk_live" }

theorem fingerprint_keeps_secret_payload :
    (fingerprint liveSecretKey).value = "fp:sk_live" := by
  rfl

def authorizedPublicFingerprint : PublicFingerprint :=
  declassify true authorized_secret_release_is_available (fingerprint liveSecretKey)

theorem authorized_public_fingerprint_keeps_payload :
    authorizedPublicFingerprint.value = "fp:sk_live" := by
  simpa [authorizedPublicFingerprint, fingerprint, liveSecretKey] using
    declassify_preserves_value
      true
      authorized_secret_release_is_available
      (fingerprint liveSecretKey)

theorem invalid_release_has_no_authority_proof :
    ¬ ∃ _proof : CanDeclassify false high low, True := by
  intro h
  rcases h with ⟨proof, _⟩
  exact no_secret_release_without_authority proof

theorem valid_release_has_authority_proof :
    ∃ _proof : CanDeclassify true high low, True := by
  exact ⟨authorized_secret_release_is_available, trivial⟩

theorem authorized_live_fingerprint_release_has_witness :
    ∃ proof : CanDeclassify true high low,
      (declassify true proof (fingerprint liveSecretKey)).value = "fp:sk_live" := by
  refine ⟨authorized_secret_release_is_available, ?_⟩
  simpa [fingerprint, liveSecretKey] using
    declassify_preserves_value
      true
      authorized_secret_release_is_available
      (fingerprint liveSecretKey)

theorem unauthorized_live_fingerprint_release_is_impossible :
    ¬ ∃ proof : CanDeclassify false high low,
      (declassify false proof (fingerprint liveSecretKey)).value = "fp:sk_live" := by
  intro h
  rcases h with ⟨proof, _⟩
  exact no_secret_release_without_authority proof

end CurrentL2IfcSecretExamples
