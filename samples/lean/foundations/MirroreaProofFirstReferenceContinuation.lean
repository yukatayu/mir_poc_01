import MirroreaProofFirstReferenceAuthority

namespace MirroreaProofFirst.ReferenceContinuation
open ReferenceSourceData ReferenceSource

-- The parsed transition placement travels with its body. The driver has no
-- separate place argument that could silently override an `at` clause.
structure Program (p : Nat) where
  place : Fin p
  items : List Located
  deriving DecidableEq, Repr
structure Superseded (p : Nat) where
  program : Program p
  completed : List Located
  stopped : Option Located
  remaining : List Located
  status : Status
  deriving DecidableEq, Repr
structure Session (p a : Nat) where
  state : State p a
  program : Program p
  completed : List Located
  remaining : List Located
  stopped : Option Located
  status : Status
  superseded : List (Superseded p)

def archive (s : Session p a) : Superseded p :=
  ⟨s.program,s.completed,s.stopped,s.remaining,s.status⟩

-- New sessions have fresh initial state. Reuse of a running state goes through
-- checked replacement/continuation and cannot relaunch away its old archive.
-- Separate roots require separate externally admitted namespace ownership;
-- this pure constructor does not establish cross-root uniqueness or authority.
def launch (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (program : Program p) : Option (Session p a) := do
  let _ ← checkProgram p [] program.items
  return ⟨ReferenceSource.initial realm view policy,program,[],program.items,none,.ready,[]⟩

def tick (s : Session p a) (member : Fin a) (principal : Nat) : Session p a :=
  match s.status with
  | .failed _ => s
  | .waiting =>
      let result := complete s.state
      {s with
        state := result.state,status := result.status,
        completed := if result.status = .ready then s.completed ++ s.stopped.toList else s.completed,
        stopped := if result.status = .ready then none else s.stopped}
  | .ready => match s.remaining with
    | [] => s
    | item :: rest =>
        let result := advance s.state member s.program.place principal item
        {s with
          state := result.state,remaining := rest,status := result.status,
          completed := if result.status = .ready then s.completed ++ [item] else s.completed,
          stopped := if result.status = .ready then none else some item}

def drive : Nat → Session p a → Fin a → Nat → Session p a
  | 0,s,_,_ => s
  | fuel+1,s,member,principal => drive fuel (tick s member principal) member principal

def run (s : Session p a) (member : Fin a) (principal : Nat) : Session p a :=
  drive (2*s.remaining.length+1) s member principal

def cancel (s : Session p a) (member : Fin a) (principal : Nat) : Session p a :=
  let result := ReferenceSource.cancel s.state member s.program.place principal
  if result.status = .ready then
    {s with state := result.state,status := .failed .cancelled}
  else s

private def adopt (s : Session p a) (program : Program p) : Option (Session p a) := do
  if s.state.waiting.isSome then none else do
    let _ ← checkProgram p (environment s.state.values) program.items
    return {s with
      program := program,completed := [],remaining := program.items,stopped := none,status := .ready,
      superseded := archive s :: s.superseded}

def replaceResidual (s : Session p a) (program : Program p) : Option (Session p a) :=
  match s.status with | .failed _ => adopt s program | _ => none

-- A completed source block can be followed by another actual source block on
-- the SAME state. Its complete old program and context remain in the archive.
def continueWith (s : Session p a) (program : Program p) : Option (Session p a) :=
  if s.status = .ready ∧ s.remaining = [] then adopt s program else none

def authorityHead (s : Session p a) (view : WorldProjection.AuthorityView a) : Option (Session p a) := do
  let next ← ReferenceAuthority.install s.state view
  return {s with state := next}

def controlInput (s : Session p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) : Option (Session p a × Option Nat) := do
  let (next,created) ← ReferenceSource.controlInput s.state member place principal raw
  return ({s with state := next},created)

def Replacement (s : Session p a) (program : Program p) : Prop :=
  (∃ reason, s.status = .failed reason) ∧ s.state.waiting = none ∧
    ∃ env, ProgramTyped p (environment s.state.values) program.items env

private theorem adopt_exact (s : Session p a) (program : Program p) :
    (∃ next, adopt s program = some next) ↔ s.state.waiting = none ∧
      ∃ env, ProgramTyped p (environment s.state.values) program.items env := by
  have programs : (∃ env, checkProgram p (environment s.state.values) program.items = some env) ↔
      ∃ env, ProgramTyped p (environment s.state.values) program.items env := by
    constructor
    · rintro ⟨env,checked⟩; exact ⟨env,(program_exact _ _ _ _).mp checked⟩
    · rintro ⟨env,meaning⟩; exact ⟨env,(program_exact _ _ _ _).mpr meaning⟩
  rw [← programs]
  cases pending : s.state.waiting <;> cases typed : checkProgram p (environment s.state.values) program.items <;>
    simp [adopt,pending,typed]

theorem replace_exact (s : Session p a) (program : Program p) :
    (∃ next, replaceResidual s program = some next) ↔ Replacement s program := by
  cases phase : s.status <;> simp [replaceResidual,Replacement,phase,adopt_exact]

def Adopted (s next : Session p a) (program : Program p) : Prop :=
  next.state = s.state ∧ next.program = program ∧ next.completed = [] ∧ next.remaining = program.items ∧
    next.stopped = none ∧ next.status = .ready ∧ next.superseded = archive s :: s.superseded

private theorem adopt_state (s next : Session p a) (program : Program p)
    (accepted : adopt s program = some next) : Adopted s next program := by
  unfold adopt at accepted
  split at accepted
  · cases accepted
  · cases typed : checkProgram p (environment s.state.values) program.items with
    | none => simp [typed] at accepted
    | some env =>
        simp only [typed,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
        subst next
        exact ⟨rfl,rfl,rfl,rfl,rfl,rfl,rfl⟩

theorem replace_state (s next : Session p a) (program : Program p)
    (accepted : replaceResidual s program = some next) : Adopted s next program := by
  unfold replaceResidual at accepted
  split at accepted
  · exact adopt_state _ _ _ accepted
  · cases accepted

theorem continue_state (s next : Session p a) (program : Program p)
    (accepted : continueWith s program = some next) : Adopted s next program := by
  unfold continueWith at accepted
  split at accepted
  · exact adopt_state _ _ _ accepted
  · cases accepted

theorem launch_state (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (program : Program p) (s : Session p a) (accepted : launch realm view policy program = some s) :
    s = ⟨ReferenceSource.initial realm view policy,program,[],program.items,none,.ready,[]⟩ := by
  unfold launch at accepted
  cases typed : checkProgram p [] program.items with
  | none => simp [typed] at accepted
  | some env => simpa [typed] using accepted.symm

theorem tick_source (s : Session p a) (member : Fin a) (principal : Nat) :
    ReferenceSourceTrace.Reached s.state (tick s member principal).state := by
  cases phase : s.status with
  | failed reason => simp only [tick,phase]; exact .refl
  | waiting => simp only [tick,phase]; exact .step .refl .complete
  | ready => cases remaining : s.remaining with
    | nil => simp only [tick,phase,remaining]; exact .refl
    | cons item rest => simp only [tick,phase,remaining]; exact .step .refl .advance

theorem drive_source (fuel : Nat) (s : Session p a) (member : Fin a) (principal : Nat) :
    ReferenceSourceTrace.Reached s.state (drive fuel s member principal).state := by
  induction fuel generalizing s with
  | zero => exact .refl
  | succ fuel ih => exact ReferenceSourceTrace.trans _ _ _ (tick_source _ _ _) (ih _)

theorem cancel_source (s : Session p a) (member : Fin a) (principal : Nat) :
    ReferenceSourceTrace.Reached s.state (cancel s member principal).state := by
  simp only [cancel]
  split
  · exact .step .refl .cancellation
  · exact .refl

theorem cancel_retains (s : Session p a) (member : Fin a) (principal : Nat) :
    (cancel s member principal).program = s.program ∧ (cancel s member principal).completed = s.completed ∧
    (cancel s member principal).remaining = s.remaining ∧ (cancel s member principal).stopped = s.stopped ∧
    (cancel s member principal).superseded = s.superseded := by
  simp only [cancel]
  split <;> exact ⟨rfl,rfl,rfl,rfl,rfl⟩

theorem cancel_no_write (s : Session p a) (member : Fin a) (principal : Nat) :
    (cancel s member principal).state.values = s.state.values ∧
      (cancel s member principal).state.writes = s.state.writes := by
  simp only [cancel]
  split
  · have equal := cancel_values_writes s.state member s.program.place principal
    exact ⟨equal.1,equal.2.1⟩
  · exact ⟨rfl,rfl⟩

theorem blocked_retained (s : Session p a) (member : Fin a) (principal : Nat)
    (item : Located) (rest : List Located) (ready : s.status = .ready) (remaining : s.remaining = item :: rest)
    (blocked : (advance s.state member s.program.place principal item).status ≠ .ready) :
    (tick s member principal).stopped = some item ∧ (tick s member principal).remaining = rest ∧
      (tick s member principal).state = (advance s.state member s.program.place principal item).state := by
  simp [tick,ready,remaining,blocked]

theorem different_document (left right : Site) (different : left.document ≠ right.document) : left ≠ right := by
  intro equal
  exact different (congrArg Site.document equal)

theorem launch_exact (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (program : Program p) :
    (∃ s, launch realm view policy program = some s) ↔ ∃ env, ProgramTyped p [] program.items env := by
  have typed : (∃ env, checkProgram p [] program.items = some env) ↔ ∃ env, ProgramTyped p [] program.items env := by
    constructor
    · rintro ⟨env,checked⟩; exact ⟨env,(program_exact _ _ _ _).mp checked⟩
    · rintro ⟨env,meaning⟩; exact ⟨env,(program_exact _ _ _ _).mpr meaning⟩
  rw [← typed]
  cases run : checkProgram p [] program.items <;> simp [launch,run]

theorem continue_exact (s : Session p a) (program : Program p) :
    (∃ next, continueWith s program = some next) ↔
      s.status = .ready ∧ s.remaining = [] ∧ s.state.waiting = none ∧
        ∃ env, ProgramTyped p (environment s.state.values) program.items env := by
  by_cases ready : s.status = .ready ∧ s.remaining = []
  · simp [continueWith,ready,adopt_exact,ready.1,ready.2]
  · simp [continueWith,ready]
    intro status remaining
    exact False.elim (ready ⟨status,remaining⟩)

#print axioms launch_exact
#print axioms continue_exact

#print axioms replace_exact
#print axioms replace_state
#print axioms continue_state
#print axioms launch_state
#print axioms tick_source
#print axioms drive_source
#print axioms cancel_source
#print axioms cancel_retains
#print axioms cancel_no_write
#print axioms blocked_retained
end MirroreaProofFirst.ReferenceContinuation
