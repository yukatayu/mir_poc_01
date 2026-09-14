import MirroreaProofFirstQualifiedSource

namespace MirroreaProofFirst.QualifiedSession
open QualifiedSource ReferenceSourceData

structure Program (p : Nat) where
  caller : Fin p
  items : List Raw
  deriving DecidableEq, Repr
structure Archive (p : Nat) where
  program : Program p
  completed : List Raw
  stopped : Option Raw
  remaining : List Raw
  status : ReferenceSource.Status
  deriving DecidableEq, Repr
structure Session (p a : Nat) where
  state : QualifiedSource.State p a
  member : Fin a
  principal : Nat
  program : Program p
  completed : List Raw
  stopped : Option Raw
  remaining : List Raw
  status : ReferenceSource.Status
  superseded : List (Archive p)

def actor (s : Session p a) : Actor p a := ⟨s.program.caller,s.member,s.principal⟩
def archive (s : Session p a) : Archive p := ⟨s.program,s.completed,s.stopped,s.remaining,s.status⟩

def Annotation (p : Nat) (raw : Raw) : Prop :=
  match raw.allocation with
  | none => True
  | some n => IsAllocation raw.source.statement ∧ n < p
def annotationCheck (p : Nat) (raw : Raw) : Bool :=
  match raw.allocation with
  | none => true
  | some n => isAllocation raw.source.statement && decide (n < p)
theorem annotation_exact (p : Nat) (raw : Raw) : annotationCheck p raw = true ↔ Annotation p raw := by
  cases h : raw.allocation <;> simp [annotationCheck,Annotation,h,allocation_exact]

def ProgramTyped (env : Environment) (program : Program p) (next : Environment) : Prop :=
  (∀ raw ∈ program.items, Annotation p raw) ∧
    ReferenceSourceData.ProgramTyped p env (program.items.map Raw.source) next

def checkProgram (env : Environment) (program : Program p) : Option Environment :=
  if program.items.all (annotationCheck p) then
    ReferenceSourceData.checkProgram p env (program.items.map Raw.source)
  else none

theorem program_exact (env : Environment) (program : Program p) (next : Environment) :
    checkProgram env program = some next ↔ ProgramTyped env program next := by
  by_cases allowed : program.items.all (annotationCheck p) = true
  · have annotations : ∀ raw ∈ program.items, Annotation p raw := by
      simpa [List.all_eq_true,annotation_exact] using allowed
    simp only [checkProgram,allowed,ite_true,ProgramTyped,ReferenceSourceData.program_exact]
    exact ⟨fun h => ⟨annotations,h⟩,And.right⟩
  · have rejected : ¬ ∀ raw ∈ program.items, Annotation p raw := by
      simpa [List.all_eq_true,annotation_exact] using allowed
    simp [checkProgram,allowed,ProgramTyped,rejected]

