import MirroreaProofFirstPublicationOwnerBudget

namespace MirroreaProofFirst.OwnerFundingCursor
open PublicationOwnerBudget

-- Accounting for an unchanged, certified source suffix while real owner work
-- has completed but its source notification has not yet been submitted.
-- paid is a prefix length, NOT an authenticated receipt or a success claim.
-- A physical consumer must supply the matching actual completed operations in
-- order, and couple notify to an accepted source transition. This algebra
-- never authorizes, executes, acknowledges, or reconstructs those operations.
def CoveredAt (suffix : List (PublicationInput.Command p a)) (paid : Nat)
    (credits : Fin p → Nat) : Prop :=
  paid ≤ suffix.length ∧ Covered (suffix.drop paid) credits

def chargeCheck (suffix : List (PublicationInput.Command p a)) (paid : Nat)
    (credits debit : Fin p → Nat) : Bool :=
  (List.finRange p).all (fun owner => decide
    (debit owner + demand (suffix.drop paid) owner ≤ credits owner))

def ChargeAdmissible (suffix : List (PublicationInput.Command p a)) (paid : Nat)
    (credits debit : Fin p → Nat) : Prop :=
  ∀ owner, debit owner + demand (suffix.drop paid) owner ≤ credits owner

theorem charge_exact : chargeCheck suffix paid credits debit = true ↔
    ChargeAdmissible suffix paid credits debit := by
  simp [chargeCheck,ChargeAdmissible,List.all_eq_true]

-- An unrelated owner command may spend only slack. This does not say the
-- command is otherwise permitted, or that its debit is a real owner cost.
theorem charge_preserves (covered : CoveredAt suffix paid credits)
    (allowed : ChargeAdmissible suffix paid credits debit) :
    CoveredAt suffix paid (fun owner => credits owner - debit owner) := by
  refine ⟨covered.1,?_⟩
  intro owner
  have enough := allowed owner
  change demand (suffix.drop paid) owner ≤ credits owner - debit owner
  omega

-- One actual completed head is counted once; later notifications do not spend
-- it again. A physical result is not supplied by this arithmetic update.
theorem completed_head (covered : CoveredAt suffix paid credits)
    (next : suffix.drop paid = command :: rest) :
    CoveredAt suffix (paid+1) (fun owner => credits owner-commandCost command owner) := by
  have tailFunded := covered_tail (credits:=credits) (by simpa [next] using covered.2)
  have lengthAt := congrArg List.length next
  have nextDrop : suffix.drop (paid+1) = rest := by
    rw [← List.drop_drop]
    simp [next]
  refine ⟨?_,?_⟩
  · simp only [List.length_drop,List.length_cons] at lengthAt
    omega
  · simpa [nextDrop] using tailFunded

-- Submission of the first completed notification consumes one accounting
-- position. paid>0 is required; no fabricated completion is inferred from a
-- command's syntax or from a successful source-local validation alone.
theorem notified_head (covered : CoveredAt (command::rest) (paid+1) credits) :
    CoveredAt rest paid credits := by
  constructor
  · have bound := covered.1
    simp only [List.length_cons] at bound
    omega
  · simpa using covered.2

theorem completed_and_notified (covered : CoveredAt (command::rest) 0 credits) :
    CoveredAt rest 0 (fun owner => credits owner-commandCost command owner) := by
  exact notified_head (completed_head covered rfl)

-- Replanning before an unnotified completion is a separate lifecycle problem.
-- With no such completion outstanding, the executable suffix checker exactly
-- establishes the accounting entry condition for the new certified suffix.
theorem new_plan_exact : check suffix credits = true ↔ CoveredAt suffix 0 credits := by
  simp [CoveredAt,check_exact]

-- A finite sequence of completed commands can be accounted before their
-- notifications. Aggregate debit is exactly the sum of this one prefix.
theorem completed_prefix (covered : Covered (done++rest) credits) :
    CoveredAt (done++rest) done.length (fun owner => credits owner-demand done owner) := by
  refine ⟨by simp,?_⟩
  intro owner
  have enough := covered owner
  rw [demand_append] at enough
  change demand ((done++rest).drop done.length) owner ≤ credits owner-demand done owner
  simp only [List.drop_left]
  omega

#print axioms charge_exact
#print axioms charge_preserves
#print axioms completed_head
#print axioms notified_head
#print axioms completed_and_notified
#print axioms new_plan_exact
#print axioms completed_prefix

