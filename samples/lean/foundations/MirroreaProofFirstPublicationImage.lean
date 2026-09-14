import MirroreaProofFirstPublicationUse
import MirroreaProofFirstPublicationProgress

namespace MirroreaProofFirst.PublicationImage

-- Executable image-only replica representation of the existing publication
-- protocol. Only the private coordinator holds Value/prepared Value. A replica
-- holds Image; neither cached nor retained publication data contains Value.
-- Channel authentication, disclosure grants and physical custody remain
-- separate refinement obligations. This is not a serializer for private state.
structure State (n : Nat) (Value Command Image : Type) where
  barrier : Publication.State n
  current : Value
  prepared : Option (Value × Command)
  images : Nat → Option Image
  cached : Fin n → Image
  held : Fin n → Option (Nat × Image)

def project (represent : Value → Image) (s : PublicationUse.State n Value Command) :
    State n Value Command Image :=
  ⟨s.base.barrier,s.base.current,s.base.prepared,
    fun r => (s.base.images r).map represent,
    fun i => represent (s.base.cached i),
    fun i => (s.held i).map (fun pair => (pair.1,represent pair.2))⟩

def initial (represent : Value → Image) (n revision : Nat) (value : Value) :
    State n Value Command Image :=
  ⟨Publication.initial n revision,value,none,
    fun r => if r = revision then some (represent value) else none,
    fun _ => represent value,fun _ => none⟩

def extra (evaluate : Value → Command → Option Value)
    (s : State n Value Command Image) : PublicationPayload.Action n Command → Bool
  | .stage command => (evaluate s.current command).isSome
  | .freeze i _ => (s.held i).isNone
  | .publish => s.prepared.isSome
  | .install i revision => (s.images revision).isSome && (s.held i).isNone
  | _ => true

def check (evaluate : Value → Command → Option Value)
    (s : State n Value Command Image) : PublicationUse.Action n Command → Bool
  | .administrative action => Publication.check s.barrier (PublicationPayload.projected action) && extra evaluate s action
  | .enter i => Publication.check s.barrier (.use i) && (s.held i).isNone
  | .finish i => (s.held i).isSome

def apply (represent : Value → Image) (evaluate : Value → Command → Option Value)
    (s : State n Value Command Image) : PublicationUse.Action n Command → State n Value Command Image
  | .administrative action =>
      let updated := {s with barrier := Publication.apply s.barrier (PublicationPayload.projected action)}
      match action with
      | .stage command => {updated with prepared := (evaluate s.current command).map (fun value => (value,command))}
      | .publish =>
          let value := (s.prepared.map Prod.fst).getD s.current
          {updated with current := value,prepared := none,images := PublicationPayload.write s.images s.barrier.announced (represent value)}
      | .install i revision =>
          {updated with cached := PublicationPayload.put s.cached i ((s.images revision).getD (s.cached i))}
      | _ => updated
  | .enter i => {s with held := PublicationPayload.put s.held i (some (s.barrier.installed i,s.cached i))}
  | .finish i => {s with held := PublicationPayload.put s.held i none}

def execute (represent : Value → Image) (evaluate : Value → Command → Option Value)
    (s : State n Value Command Image) (action : PublicationUse.Action n Command) :
    Option (State n Value Command Image) :=
  if check evaluate s action then some (apply represent evaluate s action) else none

theorem initial_project {Command : Type} (represent : Value → Image) (n revision : Nat) (value : Value) :
    project represent (PublicationUse.initial (Command:=Command) n revision value) = initial represent n revision value := by
  simp [project,PublicationUse.initial,PublicationPayload.initial,initial]

theorem check_project (represent : Value → Image) (evaluate : Value → Command → Option Value)
    (s : PublicationUse.State n Value Command) (action : PublicationUse.Action n Command) :
    check evaluate (project represent s) action = PublicationUse.check evaluate s action := by
  cases action with
  | administrative action =>
      cases action <;> simp [check,extra,project,PublicationUse.check,PublicationPayload.check,
        PublicationPayload.extraCheck,PublicationUse.freeCheck,Bool.and_assoc]
  | enter i => simp [check,project,PublicationUse.check,PublicationPayload.check,PublicationPayload.extraCheck,PublicationPayload.projected]
  | finish i => simp [check,project,PublicationUse.check]

theorem apply_project (represent : Value → Image) (evaluate : Value → Command → Option Value)
    (s : PublicationUse.State n Value Command) (action : PublicationUse.Action n Command) :
    apply represent evaluate (project represent s) action =
      project represent (PublicationUse.apply evaluate s action) := by
  cases action with
  | administrative action =>
      cases action with
      | stage command => rfl
      | freeze i r => rfl
      | acknowledge i r => rfl
      | use i => rfl
      | publish =>
          simp only [apply,project,PublicationUse.apply,PublicationPayload.apply]
          congr 1
          funext r
          simp [PublicationPayload.write]
          split <;> rfl
      | install i r =>
          simp only [apply,project,PublicationUse.apply,PublicationPayload.apply]
          congr 1
          funext j
          by_cases same : j = i
          · subst j
            simp only [PublicationPayload.put,ite_true]
            cases s.base.images r <;> rfl
          · simp [PublicationPayload.put,same]
  | enter i =>
      simp only [apply,project,PublicationUse.apply]
      congr 1
      funext j
      by_cases same : j = i <;> simp [PublicationPayload.put,same]
  | finish i =>
      simp only [apply,project,PublicationUse.apply]
      congr 1
      funext j
      by_cases same : j = i <;> simp [PublicationPayload.put,same]

