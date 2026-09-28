import MirroreaProofFirstGeneralLabels
/-! Bounded LAB lemmas for the latest relation row's declared effective label.
    Supplied metadata and grant binding are assumptions, not authority issuance. -/
namespace MirroreaProofFirst.ObserverLabelProof
open GeneralLabels
variable {A : Type}
def joinMany (T : LabelTheory A) (base : A) : List A → A
 | [] => base
 | head :: tail => T.join head (joinMany T base tail)

theorem clearance_exact (T : LabelTheory A) (base cap : A) (labels : List A) :
 T.le (joinMany T base labels) cap = true ↔
 T.le base cap = true ∧ ∀ label ∈ labels, T.le label cap = true := by
 induction labels with
 | nil => simp [joinMany]
 | cons head tail ih =>
   simp only [joinMany,T.join_le,ih,List.mem_cons]
   constructor
   · rintro ⟨headOK,baseOK,tailOK⟩
     refine ⟨baseOK,?_⟩
     intro label member
     rcases member with same | member
     · simpa [same] using headOK
     · exact tailOK label member
   · rintro ⟨baseOK,allOK⟩
     exact ⟨allOK head (Or.inl rfl),baseOK,fun label member => allOK label (Or.inr member)⟩

theorem effective_covers (T : LabelTheory A) (base : A) (labels : List A) :
 T.le base (joinMany T base labels) = true ∧
 ∀ label ∈ labels, T.le label (joinMany T base labels) = true :=
 (clearance_exact T base _ labels).mp (T.refl _)

-- The independent binding test stays outside label arithmetic. Having a valid
-- label does not set this boolean; the runtime must use the same actual grant.
def admitted (T : LabelTheory A) (binding : Bool) (base cap : A) (labels : List A) : Bool :=
 binding && T.le (joinMany T base labels) cap

def Permitted (T : LabelTheory A) (binding : Bool) (base cap : A) (labels : List A) : Prop :=
 binding = true ∧ T.le base cap = true ∧ ∀ label ∈ labels, T.le label cap = true

theorem admitted_exact (T : LabelTheory A) (binding : Bool) (base cap : A) (labels : List A) :
 admitted T binding base cap labels = true ↔ Permitted T binding base cap labels := by
 simp only [admitted,Bool.and_eq_true,clearance_exact,Permitted]

theorem missing_binding_rejected (T : LabelTheory A) (base cap : A) (labels : List A) :
 admitted T false base cap labels = false := rfl

theorem insufficient_clearance_rejected (T : LabelTheory A) (binding : Bool)
 (base cap : A) (labels : List A) (label : A) (member : label ∈ labels)
 (denied : T.le label cap = false) : admitted T binding base cap labels = false := by
 cases h : admitted T binding base cap labels with
 | false => rfl
 | true =>
   have allowed := ((admitted_exact T binding base cap labels).mp h).2.2 label member
   rw [denied] at allowed
   contradiction
#print axioms clearance_exact
#print axioms effective_covers
#print axioms admitted_exact
#print axioms missing_binding_rejected
#print axioms insufficient_clearance_rejected

-- Actual output boundary: existential over whole grants, not pooled predicates.
def admitRows (T : LabelTheory A) (binding : G → Bool) (cap : G → A)
 (base : A) (rows : List A) (grants : List G) : Bool :=
 grants.any (fun grant => binding grant && T.le base (cap grant) &&
   rows.all (fun label => T.le label (cap grant)))

def RowsPermitted (T : LabelTheory A) (binding : G → Bool) (cap : G → A)
 (base : A) (rows : List A) (grants : List G) : Prop :=
 ∃ grant ∈ grants, binding grant = true ∧ T.le base (cap grant) = true ∧
   ∀ label ∈ rows, T.le label (cap grant) = true

theorem rows_exact (T : LabelTheory A) (binding : G → Bool) (cap : G → A)
 (base : A) (rows : List A) (grants : List G) :
 admitRows T binding cap base rows grants = true ↔ RowsPermitted T binding cap base rows grants := by
 simp [admitRows,RowsPermitted,List.any_eq_true,List.all_eq_true,and_assoc]

theorem empty_rows (T : LabelTheory A) (binding : G → Bool) (cap : G → A)
 (base : A) (grants : List G) :
 admitRows T binding cap base [] grants =
 grants.any (fun grant => binding grant && T.le base (cap grant)) := by
 simp [admitRows]

