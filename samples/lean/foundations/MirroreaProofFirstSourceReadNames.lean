import MirroreaProofFirstSourceWriteHistory

namespace MirroreaProofFirst.SourceReadNames
open SourceAuthoring SourceTypes SourceExecution

def Bound (env : Environment) (names : List String) : Prop :=
  ∀ name ∈ names, ∃ type, Has env name type

theorem expression_bound (env : Environment) (expression : Expr) (typed : Expression env expression) :
    Bound env (exprReads expression) := by
  induction expression with
  | integer => intro name member; cases member
  | read name =>
      intro other member
      have same : other = name := List.mem_singleton.mp member
      subst other
      obtain ⟨mutable,has⟩ := typed
      exact ⟨.integer mutable,has⟩
  | add left right ihl ihr =>
      intro name member
      rcases List.mem_append.mp member with left | right
      · exact ihl typed.1 name left
      · exact ihr typed.2 name right
  | mul left right ihl ihr =>
      intro name member
      rcases List.mem_append.mp member with left | right
      · exact ihl typed.1 name left
      · exact ihr typed.2 name right

theorem support_bound (env : Environment) (support : Support.Formula String) (typed : References env support) :
    Bound env (supportReads support) := by
  induction support with
  | top | bottom => intro name member; cases member
  | ref name =>
      intro other member
      have same : other = name := List.mem_singleton.mp member
      subst other; exact ⟨.callable,typed⟩
  | both left right ihl ihr | either left right ihl ihr =>
      intro name member
      rcases List.mem_append.mp member with left | right
      · exact ihl typed.1 name left
      · exact ihr typed.2 name right

theorem optional_bound (env : Environment) (type : Ty) (name : Option String) (typed : Optional env type name) :
    Bound env name.toList := by
  cases name with
  | none => intro name member; cases member
  | some name =>
      intro other member
      have same : other = name := List.mem_singleton.mp member
      subst other; exact ⟨type,typed⟩

theorem statement_bound (env : Environment) (statement : Statement) (typed : Premise p env statement) :
    Bound env (reads statement) := by
  cases statement with
  | register name definition previous => exact optional_bound _ _ _ typed.2
  | instantiate name definition places parent support =>
      intro name member
      rcases List.mem_cons.mp member with same | member
      · subst name; exact ⟨.definition,typed.1⟩
      · rcases List.mem_append.mp member with parent | support
        · exact optional_bound _ _ _ typed.2.2.2.1 name parent
        · exact support_bound _ _ typed.2.2.2.2 name support
  | retire out name =>
      intro other member
      have same : other = name := List.mem_singleton.mp member
      subst other; exact ⟨.callable,typed⟩
  | reparent out name parent =>
      intro other member
      rcases List.mem_cons.mp member with same | rest
      · subst other; exact ⟨.callable,typed.1⟩
      · exact optional_bound _ _ _ typed.2 other rest
  | replace out name definition =>
      intro other member
      rcases List.mem_cons.mp member with same | rest
      · subst other; exact ⟨.callable,typed.1⟩
      · have same : other = definition := List.mem_singleton.mp rest
        subst other; exact ⟨.definition,typed.2⟩
  | leave | join => intro name member; cases member
  | localValue => exact expression_bound _ _ typed
  | assign => exact expression_bound _ _ typed.2
  | invoke out name expression =>
      intro other member
      rcases List.mem_cons.mp member with same | rest
      · subst other; exact ⟨.callable,typed.1⟩
      · exact expression_bound _ _ typed.2 other rest

theorem actual_value (values : List (String × Value)) (name : String) (type : Ty)
    (typed : Has (SourceTyping.environment values) name type) :
    ∃ value, SourceAuthoring.lookup values name = some value := by
  unfold Has at typed
  rw [SourceTyping.lookup_environment] at typed
  cases h : SourceAuthoring.lookup values name with
  | none => simp [h] at typed
  | some value => exact ⟨value,rfl⟩

def InputsPresent (writes : List Write) : Prop := ∀ w ∈ writes, ∀ r ∈ w.inputs, r.value.isSome = true

theorem advance_present (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (next : Execution p a) (present : InputsPresent s.writes)
    (accepted : advance s member place principal item = .accepted next) : InputsPresent next.writes := by
  obtain ⟨env,state,value,typed,_,_,rfl⟩ := (advance_exact _ _ _ _ _ _).mp accepted
  intro w hw r hr
  rcases List.mem_cons.mp hw with rfl | old
  · obtain ⟨name,member,rfl⟩ := List.mem_map.mp hr
    obtain ⟨type,has⟩ := statement_bound _ _ typed.1 name member
    obtain ⟨value,found⟩ := actual_value _ _ _ has
    simp [SourceExecution.read,found]
  · exact present w old r hr

theorem run_present (s : Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (present : InputsPresent s.writes) :
    InputsPresent (SourceExecution.run s member place principal program).final.writes := by
  induction program generalizing s with
  | nil => exact present
  | cons item rest ih =>
      unfold SourceExecution.run
      cases h : advance s member place principal item with
      | rejected reason => exact present
      | accepted next => exact ih next (advance_present _ _ _ _ _ _ present h)

-- Combine this with SourceWriteHistory: every actual source-local read of a
-- generated execution has exactly one earlier producer with the same name/value.
theorem unique_producer (s : Execution p a) (valid : SourceWriteHistory.Invariant s)
    (present : InputsPresent s.writes) (w : Write) (hw : w ∈ s.writes) (r : Read) (hr : r ∈ w.inputs) :
    ∃ producer ∈ s.writes, r.producer = some producer.ordinal ∧ producer.name = r.name ∧
      r.value = some producer.value ∧ producer.ordinal < w.ordinal ∧
      ∀ other ∈ s.writes, r.producer = some other.ordinal → other = producer := by
  have existsValue := present w hw r hr
  have reference := valid.referenced w hw r hr
  cases h : r.producer with
  | none =>
      have missing : r.value = none := by simpa [h] using reference
      simp [missing] at existsValue
  | some ordinal =>
      obtain ⟨producer,hp,eq,name,value,lt⟩ := (show ∃ producer ∈ s.writes,
          producer.ordinal = ordinal ∧ producer.name = r.name ∧ r.value = some producer.value ∧ producer.ordinal < w.ordinal
          from by simpa [h] using reference)
      refine ⟨producer,hp,by simp [h,eq],name,value,lt,?_⟩
      intro other ho he
      have ordinalSame : other.ordinal = producer.ordinal := by simp [h] at he; omega
      exact SourceWriteHistory.ordinal_unique _ valid.numbered other producer ho hp ordinalSame

#print axioms statement_bound
#print axioms run_present
#print axioms unique_producer
end MirroreaProofFirst.SourceReadNames
