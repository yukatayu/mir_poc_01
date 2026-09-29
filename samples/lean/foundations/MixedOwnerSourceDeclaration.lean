import MixedOwnerSourceCode
import MixedOwnerSourceIssue
namespace MirroreaProofFirst.MixedOwnerSourceInvariant
-- Shared state predicates are below the entry relation so generated declaration
-- can be an actual relation constructor without a proof/import cycle.
def Valid (s : MixedOwnerSourceIssue.State p a) : Prop :=
 MixedReferenceExecution.Invariant s.machine ∧
 MixedReferenceAllocation.Below s.machine s.nextRequest ∧
 MixedReferenceSource.PendingAgrees (MixedOwnerSourceIssue.base s)
def OwnerAgrees (s : MixedOwnerSourceIssue.State p a) : Prop :=
 ∀ waiting, s.waiting = some (.inr waiting) → s.machine.store.core.pending = [.owner waiting.saved]
end MirroreaProofFirst.MixedOwnerSourceInvariant
namespace MirroreaProofFirst.MixedOwnerSourceDeclaration
open MixedOwnerSourceIssue ReferenceSourceData

-- Compiler-generated owner declaration. The surrounding program must derive
-- name/context/labels/control from its retained source. This lower source entry
-- takes an Assignment, never a separately supplied executable body or values.
-- Subsequent construction uses the existing ordinary instantiate statement.
def declared (s : State p a) (machine : MixedReferenceExecution.Machine p a)
 (source : MixedNamedOwnerSource.Assignment) (name : String) (key : Nat) : State p a :=
 embed (MixedReferenceSource.write
   (MixedReferenceSource.mark (base s) machine (some source.site) .control 1)
   source.site s.machine.store.events.length [] name (.plain (.definition key)) false)

def declareOwner (s : State p a) (ctx : MixedNamedOwnerSource.Fields)
 (labels : List (String × Nat)) (control : Nat) (member : Fin a) (place : Fin p)
 (principal : Nat) (name : String) (source : MixedNamedOwnerSource.Assignment) : Outcome p a :=
 if s.waiting.isSome || !s.machine.store.core.pending.isEmpty || (lookup s.values name).isSome then ⟨s,.failed .rejected⟩ else
 match MixedOwnerSourceCode.compile ctx labels control source with
 | none => ⟨s,.failed .rejected⟩
 | some code =>
   match MixedReferenceExecution.manage s.machine member place principal s.nextRequest (.register (.owner code.operation) none) with
   | none => ⟨s,.failed .rejected⟩
   | some (machine,some key) => ⟨declared s machine source name key,.ready⟩
   | some (machine,none) =>
     -- Retain any actual committed prefix, even if this impossible register
     -- result shape is encountered. Never erase state through an outer Option.
     ⟨embed (MixedReferenceSource.mark (base s) machine (some source.site) .control 1),.failed .rejected⟩

theorem declaration_parts {control : Nat}
 (accepted : (declareOwner s ctx labels control member place principal name source).status = .ready) :
 s.waiting = none ∧ s.machine.store.core.pending = [] ∧ lookup s.values name = none ∧
 ∃ code machine key, MixedOwnerSourceCode.Compiles ctx labels control source code ∧
 MixedReferenceExecution.manage s.machine member place principal s.nextRequest (.register (.owner code.operation) none) = some (machine,some key) ∧
 (declareOwner s ctx labels control member place principal name source).state = declared s machine source name key := by
 unfold declareOwner at accepted ⊢
 split at accepted
 · cases accepted
 · rename_i guardOpen
   have idle : s.waiting = none ∧ s.machine.store.core.pending = [] ∧ lookup s.values name = none := by
     cases wait : s.waiting <;> cases pending : s.machine.store.core.pending <;> cases named : lookup s.values name <;> simp_all
   simp only [idle.1,idle.2.1,idle.2.2,Option.isSome_none,List.isEmpty_nil,Bool.not_true,Bool.or_self,ite_false]
   cases compiled : MixedOwnerSourceCode.compile ctx labels control source with
   | none => simp [compiled] at accepted
   | some code =>
     simp only [compiled] at accepted ⊢
     cases managed : MixedReferenceExecution.manage s.machine member place principal s.nextRequest (.register (.owner code.operation) none) with
     | none => simp [managed] at accepted
     | some pair =>
       obtain ⟨machine,key⟩ := pair
       cases key with
       | none => simp [managed] at accepted
       | some key => exact ⟨True.intro,True.intro,True.intro,code,machine,key,(MixedOwnerSourceCode.compile_exact control).mp compiled,managed,by simp [managed]⟩

