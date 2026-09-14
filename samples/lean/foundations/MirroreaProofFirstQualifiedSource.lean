import MirroreaProofFirstSourcePlacement
import MirroreaProofFirstReceivedTicket

namespace MirroreaProofFirst.QualifiedSource
open ReferenceSourceData

-- LAB semantic candidate, not parser syntax or a public contract. Qualification
-- is introduced only when creating a NEW callable/reference value. Aliases copy
-- its intent. Existing reference bindings and pending tickets are never edited.
-- An absent tag means legacy caller-local intent, NOT the saved binding locus.
inductive Intent (p : Nat) where
  | local
  | at (place : Fin p)
  deriving DecidableEq, Repr
abbrev Tags (p : Nat) := List (String × Intent p)

def intent (tags : Tags p) (name : String) : Intent p :=
  (((tags.find? fun row => row.1 == name)).map Prod.snd).getD .local

def locus (caller : Fin p) : Intent p → Fin p
  | .local => caller
  | .at place => place

structure Raw where
  source : Located
  allocation : Option Nat
  deriving DecidableEq, Repr

def IsAllocation : Statement → Prop
  | .plain (.instantiate ..) | .acquire .. => True
  | _ => False
def isAllocation : Statement → Bool
  | .plain (.instantiate ..) | .acquire .. => true
  | _ => false

theorem allocation_exact (item : Statement) : isAllocation item = true ↔ IsAllocation item := by
  cases item with
  | plain item => cases item <;> simp [isAllocation,IsAllocation]
  | _ => simp [isAllocation,IsAllocation]

-- Operations consuming qualified values inherit that value's coordinate.
-- Reader/options in a new fallback chain do not choose a placement search.
def target : Statement → Option String
  | .plain (.invoke _ name _) | .plain (.retire _ name) |
    .plain (.reparent _ name _) | .plain (.replace _ name _) => some name
  | .reacquire _ name | .release _ name => some name
  | _ => none

def inherited (tags : Tags p) (item : Statement) : Intent p :=
  ((target item).map (intent tags)).getD .local

def Selection (tags : Tags p) (raw : Raw) (chosen : Intent p) : Prop :=
  match raw.allocation with
  | none => chosen = inherited tags raw.source.statement
  | some n => IsAllocation raw.source.statement ∧ ∃ place : Fin p, place.val = n ∧ chosen = .at place

def select (tags : Tags p) (raw : Raw) : Option (Intent p) :=
  match raw.allocation with
  | none => some (inherited tags raw.source.statement)
  | some n => if isAllocation raw.source.statement then (CompositionCore.index p n).map Intent.at else none

theorem select_exact (tags : Tags p) (raw : Raw) (chosen : Intent p) :
    select tags raw = some chosen ↔ Selection tags raw chosen := by
  cases annotation : raw.allocation with
  | none => simp [select,Selection,annotation,eq_comm]
  | some n =>
      by_cases allowed : isAllocation raw.source.statement = true
      · have meaning := (allocation_exact _).mp allowed
        simp only [select,Selection,annotation,allowed,ite_true,meaning,true_and]
        constructor
        · intro selected
          cases indexed : CompositionCore.index p n with
          | none => simp [indexed] at selected
          | some place =>
              exact ⟨place,CompositionCore.index_sound _ indexed,by simpa [indexed] using selected.symm⟩
        · rintro ⟨place,same,rfl⟩
          rw [← same,CompositionCore.index_roundtrip]; rfl
      · simp [select,Selection,annotation,allowed,
          show ¬ IsAllocation raw.source.statement from fun h => allowed ((allocation_exact _).mpr h)]

structure Core (p : Nat) where
  source : Located
  caller : Fin p
  selected : Intent p
  deriving DecidableEq, Repr

def Elaborates (env : Environment) (tags : Tags p) (caller : Fin p) (raw : Raw)
    (core : Core p) (next : Environment) : Prop :=
  core.source = raw.source ∧ core.caller = caller ∧ Selection tags raw core.selected ∧
    StatementTyped p env raw.source.statement next

