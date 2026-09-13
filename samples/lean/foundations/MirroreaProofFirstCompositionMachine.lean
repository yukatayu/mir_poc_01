import MirroreaProofFirstInvocationBoundary

namespace MirroreaProofFirst.CompositionMachine
open CurrentUse WorldProjection CompositionCore InvocationBoundary

def ticketId (t : Ticket) : UseId := ⟨t.realm,t.principal,t.id⟩

inductive Occurrence where
  | management (context : ManagementEntry.Context) (created : Option Nat)
  | requested (ticket : Ticket)
  | result (ticket : Ticket) (value : Int)
  | authorityHead (generation : Nat)
  deriving DecidableEq, Repr

structure Machine (p a : Nat) where
  system : ManagementEntry.System p a
  pending : List Ticket
  events : List Occurrence

def Invariant (m : Machine p a) : Prop :=
  ManagementEntry.Invariant m.system ∧ (m.pending.map ticketId).Nodup ∧
    ∀ t ∈ m.pending, ticketId t ∉ m.system.used

def fresh (m : Machine p a) (id : UseId) : Bool :=
  !m.system.used.contains id && !(m.pending.map ticketId).contains id

theorem fresh_exact (m : Machine p a) (id : UseId) :
    fresh m id = true ↔ id ∉ m.system.used ∧ id ∉ m.pending.map ticketId := by simp [fresh]

def manage (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat) (raw : Raw) :
    Option (Machine p a × Option Nat) := do
  if !fresh m (ManagementEntry.useId m.system principal id) then none else do
    let (system,created) ← ManagementEntry.perform m.system member place principal id raw
    return ({m with
      system := system
      events := .management (ManagementEntry.current m.system member place principal id raw) created :: m.events},created)

-- The occurrence records the exact applied command before any later source
-- step observes its result. In particular reparent is an occurrence, not an
-- unrecorded edit of a snapshot. Event order below is newest-first.
theorem manage_parts (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat) (raw : Raw)
    (next : Machine p a × Option Nat) (accepted : manage m member place principal id raw = some next) :
    fresh m (ManagementEntry.useId m.system principal id) = true ∧
    ∃ e cfg created, ManagementEntry.commit m.system member place principal id raw e =
      some (ManagementEntry.after m.system cfg principal id,created) ∧
      CompositionCore.run m.system.configuration raw = some (cfg,created) ∧
      next = ({m with
        system := ManagementEntry.after m.system cfg principal id
        events := .management (ManagementEntry.current m.system member place principal id raw) created :: m.events},created) := by
  unfold manage at accepted
  split at accepted
  · cases accepted
  · rename_i hf
    have hfresh : fresh m (ManagementEntry.useId m.system principal id) = true := by simpa using hf
    unfold ManagementEntry.perform at accepted
    cases he : ManagementEntry.authorize m.system member place principal id raw with
    | none => simp [he] at accepted
    | some e =>
        simp only [he,Option.bind_eq_bind,Option.bind_some] at accepted
        cases hc : ManagementEntry.commit m.system member place principal id raw e with
        | none => simp [hc] at accepted
        | some result =>
            obtain ⟨_,cfg,created,hr,eq⟩ := ManagementEntry.commit_parts m.system member place principal id raw e result hc
            subst result
            rw [hc] at accepted
            exact ⟨hfresh,e,cfg,created,hc,hr,(Option.some.inj accepted).symm⟩

theorem manage_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat) (raw : Raw)
    (next : Machine p a × Option Nat) (valid : Invariant m)
    (accepted : manage m member place principal id raw = some next) : Invariant next.1 := by
  obtain ⟨hf,e,cfg,created,hc,hr,rfl⟩ := manage_parts m member place principal id raw next accepted
  have freshParts := (fresh_exact _ _).mp hf
  refine ⟨(ManagementEntry.commit_preserves _ _ _ _ _ _ _ _ valid.1 hc).1,valid.2.1,?_⟩
  intro t ht
  simp only [ManagementEntry.after,List.mem_cons,not_or]
  refine ⟨?_,valid.2.2 t ht⟩
  intro same
  exact freshParts.2 (same ▸ List.mem_map.mpr ⟨t,ht,rfl⟩)

def enqueue (m : Machine p a) (t : Ticket) : Machine p a :=
  {m with pending := t :: m.pending,events := .requested t :: m.events}

def start (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) (arg : Int) :
    Option (Machine p a × Ticket) := do
  let unit ← index m.system.configuration.count key
  let t ← prepare m.system.configuration.state m.system.view member unit place principal id arg
  if fresh m (ticketId t) then some (enqueue m t,t) else none

theorem start_parts (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) (arg : Int)
    (next : Machine p a × Ticket) (accepted : start m member place principal id key arg = some next) :
    fresh m (ticketId next.2) = true ∧ check m.system.configuration.state m.system.view next.2 = true ∧
      next.1 = enqueue m next.2 := by
  unfold start at accepted
  cases hi : index m.system.configuration.count key with
  | none => simp [hi] at accepted
  | some unit =>
      simp only [hi,Option.bind_eq_bind,Option.bind_some] at accepted
      cases hp : prepare m.system.configuration.state m.system.view member unit place principal id arg with
      | none => simp [hp] at accepted
      | some t =>
          rw [hp] at accepted
          change (if fresh m (ticketId t) then some (enqueue m t,t) else none) = some next at accepted
          split at accepted
          · cases accepted; exact ⟨‹_›,prepare_checked _ _ _ _ _ _ _ _ _ hp,rfl⟩
          · cases accepted

