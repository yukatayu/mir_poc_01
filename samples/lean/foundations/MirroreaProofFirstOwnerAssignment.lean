import MirroreaProofFirstFallibleFlow

namespace MirroreaProofFirst.OwnerAssignment
open ProducerFlow
variable {K J : Type}

-- Frozen is an explicit immutable value boundary, not an implicit distributed cut.
abbrev Ref (K J : Type) := Sum K J
def environment (live : K → Value) (frozen : J → Value) : Ref K J → Value :=
 Sum.elim live frozen
def classes (live : K → Nat) (frozen : J → Nat) : Ref K J → Nat :=
 Sum.elim live frozen
def types (live : K → Ty) (frozen : J → Ty) : Ref K J → Ty :=
 Sum.elim live frozen

-- Unsafe comparison only: substitution preserves values but drops frozen labels.
def eraseFrozen (snap : J → Value) : Expr (Ref K J) → Expr (Ref K J)
 | .lit v => .lit v
 | .read (.inl k) => .read (.inl k)
 | .read (.inr j) => .lit (snap j)
 | .add a b => .add (eraseFrozen snap a) (eraseFrozen snap b)
 | .sub a b => .sub (eraseFrozen snap a) (eraseFrozen snap b)
 | .choose c a b => .choose (eraseFrozen snap c) (eraseFrozen snap a) (eraseFrozen snap b)

theorem erasure_value_only (ops : FallibleFlow.Arithmetic) (live : K → Value)
 (snap : J → Value) (e : Expr (Ref K J)) :
 FallibleFlow.eval ops (environment live snap) (eraseFrozen snap e) =
 FallibleFlow.eval ops (environment live snap) e := by
 induction e with
 | lit => rfl
 | read r => cases r <;> rfl
 | add a b ha hb => simp only [eraseFrozen,FallibleFlow.eval,ha,hb]
 | sub a b ha hb => simp only [eraseFrozen,FallibleFlow.eval,ha,hb]
 | choose c a b hc ha hb => simp only [eraseFrozen,FallibleFlow.eval,hc,ha,hb]

-- Independent owner discipline ranges over all branches, including guards.
inductive Local (owner : K → Nat) (targetOwner : Nat) : Expr (Ref K J) → Prop where
 | lit : Local owner targetOwner (.lit v)
 | live : owner k = targetOwner → Local owner targetOwner (.read (.inl k))
 | frozen : Local owner targetOwner (.read (.inr j))
 | add : Local owner targetOwner a → Local owner targetOwner b → Local owner targetOwner (.add a b)
 | sub : Local owner targetOwner a → Local owner targetOwner b → Local owner targetOwner (.sub a b)
 | choose : Local owner targetOwner c → Local owner targetOwner a → Local owner targetOwner b →
    Local owner targetOwner (.choose c a b)

def localCheck (owner : K → Nat) (targetOwner : Nat) : Expr (Ref K J) → Bool
 | .lit _ => true
 | .read (.inl k) => decide (owner k = targetOwner)
 | .read (.inr _) => true
 | .add a b | .sub a b => localCheck owner targetOwner a && localCheck owner targetOwner b
 | .choose c a b => localCheck owner targetOwner c &&
    localCheck owner targetOwner a && localCheck owner targetOwner b

theorem local_exact (owner : K → Nat) (targetOwner : Nat) (e : Expr (Ref K J)) :
 localCheck owner targetOwner e = true ↔ Local owner targetOwner e := by
 induction e with
 | lit v => exact ⟨fun _ => .lit,fun _ => rfl⟩
 | read r =>
   cases r with
   | inl k =>
     simp only [localCheck,decide_eq_true_eq]
     exact ⟨Local.live,fun h => by cases h; assumption⟩
   | inr j => exact ⟨fun _ => .frozen,fun _ => rfl⟩
 | add a b ha hb =>
   simp only [localCheck,Bool.and_eq_true,ha,hb]
   exact ⟨fun ⟨a,b⟩ => .add a b,fun h => by cases h; constructor <;> assumption⟩
 | sub a b ha hb =>
   simp only [localCheck,Bool.and_eq_true,ha,hb]
   exact ⟨fun ⟨a,b⟩ => .sub a b,fun h => by cases h; constructor <;> assumption⟩
 | choose c a b hc ha hb =>
   simp only [localCheck,Bool.and_eq_true,hc,ha,hb]
   exact ⟨fun ⟨⟨c,a⟩,b⟩ => .choose c a b,
     fun h => by cases h; exact ⟨⟨by assumption,by assumption⟩,by assumption⟩⟩

