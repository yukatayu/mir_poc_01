import MirroreaProofFirstPublicationUse

namespace MirroreaProofFirst.CoordinatorUse

-- Bounded supervised coordinator protocol. Nat revision, semantic generation,
-- source activation/ordinal, and this grant serial remain distinct. The model
-- does not authenticate a control channel or execute an M9 issuer/backend.
inductive Kind where
 | issue | admit | resolve | body | report | acknowledge
 deriving DecidableEq, Repr

structure Grant (n : Nat) where
 endpoint : Fin n
 serial : Nat
 kind : Kind
 deriving DecidableEq, Repr

structure State (n : Nat) (Value Command : Type) where
 useState : PublicationUse.State n Value Command
 closed : Bool
 prepared : Fin n → Option (Nat × Value)
 active : Option (Grant n)
 nextSerial : Nat

def initial (n revision : Nat) (value : Value) : State n Value Command :=
 ⟨PublicationUse.initial n revision value,false,fun _ => none,none,0⟩

inductive Action (n : Nat) (Value Command : Type) where
 | close
 | stage (command : Command)
 | freeze (endpoint : Fin n) (revision : Nat)
 | preparedAck (endpoint : Fin n) (revision : Nat) (value : Value)
 | publish
 | activate (endpoint : Fin n) (revision : Nat)
 | reopen
 | acquire (endpoint : Fin n) (kind : Kind)
 | finish (grant : Grant n)

def projected : Action n Value Command → Option (PublicationUse.Action n Command)
 | .close | .reopen => none
 | .stage command => some (.administrative (.stage command))
 | .freeze i r => some (.administrative (.freeze i r))
 | .preparedAck i r _ => some (.administrative (.acknowledge i r))
 | .publish => some (.administrative .publish)
 | .activate i r => some (.administrative (.install i r))
 | .acquire i _ => some (.enter i)
 | .finish grant => some (.finish grant.endpoint)

-- Preparing/refeshing an actual disabled backend is a physical premise of
-- preparedAck. Here its exact staged value/revision is checked; it never enables
-- the endpoint. Authentic endpoint identity is a separate physical premise of
-- both preparedAck and finish, not a property of their serial numbers.
def Extra (s : State n Value Command) : Action n Value Command → Prop
 | .close => True
 | .stage _ => s.closed = true ∧ s.active = none
 | .freeze _ r => s.closed = true ∧ s.active = none ∧
     r = s.useState.base.barrier.announced ∧ s.useState.base.barrier.published < r
 | .preparedAck i r value => s.closed = true ∧ s.active = none ∧
     r = s.useState.base.barrier.announced ∧ s.useState.base.barrier.published < r ∧
     s.useState.base.barrier.fence i = r ∧
     ∃ command, s.useState.base.prepared = some (value,command)
 | .publish => s.closed = true ∧ s.active = none ∧
     ∀ i, ∃ value, s.prepared i = some (s.useState.base.barrier.announced,value)
 | .activate i r => s.closed = true ∧ s.active = none ∧
     r = s.useState.base.barrier.published ∧ s.prepared i = some (r,s.useState.base.current)
 | .reopen => s.closed = true ∧ s.active = none ∧ s.useState.base.prepared = none ∧
     s.useState.base.barrier.announced = s.useState.base.barrier.published ∧
     ∀ i, s.useState.base.barrier.installed i = s.useState.base.barrier.published ∧
       s.useState.base.barrier.fence i = s.useState.base.barrier.published
 | .acquire _ _ => s.closed = false ∧ s.active = none
 | .finish grant => s.active = some grant

def Allowed (evaluate : Value → Command → Option Value) (s : State n Value Command)
 (action : Action n Value Command) : Prop :=
 (match projected action with | none => True | some a => PublicationUse.Allowed evaluate s.useState a) ∧ Extra s action

def extraCheck [DecidableEq Value] (s : State n Value Command) : Action n Value Command → Bool
 | .close => true
 | .stage _ => s.closed && s.active.isNone
 | .freeze _ r => s.closed && s.active.isNone &&
     decide (r = s.useState.base.barrier.announced ∧ s.useState.base.barrier.published < r)
 | .preparedAck i r value => s.closed && s.active.isNone &&
     decide (r = s.useState.base.barrier.announced ∧ s.useState.base.barrier.published < r ∧
       s.useState.base.barrier.fence i = r) &&
     (s.useState.base.prepared.map Prod.fst == some value)
 | .publish => s.closed && s.active.isNone &&
     (List.finRange n).all (fun i => (s.prepared i).map Prod.fst == some s.useState.base.barrier.announced)
 | .activate i r => s.closed && s.active.isNone &&
     decide (r = s.useState.base.barrier.published ∧ s.prepared i = some (r,s.useState.base.current))
 | .reopen => s.closed && s.active.isNone && s.useState.base.prepared.isNone &&
     decide (s.useState.base.barrier.announced = s.useState.base.barrier.published) &&
     (List.finRange n).all (fun i => decide (s.useState.base.barrier.installed i = s.useState.base.barrier.published ∧
       s.useState.base.barrier.fence i = s.useState.base.barrier.published))
 | .acquire _ _ => !s.closed && s.active.isNone
 | .finish grant => s.active == some grant

