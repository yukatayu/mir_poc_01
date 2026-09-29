import OwnerStatementOriginalEntry
namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls
open OwnerStatementOriginalEntry
structure Plan where
 operation : Nat
 ordinal : Option Nat
 checkedPayload : Nat
 deriving DecidableEq

def ordinary : Plan := ⟨10,none,100⟩
def guardedPlan : Plan := ⟨11,some 0,101⟩
def root := [ordinary,guardedPlan]
def erased : Plan := {guardedPlan with ordinal := none}
def replaced : Plan := {ordinary with checkedPayload := 999}
#guard check Plan.ordinal root false ordinary
#guard !(check Plan.ordinal root false guardedPlan)
#guard !(check Plan.ordinal root false erased)
#guard !(check Plan.ordinal root false replaced)
#guard !(check Plan.ordinal root true ordinary)
#guard check Plan.ordinal (root ++ [⟨12,none,102⟩]) false ordinary

-- Two actual alternatives have different decisive failures.
def blanket := root.isEmpty
#guard blanket = false
-- Ignoring independently retained origin permits a stripped protected child.
def mutableOnly (candidate : Plan) : Bool := candidate.ordinal.isNone
#guard mutableOnly erased = true
#guard check Plan.ordinal root false erased = false
end MirroreaProofFirst.OwnerStatementOriginalEntryControls
