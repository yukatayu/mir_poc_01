import MixedOwnerSourceDeclaration
import MixedSourceCursor
namespace MirroreaProofFirst.MixedOwnerProgram
open ReferenceSourceData

-- Finite source profile: existing statements and ordinary owner-field
-- assignments. The programmer supplies no definition keys, generated names,
-- sends, receipt IDs or event constructors for an assignment. Field/label
-- context is retained explicitly; its authentic derivation is a later gate.
inductive Item where
 | ordinary (item : Located)
 | assignment (item : MixedNamedOwnerSource.Assignment)
 deriving DecidableEq
structure Program (p : Nat) where
 place : Fin p
 fields : MixedNamedOwnerSource.Fields
 labels : List (String × Nat)
 control : Nat
 items : List Item

def names : Item → List String
 | .ordinary item =>
   let target := match item.statement with | .plain (.assign name _) => [name] | _ => []
   ((output item.statement).map Prod.fst).toList ++ ReferenceSource.reads item.statement ++ target
 | .assignment item => item.target :: MixedNamedOwnerSource.parameters item.rhs

def width : List String → Nat
 | [] => 1
 | name::rest => max name.length (width rest) + 1

def freshName (reserved : List String) : String := String.ofList (List.replicate (width reserved) '_')
theorem width_positive (ns : List String) : 0 < width ns := by cases ns <;> simp [width]
theorem width_greater (member : name ∈ reserved) : name.length < width reserved := by
 induction reserved with
 | nil => cases member
 | cons head tail ih =>
   rcases List.mem_cons.mp member with rfl | member
   · have := width_positive tail; simp only [width]; omega
   · have := ih member; simp only [width]; omega

theorem fresh_not_member : freshName reserved ∉ reserved := by
 intro member
 have large := width_greater member
 simpa [freshName] using large

-- Each generated pair increases the name-width bound by exactly two. The
-- allocator must not recursively concatenate/sum prior generated strings and
-- produce exponential names during a long but finite source block.
theorem generated_width :
 width (freshName (freshName used::used)::freshName used::used) = width used+2 := by
 simp [width,freshName,Nat.add_assoc]

def integerAt (env : Environment) (name : String) : Bool :=
 match lookupType env name with | some (.plain (.integer _)) => true | _ => false
def CapturesTyped (env : Environment) (names : List String) : Prop :=
 ∀ name ∈ names, ∃ mutable, Has env name (.plain (.integer mutable))
theorem integer_exact : integerAt env name = true ↔ ∃ mutable, Has env name (.plain (.integer mutable)) := by
 cases found : lookupType env name with
 | none => simp [integerAt,Has,found]
 | some type =>
   cases type with
   | reference => simp [integerAt,Has,found]
   | plain type => cases type <;> simp [integerAt,Has,found]
theorem captures_exact (ns : List String) : ns.all (integerAt env) = true ↔ CapturesTyped env ns := by
 simp [List.all_eq_true,CapturesTyped,integer_exact]

-- Only compiler output has these entries. `install` consumes the retained
-- Assignment, never separately supplied code. `construct` is an ordinary
-- existing instantiate statement. `write` consumes actual current values.
inductive Entry where
 | ordinary (item : Located)
 | install (name : String) (control : Nat) (source : MixedNamedOwnerSource.Assignment)
 | write (name : String) (ordinal control : Nat) (source : MixedNamedOwnerSource.Assignment)
 deriving DecidableEq

def construction (source : MixedNamedOwnerSource.Assignment) (name definition : String) (place : Nat) : Entry :=
 .ordinary ⟨source.site,.plain (.instantiate name definition [place] none .top)⟩
def ownerEnvironment (env : Environment) (name definition : String) : Environment :=
 (name,.plain .callable)::(definition,.plain .definition)::env

