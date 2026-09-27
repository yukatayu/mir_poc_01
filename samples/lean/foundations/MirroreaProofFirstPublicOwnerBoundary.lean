import MirroreaProofFirstSourceRegistration

namespace MirroreaProofFirst.PublicOwnerBoundary

-- Owner projection at completed OUTER gate-owning public operations, not an
-- atomicity claim for their internals. A failed reentrant gate acquisition
-- during work is not another boundary. Source/query/PRE-owner-IO local
-- rejection steps frame this projection; admin refusal takes the actual budget
-- transition and can debit. This projection deliberately forgets pending
-- payment/ordinal/source fields, rather than requiring pending = none.
-- Unknown/invalid IO retires; the physical post-failure state is NOT asserted idle.
-- Refinement of all Python/byte branches to this relation remains a separate
-- obligation. In particular private raw reserve is not a public operation.
def Admin : OwnerEndpoint.Command p a → Prop
  | .owner (.initialize _) | .owner (.install _ _) | .freeze _ => True
  | _ => False

def Idle (owners : Fin p → OwnerEndpointBudget.State p a) : Prop :=
  ∀ i, OwnerEndpointBudget.held (owners i).owner = false

theorem admin_nonreserve (admin : Admin command) :
    ∀ ticket, command ≠ .owner (.reserve ticket) := by
  intro ticket equal
  subst command
  cases admin

theorem budget_idle_no_reserve {p a : Nat}
    {state next : OwnerEndpointBudget.State p a} {command : OwnerEndpoint.Command p a}
    {assigned : OwnerEvaluator.Assignment p} {scopeId capacity : Nat} {reply : Sum Nat OwnerReceipt.Envelope}
    (idle : OwnerEndpointBudget.held state.owner = false)
    (nonreserve : ∀ ticket, command ≠ .owner (.reserve ticket))
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state command = (next,reply)) :
    OwnerEndpointBudget.held next.owner = false := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases underlying : OwnerEndpointProfile.transition assigned scopeId capacity state.owner command with
    | mk owner result =>
      rw [underlying] at ran
      cases ran
      exact OwnerEndpointBudget.idle_no_reserve idle nonreserve underlying
  · cases ran
    exact idle

theorem budget_produced_idle
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state (.owner .compute) = (next,.inr envelope)) :
    OwnerEndpointBudget.held next.owner = false := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases underlying : OwnerEndpointProfile.transition assigned scopeId capacity state.owner (.owner .compute) with
    | mk owner result =>
      rw [underlying] at ran
      cases ran
      exact OwnerEndpointBudget.released_idle (OwnerEndpointBudget.produced_releases underlying) underlying
  · cases ran

-- Successful return expands all three actual owner debits. There is no premise
-- that the endpoint is idle at return, and no inference from lease cancellation.
inductive Work (assigned : OwnerEvaluator.Assignment p) (scopeId capacity : Nat)
    (ticket : OwnerOccurrence.Ticket) (probeRevision : Nat) :
    OwnerEndpointBudget.State p a → OwnerEndpointBudget.State p a → Prop where
  | run :
      OwnerEndpointBudget.transition assigned scopeId capacity old (.owner (.reserve ticket)) = (reserved,.inl 6) →
      OwnerEndpointBudget.transition assigned scopeId capacity reserved (.freeze probeRevision) = (probed,.inl 2) →
      OwnerEndpointBudget.transition assigned scopeId capacity probed (.owner .compute) = (finished,.inr envelope) →
      Work assigned scopeId capacity ticket probeRevision old finished

theorem work_idle (work : Work assigned scopeId capacity ticket probeRevision old next) :
    OwnerEndpointBudget.held next.owner = false := by
  cases work with
  | run reserved probe computed => exact budget_produced_idle computed

abbrev Owners (p a : Nat) := Fin p → OwnerEndpointBudget.State p a
-- none represents a retired handle, not erased physical resources.
abbrev State (p a : Nat) := Option (Owners p a)

