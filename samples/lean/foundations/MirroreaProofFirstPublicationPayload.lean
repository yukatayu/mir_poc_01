import MirroreaProofFirstPublication

namespace MirroreaProofFirst.PublicationPayload

-- Nonproduction refinement of the numeric fence component to whole values.
-- Protocol revision is NOT authority generation. Values may include both an
-- authority view and a configuration without identifying their transitions.
-- `images` is the retained authentic publication-message store of this model,
-- NOT a runtime oracle available to receivers. Encoding, authenticated delivery,
-- realm/cohort binding, resource bounds and process serialization are still owed.
structure State (n : Nat) (Value Command : Type) where
  barrier : Publication.State n
  current : Value
  prepared : Option (Value × Command)
  images : Nat → Option Value
  cached : Fin n → Value

def write (values : Nat → Option Value) (revision : Nat) (value : Value) : Nat → Option Value :=
  fun r => if r = revision then some value else values r

def put (values : Fin n → Value) (i : Fin n) (value : Value) : Fin n → Value :=
  fun j => if j = i then value else values j

def initial (n revision : Nat) (value : Value) : State n Value Command :=
  ⟨Publication.initial n revision,value,none,
    fun r => if r = revision then some value else none,fun _ => value⟩

inductive Action (n : Nat) (Command : Type) where
  | stage (command : Command)
  | freeze (i : Fin n) (revision : Nat)
  | acknowledge (i : Fin n) (revision : Nat)
  | publish
  | install (i : Fin n) (revision : Nat)
  | use (i : Fin n)

def projected : Action n Command → Publication.Action n
  | .stage _ => .begin
  | .freeze i r => .freeze i r
  | .acknowledge i r => .acknowledge i r
  | .publish => .publish
  | .install i r => .install i r
  | .use i => .use i

def Extra (evaluate : Value → Command → Option Value) (s : State n Value Command) :
    Action n Command → Prop
  | .stage command => ∃ value, evaluate s.current command = some value
  | .publish => ∃ pair, s.prepared = some pair
  | .install _ revision => ∃ value, s.images revision = some value
  | _ => True

def extraCheck (evaluate : Value → Command → Option Value) (s : State n Value Command) :
    Action n Command → Bool
  | .stage command => (evaluate s.current command).isSome
  | .publish => s.prepared.isSome
  | .install _ revision => (s.images revision).isSome
  | _ => true

def Allowed (evaluate : Value → Command → Option Value) (s : State n Value Command)
    (action : Action n Command) : Prop :=
  Publication.Allowed s.barrier (projected action) ∧ Extra evaluate s action

def check (evaluate : Value → Command → Option Value) (s : State n Value Command)
    (action : Action n Command) : Bool :=
  Publication.check s.barrier (projected action) && extraCheck evaluate s action

theorem check_exact (evaluate : Value → Command → Option Value) (s : State n Value Command)
    (action : Action n Command) : check evaluate s action = true ↔ Allowed evaluate s action := by
  cases action <;> simp [check,Allowed,Extra,extraCheck,Publication.check_exact,Option.isSome_iff_exists]

def apply (evaluate : Value → Command → Option Value) (s : State n Value Command)
    (action : Action n Command) : State n Value Command :=
  let updated := {s with barrier := Publication.apply s.barrier (projected action)}
  match action with
  | .stage command => {updated with prepared := (evaluate s.current command).map (fun value => (value,command))}
  | .publish =>
      let value := (s.prepared.map Prod.fst).getD s.current
      {updated with current := value,prepared := none,images := write s.images s.barrier.announced value}
  | .install i revision => {updated with cached := put s.cached i ((s.images revision).getD (s.cached i))}
  | _ => updated

theorem apply_projection (evaluate : Value → Command → Option Value) (s : State n Value Command)
    (action : Action n Command) :
    (apply evaluate s action).barrier = Publication.apply s.barrier (projected action) := by
  cases action <;> rfl

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

-- Invariant is proved from actual transitions; no constructor carries a proof
-- of it and no action tests it. In particular payload agreement is not assumed
-- when admitting a publication or endpoint use.
structure Invariant (evaluate : Value → Command → Option Value) (s : State n Value Command) : Prop where
  numeric : Publication.Invariant s.barrier
  currentImage : s.images s.barrier.published = some s.current
  cachedImage : ∀ i, s.images (s.barrier.installed i) = some (s.cached i)
  preparedFromCurrent : ∀ pair, s.prepared = some pair → evaluate s.current pair.2 = some pair.1

theorem initial_invariant (evaluate : Value → Command → Option Value) (n revision : Nat) (value : Value) :
    Invariant evaluate (initial n revision value) := by
  refine ⟨Publication.initial_invariant _ _,?_,?_,?_⟩
  · simp [initial,Publication.initial]
  · intro i; simp [initial,Publication.initial]
  · intro pair impossible; cases impossible

