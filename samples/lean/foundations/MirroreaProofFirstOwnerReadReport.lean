import Std

/-! LAB: lossless representation of an already-recorded successful read map.
This does not authenticate the map, prove the evaluator read it, or declassify it.
Rows retain full key and value; formatting strings must not merge physical keys.
The concrete BTreeMap supplies unique keys, not temporal access order. -/
namespace MirroreaProofFirst.OwnerReadReport
variable {K V L : Type}
structure Row (K V L : Type) where
  locus : L
  key : K
  value : V
  deriving DecidableEq

def report (locus : L) (actual : List (K × V)) : List (Row K V L) :=
 actual.map fun (k,v) => ⟨locus,k,v⟩

-- Independent extensional relation: neither a missing row nor an invented read.
def Faithful (locus : L) (actual : List (K × V)) (rows : List (Row K V L)) : Prop :=
 (∀ row ∈ rows, row.locus = locus ∧ (row.key,row.value) ∈ actual) ∧
 (∀ k v, (k,v) ∈ actual → (⟨locus,k,v⟩ : Row K V L) ∈ rows)

theorem report_faithful (locus : L) (actual : List (K × V)) :
 Faithful locus actual (report locus actual) := by
 constructor
 · intro row member
   obtain ⟨⟨k,v⟩,present,equal⟩ := List.mem_map.mp member
   cases equal
   exact ⟨rfl,present⟩
 · intro k v present
   exact List.mem_map.mpr ⟨(k,v),present,rfl⟩

theorem report_exact (locus : L) (actual : List (K × V)) (k : K) (v : V) :
 (⟨locus,k,v⟩ : Row K V L) ∈ report locus actual ↔ (k,v) ∈ actual := by
 constructor
 · intro member
   exact ((report_faithful locus actual).1 _ member).2
 · exact (report_faithful locus actual).2 k v

theorem report_roundtrip (locus : L) (actual : List (K × V)) :
 (report locus actual).map (fun row => (row.key,row.value)) = actual := by
 induction actual with
 | nil => rfl
 | cons item rest ih =>
   cases item
   simpa only [report,List.map_cons] using congrArg (List.cons _) ih

theorem report_length (locus : L) (actual : List (K × V)) :
 (report locus actual).length = actual.length := by simp [report]

theorem report_empty (locus : L) : report (K:=K) (V:=V) locus [] = [] := rfl

theorem fabricated_target_not_faithful (locus : L) (actual : List (K × V))
 (target : K) (zero : V) (absent : (target,zero) ∉ actual) :
 ¬ Faithful locus actual (⟨locus,target,zero⟩ :: report locus actual) := by
 intro h
 exact absent (h.1 ⟨locus,target,zero⟩ (by simp)).2

theorem drop_nonempty_not_faithful (locus : L) (actual : List (K × V))
 (k : K) (v : V) (present : (k,v) ∈ actual) :
 ¬ Faithful locus actual [] := by
 intro h
 have := h.2 k v present
 simp at this

-- Minimal repair keeps existing Core order, but only emits successful lookups.
def selected (keys : List K) (lookup : K → Option V) : List (K × V) :=
 keys.filterMap fun k => (lookup k).map fun v => (k,v)

theorem selected_exact (keys : List K) (lookup : K → Option V) (k : K) (v : V) :
 (k,v) ∈ selected keys lookup ↔ k ∈ keys ∧ lookup k = some v := by
 simp only [selected,List.mem_filterMap]
 constructor
 · rintro ⟨key,member,equal⟩
   cases found : lookup key with
   | none => simp [found] at equal
   | some value =>
     simp only [found,Option.map_some,Option.some.injEq,Prod.mk.injEq] at equal
     obtain ⟨rfl,rfl⟩ := equal
     exact ⟨member,found⟩
 · rintro ⟨member,found⟩
   exact ⟨k,member,by simp [found]⟩

theorem selected_report_exact (locus : L) (keys : List K) (lookup : K → Option V)
 (k : K) (v : V) :
 (⟨locus,k,v⟩ : Row K V L) ∈ report locus (selected keys lookup) ↔
 k ∈ keys ∧ lookup k = some v := by rw [report_exact,selected_exact]

-- Coverage is a separate extraction obligation, never an assumption of success.
theorem covered_report_exact (locus : L) (keys : List K) (lookup : K → Option V)
 (covered : ∀ k v, lookup k = some v → k ∈ keys) (k : K) (v : V) :
 (⟨locus,k,v⟩ : Row K V L) ∈ report locus (selected keys lookup) ↔
 lookup k = some v := by
 rw [selected_report_exact]
 exact ⟨And.right,fun h => ⟨covered k v h,h⟩⟩

#print axioms selected_exact
#print axioms selected_report_exact
#print axioms covered_report_exact

-- Aliases have already coalesced in the concrete successful map. No source-name
-- injectivity is assumed, and list order is not called evaluation order.
example : report "S" [("avatar[self].hp",(21 : Int))] =
 [⟨"S","avatar[self].hp",21⟩] := rfl
example : ¬ Faithful "S" ([] : List (String × Int)) [⟨"S","hp",0⟩] := by
 exact fabricated_target_not_faithful "S" [] "hp" 0 (by simp)
#print axioms report_faithful
#print axioms report_exact
#print axioms report_roundtrip
#print axioms report_length
#print axioms report_empty
#print axioms fabricated_target_not_faithful
#print axioms drop_nonempty_not_faithful
end MirroreaProofFirst.OwnerReadReport
