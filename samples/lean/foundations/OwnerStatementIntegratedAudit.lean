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
  let modules : List String := ["MirroreaProofFirstResourceBoundary", "MirroreaProofFirstLocalContract", "MirroreaProofFirstContractExport", "MirroreaProofFirstInstancePrograms", "MirroreaProofFirstSupport", "MirroreaProofFirstDynamicSupport", "MirroreaProofFirstCurrentUse", "MirroreaProofFirstDynamicIdentity", "MirroreaProofFirstTrackedValidation", "MirroreaProofFirstGraphValidation", "MirroreaProofFirstDynamicGraphs", "MirroreaProofFirstInstanceState", "MirroreaProofFirstWorldProjection", "MirroreaProofFirstCompositionCore", "MirroreaProofFirstManagementEntry", "MirroreaProofFirstInvocationBoundary", "MirroreaProofFirstReferenceCancellationBoundary", "MirroreaProofFirstCompositionMachine", "MirroreaProofFirstCatalogHistory", "MirroreaProofFirstFallbackStatic", "MirroreaProofFirstReferenceAccess", "MirroreaProofFirstReferenceAccessHistory", "MirroreaProofFirstReferenceSelection", "MirroreaProofFirstReferenceHolding", "MirroreaProofFirstReferenceOwner", "MirroreaProofFirstReferenceCancellation", "MirroreaProofFirstReferenceStore", "MirroreaProofFirstReferenceCoherence", "MirroreaProofFirstReferenceMutation", "MirroreaProofFirstReferenceResult", "MirroreaProofFirstReferenceTrace", "MirroreaProofFirstReferenceExecution", "MirroreaProofFirstSourceAuthoring", "MirroreaProofFirstSourceTypes", "MirroreaProofFirstSourcePreservation", "MirroreaProofFirstSourceTyping", "MirroreaProofFirstReferenceSourceData", "MirroreaProofFirstSourceExecution", "MirroreaProofFirstSourceFrame", "MirroreaProofFirstInvocationContract", "MirroreaProofFirstSourceCompletion", "MirroreaProofFirstSourceAllocation", "MirroreaProofFirstReferenceAllocation", "MirroreaProofFirstReferenceSource", "MixedSourceCursor", "OwnerStatementCursor", "MirroreaProofFirstPassive", "MirroreaProofFirstProducerFlow", "MirroreaProofFirstFallibleFlow", "MirroreaProofFirstOwnerAssignment", "MirroreaProofFirstOwnerPartial", "MirroreaProofFirstAbortFlow", "OwnerPartialAbort", "MirroreaProofFirstOwnerReadCoverage", "MirroreaProofFirstOwnerCheckedArithmetic", "MirroreaProofFirstOwnerReadReport", "MirroreaProofFirstOwnerRecordedArithmetic", "MirroreaProofFirstPureFunctions", "MirroreaProofFirstFunctionContractBridge", "MirroreaProofFirstModuleContractBoundary", "MirroreaProofFirstAdmissionPhases", "MirroreaProofFirstOwnerEffectService", "OwnerSourceFlow", "OwnerFlowComposition", "OwnerNumericBoundary", "MixedOperationDefinitions", "MixedInstanceState", "MixedCatalogEmbedding", "MixedCatalogService", "MixedCatalogControls", "MixedCatalogUse", "MixedPureInvocation", "MixedCompositionCore", "MixedManagementEntry", "MixedCoreEmbedding", "MixedManagementEmbedding", "OwnerSavedPending", "SourceFrontier", "OwnerEffectReceipt", "MixedRequestCore", "MixedRequestEmbedding", "MixedCancellation", "MixedCancelEntry", "MixedCancelEmbedding", "MixedCatalogHistory", "MixedFallbackStatic", "MixedReferenceSourceData", "MixedNamedOwnerSource", "MixedNamedOwnerSourceEvidence", "MixedOwnerMaterialization", "MixedOwnerSourceFootprint", "MixedOwnerSourceCode", "MirroreaProofFirstReferenceSourceElaboration", "MirroreaProofFirstReferenceSourceTyping", "MirroreaProofFirstReferenceSourceTrace", "MirroreaProofFirstReferenceAuthority", "MixedReferenceAccess", "MixedReferenceEmbedding", "MixedReferenceHistory", "MixedReferenceSelection", "MixedReferenceHolding", "MixedReferenceOwner", "MixedReferenceCancellation", "MixedCatalogTransitions", "MirroreaProofFirstOwnerStructuredKeys", "MirroreaProofFirstOwnerSourceContext", "OwnerMetadataRegistry", "OwnerMetadataManagement", "MixedReferenceStore", "MixedReferenceCoherence", "MixedReferenceMutation", "MixedReferenceResult", "MixedReferenceTrace", "MixedPendingProjection", "MixedReferenceExecution", "MixedRequestAllocation", "MixedReferenceAllocation", "MixedReferenceSource", "MixedReferenceSourceElaboration", "MixedReferenceSourceTyping", "MixedReferenceSourceTrace", "MixedReferenceAuthority", "MixedOwnerSourceIssue", "MixedOwnerSourceDeclaration", "MixedOwnerProgram", "MixedOwnerAttemptQueue", "MixedOwnerSourceTrace", "MixedOwnerSourceInvariant", "MixedOwnerContinuation", "OwnerStatementSession", "MixedReferenceChronology", "MixedReferenceOrigins", "OwnerMetadataStore", "OwnerMetadataSource", "OwnerMetadataPending", "OwnerMetadataSession", "OwnerStatementMetadataCursor", "OwnerMetadataSessionInvariant", "OwnerStatementAcceptance", "OwnerStatementBinding", "OwnerStatementMetadataBinding", "OwnerStatementHistory", "OwnerStatementMetadataHistory", "OwnerStatementProgram", "OwnerStatementAdmission", "OwnerStatementPosition", "OwnerStatementCompletion", "OwnerStatementHistoryOrigin", "MixedManagementControls", "MixedReferenceControls", "MixedCatalogSuccessorControls", "MixedRequestControls", "MixedStoreControls", "MixedCancelControls", "MixedExecutionControls", "MixedOwnerMaterializationControls", "MixedOwnerSourceIssueControls", "MixedOwnerCompletionControls", "MixedOwnerSourceTraceControls", "MixedOwnerSourceDeclarationControls", "MixedOwnerContinuationControls", "OwnerMetadataManagementControls", "OwnerMetadataStoreControls", "OwnerMetadataSourceControls", "OwnerMetadataSessionControls", "OwnerStatementExecutionControls", "OwnerStatementBoundaryControls", "OwnerStatementIdentity", "OwnerStatementControls", "OwnerStatementRetainedHistory", "OwnerStatementRetainedControls"]
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