-- Other owners may change after capture. No re-read of their live state is hidden.
theorem foreign_changes_irrelevant (ops : FallibleFlow.Arithmetic)
 {owner : K → Nat} {o : Nat} {e : Expr (Ref K J)} (locality : Local owner o e)
 (s t : K → Value) (snap : J → Value)
 (same : ∀ k, owner k = o → s k = t k) :
 FallibleFlow.eval ops (environment s snap) e = FallibleFlow.eval ops (environment t snap) e := by
 induction locality with
 | lit => rfl
 | live h => exact congrArg some (same _ h)
 | frozen => rfl
 | add ha hb ia ib => simp only [FallibleFlow.eval,ia,ib]
 | sub ha hb ia ib => simp only [FallibleFlow.eval,ia,ib]
 | choose hc ha hb ic ia ib => simp only [FallibleFlow.eval,ic,ia,ib]

variable [DecidableEq K]
-- The destination type excludes a frozen-value write by construction.
def run (ops : FallibleFlow.Arithmetic) (s : K → Value) (snap : J → Value)
 (target : K) (e : Expr (Ref K J)) : (K → Value) × FallibleFlow.Outcome K :=
 match FallibleFlow.eval ops (environment s snap) e with
 | none => (s,.failed target)
 | some v => (put s target v,.wrote target v)

theorem run_frame (ops : FallibleFlow.Arithmetic) (s : K → Value) (snap : J → Value)
 (target : K) (e : Expr (Ref K J)) (k : K) (different : k ≠ target) :
 (run ops s snap target e).1 k = s k := by
 unfold run; split <;> simp [put,different]

theorem run_failure (ops : FallibleFlow.Arithmetic) (s : K → Value) (snap : J → Value)
 (target : K) (e : Expr (Ref K J)) (failed : FallibleFlow.eval ops (environment s snap) e = none) :
 run ops s snap target e = (s,.failed target) := by simp [run,failed]

-- Capture metadata/authority and current target resolution are separate premises.
structure Admissible (owner : K → Nat) (G : K → Ty) (SG : J → Ty)
 (L : K → Nat) (SL : J → Nat) (pc : Nat) (target : K) (e : Expr (Ref K J)) : Prop where
 locality : Local owner (owner target) e
 typed : Typed (types G SG) e (G target)
 flow : Flows (classes L SL) (L target) e
 control : pc ≤ L target

def check (owner : K → Nat) (G : K → Ty) (SG : J → Ty)
 (L : K → Nat) (SL : J → Nat) (pc : Nat) (target : K) (e : Expr (Ref K J)) : Bool :=
 localCheck owner (owner target) e && decide (infer (types G SG) e = some (G target) ∧
 rank (classes L SL) e ≤ L target ∧ pc ≤ L target)

omit [DecidableEq K] in
theorem check_exact (owner : K → Nat) (G : K → Ty) (SG : J → Ty)
 (L : K → Nat) (SL : J → Nat) (pc : Nat) (target : K) (e : Expr (Ref K J)) :
 check owner G SG L SL pc target e = true ↔ Admissible owner G SG L SL pc target e := by
 simp only [check,Bool.and_eq_true,decide_eq_true_eq,local_exact,infer_exact]
 constructor
 · rintro ⟨hl,ht,hf,hpc⟩
   exact ⟨hl,ht,(flows_exact _ _ _).mp hf,hpc⟩
 · intro h
   exact ⟨h.locality,h.typed,flows_rank h.flow,h.control⟩

theorem run_typed (ops : FallibleFlow.Arithmetic) {G : K → Ty} {SG : J → Ty}
 (s : K → Value) (snap : J → Value) (typedState : ∀ k, (s k).ty = G k)
 (typedSnapshot : ∀ j, (snap j).ty = SG j) (target : K) (e : Expr (Ref K J))
 (typed : Typed (types G SG) e (G target)) :
 ∀ k, ((run ops s snap target e).1 k).ty = G k := by
 have envTyped : ∀ r, (environment s snap r).ty = types G SG r := by
   intro r; cases r with
   | inl k => exact typedState k
   | inr j => exact typedSnapshot j
 unfold run; split
 · exact typedState
 · rename_i v hv
   have tv := FallibleFlow.eval_typed typed ops _ envTyped v hv
   intro k; by_cases same : k = target
   · subst k; simpa [put] using tv
   · simpa [put,same] using typedState k

