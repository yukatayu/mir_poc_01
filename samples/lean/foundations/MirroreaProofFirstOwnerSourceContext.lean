import MirroreaProofFirstOwnerStructuredKeys
namespace MirroreaProofFirst.OwnerSourceContext
open OwnerStructuredKeys
variable {read : Read}

-- A projection of checked source declarations, not a new source syntax.
-- `visibility` names a requested channel; it never supplies a security label,
-- credential, declassification permission, or observer grant.
structure Declaration where
 namespaceName : String
 field : String
 owner : String
 typeName : String
 visibility : Option String
 deriving DecidableEq, Repr

-- Owner-side retained metadata. Authentic admission and serialization with
-- use are explicit consumer obligations, NOT consequences of this datatype.
structure Metadata where
 declaration : Declaration
 generation : Nat
 label : Nat
 deriving DecidableEq, Repr

abbrev Schema := List Declaration
def candidates (schema : Schema) (key : Key) : List Declaration :=
 schema.filter fun d => d.namespaceName == key.namespaceName && d.field == key.field

def select (schema : Schema) (key : Key) : Option Declaration :=
 match candidates schema key with | [d] => some d | _ => none

abbrev Declared (schema : Schema) (key : Key) (d : Declaration) : Prop :=
 candidates schema key = [d]

theorem select_exact : select schema key = some d ↔ Declared schema key d := by
 unfold select Declared
 cases candidates schema key with
 | nil => simp
 | cons head tail => cases tail <;> simp [eq_comm]

structure Binding where
 key : Key
 metadata : Metadata
 deriving DecidableEq, Repr

def bind (schema : Schema) (current : Key → Option Metadata)
 (arguments : String → Option String) (read : Read) : Option Binding := do
 let key ← materialize arguments read
 let declaration ← select schema key
 let metadata ← current key
 if declaration.owner = read.owner ∧ declaration.typeName = "Int" ∧
   metadata.declaration = declaration then some ⟨key,metadata⟩ else none

-- Independent requirements on successful elaboration: structural addressing,
-- a unique checked source declaration, its integer/owner typing, and EXACT
-- current metadata. Neither bind success nor any desired preservation result
-- is a premise. This is a context binding judgment, not auth admission.
def Binds (schema : Schema) (current : Key → Option Metadata)
 (arguments : String → Option String) (read : Read) (out : Binding) : Prop :=
 Materializes arguments read out.key ∧
 ∃ declaration, Declared schema out.key declaration ∧
 declaration.owner = read.owner ∧ declaration.typeName = "Int" ∧
 current out.key = some out.metadata ∧ out.metadata.declaration = declaration

theorem bind_exact : bind schema current arguments read = some out ↔
 Binds schema current arguments read out := by
 constructor
 · intro accepted
   unfold bind at accepted
   cases keyFound : materialize arguments read with
   | none => simp [keyFound] at accepted
   | some key =>
     cases declared : select schema key with
     | none => simp [keyFound,declared] at accepted
     | some declaration =>
       cases found : current key with
       | none => simp [keyFound,declared,found] at accepted
       | some metadata =>
         simp only [keyFound,declared,found,Option.bind_eq_bind,Option.bind_some] at accepted
         split at accepted
         · rename_i facts
           cases accepted
           exact ⟨materialize_exact.mp keyFound,declaration,select_exact.mp declared,
             facts.1,facts.2.1,found,facts.2.2⟩
         · cases accepted
 · rintro ⟨keyFound,declaration,declared,owner,integer,found,same⟩
   cases out
   simp_all [bind,materialize_exact.mpr keyFound,select_exact.mpr declared]

theorem bound_current (accepted : bind schema current arguments read = some out) :
 current out.key = some out.metadata := (bind_exact.mp accepted).2.choose_spec.2.2.2.1

theorem bound_label (accepted : bind schema current arguments read = some out) :
 (current out.key).map Metadata.label = some out.metadata.label := by
 rw [bound_current accepted]; rfl

