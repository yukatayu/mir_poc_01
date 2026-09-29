import MixedSourceCursor
namespace MirroreaProofFirst.OwnerStatementCursor
open MixedSourceCursor

-- An arbitrary raw ready cursor may still hold a stopped item. The shape
-- premise is essential: tick would otherwise overwrite that unresolved item.
def Shape (status : ReferenceSource.Status) (cursor : Cursor Item) : Prop :=
 match status with
 | .ready => cursor.stopped = none
 | .waiting => cursor.stopped.isSome = true
 | .failed _ => True

def Partition (program : List Item) (cursor : Cursor Item) : Prop :=
 cursor.completed ++ cursor.stopped.toList ++ cursor.remaining = program

theorem initial_shape : Shape .ready (Cursor.mk [] program none) := rfl
theorem initial_partition : Partition program (Cursor.mk [] program none) := by simp [Partition]

theorem tick_shape (shape : Shape current.status cursor) :
 Shape (tick current cursor advance complete).outcome.status (tick current cursor advance complete).cursor := by
 cases phase : current.status with
 | failed reason => simp [tick,phase,Shape]
 | waiting =>
   cases completed : complete.status <;> simp_all [tick,Shape]
 | ready =>
   cases remaining : cursor.remaining with
   | nil => simp_all [tick,Shape]
   | cons item rest =>
     cases result : (advance item).status <;> simp_all [tick,Shape]

theorem tick_partition (partition : Partition program cursor) (shape : Shape current.status cursor) :
 Partition program (tick current cursor advance complete).cursor := by
 cases phase : current.status with
 | failed reason => simpa [tick,phase] using partition
 | waiting =>
   by_cases ready : complete.status = .ready
   · simpa [tick,phase,ready,Partition,List.append_assoc] using partition
   · simpa [tick,phase,ready] using partition
 | ready =>
   have stopped : cursor.stopped = none := by simpa [Shape,phase] using shape
   cases remaining : cursor.remaining with
   | nil => simpa [tick,phase,remaining] using partition
   | cons item rest =>
     by_cases ready : (advance item).status = .ready <;>
       simpa [tick,phase,remaining,ready,Partition,stopped,List.append_assoc] using partition

-- A waiting cursor does not inspect or launch its next source statement.
theorem waiting_no_advance (waiting : current.status = .waiting) :
 (tick current cursor advance complete).outcome = complete ∧
 (tick current cursor advance complete).cursor.remaining = cursor.remaining := by
 simp [tick,waiting]

theorem completed_is_prefix (partition : Partition program cursor) :
 ∃ suffix, program = cursor.completed ++ suffix := by
 exact ⟨cursor.stopped.toList ++ cursor.remaining,by simpa [List.append_assoc] using partition.symm⟩

-- Iteration over the same cursor algebra. This proves source bookkeeping only;
-- supplied outcomes do not establish actual owner commits, receipts or auth.
def drive : Nat → Outcome State → Cursor Item → (Item → Outcome State) → Outcome State → Result State Item
 | 0,current,cursor,_,_ => ⟨current,cursor⟩
 | fuel+1,current,cursor,advance,complete =>
   let next := tick current cursor advance complete
   drive fuel next.outcome next.cursor advance complete

theorem drive_valid (partition : Partition program cursor) (shape : Shape current.status cursor) :
 Partition program (drive fuel current cursor advance complete).cursor ∧
 Shape (drive fuel current cursor advance complete).outcome.status (drive fuel current cursor advance complete).cursor := by
 induction fuel generalizing current cursor with
 | zero => exact ⟨partition,shape⟩
 | succ fuel ih => exact ih (tick_partition partition shape) (tick_shape shape)

#print axioms tick_shape
#print axioms tick_partition
#print axioms waiting_no_advance
#print axioms completed_is_prefix
#print axioms drive_valid
end MirroreaProofFirst.OwnerStatementCursor