theorem apply_preserves (evaluate : Value → Command → Option Value) (s : State n Value Command)
    (action : Action n Command) (valid : Invariant evaluate s) (allowed : Allowed evaluate s action) :
    Invariant evaluate (apply evaluate s action) := by
  have numeric : Publication.Invariant (apply evaluate s action).barrier := by
    rw [apply_projection]
    exact Publication.apply_preserves _ _ valid.numeric allowed.1
  refine ⟨numeric,?_,?_,?_⟩
  · cases action with
    | stage command => exact valid.currentImage
    | freeze i r => exact valid.currentImage
    | acknowledge i r => exact valid.currentImage
    | publish => simp [apply,projected,Publication.apply,write]
    | install i r => exact valid.currentImage
    | use i => exact valid.currentImage
  · cases action with
    | stage command => exact valid.cachedImage
    | freeze i r => exact valid.cachedImage
    | acknowledge i r => exact valid.cachedImage
    | publish =>
        intro i
        have newer : s.barrier.published < s.barrier.announced := allowed.1.1
        have older := (valid.numeric.2.1 i).1
        have different : s.barrier.installed i ≠ s.barrier.announced := by omega
        simpa [apply,projected,Publication.apply,write,different] using valid.cachedImage i
    | install i r =>
        obtain ⟨value,found⟩ := allowed.2
        intro j
        by_cases same : j = i
        · subst j
          simp [apply,projected,Publication.apply,Publication.put,put,found]
        · simpa [apply,projected,Publication.apply,Publication.put,put,same] using valid.cachedImage j
    | use i => exact valid.cachedImage
  · cases action with
    | stage command =>
        intro pair prepared
        obtain ⟨value,evaluated⟩ := allowed.2
        simp only [apply,evaluated,Option.map_some,Option.some.injEq] at prepared
        cases prepared
        exact evaluated
    | freeze i r => exact valid.preparedFromCurrent
    | acknowledge i r => exact valid.preparedFromCurrent
    | publish => intro pair impossible; cases impossible
    | install i r => exact valid.preparedFromCurrent
    | use i => exact valid.preparedFromCurrent

inductive Step (evaluate : Value → Command → Option Value) : State n Value Command → State n Value Command → Prop where
  | action : Allowed evaluate s action → Step evaluate s (apply evaluate s action)

inductive Reached (evaluate : Value → Command → Option Value) (n revision : Nat) (value : Value) :
    State n Value Command → Prop where
  | initial : Reached evaluate n revision value (PublicationPayload.initial n revision value)
  | step : Reached evaluate n revision value s → Step evaluate s next → Reached evaluate n revision value next

theorem reached_invariant (path : Reached evaluate n revision value s) : Invariant evaluate s := by
  induction path with
  | initial => exact initial_invariant _ _ _ _
  | step previous step ih =>
      cases step with
      | action allowed => exact apply_preserves _ _ _ ih allowed

theorem enabled_payload_current (path : Reached evaluate n revision value s) (i : Fin n)
    (enabled : check evaluate s (.use i) = true) : s.cached i = s.current := by
  have valid := reached_invariant path
  have openGate : s.barrier.installed i = s.barrier.fence i := ((check_exact _ _ _).mp enabled).1
  have bounds := valid.numeric.2.1 i
  have equal : s.barrier.installed i = s.barrier.published := by omega
  have image := valid.cachedImage i
  rw [equal,valid.currentImage] at image
  exact (Option.some.inj image).symm

-- Relative refinement has an explicit evaluator-correctness premise. It is
-- discharged for concrete W3 Session entries in a separate consumer, not an axiom.
theorem action_refines (relation : Value → Value → Prop)
    (sound : ∀ old command next, evaluate old command = some next → relation old next)
    (valid : Invariant evaluate s) (allowed : Allowed evaluate s action) :
    (apply evaluate s action).current = s.current ∨ relation s.current (apply evaluate s action).current := by
  cases action with
  | stage command => exact Or.inl rfl
  | freeze i r => exact Or.inl rfl
  | acknowledge i r => exact Or.inl rfl
  | publish =>
      obtain ⟨pair,prepared⟩ := allowed.2
      have step := sound _ _ _ (valid.preparedFromCurrent pair prepared)
      exact Or.inr (by simpa [apply,prepared] using step)
  | install i r => exact Or.inl rfl
  | use i => exact Or.inl rfl

#print axioms check_exact
#print axioms execute_exact
#print axioms apply_preserves
#print axioms reached_invariant
#print axioms enabled_payload_current
#print axioms action_refines
end MirroreaProofFirst.PublicationPayload