theorem start_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) (arg : Int)
    (next : Machine p a × Ticket) (valid : Invariant m)
    (accepted : start m member place principal id key arg = some next) : Invariant next.1 := by
  obtain ⟨hf,_,eq⟩ := start_parts m member place principal id key arg next accepted
  rw [eq]
  obtain ⟨unused,unpending⟩ := (fresh_exact _ _).mp hf
  refine ⟨valid.1,List.nodup_cons.mpr ⟨unpending,valid.2.1⟩,?_⟩
  intro t ht
  rcases List.mem_cons.mp ht with rfl | old
  · exact unused
  · exact valid.2.2 t old

def finishCheck (m : Machine p a) (t : Ticket) (value : Int) : Bool :=
  m.pending.contains t && !m.system.used.contains (ticketId t) &&
    resultCheck m.system.configuration.state m.system.view t value

def consume (m : Machine p a) (t : Ticket) (value : Int) : Machine p a :=
  {system := {m.system with serial := m.system.serial+1,used := ticketId t :: m.system.used},
   pending := m.pending.filter (fun other => decide (ticketId other ≠ ticketId t)),
   events := .result t value :: m.events}

def finish (m : Machine p a) (t : Ticket) (value : Int) : Option (Machine p a) :=
  if finishCheck m t value then some (consume m t value) else none

theorem finish_parts (m : Machine p a) (t : Ticket) (value : Int) (next : Machine p a)
    (accepted : finish m t value = some next) :
    t ∈ m.pending ∧ ticketId t ∉ m.system.used ∧
      resultCheck m.system.configuration.state m.system.view t value = true ∧ next = consume m t value := by
  unfold finish at accepted
  split at accepted
  · rename_i h
    have parts : t ∈ m.pending ∧ ticketId t ∉ m.system.used ∧
        resultCheck m.system.configuration.state m.system.view t value = true := by
      simpa [finishCheck,and_assoc] using h
    cases accepted
    exact ⟨parts.1,parts.2.1,parts.2.2,rfl⟩
  · cases accepted

theorem finish_preserves (m : Machine p a) (t : Ticket) (value : Int) (next : Machine p a)
    (valid : Invariant m) (accepted : finish m t value = some next) : Invariant next := by
  obtain ⟨_,unused,_,rfl⟩ := finish_parts m t value next accepted
  refine ⟨⟨valid.1.1,List.nodup_cons.mpr ⟨unused,valid.1.2⟩⟩,?_,?_⟩
  · exact List.Nodup.sublist ((List.filter_sublist).map ticketId) valid.2.1
  · intro other member
    have mem : other ∈ m.pending ∧ ticketId other ≠ ticketId t := by
      simpa [consume] using member
    simp only [consume,List.mem_cons,not_or]
    exact ⟨mem.2,valid.2.2 other mem.1⟩

theorem finish_current (m : Machine p a) (t : Ticket) (value : Int) (next : Machine p a)
    (accepted : finish m t value = some next) :
    ResultMeaning m.system.configuration.state m.system.view t value ∧
      next.events = .result t value :: m.events := by
  obtain ⟨_,_,checked,rfl⟩ := finish_parts m t value next accepted
  exact ⟨result_sound _ _ _ _ checked,rfl⟩

theorem no_double_consume (m : Machine p a) (t : Ticket) (value : Int) (other : Ticket) (otherValue : Int)
    (same : ticketId other = ticketId t) : finish (consume m t value) other otherValue = none := by
  simp [finish,finishCheck,consume,same]

-- This continues the saved source request without minting a different witness.
def resume (m : Machine p a) (t : Ticket) : Option (Machine p a × Int) := do
  let value ← execute t
  let next ← finish m t value
  return (next,value)

def authorityHead (m : Machine p a) (view : AuthorityView a) : Machine p a :=
  {m with
    system := ManagementEntry.installAuthorityHead m.system view
    events := .authorityHead view.generation :: m.events}

theorem head_preserves (m : Machine p a) (view : AuthorityView a) (valid : Invariant m) :
    Invariant (authorityHead m view) := valid

namespace Controls
def initial : Machine 3 1 :=
  ⟨{ManagementEntry.Controls.initial with
     configuration := config InstanceState.Controls.two,view := InvocationBoundary.Controls.view},[],[]⟩
def pending := start initial 0 2 7 20 1 2
#guard pending.isSome
#guard (pending.bind fun (m,t) => resume m t).map Prod.snd = some 5
#guard (pending.bind fun (m,t) => finish m t 6).isNone
#guard (pending.bind fun (m,t) => (resume m t).bind fun (m,_) => resume m t).isNone
#guard (pending.bind fun (m,_) => start m 0 2 7 20 1 2).isNone
#guard (pending.bind fun (m,t) => resume (authorityHead m
  {m.system.view with authority := {m.system.view.authority with revoked := [10]}}) t).isNone
end Controls

#print axioms manage_preserves
#print axioms start_preserves
#print axioms finish_preserves
#print axioms finish_current
#print axioms no_double_consume
#print axioms head_preserves
end MirroreaProofFirst.CompositionMachine
