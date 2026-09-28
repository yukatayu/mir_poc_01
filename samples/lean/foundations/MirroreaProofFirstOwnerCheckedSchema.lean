import MirroreaProofFirstOwnerSourceContext
namespace MirroreaProofFirst.OwnerCheckedSchema
open OwnerStructuredKeys OwnerSourceContext

-- Pure projection of M7's retained state/field declarations. This datatype does
-- not authenticate an arbitrary source image. Actual sealed M7 and its checked
-- identity/source reference carrier remain the corresponding implementation TCB.
structure Field where
 name : String
 typeName : String
 visibility : Option String
 deriving DecidableEq, Repr
structure StateDeclaration where
 name : String
 indexName : String
 indexType : String
 owner : String
 fields : List Field
 deriving DecidableEq, Repr

def row (state : StateDeclaration) (field : Field) : Declaration :=
 ⟨state.name,field.name,state.owner,field.typeName,field.visibility⟩
def project (states : List StateDeclaration) : Schema :=
 states.flatMap fun state => state.fields.map (row state)

-- Independent source membership; no call to project/select or their outputs.
def Origin (states : List StateDeclaration) (d : Declaration) : Prop :=
 ∃ state ∈ states, ∃ field ∈ state.fields,
 d.namespaceName = state.name ∧ d.field = field.name ∧ d.owner = state.owner ∧
 d.typeName = field.typeName ∧ d.visibility = field.visibility

theorem projection_origin : d ∈ project states ↔ Origin states d := by
 simp only [project,List.mem_flatMap,List.mem_map]
 constructor
 · rintro ⟨state,member,field,present,equal⟩
   subst d
   exact ⟨state,member,field,present,rfl,rfl,rfl,rfl,rfl⟩
 · rintro ⟨state,member,field,present,ns,name,owner,ty,visibility⟩
   refine ⟨state,member,field,present,?_⟩
   cases d
   simp_all [row]

theorem selected_origin (selected : select (project states) key = some d) : Origin states d := by
 apply projection_origin.mp
 have candidates := select_exact.mp selected
 have member : d ∈ OwnerSourceContext.candidates (project states) key := by rw [candidates]; simp
 exact (List.mem_filter.mp member).1

theorem declared_no_shadow (one : Declared schema key d) (two : Declared schema key e) : d=e := by
 have equal : [d] = [e] := one.symm.trans two
 simpa using equal

-- An accepted owner binding keeps each source declaration component; observer
-- visibility is retained as a request and does not determine metadata.label.
theorem bound_source_origin {read : Read} (bound : bind (project states) current arguments read = some out) :
 Origin states out.metadata.declaration ∧ out.metadata.declaration.owner = read.owner ∧
 out.metadata.declaration.typeName = "Int" ∧ current out.key = some out.metadata := by
 obtain ⟨_,d,declared,owner,integer,found,same⟩ := bind_exact.mp bound
 exact ⟨same ▸ selected_origin (select_exact.mpr declared),same ▸ owner,same ▸ integer,found⟩

-- Global uniqueness before owner selection avoids silently hiding a duplicate
-- declaration during per-owner partition. No schema ownership or permission is
-- manufactured: the owner and expected integer type are explicit conditions.
def selectForOwner (states : List StateDeclaration) (key : Key) (owner : String) : Option Declaration := do
 let d ← select (project states) key
 if d.owner=owner ∧ d.typeName="Int" then some d else none

def Resolves (states : List StateDeclaration) (key : Key) (owner : String) (d : Declaration) : Prop :=
 Declared (project states) key d ∧ d.owner=owner ∧ d.typeName="Int"

theorem owner_exact : selectForOwner states key owner = some d ↔ Resolves states key owner d := by
 unfold Resolves
 rw [←select_exact]
 cases found : select (project states) key with
 | none => simp [selectForOwner,found]
 | some declaration =>
   by_cases compatible : declaration.owner=owner ∧ declaration.typeName="Int"
   · simp [selectForOwner,found,compatible]
     intro same
     subst d
     exact compatible
   · simp [selectForOwner,found,compatible]
     intro same
     subst d
     intro own integer
     exact compatible ⟨own,integer⟩

theorem selected_owner_origin (accepted : selectForOwner states key owner = some d) :
 Origin states d ∧ d.owner=owner ∧ d.typeName="Int" := by
 obtain ⟨declared,rest⟩ := owner_exact.mp accepted
 exact ⟨selected_origin (select_exact.mpr declared),rest⟩

-- Capture-coordinate erasure is independent of field-key erasure. The exact
-- current capture values/labels and arity still belong to the source consumer.
def renameParameters {K J H : Type} (names : J → H) : OwnerCheckedArithmetic.Checked K J → OwnerCheckedArithmetic.Checked K H
 | .state key => .state key
 | .parameter name => .parameter (names name)
 | .integer value => .integer value
 | .add a b => .add (renameParameters names a) (renameParameters names b)
 | .sub a b => .sub (renameParameters names a) (renameParameters names b)

theorem parameters_evaluation {K J H : Type} (names : J → H)
 (tree : OwnerCheckedArithmetic.Checked K J) (ops : FallibleFlow.Arithmetic)
 (store : K → Option Int) (args : H → Option Int) :
 OwnerCheckedArithmetic.evaluate ops store args (renameParameters names tree) =
 OwnerCheckedArithmetic.evaluate ops store (fun name => args (names name)) tree := by
 induction tree <;> simp_all [renameParameters,OwnerCheckedArithmetic.evaluate]

namespace Controls
def hp : Field := ⟨"hp","Int",none⟩
def shown : Field := ⟨"shown","Int",some "observer_safe"⟩
def a : StateDeclaration := ⟨"player","id","Player","S",[hp,shown]⟩
def b : StateDeclaration := ⟨"team","id","Team","T",[hp]⟩
def key : Key := ⟨"player","self","hp"⟩
def duplicate := {a with owner := "T"}
#guard project [a,b] = [row a hp,row a shown,row b hp]
#guard selectForOwner [a,b] key "S" = some (row a hp)
#guard (selectForOwner [a,b] key "T").isNone
#guard (selectForOwner [a,duplicate] key "S").isNone
#guard (selectForOwner [a,duplicate] key "T").isNone
-- Smallest alternative masks the collision by selecting the owner first.
#guard (selectForOwner ([a,duplicate].filter fun s => s.owner=="S") key "S").isSome
#guard (selectForOwner [a,b] ⟨"team","self","hp"⟩ "T").isSome
#guard (selectForOwner [a] {key with field := "absent"} "S").isNone
end Controls
#print axioms parameters_evaluation
#print axioms projection_origin
#print axioms selected_origin
#print axioms declared_no_shadow
#print axioms bound_source_origin
#print axioms owner_exact
#print axioms selected_owner_origin
end MirroreaProofFirst.OwnerCheckedSchema
