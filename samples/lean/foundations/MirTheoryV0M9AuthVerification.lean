/-!
Mir Theory v0 M9 finite authorization/verification seam.

This self-contained finite model keeps source-bound M7 residuals and an
embedded M8 identity intact. It separates runtime Contract policy from
verification results; it does not identify Lean carriers with Rust/M7/M8 types.
-/

namespace MirTheoryV0M9AuthVerification

set_option autoImplicit false

inductive SourceRef where
  | authSource
  | verifySource
  | wrongSource
deriving DecidableEq, Repr

inductive CheckedIdentity where
  | deferredAuthVerifyIdentity
  | wrongIdentity
deriving DecidableEq, Repr

inductive M8Identity where
  | embeddedDeferredInstance
  | wrongEmbeddedInstance
deriving DecidableEq, Repr

inductive ResidualKind where
  | authDeferred
  | verifyDeferred
deriving DecidableEq, Repr

inductive RequiredTarget where
  | membershipAuth
  | finiteRefinement
deriving DecidableEq, Repr

structure SourceBoundResidual where
  kind : ResidualKind
  source : SourceRef
  requiredTarget : RequiredTarget
deriving DecidableEq, Repr

def canonicalAuthResidual : SourceBoundResidual :=
  { kind := .authDeferred, source := .authSource, requiredTarget := .membershipAuth }

def canonicalVerifyResidual : SourceBoundResidual :=
  { kind := .verifyDeferred, source := .verifySource, requiredTarget := .finiteRefinement }

def wrongSourceAuthResidual : SourceBoundResidual :=
  { canonicalAuthResidual with source := .wrongSource }

inductive MembershipStatus where
  | current
  | removed
deriving DecidableEq, Repr

inductive CapabilityStatus where
  | granted
  | revoked
  | absent
deriving DecidableEq, Repr

inductive ContractRef where
  | baseline
  | membershipAndCapabilityBound
deriving DecidableEq, Repr

inductive ActivationCut where
  | cut1
deriving DecidableEq, Repr

inductive PolicyModule where
  | membershipAuth
  | capabilityAuth
deriving DecidableEq, Repr

/- The bounded policy modules are non-transparent: each adds a checked
   precondition/failure/capability requirement and therefore proposes an
   explicit ContractUpdate rather than silently replacing a Contract. -/
structure ContractUpdate where
  oldRef : ContractRef
  newRef : ContractRef
  policy : PolicyModule
  reason : String
  activationCut : ActivationCut
  admittedByExistingGrant : Bool
  observationDeltaRecorded : Bool
deriving DecidableEq, Repr

def membershipContractUpdate : ContractUpdate :=
  { oldRef := .baseline
    newRef := .membershipAndCapabilityBound
    policy := .membershipAuth
    reason := "membership precondition and StaleMembership failure"
    activationCut := .cut1
    admittedByExistingGrant := true
    observationDeltaRecorded := true }

def capabilityContractUpdate : ContractUpdate :=
  { membershipContractUpdate with
    policy := .capabilityAuth
    reason := "capability requirement and MissingCapability failure" }

inductive VerifierResult where
  | evidence
  | diagnostic
  | residual
deriving DecidableEq, Repr

/- An ExtensionFrame is shared provenance only. Its two consumers below have
   disjoint result types: runtime policy may propose/activate a ContractUpdate;
   verification may return Evidence, Diagnostic, or Residual only. -/
structure ExtensionFrame where
  checkedIdentity : CheckedIdentity
  embeddedM8Identity : M8Identity
  authResidual : SourceBoundResidual
  verifyResidual : SourceBoundResidual
  contractRef : ContractRef
  membership : MembershipStatus
  capability : CapabilityStatus
  membershipEpoch : Nat
  sourceMapRetained : Bool
  invalidated : Bool
deriving DecidableEq, Repr

