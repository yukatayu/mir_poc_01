import MirroreaProofFirstReferenceSourceProvenance

namespace MirroreaProofFirst.ReferenceSourceReads
open ReferenceSourceData ReferenceSource ReferenceSourceElaboration

def Known (values : Values) (names : List String) : Prop :=
  ∀ name ∈ names, ∃ value, lookup values name = some value

theorem known_append (values : Values) (left right : List String) :
    Known values (left ++ right) ↔ Known values left ∧ Known values right := by
  simp [Known,List.mem_append,or_imp,forall_and]

theorem known_cons (values : Values) (name : String) (rest : List String) :
    Known values (name :: rest) ↔ (∃ value, lookup values name = some value) ∧ Known values rest := by
  simp [Known]

theorem evaluation_reads (values : Values) (expression : SourceAuthoring.Expr) (result : Int)
    (evaluated : Evaluates values expression result) : Known values (expressionReads expression) := by
  induction expression generalizing result with
  | integer _ => simp [expressionReads,Known]
  | read name =>
      obtain ⟨⟨mutable,found⟩,_⟩ := evaluated
      simpa [expressionReads,Known] using (show ∃ value, lookup values name = some value from ⟨_,found⟩)
  | add left right ihl ihr =>
      obtain ⟨_,_,left,right,_,_⟩ := evaluated
      exact (known_append _ _ _).mpr ⟨ihl _ left,ihr _ right⟩
  | mul left right ihl ihr =>
      obtain ⟨_,_,left,right,_,_⟩ := evaluated
      exact (known_append _ _ _).mpr ⟨ihl _ left,ihr _ right⟩

theorem support_reads (values : Values) (formula : Support.Formula String) (result : Support.Formula Nat)
    (elaborated : DependencyNames values formula result) : Known values (supportReads formula) := by
  induction formula generalizing result with
  | top => simp [supportReads,Known]
  | bottom => simp [supportReads,Known]
  | ref name =>
      obtain ⟨_,found,_⟩ := elaborated
      simpa [supportReads,Known] using (show ∃ value, lookup values name = some value from ⟨_,found⟩)
  | both left right ihl ihr =>
      obtain ⟨_,_,left,right,_⟩ := elaborated
      exact (known_append _ _ _).mpr ⟨ihl _ left,ihr _ right⟩
  | either left right ihl ihr =>
      obtain ⟨_,_,left,right,_⟩ := elaborated
      exact (known_append _ _ _).mpr ⟨ihl _ left,ihr _ right⟩

theorem optional_reads (values : Values) (wrap : Nat → Value) (name : Option String) (key : Option Nat)
    (elaborated : OptionalName (fun name key => lookup values name = some (wrap key)) name key) :
    Known values name.toList := by
  cases name with
  | none => simp [Known]
  | some name =>
      obtain ⟨key,found,_⟩ := elaborated
      simpa [Known] using (show ∃ value, lookup values name = some value from ⟨_,found⟩)

theorem plain_reads (values : Values) (statement : SourceAuthoring.Statement) (plan : Plan)
    (elaborated : PlainFor values statement plan) : Known values (plainReads statement) := by
  cases statement with
  | register name definition previous =>
      obtain ⟨_,previous,_⟩ := elaborated
      exact optional_reads _ _ _ _ previous
  | instantiate name definition places parent support =>
      obtain ⟨_,_,_,defined,parent,support,_⟩ := elaborated
      exact (known_cons _ _ _).mpr ⟨⟨_,defined⟩,(known_append _ _ _).mpr
        ⟨optional_reads _ _ _ _ parent,support_reads _ _ _ support⟩⟩
  | retire out target =>
      obtain ⟨_,found,_⟩ := elaborated
      simpa [plainReads,Known] using (show ∃ value, lookup values target = some value from ⟨_,found⟩)
  | reparent out target parent =>
      obtain ⟨_,_,found,parent,_⟩ := elaborated
      exact (known_cons _ _ _).mpr ⟨⟨_,found⟩,optional_reads _ _ _ _ parent⟩
  | replace out target definition =>
      obtain ⟨_,_,found,defined,_⟩ := elaborated
      simp only [plainReads,known_cons]
      exact ⟨⟨_,found⟩,⟨_,defined⟩,by simp [Known]⟩
  | leave _ _ | join _ _ => simp [plainReads,Known]
  | localValue name mutable expression =>
      obtain ⟨_,evaluated,_⟩ := elaborated
      exact evaluation_reads _ _ _ evaluated
  | assign name expression =>
      obtain ⟨_,evaluated,_⟩ := elaborated
      exact evaluation_reads _ _ _ evaluated
  | invoke name target expression =>
      obtain ⟨argument,resolved,evaluated,named,_⟩ := elaborated
      apply (known_cons _ _ _).mpr
      refine ⟨?_,evaluation_reads _ _ _ evaluated⟩
      cases resolved <;> exact ⟨_,named⟩

