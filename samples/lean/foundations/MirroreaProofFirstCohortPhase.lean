import MirroreaProofFirstPublicJointWork
import MirroreaProofFirstPaidHeadPhase

namespace MirroreaProofFirst.CohortPhase
open PublicationOwnerBudget

-- Candidate operational funding/ordinal projection of the private supervisor.
-- Successful actual transitions, not balance checkpoints, advance the phases.
-- Image/registration/authority and physical capture correspondence are separate
-- projections still to couple; these constructors are NOT a public control API.
inductive Mode (p : Nat) where
  | prelude (owed : Fin p → Bool)
  | ordinary
  | paid (pending : PaidHeadPhase.Pending p)
  | entered (dispatch : PublicationInput.Dispatch p)
  | reserved (dispatch : PublicationInput.Dispatch p)
  | probed (dispatch : PublicationInput.Dispatch p)
  | computed (dispatch : PublicationInput.Dispatch p) (envelope : OwnerReceipt.Envelope)

structure Live (p a : Nat) where
  driver : PublicationCapacityDriver.State p a
  owners : Fin p → OwnerEndpointBudget.State p a
  ordinal : Nat
  mode : Mode p

abbrev State (p a : Nat) := Option (Live p a)

def credits (s : Live p a) : Fin p → Nat := fun i => (s.owners i).remaining

def VectorAt (s : Live p a) (value : Vector (Fin 513) p) : Prop :=
  ∀ i, credits s i = (value[i.val]).val

def partialDebit (i : Fin p) (count : Nat) : Fin p → Nat := fun j => if j=i then count else 0

def WorkCovered (s : Live p a) (dispatch : PublicationInput.Dispatch p) (count : Nat) : Prop :=
  (∃ rest, s.driver.suffix = .finish dispatch.endpoint :: rest) ∧
  OwnerFundingCursor.CoveredPartial s.driver.suffix 0 (partialDebit dispatch.endpoint count) (credits s)

def Funding (s : Live p a) : Prop :=
  match s.mode with
  | .prelude owed => OwnerFundingCursor.InitCovered s.driver.suffix owed (credits s)
  | .ordinary => Covered s.driver.suffix (credits s)
  | .paid pending => s.ordinal = pending.sourceOrdinal ∧
      ∃ rest, s.driver.suffix = PaidHeadPhase.command pending :: rest ∧ Covered rest (credits s)
  | .entered dispatch => WorkCovered s dispatch 0
  | .reserved dispatch => WorkCovered s dispatch 1
  | .probed dispatch => WorkCovered s dispatch 2
  | .computed dispatch _ => WorkCovered s dispatch 3

