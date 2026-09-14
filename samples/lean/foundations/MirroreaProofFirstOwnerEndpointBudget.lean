import MirroreaProofFirstOwnerEndpointProfile

namespace MirroreaProofFirst.OwnerEndpointBudget

-- Semantic transition credits, distinct from wire attempts and OS failures.
-- A budget-refused input spends no semantic credit. Physical fault/CPU termination
-- still retires the private namespace; no availability or recovery is implied.
def held : Option (OwnerEndpoint.State p a) → Bool
  | none => false
  | some s => s.owner.active.isSome

-- These structural preconditions do not run the owner computation. An abandon
-- clears active work; compute does so only when its existing profile/fence admit.
def releases (state : Option (OwnerEndpoint.State p a)) : OwnerEndpoint.Command p a → Bool
  | .owner .abandon => true
  | .owner .compute => match state with
      | none => false
      | some s => OwnerEndpointProfile.activeCheck s.owner && OwnerEndpoint.permitted s .compute
  | _ => false

def needsFinish (state : Option (OwnerEndpoint.State p a)) : OwnerEndpoint.Command p a → Bool
  | .owner (.reserve _) => true
  | command => held state && !releases state command

def cost (state : Option (OwnerEndpoint.State p a)) (command : OwnerEndpoint.Command p a) : Nat :=
  if needsFinish state command then 2 else 1

structure State (p a : Nat) where
  owner : Option (OwnerEndpoint.State p a)
  remaining : Nat

def initial (remaining : Nat) : State p a := ⟨none,remaining⟩

def transition (assigned : OwnerEvaluator.Assignment p) (scopeId capacity : Nat)
    (state : State p a) (command : OwnerEndpoint.Command p a) :
    State p a × Sum Nat OwnerReceipt.Envelope :=
  if cost state.owner command ≤ state.remaining then
    let (next,reply) := OwnerEndpointProfile.transition assigned scopeId capacity state.owner command
    (⟨next,state.remaining-1⟩,reply)
  else (state,.inl 16)

def Invariant (state : State p a) : Prop :=
  OwnerEndpointProfile.Invariant state.owner ∧ (held state.owner = true → 0 < state.remaining)

theorem initial_valid : Invariant (initial (p:=p) (a:=a) remaining) := by
  simp [Invariant,initial,OwnerEndpointProfile.Invariant,held]

theorem cost_bounds : 1 ≤ cost state command ∧ cost state command ≤ 2 := by
  unfold cost; split <;> omega

theorem rejected_unchanged (insufficient : state.remaining < cost state.owner command) :
    transition assigned scopeId capacity state command = (state,.inl 16) := by
  simp [transition,Nat.not_le.mpr insufficient]

theorem funded_exact {state : State p a} (funded : cost state.owner command ≤ state.remaining) :
    transition assigned scopeId capacity state command =
      let (next,reply) := OwnerEndpointProfile.transition assigned scopeId capacity state.owner command
      ((⟨next,state.remaining-1⟩ : State p a),reply) := by simp [transition,funded]

-- No command except reserve can create an active reservation from idle.
theorem idle_no_reserve {state next : Option (OwnerEndpoint.State p a)}
    {command : OwnerEndpoint.Command p a}
    (idle : held state = false)
    (nonreserve : ∀ ticket, command ≠ .owner (.reserve ticket))
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity state command = (next,reply)) : held next = false := by
  unfold OwnerEndpointProfile.transition at ran
  split at ran
  · cases state with
    | none =>
      cases command with
      | freeze revision => cases ran; rfl
      | owner command =>
        cases command <;> simp only [OwnerEndpoint.transition,OwnerReservationWorker.transition] at ran
        case «initialize» image => split at ran <;> cases ran <;> rfl
        all_goals cases ran; rfl
    | some s =>
      have inactive : s.owner.active = none := by simpa [held] using idle
      cases command with
      | freeze revision => simp [OwnerEndpoint.transition,inactive] at ran; rcases ran with ⟨rfl,rfl⟩; exact idle
      | owner command =>
        cases command with
        | «initialize» image => cases ran; exact idle
        | reserve ticket => exact False.elim (nonreserve ticket rfl)
        | compute =>
          simp only [OwnerEndpoint.transition,OwnerEndpoint.confirms,Bool.false_eq_true,ite_false] at ran
          split at ran
          · simp [OwnerReservationWorker.transition,OwnerReservation.compute,inactive] at ran; rcases ran with ⟨rfl,rfl⟩; exact idle
          · cases ran; exact idle
        | install revision image =>
          simp only [OwnerEndpoint.transition] at ran
          split at ran
          · cases ran; exact idle
          · split at ran
            · simp only [OwnerReservationWorker.transition] at ran
              split at ran
              · simp [OwnerReservation.install,inactive] at ran; rcases ran with ⟨rfl,rfl⟩; rfl
              · cases ran; exact idle
            · cases ran; exact idle
        | abandon => cases ran; rfl
  · cases ran; exact idle