def canonicalFrame : ExtensionFrame :=
  { checkedIdentity := .deferredAuthVerifyIdentity
    embeddedM8Identity := .embeddedDeferredInstance
    authResidual := canonicalAuthResidual
    verifyResidual := canonicalVerifyResidual
    contractRef := .baseline
    membership := .current
    capability := .granted
    membershipEpoch := 1
    sourceMapRetained := true
    invalidated := false }

/- M8 alone cannot resolve either deferred residual. -/
inductive M8AdmissionResult where
  | deferredToM9
deriving DecidableEq, Repr

def m8Admit (_ : ExtensionFrame) : M8AdmissionResult := .deferredToM9

def exactDeferredPair (frame : ExtensionFrame) : Bool :=
  match frame.checkedIdentity with
  | .wrongIdentity => false
  | .deferredAuthVerifyIdentity =>
      match frame.embeddedM8Identity with
      | .wrongEmbeddedInstance => false
      | .embeddedDeferredInstance =>
          match frame.authResidual.kind with
          | .verifyDeferred => false
          | .authDeferred =>
              match frame.authResidual.source with
              | .verifySource | .wrongSource => false
              | .authSource =>
                  match frame.authResidual.requiredTarget with
                  | .finiteRefinement => false
                  | .membershipAuth =>
                      match frame.verifyResidual.kind with
                      | .authDeferred => false
                      | .verifyDeferred =>
                          match frame.verifyResidual.source with
                          | .authSource | .wrongSource => false
                          | .verifySource =>
                              match frame.verifyResidual.requiredTarget with
                              | .membershipAuth => false
                              | .finiteRefinement => frame.sourceMapRetained

/- The finite verifier models the ordinary `verify finite_refinement` route.
   It cannot produce a membership/capability grant or mutate a Contract. -/
def verifyFiniteRefinement (frame : ExtensionFrame) : VerifierResult :=
  if exactDeferredPair frame then
    if frame.invalidated then .residual else .evidence
  else .diagnostic

/- MembershipAuth evidence is a principal-claim premise, not a grant. The
   ordinary admission chain is represented separately by existing capability
   lineage in the frame. -/
def verifyMembershipAuth (frame : ExtensionFrame) : VerifierResult :=
  if exactDeferredPair frame then
    match frame.membership with
    | .current => .evidence
    | .removed => .diagnostic
  else .diagnostic

inductive PolicyResult where
  | proposedUpdate (update : ContractUpdate)
  | rejected
deriving DecidableEq, Repr

def transformContract (frame : ExtensionFrame) (module : PolicyModule) : PolicyResult :=
  if exactDeferredPair frame && !frame.invalidated then
    match module with
    | .membershipAuth => .proposedUpdate membershipContractUpdate
    | .capabilityAuth => .proposedUpdate capabilityContractUpdate
  else .rejected

/- Only policy activation can replace the runtime Contract. Evidence is not an
   `admitted_by` value and does not cross this boundary. -/
def activateContractUpdate (frame : ExtensionFrame) (update : ContractUpdate) : ExtensionFrame :=
  if update.admittedByExistingGrant && update.observationDeltaRecorded &&
      frame.membership = .current && frame.capability = .granted &&
      !frame.invalidated then
    { frame with contractRef := update.newRef }
  else frame

def revokeOrRemove (frame : ExtensionFrame) : ExtensionFrame :=
  { frame with
    membership := .removed
    capability := .revoked
    membershipEpoch := frame.membershipEpoch + 1
    invalidated := true }

inductive M9Resolution where
  | m9Admitted
  | rejectedDiagnostic
  | residual
deriving DecidableEq, Repr

/- This is the outer, source-bound M9 judgment. It retains rather than rewrites
   the M7/M8 identities and residual/source-map view represented by the frame. -/
def resolveM9 (frame : ExtensionFrame) : M9Resolution :=
  if exactDeferredPair frame then
    match verifyFiniteRefinement frame with
    | .residual => .residual
    | .diagnostic => .rejectedDiagnostic
    | .evidence =>
        match verifyMembershipAuth frame with
        | .evidence => .m9Admitted
        | .diagnostic => .rejectedDiagnostic
        | .residual => .residual
  else .rejectedDiagnostic

