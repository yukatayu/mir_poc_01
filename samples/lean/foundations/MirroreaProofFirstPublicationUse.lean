import MirroreaProofFirstPublicationPayload

namespace MirroreaProofFirst.PublicationUse

-- Explicit enter/finish interval for one protected local critical section per
-- endpoint. Freeze and install wait until it ends. This is a lock/queue model,
-- not a lease handed to callers, and gives no authorization by itself.
-- Returned futures/permits must re-enter; no detached continuation transition.
structure State (n : Nat) (Value Command : Type) where
  base : PublicationPayload.State n Value Command
  held : Fin n → Option (Nat × Value)

def initial (n revision : Nat) (value : Value) : State n Value Command :=
  ⟨PublicationPayload.initial n revision value,fun _ => none⟩

inductive Action (n : Nat) (Command : Type) where
  | administrative (action : PublicationPayload.Action n Command)
  | enter (i : Fin n)
  | finish (i : Fin n)

def FreeFor (s : State n Value Command) : PublicationPayload.Action n Command → Prop
  | .freeze i _ | .install i _ => s.held i = none
  | _ => True

def freeCheck (s : State n Value Command) : PublicationPayload.Action n Command → Bool
  | .freeze i _ | .install i _ => (s.held i).isNone
  | _ => true

def Allowed (evaluate : Value → Command → Option Value) (s : State n Value Command) : Action n Command → Prop
  | .administrative action => PublicationPayload.Allowed evaluate s.base action ∧ FreeFor s action
  | .enter i => PublicationPayload.Allowed evaluate s.base (.use i) ∧ s.held i = none
  | .finish i => ∃ pair, s.held i = some pair

def check (evaluate : Value → Command → Option Value) (s : State n Value Command) : Action n Command → Bool
  | .administrative action => PublicationPayload.check evaluate s.base action && freeCheck s action
  | .enter i => PublicationPayload.check evaluate s.base (.use i) && (s.held i).isNone
  | .finish i => (s.held i).isSome

theorem check_exact (evaluate : Value → Command → Option Value) (s : State n Value Command)
    (action : Action n Command) : check evaluate s action = true ↔ Allowed evaluate s action := by
  cases action with
  | administrative action =>
      cases action <;> simp [check,Allowed,freeCheck,FreeFor,PublicationPayload.check_exact]
  | enter i => simp [check,Allowed,PublicationPayload.check_exact]
  | finish i => simp [check,Allowed,Option.isSome_iff_exists]

def apply (evaluate : Value → Command → Option Value) (s : State n Value Command) : Action n Command → State n Value Command
  | .administrative action => {s with base := PublicationPayload.apply evaluate s.base action}
  | .enter i => {s with held := PublicationPayload.put s.held i (some (s.base.barrier.installed i,s.base.cached i))}
  | .finish i => {s with held := PublicationPayload.put s.held i none}

def execute (evaluate : Value → Command → Option Value) (s : State n Value Command)
    (action : Action n Command) : Option (State n Value Command) :=
  if check evaluate s action then some (apply evaluate s action) else none

theorem execute_exact (evaluate : Value → Command → Option Value) (s next : State n Value Command)
    (action : Action n Command) :
    execute evaluate s action = some next ↔ Allowed evaluate s action ∧ next = apply evaluate s action := by
  unfold execute
  split
  · rename_i accepted
    constructor
    · intro equal
      exact ⟨(check_exact _ _ _).mp accepted,(Option.some.inj equal).symm⟩
    · intro pair
      exact congrArg some pair.2.symm
  · rename_i denied
    constructor
    · intro impossible; cases impossible
    · intro pair
      exact False.elim (denied ((check_exact _ _ _).mpr pair.1))

structure Invariant (evaluate : Value → Command → Option Value) (s : State n Value Command) : Prop where
  payload : PublicationPayload.Invariant evaluate s.base
  active : ∀ i pair, s.held i = some pair →
    pair.1 = s.base.barrier.installed i ∧ pair.2 = s.base.cached i ∧
      s.base.barrier.installed i = s.base.barrier.fence i

theorem initial_invariant (evaluate : Value → Command → Option Value) (n revision : Nat) (value : Value) :
    Invariant evaluate (initial n revision value) := by
  exact ⟨PublicationPayload.initial_invariant _ _ _ _,fun _ _ impossible => by cases impossible⟩

theorem publication_has_no_active (valid : Invariant evaluate s)
    (allowed : PublicationPayload.Allowed evaluate s.base .publish) : ∀ i, s.held i = none := by
  intro i
  cases held : s.held i with
  | none => rfl
  | some pair =>
      have active := valid.active i pair held
      have installed := (valid.payload.numeric.2.1 i).1
      have ack := (valid.payload.numeric.2.1 i).2.2
      have allAck := allowed.1.2 i
      have newer := allowed.1.1
      have openGate := active.2.2
      omega