-- No forged binding is returned on compilation/authorization refusal. A
-- successful definition name refers to the actual management result and its
-- source write records the real before/after history interval.
theorem declared_binding :
 lookup (declared s machine source name key).values name = some (.plain (.definition key)) ∧
 (declared s machine source name key).nextRequest = s.nextRequest+1 ∧
 (declared s machine source name key).writes =
  ⟨s.writes.length,source.site,name,.plain (.definition key),[],s.machine.store.events.length,machine.store.events.length⟩::s.writes := by
 simp [declared,embed,MixedReferenceSource.write,MixedReferenceSource.mark,base,lookup]

-- Registration returns the actual new catalog coordinate and the exact
-- compiled definition is the catalog addition. A caller cannot choose its key.
theorem registration_result {id : Nat}
 (accepted : MixedReferenceExecution.manage machine member place principal id
   (.register (.owner operation) none) = some (next,key)) :
 key = some machine.store.core.system.configuration.definitions ∧
 next.store.core.system.configuration = MixedCompositionCore.config
  (MixedInstanceState.register machine.store.core.system.configuration.state (.owner operation) none) := by
 have core := MixedReferenceExecution.manage_core _ _ _ _ _ _ _ accepted
 obtain ⟨_,e,cfg,created,_,run,equal⟩ := MixedRequestCore.manage_parts core
 have resultKey := congrArg Prod.snd equal
 have resultState := congrArg (fun pair : MixedRequestCore.Machine _ _ × Option Nat => pair.1.system.configuration) equal
 simp only [MixedCompositionCore.run,MixedCompositionCore.elaborate,MixedCompositionCore.optionalIndex,
   Option.bind_eq_bind,Option.bind_some,Option.pure_def,MixedCompositionCore.apply] at run
 split at run
 · simp only [Option.some.injEq] at run
   have registered := congrArg Prod.fst run
   have coordinate := congrArg Prod.snd run
   exact ⟨resultKey.trans coordinate.symm, resultState.trans registered.symm⟩
 · cases run

