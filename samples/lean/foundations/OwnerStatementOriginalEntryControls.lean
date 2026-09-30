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

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.CountedTrace
open OwnerStatementOriginalEntry.TraceAllocator
open OwnerStatementOriginalEntryControls.TraceAllocator
def jumped : Trace Nat := ⟨original.rows,100⟩
#guard extensionCheck original jumped
#guard !(countedCheck original jumped)
#guard countedCheck original (append original 30)
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.CountedTrace

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.ResultSlots
open OwnerStatementOriginalEntry.ResultSlots
def initial : State Nat Nat := ⟨[0,1],[],[],false,0⟩
def begun : State Nat Nat := ⟨[0,1],[],[],true,0⟩
def recorded : State Nat Nat := ⟨[0,1],[],[10],true,0⟩
def beforeFinalAck : State Nat Nat := ⟨[0,1],[],[10,20],true,1⟩
def complete : State Nat Nat := ⟨[0,1],[],[10,20],false,2⟩
#guard (prepare 1 [0,1] ([] : List Nat)).isNone
#guard (prepare 2 [0,1] ([] : List Nat)).isSome
#guard (begin initial).isSome
#guard Owed begun == 2
#guard (publish begun 10).isSome
#guard Owed recorded == 1
#guard (accept begun).isNone
#guard (publish recorded 99).isNone
#guard (accept recorded).isSome
#guard (again 4 beforeFinalAck).isNone
#guard (again 3 complete).isNone
#guard (again 4 complete).isSome
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.ResultSlots

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.CarrierFrame
open OwnerStatementOriginalEntry.CarrierFrame
-- Even tags identify the original request here; messages are full payloads
-- in the general theorem. Tag-only fixtures do not authenticate real envelopes.
def original (n : Nat) : Bool := n % 2 == 0
def before : Queues Nat := ⟨[2],[1,2,3]⟩
def lostReply : Queues Nat := ⟨[],before.incoming⟩
def skippedHead : Queues Nat := ⟨before.outgoing,[2,3]⟩
def unrelated : Queues Nat := ⟨before.outgoing++[5],before.incoming++[7]⟩
#guard !(check original before lostReply)
#guard !(check original before skippedHead)
#guard check original before unrelated
#guard barrier original before.incoming == [1,2]
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.CarrierFrame

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.ReadOrigin
open OwnerStatementOriginalEntry.ReadOrigin
-- Keys stand for exact owner+namespace+index+field, never bare field names.
def prior : Write Nat Nat := ⟨1,10,1⟩
def latest : Write Nat Nat := ⟨1,20,2⟩
def other : Write Nat Nat := ⟨2,20,3⟩
def history : List (Write Nat Nat) := [other,latest,prior]
#guard bindRead 1 history 20 = some 2
#guard bindRead 1 history 10 = none
#guard bindRead 2 history 20 = some 3
#guard bindRead 9 history 20 = none
#guard replay (fun _ => some 99) 1 history = some 20
#guard replay (fun _ => some 99) 9 history = some 99
#guard project (fun _ => false) (fun n => n == 2) [2,3] = [2]
#guard project (fun n => n == 3) (fun n => n == 2) [2,3] = [2,3]
#guard project (fun _ => false) (fun n => n == 2) [3] = []
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.ReadOrigin

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.ReportReservation
open OwnerStatementOriginalEntry.ReportReservation
def held : State := ⟨6,3⟩
#guard foreign 9 1 held = none
#guard foreign 10 1 held = some ⟨7,3⟩
#guard discharge 3 held = some ⟨9,0⟩
#guard discharge 4 held = none
#guard acquire 10 2 held = none
#guard acquire 10 1 held = some ⟨6,4⟩
#guard release 1 held = some ⟨6,2⟩
#guard release 4 held = none
#guard foreign 9 0 held = some held
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.ReportReservation

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.ReportPhases
open OwnerStatementOriginalEntry.ReportReservation
#guard reportOwed 4 0 0 false = 4
#guard reportOwed 4 1 0 false = 4
#guard reportOwed 4 4 1 true = 3
#guard reportOwed 4 1 1 true = 0
#guard reportOwed 4 4 3 true = 1
#guard foreign 10 3 ⟨6,1⟩ = some ⟨9,1⟩
#guard foreign 10 1 ⟨6,1⟩ = some ⟨7,1⟩
#guard foreign 10 2 ⟨6,3⟩ = none
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.ReportPhases

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.EndpointBudget
open OwnerStatementOriginalEntry.EndpointBudget
#guard remote.sum = 17
#guard localWork.sum = 1
#guard admits 17 0 0 0 0 remote = true
#guard admits 16 0 0 0 0 remote = false
#guard admits 17 4 1 0 0 [3,1,4,3,1,1] = false
#guard admits 18 0 0 0 1 remote = true
#guard admits 21 0 3 1 0 remote = true
#guard admits 20 0 3 1 0 remote = false
#guard admits 20 0 3 0 0 remote = true
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.EndpointBudget

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.FiniteScan
open OwnerStatementOriginalEntry.FiniteScan
def allows (n : Nat) : Bool := n == 3
#guard scan allows [1,2,3,4] = some 3
#guard scan allows [1,2] = none
#guard examined allows [1,2,3,4] = 3
#guard withFallback allows [1,2] (some 99) = some (.inr 99)
#guard withFallback allows [1,3] (some 99) = some (.inl 3)
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.FiniteScan

namespace MirroreaProofFirst.OwnerStatementOriginalEntryControls.FiniteScan
open OwnerStatementOriginalEntry.FiniteScan
#guard heads [[1,3],[],[3]] = [1,3]
#guard scan allows (heads [[1,3]]) = none
#guard scan allows (heads [[1,3],[3]]) = some 3
#guard withFallback allows [1,2] (scan allows (heads [[1,3],[3]])) = some (.inr 3)
end MirroreaProofFirst.OwnerStatementOriginalEntryControls.FiniteScan
