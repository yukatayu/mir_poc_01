import MirroreaProofFirstPublicationLifecycle

namespace MirroreaProofFirst.PublicationOwnerBudget
open PublicationCapacityDriver (State Status)

-- Owner-side resources projected from the source's actual certified residual.
-- Freeze/install each need one idle owner transition; a future source finish
-- needs the generated reserve/probe/compute interval (three transitions).
-- These numbers are not authority, acknowledgements, actual work or a forecast
-- of a still-unknown result's future residual. The endpoint must bind current
-- credits and the matching actual owner events before consuming this plan.
def commandCost (command : PublicationInput.Command p a) (owner : Fin p) : Nat :=
  match command with
  | .freeze endpoint _ | .install endpoint _ => if owner = endpoint then 1 else 0
  | .finish endpoint => if owner = endpoint then 3 else 0
  | _ => 0

def demand (commands : List (PublicationInput.Command p a)) (owner : Fin p) : Nat :=
  (commands.map (fun command => commandCost command owner)).sum

def Covered (commands : List (PublicationInput.Command p a)) (credits : Fin p → Nat) : Prop :=
  ∀ owner, demand commands owner ≤ credits owner

def check (commands : List (PublicationInput.Command p a)) (credits : Fin p → Nat) : Bool :=
  (List.finRange p).all (fun owner => decide (demand commands owner ≤ credits owner))

theorem check_exact : check commands credits = true ↔ Covered commands credits := by
  simp [check,Covered,List.all_eq_true]

theorem demand_cons : demand (command::rest) owner = commandCost command owner + demand rest owner := by
  simp [demand]

theorem demand_append : demand (left++right) owner = demand left owner + demand right owner := by
  simp [demand]

-- Consumption is about resources only. The separate source/owner transition
-- and truthful notification proofs determine whether this head can be used.
theorem covered_tail (covered : Covered (command::rest) credits) :
    Covered rest (fun owner => credits owner-commandCost command owner) := by
  intro owner
  have enough := covered owner
  rw [demand_cons] at enough
  change demand rest owner ≤ credits owner-commandCost command owner
  omega

theorem covered_cons (head : ∀ owner, commandCost command owner ≤ credits owner)
    (tail : Covered rest (fun owner => credits owner-commandCost command owner)) :
    Covered (command::rest) credits := by
  intro owner
  have first := head owner
  have last := tail owner
  change demand rest owner ≤ credits owner-commandCost command owner at last
  rw [demand_cons]
  omega

-- The source's established semantic/profile transition is evaluated first as
-- a pure candidate. Resource refusal adopts none of its source state changes.
-- Physical code must not execute this candidate as an external effect first.
def transition (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (old : State p a)
    (credits : Fin p → Nat) (input : PublicationInput.Input p a) : State p a × Status :=
  let (next,status) := PublicationLifecycle.transitionFast assigned scopeId seed old input
  if status = .accepted then
    if check next.suffix credits then (next,status) else (old,.profileRefused)
  else (next,status)

def Admissible (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (old next : State p a)
    (credits : Fin p → Nat) (input : PublicationInput.Input p a) : Prop :=
  PublicationLifecycle.transitionFast assigned scopeId seed old input = (next,.accepted) ∧
  Covered next.suffix credits

theorem accepted_exact : transition assigned scopeId seed old credits input = (next,.accepted) ↔
    Admissible assigned scopeId seed old next credits input := by
  cases ran : PublicationLifecycle.transitionFast assigned scopeId seed old input with
  | mk actual status =>
    cases status <;> simp [transition,ran,Admissible]
    by_cases enough : check actual.suffix credits = true
    · simp [enough,eq_comm]
      intro same; subst next; exact check_exact.mp enough
    · have no : ¬ Covered actual.suffix credits := fun h => enough (check_exact.mpr h)
      simp [enough]
      intro same; subst next; exact no

theorem refused_candidate_unchanged
    (candidate : PublicationLifecycle.transitionFast assigned scopeId seed old input = (next,.accepted))
    (unfunded : ¬ Covered next.suffix credits) :
    transition assigned scopeId seed old credits input = (old,.profileRefused) := by
  have denied : ¬ check next.suffix credits = true := fun yes => unfunded (check_exact.mp yes)
  simp [transition,candidate,denied]

-- The wrapper preserves all old source/history/quiet-prefix obligations; it
-- never publishes a candidate merely because owner credits were available.
theorem preserves (valid : PublicationLifecycle.Invariant assigned scopeId seed old) :
    PublicationLifecycle.Invariant assigned scopeId seed (transition assigned scopeId seed old credits input).1 := by
  have original := PublicationLifecycle.transitionFast_preserves valid (input:=input)
  cases ran : PublicationLifecycle.transitionFast assigned scopeId seed old input with
  | mk next status =>
    rw [ran] at original
    cases status with
    | accepted =>
      by_cases enough : check next.suffix credits = true
      · simpa [transition,ran,enough] using original
      · simpa [transition,ran,enough] using valid
    | semanticRefused => simpa [transition,ran] using original
    | profileRefused => simpa [transition,ran] using original

#print axioms check_exact
#print axioms demand_cons
#print axioms demand_append
#print axioms covered_tail
#print axioms covered_cons
#print axioms accepted_exact
#print axioms refused_candidate_unchanged
#print axioms preserves
end MirroreaProofFirst.PublicationOwnerBudget
