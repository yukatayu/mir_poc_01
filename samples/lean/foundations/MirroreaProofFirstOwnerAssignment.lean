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

-- Later unreviewed capture extension; excluded from the frozen Oracle packet.
namespace MirroreaProofFirst.OwnerAssignment.Capture
open ProducerFlow
variable {I J K : Type}

-- Fixed capture expressions, not unlabelled deserialized argument strings.
-- These judgments do not authenticate metadata or choose a distributed cut.
structure Allowed (G : I → Ty) (L : I → Nat) (pc : Nat)
 (e : Expr I) (t : Ty) (label : Nat) : Prop where
 typed : Typed G e t
 flow : Flows L label e
 control : pc ≤ label

def check (G : I → Ty) (L : I → Nat) (pc : Nat)
 (e : Expr I) (t : Ty) (label : Nat) : Bool :=
 decide (infer G e = some t ∧ rank L e ≤ label ∧ pc ≤ label)

theorem check_exact (G : I → Ty) (L : I → Nat) (pc : Nat)
 (e : Expr I) (t : Ty) (label : Nat) :
 check G L pc e t label = true ↔ Allowed G L pc e t label := by
 simp only [check,decide_eq_true_eq,infer_exact,flows_exact]
 exact ⟨fun ⟨ht,hf,hp⟩ => ⟨ht,hf,hp⟩,fun h => ⟨h.typed,h.flow,h.control⟩⟩

-- Each slot uses its own read valuation; captures need not share a global cut.
-- Each captured slot records an actual successful evaluation. No claim of atomic
-- multi-slot capture, termination, presence confidentiality or authority is made.
def Captured (ops : FallibleFlow.Arithmetic) (env : J → I → Value)
 (source : J → Expr I) (values : J → Value) : Prop :=
 ∀ j, FallibleFlow.eval ops (env j) (source j) = some (values j)

theorem captured_typed (ops : FallibleFlow.Arithmetic)
 {G : I → Ty} {SG : J → Ty} {env : J → I → Value} {source : J → Expr I} {values : J → Value}
 (typedEnv : ∀ j i, (env j i).ty = G i)
 (typedSource : ∀ j, Typed G (source j) (SG j))
 (actual : Captured ops env source values) : ∀ j, (values j).ty = SG j := by
 intro j
 exact FallibleFlow.eval_typed (typedSource j) ops (env j) (typedEnv j) (values j) (actual j)