theorem options_reads (s : InstanceState.State d p n) (values : Values)
    (declarations : List OptionSyntax) (options : List FallbackStatic.OptionDecl)
    (elaborated : OptionsElaborate s values declarations options) :
    Known values (declarations.map OptionSyntax.target) := by
  induction elaborated with
  | nil => simp [Known]
  | cons first rest ih =>
      obtain ⟨key,found,_⟩ := first
      exact (known_cons _ _ _).mpr ⟨⟨_,(instance_exact _ _ _).mp found⟩,ih⟩

theorem elaborated_reads (s : InstanceState.State d p n) (values : Values) (statement : Statement) (plan : Plan)
    (elaborated : PlanFor s values statement plan) : Known values (reads statement) := by
  cases statement with
  | plain statement => exact plain_reads _ _ _ elaborated
  | acquire name decl =>
      obtain ⟨chain,⟨⟨reader,found,_⟩,options,_⟩,_,_⟩ := elaborated
      exact (known_cons _ _ _).mpr ⟨⟨_,(instance_exact _ _ _).mp found⟩,options_reads _ _ _ _ options⟩
  | alias name target | reacquire name target | release name target =>
      obtain ⟨_,found,_⟩ := elaborated
      simpa [reads,Known] using (show ∃ value, lookup values target = some value from ⟨_,found⟩)

-- Non-stuttering source admission necessarily elaborated the whole statement,
-- before normalization or any other mutation. This includes a committed prefix
-- followed by failure; it does not bless an arbitrary helper Plan.
theorem advance_reads (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located)
    (changed : (advance s member place principal item).state ≠ s) : Known s.values (reads item.statement) := by
  obtain ⟨_,_,plan,_,meaning,_⟩ := ReferenceSourceTrace.advance_effect_basis _ _ _ _ _ changed
  exact elaborated_reads _ _ _ _ meaning

-- ReadFrom permits absent probes. Combined with actual successful elaboration,
-- however, EVERY listed dependency is an existing value from a real prior write.
theorem actual_read_producer (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located)
    (valid : ReferenceSourceProvenance.Invariant s)
    (changed : (advance s member place principal item).state ≠ s) (r : Read) (readAt : r ∈ inputs s item) :
    ∃ producer ∈ s.writes, r.producer = some producer.ordinal ∧ producer.name = r.name ∧
      r.value = some producer.value ∧ producer.ordinal < s.writes.length := by
  obtain ⟨name,named,rfl⟩ := List.mem_map.mp readAt
  obtain ⟨value,existsValue⟩ := advance_reads _ _ _ _ _ changed name named
  have provenance := ReferenceSourceProvenance.read_from _ valid name
  cases index : (ReferenceSource.read s name).producer with
  | none =>
      simp only [ReferenceSourceProvenance.ReadFrom,index] at provenance
      change lookup s.values name = none at provenance
      rw [existsValue] at provenance
      cases provenance
  | some ordinal =>
      simp only [ReferenceSourceProvenance.ReadFrom,index] at provenance
      obtain ⟨producer,present,number,key,value,bound⟩ := provenance
      exact ⟨producer,present,congrArg some number.symm,key,value,bound⟩

#print axioms evaluation_reads
#print axioms plain_reads
#print axioms elaborated_reads
#print axioms advance_reads
#print axioms actual_read_producer
end MirroreaProofFirst.ReferenceSourceReads
