import Std

namespace MirroreaProofFirst.SharedAuthorityUse
-- A local refinement boundary for the existing Arc<Mutex<M9AuthorityGeneration>>.
-- Facts excludes validation counters; all authority-relevant fields remain bound.
-- The publisher's monotonicity and the local checker's auth meaning are separate
-- obligations. This model neither issues authority nor supplies distributed locks.
variable {Facts Request : Type} [DecidableEq Facts] {n : Nat}

structure State (n : Nat) (Facts : Type) where
  head : Facts
  cached : Fin n → Facts
  held : Option (Fin n)

def initial (head : Facts) (cached : Fin n → Facts) : State n Facts :=
  ⟨head, cached, none⟩

inductive Action (n : Nat) (Facts Request : Type) where
  | acquire (i : Fin n)
  | use (i : Fin n) (request : Request)
  | release (i : Fin n)
  | publish (facts : Facts)
  | refresh (i : Fin n)

-- The declarative side is independent of the Boolean gate.
def Allowed (Authorizes : Facts → Request → Prop) (s : State n Facts) :
    Action n Facts Request → Prop
  | .acquire i => s.held = none ∧ s.cached i = s.head
  | .use i request => s.held = some i ∧ Authorizes (s.cached i) request
  | .release i => s.held = some i
  | .publish _ => s.held = none
  | .refresh _ => s.held = none

def check (localCheck : Facts → Request → Bool) (s : State n Facts) :
    Action n Facts Request → Bool
  | .acquire i => s.held.isNone && decide (s.cached i = s.head)
  | .use i request => decide (s.held = some i) && localCheck (s.cached i) request
  | .release i => decide (s.held = some i)
  | .publish _ => s.held.isNone
  | .refresh _ => s.held.isNone

def apply (s : State n Facts) : Action n Facts Request → State n Facts
  | .acquire i => {s with held := some i}
  | .use _ _ => s
  | .release _ => {s with held := none}
  | .publish facts => {s with head := facts}
  | .refresh i => {s with cached := fun j => if j = i then s.head else s.cached j}

def execute (localCheck : Facts → Request → Bool) (s : State n Facts)
    (action : Action n Facts Request) : Option (State n Facts) :=
  if check localCheck s action then some (apply s action) else none

theorem check_exact (exactLocal : ∀ facts request,
    localCheck facts request = true ↔ Authorizes facts request)
    (s : State n Facts) (action : Action n Facts Request) :
    check localCheck s action = true ↔ Allowed Authorizes s action := by
  cases action <;> simp [check, Allowed, exactLocal]

theorem execute_exact (exactLocal : ∀ facts request,
    localCheck facts request = true ↔ Authorizes facts request)
    (s next : State n Facts) (action : Action n Facts Request) :
    execute localCheck s action = some next ↔
      Allowed Authorizes s action ∧ next = apply s action := by
  unfold execute
  split
  · rename_i passed
    simp only [Option.some.injEq]
    exact ⟨fun eq => ⟨(check_exact exactLocal s action).mp passed, eq.symm⟩,
      fun h => h.2.symm⟩
  · rename_i failed
    constructor
    · intro impossible; cases impossible
    · intro pair
      exact False.elim (failed ((check_exact exactLocal s action).mpr pair.1))

def Invariant (s : State n Facts) : Prop :=
  ∀ i, s.held = some i → s.cached i = s.head

theorem initial_valid (head : Facts) (cached : Fin n → Facts) :
    Invariant (initial head cached) := by
  intro i impossible
  cases impossible

variable {s next start : State n Facts} {action : Action n Facts Request}
  {Authorizes : Facts → Request → Prop} {localCheck : Facts → Request → Bool}
  {i : Fin n} {request : Request} {head facts : Facts} {cached : Fin n → Facts}

theorem preserves (valid : Invariant s) (allowed : Allowed Authorizes s action) :
    Invariant (apply s action) := by
  cases action with
  | acquire i =>
    intro j held
    have same : i = j := Option.some.inj held
    subst j
    exact allowed.2
  | use i request => exact valid
  | release i => intro j impossible; cases impossible
  | publish facts =>
    intro j held
    change s.held = some j at held
    rw [allowed] at held
    cases held
  | refresh i =>
    intro j held
    change s.held = some j at held
    rw [allowed] at held
    cases held

inductive Runs (Authorizes : Facts → Request → Prop) :
    State n Facts → State n Facts → Prop where
  | nil {s : State n Facts} : Runs Authorizes s s
  | step {start s : State n Facts} {action : Action n Facts Request} :
      Runs Authorizes start s → Allowed Authorizes s action →
      Runs Authorizes start (apply s action)

theorem runs_preserves (valid : Invariant start) (path : Runs Authorizes start s) :
    Invariant s := by
  induction path with
  | nil => exact valid
  | step _ allowed ih => exact preserves ih allowed