def Invariant (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (state : State p a) : Prop :=
  ∀ s, state = some s → PublicationLifecycle.Invariant assigned scope bootstrap s.driver ∧ Funding s

def sourceResult (s : Live p a) (next : PublicationCapacityDriver.State p a) (mode : Mode p) : Live p a :=
  {s with driver := next,ordinal := s.ordinal+1,mode := mode}

def ownerResult (s : Live p a) (target : Fin p) (next : OwnerEndpointBudget.State p a) (mode : Mode p) : Live p a :=
  {s with owners := PublicOwnerBoundary.put s.owners target next,mode := mode}

def paymentRequest (pending : PaidHeadPhase.Pending p) (source : PublicationInput.State p a) : OwnerEndpoint.Command p a :=
  if pending.freeze then .freeze pending.revision else
    .owner (.install pending.revision (CustodyPublication.image source.publication.current))

def paymentReply (pending : PaidHeadPhase.Pending p) : Sum Nat OwnerReceipt.Envelope :=
  .inl (if pending.freeze then 12 else 7)

def HeadPayment (driver : PublicationCapacityDriver.State p a) (target : Fin p)
    (request : OwnerEndpoint.Command p a) : Prop :=
  (∃ revision rest, driver.suffix = .freeze target revision :: rest ∧ request = .freeze revision) ∨
  (∃ source revision rest, driver.source = some source ∧ driver.suffix = .install target revision :: rest ∧
    request = .owner (.install revision (CustodyPublication.image source.publication.current)))

def QueryAllowed : Mode p → Prop
  | .prelude _ | .ordinary | .entered _ => True
  | _ => False

-- Every successful administrative payment immediately creates the exact
-- pending occurrence. There is no rule delaying this update until a checkpoint.
-- Unknown IO/contradictions retire; none denotes loss of a continuing handle,
-- never physical idle, erased reservations, successful EOF or rollback.
inductive Event (p a : Nat) where
  | local
  | retire
  | query (request : Sum (SourceFundingQuery.HeadRequest p) (Fin p))
  | source (request : SourceFundingInput.Request p a)
  | owner (target : Fin p) (request : OwnerEndpoint.Command p a)

inductive Step (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat) : State p a → Event p a → State p a → Prop where
  | frame : Step assigned scope bootstrap capacity state .local state
  | retire : Step assigned scope bootstrap capacity state .retire none
  | query : QueryAllowed s.mode →
      Step assigned scope bootstrap capacity (some s) (.query query) (some {s with ordinal := s.ordinal+1})
  | launch : s.mode = .prelude owed → s.driver.source = none → VectorAt s vector →
      SourceFundingInput.execute assigned scope bootstrap s.driver (vector,.launch program) = (next,.accepted) →
      Step assigned scope bootstrap capacity (some s) (.source (vector,.launch program)) (some (sourceResult s next (.prelude (fun _ => true))))
  | initialize : s.mode = .prelude owed → owed target = true → s.driver.source = some source →
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target)
        (.owner (.initialize (CustodyPublication.image source.publication.current))) = (next,.inl 10) →
      Step assigned scope bootstrap capacity (some s) (.owner target (.owner (.initialize (CustodyPublication.image source.publication.current))))
        (some (ownerResult s target next (.prelude (fun i => if i=target then false else owed i))))
  | extraInitialize : s.mode = .prelude owed → owed target = false → s.driver.source = some source →
      1+demand s.driver.suffix target ≤ credits s target →
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target)
        (.owner (.initialize (CustodyPublication.image source.publication.current))) = (next,reply) →
      Step assigned scope bootstrap capacity (some s) (.owner target (.owner (.initialize (CustodyPublication.image source.publication.current)))) (some (ownerResult s target next (.prelude owed)))
  | preludeDone : s.mode = .prelude owed → (∀ i, owed i = false) →
      Step assigned scope bootstrap capacity (some s) .local (some {s with mode := .ordinary})
  | source : s.mode = .ordinary → PublicJointHistory.publicSource command → VectorAt s vector →
      SourceFundingInput.execute assigned scope bootstrap s.driver (vector,.step command) = (next,.accepted) →
      Step assigned scope bootstrap capacity (some s) (.source (vector,.step command)) (some (sourceResult s next .ordinary))
  | refused : (s.mode = .ordinary ∨ ∃ owed, s.mode = .prelude owed) → VectorAt s vector →
      (SourceFundingInput.execute assigned scope bootstrap s.driver (vector,input)).2 ≠ .accepted →
      Step assigned scope bootstrap capacity (some s) (.source (vector,input)) (some {s with ordinal := s.ordinal+1})
  | administration : s.mode = .ordinary → PublicOwnerBoundary.Admin command →
      ¬ HeadPayment s.driver target command →
      1+demand s.driver.suffix target ≤ credits s target →
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.owners target) command = (next,reply) →
      Step assigned scope bootstrap capacity (some s) (.owner target command) (some (ownerResult s target next .ordinary))
  | pay : s.mode = .ordinary → s.driver.source = some source →
      s.driver.suffix = PaidHeadPhase.command pending :: rest → pending.sourceOrdinal = s.ordinal →
      OwnerEndpointBudget.transition ⟨assigned.realm,pending.target⟩ scope (capacity pending.target)
        (s.owners pending.target) (paymentRequest pending source) = (next,paymentReply pending) →
      Step assigned scope bootstrap capacity (some s) (.owner pending.target (paymentRequest pending source)) (some (ownerResult s pending.target next (.paid pending)))
  | notify : s.mode = .paid pending → PaidHeadPhase.permits pending s.ordinal command = true → VectorAt s vector →
      SourceFundingInput.execute assigned scope bootstrap s.driver (vector,.step command) = (next,.accepted) →
      Step assigned scope bootstrap capacity (some s) (.source (vector,.step command)) (some (sourceResult s next .ordinary))
  | enter : s.mode = .ordinary → VectorAt s vector →
      SourceFundingInput.execute assigned scope bootstrap s.driver (vector,.step (.enter dispatch.endpoint)) = (next,.accepted) →
      next.source = some entered → entered.dispatch = some dispatch → next.suffix = .finish dispatch.endpoint :: rest →
      Step assigned scope bootstrap capacity (some s) (.source (vector,.step (.enter dispatch.endpoint))) (some (sourceResult s next (.entered dispatch)))
  | reserve : s.mode = .entered dispatch →
      OwnerEndpointBudget.transition ⟨assigned.realm,dispatch.endpoint⟩ scope (capacity dispatch.endpoint) (s.owners dispatch.endpoint)
        (.owner (.reserve dispatch.ticket)) = (next,.inl 6) →
      Step assigned scope bootstrap capacity (some s) (.owner dispatch.endpoint (.owner (.reserve dispatch.ticket))) (some (ownerResult s dispatch.endpoint next (.reserved dispatch)))
  | probe : s.mode = .reserved dispatch →
      OwnerEndpointBudget.transition ⟨assigned.realm,dispatch.endpoint⟩ scope (capacity dispatch.endpoint) (s.owners dispatch.endpoint)
        (.freeze revision) = (next,.inl 2) →
      Step assigned scope bootstrap capacity (some s) (.owner dispatch.endpoint (.freeze revision)) (some (ownerResult s dispatch.endpoint next (.probed dispatch)))
  | compute : s.mode = .probed dispatch →
      OwnerEndpointBudget.transition ⟨assigned.realm,dispatch.endpoint⟩ scope (capacity dispatch.endpoint) (s.owners dispatch.endpoint)
        (.owner .compute) = (next,.inr envelope) →
      envelope.ticket = dispatch.ticket → envelope.scopeId = dispatch.context.scopeId → envelope.revision = dispatch.context.revision →
      Step assigned scope bootstrap capacity (some s) (.owner dispatch.endpoint (.owner .compute)) (some (ownerResult s dispatch.endpoint next (.computed dispatch envelope)))
  | finish : s.mode = .computed dispatch envelope → VectorAt s vector →
      SourceFundingInput.execute assigned scope bootstrap s.driver (vector,.step (.finish dispatch.endpoint)) = (next,.accepted) →
      Step assigned scope bootstrap capacity (some s) (.source (vector,.step (.finish dispatch.endpoint))) (some (sourceResult s next .ordinary))

