import MirroreaProofFirstSourceTyping

namespace MirroreaProofFirst.SourceExecution
open SourceAuthoring SourceTypes

def outputName : Statement → String
  | .register name .. | .instantiate name .. | .retire name .. | .reparent name ..
  | .replace name .. | .leave name .. | .join name .. | .localValue name ..
  | .assign name .. | .invoke name .. => name

def exprReads : Expr → List String
  | .integer _ => []
  | .read name => [name]
  | .add a b | .mul a b => exprReads a ++ exprReads b

def supportReads : Support.Formula String → List String
  | .top | .bottom => []
  | .ref name => [name]
  | .both a b | .either a b => supportReads a ++ supportReads b

def reads : Statement → List String
  | .register _ _ predecessor => predecessor.toList
  | .instantiate _ definition _ parent support => definition :: parent.toList ++ supportReads support
  | .retire _ name => [name]
  | .reparent _ name parent => name :: parent.toList
  | .replace _ name definition => [name,definition]
  | .leave .. | .join .. => []
  | .localValue _ _ expression => exprReads expression
  | .assign _ expression => exprReads expression
  | .invoke _ target expression => target :: exprReads expression

structure Read where
  name : String
  value : Option Value
  producer : Option Nat
  deriving DecidableEq, Repr

-- This is a private semantic write record, emitted by successful execution,
-- before the next statement. It is not a public observer or a synthetic result.
-- Producer none explicitly means an input lacking an earlier source write.
structure Write where
  ordinal : Nat
  startByte : Nat
  name : String
  value : Value
  inputs : List Read
  machineBefore : Nat
  machineAfter : Nat
  deriving DecidableEq, Repr

structure Execution (p a : Nat) where
  source : State p a
  writes : List Write

def read (s : Execution p a) (name : String) : Read :=
  ⟨name,SourceAuthoring.lookup s.source.values name,
   (s.writes.find? (fun item => item.name == name)).map Write.ordinal⟩

def event (s : Execution p a) (item : Located) (next : State p a) (value : Value) : Write :=
  ⟨s.writes.length,item.startByte,outputName item.statement,value,(reads item.statement).map (read s),
   s.source.machine.events.length,next.machine.events.length⟩

def appendWrite (s : Execution p a) (item : Located) (next : State p a) (value : Value) : Execution p a :=
  ⟨next,event s item next value :: s.writes⟩

inductive Failure where
  | staticType
  | dynamicRejected
  | missingOutput
  deriving DecidableEq, Repr

inductive StepResult (p a : Nat) where
  | accepted (next : Execution p a)
  | rejected (reason : Failure)

def advance (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located) : StepResult p a :=
  match checkStatement p (SourceTyping.environment s.source.values) item.statement with
  | none => .rejected .staticType
  | some _ => match SourceAuthoring.execute s.source member place principal item with
    | none => .rejected .dynamicRejected
    | some next => match SourceAuthoring.lookup next.values (outputName item.statement) with
      | none => .rejected .missingOutput
      | some value => .accepted (appendWrite s item next value)

theorem output_exists (s : State p a) (item : Located) (next : State p a) (env : Environment)
    (typed : StatementTyped p (SourceTyping.environment s.values) item.statement env)
    (aligned : SourceTyping.environment next.values = env) :
    ∃ value, SourceAuthoring.lookup next.values (outputName item.statement) = some value := by
  have existsType : ∃ type, SourceTypes.lookup env (outputName item.statement) = some type := by
    cases h : item.statement <;> simp only [h,StatementTyped,Premise,Produces,output,outputName] at typed ⊢
    case assign =>
      rcases typed with ⟨⟨found,_⟩,rfl⟩
      exact ⟨_,found⟩
    all_goals rcases typed.2 with ⟨_,rfl⟩; simp [SourceTypes.lookup]
  obtain ⟨type,found⟩ := existsType
  rw [← aligned,SourceTyping.lookup_environment] at found
  cases h : SourceAuthoring.lookup next.values (outputName item.statement) with
  | none => simp [h] at found
  | some value => exact ⟨value,rfl⟩

-- missingOutput is a defensive boundary for a malformed adapter/runtime. It
-- cannot be reached by the actual checked step, including assignment.
theorem no_missing_output (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located) :
    advance s member place principal item ≠ .rejected .missingOutput := by
  unfold advance
  cases hc : checkStatement p (SourceTyping.environment s.source.values) item.statement with
  | none => simp
  | some env =>
      cases he : SourceAuthoring.execute s.source member place principal item with
      | none => simp
      | some next =>
          have typed := (statement_exact _ _ _ _).mp hc
          obtain ⟨value,hv⟩ := output_exists s.source item next env typed
            (SourceTyping.execute_environment _ _ _ _ _ _ _ typed he)
          simp [hv]