theorem worker_compute_idle
    (ran : OwnerReservationWorker.transition assigned scopeId capacity (some s) .compute = (next,reply)) :
    ∀ value, next = some value → value.active = none := by
  cases computed : OwnerReservation.compute s with
  | none =>
      have idle : s.active = none := by
        cases active : s.active with
        | none => rfl
        | some ticket => simp [OwnerReservation.compute,active] at computed
      simp only [OwnerReservationWorker.transition,computed,Prod.mk.injEq] at ran
      intro value same
      have equal := Option.some.inj (ran.1.trans same)
      simpa [← equal] using idle
  | some pair =>
      obtain ⟨value,response⟩ := pair
      obtain ⟨_,_,_,_,atNext⟩ := OwnerReservation.compute_parts computed
      simp only [OwnerReservationWorker.transition,computed,Prod.mk.injEq] at ran
      intro other same
      have equal := Option.some.inj (ran.1.trans same)
      rw [← equal,atNext]

theorem released_idle
    (release : releases state command = true)
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity state command = (next,reply)) : held next = false := by
  cases command with
  | freeze revision => cases release
  | owner command =>
    cases command with
    | «initialize» image => cases release
    | reserve ticket => cases release
    | install revision image => cases release
    | abandon => cases state <;> cases ran <;> rfl
    | compute =>
      cases state with
      | none => cases release
      | some s =>
        have checks : OwnerEndpointProfile.activeCheck s.owner = true ∧ OwnerEndpoint.permitted s .compute = true := by
          simpa [releases] using release
        simp only [OwnerEndpointProfile.transition,OwnerEndpointProfile.check,checks.1,ite_true,
          OwnerEndpoint.transition,OwnerEndpoint.confirms,Bool.false_eq_true,ite_false,checks.2] at ran
        cases previous : OwnerReservationWorker.transition assigned scopeId capacity (some s.owner) .compute with
        | mk owner response =>
          rw [previous] at ran
          cases owner with
          | none => cases ran; rfl
          | some value =>
            have idle := worker_compute_idle previous value rfl
            cases ran; simp [held,idle]

theorem held_needs_two
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity state command = (next,reply))
    (active : held next = true) : cost state command = 2 := by
  have oldOrReserve : held state = true ∨ ∃ ticket, command = .owner (.reserve ticket) := by
    by_cases old : held state = true
    · exact Or.inl old
    · have before : held state = false := by simpa using old
      by_cases reserve : ∃ ticket, command = .owner (.reserve ticket)
      · exact Or.inr reserve
      · have nonreserve : ∀ ticket, command ≠ .owner (.reserve ticket) := fun ticket same => reserve ⟨ticket,same⟩
        have empty := idle_no_reserve before nonreserve ran
        rw [empty] at active; cases active
  rcases oldOrReserve with old | ⟨ticket,rfl⟩
  · have noRelease : releases state command = false := by
      by_cases yes : releases state command = true
      · have empty := released_idle yes ran
        rw [empty] at active; cases active
      · simpa using yes
    cases command with
    | freeze revision => simp [cost,needsFinish,old,noRelease]
    | owner command => cases command <;> simp [cost,needsFinish,old,noRelease]
  · rfl

theorem preserves (valid : Invariant state)
    (ran : transition assigned scopeId capacity state command = (next,reply)) : Invariant next := by
  unfold transition at ran
  split at ran
  · rename_i funded
    cases previous : OwnerEndpointProfile.transition assigned scopeId capacity state.owner command with
    | mk owner response =>
      rw [previous] at ran
      cases ran
      refine ⟨OwnerEndpointProfile.preserves valid.1 previous,?_⟩
      intro active
      have costAt := held_needs_two previous active
      rw [costAt] at funded
      change 0 < state.remaining - 1
      omega
  · cases ran; exact valid

inductive Runs (assigned : OwnerEvaluator.Assignment p) (scopeId capacity budget : Nat) : State p a → Prop where
  | fresh : Runs assigned scopeId capacity budget (initial budget)
  | next : Runs assigned scopeId capacity budget state →
      transition assigned scopeId capacity state command = (next,reply) →
      Runs assigned scopeId capacity budget next

theorem rooted_valid (path : Runs assigned scopeId capacity budget state) : Invariant state := by
  induction path with
  | fresh => exact initial_valid
  | next _ ran ih => exact preserves ih ran

theorem exhausted_idle (valid : Invariant state) (empty : state.remaining = 0) : held state.owner = false := by
  by_cases active : held state.owner = true
  · have room := valid.2 active; omega
  · simpa using active

