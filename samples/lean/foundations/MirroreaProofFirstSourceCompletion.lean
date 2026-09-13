import MirroreaProofFirstSourceFrame
import MirroreaProofFirstCatalogHistory
import MirroreaProofFirstInvocationContract

namespace MirroreaProofFirst.SourceCompletion
open SourceAuthoring SourceTypes SourceTyping CompositionCore CompositionMachine

def expectedCreated (cfg : Config p) : Raw → Option Nat
  | .register .. => some cfg.definitions
  | .instantiate .. => some cfg.count
  | _ => none

theorem run_created (cfg : Config p) (raw : Raw) (next : Config p × Option Nat)
    (valid : CompositionCore.Invariant cfg) (accepted : CompositionCore.run cfg raw = some next) :
    next.2 = expectedCreated cfg raw := by
  obtain ⟨cmd,eq,_,rfl⟩ := CatalogHistory.run_parts _ _ _ valid accepted
  rw [← eq]
  cases cmd <;> rfl

theorem control_created (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : Raw) (next : State p a × Option Nat) (valid : CompositionMachine.Invariant s.machine)
    (accepted : control s member place principal raw = some next) :
    next.2 = expectedCreated s.machine.system.configuration raw := by
  unfold SourceAuthoring.control at accepted
  cases h : manage s.machine member place principal s.nextRequest raw with
  | none => simp [h] at accepted
  | some managed =>
      rw [h] at accepted; cases accepted
      obtain ⟨_,_,cfg,created,_,ran,eq⟩ := manage_parts _ _ _ _ _ _ _ h
      rw [eq]
      exact run_created _ _ _ valid.1.1 ran

theorem typed_fresh (s : State p a) (statement : Statement) (env : Environment)
    (name : String) (type : Ty) (typed : StatementTyped p (environment s.values) statement env)
    (output : SourceTypes.output statement = some (name,type)) : SourceAuthoring.lookup s.values name = none := by
  have absent := typed.2
  rw [output] at absent
  have missing : SourceTypes.lookup (environment s.values) name = none := absent.1
  rw [lookup_environment] at missing
  cases h : SourceAuthoring.lookup s.values name <;> simp_all

theorem bind_completes (s : State p a) (name : String) (value : Value)
    (fresh : SourceAuthoring.lookup s.values name = none) :
    SourceAuthoring.bind s name value = some {s with values := (name,value)::s.values} := by
  simp [SourceAuthoring.bind,fresh]

theorem controlled_bind_completes (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : Raw) (managed : State p a × Option Nat) (name : String) (value : Value)
    (fresh : SourceAuthoring.lookup s.values name = none)
    (accepted : control s member place principal raw = some managed) :
    SourceAuthoring.bind managed.1 name value = some {managed.1 with values := (name,value)::s.values} := by
  have values := control_values _ _ _ _ _ _ accepted
  simpa [values] using bind_completes managed.1 name value (by simpa [values] using fresh)