theorem run_noninterference (ops : FallibleFlow.Arithmetic)
 {L : K → Nat} {SL : J → Nat} {level : Nat} {s t : K → Value} {snap snap' : J → Value}
 (low : LowEq L level s t) (snapshotLow : LowEq SL level snap snap')
 (target : K) (e : Expr (Ref K J)) (flow : rank (classes L SL) e ≤ L target) :
 LowEq L level (run ops s snap target e).1 (run ops t snap' target e).1 ∧
 FallibleFlow.project L level (run ops s snap target e).2 =
 FallibleFlow.project L level (run ops t snap' target e).2 := by
 have both : LowEq (classes L SL) level (environment s snap) (environment t snap') := by
   intro r hr; cases r with
   | inl k => exact low k hr
   | inr j => exact snapshotLow j hr
 by_cases visible : L target ≤ level
 · have ev := FallibleFlow.eval_low ops both e (Nat.le_trans flow visible)
   simp only [run,ev]
   split
   · exact ⟨low,rfl⟩
   · constructor
     · intro k hk; simp only [put]; split
       · rfl
       · exact low k hk
     · rfl
 · constructor
   · intro k hk
     have different : k ≠ target := by intro eq; subst k; exact visible hk
     rw [run_frame _ _ _ _ _ _ different,run_frame _ _ _ _ _ _ different]
     exact low k hk
   · simp only [run]; split <;> split <;>
       simp [FallibleFlow.project,FallibleFlow.Outcome.key,visible]

-- Assignment to an imported predecessor's state is an ordinary target reference.
-- There is no example-name dispatch or new Core "parent" primitive.
structure Source (K J : Type) where
 origin : Nat
 target : K
 rhs : Expr (Ref K J)

inductive EdgeKind where | request | result deriving DecidableEq, Repr
structure Edge where
 source : Nat
 destination : Nat
 kind : EdgeKind
 deriving DecidableEq, Repr
def remoteEdges (origin destination : Nat) : List Edge :=
 [⟨origin,destination,.request⟩,⟨destination,origin,.result⟩]
structure Core (K J : Type) where
 origin : Nat
 owner : Nat
 target : K
 rhs : Expr (Ref K J)
 edges : List Edge

def lower (owner : K → Nat) (s : Source K J) : Core K J :=
 ⟨s.origin,owner s.target,s.target,s.rhs,
   if s.origin = owner s.target then [] else remoteEdges s.origin (owner s.target)⟩

-- This is a static generated-edge consequence, never fabricated runtime evidence.
inductive Elaborates (owner : K → Nat) (G : K → Ty) (SG : J → Ty)
 (L : K → Nat) (SL : J → Nat) (pc : Nat) : Source K J → Core K J → Prop where
 | same {s} : Admissible owner G SG L SL pc s.target s.rhs → s.origin = owner s.target →
   Elaborates owner G SG L SL pc s ⟨s.origin,owner s.target,s.target,s.rhs,[]⟩
 | different {s} : Admissible owner G SG L SL pc s.target s.rhs → s.origin ≠ owner s.target →
   Elaborates owner G SG L SL pc s
     ⟨s.origin,owner s.target,s.target,s.rhs,remoteEdges s.origin (owner s.target)⟩

def elaborate (owner : K → Nat) (G : K → Ty) (SG : J → Ty)
 (L : K → Nat) (SL : J → Nat) (pc : Nat) (s : Source K J) : Option (Core K J) :=
 if check owner G SG L SL pc s.target s.rhs then some (lower owner s) else none

omit [DecidableEq K] in
theorem elaborate_exact (owner : K → Nat) (G : K → Ty) (SG : J → Ty)
 (L : K → Nat) (SL : J → Nat) (pc : Nat) (s : Source K J) (c : Core K J) :
 elaborate owner G SG L SL pc s = some c ↔ Elaborates owner G SG L SL pc s c := by
 constructor
 · intro h
   unfold elaborate at h; split at h
   · rename_i hc
     cases h
     have accepted := (check_exact _ _ _ _ _ _ _ _).mp hc
     by_cases same : s.origin = owner s.target
     · simpa [lower,same] using Elaborates.same accepted same
     · simpa [lower,same] using Elaborates.different accepted same
   · contradiction
 · intro h; cases h with
   | same ha hs => simp [elaborate,(check_exact _ _ _ _ _ _ _ _).mpr ha,lower,hs]
   | different ha hs => simp [elaborate,(check_exact _ _ _ _ _ _ _ _).mpr ha,lower,hs]

omit [DecidableEq K] in
theorem generated_destination {owner : K → Nat} {G : K → Ty} {SG : J → Ty}
 {L : K → Nat} {SL : J → Nat} {pc : Nat} {s : Source K J} {c : Core K J}
 (h : Elaborates owner G SG L SL pc s c) :
 c.target = s.target ∧ c.owner = owner s.target ∧ c.rhs = s.rhs := by
 cases h <;> exact ⟨rfl,rfl,rfl⟩

namespace Controls
abbrev Key := Fin 3
abbrev Capture := Fin 1
def owner (k : Key) : Nat := if k = 2 then 20 else 10
def state (k : Key) : Value := .int (if k = 2 then 3 else 10)
def changed (k : Key) : Value := .int (if k = 2 then 99 else 10)
def snap (_ : Capture) : Value := .int 3
def formula : Expr (Ref Key Capture) := .add (.read (.inl 0)) (.read (.inr 0))
example : check (J := Capture) owner (fun _ => .int) (fun _ => .int) (fun _ => 1) (fun _ => 1) 0 0 formula = true := by decide
example : (run (FallibleFlow.signed 63) state snap 0 formula).1 0 = .int 13 := by decide
example : (run (FallibleFlow.signed 63) changed snap 0 formula).1 0 = .int 13 := by decide
-- Substituting a live remote read changes the meaning and is rejected.
example : localCheck owner 10 (.add (.read (.inl 0)) (.read (.inl 2)) : Expr (Ref Key Capture)) = false := by decide
-- Captured secret metadata must not be erased by turning its value into a literal.
example : check (J := Capture) owner (fun _ => .int) (fun _ => .int) (fun _ => 0) (fun _ => 1) 0 0
 (.read (.inr 0)) = false := by decide
example : check (J := Capture) owner (fun _ => .int) (fun _ => .int) (fun _ => 0) (fun _ => 1) 0 0
 (.lit (.int 3)) = true := by decide
def snapOther (_ : Capture) : Value := .int 4
def secretRead : Expr (Ref Key Capture) := .read (.inr 0)
-- Same source, low-equivalent inputs, but code produced by unsafe erasure varies.
example : LowEq (classes (fun (_ : Key) => 0) (fun (_ : Capture) => 1)) 0
 (environment state snap) (environment state snapOther) := by
 intro r hr; cases r with
 | inl k => rfl
 | inr j => simp [classes] at hr
example : check (J := Capture) owner (fun _ => .int) (fun _ => .int) (fun _ => 0) (fun _ => 1) 0 0
 (eraseFrozen snap secretRead) = true := by decide
example : check (J := Capture) owner (fun _ => .int) (fun _ => .int) (fun _ => 0) (fun _ => 1) 0 0
 (eraseFrozen snapOther secretRead) = true := by decide
example : (run (FallibleFlow.signed 63) state snap 0 (eraseFrozen snap secretRead)).1 0 ≠
 (run (FallibleFlow.signed 63) state snapOther 0 (eraseFrozen snapOther secretRead)).1 0 := by decide
def predecessorWrite : Source Key Capture := ⟨30,0,.add (.read (.inl 0)) (.lit (.int 3))⟩
example : (lower owner predecessorWrite).edges = [⟨30,10,.request⟩,⟨10,30,.result⟩] := by decide
example : (elaborate owner (fun _ => .int) (fun _ => .int) (fun _ => 1) (fun _ => 1)
 0 predecessorWrite).isSome = true := by decide
example : (run (FallibleFlow.signed 63) state snap predecessorWrite.target predecessorWrite.rhs).1 0 = .int 13 := by decide
end Controls

#print axioms local_exact
#print axioms erasure_value_only
#print axioms foreign_changes_irrelevant
#print axioms run_frame
#print axioms run_failure
#print axioms check_exact
#print axioms run_typed
#print axioms run_noninterference
#print axioms elaborate_exact
#print axioms generated_destination

end MirroreaProofFirst.OwnerAssignment
