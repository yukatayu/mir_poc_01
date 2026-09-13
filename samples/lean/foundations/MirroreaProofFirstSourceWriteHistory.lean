import MirroreaProofFirstSourceFrame

namespace MirroreaProofFirst.SourceWriteHistory
open SourceAuthoring SourceExecution

-- Both sides refer to the actual values and actual writes of one execution.
-- Empty generated entry satisfies this; arbitrary injected histories need proof.
def Aligned (s : Execution p a) : Prop :=
  ∀ name, (s.writes.find? (fun w => w.name == name)).map Write.value = SourceAuthoring.lookup s.source.values name

inductive Numbered : List Write → Prop where
  | nil : Numbered []
  | cons : w.ordinal = rest.length → Numbered rest → Numbered (w :: rest)

def Referenced (writes : List Write) : Prop :=
  ∀ w ∈ writes, ∀ r ∈ w.inputs,
    match r.producer with
    | none => r.value = none
    | some ordinal => ∃ producer ∈ writes, producer.ordinal = ordinal ∧ producer.name = r.name ∧
        r.value = some producer.value ∧ producer.ordinal < w.ordinal

structure Invariant (s : Execution p a) : Prop where
  aligned : Aligned s
  numbered : Numbered s.writes
  referenced : Referenced s.writes

theorem empty (s : State p a) (initial : s.values = []) : Invariant ⟨s,[]⟩ := by
  refine ⟨?_,.nil,?_⟩
  · intro name; simp [initial,SourceAuthoring.lookup]
  · intro w member; cases member

theorem ordinal_lt (writes : List Write) (numbered : Numbered writes) (w : Write) (member : w ∈ writes) :
    w.ordinal < writes.length := by
  induction numbered with
  | nil => cases member
  | @cons head rest eq numbered ih =>
      rcases List.mem_cons.mp member with rfl | member
      · simp [eq]
      · exact Nat.lt_trans (ih member) (by simp)

theorem ordinal_unique (writes : List Write) (numbered : Numbered writes)
    (left right : Write) (hl : left ∈ writes) (hr : right ∈ writes) (same : left.ordinal = right.ordinal) : left = right := by
  induction numbered with
  | nil => cases hl
  | @cons rest head eq numbered ih =>
      rcases List.mem_cons.mp hl with leftHead | leftTail
      · rcases List.mem_cons.mp hr with rightHead | rightTail
        · exact leftHead.trans rightHead.symm
        · have less := ordinal_lt rest numbered right rightTail
          have hleft : left.ordinal = rest.length := leftHead ▸ eq
          omega
      · rcases List.mem_cons.mp hr with rightHead | rightTail
        · have less := ordinal_lt rest numbered left leftTail
          have hright : right.ordinal = rest.length := rightHead ▸ eq
          omega
        · exact ih leftTail rightTail

theorem append_aligned (s : Execution p a) (item : Located) (next : State p a) (value : Value)
    (aligned : Aligned s)
    (output : SourceAuthoring.lookup next.values (outputName item.statement) = some value)
    (frame : SourceFrame.Frame s.source.values next.values (outputName item.statement)) :
    Aligned (appendWrite s item next value) := by
  intro name
  by_cases eq : name = outputName item.statement
  · subst name; simpa [appendWrite,event] using output.symm
  · simpa [appendWrite,event,Ne.symm eq] using (aligned name).trans (frame name eq).symm

theorem append_referenced (s : Execution p a) (item : Located) (next : State p a) (value : Value)
    (valid : Invariant s) : Referenced (appendWrite s item next value).writes := by
  intro w member r input
  rcases List.mem_cons.mp member with rfl | member
  · obtain ⟨name,_,rfl⟩ := List.mem_map.mp input
    change (match ((s.writes.find? (fun w => w.name == name)).map Write.ordinal) with
      | none => SourceAuthoring.lookup s.source.values name = none
      | some ordinal => ∃ producer ∈ event s item next value :: s.writes,
          producer.ordinal = ordinal ∧ producer.name = name ∧
          SourceAuthoring.lookup s.source.values name = some producer.value ∧
          producer.ordinal < s.writes.length)
    cases found : s.writes.find? (fun w => w.name == name) with
    | none => simpa [found] using (valid.aligned name).symm
    | some producer =>
        have member := List.mem_of_find?_eq_some found
        have key : producer.name = name := by
          simpa using List.find?_some found
        exact ⟨producer,List.mem_cons_of_mem _ member,rfl,key,
          by simpa [found] using (valid.aligned name).symm,
          ordinal_lt s.writes valid.numbered producer member⟩
  · have old := valid.referenced w member r input
    cases found : r.producer with
    | none => simpa [found] using old
    | some ordinal =>
        obtain ⟨producer,hp,e1,e2,e3,e4⟩ := (show ∃ producer ∈ s.writes,
          producer.ordinal = ordinal ∧ producer.name = r.name ∧ r.value = some producer.value ∧ producer.ordinal < w.ordinal
          from by simpa [found] using old)
        exact ⟨producer,List.mem_cons_of_mem _ hp,e1,e2,e3,e4⟩

theorem advance_preserves (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (next : Execution p a) (valid : Invariant s)
    (accepted : advance s member place principal item = .accepted next) : Invariant next := by
  obtain ⟨env,state,value,_,he,hv,rfl⟩ := (advance_exact _ _ _ _ _ _).mp accepted
  exact ⟨append_aligned _ _ _ _ valid.aligned hv (SourceFrame.execute_frame _ _ _ _ _ _ he),
    .cons rfl valid.numbered,append_referenced _ _ _ _ valid⟩

theorem run_preserves (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (valid : Invariant s) : Invariant (SourceExecution.run s member place principal program).final := by
  induction program generalizing s with
  | nil => exact valid
  | cons item rest ih =>
      unfold SourceExecution.run
      cases h : advance s member place principal item with
      | rejected reason => exact valid
      | accepted next => exact ih next (advance_preserves _ _ _ _ _ _ valid h)

#print axioms ordinal_unique
#print axioms advance_preserves
#print axioms run_preserves
end MirroreaProofFirst.SourceWriteHistory