-- Recheck after queueing / before use. Full equality includes label, declaration
-- and generation; a same-looking field in a different generation does not pass.
def usable (schema : Schema) (current : Key → Option Metadata) (bound : Binding) : Bool :=
 decide (Declared schema bound.key bound.metadata.declaration ∧ current bound.key = some bound.metadata)

def Usable (schema : Schema) (current : Key → Option Metadata) (bound : Binding) : Prop :=
 Declared schema bound.key bound.metadata.declaration ∧ current bound.key = some bound.metadata

theorem usable_exact : usable schema current bound = true ↔ Usable schema current bound := by
 simp [usable,Usable]

theorem freshly_bound_usable (accepted : bind schema current arguments read = some out) :
 usable schema current out = true := by
 obtain ⟨_,declaration,declared,_,_,found,same⟩ := bind_exact.mp accepted
 exact usable_exact.mpr ⟨same ▸ declared,found⟩

theorem replaced_metadata_refuses (now : current bound.key = some different)
 (changed : different ≠ bound.metadata) : usable schema current bound = false := by
 simp [usable,now,changed]

theorem retired_refuses (absent : current bound.key = none) : usable schema current bound = false := by
 simp [usable,absent]

theorem accepted_current_generation (accepted : usable schema current bound = true)
 (found : current bound.key = some metadata) : metadata.generation = bound.metadata.generation := by
 have same := (usable_exact.mp accepted).2
 rw [found] at same
 exact congrArg Metadata.generation (Option.some.inj same)

theorem accepted_current_label (accepted : usable schema current bound = true)
 (found : current bound.key = some metadata) : metadata.label = bound.metadata.label := by
 have same := (usable_exact.mp accepted).2
 rw [found] at same
 exact congrArg Metadata.label (Option.some.inj same)

-- Every state-read occurrence is elaborated; callers cannot provide a shorter
-- list of allegedly relevant metadata. Aliases remain repeated occurrences of
-- the same canonical key, with metadata taken from the SAME owner context.
open OwnerCheckedArithmetic
def bindTree (schema : Schema) (current : Key → Option Metadata)
 (arguments : String → Option String) : Checked Read String →
 Option (Checked Key String × List Binding)
 | .state read => do
   let bound ← bind schema current arguments read
   return (.state bound.key,[bound])
 | .parameter name => some (.parameter name,[])
 | .integer value => some (.integer value,[])
 | .add left right => do
   let (a,x) ← bindTree schema current arguments left
   let (b,y) ← bindTree schema current arguments right
   return (.add a b,x++y)
 | .sub left right => do
   let (a,x) ← bindTree schema current arguments left
   let (b,y) ← bindTree schema current arguments right
   return (.sub a b,x++y)

inductive TreeBinds (schema : Schema) (current : Key → Option Metadata)
 (arguments : String → Option String) : Checked Read String →
 Checked Key String → List Binding → Prop where
 | state {read : Read} : Binds schema current arguments read bound →
   TreeBinds schema current arguments (.state read) (.state bound.key) [bound]
 | parameter : TreeBinds schema current arguments (.parameter name) (.parameter name) []
 | integer : TreeBinds schema current arguments (.integer value) (.integer value) []
 | add : TreeBinds schema current arguments left a x →
   TreeBinds schema current arguments right b y →
   TreeBinds schema current arguments (.add left right) (.add a b) (x++y)
 | sub : TreeBinds schema current arguments left a x →
   TreeBinds schema current arguments right b y →
   TreeBinds schema current arguments (.sub left right) (.sub a b) (x++y)

