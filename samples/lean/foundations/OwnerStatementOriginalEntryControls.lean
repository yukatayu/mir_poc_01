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

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls
open OwnerStatementOriginalEntry
structure SourcePlan where
 handler : Nat
 ordinal : Nat
 payload : Nat
 deriving DecidableEq

def sourceFirst : SourcePlan := ⟨1,0,100⟩
def sourceSecond : SourcePlan := ⟨1,1,200⟩
def sourceOther : SourcePlan := ⟨2,0,300⟩
def sourceRoot := [sourceFirst,sourceOther,sourceSecond]
def completeSource := [sourceFirst,sourceSecond]
#guard manifestCheck SourcePlan.handler sourceRoot completeSource
#guard !(manifestCheck SourcePlan.handler sourceRoot [])
#guard !(manifestCheck SourcePlan.handler sourceRoot [sourceSecond])
#guard !(manifestCheck SourcePlan.handler sourceRoot [sourceFirst])
#guard !(manifestCheck SourcePlan.handler sourceRoot [sourceSecond,sourceFirst])
#guard !(manifestCheck SourcePlan.handler sourceRoot [sourceFirst,sourceFirst,sourceSecond])
#guard !(manifestCheck SourcePlan.handler sourceRoot [sourceFirst,sourceOther,sourceSecond])
#guard manifestCheck SourcePlan.handler sourceRoot [sourceOther]
-- Authentic membership alone admits the decisive second-only counterexample.
#guard [sourceSecond].all (fun p => sourceRoot.contains p)
end MirroreaProofFirst.OwnerStatementOriginalEntryControls

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls
open OwnerStatementOriginalEntry
#guard replacementCheck SourcePlan.handler completeSource sourceRoot 1
#guard manifestCheck SourcePlan.handler (completeSource ++ [sourceOther]) completeSource
#guard !(replacementCheck SourcePlan.handler completeSource [sourceSecond] 1)
#guard !(replacementCheck SourcePlan.handler completeSource [sourceSecond,sourceFirst] 1)
#guard !(replacementCheck SourcePlan.handler completeSource (completeSource ++ [sourceFirst]) 1)
end MirroreaProofFirst.OwnerStatementOriginalEntryControls

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls
open OwnerStatementOriginalEntry.Publication
-- Frozen logical payload and unrelated configuration vary independently.
def ownedSource : Owned Nat Nat := ⟨19,2⟩
def stagedSource : Candidate Nat Nat := ⟨19,19,7⟩
#guard publish (some ownedSource,none) stagedSource = some (none,some ⟨19,7⟩)
#guard publish (some {ownedSource with source := 20},none) stagedSource = none
#guard publish (some ownedSource,none) {stagedSource with preview := 20} = none
#guard publish (none,none) stagedSource = none
#guard publish (some ownedSource,some ownedSource) stagedSource = none
end MirroreaProofFirst.OwnerStatementOriginalEntryControls


namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.DependencyFrame
open OwnerStatementOriginalEntry.DependencyFrame
def graph (key : Nat) : Option (List Nat) := if key = 2 then some [1] else none
def shortcut (key : Nat) : Option (List Nat) := if key = 2 then some [0] else none
def extended (key : Nat) : Option (List Nat) := if key = 3 then some [2] else graph key
#guard queriesCheck [2] graph extended
#guard !(queriesCheck [2] graph shortcut)
#guard traceCheck [10,11] [10,11,12]
#guard !(traceCheck [10,11] [10,12])
def ownerOperation (shared parked : Nat) := (shared+1,parked+1,shared)
#guard visible (ownerStep ownerOperation ⟨200,0,7⟩) = visible (ownerStep ownerOperation ⟨200,999,7⟩)
#guard visible (ownerStep ownerOperation ⟨200,0,7⟩) ≠ visible (ownerStep ownerOperation ⟨999,0,7⟩)
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.DependencyFrame

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.DecodedOrigin
open OwnerStatementOriginalEntry.DecodedOrigin
-- Numeric tags stand only for full typed payloads in these finite controls.
def original : Image (List (Nat × Option Nat)) := ⟨[(1,some 0),(2,none)],[[(3,some 1)]]⟩
def erased : Decoded (List (Nat × Option Nat)) := ⟨⟨[(1,none),(2,none)],original.nested⟩⟩
def nestedErased : Decoded (List (Nat × Option Nat)) := ⟨⟨original.primary,[[(3,none)]]⟩⟩
#guard promote original ⟨original⟩ = some ⟨original⟩
#guard promote original erased = none
#guard promote original nestedErased = none
#guard OwnerStatementOriginalEntry.check Prod.snd original.primary false (2,none)
#guard !(OwnerStatementOriginalEntry.check Prod.snd original.primary false (1,none))
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.DecodedOrigin

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.TraceAllocator
open OwnerStatementOriginalEntry.TraceAllocator

def original : Trace Nat := ⟨[(0,10),(1,20)],2⟩
def rewind : Trace Nat := ⟨original.rows,0⟩
def duplicate : Trace Nat := ⟨original.rows ++ [(1,99)],3⟩
def future : Trace Nat := ⟨original.rows ++ [(3,99)],3⟩
#guard validCheck original
#guard !(extensionCheck original rewind)
#guard !(extensionCheck original duplicate)
#guard !(extensionCheck original future)
#guard extensionCheck original (append original 30)
#guard push 2 original 30 = none
#guard push 3 original 30 = some (append original 30)
#guard (append original 30).rows.find? (fun row => row.1 == 0) = some (0,10)
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.TraceAllocator
