import OwnerStatementJournal
import SharedHostPrefix
open MirroreaProofFirst SharedHostCaptureReplay OwnerCommitJournal
namespace BoundWriterStatements
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- The bound is obtained from the active writer of the SAME certified cohort
-- cursor. No independent owner/native transition is replayed by this gate.
def origin (bound : BoundWriter base) : Journal p a :=
  ⟨bound.writer.before,stores bound.writer.before bound.writer.command bound.writer.reply,false⟩

structure Prefix (bound : BoundWriter base) (observations : List (OwnerStatementJournal.Observation p a)) where
  journal : Journal p a
  path : OwnerStatementJournal.Runs (origin bound) observations journal

def checkPrefix (bound : BoundWriter base) (observations : List (OwnerStatementJournal.Observation p a)) :
    Option (Prefix bound observations) :=
  match checked : OwnerStatementJournal.run (origin bound) observations with
  | none => none
  | some journal => some ⟨journal,OwnerStatementJournal.run_exact.mp checked⟩

variable {bound : BoundWriter base}

theorem prefix_exact : (checkPrefix bound observations).isSome = true ↔
    ∃ journal, OwnerStatementJournal.Runs (origin bound) observations journal := by
  unfold checkPrefix
  split
  · rename_i failed
    simp only [Option.isSome_none,Bool.false_eq_true,false_iff,not_exists]
    intro journal path
    have ran := OwnerStatementJournal.run_exact.mpr path
    rw [failed] at ran
    cases ran
  · rename_i journal checked
    exact ⟨fun _ => ⟨journal,OwnerStatementJournal.run_exact.mp checked⟩,fun _ => rfl⟩

theorem statement_projection (path : OwnerStatementJournal.Runs before observations after) :
    OwnerCommitJournal.Runs before after := by
  induction path with
  | nil => exact .nil
  | cons first rest ih => exact (OwnerCommitJournal.Runs.step .nil (OwnerStatementJournal.step_projection first)).trans ih

structure Complete (bound : BoundWriter base) (observations : List (OwnerStatementJournal.Observation p a))
    (observed : Memory p a) where
  traced : Prefix bound observations
  done : OwnerStatementJournal.finish traced.journal observed = true

def checkComplete (bound : BoundWriter base) (observations : List (OwnerStatementJournal.Observation p a))
    (observed : Memory p a) : Option (Complete bound observations observed) :=
  match checkPrefix bound observations with
  | none => none
  | some traced =>
    if done : OwnerStatementJournal.finish traced.journal observed = true then
      some ⟨traced,done⟩
    else none

theorem Complete.actualPath (complete : Complete bound observations observed) :
    OwnerStatementJournal.Runs (origin bound) observations ⟨observed,[],false⟩ := by
  obtain ⟨stopped,pending,memory⟩ := OwnerStatementJournal.finish_exact.mp complete.done
  have equal : complete.traced.journal = ⟨observed,[],false⟩ := by
    cases shape : complete.traced.journal
    simp_all
  exact equal ▸ complete.traced.path

theorem Complete.count (complete : Complete bound observations observed) :
    observations.length = (stores bound.writer.before bound.writer.command bound.writer.reply).length :=
  OwnerStatementJournal.completion_count complete.traced.path complete.done

-- General native correspondence is inherited from the exact current bound's
-- checked receipt and proved using the actual captured statement path.
theorem Complete.native (complete : Complete bound observations observed) :
    observed = ⟨project ((SharedWireLifetime.native bound.later.current).owners bound.writer.owner),none,false⟩ := by
  have targetEqual := runs_target (statement_projection complete.actualPath)
  have targetNative := bound.writer.nativeTarget
  have equal : observed = ⟨project bound.writer.nativeAfter,none,false⟩ := by
    simpa [origin,target] using targetEqual.trans targetNative
  simpa only [bound.afterBinding] using equal

def Complete.closed (complete : Complete bound observations observed) : ClosedWriter base :=
  ⟨bound,observed,statement_projection complete.actualPath,
    congrArg (fun m : Memory p a => m.data) complete.native,
    congrArg (fun m : Memory p a => m.lease) complete.native,
    congrArg (fun m : Memory p a => m.entered) complete.native⟩

-- A completion is tied to the original native receipt/whole wire states,
-- rather than to a count, a digest, or a separately replayed expected state.
theorem Complete.sameBound (complete : Complete bound observations observed) :
    complete.closed.bound = bound := rfl

theorem complete_exact : (checkComplete bound observations observed).isSome = true ↔
    OwnerStatementJournal.Runs (origin bound) observations ⟨observed,[],false⟩ := by
  constructor
  · intro checked
    unfold checkComplete at checked
    split at checked
    · cases checked
    · rename_i traced got
      split at checked
      · rename_i done
        exact (Complete.actualPath ⟨traced,done⟩)
      · cases checked
  · intro path
    have ran := OwnerStatementJournal.run_exact.mpr path
    have available := prefix_exact.mpr ⟨_,path⟩
    cases got : checkPrefix bound observations with
    | none => simp [got] at available
    | some traced =>
      have equal : traced.journal = ⟨observed,[],false⟩ :=
        Option.some.inj ((OwnerStatementJournal.run_exact.mpr traced.path).symm.trans ran)
      have done : OwnerStatementJournal.finish traced.journal observed = true := by
        rw [equal]
        simp [OwnerStatementJournal.finish,OwnerCommitJournal.memoryEq_exact]
      simp [checkComplete,got,done]

-- Non-vacuous relative admission: any actual exact statement path finishing
-- the receipt recipe is accepted. No future invariant is a checker guard.
theorem admitted (path : OwnerStatementJournal.Runs (origin bound) observations ⟨observed,[],false⟩) :
    (checkComplete bound observations observed).isSome = true := complete_exact.mpr path

theorem omitted_refused
    (short : observations.length < (stores bound.writer.before bound.writer.command bound.writer.reply).length) :
    (checkComplete bound observations observed).isSome = false := by
  cases checked : (checkComplete bound observations observed).isSome with
  | false => rfl
  | true =>
    have count := (complete_exact.mp checked).count
    simp only [List.length_nil,Nat.add_zero,origin] at count
    omega

-- The ordinary close produced by the same joined step retains this exact
-- bound. This theorem connects the added gate to the existing history record.
theorem confirmed_same
    (step : JoinedStep before (.confirmed owner memory) after)
    (active : before.active = some opened)
    (complete : Complete opened.bound observations memory) :
    ∃ closed ∈ after.closed, closed.openedAt = opened.position ∧
      closed.writer.bound = opened.bound ∧ closed.writer.observed = memory ∧
      observations.length = (stores closed.writer.bound.writer.before
        closed.writer.bound.writer.command closed.writer.bound.writer.reply).length := by
  cases step with
  | confirmed live current same finished =>
    have equal := Option.some.inj (current.symm.trans active)
    subst opened
    refine ⟨_,List.mem_cons_self, rfl, finish_bound finished, finish_observed finished,?_⟩
    rw [finish_bound finished]
    exact complete.count

#print axioms prefix_exact
#print axioms statement_projection
#print axioms Complete.actualPath
#print axioms Complete.count
#print axioms Complete.native
#print axioms Complete.sameBound
#print axioms complete_exact
#print axioms admitted
#print axioms omitted_refused
#print axioms confirmed_same
end BoundWriterStatements