def check [DecidableEq Value] (evaluate : Value → Command → Option Value) (s : State n Value Command)
 (action : Action n Value Command) : Bool :=
 (match projected action with | none => true | some a => PublicationUse.check evaluate s.useState a) && extraCheck s action

private theorem map_first (x : Option (A × B)) (a : A) :
 x.map Prod.fst = some a ↔ ∃ b, x = some (a,b) := by
 cases x with
 | none => simp
 | some pair => rcases pair with ⟨first,second⟩; simp [eq_comm]

variable {n : Nat} {Value Command : Type} {s next : State n Value Command}
 {evaluate : Value → Command → Option Value} {action : Action n Value Command}
 {grant : Grant n} {i : Fin n} {kind : Kind} {command : Command} {r : Nat} {value : Value}

theorem check_exact [DecidableEq Value] : check evaluate s action = true ↔ Allowed evaluate s action := by
 cases action <;>
   simp [check,Allowed,projected,extraCheck,Extra,PublicationUse.check_exact,
     List.all_eq_true,and_assoc]

def apply (evaluate : Value → Command → Option Value) (s : State n Value Command)
 (action : Action n Value Command) : State n Value Command :=
 let next := {s with useState := match projected action with
   | none => s.useState
   | some a => PublicationUse.apply evaluate s.useState a}
 match action with
 | .close => {next with closed := true}
 | .stage _ => {next with prepared := fun _ => none}
 | .preparedAck i r value => {next with prepared := PublicationPayload.put s.prepared i (some (r,value))}
 | .reopen => {next with closed := false}
 | .acquire i kind => {next with active := some ⟨i,s.nextSerial,kind⟩,nextSerial := s.nextSerial+1}
 | .finish _ => {next with active := none}
 | _ => next

def execute [DecidableEq Value] (evaluate : Value → Command → Option Value) (s : State n Value Command)
 (action : Action n Value Command) : Option (State n Value Command) :=
 if check evaluate s action then some (apply evaluate s action) else none

theorem execute_exact [DecidableEq Value] : execute evaluate s action = some next ↔
 Allowed evaluate s action ∧ next = apply evaluate s action := by
 unfold execute
 split
 · rename_i yes
   simp only [Option.some.injEq]
   exact ⟨fun h => ⟨check_exact.mp yes,h.symm⟩,fun h => h.2.symm⟩
 · rename_i no
   simp only [reduceCtorEq,false_iff,not_and]
   exact fun allowed => False.elim (no (check_exact.mpr allowed))

theorem apply_projection : (apply evaluate s action).useState =
 match projected action with | none => s.useState | some a => PublicationUse.apply evaluate s.useState a := by
 cases action <;> rfl

inductive Step (evaluate : Value → Command → Option Value) : State n Value Command → State n Value Command → Prop where
 | action {s : State n Value Command} {action : Action n Value Command} :
     Allowed evaluate s action → Step evaluate s (apply evaluate s action)

inductive Reached (evaluate : Value → Command → Option Value) (n revision : Nat) (value : Value) :
 State n Value Command → Prop where
 | initial : Reached evaluate n revision value (CoordinatorUse.initial n revision value)
 | step {s next : State n Value Command} : Reached evaluate n revision value s →
     Step evaluate s next → Reached evaluate n revision value next

theorem reached_use (path : Reached evaluate n revision value s) :
 PublicationUse.Reached evaluate n revision value s.useState := by
 induction path with
 | initial => exact .initial
 | step _ step ih =>
   cases step with
   | @action action allowed =>
     rw [apply_projection]
     cases projection : projected action with
     | none => exact ih
     | some a => exact .step ih (.action (by simpa [Allowed,projection] using allowed.1))

def ActiveHeld (s : State n Value Command) : Prop :=
 ∀ grant, s.active = some grant → ∃ pair, s.useState.held grant.endpoint = some pair