-- Completion of arithmetic/strict lookups is data dependent even when the
-- subsequent assignment has a constant RHS. Target labels do not substitute
-- for this rank: writing a constant to a private target need not raise it.
def completion (code : MixedOwnerSourceCode.Code) : Nat :=
 ProducerFlow.rank
  (OwnerAssignment.classes (MixedOperationDefinitions.labelAt code.operation.contract)
    (MixedOperationDefinitions.capturedAt code.operation.contract))
  (OwnerCheckedArithmetic.translate code.operation.body.tree)

def lower (p : Nat) (ctx : MixedNamedOwnerSource.Fields) (labels : List (String × Nat)) (control : Nat)
 (ordinal : Nat) (reserved : List String) (env : Environment) : List Item → Option (List Entry × Environment)
 | [] => some ([],env)
 | .ordinary item::rest => do
   let next ← checkStatement p env item.statement
   let (tail,final) ← lower p ctx labels control (ordinal+1) reserved next rest
   return (.ordinary item::tail,final)
 | .assignment source::rest => do
   let code ← MixedOwnerSourceCode.compile ctx labels control source
   let place := MixedOperationDefinitions.ownerAt code.operation.contract code.operation.body.target
   if !(MixedNamedOwnerSource.parameters source.rhs).all (integerAt env) || !(place < p) then none else do
   let definition := freshName reserved
   let name := freshName (definition::reserved)
   let (tail,final) ← lower p ctx labels (max control (completion code)) (ordinal+1) (name::definition::reserved)
     (ownerEnvironment env name definition) rest
   return (.install definition control source::construction source name definition place::.write name ordinal control source::tail,final)

inductive Lowers (p : Nat) (ctx : MixedNamedOwnerSource.Fields) (labels : List (String × Nat)) :
 Nat → Nat → List String → Environment → List Item → List Entry → Environment → Prop where
 | nil {control : Nat} : Lowers p ctx labels control ordinal reserved env [] [] env
 | ordinary {control : Nat} : StatementTyped p env item.statement next →
   Lowers p ctx labels control (ordinal+1) reserved next rest tail final →
   Lowers p ctx labels control ordinal reserved env (.ordinary item::rest) (.ordinary item::tail) final
 | assignment {control : Nat} : MixedOwnerSourceCode.Compiles ctx labels control source code →
   CapturesTyped env (MixedNamedOwnerSource.parameters source.rhs) →
   MixedOperationDefinitions.ownerAt code.operation.contract code.operation.body.target < p →
   Lowers p ctx labels (max control (completion code)) (ordinal+1)
     (freshName (freshName reserved::reserved)::freshName reserved::reserved)
     (ownerEnvironment env (freshName (freshName reserved::reserved)) (freshName reserved)) rest tail final →
   Lowers p ctx labels control ordinal reserved env (.assignment source::rest)
     (.install (freshName reserved) control source::
       construction source (freshName (freshName reserved::reserved)) (freshName reserved)
         (MixedOperationDefinitions.ownerAt code.operation.contract code.operation.body.target)::
       .write (freshName (freshName reserved::reserved)) ordinal control source::tail) final

