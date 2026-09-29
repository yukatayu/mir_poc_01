import MixedOwnerProgram
import MixedOwnerSourceInvariant
namespace MirroreaProofFirst.MixedOwnerContinuation
open ReferenceSourceData MixedOwnerProgram

abbrev Cursor := MixedSourceCursor.Cursor Entry
abbrev Outcome (p a : Nat) := MixedSourceCursor.Outcome (MixedOwnerSourceTrace.State p a)
structure Archive (p : Nat) where
 program : Program p
 entries : List Entry
 cursor : Cursor
 status : ReferenceSource.Status
 activation : Nat
 control : Nat
structure Session (p a : Nat) where
 state : MixedOwnerSourceTrace.State p a
 program : Program p
 entries : List Entry
 cursor : Cursor
 status : ReferenceSource.Status
 activation : Nat
 control : Nat
 superseded : List (Archive p)

def archive (s : Session p a) : Archive p :=
 ⟨s.program,s.entries,s.cursor,s.status,s.activation,s.control⟩

def launch (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
 (store : Nat → Option Int) (program : Program p) : Option (Session p a) := do
 let (entries,_) ← MixedOwnerProgram.compile [] program
 let source := MixedOwnerSourceIssue.embed (MixedReferenceSource.initial realm view policy)
 return ⟨MixedOwnerSourceTrace.initial source store,program,entries,⟨[],entries,none⟩,.ready,0,program.control,[]⟩

def advance (s : Session p a) (member : Fin a) (principal : Nat) : Entry → Outcome p a
 | .ordinary item =>
   let r := MixedOwnerSourceIssue.advancePure s.state.source member s.program.place principal item
   ⟨{s.state with source := r.state},r.status⟩
 | .install name control source =>
   if ¬s.control ≤ control then ⟨s.state,.failed .rejected⟩ else
   let r := MixedOwnerSourceDeclaration.declareOwner s.state.source s.program.fields s.program.labels
     control member s.program.place principal name source
   ⟨{s.state with source := r.state},r.status⟩
 | .write name ordinal control source =>
   if ¬s.control ≤ control then ⟨s.state,.failed .rejected⟩ else
   match MixedOwnerSourceTrace.issue s.state s.program.fields s.program.labels member principal
     s.activation ordinal control name source with
   | none => ⟨s.state,.failed .rejected⟩
   | some next => ⟨next,.waiting⟩

-- Owner completion receives only an ACTUAL inbox result. An empty inbox means
-- still waiting; current authorization refusal retains the committed prefix and
-- stopped statement. It never substitutes an integer return value for unit ack.
def complete (s : Session p a) : Outcome p a :=
 match MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting with
 | none =>
   let r := MixedOwnerSourceIssue.completePure s.state.source
   ⟨{s.state with source := r.state},r.status⟩
 | some _ =>
   if s.state.inbox.isEmpty then ⟨s.state,.waiting⟩ else
   match MixedOwnerSourceTrace.receive s.state with
   | none => ⟨s.state,.failed .rejected⟩
   | some next => ⟨next,.ready⟩

-- Retain the arithmetic completion dependency as soon as the source write is
-- attempted, including issue-time failure. Transport, waiting, refusal and
-- replacement cannot clear it. Ordinary/callee/management completion remains
-- an explicit separate integration obligation, not a claimed NI theorem.
def attemptedControl (s : Session p a) : Nat :=
 match s.status,s.cursor.remaining with
 | .ready,.install _ control _::_ => max s.control control
 | .ready,.write _ _ control source::_ =>
   let incoming := max s.control control
   match MixedOwnerSourceCode.compile s.program.fields s.program.labels control source with
   | none => incoming
   | some code => max incoming (MixedOwnerProgram.completion code)
 | _,_ => s.control

def tick (s : Session p a) (member : Fin a) (principal : Nat) : Session p a :=
 let r := MixedSourceCursor.tick ⟨s.state,s.status⟩ s.cursor (advance s member principal) (complete s)
 {s with state := r.outcome.state,status := r.outcome.status,cursor := r.cursor,control := attemptedControl s}

def drive : Nat → Session p a → Fin a → Nat → Session p a
 | 0,s,_,_ => s
 | fuel+1,s,member,principal => drive fuel (tick s member principal) member principal

-- Transport/service are distinct inputs on the SAME session; tick does not
-- secretly execute them, nor does a fake expected trace supply a missing reply.
def transfer (s : Session p a) : Option (Session p a) :=
 (MixedOwnerSourceTrace.transfer s.state).map fun next => {s with state := next}
def service (s : Session p a) (ops : FallibleFlow.Arithmetic) (metadata : Nat → Option (Nat × Nat)) : Session p a :=
 {s with state := MixedOwnerSourceTrace.service ops metadata s.state}
def authorityHead (s : Session p a) (view : WorldProjection.AuthorityView a) : Option (Session p a) :=
 (MixedOwnerSourceTrace.authorityHead s.state view).map fun next => {s with state := next}
def controlInput (s : Session p a) (member : Fin a) (place : Fin p) (principal : Nat) (raw : MixedCompositionCore.Raw) :
 Option (Session p a × Option Nat) :=
 (MixedOwnerSourceTrace.controlInput s.state member place principal raw).map fun (next,key) => ({s with state := next},key)

-- Retain all source, queue, owner history, attempts and allocator state. A new
-- source block cannot erase an outstanding item, box or owner queue.
def drained (s : Session p a) : Bool :=
 s.state.source.waiting.isNone && s.state.source.machine.store.core.pending.isEmpty &&
 s.state.outbox.isEmpty && s.state.inbox.isEmpty && s.state.owner.queued.isNone
def resumedProgram (s : Session p a) (program : Program p) : Program p :=
 {program with control := max s.control program.control}
private def adopt (s : Session p a) (program : Program p) : Option (Session p a) := do
 if !drained s then none else do
 let effective := resumedProgram s program
 let (entries,_) ← MixedOwnerProgram.compile (environment s.state.source.values) effective
 return {s with
   program := effective,entries := entries,cursor := ⟨[],entries,none⟩,
   status := .ready,activation := s.activation+1,control := effective.control,superseded := archive s::s.superseded}
def replaceResidual (s : Session p a) (program : Program p) : Option (Session p a) :=
 match s.status with | .failed _ => adopt s program | _ => none
def continueWith (s : Session p a) (program : Program p) : Option (Session p a) :=
 if s.status = .ready ∧ s.cursor.remaining = [] then adopt s program else none

-- Preservation is about the actual entered transitions, including failures.
theorem advance_rooted (path : MixedOwnerSourceTrace.Rooted root store s.state) :
 MixedOwnerSourceTrace.Rooted root store (advance s member principal item).state := by
 cases item with
 | ordinary item => exact .step path .pureAdvance
 | install name control source =>
   simp only [advance]
   split
   · exact path
   · exact .step path .declaration
 | write name ordinal control source =>
   simp only [advance]
   split
   · exact path
   · cases run : MixedOwnerSourceTrace.issue s.state s.program.fields s.program.labels member principal s.activation ordinal control name source with
     | none => exact path
     | some next => exact .step path (.issue run)

theorem complete_rooted (path : MixedOwnerSourceTrace.Rooted root store s.state) :
 MixedOwnerSourceTrace.Rooted root store (complete s).state := by
 unfold complete
 cases waiting : MixedOwnerSourceIssue.ownerWaiting s.state.source.waiting with
 | none => exact .step path .pureComplete
 | some held =>
   dsimp only
   split
   · exact path
   · cases run : MixedOwnerSourceTrace.receive s.state with
     | none => exact path
     | some next => exact .step path (.receive run)

theorem tick_rooted (path : MixedOwnerSourceTrace.Rooted root store s.state) :
 MixedOwnerSourceTrace.Rooted root store (tick s member principal).state := by
 cases phase : s.status with
 | failed reason => simpa [tick,MixedSourceCursor.tick,phase] using path
 | waiting => simpa only [tick,MixedSourceCursor.tick,phase] using complete_rooted path
 | ready =>
   cases rest : s.cursor.remaining with
   | nil => simpa [tick,MixedSourceCursor.tick,phase,rest] using path
   | cons item rest => simpa only [tick,MixedSourceCursor.tick,phase,rest] using advance_rooted (item:=item) (member:=member) (principal:=principal) path

theorem drive_rooted (path : MixedOwnerSourceTrace.Rooted root store s.state) :
 MixedOwnerSourceTrace.Rooted root store (drive fuel s member principal).state := by
 induction fuel generalizing s with
 | zero => exact path
 | succ fuel ih => exact ih (tick_rooted path)

theorem transfer_rooted (path : MixedOwnerSourceTrace.Rooted root store s.state) (accepted : transfer s = some next) :
 MixedOwnerSourceTrace.Rooted root store next.state := by
 unfold transfer at accepted
 cases run : MixedOwnerSourceTrace.transfer s.state with
 | none => simp [run] at accepted
 | some state => simp only [run,Option.map_some,Option.some.injEq] at accepted; subst next; exact .step path (.transfer run)

theorem service_rooted (path : MixedOwnerSourceTrace.Rooted root store s.state) :
 MixedOwnerSourceTrace.Rooted root store (service s ops metadata).state := .step path .service

private theorem adopt_keeps (accepted : adopt s program = some next) :
 next.state = s.state ∧ next.program = resumedProgram s program ∧ next.entries = next.cursor.remaining ∧
 next.cursor.completed = [] ∧ next.cursor.stopped = none ∧ next.status = .ready ∧
 next.activation = s.activation+1 ∧ next.superseded = archive s::s.superseded := by
 unfold adopt at accepted
 split at accepted
 · cases accepted
 · cases compiled : MixedOwnerProgram.compile (environment s.state.source.values) (resumedProgram s program) with
   | none => simp [compiled] at accepted
   | some pair =>
     obtain ⟨entries,env⟩ := pair
     simp only [compiled,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
     subst next
     exact ⟨rfl,rfl,rfl,rfl,rfl,rfl,rfl,rfl⟩

theorem continued_keeps (accepted : continueWith s program = some next) :
 next.state = s.state ∧ next.activation = s.activation+1 ∧ next.superseded = archive s::s.superseded := by
 unfold continueWith at accepted
 split at accepted
 · have h := adopt_keeps accepted; exact ⟨h.1,h.2.2.2.2.2.2.1,h.2.2.2.2.2.2.2⟩
 · cases accepted

theorem replaced_keeps (accepted : replaceResidual s program = some next) :
 next.state = s.state ∧ next.activation = s.activation+1 ∧ next.superseded = archive s::s.superseded := by
 unfold replaceResidual at accepted
 split at accepted
 · have h := adopt_keeps accepted; exact ⟨h.1,h.2.2.2.2.2.2.1,h.2.2.2.2.2.2.2⟩
 · cases accepted

theorem launch_parts {p a : Nat} {program : Program p} {s : Session p a} {realm : Nat} {view : WorldProjection.AuthorityView a} {policy : Nat → CurrentUse.Policy} {store : Nat → Option Int} (accepted : launch realm view policy store program = some s) :
 ∃ entries env,
 Lowers p program.fields program.labels program.control 0 (MixedOwnerProgram.reserved [] program) [] program.items entries env ∧
 s = ⟨MixedOwnerSourceTrace.initial (MixedOwnerSourceIssue.embed (MixedReferenceSource.initial realm view policy)) store,
       program,entries,⟨[],entries,none⟩,.ready,0,program.control,[]⟩ := by
 unfold launch at accepted
 cases compiled : MixedOwnerProgram.compile [] program with
 | none => simp [compiled] at accepted
 | some pair =>
   obtain ⟨entries,env⟩ := pair
   simp only [compiled,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
   exact ⟨entries,env,lower_sound compiled,accepted.symm⟩

theorem launch_exact {p a : Nat} {program : Program p} {view : WorldProjection.AuthorityView a} : (∃ s, launch realm view policy store program = some s) ↔
 ∃ entries env, Lowers p program.fields program.labels program.control 0
   (MixedOwnerProgram.reserved [] program) [] program.items entries env := by
 constructor
 · rintro ⟨s,accepted⟩; obtain ⟨entries,env,typed,_⟩ := launch_parts accepted; exact ⟨entries,env,typed⟩
 · rintro ⟨entries,env,typed⟩
   have compiled : MixedOwnerProgram.compile [] program = some (entries,env) := lower_complete typed
   simp only [launch,compiled,Option.bind_eq_bind,Option.bind_some,Option.pure_def]; exact ⟨_,rfl⟩

theorem authority_rooted (path : MixedOwnerSourceTrace.Rooted root store s.state)
 (accepted : authorityHead s view = some next) : MixedOwnerSourceTrace.Rooted root store next.state := by
 unfold authorityHead at accepted
 cases run : MixedOwnerSourceTrace.authorityHead s.state view with
 | none => simp [run] at accepted
 | some state => simp only [run,Option.map_some,Option.some.injEq] at accepted; subst next; exact .step path (.authority run)

theorem control_rooted (path : MixedOwnerSourceTrace.Rooted root store s.state)
 (accepted : controlInput s member place principal raw = some (next,key)) : MixedOwnerSourceTrace.Rooted root store next.state := by
 unfold controlInput at accepted
 cases run : MixedOwnerSourceTrace.controlInput s.state member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨state,key⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   exact .step path (.control run)

def cancel (s : Session p a) (member : Fin a) (principal : Nat) : Session p a :=
 let r := MixedOwnerSourceIssue.cancelPure s.state.source member s.program.place principal
 if r.status = .ready then {s with state := {s.state with source := r.state},status := .failed .cancelled} else s

theorem cancel_rooted (path : MixedOwnerSourceTrace.Rooted root store s.state) :
 MixedOwnerSourceTrace.Rooted root store (cancel s member principal).state := by
 simp only [cancel]
 split
 · exact .step path .pureCancel
 · exact path

-- Admitted sessions retain the original fresh source root across all program
-- replacements. Raw record construction and populated-image import are excluded.
inductive Rooted (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
 (store : Nat → Option Int) : Session p a → Prop where
 | launch : launch realm view policy store program = some s → Rooted realm view policy store s
 | tick : Rooted realm view policy store s → Rooted realm view policy store (tick s member principal)
 | transfer : Rooted realm view policy store s → transfer s = some next → Rooted realm view policy store next
 | service : Rooted realm view policy store s → Rooted realm view policy store (service s ops metadata)
 | authority : Rooted realm view policy store s → authorityHead s nextView = some next → Rooted realm view policy store next
 | control : Rooted realm view policy store s → controlInput s member place principal raw = some (next,key) → Rooted realm view policy store next
 | cancel : Rooted realm view policy store s → Rooted realm view policy store (cancel s member principal)
 | continued : Rooted realm view policy store s → continueWith s program = some next → Rooted realm view policy store next
 | replaced : Rooted realm view policy store s → replaceResidual s program = some next → Rooted realm view policy store next

theorem rooted_source (path : Rooted realm view policy store s) :
 MixedOwnerSourceTrace.Rooted (MixedOwnerSourceIssue.embed (MixedReferenceSource.initial realm view policy)) store s.state := by
 induction path with
 | launch accepted => obtain ⟨_,_,_,rfl⟩ := launch_parts accepted; exact .initial
 | tick _ ih => exact tick_rooted ih
 | transfer _ accepted ih => exact transfer_rooted ih accepted
 | service _ ih => exact service_rooted ih
 | authority _ accepted ih => exact authority_rooted ih accepted
 | control _ accepted ih => exact control_rooted ih accepted
 | cancel _ ih => exact cancel_rooted ih
 | continued _ accepted ih => rw [(continued_keeps accepted).1]; exact ih
 | replaced _ accepted ih => rw [(replaced_keeps accepted).1]; exact ih

theorem rooted_joint {p a : Nat} {s : Session p a} {realm : Nat} {view : WorldProjection.AuthorityView a} {policy : Nat → CurrentUse.Policy} {store : Nat → Option Int} (path : Rooted realm view policy store s) :
 MixedOwnerSourceInvariant.Valid s.state.source ∧ MixedOwnerSourceInvariant.OwnerAgrees s.state.source ∧
 MixedOwnerAttemptQueue.Invariant s.state.owner ∧ MixedOwnerSourceTrace.Provenance s.state := by
 have rootValid : MixedOwnerSourceInvariant.Valid
    (MixedOwnerSourceIssue.embed (MixedReferenceSource.initial (p:=p) realm view policy)) :=
   ⟨MixedReferenceExecution.initial_invariant _ _ _,MixedReferenceAllocation.initial_below _ _ _,by
     simp [MixedReferenceSource.PendingAgrees,MixedOwnerSourceIssue.base,MixedOwnerSourceIssue.embed,
       MixedReferenceSource.initial,MixedOwnerSourceIssue.pureWaiting,MixedReferenceExecution.initial]⟩
 have rootAgrees : MixedOwnerSourceInvariant.OwnerAgrees
    (MixedOwnerSourceIssue.embed (MixedReferenceSource.initial (p:=p) realm view policy)) := by
   intro waiting impossible; cases impossible
 have trace := rooted_source path
 have joint := MixedOwnerSourceInvariant.rooted_joint rootValid rootAgrees trace
 exact ⟨joint.1,joint.2,MixedOwnerSourceTrace.rooted_owner_invariant trace,MixedOwnerSourceTrace.rooted_provenance trace⟩

theorem attempted_control_monotone : s.control ≤ attemptedControl s := by
 unfold attemptedControl
 split
 · exact Nat.le_max_left _ _
 · split
   · exact Nat.le_max_left _ _
   · exact Nat.le_trans (Nat.le_max_left _ _) (Nat.le_max_left _ _)
 · exact Nat.le_refl _

theorem tick_control_monotone : s.control ≤ (tick s member principal).control :=
 attempted_control_monotone

theorem drive_control_monotone : s.control ≤ (drive fuel s member principal).control := by
 induction fuel generalizing s with
 | zero => exact Nat.le_refl _
 | succ fuel ih => exact Nat.le_trans tick_control_monotone ih

-- Raise on the attempt, not only on success. A refused issue cannot be used as
-- a branch that resets pc before replacing the failed residual program.
theorem write_completion_retained {control : Nat}
 (ready : s.status = .ready)
 (head : s.cursor.remaining = .write name ordinal control source::rest)
 (compiled : MixedOwnerSourceCode.compile s.program.fields s.program.labels control source = some code) :
 MixedOwnerProgram.completion code ≤ (tick s member principal).control := by
 simp only [tick,attemptedControl,ready,head,compiled]
 exact Nat.le_max_right _ _

theorem launch_control (accepted : launch realm view policy store program = some s) :
 s.control = program.control := by
 obtain ⟨_,_,_,rfl⟩ := launch_parts accepted
 rfl

private theorem adopt_control (accepted : adopt s program = some next) :
 next.control = max s.control program.control := by
 unfold adopt at accepted
 split at accepted
 · cases accepted
 · cases compiled : MixedOwnerProgram.compile (environment s.state.source.values) (resumedProgram s program) with
   | none => simp [compiled] at accepted
   | some pair =>
     obtain ⟨entries,env⟩ := pair
     simp only [compiled,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
     subst next
     rfl

theorem continued_control (accepted : continueWith s program = some next) :
 next.control = max s.control program.control := by
 unfold continueWith at accepted
 split at accepted
 · exact adopt_control accepted
 · cases accepted

theorem replaced_control (accepted : replaceResidual s program = some next) :
 next.control = max s.control program.control := by
 unfold replaceResidual at accepted
 split at accepted
 · exact adopt_control accepted
 · cases accepted

theorem transfer_control (accepted : transfer s = some next) : next.control = s.control := by
 unfold transfer at accepted
 cases h : MixedOwnerSourceTrace.transfer s.state <;> simp [h] at accepted
 subst next
 rfl

theorem service_control : (service s ops metadata).control = s.control := rfl

theorem authority_control (accepted : authorityHead s view = some next) : next.control = s.control := by
 unfold authorityHead at accepted
 cases h : MixedOwnerSourceTrace.authorityHead s.state view <;> simp [h] at accepted
 subst next
 rfl

theorem control_input_control (accepted : controlInput s member place principal raw = some (next,key)) :
 next.control = s.control := by
 unfold controlInput at accepted
 cases h : MixedOwnerSourceTrace.controlInput s.state member place principal raw with
 | none => simp [h] at accepted
 | some pair =>
   obtain ⟨state,key⟩ := pair
   simp only [h,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   rfl

theorem cancel_control : (cancel s member principal).control = s.control := by
 simp only [cancel]
 split <;> rfl

#print axioms attempted_control_monotone
#print axioms tick_control_monotone
#print axioms drive_control_monotone
#print axioms write_completion_retained
#print axioms launch_control
#print axioms continued_control
#print axioms replaced_control
#print axioms transfer_control
#print axioms service_control
#print axioms authority_control
#print axioms control_input_control
#print axioms cancel_control
#print axioms launch_parts
#print axioms launch_exact
#print axioms authority_rooted
#print axioms control_rooted
#print axioms cancel_rooted
#print axioms rooted_source
#print axioms rooted_joint
#print axioms advance_rooted
#print axioms complete_rooted
#print axioms tick_rooted
#print axioms drive_rooted
#print axioms transfer_rooted
#print axioms service_rooted
#print axioms continued_keeps
#print axioms replaced_keeps
end MirroreaProofFirst.MixedOwnerContinuation