theorem advance_exact (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located)
    (result : Execution p a) : advance s member place principal item = .accepted result ↔
      ∃ env next value, StatementTyped p (SourceTyping.environment s.source.values) item.statement env ∧
        SourceAuthoring.execute s.source member place principal item = some next ∧
        SourceAuthoring.lookup next.values (outputName item.statement) = some value ∧
        result = appendWrite s item next value := by
  constructor
  · intro accepted
    unfold advance at accepted
    cases hc : checkStatement p (SourceTyping.environment s.source.values) item.statement with
    | none => simp [hc] at accepted
    | some env =>
        simp only [hc] at accepted
        cases he : SourceAuthoring.execute s.source member place principal item with
        | none => simp [he] at accepted
        | some next =>
            simp only [he] at accepted
            cases hv : SourceAuthoring.lookup next.values (outputName item.statement) with
            | none => simp [hv] at accepted
            | some value =>
                simp only [hv,StepResult.accepted.injEq] at accepted
                exact ⟨env,next,value,(statement_exact _ _ _ _).mp hc,rfl,hv,accepted.symm⟩
  · rintro ⟨env,next,value,typed,he,hv,rfl⟩
    simp [advance,(statement_exact _ _ _ _).mpr typed,he,hv]

theorem advance_preserves (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located)
    (next : Execution p a) (valid : SourcePreservation.Invariant s.source)
    (accepted : advance s member place principal item = .accepted next) : SourcePreservation.Invariant next.source := by
  obtain ⟨_,state,_,_,he,_,rfl⟩ := (advance_exact _ _ _ _ _ _).mp accepted
  exact SourcePreservation.execute_preserves _ _ _ _ _ _ valid he

theorem advance_write (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located)
    (next : Execution p a) (accepted : advance s member place principal item = .accepted next) :
    ∃ value, next.writes = event s item next.source value :: s.writes ∧
      SourceAuthoring.lookup next.source.values (outputName item.statement) = some value := by
  obtain ⟨_,state,value,_,he,hv,rfl⟩ := (advance_exact _ _ _ _ _ _).mp accepted
  exact ⟨value,rfl,hv⟩

-- The last successful state is returned on rejection. A program is a sequence
-- of checked operations, not an implicitly atomic multi-operation transaction.
structure Outcome (p a : Nat) where
  final : Execution p a
  rejected : Option (Located × Failure)

def run (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat) : List Located → Outcome p a
  | [] => ⟨s,none⟩
  | item :: rest => match advance s member place principal item with
    | .rejected reason => ⟨s,some (item,reason)⟩
    | .accepted next => run next member place principal rest

-- Independent operational derivation: an actual typed source operation must
-- occur for each semantic write. An observer cannot append one on its own.
inductive Steps (member : Fin a) (place : Fin p) (principal : Nat) :
    Execution p a → List Located → Execution p a → Prop where
  | nil : Steps member place principal s [] s
  | cons : StatementTyped p (SourceTyping.environment s.source.values) item.statement env →
      SourceAuthoring.execute s.source member place principal item = some next →
      SourceAuthoring.lookup next.values (outputName item.statement) = some value →
      Steps member place principal (appendWrite s item next value) rest final →
      Steps member place principal s (item :: rest) final

