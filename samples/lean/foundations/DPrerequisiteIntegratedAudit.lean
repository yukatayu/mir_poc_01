import MirroreaProofFirstAuthorityLocalFrame
import MirroreaProofFirstAuthorityLocalFrameControls
import OwnerStatementHeldAuthority
import OwnerStatementHeldAuthorityControls
import MirroreaProofFirstCoordinatorUse
import MirroreaProofFirstCoordinatorUseControls
import MirroreaProofFirstPublication
import MirroreaProofFirstPublicationPayload
import MirroreaProofFirstPublicationUse
import OwnerStatementOriginalEntryControls
import OwnerStatementAcknowledgmentControls
import OwnerStatementResultLifecycleControls
import OwnerStatementResultOriginControls
import OwnerStatementResultCollectionControls
import OwnerStatementCounterBudget
import OwnerStatementSourceCustodianControls
import OwnerStatementRegistryReviewControls
import OwnerStatementRegistryControls
import OwnerStatementEntryGate
import OwnerStatementRetainedHistory
import OwnerStatementRetainedControls
import OwnerStatementCursor
import OwnerStatementSession
import OwnerStatementMetadataCursor
import OwnerStatementAcceptance
import OwnerStatementBinding
import OwnerStatementMetadataBinding
import OwnerStatementHistory
import OwnerStatementMetadataHistory
import OwnerStatementProgram
import OwnerStatementAdmission
import OwnerStatementPosition
import OwnerStatementCompletion
import OwnerStatementHistoryOrigin
import OwnerStatementExecutionControls
import OwnerStatementBoundaryControls
import OwnerStatementIdentity
import OwnerStatementControls
import Lean
set_option maxRecDepth 10000
open Lean Elab Command
run_cmd do
  let env ← getEnv
  let modules : List String := ["MirroreaProofFirstAbortFlow", "MirroreaProofFirstAdmissionPhases", "MirroreaProofFirstAuthorityLocalFrame", "MirroreaProofFirstAuthorityLocalFrameControls", "MirroreaProofFirstCatalogHistory", "MirroreaProofFirstCompositionCore", "MirroreaProofFirstCompositionMachine", "MirroreaProofFirstContractExport", "MirroreaProofFirstCoordinatorUse", "MirroreaProofFirstCoordinatorUseControls", "MirroreaProofFirstCurrentUse", "MirroreaProofFirstDynamicGraphs", "MirroreaProofFirstDynamicIdentity", "MirroreaProofFirstDynamicSupport", "MirroreaProofFirstFallbackStatic", "MirroreaProofFirstFallibleFlow", "MirroreaProofFirstFunctionContractBridge", "MirroreaProofFirstGraphValidation", "MirroreaProofFirstInstancePrograms", "MirroreaProofFirstInstanceState", "MirroreaProofFirstInvocationBoundary", "MirroreaProofFirstInvocationContract", "MirroreaProofFirstLocalContract", "MirroreaProofFirstManagementEntry", "MirroreaProofFirstModuleContractBoundary", "MirroreaProofFirstOwnerAssignment", "MirroreaProofFirstOwnerCheckedArithmetic", "MirroreaProofFirstOwnerEffectService", "MirroreaProofFirstOwnerPartial", "MirroreaProofFirstOwnerReadCoverage", "MirroreaProofFirstOwnerReadReport", "MirroreaProofFirstOwnerRecordedArithmetic", "MirroreaProofFirstOwnerSourceContext", "MirroreaProofFirstOwnerStructuredKeys", "MirroreaProofFirstPassive", "MirroreaProofFirstProducerFlow", "MirroreaProofFirstPublication", "MirroreaProofFirstPublicationPayload", "MirroreaProofFirstPublicationUse", "MirroreaProofFirstPureFunctions", "MirroreaProofFirstReferenceAccess", "MirroreaProofFirstReferenceAccessHistory", "MirroreaProofFirstReferenceAllocation", "MirroreaProofFirstReferenceAuthority", "MirroreaProofFirstReferenceCancellation", "MirroreaProofFirstReferenceCancellationBoundary", "MirroreaProofFirstReferenceCoherence", "MirroreaProofFirstReferenceExecution", "MirroreaProofFirstReferenceHolding", "MirroreaProofFirstReferenceMutation", "MirroreaProofFirstReferenceOwner", "MirroreaProofFirstReferenceResult", "MirroreaProofFirstReferenceSelection", "MirroreaProofFirstReferenceSource", "MirroreaProofFirstReferenceSourceData", "MirroreaProofFirstReferenceSourceElaboration", "MirroreaProofFirstReferenceSourceTrace", "MirroreaProofFirstReferenceSourceTyping", "MirroreaProofFirstReferenceStore", "MirroreaProofFirstReferenceTrace", "MirroreaProofFirstResourceBoundary", "MirroreaProofFirstSourceAllocation", "MirroreaProofFirstSourceAuthoring", "MirroreaProofFirstSourceCompletion", "MirroreaProofFirstSourceExecution", "MirroreaProofFirstSourceFrame", "MirroreaProofFirstSourcePreservation", "MirroreaProofFirstSourceTypes", "MirroreaProofFirstSourceTyping", "MirroreaProofFirstSupport", "MirroreaProofFirstTrackedValidation", "MirroreaProofFirstWorldProjection", "MixedCancelControls", "MixedCancelEmbedding", "MixedCancelEntry", "MixedCancellation", "MixedCatalogControls", "MixedCatalogEmbedding", "MixedCatalogHistory", "MixedCatalogService", "MixedCatalogSuccessorControls", "MixedCatalogTransitions", "MixedCatalogUse", "MixedCompositionCore", "MixedCoreEmbedding", "MixedExecutionControls", "MixedFallbackStatic", "MixedInstanceState", "MixedManagementControls", "MixedManagementEmbedding", "MixedManagementEntry", "MixedNamedOwnerSource", "MixedNamedOwnerSourceEvidence", "MixedOperationDefinitions", "MixedOwnerAttemptQueue", "MixedOwnerCompletionControls", "MixedOwnerContinuation", "MixedOwnerContinuationControls", "MixedOwnerMaterialization", "MixedOwnerMaterializationControls", "MixedOwnerProgram", "MixedOwnerSourceCode", "MixedOwnerSourceDeclaration", "MixedOwnerSourceDeclarationControls", "MixedOwnerSourceFootprint", "MixedOwnerSourceInvariant", "MixedOwnerSourceIssue", "MixedOwnerSourceIssueControls", "MixedOwnerSourceTrace", "MixedOwnerSourceTraceControls", "MixedPendingProjection", "MixedPureInvocation", "MixedReferenceAccess", "MixedReferenceAllocation", "MixedReferenceAuthority", "MixedReferenceCancellation", "MixedReferenceChronology", "MixedReferenceCoherence", "MixedReferenceControls", "MixedReferenceEmbedding", "MixedReferenceExecution", "MixedReferenceHistory", "MixedReferenceHolding", "MixedReferenceMutation", "MixedReferenceOrigins", "MixedReferenceOwner", "MixedReferenceResult", "MixedReferenceSelection", "MixedReferenceSource", "MixedReferenceSourceData", "MixedReferenceSourceElaboration", "MixedReferenceSourceTrace", "MixedReferenceSourceTyping", "MixedReferenceStore", "MixedReferenceTrace", "MixedRequestAllocation", "MixedRequestControls", "MixedRequestCore", "MixedRequestEmbedding", "MixedSourceCursor", "MixedStoreControls", "OwnerEffectReceipt", "OwnerFlowComposition", "OwnerMetadataManagement", "OwnerMetadataManagementControls", "OwnerMetadataPending", "OwnerMetadataRegistry", "OwnerMetadataSession", "OwnerMetadataSessionControls", "OwnerMetadataSessionInvariant", "OwnerMetadataSource", "OwnerMetadataSourceControls", "OwnerMetadataStore", "OwnerMetadataStoreControls", "OwnerNumericBoundary", "OwnerPartialAbort", "OwnerSavedPending", "OwnerSourceFlow", "OwnerStatementAcceptance", "OwnerStatementAcknowledgment", "OwnerStatementAcknowledgmentControls", "OwnerStatementAdmission", "OwnerStatementBinding", "OwnerStatementBoundaryControls", "OwnerStatementCompletion", "OwnerStatementControls", "OwnerStatementCounterBudget", "OwnerStatementCursor", "OwnerStatementEntryGate", "OwnerStatementExecutionControls", "OwnerStatementHeldAuthority", "OwnerStatementHeldAuthorityControls", "OwnerStatementHistory", "OwnerStatementHistoryOrigin", "OwnerStatementIdentity", "OwnerStatementLiveCustodian", "OwnerStatementMetadataBinding", "OwnerStatementMetadataCursor", "OwnerStatementMetadataHistory", "OwnerStatementOriginalEntry", "OwnerStatementOriginalEntryControls", "OwnerStatementPosition", "OwnerStatementProgram", "OwnerStatementRegistryControls", "OwnerStatementRegistryInvariant", "OwnerStatementRegistryReviewControls", "OwnerStatementRegistrySelection", "OwnerStatementResultCollection", "OwnerStatementResultCollectionControls", "OwnerStatementResultLifecycle", "OwnerStatementResultLifecycleControls", "OwnerStatementResultOrigin", "OwnerStatementResultOriginControls", "OwnerStatementRetainedControls", "OwnerStatementRetainedHistory", "OwnerStatementSession", "OwnerStatementSourceCustodian", "OwnerStatementSourceCustodianControls", "SourceFrontier"]
  let allowed : List Name := [``propext, ``Classical.choice, ``Quot.sound]
  let mut counts : List (String × Nat) := modules.map (fun name => (name,0))
  for (name, _) in env.constants.toList do
    if let some index := env.getModuleIdxFor? name then
      let owner := env.header.moduleNames[index.toNat]!.toString
      if modules.contains owner then
        counts := counts.map (fun (m,k) => (m,if m == owner then k+1 else k))
        for dependency in (← Lean.collectAxioms name) do
          unless allowed.contains dependency do
            throwError "AXIOM_AUDIT_REJECT {name}: {dependency}"
  for (name,count) in counts do
    if count == 0 then throwError "AXIOM_AUDIT_EMPTY {name}"
    logInfo m!"AXIOM_AUDIT_OK {name} {count}"