theorem execute_project (represent : Value → Image) (evaluate : Value → Command → Option Value)
    (s : PublicationUse.State n Value Command) (action : PublicationUse.Action n Command) :
    execute represent evaluate (project represent s) action =
      (PublicationUse.execute evaluate s action).map (project represent) := by
  simp only [execute,check_project,PublicationUse.execute]
  split <;> simp [apply_project]

inductive Reached (represent : Value → Image) (evaluate : Value → Command → Option Value)
    (n revision : Nat) (value : Value) : State n Value Command Image → Prop where
  | initial : Reached represent evaluate n revision value (PublicationImage.initial represent n revision value)
  | step : Reached represent evaluate n revision value s →
      execute represent evaluate s action = some next → Reached represent evaluate n revision value next

-- Existential abstract state is constructed from the actual concrete path;
-- no image-currentness invariant is tested or assumed by the checker.
theorem reached_lifts {represent : Value → Image} {evaluate : Value → Command → Option Value}
    {s : State n Value Command Image} (path : Reached represent evaluate n revision value s) :
    ∃ abstract, PublicationUse.Reached evaluate n revision value abstract ∧ s = project represent abstract := by
  induction path with
  | initial => exact ⟨_,.initial,(initial_project _ _ _ _).symm⟩
  | step prior accepted ih =>
      obtain ⟨abstract,reached,rfl⟩ := ih
      rw [execute_project] at accepted
      obtain ⟨result,ran,equal⟩ := Option.map_eq_some_iff.mp accepted
      obtain ⟨allowed,same⟩ := (PublicationUse.execute_exact _ _ _ _).mp ran
      refine ⟨result,?_,equal.symm⟩
      rw [same]
      exact .step reached (.action allowed)

theorem held_current {represent : Value → Image} {evaluate : Value → Command → Option Value}
    {s : State n Value Command Image} (path : Reached represent evaluate n revision value s)
    (endpoint : Fin n) (pair : Nat × Image) (held : s.held endpoint = some pair) :
    pair.1 = s.barrier.published ∧ pair.2 = represent s.current := by
  obtain ⟨abstract,reached,rfl⟩ := reached_lifts path
  cases retained : abstract.held endpoint with
  | none => simp [project,retained] at held
  | some original =>
      simp only [project,retained,Option.map_some,Option.some.injEq] at held
      subst pair
      obtain ⟨revisionAt,current⟩ := PublicationUse.held_value_current reached endpoint original retained
      exact ⟨revisionAt,congrArg represent current⟩

def run (represent : Value → Image) (evaluate : Value → Command → Option Value) :
    State n Value Command Image → List (PublicationUse.Action n Command) → Option (State n Value Command Image)
  | s,[] => some s
  | s,action :: rest => execute represent evaluate s action >>= fun next => run represent evaluate next rest

theorem run_project (represent : Value → Image) (evaluate : Value → Command → Option Value)
    (s : PublicationUse.State n Value Command) (actions : List (PublicationUse.Action n Command)) :
    run represent evaluate (project represent s) actions =
      (PublicationProgress.useRun evaluate s actions).map (project represent) := by
  induction actions generalizing s with
  | nil => rfl
  | cons action rest ih =>
      simp only [run,execute_project,PublicationProgress.useRun]
      cases step : PublicationUse.execute evaluate s action <;> simp [ih]

theorem run_reached (path : Reached represent evaluate n revision initialValue s)
    (ran : run represent evaluate s actions = some next) : Reached represent evaluate n revision initialValue next := by
  induction actions generalizing s with
  | nil => cases ran; exact path
  | cons action rest ih =>
      cases step : execute represent evaluate s action with
      | none => simp [run,step] at ran
      | some middle => exact ih (.step path step) (by simpa [run,step] using ran)

theorem normal_succeeds {represent : Value → Image} {evaluate : Value → Command → Option Value}
    {s : State n Value Command Image} (nonempty : 0 < n)
    (path : Reached represent evaluate n revision initialValue s)
    (settled : s.barrier.announced = s.barrier.published) (free : ∀ i, s.held i = none)
    (computed : evaluate s.current command = some value) :
    ∃ next, run represent evaluate s
        ((PublicationProgress.normal n (s.barrier.published+1)).map (PublicationProgress.lift command)) = some next ∧
      Reached represent evaluate n revision initialValue next ∧ next.current = value ∧
      next.barrier.published = s.barrier.published+1 ∧
      ∀ i, next.cached i = represent value ∧ next.held i = none ∧ check evaluate next (.enter i) = true := by
  obtain ⟨abstract,reached,rfl⟩ := reached_lifts path
  have noHeld : ∀ i, abstract.held i = none := by
    intro i
    have no := free i
    cases held : abstract.held i <;> simp_all [project]
  obtain ⟨next,ran,nextPath,current,published,_,all,_⟩ :=
    PublicationProgress.normal_payload_succeeds nonempty reached settled noHeld computed
  have actualRun : run represent evaluate (project represent abstract)
      ((PublicationProgress.normal n (abstract.base.barrier.published+1)).map (PublicationProgress.lift command)) =
      some (project represent next) := by rw [run_project,ran]; rfl
  refine ⟨project represent next,actualRun,run_reached path actualRun,current,published,?_⟩
  intro i
  exact ⟨congrArg represent (all i).1,by simp [project,(all i).2.1],by rw [check_project]; exact (all i).2.2⟩

#print axioms initial_project
#print axioms check_project
#print axioms apply_project
#print axioms execute_project
#print axioms reached_lifts
#print axioms held_current
#print axioms run_project
#print axioms normal_succeeds
end MirroreaProofFirst.PublicationImage