theorem run_completed_exact (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (final : Execution p a) :
    run s member place principal program = ⟨final,none⟩ ↔ Steps member place principal s program final := by
  induction program generalizing s with
  | nil => constructor <;> intro h <;> cases h <;> constructor
  | cons item rest ih =>
      constructor
      · intro completed
        unfold run at completed
        cases h : advance s member place principal item with
        | rejected reason => simp [h] at completed
        | accepted next =>
            obtain ⟨env,state,value,typed,he,hv,rfl⟩ := (advance_exact _ _ _ _ _ _).mp h
            exact .cons typed he hv ((ih _).mp (by simpa [h] using completed))
      · intro steps
        cases steps with
        | cons typed he hv tail =>
            have accepted := (advance_exact _ _ _ _ _ _).mpr ⟨_,_,_,typed,he,hv,rfl⟩
            simp [run,accepted,(ih _).mpr tail]

theorem run_preserves (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (valid : SourcePreservation.Invariant s.source) :
    SourcePreservation.Invariant (run s member place principal program).final.source := by
  induction program generalizing s with
  | nil => exact valid
  | cons item rest ih =>
      unfold run
      cases h : advance s member place principal item with
      | rejected reason => exact valid
      | accepted next => exact ih next (advance_preserves _ _ _ _ _ _ valid h)

theorem run_prefix (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) :
    ∃ completed rest, program = completed ++ rest ∧
      Steps member place principal s completed (run s member place principal program).final ∧
      ((run s member place principal program).rejected = none ∧ rest = [] ∨
       ∃ item tail reason, rest = item :: tail ∧
         (run s member place principal program).rejected = some (item,reason) ∧
         advance (run s member place principal program).final member place principal item = .rejected reason) := by
  induction program generalizing s with
  | nil => exact ⟨[],[],rfl,.nil,Or.inl ⟨rfl,rfl⟩⟩
  | cons item tail ih =>
      cases h : advance s member place principal item with
      | rejected reason =>
          refine ⟨[],item::tail,rfl,?_,Or.inr ⟨item,tail,reason,rfl,?_,?_⟩⟩
          · simpa [run,h] using (Steps.nil (s := s) (member := member) (place := place) (principal := principal))
          · simp [run,h]
          · simpa [run,h] using h
      | accepted next =>
          obtain ⟨completed,rest,eq,steps,last⟩ := ih next
          obtain ⟨env,state,value,typed,he,hv,rfl⟩ := (advance_exact _ _ _ _ _ _).mp h
          refine ⟨item::completed,rest,by simp [eq],?_,?_⟩
          · simpa [run,h] using (Steps.cons typed he hv steps)
          · simpa [run,h] using last

def Ordered (s : Execution p a) : Prop :=
  ∀ w ∈ s.writes, w.ordinal < s.writes.length ∧
    ∀ r ∈ w.inputs, ∀ producer, r.producer = some producer → producer < w.ordinal

theorem read_backward (s : Execution p a) (name : String) (producer : Nat)
    (ordered : Ordered s) (found : (read s name).producer = some producer) : producer < s.writes.length := by
  unfold read at found
  cases h : s.writes.find? (fun item => item.name == name) with
  | none => simp [h] at found
  | some w =>
      simp only [h,Option.map_some,Option.some.injEq] at found
      exact found ▸ (ordered w (List.mem_of_find?_eq_some h)).1

theorem append_ordered (s : Execution p a) (item : Located) (next : State p a) (value : Value)
    (ordered : Ordered s) : Ordered (appendWrite s item next value) := by
  intro w hw
  rcases List.mem_cons.mp hw with rfl | old
  · refine ⟨by simp [appendWrite,event],?_⟩
    intro r hr producer found
    obtain ⟨name,_,rfl⟩ := List.mem_map.mp hr
    exact read_backward s name producer ordered found
  · have parts := ordered w old
    exact ⟨Nat.lt_trans parts.1 (by simp [appendWrite]),parts.2⟩

theorem run_ordered (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (ordered : Ordered s) : Ordered (run s member place principal program).final := by
  induction program generalizing s with
  | nil => exact ordered
  | cons item rest ih =>
      unfold run
      cases h : advance s member place principal item with
      | rejected reason => exact ordered
      | accepted next =>
          obtain ⟨_,state,value,_,_,_,rfl⟩ := (advance_exact _ _ _ _ _ _).mp h
          exact ih _ (append_ordered _ _ _ _ ordered)

theorem writes_preserved (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) : s.writes <:+ (run s member place principal program).final.writes := by
  induction program generalizing s with
  | nil => exact ⟨[],rfl⟩
  | cons item rest ih =>
      unfold run
      cases h : advance s member place principal item with
      | rejected reason => exact ⟨[],rfl⟩
      | accepted next =>
          obtain ⟨_,state,value,_,_,_,rfl⟩ := (advance_exact _ _ _ _ _ _).mp h
          exact List.IsSuffix.trans ⟨[event s item state value],rfl⟩ (ih (appendWrite s item state value))

#print axioms no_missing_output
#print axioms advance_exact
#print axioms advance_write
#print axioms run_completed_exact
#print axioms run_preserves
#print axioms run_prefix
#print axioms run_ordered
#print axioms writes_preserved
end MirroreaProofFirst.SourceExecution