-- A three-transition work recipe can be partly paid before its finish
-- notification. Each entry below is an ACCOUNTING operation after known IO;
-- the physical recipe/occurrence relation is separate and indispensable.
def headCost (suffix : List (PublicationInput.Command p a)) (paid : Nat) (owner : Fin p) : Nat :=
  match suffix.drop paid with | [] => 0 | command::_ => commandCost command owner

def CoveredPartial (suffix : List (PublicationInput.Command p a)) (paid : Nat)
    (partialPaid credits : Fin p → Nat) : Prop :=
  paid ≤ suffix.length ∧ ∀ owner,
    partialPaid owner ≤ headCost suffix paid owner ∧
    demand (suffix.drop paid) owner ≤ credits owner + partialPaid owner

theorem zero_partial : CoveredPartial suffix paid (fun _ => 0) credits ↔
    CoveredAt suffix paid credits := by
  simp [CoveredPartial,CoveredAt,Covered]

-- Preserve coverage at EACH known microstep of the head's actual recipe.
-- Partial payment cannot exceed this one occurrence's projected cost.
theorem matching_microstep {p a : Nat} {suffix : List (PublicationInput.Command p a)}
    {paid : Nat} {partialPaid credits debit : Fin p → Nat}
    (covered : CoveredPartial suffix paid partialPaid credits)
    (funded : ∀ owner, debit owner ≤ credits owner)
    (within : ∀ owner, partialPaid owner+debit owner ≤ headCost suffix paid owner) :
    CoveredPartial suffix paid (fun owner => partialPaid owner+debit owner)
      (fun owner => credits owner-debit owner) := by
  refine ⟨covered.1,?_⟩
  intro owner
  refine ⟨within owner,?_⟩
  have old := (covered.2 owner).2
  have enough := funded owner
  change demand (suffix.drop paid) owner ≤ credits owner-debit owner+(partialPaid owner+debit owner)
  omega

-- Once exactly the head recipe has been paid, move it into the completed
-- prefix without spending again. Actual terminal origin is not inferred here.
theorem completed_partial_head (covered : CoveredPartial suffix paid partialPaid credits)
    (next : suffix.drop paid = command::rest)
    (complete : ∀ owner, partialPaid owner = commandCost command owner) :
    CoveredAt suffix (paid+1) credits := by
  have lengthAt := congrArg List.length next
  have nextDrop : suffix.drop (paid+1) = rest := by
    rw [← List.drop_drop]
    simp [next]
  refine ⟨?_,?_⟩
  · simp only [List.length_drop,List.length_cons] at lengthAt
    omega
  · intro owner
    have old := (covered.2 owner).2
    rw [next,demand_cons,complete owner] at old
    change demand (suffix.drop (paid+1)) owner ≤ credits owner
    rw [nextDrop]
    omega

theorem notified_partial_head
    (covered : CoveredPartial (command::rest) (paid+1) partialPaid credits) :
    CoveredPartial rest paid partialPaid credits := by
  constructor
  · have bound := covered.1
    simp only [List.length_cons] at bound
    omega
  · simpa [headCost] using covered.2

-- Independent unrelated spending is admissible only if it leaves real credit
-- and matched partial payment sufficient. It never increases paid evidence.
def slackCheck (suffix : List (PublicationInput.Command p a)) (paid : Nat)
    (partialPaid credits debit : Fin p → Nat) : Bool :=
  (List.finRange p).all (fun owner => decide (debit owner ≤ credits owner ∧
    debit owner+demand (suffix.drop paid) owner ≤ credits owner+partialPaid owner))

theorem slack_exact : slackCheck suffix paid partialPaid credits debit = true ↔
    ∀ owner, debit owner ≤ credits owner ∧
      debit owner+demand (suffix.drop paid) owner ≤ credits owner+partialPaid owner := by
  simp [slackCheck,List.all_eq_true]

theorem unrelated_spend (covered : CoveredPartial suffix paid partialPaid credits)
    (allowed : slackCheck suffix paid partialPaid credits debit = true) :
    CoveredPartial suffix paid partialPaid (fun owner => credits owner-debit owner) := by
  refine ⟨covered.1,?_⟩
  intro owner
  obtain ⟨funded,slack⟩ := slack_exact.mp allowed owner
  refine ⟨(covered.2 owner).1,?_⟩
  change demand (suffix.drop paid) owner ≤ credits owner-debit owner+partialPaid owner
  omega