/- OBL-026 finite carrier: exactly two transparent overlays with equal selected
   ContractRef results. MembershipAuth/CapabilityAuth are not transparent and
   are therefore excluded from this proposition. -/
inductive TransparentOverlay where
  | identity
  | redactMore
deriving DecidableEq, Repr

def applyTransparent : TransparentOverlay → ContractRef → ContractRef
  | .identity, contract => contract
  | .redactMore, contract => contract

def composeTransparent (left right : TransparentOverlay) : TransparentOverlay :=
  match left, right with
  | .identity, overlay => overlay
  | overlay, .identity => overlay
  | .redactMore, .redactMore => .redactMore

/- This fixed toy trace is not OBL-028's model-status carrier. That status has
   separate accepted bounded Rust evidence; this Lean theorem remains unchanged. -/
def toyRevocationTrace : List ExtensionFrame :=
  [canonicalFrame, revokeOrRemove canonicalFrame]

theorem external_m9_resolution_requires_exact_source_bound_rows :
    resolveM9 canonicalFrame = .m9Admitted ∧
    resolveM9 { canonicalFrame with authResidual := wrongSourceAuthResidual } =
      .rejectedDiagnostic ∧
    resolveM9 { canonicalFrame with embeddedM8Identity := .wrongEmbeddedInstance } =
      .rejectedDiagnostic ∧
    ({ canonicalFrame with authResidual := wrongSourceAuthResidual }).sourceMapRetained =
      canonicalFrame.sourceMapRetained := by
  exact ⟨rfl, rfl, rfl, rfl⟩

theorem m8_admit_alone_remains_deferred_to_m9 :
    m8Admit canonicalFrame = .deferredToM9 ∧
    m8Admit (revokeOrRemove canonicalFrame) = .deferredToM9 := by
  exact ⟨rfl, rfl⟩

theorem verifier_evidence_cannot_mint_authority_or_activate_contract_update :
    verifyFiniteRefinement canonicalFrame = .evidence ∧
    canonicalFrame.capability = .granted ∧
    (activateContractUpdate canonicalFrame
      { membershipContractUpdate with admittedByExistingGrant := false }).contractRef =
      canonicalFrame.contractRef ∧
    (transformContract canonicalFrame .membershipAuth = .proposedUpdate membershipContractUpdate) := by
  exact ⟨rfl, rfl, rfl, rfl⟩

theorem revoked_or_removed_provenance_rejects_without_runtime_mutation :
    verifyMembershipAuth (revokeOrRemove canonicalFrame) = .diagnostic ∧
    verifyFiniteRefinement (revokeOrRemove canonicalFrame) = .residual ∧
    resolveM9 (revokeOrRemove canonicalFrame) = .residual ∧
    activateContractUpdate (revokeOrRemove canonicalFrame) membershipContractUpdate =
      revokeOrRemove canonicalFrame ∧
    toyRevocationTrace = [canonicalFrame, revokeOrRemove canonicalFrame] := by
  exact ⟨rfl, rfl, rfl, rfl, rfl⟩

theorem transparent_overlay_composition_is_finite_and_exact :
    applyTransparent (composeTransparent .identity .redactMore) .baseline = .baseline ∧
    applyTransparent (composeTransparent .redactMore .identity) .baseline = .baseline ∧
    applyTransparent (composeTransparent .redactMore .redactMore) .baseline = .baseline ∧
    transformContract canonicalFrame .capabilityAuth = .proposedUpdate capabilityContractUpdate := by
  exact ⟨rfl, rfl, rfl, rfl⟩

#print axioms external_m9_resolution_requires_exact_source_bound_rows
#print axioms m8_admit_alone_remains_deferred_to_m9
#print axioms verifier_evidence_cannot_mint_authority_or_activate_contract_update
#print axioms revoked_or_removed_provenance_rejects_without_runtime_mutation
#print axioms transparent_overlay_composition_is_finite_and_exact

end MirTheoryV0M9AuthVerification