-- Independent actual transition accounting: refusal16 frames credits; every
-- other actual reply spends exactly one. These facts do not authenticate a
-- head payment; Step.pay above additionally fixes the operation and success.
theorem remaining_lower
    (ran : OwnerEndpointBudget.transition assigned scope capacity old command = (next,reply)) :
    old.remaining-1 ≤ next.remaining := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases result : OwnerEndpointProfile.transition assigned scope capacity old.owner command with
    | mk owner response => rw [result] at ran; cases ran; exact Nat.le_refl _
  · cases ran; omega

theorem accepted_funding
    (bound : VectorAt s vector)
    (ran : SourceFundingInput.execute assigned scope bootstrap s.driver (vector,input) = (next,.accepted)) :
    Covered next.suffix (credits s) := by
  intro i
  have enough := (SourceFundingInput.accepted_exact.mp ran).2 i
  rw [bound i]
  change demand next.suffix i ≤ (vector[i.val]).val
  change SourceFundingInput.initialDebit s.driver i+demand next.suffix i ≤ (vector[i.val]).val at enough
  omega

#print axioms remaining_lower
#print axioms accepted_funding


theorem extra_covered
    (covered : Covered s.driver.suffix (credits s))
    (slack : 1+demand s.driver.suffix target ≤ credits s target)
    (ran : OwnerEndpointBudget.transition assigned scope capacity (s.owners target) command = (next,reply)) :
    Covered s.driver.suffix (credits (ownerResult s target next mode)) := by
  intro i
  by_cases equal : i=target
  · subst i
    have lower := remaining_lower ran
    change demand s.driver.suffix target ≤ (PublicOwnerBoundary.put s.owners target next target).remaining
    simp only [PublicOwnerBoundary.put,ite_true]
    change 1+demand s.driver.suffix target ≤ (s.owners target).remaining at slack
    omega
  · simpa [credits,ownerResult,PublicOwnerBoundary.put,equal] using covered i

