import MirroreaProofFirstOwnerInvocationContext
import Lean
set_option maxRecDepth 10000
open Lean Elab Command
run_cmd do
  let env ← getEnv
  let modules : List String := ["MirroreaProofFirstPassive", "MirroreaProofFirstProducerFlow", "MirroreaProofFirstFallibleFlow", "MirroreaProofFirstOwnerAssignment", "MirroreaProofFirstOwnerPartial", "MirroreaProofFirstOwnerReadCoverage", "MirroreaProofFirstOwnerCheckedArithmetic", "MirroreaProofFirstOwnerReadReport", "MirroreaProofFirstOwnerRecordedArithmetic", "MirroreaProofFirstResourceBoundary", "MirroreaProofFirstLocalContract", "MirroreaProofFirstContractExport", "MirroreaProofFirstInstancePrograms", "MirroreaProofFirstSupport", "MirroreaProofFirstDynamicSupport", "MirroreaProofFirstCurrentUse", "MirroreaProofFirstDynamicIdentity", "MirroreaProofFirstTrackedValidation", "MirroreaProofFirstGraphValidation", "MirroreaProofFirstDynamicGraphs", "MirroreaProofFirstInstanceState", "MirroreaProofFirstWorldProjection", "MirroreaProofFirstCompositionCore", "MirroreaProofFirstManagementEntry", "MirroreaProofFirstInvocationBoundary", "MirroreaProofFirstReferenceCancellationBoundary", "MirroreaProofFirstCompositionMachine", "MirroreaProofFirstCatalogHistory", "MirroreaProofFirstFallbackStatic", "MirroreaProofFirstReferenceAccess", "MirroreaProofFirstPureFunctions", "MirroreaProofFirstFunctionContractBridge", "MirroreaProofFirstModuleContractBoundary", "MirroreaProofFirstAdmissionPhases", "MirroreaProofFirstOwnerEffectService", "MirroreaProofFirstOwnerStructuredKeys", "MirroreaProofFirstOwnerSourceContext", "MirroreaProofFirstOwnerCheckedSchema", "MirroreaProofFirstOwnerRequiredSchema", "MirroreaProofFirstOwnerTypedIndex", "MirroreaProofFirstOwnerArgumentPresence", "MirroreaProofFirstOwnerArgumentDomain", "MirroreaProofFirstOwnerInvocationContext"]
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