def initial (budget : Nat) : State p a := some (fun _ => OwnerEndpointBudget.initial budget)

def put (owners : Owners p a) (target : Fin p) (next : OwnerEndpointBudget.State p a) : Owners p a :=
  fun i => if i = target then next else owners i

inductive Step {p a : Nat} (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat)
    (capacity : Fin p → Nat) : State p a → State p a → Prop where
  -- Only owner-framing effects: source/query or pre-owner-IO refusal.
  -- An admin refusal with a real debit belongs to admin, never frame.
  | frame : Step assigned scopeId capacity state state
  | admin {owners : Owners p a} {target : Fin p} {command : OwnerEndpoint.Command p a}
      {next : OwnerEndpointBudget.State p a} {reply : Sum Nat OwnerReceipt.Envelope} : Admin command →
      OwnerEndpointBudget.transition (assigned target) scopeId (capacity target) (owners target) command = (next,reply) →
      Step assigned scopeId capacity (some owners) (some (put owners target next))
  | work {owners : Owners p a} {target : Fin p} {ticket : OwnerOccurrence.Ticket}
      {probeRevision : Nat} {next : OwnerEndpointBudget.State p a} : Work (assigned target) scopeId (capacity target) ticket probeRevision (owners target) next →
      Step assigned scopeId capacity (some owners) (some (put owners target next))
  | retire : Step assigned scopeId capacity state none

def Invariant (s : State p a) : Prop := ∀ owners, s = some owners → Idle owners

theorem step_preserves (valid : Invariant old) (step : Step assigned scopeId capacity old next) :
    Invariant next := by
  cases step with
  | frame => exact valid
  | retire => intro owners impossible; cases impossible
  | @admin owners target command after reply allowed ran =>
    intro actual equal
    cases equal
    intro i
    by_cases same : i = target
    · subst i
      simpa [put] using budget_idle_no_reserve (valid owners rfl target) (admin_nonreserve allowed) ran
    · simpa [put,same] using valid owners rfl i
  | @work owners target ticket revision after work =>
    intro actual equal
    cases equal
    intro i
    by_cases same : i = target
    · subst i
      simpa [put] using work_idle work
    · simpa [put,same] using valid owners rfl i

inductive Runs {p a : Nat} (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat)
    (capacity : Fin p → Nat) (budget : Nat) : State p a → Prop where
  | fresh : Runs assigned scopeId capacity budget (initial budget)
  | step : Runs assigned scopeId capacity budget old → Step assigned scopeId capacity old next →
      Runs assigned scopeId capacity budget next

theorem continuing_idle {p a : Nat} {owners : Owners p a}
    {assigned : Fin p → OwnerEvaluator.Assignment p} {scopeId budget : Nat} {capacity : Fin p → Nat} (path : Runs assigned scopeId capacity budget (some owners)) : Idle owners := by
  have preserved {s : State p a} (path : Runs assigned scopeId capacity budget s) : Invariant s := by
    induction path with
    | fresh => intro owners equal; cases equal; intro i; rfl
    | step prior step ih => exact step_preserves ih step
  exact preserved path owners rfl

theorem retired_absorbing (step : Step assigned scopeId capacity none next) : next = none := by
  cases step <;> rfl

theorem actual_present_idle (path : Runs assigned scopeId capacity budget (some owners))
    (present : (owners i).owner = some owner) : owner.owner.active = none := by
  have idle := continuing_idle path i
  simpa [OwnerEndpointBudget.held,present] using idle

#print axioms admin_nonreserve
#print axioms budget_idle_no_reserve
#print axioms budget_produced_idle
#print axioms work_idle
#print axioms step_preserves
#print axioms continuing_idle
#print axioms retired_absorbing
#print axioms actual_present_idle