theorem initialization_covered
    (covered : OwnerFundingCursor.InitCovered s.driver.suffix owed (credits s))
    (due : owed target = true)
    (ran : OwnerEndpointBudget.transition assigned scope capacity (s.owners target) command = (next,.inl 10)) :
    OwnerFundingCursor.InitCovered s.driver.suffix (fun i => if i=target then false else owed i)
      (credits (ownerResult s target next mode)) := by
  have paid := PaidHeadPhase.actual_debit ran (by decide)
  have result := OwnerFundingCursor.initialization_paid covered due
  intro i
  by_cases equal : i=target
  · subst i
    simpa [credits,ownerResult,PublicOwnerBoundary.put,paid] using result target
  · simpa [credits,ownerResult,PublicOwnerBoundary.put,equal] using result i

theorem extra_initialization_covered
    (covered : OwnerFundingCursor.InitCovered s.driver.suffix owed (credits s))
    (done : owed target = false)
    (slack : 1+demand s.driver.suffix target ≤ credits s target)
    (ran : OwnerEndpointBudget.transition assigned scope capacity (s.owners target) command = (next,reply)) :
    OwnerFundingCursor.InitCovered s.driver.suffix owed (credits (ownerResult s target next mode)) := by
  intro i
  by_cases equal : i=target
  · subst i
    have lower := remaining_lower ran
    simp only [done,Bool.false_eq_true,ite_false,Nat.zero_add]
    change demand s.driver.suffix target ≤ (PublicOwnerBoundary.put s.owners target next target).remaining
    simp only [PublicOwnerBoundary.put,ite_true]
    change 1+demand s.driver.suffix target ≤ (s.owners target).remaining at slack
    omega
  · simpa [credits,ownerResult,PublicOwnerBoundary.put,equal] using covered i

theorem nonrefused_positive
    (ran : OwnerEndpointBudget.transition assigned scope capacity old command = (next,reply))
    (paid : reply ≠ .inl 16) : 0 < old.remaining := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · rename_i enough
    have cost := OwnerEndpointBudget.cost_bounds (state:=old.owner) (command:=command)
    omega
  · cases ran; exact False.elim (paid rfl)

theorem work_microstep
    (covered : WorkCovered s dispatch count) (bounded : count < 3)
    (ran : OwnerEndpointBudget.transition assigned scope capacity (s.owners dispatch.endpoint) command = (next,reply))
    (paid : reply ≠ .inl 16) :
    WorkCovered (ownerResult s dispatch.endpoint next mode) dispatch (count+1) := by
  obtain ⟨⟨rest,head⟩,length,covered⟩ := covered
  have debit := PaidHeadPhase.actual_debit ran paid
  have positive := nonrefused_positive ran paid
  refine ⟨⟨rest,head⟩,length,?_⟩
  intro i
  have prior := covered i
  by_cases equal : i=dispatch.endpoint
  · subst i
    simp only [partialDebit,ite_true] at prior ⊢
    have cost : OwnerFundingCursor.headCost s.driver.suffix 0 dispatch.endpoint = 3 := by
      simp [OwnerFundingCursor.headCost,head,commandCost]
    simp only [ownerResult,credits,PublicOwnerBoundary.put,ite_true,debit]
    change count ≤ _ ∧ demand _ _ ≤ (s.owners dispatch.endpoint).remaining+count at prior
    simp only [List.drop_zero,head] at prior
    simp only [OwnerFundingCursor.headCost,head,List.drop_zero,commandCost,ite_true]
    constructor
    · omega
    · omega
  · simpa [ownerResult,credits,PublicOwnerBoundary.put,partialDebit,equal] using prior

#print axioms extra_covered
#print axioms initialization_covered
#print axioms extra_initialization_covered
#print axioms nonrefused_positive
#print axioms work_microstep


theorem query_funding (funded : Funding s) (allowed : QueryAllowed s.mode) :
    Funding {s with ordinal := s.ordinal+1} := by
  cases phase : s.mode <;> simp_all [QueryAllowed,Funding,WorkCovered]
  all_goals first | exact funded | exact funded.2

theorem accepted_ordinary
    (valid : PublicationLifecycle.Invariant assigned scope bootstrap s.driver)
    (bound : VectorAt s vector)
    (ran : SourceFundingInput.execute assigned scope bootstrap s.driver (vector,input) = (next,.accepted)) :
    Invariant assigned scope bootstrap (some (sourceResult s next .ordinary)) := by
  intro actual equal
  cases equal
  refine ⟨?_,?_⟩
  · have preserved := SourceFundingInput.preserves valid (value:=(vector,input))
    simpa [ran] using preserved
  · exact accepted_funding (s:=s) bound ran

