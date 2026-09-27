import BoundWriterStatements
open MirroreaProofFirst SharedHostCaptureReplay OwnerCommitJournal
namespace StatementClosureHistory
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Occurrence positions come from the same cursor immediately before confirm.
-- Values alone are not an occurrence key, including repeated equal operations.
structure Captured (p a : Nat) where
  openedAt : Nat
  confirmedAt : Nat
  observations : List (OwnerStatementJournal.Observation p a)

def Matches (closed : RecordedClosed base) (captured : Captured p a) : Prop :=
  captured.openedAt = closed.openedAt ∧ captured.confirmedAt = closed.confirmedAt ∧
  OwnerStatementJournal.Runs (BoundWriterStatements.origin closed.writer.bound)
    captured.observations ⟨closed.writer.observed,[],false⟩

def checkOne (closed : RecordedClosed base) (captured : Captured p a) : Bool :=
  decide (captured.openedAt = closed.openedAt) && decide (captured.confirmedAt = closed.confirmedAt) &&
  (BoundWriterStatements.checkComplete closed.writer.bound captured.observations closed.writer.observed).isSome

theorem checkOne_exact : checkOne closed captured = true ↔ Matches closed captured := by
  simp [checkOne,Matches,BoundWriterStatements.complete_exact,and_assoc]

-- Pointwise and ordered; neither equal list lengths nor equal final values are
-- a substitute for a statement certificate at each actual historical close.
inductive Paired : List (RecordedClosed base) → List (Captured p a) → Prop where
  | nil : Paired [] []
  | cons : Matches closed captured → Paired rest tail → Paired (closed::rest) (captured::tail)

def check : List (RecordedClosed base) → List (Captured p a) → Bool
  | [],[] => true
  | closed::rest,captured::tail => checkOne closed captured && check rest tail
  | _,_ => false

theorem check_exact : check closed captured = true ↔ Paired closed captured := by
  induction closed generalizing captured with
  | nil =>
    cases captured with
    | nil => exact ⟨fun _ => .nil,fun _ => rfl⟩
    | cons => exact ⟨fun no => Bool.noConfusion no,fun path => by cases path⟩
  | cons first rest ih =>
    cases captured with
    | nil => exact ⟨fun no => Bool.noConfusion no,fun path => by cases path⟩
    | cons head tail =>
      constructor
      · intro accepted
        have facts : checkOne first head = true ∧ check rest tail = true := by
          simpa only [check,Bool.and_eq_true] using accepted
        exact .cons (checkOne_exact.mp facts.1) (ih.mp facts.2)
      · intro path
        cases path with
        | cons matching restPaired =>
          simp only [check,Bool.and_eq_true]
          exact ⟨checkOne_exact.mpr matching,ih.mpr restPaired⟩

structure Certified (closed : List (RecordedClosed base)) where
  captured : List (Captured p a)
  paired : Paired closed captured

def certify (closed : List (RecordedClosed base)) (captured : List (Captured p a)) : Option (Certified closed) :=
  if matched : check closed captured = true then some ⟨captured,check_exact.mp matched⟩ else none

theorem Paired.covers (paired : Paired closed captured) (member : record ∈ closed) :
    ∃ observed ∈ captured, Matches record observed := by
  induction paired with
  | nil => cases member
  | cons matching tail ih =>
    rcases List.mem_cons.mp member with same | later
    · subst record; exact ⟨_,List.mem_cons_self,matching⟩
    · obtain ⟨observed,present,bound⟩ := ih later
      exact ⟨observed,List.mem_cons_of_mem _ present,bound⟩

theorem Certified.covers (history : Certified closed) (member : record ∈ closed) :
    ∃ observed ∈ history.captured,
      observed.openedAt = record.openedAt ∧ observed.confirmedAt = record.confirmedAt ∧
      OwnerStatementJournal.Runs (BoundWriterStatements.origin record.writer.bound)
        observed.observations ⟨record.writer.observed,[],false⟩ := history.paired.covers member

theorem Paired.length (paired : Paired closed captured) : closed.length = captured.length := by
  induction paired <;> simp_all

theorem missing_history_refused (different : closed.length ≠ captured.length) : check closed captured = false := by
  cases checked : check closed captured with
  | false => rfl
  | true => exact False.elim (different (check_exact.mp checked).length)

theorem wrong_occurrence_refused (different : captured.openedAt ≠ closed.openedAt) :
    checkOne closed captured = false := by simp [checkOne,different]

theorem valid_history_admitted (paired : Paired closed captured) : check closed captured = true := check_exact.mpr paired

#print axioms checkOne_exact
#print axioms check_exact
#print axioms Paired.covers
#print axioms Certified.covers
#print axioms Paired.length
#print axioms missing_history_refused
#print axioms wrong_occurrence_refused
#print axioms valid_history_admitted
end StatementClosureHistory