-- Relative completeness assumes the independent actor/authority judgment and
-- the retained reference floor at the tentative result. It assumes neither a
-- successful registration nor the desired final source state.
theorem registration_exists {p a : Nat} {machine : MixedReferenceExecution.Machine p a} {member : Fin a} {place : Fin p} {id principal : Nat} {operation : MixedOperationDefinitions.OwnerDefinition}
 (valid : MixedReferenceExecution.Invariant machine)
 (typed : MixedOperationDefinitions.OwnerSatisfies operation)
 (fresh : MixedRequestCore.fresh machine.store.core
   (MixedManagementEntry.useId machine.store.core.system principal id) = true)
 (auth : MixedManagementEntry.Allowed machine.store.core.system member place principal id (.register (.owner operation) none))
 (floor : MixedReferenceStore.Floors
   (MixedInstanceState.register machine.store.core.system.configuration.state (.owner operation) none) machine.store.bindings) :
 ∃ next, MixedReferenceExecution.manage machine member place principal id (.register (.owner operation) none) =
   some (next,some machine.store.core.system.configuration.definitions) := by
 let system := machine.store.core.system
 let command : MixedCompositionCore.Command system.configuration.definitions p system.configuration.count := .register (.owner operation) none
 have structural : MixedCompositionCore.Allowed system.configuration.state command :=
   ⟨typed,by intro old impossible; cases impossible⟩
 have systemValid : MixedManagementEntry.Invariant system := valid.1.1.1.1
 obtain ⟨result,performed⟩ := MixedManagementEntry.perform_complete system member place principal id command
   systemValid structural auth (MixedRequestCore.fresh_exact.mp fresh).1
 have coreRun : ∃ result, MixedRequestCore.manage machine.store.core member place principal id
     (.register (.owner operation) none) = some result := by
   obtain ⟨newSystem,key⟩ := result
   refine ⟨({machine.store.core with system := newSystem, events := (.management (MixedManagementEntry.current system member place principal id (.register (.owner operation) none)) key :: machine.store.core.events)},key),?_⟩
   have performed' : MixedManagementEntry.perform system member place principal id
      (.register (.owner operation) none) = some (newSystem,key) := performed
   simp [MixedRequestCore.manage,fresh,performed',system]
 obtain ⟨coreResult,coreRun⟩ := coreRun
 obtain ⟨_,e,cfg,created,_,run,equal⟩ := MixedRequestCore.manage_parts coreRun
 have checkOwner : MixedOperationDefinitions.ownerCheck operation = true := MixedOperationDefinitions.owner_check_exact.mpr typed
 have shape : (cfg,created) = (MixedCompositionCore.config (MixedInstanceState.register system.configuration.state (.owner operation) none),some system.configuration.definitions) := by
   simpa [MixedCompositionCore.run,MixedCompositionCore.elaborate,MixedCompositionCore.optionalIndex,
     MixedCompositionCore.apply,MixedCompositionCore.check,MixedInstanceState.registrationCheck,
     MixedOperationDefinitions.check,checkOwner,MixedCompositionCore.outcome] using run.symm
 have floors : MixedReferenceStore.floorsCheck coreResult.1.system.configuration.state machine.store.bindings = true := by
   rw [equal]
   change MixedReferenceStore.floorsCheck cfg.state machine.store.bindings = true
   rw [(Prod.mk.inj shape).1]
   exact (MixedReferenceStore.floors_exact _ _).mpr floor
 have succeeds : (MixedReferenceExecution.manage machine member place principal id (.register (.owner operation) none)).isSome = true := by
   simp [MixedReferenceExecution.manage,MixedReferenceStore.manage,coreRun,floors]
 cases accepted : MixedReferenceExecution.manage machine member place principal id (.register (.owner operation) none) with
 | none => simp [accepted] at succeeds
 | some pair =>
   obtain ⟨next,key⟩ := pair
   have coordinate := (registration_result accepted).1
   exact ⟨next,by simpa [coordinate] using accepted⟩

theorem declaration_complete {control : Nat}
 (valid : MixedOwnerSourceInvariant.Valid s) (idle : s.waiting = none)
 (drained : s.machine.store.core.pending = []) (freshName : lookup s.values name = none)
 (compiled : MixedOwnerSourceCode.Compiles ctx labels control source code)
 (auth : MixedManagementEntry.Allowed s.machine.store.core.system member place principal s.nextRequest (.register (.owner code.operation) none))
 (floor : MixedReferenceStore.Floors
   (MixedInstanceState.register s.machine.store.core.system.configuration.state (.owner code.operation) none) s.machine.store.bindings) :
 (declareOwner s ctx labels control member place principal name source).status = .ready := by
 have typed : MixedOperationDefinitions.OwnerSatisfies code.operation := by
   obtain ⟨_,_,_,_,_,_,typed,rfl⟩ := compiled; exact typed
 have fresh := MixedReferenceAllocation.fresh_bound s.machine s.nextRequest
   s.machine.store.core.system.configuration.state.realm principal valid.2.1
 obtain ⟨machine,managed⟩ := registration_exists valid.1 typed fresh auth floor
 simp [declareOwner,idle,drained,freshName,(MixedOwnerSourceCode.compile_exact control).mpr compiled,managed]

-- The successful source definition value is tied to exactly the generated
-- catalog addition, not merely to a successful status or arbitrary numeric key.
theorem declaration_catalog {control : Nat}
 (accepted : (declareOwner s ctx labels control member place principal name source).status = .ready) :
 ∃ code, MixedOwnerSourceCode.Compiles ctx labels control source code ∧
 lookup (declareOwner s ctx labels control member place principal name source).state.values name =
  some (.plain (.definition s.machine.store.core.system.configuration.definitions)) ∧
 (declareOwner s ctx labels control member place principal name source).state.machine.store.core.system.configuration =
  MixedCompositionCore.config (MixedInstanceState.register s.machine.store.core.system.configuration.state (.owner code.operation) none) := by
 obtain ⟨_,_,_,code,machine,key,compiled,managed,equal⟩ := declaration_parts accepted
 obtain ⟨coordinate,catalog⟩ := registration_result managed
 have keyEqual := Option.some.inj coordinate
 refine ⟨code,compiled,?_,?_⟩
 · rw [equal]; simpa [keyEqual] using (declared_binding (s:=s) (machine:=machine) (source:=source) (name:=name) (key:=key)).1
 · rw [equal]; exact catalog

-- Every outcome, including retained-prefix failures, preserves the shared
-- execution invariant and the single allocator bound. Registration cannot
-- introduce a hidden pure or owner wait.
theorem marked_valid (valid : MixedOwnerSourceInvariant.Valid s)
 (idle : s.waiting = none)
 (managed : MixedReferenceExecution.manage s.machine member place principal s.nextRequest raw = some (machine,key)) :
 MixedOwnerSourceInvariant.Valid (embed (MixedReferenceSource.mark (base s) machine site .control 1)) := by
 have core := MixedReferenceExecution.manage_preserves _ _ _ _ _ _ _ valid.1 managed
 have bound := MixedReferenceAllocation.manage_below _ _ _ _ _ _ _ valid.2.1 managed
 have pending := MixedReferenceExecution.manage_pending _ _ _ _ _ _ _ managed
 have empty : s.machine.pending = [] := by
   simpa [MixedReferenceSource.PendingAgrees,base,pureWaiting,idle] using valid.2.2
 exact ⟨core,bound,by simp [MixedReferenceSource.PendingAgrees,base,embed,pureWaiting,
   MixedReferenceSource.mark,idle]; exact pending.trans empty⟩

theorem declared_valid (valid : MixedOwnerSourceInvariant.Valid s)
 (idle : s.waiting = none)
 (managed : MixedReferenceExecution.manage s.machine member place principal s.nextRequest raw = some (machine,resultKey)) :
 MixedOwnerSourceInvariant.Valid (declared s machine source name key) := by
 exact marked_valid (site := some source.site) valid idle managed

theorem declaration_preserves {control : Nat} (valid : MixedOwnerSourceInvariant.Valid s)
 (agrees : MixedOwnerSourceInvariant.OwnerAgrees s) :
 MixedOwnerSourceInvariant.Valid (declareOwner s ctx labels control member place principal name source).state ∧
 MixedOwnerSourceInvariant.OwnerAgrees (declareOwner s ctx labels control member place principal name source).state := by
 unfold declareOwner
 split
 · exact ⟨valid,agrees⟩
 · rename_i guardOpen
   have idle : s.waiting = none := by cases wait : s.waiting <;> simp_all
   cases compiled : MixedOwnerSourceCode.compile ctx labels control source with
   | none => exact ⟨valid,agrees⟩
   | some code =>
     dsimp only
     cases managed : MixedReferenceExecution.manage s.machine member place principal s.nextRequest (.register (.owner code.operation) none) with
     | none => exact ⟨valid,agrees⟩
     | some pair =>
       obtain ⟨machine,key⟩ := pair
       cases key with
       | none =>
         exact ⟨marked_valid valid idle managed,by
           intro waiting held
           simp [embed,base,pureWaiting,idle,MixedReferenceSource.mark] at held⟩
       | some key =>
         exact ⟨declared_valid valid idle managed,by
           intro waiting held
           simp [declared,embed,base,pureWaiting,idle,MixedReferenceSource.mark,MixedReferenceSource.write] at held⟩

theorem declaration_valid {control : Nat} (valid : MixedOwnerSourceInvariant.Valid s) :
 MixedOwnerSourceInvariant.Valid (declareOwner s ctx labels control member place principal name source).state := by
 cases held : s.waiting with
 | none => exact (declaration_preserves valid (by intro waiting impossible; simp [held] at impossible)).1
 | some saved => simpa [declareOwner,held] using valid

#print axioms declaration_valid
#print axioms declaration_complete
#print axioms declaration_catalog
#print axioms registration_result
#print axioms registration_exists
#print axioms marked_valid
#print axioms declared_valid
#print axioms declaration_preserves
#print axioms declaration_parts
#print axioms declared_binding
end MirroreaProofFirst.MixedOwnerSourceDeclaration