-- Semantic discriminator: exposing a successful raw reservation as a completed
-- public operation violates the idle boundary; a source/writer lease is irrelevant.
theorem profile_reserved_busy
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity state (.owner (.reserve ticket)) = (next,.inl 6)) :
    OwnerEndpointBudget.held next = true := by
  cases state with
  | none => simp [OwnerEndpointProfile.transition,OwnerEndpointProfile.check,
      OwnerEndpoint.transition,OwnerReservationWorker.transition] at ran
  | some owner =>
    simp only [OwnerEndpointProfile.transition] at ran
    split at ran
    · simp only [OwnerEndpoint.transition,OwnerEndpoint.confirms,Bool.false_eq_true,ite_false] at ran
      split at ran
      · cases reserved : OwnerReservation.reserve owner.owner scopeId ticket with
        | mk after response =>
          simp only [OwnerReservationWorker.transition,reserved,Option.map_some,Prod.mk.injEq] at ran
          cases response <;> simp only [OwnerReservationWorker.responseCode,Sum.inl.injEq] at ran
          all_goals try omega
          obtain ⟨rfl,_⟩ := ran
          have active : after.active = some ticket := by rw [(OwnerReservation.reserve_parts reserved).2.2.2.2.2]
          simp [OwnerEndpointBudget.held,active]
      · cases ran
    · cases ran

theorem budget_reserved_busy
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity state (.owner (.reserve ticket)) = (next,.inl 6)) :
    OwnerEndpointBudget.held next.owner = true := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases underlying : OwnerEndpointProfile.transition assigned scopeId capacity state.owner (.owner (.reserve ticket)) with
    | mk owner result =>
      rw [underlying] at ran
      cases ran
      exact profile_reserved_busy underlying
  · cases ran

theorem no_raw_reserve_boundary {p a : Nat} {owners : Owners p a} {target : Fin p}
    {capacity : Fin p → Nat} {next : OwnerEndpointBudget.State p a}
    {assigned : OwnerEvaluator.Assignment p} {scopeId : Nat} {ticket : OwnerOccurrence.Ticket}
    (ran : OwnerEndpointBudget.transition assigned scopeId (capacity target) (owners target)
      (.owner (.reserve ticket)) = (next,.inl 6)) :
    ¬ Idle (put owners target next) := by
  intro idle
  have atTarget : OwnerEndpointBudget.held next.owner = false := by simpa [put] using idle target
  have busy := budget_reserved_busy ran
  rw [atTarget] at busy
  cases busy

-- Constructive success from independent semantic/resource premises. This
-- remains conditional on actual room/freshness and is not all-source progress.
theorem enabled_work {s : OwnerEndpoint.State p a}
    (path : OwnerReservation.Steps (OwnerReservation.initial initialAssigned initialScope initialImage initialCapacity) s.owner)
    (small : s.owner.core.capacity ≤ 64)
    (profile : OwnerResponseProfile.ResponseFits s.owner.core.scopeId s.owner.core.revision ticket)
    (current : s.owner.core.revision = s.fence)
    (scope : scopeId = s.owner.core.scopeId)
    (admitted : OwnerEvaluator.Admitted s.owner.core.assigned s.owner.core.image ticket)
    (idle : s.owner.active = none) (fresh : OwnerReservation.hasKey s.owner ticket = false)
    (room : s.owner.reserved.length < s.owner.core.capacity) (funded : 3 ≤ credits) :
    ∃ next, Work assigned scopeId capacity ticket revision ⟨some s,credits⟩ next ∧
      OwnerEndpointBudget.held next.owner = false := by
  obtain ⟨reserved,probed,computed,record,first,second,third,rest⟩ :=
    OwnerWorkInterval.enabled_interval (assigned:=assigned) (capacity:=capacity) (revision:=revision)
      path small profile current scope admitted idle fresh room funded
  exact ⟨computed,.run first second third,budget_produced_idle third⟩

#print axioms profile_reserved_busy
#print axioms budget_reserved_busy
#print axioms no_raw_reserve_boundary
#print axioms enabled_work

end MirroreaProofFirst.PublicOwnerBoundary