#print axioms zero_partial
#print axioms matching_microstep
#print axioms completed_partial_head
#print axioms notified_partial_head
#print axioms slack_exact
#print axioms unrelated_spend


-- The enforced initialization prelude has per-owner outstanding debt. Source
-- presence alone cannot clear it. Its physical consumer may discharge one
-- entry only after the matching first native initialization has really paid.
def InitCovered (suffix : List (PublicationInput.Command p a))
    (pending : Fin p → Bool) (credits : Fin p → Nat) : Prop :=
  ∀ owner, (if pending owner then 1 else 0)+demand suffix owner ≤ credits owner

def initCheck (suffix : List (PublicationInput.Command p a))
    (pending : Fin p → Bool) (credits : Fin p → Nat) : Bool :=
  (List.finRange p).all (fun owner => decide
    ((if pending owner then 1 else 0)+demand suffix owner ≤ credits owner))

theorem init_exact : initCheck suffix pending credits = true ↔ InitCovered suffix pending credits := by
  simp [initCheck,InitCovered,List.all_eq_true]

theorem initialization_paid {p a : Nat} {suffix : List (PublicationInput.Command p a)}
    {pending : Fin p → Bool} {credits : Fin p → Nat} {owner : Fin p}
    (covered : InitCovered suffix pending credits) (owed : pending owner = true) :
    InitCovered suffix (fun i => if i=owner then false else pending i)
      (fun i => if i=owner then credits i-1 else credits i) := by
  intro i
  by_cases same : i=owner
  · subst i
    have enough := covered owner
    simp only [owed,ite_true] at enough
    simp only [ite_true,Bool.false_eq_true,ite_false,Nat.zero_add]
    omega
  · simpa [same] using covered i

theorem initialization_done {p a : Nat} {suffix : List (PublicationInput.Command p a)}
    {pending : Fin p → Bool} {credits : Fin p → Nat}
    (covered : InitCovered suffix pending credits) (done : ∀ owner, pending owner = false) :
    CoveredAt suffix 0 credits := by
  refine ⟨by omega,?_⟩
  intro owner
  simpa [done owner] using covered owner

-- During a partial prelude, unrelated administration may consume only slack;
-- it does not discharge another owner's still outstanding first initialize.
theorem initialization_extra {p a : Nat} {suffix : List (PublicationInput.Command p a)}
    {pending : Fin p → Bool} {credits debit : Fin p → Nat}
    (allowed : ∀ owner, debit owner+(if pending owner then 1 else 0)+demand suffix owner ≤ credits owner) :
    InitCovered suffix pending (fun owner => credits owner-debit owner) := by
  intro owner
  have enough := allowed owner
  change (if pending owner then 1 else 0)+demand suffix owner ≤ credits owner-debit owner
  omega

-- Exact-slack refusal and nonvacuous head progress use independent arithmetic
-- predicates; the required operation is not reclassified as an extra debit.
theorem zero_slack_extra_refused {p a : Nat} {suffix : List (PublicationInput.Command p a)}
    {credits : Fin p → Nat} {owner : Fin p}
    (tight : credits owner = demand suffix owner) :
    ¬ ChargeAdmissible suffix 0 credits (fun i => if i=owner then 1 else 0) := by
  intro permitted
  have enough := permitted owner
  simp [tight] at enough
  omega

theorem paid_head_notified {p a : Nat} {command : PublicationInput.Command p a}
    {rest : List (PublicationInput.Command p a)} {credits : Fin p → Nat}
    (covered : Covered (command::rest) credits) :
    CoveredAt (command::rest) 1 (fun owner => credits owner-commandCost command owner) ∧
    CoveredAt rest 0 (fun owner => credits owner-commandCost command owner) := by
  have started : CoveredAt (command::rest) 0 credits := ⟨by omega,by simpa using covered⟩
  have paid := completed_head started rfl
  exact ⟨paid,notified_head paid⟩

#print axioms init_exact
#print axioms initialization_paid
#print axioms initialization_done
#print axioms initialization_extra
#print axioms zero_slack_extra_refused
#print axioms paid_head_notified

end MirroreaProofFirst.OwnerFundingCursor