#print axioms initial_valid
#print axioms cost_bounds
#print axioms rejected_unchanged
#print axioms funded_exact
#print axioms idle_no_reserve
#print axioms worker_compute_idle
#print axioms released_idle
#print axioms held_needs_two
#print axioms preserves
#print axioms rooted_valid
#print axioms exhausted_idle

theorem produced_releases
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity state (.owner .compute) = (next,.inr envelope)) :
    releases state (.owner .compute) = true := by
  cases state with
  | none => simp [OwnerEndpointProfile.transition,OwnerEndpointProfile.check,
      OwnerEndpoint.transition,OwnerReservationWorker.transition] at ran
  | some s =>
    simp only [OwnerEndpointProfile.transition,OwnerEndpointProfile.check] at ran
    split at ran
    · rename_i profile
      simp only [OwnerEndpoint.transition,OwnerEndpoint.confirms,Bool.false_eq_true,ite_false] at ran
      split at ran
      · rename_i permitted; simp [releases,profile,permitted]
      · cases ran
    · cases ran

theorem produced_compute_cost
    (ran : OwnerEndpointProfile.transition assigned scopeId capacity state (.owner .compute) = (next,.inr envelope)) :
    cost state (.owner .compute) = 1 := by
  simp [cost,needsFinish,produced_releases ran]

theorem refusal_readable :
    OwnerResponseProfile.WireFits OwnerReservationWorker.replyCodec 256 (.inl 16) := by
  have number := OwnerResponseProfile.fits_natural (show 16 < 2^5 by decide) (by decide : 5 ≤ 128) (textLimit:=256)
  exact OwnerResponseProfile.fits_wire (OwnerResponseProfile.fits_inl number OwnerReceipt.codec) (by decide) (by decide)

theorem every_reply_readable
    (ran : transition assigned scopeId capacity state command = (next,reply)) :
    OwnerResponseProfile.WireFits OwnerReservationWorker.replyCodec 256 reply := by
  unfold transition at ran
  split at ran
  · cases previous : OwnerEndpointProfile.transition assigned scopeId capacity state.owner command with
    | mk owner response =>
      rw [previous] at ran
      cases ran
      exact OwnerEndpointProfile.every_reply_readable previous
  · cases ran; exact refusal_readable

-- Independent semantic/profile/resource premises yield both funded transitions.
-- The derived model compute is a proof witness, not a native dispatch event.
theorem enabled_roundtrip {s : OwnerEndpoint.State p a}
    (path : OwnerReservation.Steps (OwnerReservation.initial initialAssigned initialScope initialImage initialCapacity) s.owner)
    (small : s.owner.core.capacity ≤ 64)
    (profile : OwnerResponseProfile.ResponseFits s.owner.core.scopeId s.owner.core.revision ticket)
    (current : s.owner.core.revision = s.fence)
    (scope : scopeId = s.owner.core.scopeId)
    (admitted : OwnerEvaluator.Admitted s.owner.core.assigned s.owner.core.image ticket)
    (idle : s.owner.active = none) (fresh : OwnerReservation.hasKey s.owner ticket = false)
    (room : s.owner.reserved.length < s.owner.core.capacity) (funded : 2 ≤ credits) :
    ∃ (reserved computed : State p a) (record : OwnerOccurrence.Record p a),
      transition assigned scopeId capacity ⟨some s,credits⟩ (.owner (.reserve ticket)) = (reserved,.inl 6) ∧
      transition assigned scopeId capacity reserved (.owner .compute) = (computed,.inr (OwnerReceipt.project record)) ∧
      reserved.remaining = credits-1 ∧ computed.remaining = credits-2 ∧
      OwnerResponseProfile.WireFits OwnerReservationWorker.replyCodec 256 (.inr (OwnerReceipt.project record)) ∧
      OwnerResponseProfile.WireFits (PublicationInput.input p a) 256 (.step (.arrival (OwnerReceipt.project record))) := by
  obtain ⟨owner,later,record,first,second,readable,arrival⟩ :=
    OwnerEndpointProfile.enabled_roundtrip (assigned:=assigned) (capacity:=capacity)
      path small profile current scope admitted idle fresh room
  have computeCost := produced_compute_cost second
  refine ⟨⟨some owner,credits-1⟩,⟨some later,credits-1-1⟩,record,?_,?_,rfl,?_,readable,arrival⟩
  · rw [funded_exact (show cost (some s) (.owner (.reserve ticket)) ≤ credits from funded)]
    simp [first]
  · have nextFunded : cost (some owner) (.owner .compute) ≤ credits-1 := by rw [computeCost]; omega
    rw [funded_exact nextFunded]
    simp [second]
  · simp [Nat.sub_sub]

#print axioms produced_releases
#print axioms produced_compute_cost
#print axioms refusal_readable
#print axioms every_reply_readable
#print axioms enabled_roundtrip
end MirroreaProofFirst.OwnerEndpointBudget
