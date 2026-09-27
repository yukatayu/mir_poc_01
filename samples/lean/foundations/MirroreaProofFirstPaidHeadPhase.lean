import MirroreaProofFirstPublicJointHistory

namespace MirroreaProofFirst.PaidHeadPhase
open PublicationOwnerBudget

-- A pending payment names one occurrence at one private source transport
-- ordinal. Freeze and installation are both represented. This is distinct
-- from SourceOwnerFloor.pending, which projects only unnotified freezes.
structure Pending (p : Nat) where
  freeze : Bool
  target : Fin p
  revision : Nat
  sourceOrdinal : Nat

def command (pending : Pending p) : PublicationInput.Command p a :=
  if pending.freeze then .freeze pending.target pending.revision else .install pending.target pending.revision

def permits (pending : Pending p) (ordinal : Nat) (input : PublicationInput.Command p a) : Bool :=
  decide (ordinal = pending.sourceOrdinal) &&
    decide (OwnerPacketCodec.encode (PublicationInput.command p a) input =
      OwnerPacketCodec.encode (PublicationInput.command p a) (command pending))

theorem permits_exact {p a : Nat} {pending : Pending p} {ordinal : Nat}
    {input : PublicationInput.Command p a} : permits pending ordinal input = true ↔
    ordinal = pending.sourceOrdinal ∧ input = command pending := by
  simp only [permits,Bool.and_eq_true,decide_eq_true_eq]
  constructor
  · rintro ⟨atOrdinal,encoded⟩
    have decoded := congrArg (OwnerPacketCodec.decode (PublicationInput.command p a)) encoded
    rw [OwnerPacketCodec.roundtrip,OwnerPacketCodec.roundtrip] at decoded
    exact ⟨atOrdinal,Option.some.inj decoded⟩
  · rintro ⟨atOrdinal,equal⟩
    exact ⟨atOrdinal,congrArg (OwnerPacketCodec.encode (PublicationInput.command p a)) equal⟩

-- Not a claim that ordinal comparison authenticates a request. Custody of
-- the ordinal and actual transition history remains an implementation duty.
theorem moved_refused (moved : ordinal ≠ pending.sourceOrdinal) :
    permits pending ordinal input = false := by
  simp [permits,moved]

theorem wrong_notice_refused (wrong : input ≠ command pending) :
    permits pending ordinal input = false := by
  cases check : permits pending ordinal input with
  | false => rfl
  | true => exact False.elim (wrong (permits_exact.mp check).2)

theorem actual_debit
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity old request = (next,reply))
    (paid : reply ≠ .inl 16) : next.remaining = old.remaining - 1 := by
  unfold OwnerEndpointBudget.transition at ran
  split at ran
  · cases result : OwnerEndpointProfile.transition assigned scopeId capacity old.owner request with
    | mk owner response => rw [result] at ran; cases ran; rfl
  · cases ran; exact False.elim (paid rfl)

def afterOwners (owners : Fin p → OwnerEndpointBudget.State p a) (target : Fin p)
    (next : OwnerEndpointBudget.State p a) := PublicOwnerBoundary.put owners target next

theorem residual_covered {p a : Nat} {owners : Fin p → OwnerEndpointBudget.State p a}
    {assigned : OwnerEvaluator.Assignment p} {scopeId capacity : Nat}
    {request : OwnerEndpoint.Command p a} {next : OwnerEndpointBudget.State p a}
    {reply : Sum Nat OwnerReceipt.Envelope}
    (pending : Pending p) (rest : List (PublicationInput.Command p a))
    (covered : Covered (command pending :: rest) (fun i => (owners i).remaining))
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity (owners pending.target) request = (next,reply))
    (paid : reply ≠ .inl 16) :
    Covered rest (fun i => (afterOwners owners pending.target next i).remaining) := by
  have debit := actual_debit ran paid
  have tail := covered_tail covered
  intro i
  have enough := tail i
  by_cases same : i = pending.target
  · subst i
    cases kind : pending.freeze <;>
      simpa [afterOwners,PublicOwnerBoundary.put,debit,command,kind,commandCost] using enough
  · cases kind : pending.freeze <;>
      simpa [afterOwners,PublicOwnerBoundary.put,same,command,kind,commandCost] using enough

-- General discriminator for BOTH head kinds: after a real debit at exact
-- funding, the unchanged source suffix cannot still be fully covered.
theorem exact_payment_not_old_covered
    (pending : Pending p) (rest : List (PublicationInput.Command p a))
    (tight : (owners pending.target).remaining = demand (command pending :: rest) pending.target)
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity (owners pending.target) request = (next,reply))
    (paid : reply ≠ .inl 16) :
    ¬ Covered (command pending :: rest) (fun i => (afterOwners owners pending.target next i).remaining) := by
  intro covered
  have atTarget := covered pending.target
  have debit := actual_debit ran paid
  have cost : commandCost (command pending : PublicationInput.Command p a) pending.target = 1 := by
    cases kind : pending.freeze <;> simp [command,kind,commandCost]
  rw [demand_cons,cost] at tight atTarget
  simp only [afterOwners,PublicOwnerBoundary.put,ite_true,debit] at atTarget
  omega

-- Completion from residual coverage, without requiring the paid head to be
-- funded a second time. The source's certified sequence supplies an actual
-- next transition; the vector supplies all current owner coordinates.
theorem residual_notification_admitted
    {p a : Nat} {assigned : SourceInput.Assignment p a} {scopeId : Nat}
    {seed : SourceInput.Bootstrap a} {old : PublicationCapacityDriver.State p a}
    {source : PublicationInput.State p a} {input : PublicationInput.Command p a}
    {rest : List (PublicationInput.Command p a)} {actual : Vector (Fin 513) p}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old)
    (present : old.source = some source) (head : old.suffix = input::rest)
    (covered : Covered rest (SourceFundingInput.credits (actual,.step input))) :
    ∃ next, SourceFundingInput.execute assigned scopeId seed old (actual,.step input) = (next,.accepted) ∧
      next.suffix = rest ∧ PublicationLifecycle.Invariant assigned scopeId seed next ∧
      Covered next.suffix (SourceFundingInput.credits (actual,.step input)) := by
  obtain ⟨space,final,path,quiet⟩ := valid.2 source present
  rw [head] at space path
  cases path with
  | cons fit executed tail =>
    rename_i middle
    cases budget : old.remaining with
    | zero => simp [budget] at space
    | succ remaining =>
      have base := PublicationLifecycle.scheduled_successor valid present head budget executed
      have fast := (PublicationLifecycle.transitionFast_exact valid (input:=.step input)).trans base
      let next : PublicationCapacityDriver.State p a := ⟨some middle,remaining,rest⟩
      have admitted : SourceFundingInput.execute assigned scopeId seed old (actual,.step input) = (next,.accepted) := by
        apply SourceFundingInput.accepted_exact.mpr
        refine ⟨fast,?_⟩
        intro owner
        simpa [SourceFundingInput.initialDebit,present,next] using covered owner
      refine ⟨next,admitted,rfl,?_,covered⟩
      have preserved := SourceFundingInput.preserves valid (value:=(actual,.step input))
      simpa [admitted] using preserved

#print axioms permits_exact
#print axioms moved_refused
#print axioms wrong_notice_refused
#print axioms actual_debit
#print axioms residual_covered
#print axioms exact_payment_not_old_covered
#print axioms residual_notification_admitted

end MirroreaProofFirst.PaidHeadPhase
