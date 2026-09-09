import MirroreaProofFirstGraphValidation
open MirroreaProofFirst.TrackedValidation MirroreaProofFirst.GraphValidation
namespace GraphReview
namespace MeaningfulLive
def start : Fin 2 → Cell (List (Fin 2)) := fun k => ⟨0,if k = 1 then some [0] else none⟩
def patch : List (Fin 2 × Option (List (Fin 2))) := [(0,some [1])]
def finish := write start 1 (some [])
def reads : List (Fin 2 × Cell (List (Fin 2))) := [(0,start 0),(1,finish 1)]
example : PatchPreparation patch start (dagQuery 2) finish true reads := by
  apply PatchPreparation.read
  apply PatchPreparation.mutate (k := 1) (v := some [])
  apply PatchPreparation.read
  exact PatchPreparation.done
#guard checkObservedStamps finish reads
#guard !(evaluate (overlay (fun k => (start k).value) patch) (dagQuery 2)).1
#guard checkAcyclic (graphOf (fun k => (publish finish patch k).value))
#guard (commit start finish patch (dagQuery 2)).isNone
end MeaningfulLive
namespace OneVertex
def start : Fin 1 → Cell (List (Fin 1)) := fun _ => ⟨0,none⟩
def patch : List (Fin 1 × Option (List (Fin 1))) := [(0,some [])]
def finish := write start 0 (some [])
example : PatchPreparation patch start (dagQuery 1) finish true [(0,finish 0)] := by
  apply PatchPreparation.mutate (k := 0) (v := some [])
  apply PatchPreparation.read
  exact PatchPreparation.done
#guard checkObservedStamps finish [(0,finish 0)]
#guard (commit start finish patch (dagQuery 1)).isNone
#guard (publish finish patch 0).version == 2
-- Equal stamps without lawful history do not authenticate values.
def forged : Fin 1 → Cell (List (Fin 1)) := fun _ => ⟨0,some [0]⟩
#guard (commit start forged [] (dagQuery 1)).isSome
#guard !checkAcyclic (graphOf (fun k => (forged k).value))
-- Duplicate keys: safe endpoint, unsafe sequential prefix.
def duplicate : List (Fin 1 × Option (List (Fin 1))) := [(0,some []),(0,some [0])]
#guard (commit start start duplicate (dagQuery 1)).isSome
#guard (publish start duplicate 0).version == 2
#guard !checkAcyclic (graphOf (fun k => (write start 0 (some [0]) k).value))
end OneVertex
-- Finite graph absence is not an existence predicate.
def absent : Fin 2 → Cell (List (Fin 2)) := fun _ => ⟨0,none⟩
#guard (commit absent absent [(0,some [1])] (dagQuery 2)).isSome
#guard (publish absent [(0,some [1])] 1).value == none
#guard (commit GraphCommitControls.start (write GraphCommitControls.start 2 (some [])) [(0,some [1])] (dagQuery 3)).isNone
#guard checkAcyclic (fun (_ _ : Fin 0) => false)
-- Distinct acyclic relations need not have an acyclic union.
#guard checkAcyclic (fun (x y : Fin 2) => x == 0 && y == 1)
#guard checkAcyclic (fun (x y : Fin 2) => x == 1 && y == 0)
#guard !checkAcyclic (fun (x y : Fin 2) => x != y)
-- Validation's Boolean can distinguish a protected reverse edge.
#guard (evaluate (overlay (fun (_ : Fin 2) => none) MeaningfulLive.patch) (dagQuery 2)).1
#guard !(evaluate (overlay (fun k => (MeaningfulLive.start k).value) MeaningfulLive.patch) (dagQuery 2)).1
end GraphReview
