import MirroreaProofFirstOwnerArgumentPresence
namespace MirroreaProofFirst.OwnerArgumentDomain
open OwnerTypedIndex OwnerStructuredKeys

-- A finite input map represented by its entries. Raw duplicate rows are
-- rejected here; Rust BTreeMap iteration already has distinct keys. A builder
-- which replaces an old value is not certified to have retained duplicate rows.
abbrev Arguments := List (String × String)
def names (signature : List Parameter) := signature.map Parameter.name
def lookup (args : Arguments) (name : String) := args.lookup name

def check (signature : List Parameter) (args : Arguments) : Bool :=
 decide (names signature).Nodup && decide (args.map Prod.fst).Nodup &&
 signature.all (fun p => (lookup args p.name).isSome) &&
 args.all (fun row => (names signature).contains row.1)

-- Declarative exact domain, stated over the original finite inventories.
-- Values remain strings: no scalar/nominal/authority claim is made.
def Matches (signature : List Parameter) (args : Arguments) : Prop :=
 (names signature).Nodup ∧ (args.map Prod.fst).Nodup ∧
 (∀ p ∈ signature, ∃ row ∈ args, p.name = row.1) ∧
 (∀ row ∈ args, row.1 ∈ names signature)

theorem check_exact : check signature args = true ↔ Matches signature args := by
 simp [check,Matches,lookup,List.all_eq_true,List.lookup_isSome_iff,and_assoc]

theorem matches_supplied (h : Matches signature args) :
 OwnerArgumentPresence.Supplied signature (lookup args) := by
 refine ⟨h.1,?_⟩
 intro p member
 have present : (lookup args p.name).isSome := by
   simpa [lookup,List.lookup_isSome_iff] using h.2.2.1 p member
 exact Option.isSome_iff_exists.mp present

theorem nonparameter_absent (h : Matches signature args)
 (outside : name ∉ names signature) : lookup args name = none := by
 apply List.lookup_eq_none_iff.mpr
 intro row member
 have ne : name ≠ row.1 := by
   intro same
   exact outside (same ▸ h.2.2.2 row member)
 simpa using ne

-- The existing substitution-or-literal operator is safe on this admitted
-- domain for nonparameters as well as parameters. It must consume this exact
-- signature/map pair at use; no caller-only certificate is assumed.
theorem nonparameter_fixed (h : Matches signature args)
 (outside : index ∉ names signature) (ns field owner : String) :
 materialize (lookup args) ⟨ns,some index,some field,owner⟩ =
 some ⟨ns,index,field⟩ := by
 simp [materialize,nonparameter_absent h outside]

theorem parameter_supplied (h : Matches signature args) (p : Parameter)
 (member : p ∈ signature) (ns field owner : String) :
 ∃ value, lookup args p.name = some value ∧
 materialize (lookup args) ⟨ns,some p.name,some field,owner⟩ =
 some ⟨ns,value,field⟩ :=
 OwnerArgumentPresence.supplied_index (matches_supplied h) p member ns field owner

theorem extra_refuses (row : String × String) (member : row ∈ args)
 (outside : row.1 ∉ names signature) : check signature args = false := by
 cases outcome : check signature args with
 | false => rfl
 | true => exact False.elim (outside ((check_exact.mp outcome).2.2.2 row member))

-- A result of refusal can be externally represented by the existing declared
-- failure machinery. Only the protected body state/effects below are framed;
-- enqueue IDs, authorization decisions and diagnostic trace are not erased.
structure Outcome (State Event Result : Type) where
 state : State
 events : List Event
 result : Option Result
 deriving DecidableEq, Repr

def invoke (signature : List Parameter) (args : Arguments) (state : State)
 (body : Arguments → State → Outcome State Event Result) : Outcome State Event Result :=
 if check signature args then body args state else ⟨state,[],none⟩

inductive Invocation (signature : List Parameter) (args : Arguments) (state : State)
 (body : Arguments → State → Outcome State Event Result) : Outcome State Event Result → Prop
 | admitted (domain : Matches signature args) : Invocation signature args state body (body args state)
 | refused (bad : ¬ Matches signature args) : Invocation signature args state body ⟨state,[],none⟩

theorem invoke_exact : invoke signature args state body = out ↔
 Invocation signature args state body out := by
 constructor
 · intro result
   unfold invoke at result
   split at result
   · rename_i admitted
     subst out
     exact Invocation.admitted (check_exact.mp admitted)
   · rename_i refused
     subst out
     exact Invocation.refused (fun h => refused (check_exact.mpr h))
 · intro derivation
   cases derivation with
   | admitted domain => simp [invoke,check_exact.mpr domain]
   | refused bad =>
     have rejected : check signature args ≠ true := fun h => bad (check_exact.mp h)
     simp [invoke,rejected]

theorem invoke_preserves (h : Matches signature args) :
 invoke signature args state body = body args state := by simp [invoke,check_exact.mpr h]

theorem invoke_refusal_frame (bad : ¬ Matches signature args) :
 (invoke signature args state body).state = state ∧
 (invoke signature args state body).events = [] ∧
 (invoke signature args state body).result = none := by
 have rejected : check signature args ≠ true := fun h => bad (check_exact.mp h)
 simp [invoke,rejected]

namespace Controls
def sig : List Parameter := [⟨"target","Player"⟩]
def good : Arguments := [("target","alice")]
def extra : Arguments := [("target","alice"),("self","mallory")]
#guard check sig good
#guard !check sig extra
#guard OwnerArgumentPresence.complete sig (lookup extra)
#guard !check sig []
#guard !check sig (good++good)
#guard check [] []
#guard !check [] [("self","mallory")]
#guard check [⟨"n","Int"⟩] [("n","bad-integer")]
def body (_ : Arguments) (s : Nat) : Outcome Nat Nat Nat := ⟨s+1,[s],some s⟩
#guard invoke sig good 7 body = ⟨8,[7],some 7⟩
#guard invoke sig extra 7 body = ⟨7,[],none⟩
end Controls
#print axioms check_exact
#print axioms nonparameter_fixed
#print axioms parameter_supplied
#print axioms invoke_exact
#print axioms invoke_refusal_frame
end MirroreaProofFirst.OwnerArgumentDomain
