import MirroreaProofFirstReferenceSource
namespace MirroreaProofFirst.MixedSourceCursor

-- Factor only the existing Session cursor bookkeeping. State transitions and
-- program admission remain supplied by their actual source executor. There is
-- no new queue, effect, request allocator or authority in this module.
structure Cursor (Item : Type) where
 completed : List Item
 remaining : List Item
 stopped : Option Item
 deriving DecidableEq
structure Outcome (State : Type) where
 state : State
 status : ReferenceSource.Status
structure Result (State Item : Type) where
 outcome : Outcome State
 cursor : Cursor Item

def tick (current : Outcome State) (cursor : Cursor Item)
 (advance : Item → Outcome State) (complete : Outcome State) : Result State Item :=
 match current.status with
 | .failed _ => ⟨current,cursor⟩
 | .waiting =>
   ⟨complete,{cursor with
     completed := if complete.status = .ready then cursor.completed ++ cursor.stopped.toList else cursor.completed,
     stopped := if complete.status = .ready then none else cursor.stopped}⟩
 | .ready => match cursor.remaining with
   | [] => ⟨current,cursor⟩
   | item::rest =>
     let result := advance item
     ⟨result,⟨if result.status = .ready then cursor.completed ++ [item] else cursor.completed,
       rest,if result.status = .ready then none else some item⟩⟩

-- Cursor updates never manufacture a completion or discard an uncompleted
-- source item: exact before/after laws, valid for every executor and item type.
theorem failure_retained (failed : current.status = .failed reason) :
 tick current cursor advance complete = ⟨current,cursor⟩ := by simp [tick,failed]
theorem waiting_refused (waiting : current.status = .waiting) (refused : complete.status ≠ .ready) :
 (tick current cursor advance complete).outcome = complete ∧
 (tick current cursor advance complete).cursor = cursor := by
 simp [tick,waiting,refused]
theorem waiting_completed (waiting : current.status = .waiting) (ready : complete.status = .ready) :
 (tick current cursor advance complete).outcome = complete ∧
 (tick current cursor advance complete).cursor =
  ⟨cursor.completed ++ cursor.stopped.toList,cursor.remaining,none⟩ := by simp [tick,waiting,ready]
theorem item_blocked (ready : current.status = .ready) (remaining : cursor.remaining = item::rest)
 (blocked : (advance item).status ≠ .ready) :
 (tick current cursor advance complete).outcome = advance item ∧
 (tick current cursor advance complete).cursor = ⟨cursor.completed,rest,some item⟩ := by
 simp [tick,ready,remaining,blocked]
theorem item_completed (ready : current.status = .ready) (remaining : cursor.remaining = item::rest)
 (completed : (advance item).status = .ready) :
 (tick current cursor advance complete).outcome = advance item ∧
 (tick current cursor advance complete).cursor = ⟨cursor.completed ++ [item],rest,none⟩ := by
 simp [tick,ready,remaining,completed]

#print axioms failure_retained
#print axioms waiting_refused
#print axioms waiting_completed
#print axioms item_blocked
#print axioms item_completed
end MirroreaProofFirst.MixedSourceCursor