theorem captured_low (ops : FallibleFlow.Arithmetic)
 {L : I → Nat} {SL : J → Nat} {level : Nat} {env env' : J → I → Value}
 {source : J → Expr I} {values values' : J → Value}
 (low : ∀ j, LowEq L level (env j) (env' j))
 (flow : ∀ j, rank L (source j) ≤ SL j)
 (actual : Captured ops env source values) (actual' : Captured ops env' source values') :
 LowEq SL level values values' := by
 intro j hj
 have same := FallibleFlow.eval_low ops (low j) (source j) (Nat.le_trans (flow j) hj)
 rw [actual j,actual' j] at same
 exact Option.some.inj same

-- Compose source-side capture preservation with destination-side ordinary write.
-- Successful captures on both sides are explicit; observable capture failure and
-- secret-dependent selection require their own event/control boundary.
theorem capture_then_write_low [DecidableEq K] (ops : FallibleFlow.Arithmetic)
 {IL : I → Nat} {L : K → Nat} {SL : J → Nat} {level : Nat}
 {env env' : J → I → Value} {s t : K → Value} {source : J → Expr I}
 {values values' : J → Value}
 (inputLow : ∀ j, LowEq IL level (env j) (env' j)) (liveLow : LowEq L level s t)
 (captureFlow : ∀ j, rank IL (source j) ≤ SL j)
 (actual : Captured ops env source values) (actual' : Captured ops env' source values')
 (target : K) (e : Expr (Ref K J)) (writeFlow : rank (classes L SL) e ≤ L target) :
 LowEq L level (run ops s values target e).1 (run ops t values' target e).1 ∧
 FallibleFlow.project L level (run ops s values target e).2 =
 FallibleFlow.project L level (run ops t values' target e).2 :=
 run_noninterference ops liveLow (captured_low ops inputLow captureFlow actual actual')
 target e writeFlow

-- One explicit capture followed by a dependent operation. Failure is an actual
-- failed destination operation, not a success-conditioned omission from the trace.
def attempt [DecidableEq K] (ops : FallibleFlow.Arithmetic) (input : I → Value)
 (live : K → Value) (target : K) (source : Expr I) (body : Expr (Ref K Unit)) :
 (K → Value) × FallibleFlow.Outcome K :=
 match FallibleFlow.eval ops input source with
 | none => (live,.failed target)
 | some v => run ops live (fun _ => v) target body

theorem attempt_frame [DecidableEq K] (ops : FallibleFlow.Arithmetic)
 (input : I → Value) (live : K → Value) (target : K)
 (source : Expr I) (body : Expr (Ref K Unit)) (k : K) (hne : k ≠ target) :
 (attempt ops input live target source body).1 k = live k := by
 unfold attempt; split
 · rfl
 · exact run_frame _ _ _ _ _ _ hne

theorem attempt_key [DecidableEq K] (ops : FallibleFlow.Arithmetic)
 (input : I → Value) (live : K → Value) (target : K)
 (source : Expr I) (body : Expr (Ref K Unit)) :
 (attempt ops input live target source body).2.key = target := by
 unfold attempt; split
 · rfl
 · unfold run; split <;> rfl

structure SequenceAllowed (owner : K → Nat) (G : K → Ty) (IG : I → Ty)
 (L : K → Nat) (IL : I → Nat) (pc capturePC : Nat) (target : K)
 (source : Expr I) (body : Expr (Ref K Unit)) (captureTy : Ty) (label : Nat) : Prop where
 capture : Allowed IG IL capturePC source captureTy label
 body : Admissible owner G (fun (_ : Unit) => captureTy) L (fun _ => label) pc target body
 completion : label ≤ L target

def sequenceCheck (owner : K → Nat) (G : K → Ty) (IG : I → Ty)
 (L : K → Nat) (IL : I → Nat) (pc capturePC : Nat) (target : K)
 (source : Expr I) (body : Expr (Ref K Unit)) (captureTy : Ty) (label : Nat) : Bool :=
 check IG IL capturePC source captureTy label &&
 OwnerAssignment.check owner G (fun (_ : Unit) => captureTy) L (fun _ => label) pc target body &&
 decide (label ≤ L target)

theorem sequence_exact (owner : K → Nat) (G : K → Ty) (IG : I → Ty)
 (L : K → Nat) (IL : I → Nat) (pc capturePC : Nat) (target : K)
 (source : Expr I) (body : Expr (Ref K Unit)) (captureTy : Ty) (label : Nat) :
 sequenceCheck owner G IG L IL pc capturePC target source body captureTy label = true ↔
 SequenceAllowed owner G IG L IL pc capturePC target source body captureTy label := by
 simp only [sequenceCheck,Bool.and_eq_true,check_exact,OwnerAssignment.check_exact,decide_eq_true_eq]
 exact ⟨fun ⟨⟨hc,hb⟩,hf⟩ => ⟨hc,hb,hf⟩,fun h => ⟨⟨h.capture,h.body⟩,h.completion⟩⟩

theorem attempt_typed [DecidableEq K] (ops : FallibleFlow.Arithmetic)
 {G : K → Ty} {IG : I → Ty} {input : I → Value} {live : K → Value}
 (inputTyped : ∀ i, (input i).ty = IG i) (liveTyped : ∀ k, (live k).ty = G k)
 (target : K) (source : Expr I) (body : Expr (Ref K Unit)) (captureTy : Ty)
 (sourceTyped : Typed IG source captureTy)
 (bodyTyped : Typed (types G (fun (_ : Unit) => captureTy)) body (G target)) :
 ∀ k, ((attempt ops input live target source body).1 k).ty = G k := by
 unfold attempt; split
 · exact liveTyped
 · rename_i v hv
   exact run_typed ops live (fun _ => v) liveTyped
     (fun _ => FallibleFlow.eval_typed sourceTyped ops input inputTyped v hv) target body bodyTyped

theorem attempt_low [DecidableEq K] (ops : FallibleFlow.Arithmetic)
 {IL : I → Nat} {L : K → Nat} {label level : Nat}
 {input input' : I → Value} {live live' : K → Value}
 (inputLow : LowEq IL level input input') (liveLow : LowEq L level live live')
 (target : K) (source : Expr I) (body : Expr (Ref K Unit))
 (captureFlow : rank IL source ≤ label) (completionFlow : label ≤ L target)
 (bodyFlow : rank (classes L (fun (_ : Unit) => label)) body ≤ L target) :
 LowEq L level (attempt ops input live target source body).1
   (attempt ops input' live' target source body).1 ∧
 FallibleFlow.project L level (attempt ops input live target source body).2 =
 FallibleFlow.project L level (attempt ops input' live' target source body).2 := by
 by_cases visible : L target ≤ level
 · have ev := FallibleFlow.eval_low ops inputLow source
     (Nat.le_trans captureFlow (Nat.le_trans completionFlow visible))
   simp only [attempt,ev]
   split
   · exact ⟨liveLow,rfl⟩
   · exact run_noninterference ops liveLow (fun _ _ => rfl) target body bodyFlow
 · constructor
   · intro k hk
     have different : k ≠ target := by intro eq; subst k; exact visible hk
     rw [attempt_frame _ _ _ _ _ _ _ different,attempt_frame _ _ _ _ _ _ _ different]
     exact liveLow k hk
   · simp [FallibleFlow.project,attempt_key,visible]

namespace FailureControl
abbrev Key := Fin 1
def input (v : Int) (_ : Key) : Value := .int v
def live (_ : Key) : Value := .int 10
def source : Expr Key := .add (.read 0) (.lit (.int 1))
def body : Expr (Ref Key Unit) := .lit (.int 11)
-- Secret-dependent overflow controls whether even an unused capture allows a
-- public write. Checking only body rank misses this completion dependency.
example : LowEq (fun (_ : Key) => 1) 0 (input 9223372036854775806) (input 9223372036854775807) := by
 intro k h; contradiction
example : rank (classes (fun (_ : Key) => 0) (fun (_ : Unit) => 1)) body = 0 := by decide
example : (attempt (FallibleFlow.signed 63) (input 9223372036854775806) live 0 source body).2 =
 .wrote 0 (.int 11) := by decide
example : (attempt (FallibleFlow.signed 63) (input 9223372036854775807) live 0 source body).2 =
 .failed 0 := by decide
example : FallibleFlow.project (fun (_ : Key) => 0) 0
 (attempt (FallibleFlow.signed 63) (input 9223372036854775806) live 0 source body).2 ≠
 FallibleFlow.project (fun (_ : Key) => 0) 0
 (attempt (FallibleFlow.signed 63) (input 9223372036854775807) live 0 source body).2 := by decide
example : sequenceCheck (fun (_ : Key) => 10) (fun _ => .int) (fun _ => .int)
 (fun _ => 0) (fun _ => 1) 0 0 0 source body .int 1 = false := by decide
example : sequenceCheck (fun (_ : Key) => 10) (fun _ => .int) (fun _ => .int)
 (fun _ => 1) (fun _ => 1) 0 0 0 source body .int 1 = true := by decide
end FailureControl

-- Control cannot be discarded even for a constant captured expression.
example : check (fun (_ : Fin 1) => .int) (fun _ => 0) 1 (.lit (.int 3)) .int 0 = false := by decide
example : check (fun (_ : Fin 1) => .int) (fun _ => 0) 1 (.lit (.int 3)) .int 1 = true := by decide
example : check (fun (_ : Fin 1) => .int) (fun _ => 1) 0 (.read 0) .int 0 = false := by decide
example : check (fun (_ : Fin 1) => .int) (fun _ => 1) 0 (.read 0) .int 1 = true := by decide

#print axioms sequence_exact
#print axioms attempt_typed
#print axioms attempt_frame
#print axioms attempt_low
#print axioms check_exact
#print axioms captured_typed
#print axioms captured_low
#print axioms capture_then_write_low
end MirroreaProofFirst.OwnerAssignment.Capture