theorem bind_tree_sound
 (accepted : bindTree schema current arguments source = some (tree,bindings)) :
 TreeBinds schema current arguments source tree bindings := by
 induction source generalizing tree bindings with
 | state read =>
   cases found : bind schema current arguments read with
   | none => simp [bindTree,found] at accepted
   | some bound =>
     simp only [bindTree,found,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at accepted
     obtain ⟨rfl,rfl⟩ := accepted
     exact .state (bind_exact.mp found)
 | parameter name => cases accepted; exact .parameter
 | integer value => cases accepted; exact .integer
 | add left right il ir =>
   cases l : bindTree schema current arguments left <;>
     cases r : bindTree schema current arguments right <;> simp [bindTree,l,r] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   exact .add (il l) (ir r)
 | sub left right il ir =>
   cases l : bindTree schema current arguments left <;>
     cases r : bindTree schema current arguments right <;> simp [bindTree,l,r] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   exact .sub (il l) (ir r)

theorem bind_tree_complete (meaning : TreeBinds schema current arguments source tree bindings) :
 bindTree schema current arguments source = some (tree,bindings) := by
 induction meaning with
 | state meaning => simp [bindTree,bind_exact.mpr meaning]
 | parameter | integer => rfl
 | add _ _ il ir => simp [bindTree,il,ir]
 | sub _ _ il ir => simp [bindTree,il,ir]

theorem bind_tree_exact : bindTree schema current arguments source = some (tree,bindings) ↔
 TreeBinds schema current arguments source tree bindings := ⟨bind_tree_sound,bind_tree_complete⟩

theorem tree_bindings_current (meaning : TreeBinds schema current arguments source tree bindings) :
 ∀ bound ∈ bindings, Usable schema current bound := by
 induction meaning with
 | state meaning =>
   intro bound member
   simp only [List.mem_singleton] at member
   subst bound
   exact usable_exact.mp (freshly_bound_usable (bind_exact.mpr meaning))
 | parameter | integer => simp
 | add _ _ il ir | sub _ _ il ir =>
   intro bound member
   rcases List.mem_append.mp member with left | right
   · exact il bound left
   · exact ir bound right

def references : Checked Read String → List Read
 | .state ref => [ref]
 | .parameter _ | .integer _ => []
 | .add a b | .sub a b => references a ++ references b

-- Both directions: no source read loses a metadata binding and no unrelated
-- read binding is invented. Target metadata is handled separately below.
theorem tree_coverage (meaning : TreeBinds schema current arguments source tree bindings) :
 ∀ ref ∈ references source, ∃ bound ∈ bindings, Binds schema current arguments ref bound := by
 induction meaning with
 | state meaning =>
   intro ref member
   simp only [references,List.mem_singleton] at member
   subst ref
   exact ⟨_,by simp,meaning⟩
 | parameter | integer => simp [references]
 | add _ _ il ir | sub _ _ il ir =>
   intro ref member
   rcases List.mem_append.mp member with left | right
   · obtain ⟨bound,present,binds⟩ := il ref left
     exact ⟨bound,List.mem_append_left _ present,binds⟩
   · obtain ⟨bound,present,binds⟩ := ir ref right
     exact ⟨bound,List.mem_append_right _ present,binds⟩

theorem tree_no_invented_binding (meaning : TreeBinds schema current arguments source tree bindings) :
 ∀ bound ∈ bindings, ∃ ref ∈ references source, Binds schema current arguments ref bound := by
 induction meaning with
 | state meaning =>
   intro bound member
   simp only [List.mem_singleton] at member
   subst bound
   exact ⟨_,by simp [references],meaning⟩
 | parameter | integer => simp
 | add _ _ il ir | sub _ _ il ir =>
   intro bound member
   rcases List.mem_append.mp member with left | right
   · obtain ⟨ref,present,binds⟩ := il bound left
     exact ⟨ref,List.mem_append_left _ present,binds⟩
   · obtain ⟨ref,present,binds⟩ := ir bound right
     exact ⟨ref,List.mem_append_right _ present,binds⟩

theorem tree_evaluation (meaning : TreeBinds schema current arguments source tree bindings)
 (ops : FallibleFlow.Arithmetic) (store : Key → Option Int) (captures : String → Option Int) :
 evaluate ops store captures tree =
 evaluate ops (fun read => (materialize arguments read).bind store) captures source := by
 induction meaning with
 | state meaning => simp [evaluate,materialize_exact.mpr meaning.1]
 | parameter | integer => rfl
 | add _ _ il ir | sub _ _ il ir => simp only [evaluate,il,ir]

def retainedCheck (schema : Schema) (current : Key → Option Metadata) (bindings : List Binding) : Bool :=
 bindings.all (usable schema current)

theorem retained_exact : retainedCheck schema current bindings = true ↔
 ∀ bound ∈ bindings, Usable schema current bound := by
 simp [retainedCheck,List.all_eq_true,usable_exact]

theorem tree_retained (accepted : bindTree schema current arguments source = some (tree,bindings)) :
 retainedCheck schema current bindings = true := retained_exact.mpr (tree_bindings_current (bind_tree_sound accepted))

theorem changed_read_refuses (member : bound ∈ bindings)
 (changed : current bound.key ≠ some bound.metadata) : retainedCheck schema current bindings = false := by
 cases result : retainedCheck schema current bindings with
 | false => rfl
 | true => exact False.elim (changed ((retained_exact.mp result) bound member).2)

def Local (owner : String) : Checked Read String → Prop
 | .state ref => ref.owner = owner
 | .parameter _ | .integer _ => True
 | .add a b | .sub a b => Local owner a ∧ Local owner b

def localCheck (owner : String) : Checked Read String → Bool
 | .state ref => ref.owner == owner
 | .parameter _ | .integer _ => true
 | .add a b | .sub a b => localCheck owner a && localCheck owner b

theorem local_exact (source : Checked Read String) : localCheck owner source = true ↔ Local owner source := by
 induction source <;> simp_all [localCheck,Local]

structure Assignment where
 target : Read
 rhs : Checked Read String

structure BoundAssignment where
 target : Binding
 rhs : Checked Key String
 reads : List Binding

-- Target metadata is required even for a constant RHS. It is NOT reported as
-- a value read; expression reads and target metadata remain separate.
def bindAssignment (schema : Schema) (current : Key → Option Metadata)
 (arguments : String → Option String) (source : Assignment) : Option BoundAssignment := do
 let target ← bind schema current arguments source.target
 let (tree,reads) ← bindTree schema current arguments source.rhs
 if localCheck source.target.owner source.rhs then some ⟨target,tree,reads⟩ else none

def AssignmentBinds (schema : Schema) (current : Key → Option Metadata)
 (arguments : String → Option String) (source : Assignment) (out : BoundAssignment) : Prop :=
 Binds schema current arguments source.target out.target ∧
 TreeBinds schema current arguments source.rhs out.rhs out.reads ∧
 Local source.target.owner source.rhs

theorem assignment_exact : bindAssignment schema current arguments source = some out ↔
 AssignmentBinds schema current arguments source out := by
 constructor
 · intro accepted
   unfold bindAssignment at accepted
   cases target : bind schema current arguments source.target with
   | none => simp [target] at accepted
   | some bound =>
     cases tree : bindTree schema current arguments source.rhs with
     | none => simp [target,tree] at accepted
     | some pair =>
       obtain ⟨rhs,reads⟩ := pair
       simp only [target,tree,Option.bind_eq_bind,Option.bind_some] at accepted
       split at accepted
       · rename_i locality
         cases accepted
         exact ⟨bind_exact.mp target,bind_tree_sound tree,local_exact _ |>.mp locality⟩
       · cases accepted
 · rintro ⟨target,tree,locality⟩
   cases out
   simp_all [bindAssignment,bind_exact.mpr target,bind_tree_complete tree,local_exact _ |>.mpr locality]

def assignmentUsable (schema : Schema) (current : Key → Option Metadata) (bound : BoundAssignment) : Bool :=
 usable schema current bound.target && retainedCheck schema current bound.reads

theorem assignment_fresh (accepted : bindAssignment schema current arguments source = some out) :
 assignmentUsable schema current out = true := by
 obtain ⟨target,reads,_⟩ := assignment_exact.mp accepted
 simp [assignmentUsable,freshly_bound_usable (bind_exact.mpr target),tree_retained (bind_tree_complete reads)]

theorem assignment_target_current (accepted : assignmentUsable schema current bound = true) :
 current bound.target.key = some bound.target.metadata := by
 have parts : usable schema current bound.target = true ∧ retainedCheck schema current bound.reads = true := by simpa [assignmentUsable] using accepted
 exact (usable_exact.mp parts.1).2

theorem assignment_reads_current (accepted : assignmentUsable schema current bound = true) :
 ∀ read ∈ bound.reads, current read.key = some read.metadata := by
 intro read member
 have parts : usable schema current bound.target = true ∧ retainedCheck schema current bound.reads = true := by simpa [assignmentUsable] using accepted
 exact ((retained_exact.mp parts.2) read member).2

namespace Controls
def key : Key := ⟨"player","self","hp"⟩
def declaration : Declaration := ⟨"player","hp","S","Int",some "observer_safe"⟩
def privateMetadata : Metadata := ⟨declaration,7,3⟩
def current (k : Key) : Option Metadata := if k=key then some privateMetadata else none
def checkedRead : Read := ⟨"player",some "self",some "hp","S"⟩
def bound : Binding := ⟨key,privateMetadata⟩
#guard bind [declaration] current (fun _ => none) checkedRead = some bound
-- observer_safe does not lower the actual private label.
#guard (bind [declaration] current (fun _ => none) checkedRead |>.map fun b => b.metadata.label) = some 3
#guard (bind [declaration,declaration] current (fun _ => none) checkedRead).isNone
#guard (bind [declaration] current (fun _ => none) {checkedRead with owner := "T"}).isNone
#guard (bind [declaration] current (fun _ => none) {checkedRead with namespaceName := "other"}).isNone
#guard !usable [declaration] (fun _ => some {privateMetadata with generation := 8}) bound
#guard !usable [declaration] (fun _ => some {privateMetadata with label := 0}) bound
#guard !usable [] current bound
#guard !usable [declaration] (fun _ => none) bound
def rhs : Checked Read String := .add (.state checkedRead) (.parameter "amount")
#guard (bindTree [declaration] current (fun _ => none) rhs |>.map fun x => x.2) = some [bound]
#guard (bindAssignment [declaration] current (fun _ => none) ⟨checkedRead,.integer 5⟩ |>.map fun x => x.reads) = some []
#guard (bindAssignment [declaration] current (fun _ => none) ⟨{checkedRead with field := some "missing"},.integer 5⟩).isNone
def foreignDeclaration : Declaration := ⟨"other","hp","T","Int",none⟩
def foreignRead : Read := ⟨"other",some "self",some "hp","T"⟩
def withForeign (k : Key) : Option Metadata := if k.namespaceName="other" then some ⟨foreignDeclaration,1,3⟩ else current k
#guard (bindTree [declaration,foreignDeclaration] withForeign (fun _ => none) (.state foreignRead)).isSome
#guard (bindAssignment [declaration,foreignDeclaration] withForeign (fun _ => none) ⟨checkedRead,.state foreignRead⟩).isNone
#guard (bindTree [declaration] current (fun _ => none) (.add (.state checkedRead) (.state checkedRead)) |>.map fun x => x.2) = some [bound,bound]
#guard (bindAssignment [declaration] current (fun _ => none) ⟨checkedRead,rhs⟩ |>.map fun b => assignmentUsable [declaration] (fun _ => some {privateMetadata with generation := 8}) b) = some false
#guard (bindAssignment [declaration] current (fun _ => none) ⟨checkedRead,.integer 5⟩ |>.map fun b => assignmentUsable [declaration] (fun _ => none) b) = some false
end Controls

#print axioms bind_exact
#print axioms bound_label
#print axioms freshly_bound_usable
#print axioms replaced_metadata_refuses
#print axioms retired_refuses
#print axioms accepted_current_generation
#print axioms accepted_current_label
#print axioms bind_tree_exact
#print axioms tree_bindings_current
#print axioms tree_coverage
#print axioms tree_no_invented_binding
#print axioms tree_evaluation
#print axioms tree_retained
#print axioms changed_read_refuses
#print axioms local_exact
#print axioms assignment_exact
#print axioms assignment_fresh
#print axioms assignment_target_current
#print axioms assignment_reads_current
end MirroreaProofFirst.OwnerSourceContext
