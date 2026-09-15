import MirroreaProofFirstOwnerFenceMonitor

namespace MirroreaProofFirst.OwnerImageMonitor

abbrev Current (p a : Nat) := Nat × OwnerImage.Image p a

def view (state : OwnerReservation.State p a) : Current p a :=
  (state.core.revision,state.core.image)

def project (state : Option (OwnerEndpoint.State p a)) : Option (Current p a) :=
  state.map (fun state => view state.owner)

-- Keep the complete immutable input image only after a successful actual
-- installation/initialization reply. No hash, caller label or source snapshot
-- substitutes for the complete image retained by the native transition.
def observe (known : Option (Current p a)) (command : OwnerEndpoint.Command p a)
    (reply : Sum Nat OwnerReceipt.Envelope) : Option (Current p a) :=
  match command,reply with
  | .owner (.initialize image),.inl 10 => some (0,image)
  | .owner (.install revision image),.inl 7 => some (revision,image)
  | _,_ => known

theorem submit_view
    (ran : OwnerOccurrence.submit state scopeId ticket = (next,reply)) :
    next.revision = state.revision ∧ next.image = state.image := by
  unfold OwnerOccurrence.submit at ran
  split at ran
  · cases ran; exact ⟨rfl,rfl⟩
  · split at ran
    · split at ran <;> cases ran <;> exact ⟨rfl,rfl⟩
    · split at ran <;> cases ran <;> exact ⟨rfl,rfl⟩

theorem compute_view
    (ran : OwnerReservation.compute state = some (next,reply)) : view next = view state := by
  obtain ⟨ticket,core,_,submitted,equal⟩ := OwnerReservation.compute_parts ran
  obtain ⟨revision,image⟩ := submit_view submitted
  rw [equal]
  simp only [view]
  exact Prod.ext revision image

theorem worker_exact
    (ran : OwnerReservationWorker.transition assigned scopeId capacity state command = (next,reply)) :
    next.map view = observe (state.map view) (.owner command) reply := by
  cases state with
  | none =>
    cases command <;> simp only [OwnerReservationWorker.transition] at ran
    case «initialize» image => split at ran <;> cases ran <;> rfl
    all_goals cases ran; rfl
  | some state =>
    cases command with
    | «initialize» image => cases ran; rfl
    | reserve ticket =>
      cases reserved : OwnerReservation.reserve state scopeId ticket with
      | mk owner status =>
        have core := OwnerReservation.reserve_core reserved
        simp only [OwnerReservationWorker.transition,reserved] at ran
        cases ran
        cases status <;> simp [observe,view,core]
    | compute =>
      cases computed : OwnerReservation.compute state with
      | none => simp only [OwnerReservationWorker.transition,computed] at ran; cases ran; rfl
      | some pair =>
        obtain ⟨owner,status⟩ := pair
        have same := compute_view computed
        simp only [OwnerReservationWorker.transition,computed] at ran
        cases ran
        cases status <;> simpa only [observe,Option.map_some] using congrArg some same
    | install revision image =>
      simp only [OwnerReservationWorker.transition] at ran
      split at ran
      · cases installed : OwnerReservation.install state revision image with
        | none => simp only [installed] at ran; cases ran; rfl
        | some owner =>
          have same := (OwnerReservation.install_idle installed).2
          simp only [installed] at ran
          cases ran
          simp [observe,view,same,OwnerOccurrence.install]
      · cases ran; rfl
    | abandon => cases ran; rfl

theorem endpoint_exact
    (ran : OwnerEndpoint.transition assigned scopeId capacity state command = (next,reply)) :
    project next = observe (project state) command reply := by
  cases state with
  | none =>
    cases command with
    | freeze revision => cases ran; rfl
    | owner command =>
      cases worker : OwnerReservationWorker.transition assigned scopeId capacity none command with
      | mk owner response =>
        have exact := worker_exact worker
        simp only [OwnerEndpoint.transition,worker] at ran
        cases ran
        simpa only [project,Option.map_map,Function.comp_def] using exact
  | some state =>
    cases command with
    | freeze revision =>
      simp only [OwnerEndpoint.transition] at ran
      split at ran <;> cases ran <;> rfl
    | owner command =>
      simp only [OwnerEndpoint.transition] at ran
      split at ran
      · rename_i confirmed
        cases command with
        | install revision image =>
          obtain ⟨rev,_,img⟩ := OwnerEndpoint.confirms_exact.mp confirmed
          cases ran
          simp [project,view,observe,rev,img]
        | «initialize» image => cases confirmed
        | reserve ticket => cases confirmed
        | compute => cases confirmed
        | abandon => cases confirmed
      · split at ran
        · cases worker : OwnerReservationWorker.transition assigned scopeId capacity (some state.owner) command with
          | mk owner response =>
            have exact := worker_exact worker
            rw [worker] at ran
            cases ran
            simpa only [project,Option.map_map,Function.comp_def,Option.map_some] using exact
        · cases ran
          cases command <;> rfl

theorem profile_exact
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity state command = (next,reply)) :
    project next = observe (project state) command reply := by
  unfold OwnerEndpointProfile.transition at ran
  split at ran
  · exact endpoint_exact ran
  · cases ran
    cases command with
    | freeze revision => rfl
    | owner command => cases command <;> rfl