theorem no_binding (T : LabelTheory A) (binding : G → Bool) (cap : G → A)
 (base : A) (rows : List A) (grants : List G)
 (absent : ∀ grant ∈ grants, binding grant = false) :
 admitRows T binding cap base rows grants = false := by
 cases h : admitRows T binding cap base rows grants with
 | false => rfl
 | true =>
   obtain ⟨g,member,bound,_,_⟩ := (rows_exact T binding cap base rows grants).mp h
   rw [absent g member] at bound
   contradiction

-- The list here is the already selected kind-ordered observation vector. This
-- take is the Rust prefix bound, not Passive.feed's event-retention algorithm.
def rowClass (T : LabelTheory A) (base : A) (labels : List A) (relation : R → Bool) (row : R) : A :=
 if relation row then joinMany T base labels else base

theorem retained_row_cover (T : LabelTheory A) (base : A) (labels : List A)
 (relation : R → Bool) (selected : List R) (limit : Nat)
 (binding : G → Bool) (cap : G → A) (grants : List G)
 (accepted : admitRows T binding cap base
   ((selected.take limit).map (rowClass T base labels relation)) grants = true) :
 ∃ grant ∈ grants, binding grant = true ∧ T.le base (cap grant) = true ∧
   ∀ row ∈ selected.take limit,
     T.le (rowClass T base labels relation row) (cap grant) = true ∧
     (relation row = true → ∀ label ∈ labels, T.le label (cap grant) = true) := by
 obtain ⟨g,member,bound,baseOK,allOK⟩ := (rows_exact T binding cap base _ grants).mp accepted
 refine ⟨g,member,bound,baseOK,?_⟩
 intro row retained
 have rowOK := allOK _ (List.mem_map.mpr ⟨row,retained,rfl⟩)
 refine ⟨rowOK,?_⟩
 intro isRelation
 rw [rowClass,isRelation] at rowOK
 exact ((clearance_exact T base (cap g) labels).mp rowOK).2

theorem no_retained_relation (T : LabelTheory A) (base : A) (labels : List A)
 (relation : R → Bool) (selected : List R) (limit : Nat)
 (binding : G → Bool) (cap : G → A) (grants : List G)
 (absent : ∀ row ∈ selected.take limit, relation row = false) :
 admitRows T binding cap base ((selected.take limit).map (rowClass T base labels relation)) grants =
 grants.any (fun grant => binding grant && T.le base (cap grant)) := by
 apply Bool.eq_iff_iff.mpr
 rw [rows_exact]
 simp only [RowsPermitted,List.any_eq_true,Bool.and_eq_true]
 constructor
 · rintro ⟨g,member,bound,baseOK,_⟩
   exact ⟨g,member,bound,baseOK⟩
 · rintro ⟨g,member,bound,baseOK⟩
   refine ⟨g,member,bound,baseOK,?_⟩
   intro label member
   obtain ⟨row,retained,rfl⟩ := List.mem_map.mp member
   simpa [rowClass,absent row retained] using baseOK

-- Exhaustion of all three finite class constructors establishes this algebra
-- instance only; it is not a general program proof or a Rust compiler proof.
inductive M8Class where | publicClass | restrictedClass | privateClass
 deriving DecidableEq, Repr

def classRank : M8Class → Nat | .publicClass => 0 | .restrictedClass => 1 | .privateClass => 2

def m8Theory : LabelTheory M8Class where
 le a b := decide (classRank a ≤ classRank b)
 bottom := .publicClass
 join a b := if classRank a ≤ classRank b then b else a
 refl a := by cases a <;> decide
 trans := by intro a b c; cases a <;> cases b <;> cases c <;> decide
 bottom_le a := by cases a <;> decide
 join_le a b c := by cases a <;> cases b <;> cases c <;> decide

#guard admitRows m8Theory (fun (_ : Unit) => true) (fun _ => .privateClass) .publicClass [.privateClass] [()] = true
#guard admitRows m8Theory (fun (_ : Unit) => true) (fun _ => .restrictedClass) .publicClass [.privateClass] [()] = false
#guard admitRows m8Theory (fun (_ : Unit) => false) (fun _ => .privateClass) .publicClass [] [()] = false

-- Generic incomparable-label countermodel; impossible in the concrete M8 chain.
#guard admitRows GeneralLabels.Controls.diamond (fun (_ : Bool × Bool) => true) id
 (false,false) [(true,false),(false,true)] [(true,false),(false,true)] = false
#guard ([(true,false),(false,true)] : List (Bool × Bool)).all (fun row =>
 [(true,false),(false,true)].any (fun cap => GeneralLabels.Controls.diamond.le row cap)) = true

#print axioms rows_exact
#print axioms empty_rows
#print axioms no_binding
#print axioms retained_row_cover
#print axioms no_retained_relation
#print axioms m8Theory

end MirroreaProofFirst.ObserverLabelProof