theorem lower_sound {control : Nat} {used : List String}
 (accepted : lower p ctx labels control ordinal used env items = some (entries,final)) :
 Lowers p ctx labels control ordinal used env items entries final := by
 induction items generalizing control ordinal used env entries final with
 | nil => simp only [lower,Option.some.injEq,Prod.mk.injEq] at accepted; obtain ⟨rfl,rfl⟩ := accepted; exact .nil
 | cons item rest ih =>
   cases item with
   | ordinary item =>
     cases checked : checkStatement p env item.statement with
     | none => simp [lower,checked] at accepted
     | some next =>
       cases tailRun : lower p ctx labels control (ordinal+1) used next rest with
       | none => simp [lower,checked,tailRun] at accepted
       | some pair =>
         obtain ⟨tail,last⟩ := pair
         simp only [lower,checked,tailRun,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at accepted
         obtain ⟨rfl,rfl⟩ := accepted
         exact .ordinary ((statement_exact _ _ _ _).mp checked) (ih tailRun)
   | assignment source =>
     cases compiled : MixedOwnerSourceCode.compile ctx labels control source with
     | none => simp [lower,compiled] at accepted
     | some code =>
       simp only [lower,compiled,Option.bind_eq_bind,Option.bind_some] at accepted
       split at accepted
       · cases accepted
       · rename_i guardOpen
         have premises : (MixedNamedOwnerSource.parameters source.rhs).all (integerAt env) = true ∧
           MixedOperationDefinitions.ownerAt code.operation.contract code.operation.body.target < p := by simpa using guardOpen
         cases tailRun : lower p ctx labels (max control (completion code)) (ordinal+1)
             (freshName (freshName used::used)::freshName used::used)
             (ownerEnvironment env (freshName (freshName used::used)) (freshName used)) rest with
         | none => simp [tailRun] at accepted
         | some pair =>
           obtain ⟨tail,last⟩ := pair
           simp only [tailRun,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at accepted
           obtain ⟨rfl,rfl⟩ := accepted
           exact .assignment ((MixedOwnerSourceCode.compile_exact control).mp compiled)
             ((captures_exact _).mp premises.1) premises.2 (ih tailRun)

theorem lower_complete {control : Nat} {used : List String}
 (meaning : Lowers p ctx labels control ordinal used env items entries final) :
 lower p ctx labels control ordinal used env items = some (entries,final) := by
 induction meaning with
 | nil => rfl
 | ordinary typed _ ih => simp [lower,(statement_exact _ _ _ _).mpr typed,ih]
 | assignment typed captures bound _ ih =>
   simp [lower,(MixedOwnerSourceCode.compile_exact _).mpr typed,(captures_exact _).mpr captures,bound,ih]

theorem lower_exact {control : Nat} {used : List String} :
 lower p ctx labels control ordinal used env items = some (entries,final) ↔
 Lowers p ctx labels control ordinal used env items entries final := ⟨lower_sound,lower_complete⟩

def generatedNames : List Entry → List String
 | [] => []
 | .ordinary _::rest => generatedNames rest
 | .install name _ _::rest | .write name _ _ _::rest => name::generatedNames rest

theorem lower_hygiene {control : Nat} {used : List String}
 (meaning : Lowers p ctx labels control ordinal used env items entries final) :
 (∀ name ∈ generatedNames entries, name ∉ used) ∧ (generatedNames entries).Nodup := by
 induction meaning with
 | nil => simp [generatedNames]
 | ordinary _ _ ih => exact ih
 | @assignment source code env ordinal used rest tail final control _ _ _ _ ih =>
   have definitionFresh := fresh_not_member (reserved:=used)
   have nameFresh := fresh_not_member (reserved:=freshName used::used)
   have nameDifferent : freshName (freshName used::used) ≠ freshName used := by
     intro same; exact nameFresh (by simp [same])
   have nameAbsent : freshName (freshName used::used) ∉ used := by
     intro present; exact nameFresh (List.mem_cons_of_mem _ present)
   have tailDefinition : freshName used ∉ generatedNames tail := by
     intro present; exact ih.1 _ present (by simp)
   have tailName : freshName (freshName used::used) ∉ generatedNames tail := by
     intro present; exact ih.1 _ present (by simp)
   constructor
   · intro name present
     simp only [generatedNames,construction,List.mem_cons] at present
     rcases present with rfl | rfl | prior
     · exact definitionFresh
     · exact nameAbsent
     · intro member; exact ih.1 _ prior (by simp [member])
   · simpa [generatedNames,construction,List.nodup_cons,nameDifferent.symm,tailDefinition,tailName] using ih.2

def reserved (env : Environment) (program : Program p) : List String := env.map Prod.fst ++ program.items.flatMap names

def compile (env : Environment) (program : Program p) : Option (List Entry × Environment) :=
 lower p program.fields program.labels program.control 0 (reserved env program) env program.items

#print axioms generated_width
#print axioms lower_hygiene
#print axioms lower_sound
#print axioms lower_complete
#print axioms lower_exact
#print axioms fresh_not_member
#print axioms integer_exact
#print axioms captures_exact
end MirroreaProofFirst.MixedOwnerProgram