theorem budget_exact
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply)) :
    project next.owner = observe (project state.owner) command reply := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases native : OwnerEndpointProfile.transition assigned scopeId capacity state.owner command with
    | mk owner response =>
      rw [native] at ran
      cases ran
      exact profile_exact native
  · cases ran
    cases command with
    | freeze revision => rfl
    | owner command => cases command <;> rfl

inductive Runs (assigned : OwnerEvaluator.Assignment p) (scopeId capacity budget : Nat) :
    OwnerEndpointBudget.State p a → Option (Current p a) → Prop where
  | fresh : Runs assigned scopeId capacity budget (OwnerEndpointBudget.initial budget) none
  | step : Runs assigned scopeId capacity budget state known →
      OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply) →
      Runs assigned scopeId capacity budget next (observe known command reply)

theorem reached_exact (path : Runs assigned scopeId capacity budget state known) :
    project state.owner = known := by
  induction path with
  | fresh => rfl
  | step prior ran ih => rw [budget_exact ran,ih]

-- Source currentness is compared to an independently tracked owner, not
-- presumed from a head query. Exact image bytes have proved codec injectivity.
def currentCheck (known : Option (Current p a)) (revision : Nat) (image : OwnerImage.Image p a) : Bool :=
  match known with
  | none => false
  | some (stored,actual) => decide (stored = revision) && OwnerEndpoint.sameImage actual image

theorem current_check_exact : currentCheck known revision image = true ↔ known = some (revision,image) := by
  cases known with
  | none => simp [currentCheck]
  | some pair =>
    obtain ⟨stored,actual⟩ := pair
    simp [currentCheck,OwnerEndpoint.sameImage_exact]

theorem checked_actual_image
    (path : Runs assigned scopeId capacity budget state known)
    (checked : currentCheck known revision image = true) :
    ∃ owner, state.owner = some owner ∧ owner.owner.core.revision = revision ∧ owner.owner.core.image = image := by
  have exact := (reached_exact path).trans (current_check_exact.mp checked)
  cases present : state.owner with
  | none => simp [project,present] at exact
  | some owner =>
    have fields : owner.owner.core.revision = revision ∧ owner.owner.core.image = image := by
      simpa [project,present,view] using exact
    exact ⟨owner,rfl,fields⟩

-- One trace drives BOTH monitors. A host cannot pair an image from one
-- successful history with a fence from another namespace/history.
inductive JointRuns (assigned : OwnerEvaluator.Assignment p) (scopeId capacity budget : Nat) :
    OwnerEndpointBudget.State p a → Option (Current p a) → Nat → Prop where
  | fresh : JointRuns assigned scopeId capacity budget (OwnerEndpointBudget.initial budget) none 0
  | step : JointRuns assigned scopeId capacity budget state known stoppedAt →
      OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply) →
      JointRuns assigned scopeId capacity budget next (observe known command reply)
        (OwnerFenceMonitor.advance stoppedAt command reply)

theorem joint_exact (path : JointRuns assigned scopeId capacity budget state known stoppedAt) :
    project state.owner = known ∧ OwnerFenceMonitor.floor state.owner = stoppedAt := by
  induction path with
  | fresh => exact ⟨rfl,rfl⟩
  | step prior ran ih =>
    exact ⟨(budget_exact ran).trans (congrArg (fun value => observe value _ _) ih.1),
      (OwnerFenceMonitor.budget_exact ran).trans
        (congrArg (fun value => OwnerFenceMonitor.advance value _ _) ih.2)⟩

def usableCheck (known : Option (Current p a)) (stoppedAt revision : Nat)
    (image : OwnerImage.Image p a) : Bool :=
  currentCheck known revision image && decide (stoppedAt = revision)

-- Declarative condition speaks about the ACTUAL native-model state. Its
-- checker reads only observations; it never receives a state/invariant proof.
def Usable (state : OwnerEndpointBudget.State p a) (revision : Nat) (image : OwnerImage.Image p a) : Prop :=
  ∃ owner, state.owner = some owner ∧ owner.owner.core.revision = revision ∧
    owner.owner.core.image = image ∧ owner.fence = revision

theorem usable_check_exact
    (path : JointRuns assigned scopeId capacity budget state known stoppedAt) :
    usableCheck known stoppedAt revision image = true ↔ Usable state revision image := by
  have exact := joint_exact path
  simp only [usableCheck,Bool.and_eq_true,current_check_exact,decide_eq_true_eq]
  constructor
  · rintro ⟨installed,frozen⟩
    have current := exact.1.trans installed
    cases present : state.owner with
    | none => simp [project,present] at current
    | some owner =>
      have pair : owner.owner.core.revision = revision ∧ owner.owner.core.image = image := by
        simpa [project,present,view] using current
      have fence : owner.fence = revision := by
        simpa [OwnerFenceMonitor.floor,present] using exact.2.trans frozen
      exact ⟨owner,present,pair.1,pair.2,fence⟩
  · rintro ⟨owner,present,rev,img,fence⟩
    constructor
    · rw [←exact.1]
      simp [project,present,view,rev,img]
    · rw [←exact.2]
      simp [OwnerFenceMonitor.floor,present,fence]

#print axioms joint_exact
#print axioms usable_check_exact
#print axioms submit_view
#print axioms compute_view
#print axioms worker_exact
#print axioms endpoint_exact
#print axioms profile_exact
#print axioms budget_exact
#print axioms reached_exact
#print axioms current_check_exact
#print axioms checked_actual_image
end MirroreaProofFirst.OwnerImageMonitor