-- Ordinary use relies on cached local validation. Equality to the authoritative
-- head follows from all intervening transitions, not a use-rule premise.
theorem actual_use_current (valid : Invariant s)
    (allowed : Allowed Authorizes s (.use i request)) : Authorizes s.head request := by
  have authorized := allowed.2
  rw [valid i allowed.1] at authorized
  exact authorized

theorem accepted_use_current (exactLocal : ∀ facts request,
    localCheck facts request = true ↔ Authorizes facts request)
    (path : Runs Authorizes (initial head cached) s)
    (ran : execute localCheck s (.use i request) = some next) :
    Authorizes s.head request :=
  actual_use_current (runs_preserves (initial_valid head cached) path)
    ((execute_exact exactLocal s next (.use i request)).mp ran).1

theorem relative_complete (exactLocal : ∀ facts request,
    localCheck facts request = true ↔ Authorizes facts request)
    (allowed : Allowed Authorizes s action) :
    execute localCheck s action = some (apply s action) :=
  (execute_exact exactLocal s _ action).mpr ⟨allowed, rfl⟩

theorem stale_acquire_rejected (stale : s.cached i ≠ s.head) :
    execute localCheck s (.acquire i) = none := by
  simp [execute, check, stale]

theorem publication_waits (held : s.held = some i) :
    execute localCheck s (.publish facts) = none := by
  simp [execute, check, held]

-- Facts freshness is independent of the meaning/soundness of localCheck.
-- Instantiating the relation below with observed checker acceptance proves no
-- semantic authorization; it only reuses the transition preservation lemma.
theorem checked_preserves (valid : Invariant s)
    (ran : execute localCheck s action = some next) : Invariant next := by
  have parts := (execute_exact
    (Authorizes := fun facts request => localCheck facts request = true)
    (fun _ _ => Iff.rfl) s next action).mp ran
  rw [parts.2]
  exact preserves valid parts.1

inductive CheckedRuns (localCheck : Facts → Request → Bool) :
    State n Facts → State n Facts → Prop where
  | nil {s : State n Facts} : CheckedRuns localCheck s s
  | step {start s next : State n Facts} {action : Action n Facts Request} :
      CheckedRuns localCheck start s → execute localCheck s action = some next →
      CheckedRuns localCheck start next

theorem checked_runs_preserves (valid : Invariant start)
    (path : CheckedRuns localCheck start s) : Invariant s := by
  induction path with
  | nil => exact valid
  | step _ ran ih => exact checked_preserves ih ran

theorem checked_use_facts_current (valid : Invariant s)
    (ran : execute localCheck s (.use i request) = some next) : s.cached i = s.head := by
  have parts := (execute_exact
    (Authorizes := fun facts request => localCheck facts request = true)
    (fun _ _ => Iff.rfl) s next (.use i request)).mp ran
  exact valid i parts.1.1

theorem checked_history_use_facts_current
    (path : CheckedRuns localCheck (initial head cached) s)
    (ran : execute localCheck s (.use i request) = some next) : s.cached i = s.head :=
  checked_use_facts_current (checked_runs_preserves (initial_valid head cached) path) ran

#print axioms checked_preserves
#print axioms checked_runs_preserves
#print axioms checked_use_facts_current
#print axioms checked_history_use_facts_current

-- Two actual mechanisms compared: retaining the critical section vs releasing
-- after preflight. This finite falsifier is evidence, not a general theorem.
def exampleCheck (facts request : Nat) : Bool := decide (facts = request)
def ready : State 2 Nat := initial 7 (fun _ => 7)
def held : State 2 Nat := apply ready (Action.acquire 0 : Action 2 Nat Nat)
def released : State 2 Nat := apply held (Action.release 0 : Action 2 Nat Nat)
def revoked : State 2 Nat := apply released (Action.publish 8 : Action 2 Nat Nat)
#guard execute exampleCheck ready (.acquire 0) |>.isSome
#guard execute exampleCheck held (.use 0 7) |>.isSome
#guard execute exampleCheck held (.publish 8) |>.isNone
#guard execute exampleCheck revoked (.acquire 0) |>.isNone
#guard execute exampleCheck revoked (.use 0 7) |>.isNone
#guard exampleCheck (revoked.cached 0) 7 && !exampleCheck revoked.head 7
#guard execute exampleCheck (apply revoked (.refresh 0 : Action 2 Nat Nat)) (.acquire 0) |>.isSome

#print axioms check_exact
#print axioms execute_exact
#print axioms initial_valid
#print axioms preserves
#print axioms runs_preserves
#print axioms actual_use_current
#print axioms accepted_use_current
#print axioms relative_complete
#print axioms stale_acquire_rejected
#print axioms publication_waits
end MirroreaProofFirst.SharedAuthorityUse
