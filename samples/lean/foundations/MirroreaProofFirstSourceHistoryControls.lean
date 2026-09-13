import MirroreaProofFirstSourceReadNames
import MirroreaProofFirstSourceAllocation
import MirroreaProofFirstSourceLocators

namespace MirroreaProofFirst.SourceHistoryControls
open SourceAuthoring SourceExecution

def initial : State 3 1 := ⟨⟨ManagementEntry.Controls.initial,[],[]⟩,[],0,[]⟩
def program : List Located := [⟨10,.localValue "x" true (.integer 0)⟩,
  ⟨20,.assign "x" (.add (.read "x") (.integer 1))⟩,
  ⟨30,.localValue "y" false (.read "x")⟩]
def result := SourceExecution.run ⟨initial,[]⟩ 0 0 7 program
example : SourceWriteHistory.Invariant result.final :=
  SourceWriteHistory.run_preserves ⟨initial,[]⟩ 0 0 7 program (SourceWriteHistory.empty initial rfl)
def allocated := SourceAllocation.execution_preserves ⟨initial,[]⟩ 0 0 7 program
  (SourceAllocation.empty initial rfl rfl)
def inputsPresent := SourceReadNames.run_present ⟨initial,[]⟩ 0 0 7 program
  (by intro w member; cases member)
#guard result.rejected.isNone
#guard result.final.writes.map Write.ordinal = [2,1,0]
#guard (result.final.writes.find? (fun w => w.name == "y")).map Write.inputs =
  some [⟨"x",some (.integer 1 true),some 1⟩]

-- Mere order accepted this forged latest value; actual-value alignment rejects.
def forged : Execution 3 1 :=
  ⟨{initial with values := [("x",.integer 2 true)]},[⟨0,10,"x",.integer 1 true,[],0,0⟩]⟩
example : Ordered forged := by simp [Ordered,forged]
theorem forged_rejected : ¬ SourceWriteHistory.Aligned forged := by
  intro aligned
  have h := aligned "x"
  simp [forged,SourceAuthoring.lookup] at h

-- System safety alone does not make a supplied allocator counter fresh.
def reused : State 3 1 :=
  {initial with machine := {initial.machine with
    system := {initial.machine.system with used := [⟨91,7,0⟩]}}}
example : CompositionMachine.Invariant reused.machine := by
  refine ⟨⟨(CatalogHistory.empty_valid ManagementEntry.Controls.initial.configuration.state).toValid,?_⟩,?_,?_⟩
  · simp [reused,initial]
  · simp [reused,initial]
  · intro t member; cases member
#guard !CompositionMachine.fresh reused.machine ⟨91,7,0⟩
theorem reused_rejected : ¬ SourceAllocation.Invariant reused := by
  intro h
  have less := h.1 ⟨91,7,0⟩ (by simp [reused,initial])
  simp [reused,initial] at less

-- A type-correct external value can still name a nonexistent instance.
def missingLocator : State 3 1 := {initial with values := [("ghost",.callable 0)]}
#guard SourceTypes.lookup (SourceTyping.environment missingLocator.values) "ghost" = some .callable
theorem missing_locator_rejected : ¬ SourceLocators.Invariant missingLocator := by
  intro h
  have valid := h.2 ("ghost",.callable 0) (by simp [missingLocator])
  simp [SourceLocators.ValueValid,missingLocator,initial,ManagementEntry.Controls.initial,
    CompositionCore.config] at valid

#print axioms missing_locator_rejected
#print axioms forged_rejected
#print axioms reused_rejected
end MirroreaProofFirst.SourceHistoryControls