theorem apply_active (valid : ActiveHeld s) (allowed : Allowed evaluate s action) :
 ActiveHeld (apply evaluate s action) := by
 cases action with
 | close => exact valid
 | stage command => exact valid
 | freeze i r => exact valid
 | preparedAck i r value => exact valid
 | publish => exact valid
 | activate i r => exact valid
 | reopen => exact valid
 | acquire i kind =>
   intro grant found
   have same : (⟨i,s.nextSerial,kind⟩ : Grant _) = grant := Option.some.inj found
   subst grant
   exact ⟨(s.useState.base.barrier.installed i,s.useState.base.cached i),
     by simp [apply,projected,PublicationUse.apply,PublicationPayload.put]⟩
 | finish grant => intro other impossible; cases impossible

theorem reached_active (path : Reached evaluate n revision value s) : ActiveHeld s := by
 induction path with
 | initial => intro grant impossible; cases impossible
 | step _ step ih => cases step with | action allowed => exact apply_active ih allowed

theorem active_value_current (path : Reached evaluate n revision value s)
 (active : s.active = some grant) : ∃ pair, s.useState.held grant.endpoint = some pair ∧
 pair.1 = s.useState.base.barrier.published ∧ pair.2 = s.useState.base.current := by
 obtain ⟨pair,held⟩ := reached_active path grant active
 exact ⟨pair,held,PublicationUse.held_value_current (reached_use path) grant.endpoint pair held⟩

theorem wrong_finish_rejected [DecidableEq Value] (wrong : s.active ≠ some grant) :
 execute evaluate s (.finish grant) = none := by
 simp [execute,check,extraCheck,wrong]

theorem closed_acquire_rejected [DecidableEq Value] (closed : s.closed = true) :
 execute evaluate s (.acquire i kind) = none := by simp [execute,check,extraCheck,closed]

theorem active_stage_rejected [DecidableEq Value] (active : s.active = some grant) :
 execute evaluate s (.stage command) = none := by simp [execute,check,extraCheck,active]

theorem active_publish_rejected [DecidableEq Value] (active : s.active = some grant) :
 execute evaluate s .publish = none := by simp [execute,check,extraCheck,active]

theorem preparation_not_activation :
 (apply evaluate s (.preparedAck i r value)).useState.base.barrier.installed = s.useState.base.barrier.installed ∧
 (apply evaluate s (.preparedAck i r value)).useState.base.cached = s.useState.base.cached ∧
 (apply evaluate s (.preparedAck i r value)).useState.base.current = s.useState.base.current := ⟨rfl,rfl,rfl⟩

theorem activate_published (allowed : Allowed evaluate s (.activate i r)) :
 r = s.useState.base.barrier.published ∧ s.prepared i = some (r,s.useState.base.current) := allowed.2.2.2

theorem finish_exact_identity (allowed : Allowed evaluate s (.finish grant)) :
 s.active = some grant := allowed.2

theorem serial_monotone : s.nextSerial ≤ (apply evaluate s action).nextSerial := by
 cases action <;> simp [apply,projected]

theorem acquire_fresh_serial : (apply evaluate s (.acquire i kind)).active = some ⟨i,s.nextSerial,kind⟩ ∧
 (apply evaluate s (.acquire i kind)).nextSerial = s.nextSerial+1 := ⟨rfl,rfl⟩

def SerialFresh (s : State n Value Command) : Prop :=
 ∀ grant, s.active = some grant → grant.serial < s.nextSerial

theorem apply_serial (valid : SerialFresh s) : SerialFresh (apply evaluate s action) := by
 cases action with
 | close => exact valid
 | stage command => exact valid
 | freeze i r => exact valid
 | preparedAck i r value => exact valid
 | publish => exact valid
 | activate i r => exact valid
 | reopen => exact valid
 | acquire i kind =>
   intro grant found
   have same : (⟨i,s.nextSerial,kind⟩ : Grant _) = grant := Option.some.inj found
   subst grant
   exact Nat.lt_succ_self _
 | finish grant => intro other impossible; cases impossible

theorem reached_serial (path : Reached evaluate n revision value s) : SerialFresh s := by
 induction path with
 | initial => intro grant impossible; cases impossible
 | step _ step ih => cases step with | action allowed => exact apply_serial ih

theorem old_finish_after_new_acquire [DecidableEq Value] (old : grant.serial < s.nextSerial) :
 execute evaluate (apply evaluate s (.acquire i kind)) (.finish grant) = none := by
 apply wrong_finish_rejected
 intro same
 have serial := congrArg Grant.serial (Option.some.inj same)
 change s.nextSerial = grant.serial at serial
 omega

#print axioms check_exact
#print axioms execute_exact
#print axioms reached_use
#print axioms reached_active
#print axioms active_value_current
#print axioms preparation_not_activation
#print axioms activate_published
#print axioms finish_exact_identity
#print axioms wrong_finish_rejected
#print axioms active_publish_rejected
#print axioms serial_monotone
#print axioms reached_serial
#print axioms old_finish_after_new_acquire
end MirroreaProofFirst.CoordinatorUse