-- The accepted synchronous start cannot become a silent late failure before
-- resume: no environment transition occurs between these two actual operations.
-- An intervening revocation remains a different, rejecting path.
theorem started_completes (m : CompositionMachine.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (argument : Int) (started : CompositionMachine.Machine p a × InvocationBoundary.Ticket)
    (valid : CompositionMachine.Invariant m)
    (accepted : start m member place principal id key argument = some started) :
    ∃ value, resume started.1 started.2 = some (consume started.1 started.2 value,value) := by
  rcases started with ⟨pending,t⟩
  obtain ⟨fresh,checked,eq⟩ := start_parts _ _ _ _ _ _ _ _ accepted
  change pending = enqueue m t at eq
  subst pending
  obtain ⟨value,computed,result⟩ := InvocationBoundary.fresh_result_completes _ _ _ valid.1.1 checked
  have unused := ((fresh_exact _ _).mp fresh).1
  have finished : finish (enqueue m t) t value = some (consume (enqueue m t) t value) := by
    simp [finish,finishCheck,enqueue,unused,result]
  exact ⟨value,by simp [resume,computed,finished]⟩

theorem register_completes (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (name : String) (definition : InstancePrograms.Definition) (predecessor : Option String)
    (previous : Option Nat) (managed : State p a × Option Nat) (env : Environment)
    (valid : CompositionMachine.Invariant s.machine)
    (typed : StatementTyped p (environment s.values) (.register name definition predecessor) env)
    (resolved : optional (definitionKey s.values) predecessor = some previous)
    (accepted : control s member place principal (.register definition previous) = some managed) :
    ∃ next, step s member place principal (.register name definition predecessor) = some next ∧
      next.machine = managed.1.machine := by
  have fresh := typed_fresh _ _ _ _ _ typed rfl
  have created := control_created _ _ _ _ _ _ valid accepted
  have complete := controlled_bind_completes _ _ _ _ _ _ name
    (.definition s.machine.system.configuration.definitions) fresh accepted
  refine ⟨{managed.1 with values := (name,.definition s.machine.system.configuration.definitions)::s.values},?_,rfl⟩
  simpa [step,resolved,accepted,created,expectedCreated] using complete

theorem instantiate_completes (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (name definition : String) (places : List Nat) (parent : Option String) (support : Support.Formula String)
    (key : Nat) (target : Option Nat) (dependencies' : Support.Formula Nat)
    (managed : State p a × Option Nat) (env : Environment)
    (valid : CompositionMachine.Invariant s.machine)
    (typed : StatementTyped p (environment s.values) (.instantiate name definition places parent support) env)
    (resolved : definitionKey s.values definition = some key)
    (resolvedParent : optional (instanceKey s.values) parent = some target)
    (resolvedSupport : dependencies s.values support = some dependencies')
    (accepted : control s member place principal (.instantiate key principal places target dependencies') = some managed) :
    ∃ next, step s member place principal (.instantiate name definition places parent support) = some next ∧
      next.machine = managed.1.machine := by
  have fresh := typed_fresh _ _ _ _ _ typed rfl
  have created := control_created _ _ _ _ _ _ valid accepted
  have complete := controlled_bind_completes _ _ _ _ _ _ name
    (.callable s.machine.system.configuration.count) fresh accepted
  refine ⟨{managed.1 with values := (name,.callable s.machine.system.configuration.count)::s.values},?_,rfl⟩
  simpa [step,resolved,resolvedParent,resolvedSupport,accepted,created,expectedCreated] using complete

theorem invoke_completes (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (name target : String) (expression : Expr) (key : Nat) (argument : Int) (env : Environment)
    (started : CompositionMachine.Machine p a × InvocationBoundary.Ticket)
    (valid : CompositionMachine.Invariant s.machine)
    (typed : StatementTyped p (environment s.values) (.invoke name target expression) env)
    (resolved : instanceKey s.values target = some key) (evaluated : eval s.values expression = some argument)
    (accepted : start s.machine member place principal s.nextRequest key argument = some started) :
    ∃ next value, step s member place principal (.invoke name target expression) = some next ∧
      next.machine = consume started.1 started.2 value ∧
      SourceAuthoring.lookup next.values name = some (.integer value false) := by
  have fresh := typed_fresh _ _ _ _ _ typed rfl
  obtain ⟨value,completed⟩ := started_completes _ _ _ _ _ _ _ _ valid accepted
  let next : State p a :=
    {s with
      machine := consume started.1 started.2 value
      nextRequest := s.nextRequest+1
      values := (name,.integer value false)::s.values}
  refine ⟨next,value,?_,rfl,?_⟩
  · simp [step,resolved,evaluated,accepted,completed,SourceAuthoring.bind,fresh,next]
  · simp [next,SourceAuthoring.lookup]

#print axioms run_created
#print axioms started_completes
#print axioms register_completes
#print axioms instantiate_completes
#print axioms invoke_completes
end MirroreaProofFirst.SourceCompletion