def compile (env : Environment) (tags : Tags p) (caller : Fin p) (raw : Raw) :
    Option (Core p × Environment) := do
  let chosen ← select tags raw
  let next ← checkStatement p env raw.source.statement
  return (⟨raw.source,caller,chosen⟩,next)

theorem compile_exact (env : Environment) (tags : Tags p) (caller : Fin p) (raw : Raw)
    (core : Core p) (next : Environment) :
    compile env tags caller raw = some (core,next) ↔ Elaborates env tags caller raw core next := by
  constructor
  · intro checked
    unfold compile at checked
    cases selected : select tags raw with
    | none => simp [selected] at checked
    | some chosen =>
        simp only [selected,Option.bind_eq_bind,Option.bind_some] at checked
        cases typed : checkStatement p env raw.source.statement with
        | none => simp [typed] at checked
        | some output =>
            simp only [typed,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at checked
            obtain ⟨rfl,rfl⟩ := checked
            exact ⟨rfl,rfl,(select_exact _ _ _).mp selected,(statement_exact _ _ _ _).mp typed⟩
  · rintro ⟨sourceAt,callerAt,selected,typed⟩
    have chosen := (select_exact _ _ _).mpr selected
    have checked := (statement_exact _ _ _ _).mpr typed
    cases core
    simp_all [compile]

def taggedOutput (tags : Tags p) (core : Core p) : Option (String × Intent p) :=
  match core.source.statement with
  | .plain (.instantiate name ..) | .acquire name _ => some (name,core.selected)
  | .alias name other => some (name,intent tags other)
  | _ => none

def afterTags (tags : Tags p) (core : Core p) (status : ReferenceSource.Status) : Tags p :=
  if status = .ready then (taggedOutput tags core).toList ++ tags else tags

structure State (p a : Nat) where
  source : ReferenceSource.State p a
  tags : Tags p

structure Outcome (p a : Nat) where
  state : State p a
  status : ReferenceSource.Status

-- The admitted caller activation supplies these coordinates; incoming messages
-- cannot nominate them. Authentic registry/process custody is a separate W4
-- implementation obligation. This record alone grants no authority.
structure Actor (p a : Nat) where
  caller : Fin p
  member : Fin a
  principal : Nat
  deriving DecidableEq, Repr

def runCore (s : State p a) (actor : Actor p a) (core : Core p) : Outcome p a :=
  let result := ReferenceSource.advance s.source actor.member (locus actor.caller core.selected)
    actor.principal core.source
  ⟨⟨result.state,afterTags s.tags core result.status⟩,result.status⟩

-- Executable entry compiles from ACTUAL current values and the fixed caller;
-- raw Core construction is not an admitted source entry.
def advance (s : State p a) (actor : Actor p a) (raw : Raw) : Outcome p a :=
  match compile (environment s.source.values) s.tags actor.caller raw with
  | none => ⟨s,.failed .staticType⟩
  | some (core,_) => runCore s actor core

theorem advance_basis (changed : (advance s actor raw).state.source ≠ s.source) :
    ∃ core env, Elaborates (environment s.source.values) s.tags actor.caller raw core env ∧
      advance s actor raw = runCore s actor core := by
  cases checked : compile (environment s.source.values) s.tags actor.caller raw with
  | none => simp [advance,checked] at changed
  | some pair => exact ⟨pair.1,pair.2,(compile_exact _ _ _ _ _ _).mp checked,by simp [advance,checked]⟩

theorem advance_source (s : State p a) (actor : Actor p a) (raw : Raw) :
    ReferenceSourceTrace.Reached s.source (advance s actor raw).state.source := by
  cases checked : compile (environment s.source.values) s.tags actor.caller raw with
  | none => simp only [advance,checked]; exact .refl
  | some pair => simp only [advance,checked,runCore]; exact .step .refl .advance

theorem waiting_tags (s : State p a) (actor : Actor p a) (raw : Raw)
    (waiting : (advance s actor raw).status = .waiting) :
    (advance s actor raw).state.tags = s.tags := by
  cases checked : compile (environment s.source.values) s.tags actor.caller raw with
  | none => simp [advance,checked] at waiting
  | some pair =>
      simp only [advance,checked,runCore] at waiting ⊢
      simp [afterTags,waiting]

theorem alias_intent (tags : Tags p) (caller : Fin p) (selected : Intent p) (site : Site) (name other : String) :
    intent (afterTags tags ⟨⟨site,.alias name other⟩,caller,selected⟩ .ready) name = intent tags other := by
  simp [afterTags,taggedOutput,intent]

theorem nonallocation_cannot_retarget (raw : Raw) (notNew : ¬ IsAllocation raw.source.statement)
    (explicit : raw.allocation = some n) : select (p := p) tags raw = none := by
  have refused : isAllocation raw.source.statement = false := by
    cases found : isAllocation raw.source.statement
    · rfl
    · exact False.elim (notNew ((allocation_exact _).mp found))
  simp [select,explicit,refused]

-- This general capture statement connects current-environment compilation,
-- fixed caller, selected operation locus and the original saved source reads.
-- Physical origin is deliberately absent: it still needs endpoint refinement.
theorem advance_waiting (s : State p a) (actor : Actor p a) (raw : Raw)
    (waiting : (advance s actor raw).status = .waiting) :
    ∃ core env saved, Elaborates (environment s.source.values) s.tags actor.caller raw core env ∧
      (advance s actor raw).state.source.waiting = some saved ∧
      saved.entry.ticket.place = (locus actor.caller core.selected).val ∧
      saved.site = raw.source.site ∧ saved.inputs = ReferenceSource.inputs s.source raw.source := by
  cases checked : compile (environment s.source.values) s.tags actor.caller raw with
  | none => simp [advance,checked] at waiting
  | some pair =>
      obtain ⟨core,env⟩ := pair
      have meaning := (compile_exact _ _ _ _ _ _).mp checked
      have atSource : (SourcePlacement.execute s.source actor.member actor.principal
          ⟨core.source,actor.caller,locus actor.caller core.selected⟩).status = .waiting := by
        simpa [advance,checked,runCore,SourcePlacement.execute] using waiting
      obtain ⟨saved,savedAt,placeAt,siteAt,inputsAt⟩ := SourcePlacement.execute_waiting_target _ _ _ _ atSource
      refine ⟨core,env,saved,meaning,?_,placeAt,?_,?_⟩
      · simpa [advance,checked,runCore,SourcePlacement.execute] using savedAt
      · simpa [meaning.1] using siteAt
      · simpa [meaning.1] using inputsAt

theorem alias_machine_identity (s : ReferenceSource.State p a) (member : Fin a) (place : Fin p)
    (principal : Nat) (site : Site) (name other : String) (key : Nat) (env : Environment)
    (idle : s.waiting = none)
    (typed : checkStatement p (environment s.values) (.alias name other) = some env)
    (found : lookup s.values other = some (.reference key)) :
    (ReferenceSource.advance s member place principal ⟨site,.alias name other⟩).state.machine = s.machine ∧
    lookup (ReferenceSource.advance s member place principal ⟨site,.alias name other⟩).state.values name = some (.reference key) := by
  have keyAt : referenceKey s.values other = some key := by simp [referenceKey,found]
  have elaborated : ReferenceSource.elaborate s.machine.store.core.system.configuration.state s.values
      (.alias name other) = some (.pureValue name (.reference key) false) := by
    simp [ReferenceSource.elaborate,keyAt]
  simp [ReferenceSource.advance,idle,typed,elaborated,ReferenceSource.executePlan,ReferenceSource.write,lookup]

theorem advance_nonwaiting (s : State p a) (actor : Actor p a) (raw : Raw)
    (notWaiting : (advance s actor raw).status ≠ .waiting) :
    (advance s actor raw).state.source.waiting = s.source.waiting := by
  cases checked : compile (environment s.source.values) s.tags actor.caller raw with
  | none => simp [advance,checked]
  | some pair =>
      simp only [advance,checked,runCore] at notWaiting ⊢
      exact ReferenceSourceTyping.advance_nonwaiting _ _ _ _ _ notWaiting

theorem advance_environment (s : State p a) (actor : Actor p a) (raw : Raw) (env : Environment)
    (typed : StatementTyped p (environment s.source.values) raw.source.statement env)
    (ready : (advance s actor raw).status = .ready) :
    environment (advance s actor raw).state.source.values = env := by
  cases checked : compile (environment s.source.values) s.tags actor.caller raw with
  | none => simp [advance,checked] at ready
  | some pair =>
      have sourceAt := ((compile_exact _ _ _ _ _ _).mp checked).1
      have atSource : (ReferenceSource.advance s.source actor.member
          (locus actor.caller pair.1.selected) actor.principal raw.source).status = .ready := by
        simpa [advance,checked,runCore,sourceAt] using ready
      simpa [advance,checked,runCore,sourceAt] using
        ReferenceSourceTyping.advance_environment _ _ _ _ _ _ typed atSource

theorem advance_waiting_environment (s : State p a) (actor : Actor p a) (raw : Raw) (env : Environment)
    (typed : StatementTyped p (environment s.source.values) raw.source.statement env)
    (waiting : (advance s actor raw).status = .waiting) :
    (advance s actor raw).state.source.values = s.source.values ∧
      ∃ saved, (advance s actor raw).state.source.waiting = some saved ∧
        lookupType (environment s.source.values) saved.name = none ∧
        env = (saved.name,.plain (.integer false)) :: environment s.source.values := by
  cases checked : compile (environment s.source.values) s.tags actor.caller raw with
  | none => simp [advance,checked] at waiting
  | some pair =>
      have sourceAt := ((compile_exact _ _ _ _ _ _).mp checked).1
      have atSource : (ReferenceSource.advance s.source actor.member
          (locus actor.caller pair.1.selected) actor.principal raw.source).status = .waiting := by
        simpa [advance,checked,runCore,sourceAt] using waiting
      simpa [advance,checked,runCore,sourceAt] using
        ReferenceSourceTyping.advance_waiting _ _ _ _ _ _ typed atSource

theorem advance_pending_site (s : State p a) (actor : Actor p a) (raw : Raw)
    (idle : s.source.waiting = none) (saved : ReferenceSource.Awaiting)
    (present : (advance s actor raw).state.source.waiting = some saved) :
    (advance s actor raw).status = .waiting ∧ saved.site = raw.source.site := by
  by_cases waiting : (advance s actor raw).status = .waiting
  · obtain ⟨_,_,actual,_,atSaved,_,atSite,_⟩ := advance_waiting _ _ _ waiting
    have equal : actual = saved := Option.some.inj (atSaved.symm.trans present)
    exact ⟨waiting,equal ▸ atSite⟩
  · have same := advance_nonwaiting s actor raw waiting
    rw [idle,present] at same
    cases same

-- C cancellation resolves ONLY the original private pending ticket. It does
-- not require invocation or holding permission and does not roll back the owner.
def cancel (s : State p a) (actor : Actor p a) : Outcome p a :=
  match s.source.waiting with
  | none => ⟨s,.failed .awaiting⟩
  | some saved => match CompositionCore.index p saved.entry.ticket.place with
    | none => ⟨s,.failed .rejected⟩
    | some operation =>
        let result := ReferenceSource.cancel s.source actor.member operation actor.principal
        ⟨⟨result.state,s.tags⟩,result.status⟩

theorem cancel_source (s : State p a) (actor : Actor p a) :
    ReferenceSourceTrace.Reached s.source (cancel s actor).state.source := by
  cases waiting : s.source.waiting with
  | none => simp only [cancel,waiting]; exact .refl
  | some saved =>
      cases selected : CompositionCore.index p saved.entry.ticket.place with
      | none => simp only [cancel,waiting,selected]; exact .refl
      | some operation => simp only [cancel,waiting,selected]; exact .step .refl .cancellation

theorem cancel_no_write (s : State p a) (actor : Actor p a) :
    (cancel s actor).state.source.values = s.source.values ∧
    (cancel s actor).state.source.writes = s.source.writes ∧ (cancel s actor).state.tags = s.tags := by
  cases waiting : s.source.waiting with
  | none => simp [cancel,waiting]
  | some saved =>
      cases selected : CompositionCore.index p saved.entry.ticket.place with
      | none => simp [cancel,waiting,selected]
      | some operation =>
          have unchanged := ReferenceSource.cancel_values_writes s.source actor.member operation actor.principal
          exact ⟨by simpa [cancel,waiting,selected] using unchanged.1,
            by simpa [cancel,waiting,selected] using unchanged.2.1,by simp [cancel,waiting,selected]⟩

theorem cancel_clears (s : State p a) (actor : Actor p a) (ready : (cancel s actor).status = .ready) :
    (cancel s actor).state.source.waiting = none := by
  cases waiting : s.source.waiting with
  | none => simp [cancel,waiting] at ready
  | some saved =>
      cases indexed : CompositionCore.index p saved.entry.ticket.place with
      | none => simp [cancel,waiting,indexed] at ready
      | some place =>
          have atSource : (ReferenceSource.cancel s.source actor.member place actor.principal).status = .ready := by
            simpa [cancel,waiting,indexed] using ready
          obtain ⟨_,_,_,_,clear⟩ := ReferenceSource.cancel_success _ _ _ _ atSource
          simpa [cancel,waiting,indexed] using congrArg ReferenceSource.State.waiting clear

def received (s : State p a) (ticket : InvocationBoundary.Ticket) (value : Int) : Option (State p a) :=
  (ReceivedTicket.accept s.source ticket value).map fun source => ⟨source,s.tags⟩

theorem received_source (accepted : received s ticket value = some next) :
    ReferenceSourceTrace.Reached s.source next.source := by
  cases receivedAt : ReceivedTicket.accept s.source ticket value with
  | none => simp [received,receivedAt] at accepted
  | some source =>
      simp only [received,receivedAt,Option.map_some,Option.some.injEq] at accepted
      subst next
      obtain ⟨saved,_,_,valid⟩ := (ReceivedTicket.accept_exact _ _ _ _).mp receivedAt
      have exactResult := ReceivedResult.accept_sound valid
      have path : ReferenceSourceTrace.Reached s.source (ReferenceSource.complete s.source).state := .step .refl .complete
      simpa [exactResult] using path

theorem received_clears (accepted : received s ticket value = some next) : next.source.waiting = none := by
  cases receivedAt : ReceivedTicket.accept s.source ticket value with
  | none => simp [received,receivedAt] at accepted
  | some source =>
      simp only [received,receivedAt,Option.map_some,Option.some.injEq] at accepted
      subst next
      obtain ⟨saved,_,_,valid⟩ := (ReceivedTicket.accept_exact _ _ _ _).mp receivedAt
      obtain ⟨_,_,_,_,_,rfl⟩ := ReceivedResult.accept_parts valid
      rfl

theorem received_environment (accepted : received s ticket value = some next) :
    ∃ saved, s.source.waiting = some saved ∧
      environment next.source.values = (saved.name,.plain (.integer false)) :: environment s.source.values := by
  cases receivedAt : ReceivedTicket.accept s.source ticket value with
  | none => simp [received,receivedAt] at accepted
  | some source =>
      simp only [received,receivedAt,Option.map_some,Option.some.injEq] at accepted
      subst next
      obtain ⟨_,_,_,valid⟩ := (ReceivedTicket.accept_exact _ _ _ _).mp receivedAt
      obtain ⟨saved,_,waiting,_,_,rfl⟩ := ReceivedResult.accept_parts valid
      exact ⟨saved,waiting,by simp [ReferenceSource.write,ReferenceSource.mark,environment,typeOf,SourceTypes.typeOf]⟩

#print axioms select_exact
#print axioms compile_exact
#print axioms advance_basis
#print axioms advance_source
#print axioms waiting_tags
#print axioms alias_intent
#print axioms nonallocation_cannot_retarget
#print axioms advance_waiting
#print axioms alias_machine_identity
#print axioms advance_nonwaiting
#print axioms advance_environment
#print axioms advance_waiting_environment
#print axioms advance_pending_site
#print axioms cancel_source
#print axioms cancel_no_write
#print axioms received_source
#print axioms cancel_clears
#print axioms received_clears
#print axioms received_environment
end MirroreaProofFirst.QualifiedSource