-- A fresh logical activation records caller/member/principal once. Authentic
-- ownership of this activation and namespace is not produced by this constructor.
def launch (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (member : Fin a) (principal : Nat) (program : Program p) : Option (Session p a) := do
  let _ ← checkProgram [] program
  return ⟨⟨ReferenceSource.initial realm view policy,[]⟩,member,principal,program,[],none,program.items,.ready,[]⟩

-- There is deliberately no waiting evaluator branch. Source completion is an
-- accepted receipt; owner computation/preparation alone does not complete a line.
def tick (s : Session p a) : Session p a :=
  if s.status = .ready then match s.remaining with
    | [] => s
    | item :: rest =>
        let result := QualifiedSource.advance s.state (actor s) item
        {s with
          state := result.state,remaining := rest,status := result.status,
          completed := if result.status = .ready then s.completed ++ [item] else s.completed,
          stopped := if result.status = .ready then none else some item}
  else s

def receive (s : Session p a) (ticket : InvocationBoundary.Ticket) (value : Int) : Option (Session p a) :=
  if s.status = .waiting then do
    let next ← QualifiedSource.received s.state ticket value
    return {s with state := next,status := .ready,completed := s.completed ++ s.stopped.toList,stopped := none}
  else none

def cancel (s : Session p a) : Session p a :=
  let result := QualifiedSource.cancel s.state (actor s)
  if result.status = .ready then {s with state := result.state,status := .failed .cancelled} else s

-- This actor's private environment cannot move to another caller via adoption.
-- A new independently admitted source activation is a separate boundary.
def adopt (s : Session p a) (program : Program p) : Option (Session p a) := do
  if program.caller != s.program.caller || s.state.source.waiting.isSome then none else do
    let _ ← checkProgram (environment s.state.source.values) program
    return {s with
      program := program,completed := [],stopped := none,remaining := program.items,
      status := .ready,superseded := archive s :: s.superseded}

def replaceResidual (s : Session p a) (program : Program p) : Option (Session p a) :=
  match s.status with | .failed _ => adopt s program | _ => none
def continueWith (s : Session p a) (program : Program p) : Option (Session p a) :=
  if s.status = .ready ∧ s.remaining = [] then adopt s program else none

def authorityHead (s : Session p a) (view : WorldProjection.AuthorityView a) : Option (Session p a) := do
  let next ← ReferenceAuthority.install s.state.source view
  return {s with state := ⟨next,s.state.tags⟩}
def controlInput (s : Session p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) : Option (Session p a × Option Nat) := do
  let (next,created) ← ReferenceSource.controlInput s.state.source member place principal raw
  return ({s with state := ⟨next,s.state.tags⟩},created)

def Partition (program : Program p) (completed : List Raw) (stopped : Option Raw) (remaining : List Raw) : Prop :=
  program.items = completed ++ stopped.toList ++ remaining
def ArchiveValid (old : Archive p) : Prop :=
  Partition old.program old.completed old.stopped old.remaining ∧ (old.status = .ready → old.stopped = none)
def Metadata (s : Session p a) : Prop :=
  Partition s.program s.completed s.stopped s.remaining ∧ (s.status = .ready → s.stopped = none) ∧
    ∀ old ∈ s.superseded, ArchiveValid old

theorem tick_metadata (valid : Metadata s) : Metadata (tick s) := by
  obtain ⟨partition,ready,archived⟩ := valid
  by_cases phase : s.status = .ready
  · have noStopped := ready phase
    cases remaining : s.remaining with
    | nil => simpa [tick,phase,remaining] using (show Metadata s from ⟨partition,ready,archived⟩)
    | cons item rest =>
        simp only [tick,phase,ite_true,remaining]
        by_cases finished : (QualifiedSource.advance s.state (actor s) item).status = .ready
        · simp only [finished,ite_true]
          exact ⟨by simpa [Partition,noStopped,remaining,List.append_assoc] using partition,fun _ => rfl,archived⟩
        · simp only [finished,ite_false]
          exact ⟨by simpa [Partition,noStopped,remaining,List.append_assoc] using partition,
            fun impossible => False.elim (finished impossible),archived⟩
  · simpa [tick,phase] using (show Metadata s from ⟨partition,ready,archived⟩)

theorem receive_parts (accepted : receive s ticket value = some next) :
    s.status = .waiting ∧ ∃ state, QualifiedSource.received s.state ticket value = some state ∧
      next = {s with state := state,status := .ready,completed := s.completed ++ s.stopped.toList,stopped := none} := by
  unfold receive at accepted
  split at accepted
  · rename_i waiting
    cases got : QualifiedSource.received s.state ticket value with
    | none => simp [got] at accepted
    | some state => exact ⟨waiting,state,rfl,by simpa [got] using accepted.symm⟩
  · cases accepted

theorem receive_metadata (valid : Metadata s) (accepted : receive s ticket value = some next) : Metadata next := by
  obtain ⟨_,_,_,rfl⟩ := receive_parts accepted
  exact ⟨by simpa [Partition,List.append_assoc] using valid.1,fun _ => rfl,valid.2.2⟩

theorem cancel_metadata (valid : Metadata s) : Metadata (cancel s) := by
  simp only [cancel]
  split
  · refine ⟨valid.1,?_,valid.2.2⟩
    intro h
    cases h
  · exact valid

theorem adopt_parts (accepted : adopt s program = some next) :
    program.caller = s.program.caller ∧ s.state.source.waiting = none ∧
    next = {s with
      program := program,completed := [],stopped := none,remaining := program.items,
      status := .ready,superseded := archive s :: s.superseded} := by
  unfold adopt at accepted
  split at accepted
  · cases accepted
  · rename_i permitted
    have guard : program.caller = s.program.caller ∧ s.state.source.waiting = none := by
      simpa using permitted
    cases checked : checkProgram (environment s.state.source.values) program with
    | none => simp [checked] at accepted
    | some env => exact ⟨guard.1,guard.2,by simpa [checked] using accepted.symm⟩

theorem adopt_metadata (valid : Metadata s) (accepted : adopt s program = some next) : Metadata next := by
  obtain ⟨_,_,rfl⟩ := adopt_parts accepted
  refine ⟨by simp [Partition],fun _ => rfl,?_⟩
  intro old present
  rcases List.mem_cons.mp present with rfl | past
  · exact ⟨valid.1,valid.2.1⟩
  · exact valid.2.2 old past

theorem tick_source (s : Session p a) : ReferenceSourceTrace.Reached s.state.source (tick s).state.source := by
  by_cases ready : s.status = .ready
  · cases remaining : s.remaining with
    | nil => simp only [tick,ready,ite_true,remaining]; exact .refl
    | cons item rest => simp only [tick,ready,ite_true,remaining]; exact QualifiedSource.advance_source _ _ _
  · simp only [tick,ready,ite_false]; exact .refl

theorem waiting_tick_unchanged (waiting : s.status = .waiting) : tick s = s := by simp [tick,waiting]

-- Fixed logical identity preservation. Physical caller custody and admission
-- remain separate obligations; this theorem is not an authenticated launch.
theorem tick_actor (s : Session p a) : actor (tick s) = actor s := by
  unfold tick
  split
  · split <;> rfl
  · rfl
theorem receive_actor (accepted : receive s ticket value = some next) : actor next = actor s := by
  obtain ⟨_,_,_,rfl⟩ := receive_parts accepted
  rfl
theorem cancel_actor (s : Session p a) : actor (cancel s) = actor s := by
  simp only [cancel]
  split <;> rfl
theorem adopt_actor (accepted : adopt s program = some next) : actor next = actor s := by
  obtain ⟨same,_,rfl⟩ := adopt_parts accepted
  simp [actor,same]

theorem launch_state (accepted : launch realm view policy member principal program = some s) :
    s = ⟨⟨ReferenceSource.initial realm view policy,[]⟩,member,principal,program,[],none,program.items,.ready,[]⟩ := by
  unfold launch at accepted
  cases checked : checkProgram [] program with
  | none => simp [checked] at accepted
  | some env => simpa [checked] using accepted.symm

theorem head_parts (accepted : authorityHead s view = some next) :
    ReferenceAuthority.Successor s.state.source.machine.store.core.system.view view ∧
    next = {s with state := ⟨ReferenceSource.authorityHead s.state.source view,s.state.tags⟩} := by
  unfold authorityHead at accepted
  cases installed : ReferenceAuthority.install s.state.source view with
  | none => simp [installed] at accepted
  | some state =>
      obtain ⟨valid,rfl⟩ := ReferenceAuthority.install_parts _ _ _ installed
      exact ⟨valid,by simpa [installed] using accepted.symm⟩

theorem control_parts (accepted : controlInput s member place principal raw = some (next,created)) :
    ∃ state, ReferenceSource.controlInput s.state.source member place principal raw = some (state,created) ∧
      next = {s with state := ⟨state,s.state.tags⟩} := by
  unfold controlInput at accepted
  cases run : ReferenceSource.controlInput s.state.source member place principal raw with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨state,key⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      exact ⟨state,rfl,rfl⟩

-- Complete admitted entry set for this finite candidate. No populated launch,
-- raw Core execution, tag-table update, waiting tick or free actor replacement.
inductive Step : Session p a → Session p a → Prop where
  | tick : Step s (tick s)
  | receive : receive s ticket value = some next → Step s next
  | cancel : Step s (cancel s)
  | head : authorityHead s view = some next → Step s next
  | control : controlInput s member place principal raw = some (next,created) → Step s next
  | replace : replaceResidual s program = some next → Step s next
  | continueWith : continueWith s program = some next → Step s next

theorem replacement_adopts (accepted : replaceResidual s program = some next) : adopt s program = some next := by
  cases phase : s.status <;> simp_all [replaceResidual]
theorem continuation_adopts (accepted : continueWith s program = some next) : adopt s program = some next := by
  unfold continueWith at accepted
  split at accepted
  · exact accepted
  · cases accepted

theorem step_metadata (valid : Metadata s) (step : Step s next) : Metadata next := by
  cases step with
  | tick => exact tick_metadata valid
  | receive accepted => exact receive_metadata valid accepted
  | cancel => exact cancel_metadata valid
  | head accepted => rw [(head_parts accepted).2]; exact valid
  | control accepted => obtain ⟨_,_,rfl⟩ := control_parts accepted; exact valid
  | replace accepted => exact adopt_metadata valid (replacement_adopts accepted)
  | continueWith accepted => exact adopt_metadata valid (continuation_adopts accepted)

theorem step_source (step : Step s next) : ReferenceSourceTrace.Reached s.state.source next.state.source := by
  cases step with
  | tick => exact tick_source _
  | receive accepted => obtain ⟨_,_,received,rfl⟩ := receive_parts accepted; exact QualifiedSource.received_source received
  | cancel =>
      simp only [cancel]
      split
      · exact QualifiedSource.cancel_source _ _
      · exact .refl
  | head accepted => rw [(head_parts accepted).2]; exact .step .refl .head
  | control accepted => obtain ⟨_,committed,rfl⟩ := control_parts accepted; exact .step .refl (.control committed)
  | replace accepted => rw [(adopt_parts (replacement_adopts accepted)).2.2]; exact .refl
  | continueWith accepted => rw [(adopt_parts (continuation_adopts accepted)).2.2]; exact .refl

theorem step_actor (step : Step s next) : actor next = actor s := by
  cases step with
  | tick => exact tick_actor _
  | receive accepted => exact receive_actor accepted
  | cancel => exact cancel_actor _
  | head accepted => rw [(head_parts accepted).2]; rfl
  | control accepted => obtain ⟨_,_,rfl⟩ := control_parts accepted; rfl
  | replace accepted => exact adopt_actor (replacement_adopts accepted)
  | continueWith accepted => exact adopt_actor (continuation_adopts accepted)

inductive Rooted (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (identity : Actor p a) : Session p a → Prop where
  | launch : launch realm view policy member principal program = some s →
      identity = ⟨program.caller,member,principal⟩ → Rooted realm view policy identity s
  | step : Rooted realm view policy identity s → Step s next → Rooted realm view policy identity next

theorem rooted_invariants (root : Rooted realm view policy identity s) :
    Metadata s ∧ ReferenceSourceTrace.Rooted realm view policy s.state.source ∧ actor s = identity := by
  induction root with
  | launch accepted same =>
      rw [launch_state accepted]
      exact ⟨by simp [Metadata,Partition],.refl,same.symm⟩
  | step previous step ih =>
      exact ⟨step_metadata ih.1 step,ReferenceSourceTrace.trans _ _ _ ih.2.1 (step_source step),
        (step_actor step).trans ih.2.2⟩

theorem rooted_protection (root : Rooted realm view policy identity s) :
    ReferenceExecution.Invariant s.state.source.machine ∧ ReferenceSource.Allocated s.state.source ∧
    ReferenceSource.PendingAgrees s.state.source ∧ ReferenceSourceProvenance.Invariant s.state.source := by
  have source := (rooted_invariants root).2.1
  have machine := ReferenceSourceTrace.rooted_invariants _ _ _ _ source
  exact ⟨machine.1,machine.2,ReferenceSourceTrace.rooted_pending _ _ _ _ source,
    ReferenceSourceProvenance.rooted_invariant _ _ _ _ source⟩

-- Pending-site agreement is separate from the source list partition: a
-- preserved partition alone could retain the wrong source line for a reply.
def Continuing (s : Session p a) : Prop :=
  (s.status = .ready → s.state.source.waiting = none) ∧
  (s.status = .waiting → s.state.source.waiting.isSome = true) ∧
  ∀ saved, s.state.source.waiting = some saved →
    ∃ item, s.stopped = some item ∧ saved.site = item.source.site

theorem tick_continuing (valid : Continuing s) : Continuing (tick s) := by
  by_cases phase : s.status = .ready
  · have idle := valid.1 phase
    cases remaining : s.remaining with
    | nil => simpa [tick,phase,remaining] using valid
    | cons item rest =>
        simp only [tick,phase,ite_true,remaining]
        refine ⟨?_,?_,?_⟩
        · intro ready
          dsimp only at ready
          have notWaiting : (QualifiedSource.advance s.state (actor s) item).status ≠ .waiting := by simp [ready]
          exact (QualifiedSource.advance_nonwaiting _ _ _ notWaiting).trans idle
        · intro waiting
          obtain ⟨_,_,saved,_,atSaved,_,_,_⟩ := QualifiedSource.advance_waiting _ _ _ waiting
          simp [atSaved]
        · intro saved present
          obtain ⟨waiting,site⟩ := QualifiedSource.advance_pending_site _ _ _ idle saved present
          exact ⟨item,by simp [waiting],site⟩
  · simpa [tick,phase] using valid

theorem receive_continuing (accepted : receive s ticket value = some next) : Continuing next := by
  obtain ⟨_,state,received,rfl⟩ := receive_parts accepted
  have empty := QualifiedSource.received_clears received
  simp [Continuing,empty]

theorem cancel_continuing (valid : Continuing s) : Continuing (cancel s) := by
  simp only [cancel]
  split
  · rename_i ready
    have empty := QualifiedSource.cancel_clears _ _ ready
    simp [Continuing,empty]
  · exact valid

theorem adopt_continuing (accepted : adopt s program = some next) : Continuing next := by
  obtain ⟨_,empty,rfl⟩ := adopt_parts accepted
  simp [Continuing,empty]

theorem step_continuing (valid : Continuing s) (step : Step s next) : Continuing next := by
  cases step with
  | tick => exact tick_continuing valid
  | receive accepted => exact receive_continuing accepted
  | cancel => exact cancel_continuing valid
  | head accepted => rw [(head_parts accepted).2]; exact valid
  | control accepted =>
      obtain ⟨state,committed,rfl⟩ := control_parts accepted
      have equal := (ReferenceSource.controlInput_projects _ _ _ _ _ _ committed).2.1
      dsimp only at equal
      simpa only [Continuing,equal] using valid
  | replace accepted => exact adopt_continuing (replacement_adopts accepted)
  | continueWith accepted => exact adopt_continuing (continuation_adopts accepted)

theorem rooted_continuing (root : Rooted realm view policy identity s) : Continuing s := by
  induction root with
  | launch accepted _ => rw [launch_state accepted]; simp [Continuing,ReferenceSource.initial]
  | step previous step ih => exact step_continuing ih step

-- The residual is typed against actual values when ready, and against the
-- fresh saved result binder only while waiting. The binder is not installed
-- in actual values before accepted receipt. Failed residuals require explicit
-- checked replacement, rather than an implicit restart or environment reset.
def SuffixTyped (p : Nat) (env : Environment) (items : List Raw) : Prop :=
  ∃ output, (∀ raw ∈ items, Annotation p raw) ∧
    ReferenceSourceData.ProgramTyped p env (items.map Raw.source) output

def ResidualTyped (s : Session p a) : Prop :=
  match s.status with
  | .ready => SuffixTyped p (environment s.state.source.values) s.remaining
  | .waiting => ∃ saved, s.state.source.waiting = some saved ∧
      SuffixTyped p ((saved.name,.plain (.integer false)) :: environment s.state.source.values) s.remaining
  | .failed _ => True

theorem tick_residual {s : Session p a} (valid : ResidualTyped s) : ResidualTyped (tick s) := by
  by_cases phase : s.status = .ready
  · have before : SuffixTyped p (environment s.state.source.values) s.remaining := by
      simpa [ResidualTyped,phase] using valid
    cases remaining : s.remaining with
    | nil => simpa [tick,phase,remaining] using valid
    | cons item rest =>
        obtain ⟨output,annotations,typed⟩ := before
        rw [remaining,List.map_cons] at typed
        cases typed with
        | cons head tail =>
            have suffix : ∀ raw ∈ rest, Annotation p raw := by
              intro raw present
              exact annotations raw (by simp [remaining,present])
            simp only [tick,phase,ite_true,remaining,ResidualTyped]
            cases result : (QualifiedSource.advance s.state (actor s) item).status with
            | ready =>
                have envAt := QualifiedSource.advance_environment _ _ _ _ head result
                exact ⟨output,suffix,by rw [envAt]; exact tail⟩
            | waiting =>
                obtain ⟨same,saved,present,fresh,envAt⟩ :=
                  QualifiedSource.advance_waiting_environment _ _ _ _ head result
                exact ⟨saved,present,output,suffix,by rw [same,← envAt]; exact tail⟩
            | failed reason => trivial
  · simpa [tick,phase] using valid

theorem receive_residual {s : Session p a} (valid : ResidualTyped s) (accepted : receive s ticket value = some next) :
    ResidualTyped next := by
  obtain ⟨phase,state,received,rfl⟩ := receive_parts accepted
  obtain ⟨old,oldAt,typed⟩ := (show ∃ saved, s.state.source.waiting = some saved ∧
      SuffixTyped p ((saved.name,.plain (.integer false)) :: environment s.state.source.values) s.remaining from
    by simpa [ResidualTyped,phase] using valid)
  obtain ⟨saved,present,envAt⟩ := QualifiedSource.received_environment received
  have same : saved = old := Option.some.inj (present.symm.trans oldAt)
  simp only [ResidualTyped]
  rw [envAt,same]
  exact typed

theorem cancel_residual (valid : ResidualTyped s) : ResidualTyped (cancel s) := by
  simp only [cancel]
  split
  · trivial
  · exact valid

theorem adopt_residual (accepted : adopt s program = some next) : ResidualTyped next := by
  unfold adopt at accepted
  split at accepted
  · cases accepted
  · cases checked : checkProgram (environment s.state.source.values) program with
    | none => simp [checked] at accepted
    | some env =>
        have atNext : next = {s with
            program := program,completed := [],stopped := none,remaining := program.items,
            status := .ready,superseded := archive s :: s.superseded} := by
          simpa [checked] using accepted.symm
        rw [atNext]
        exact ⟨env,(program_exact _ _ _).mp checked⟩

theorem step_residual (valid : ResidualTyped s) (step : Step s next) : ResidualTyped next := by
  cases step with
  | tick => exact tick_residual valid
  | receive accepted => exact receive_residual valid accepted
  | cancel => exact cancel_residual valid
  | head accepted => rw [(head_parts accepted).2]; exact valid
  | control accepted =>
      obtain ⟨state,committed,rfl⟩ := control_parts accepted
      have parts := (ReferenceSource.controlInput_projects _ _ _ _ _ _ committed).2
      dsimp only at parts
      simpa only [ResidualTyped,parts.1,parts.2.1] using valid
  | replace accepted => exact adopt_residual (replacement_adopts accepted)
  | continueWith accepted => exact adopt_residual (continuation_adopts accepted)

theorem launch_residual (accepted : launch realm view policy member principal program = some s) : ResidualTyped s := by
  have checked : ∃ env, checkProgram [] program = some env := by
    unfold launch at accepted
    cases result : checkProgram [] program with
    | none => simp [result] at accepted
    | some env => exact ⟨env,rfl⟩
  obtain ⟨env,checked⟩ := checked
  rw [launch_state accepted]
  exact ⟨env,(program_exact _ _ _).mp checked⟩

theorem rooted_residual (root : Rooted realm view policy identity s) : ResidualTyped s := by
  induction root with
  | launch accepted _ => exact launch_residual accepted
  | step previous step ih => exact step_residual ih step

theorem rooted_fresh (root : Rooted realm view policy identity s) :
    ReferenceSourceTyping.Unique s.state.source ∧ ReferenceSourceTyping.WaitingFresh s.state.source :=
  ReferenceSourceTrace.rooted_names _ _ _ _ (rooted_invariants root).2.1

#print axioms program_exact
#print axioms tick_metadata
#print axioms receive_metadata
#print axioms cancel_metadata
#print axioms adopt_metadata
#print axioms tick_source
#print axioms waiting_tick_unchanged
#print axioms tick_actor
#print axioms receive_actor
#print axioms cancel_actor
#print axioms adopt_actor
#print axioms step_metadata
#print axioms step_source
#print axioms step_actor
#print axioms rooted_invariants
#print axioms rooted_protection
#print axioms step_continuing
#print axioms rooted_continuing
#print axioms tick_residual
#print axioms receive_residual
#print axioms step_residual
#print axioms rooted_residual
#print axioms rooted_fresh
end MirroreaProofFirst.QualifiedSession