theorem apply_preserves (evaluate : Value → Command → Option Value) (s : State n Value Command)
    (action : Action n Command) (valid : Invariant evaluate s) (allowed : Allowed evaluate s action) :
    Invariant evaluate (apply evaluate s action) := by
  cases action with
  | administrative action =>
      refine ⟨PublicationPayload.apply_preserves _ _ _ valid.payload allowed.1,?_⟩
      intro j pair held
      change s.held j = some pair at held
      have prior := valid.active j pair held
      cases action with
      | stage command => exact prior
      | freeze i r =>
          by_cases equal : j = i
          · subst j
            have free : s.held i = none := allowed.2
            rw [free] at held
            cases held
          · simpa [apply,PublicationPayload.apply,PublicationPayload.projected,Publication.apply,Publication.put,equal] using prior
      | acknowledge i r => exact prior
      | publish =>
          have free := publication_has_no_active valid allowed.1 j
          rw [free] at held
          cases held
      | install i r =>
          by_cases equal : j = i
          · subst j
            have free : s.held i = none := allowed.2
            rw [free] at held
            cases held
          · simpa [apply,PublicationPayload.apply,PublicationPayload.projected,Publication.apply,
              Publication.put,PublicationPayload.put,equal] using prior
      | use i => exact prior
  | enter i =>
      refine ⟨valid.payload,?_⟩
      intro j pair held
      by_cases equal : j = i
      · subst j
        simp only [apply,PublicationPayload.put,ite_true,Option.some.injEq] at held
        cases held
        exact ⟨rfl,rfl,allowed.1.1⟩
      · exact valid.active j pair (by simpa [apply,PublicationPayload.put,equal] using held)
  | finish i =>
      refine ⟨valid.payload,?_⟩
      intro j pair held
      by_cases equal : j = i
      · subst j
        simp [apply,PublicationPayload.put] at held
      · exact valid.active j pair (by simpa [apply,PublicationPayload.put,equal] using held)

inductive Step (evaluate : Value → Command → Option Value) : State n Value Command → State n Value Command → Prop where
  | action : Allowed evaluate s action → Step evaluate s (apply evaluate s action)

inductive Reached (evaluate : Value → Command → Option Value) (n revision : Nat) (value : Value) :
    State n Value Command → Prop where
  | initial : Reached evaluate n revision value (PublicationUse.initial n revision value)
  | step : Reached evaluate n revision value s → Step evaluate s next → Reached evaluate n revision value next

theorem reached_invariant (path : Reached evaluate n revision value s) : Invariant evaluate s := by
  induction path with
  | initial => exact initial_invariant _ _ _ _
  | step previous step ih =>
      cases step with
      | action allowed => exact apply_preserves _ _ _ ih allowed

theorem reached_payload (path : Reached evaluate n revision value s) :
    PublicationPayload.Reached evaluate n revision value s.base := by
  induction path with
  | initial => exact .initial
  | step previous step ih =>
      cases step with
      | @action action allowed =>
          cases action with
          | administrative action => exact .step ih (.action allowed.1)
          | enter i => exact ih
          | finish i => exact ih

theorem held_value_current {evaluate : Value → Command → Option Value} {s : State n Value Command}
    (path : Reached evaluate n revision value s) (i : Fin n) (pair : Nat × Value)
    (held : s.held i = some pair) : pair.1 = s.base.barrier.published ∧ pair.2 = s.base.current := by
  have valid := reached_invariant path
  have active := valid.active i pair held
  have bounds := valid.payload.numeric.2.1 i
  have openGate := active.2.2
  have current : s.base.barrier.installed i = s.base.barrier.published := by omega
  have image := valid.payload.cachedImage i
  rw [current,valid.payload.currentImage] at image
  exact ⟨active.1.trans current,active.2.1.trans (Option.some.inj image).symm⟩

theorem freeze_waits_for_active (s : State n Value Command) (i : Fin n) (pair : Nat × Value)
    (held : s.held i = some pair) (revision : Nat) :
    execute evaluate s (.administrative (.freeze i revision)) = none := by
  simp [execute,check,freeCheck,held]

#print axioms check_exact
#print axioms execute_exact
#print axioms apply_preserves
#print axioms publication_has_no_active
#print axioms reached_invariant
#print axioms reached_payload
#print axioms held_value_current
#print axioms freeze_waits_for_active
end MirroreaProofFirst.PublicationUse