theorem step_preserves
    (valid : Invariant assigned scope bootstrap old)
    (step : Step assigned scope bootstrap capacity old event next) : Invariant assigned scope bootstrap next := by
  cases step with
  | frame => exact valid
  | retire => intro actual impossible; cases impossible
  | query allowed =>
    intro actual equal; cases equal
    exact ⟨(valid _ rfl).1,query_funding (valid _ rfl).2 allowed⟩
  | @launch s vector program next owed mode fresh bound ran =>
    intro actual equal; cases equal
    refine ⟨?_,?_⟩
    · have preserved := SourceFundingInput.preserves (valid _ rfl).1 (value:=(vector,.launch program))
      simpa [ran] using preserved
    · have funded := SourceFundingInput.initial_reserved ran fresh
      intro i
      change 1+demand next.suffix i ≤ credits s i
      rw [bound i]
      simpa [SourceFundingInput.credits] using funded i
  | «initialize» mode due sourceAt ran =>
    intro actual equal; cases equal
    refine ⟨(valid _ rfl).1,?_⟩
    exact initialization_covered (by simpa [Funding,mode] using (valid _ rfl).2) due ran
  | extraInitialize mode done sourceAt slack ran =>
    intro actual equal; cases equal
    refine ⟨(valid _ rfl).1,?_⟩
    exact extra_initialization_covered (by simpa [Funding,mode] using (valid _ rfl).2) done slack ran
  | preludeDone mode done =>
    intro actual equal; cases equal
    have funded := OwnerFundingCursor.initialization_done
      (by simpa [Funding,mode] using (valid _ rfl).2) done
    exact ⟨(valid _ rfl).1,by simpa [Funding,credits] using funded.2⟩
  | source mode allowed bound ran => exact accepted_ordinary (valid _ rfl).1 bound ran
  | @refused s vector input phase bound refused =>
    intro actual equal; cases equal
    have allowed : QueryAllowed s.mode := by
      rcases phase with ordinary | ⟨owed,prelude⟩
      · simp [ordinary,QueryAllowed]
      · simp [prelude,QueryAllowed]
    exact ⟨(valid _ rfl).1,query_funding (valid _ rfl).2 allowed⟩
  | administration mode admin offhead slack ran =>
    intro actual equal; cases equal
    exact ⟨(valid _ rfl).1,extra_covered (by simpa [Funding,mode] using (valid _ rfl).2) slack ran⟩
  | pay mode sourceAt head ordinal ran =>
    intro actual equal; cases equal
    refine ⟨(valid _ rfl).1,ordinal.symm,_,head,?_⟩
    apply PaidHeadPhase.residual_covered _ _ (by simpa [head,Funding,mode] using (valid _ rfl).2) ran
    simp [paymentReply]
    split <;> decide
  | notify mode allowed bound ran => exact accepted_ordinary (valid _ rfl).1 bound ran
  | @enter s vector next entered dispatch rest mode bound ran sourceAt dispatchAt head =>
    intro actual equal; cases equal
    refine ⟨?_,⟨_,head⟩,?_⟩
    · have preserved := SourceFundingInput.preserves (valid _ rfl).1 (value:=(vector,.step (.enter dispatch.endpoint)))
      simpa [ran] using preserved
    · have covered := accepted_funding bound ran
      have zero := OwnerFundingCursor.zero_partial.mpr (show OwnerFundingCursor.CoveredAt _ 0 _ from ⟨by omega,by simpa using covered⟩)
      change OwnerFundingCursor.CoveredPartial next.suffix 0 (partialDebit dispatch.endpoint 0) (credits s)
      have zeroDebit : partialDebit dispatch.endpoint 0 = (fun _ => 0) := by funext j; simp [partialDebit]
      rw [zeroDebit]
      exact zero
  | reserve mode ran =>
    intro actual equal; cases equal
    exact ⟨(valid _ rfl).1,work_microstep (count:=0) (by simpa [Funding,mode] using (valid _ rfl).2) (by decide) ran (by decide)⟩
  | probe mode ran =>
    intro actual equal; cases equal
    exact ⟨(valid _ rfl).1,work_microstep (count:=1) (by simpa [Funding,mode] using (valid _ rfl).2) (by decide) ran (by decide)⟩
  | compute mode ran ticket scopeAt revision =>
    intro actual equal; cases equal
    exact ⟨(valid _ rfl).1,work_microstep (count:=2) (by simpa [Funding,mode] using (valid _ rfl).2) (by decide) ran (by simp)⟩
  | finish mode bound ran => exact accepted_ordinary (valid _ rfl).1 bound ran

#print axioms query_funding
#print axioms accepted_ordinary
#print axioms step_preserves

-- This rooted relation is the operational FUNDING/ordinal projection only.
-- It does not assert full native-byte, image, authorization or Joint coupling.
def initial (sourceBudget ownerBudget : Nat) : State p a := some
  ⟨PublicationCapacityDriver.initial sourceBudget,fun _ => OwnerEndpointBudget.initial ownerBudget,
    0,.prelude (fun _ => false)⟩

theorem initial_invariant : Invariant assigned scope bootstrap (initial sourceBudget ownerBudget) := by
  intro s equal
  cases equal
  refine ⟨PublicationLifecycle.initial_invariant assigned scope bootstrap sourceBudget,?_⟩
  simp [Funding,OwnerFundingCursor.InitCovered,PublicationCapacityDriver.initial,demand]

inductive Runs (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (sourceBudget ownerBudget : Nat) : List (Event p a) → State p a → Prop where
  | initial : Runs assigned scope bootstrap capacity sourceBudget ownerBudget [] (initial sourceBudget ownerBudget)
  | step : Runs assigned scope bootstrap capacity sourceBudget ownerBudget events old →
      Step assigned scope bootstrap capacity old event next →
      Runs assigned scope bootstrap capacity sourceBudget ownerBudget (events ++ [event]) next

theorem runs_invariant (path : Runs assigned scope bootstrap capacity sourceBudget ownerBudget events state) :
    Invariant assigned scope bootstrap state := by
  induction path with
  | initial => exact initial_invariant
  | step path step ih => exact step_preserves ih step

theorem retired_absorbing (step : Step assigned scope bootstrap capacity none event next) : next = none := by
  cases step <;> rfl

theorem ordinal_monotone (step : Step assigned scope bootstrap capacity (some s) event (some t)) :
    s.ordinal ≤ t.ordinal := by
  cases step <;> simp [sourceResult,ownerResult]

-- A phase change cannot be obtained from arithmetic spending alone. The
-- exact pending operation and its successful native reply are its provenance.
theorem paid_origin
    (step : Step assigned scope bootstrap capacity (some s) event (some t))
    (ordinary : s.mode = .ordinary) (paid : t.mode = .paid pending) :
    ∃ source rest next,
      s.driver.source = some source ∧ s.driver.suffix = PaidHeadPhase.command pending :: rest ∧
      pending.sourceOrdinal = s.ordinal ∧
      OwnerEndpointBudget.transition ⟨assigned.realm,pending.target⟩ scope (capacity pending.target)
        (s.owners pending.target) (paymentRequest pending source) = (next,paymentReply pending) ∧
      t = ownerResult s pending.target next (.paid pending) ∧
      event = .owner pending.target (paymentRequest pending source) := by
  cases step <;> simp_all [sourceResult,ownerResult]

#print axioms initial_invariant
#print axioms runs_invariant
#print axioms retired_absorbing
#print axioms ordinal_monotone
#print axioms paid_origin

theorem wrong_payment_cannot_advance {p a : Nat}
    {s t : Live p a} {pending : PaidHeadPhase.Pending p} {event : Event p a}
    {assigned : SourceInput.Assignment p a} {scope : Nat}
    {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat}
    (ordinary : s.mode = .ordinary)
    (wrong : ∀ source, s.driver.source = some source →
      event ≠ Event.owner pending.target (paymentRequest pending source))
    (step : Step assigned scope bootstrap capacity (some s) event (some t)) :
    t.mode ≠ .paid pending := by
  intro paid
  obtain ⟨source,rest,next,present,head,ordinal,ran,result,label⟩ := paid_origin step ordinary paid
  exact wrong source present label

theorem pending_query_excluded (pendingAt : s.mode = .paid pending)
    (step : Step assigned scope bootstrap capacity (some s) (.query request) (some t)) : False := by
  cases step with
  | query allowed => simp [pendingAt,QueryAllowed] at allowed

#print axioms wrong_payment_cannot_advance
#print axioms pending_query_excluded
end MirroreaProofFirst.CohortPhase
