import MirroreaProofFirstCoordinatorUse

namespace MirroreaProofFirst.AuthorityLocalFrame

-- Field-level model of the NEW M9 local preparation seam. Authority facts are
-- separate from the three M9-owned validation maps and the retained runtime.
-- No observation snapshot from the parent/decoded frame is an input here.
-- Actual M9 stage origin, endpoint restriction and control-FD provenance remain
-- explicit physical obligations; this model does not turn data into authority.
structure Observations (Owner Consumer Release : Type) where
  owner : Owner
  consumer : Consumer
  release : Release
  deriving DecidableEq, Repr

structure Local (Fact Owner Consumer Release Runtime : Type) where
  facts : Fact
  floor : Fact
  observations : Observations Owner Consumer Release
  retained : Runtime
  deriving DecidableEq, Repr

structure Delta (Fact : Type) where
  prior : Fact
  next : Fact
  deriving DecidableEq, Repr

-- Project the genuine parent evaluator's full predecessor/successor, after
-- evaluation, using the same already fixed endpoint restriction.
def stage (evaluate : Parent → Command → Option Parent) (project : Parent → Fact)
    (prior : Parent) (command : Command) : Option (Delta Fact) :=
  (evaluate prior command).map fun next => ⟨project prior,project next⟩

def Ready (s : Local Fact Owner Consumer Release Runtime) (delta : Delta Fact) : Prop :=
  s.facts = delta.prior ∧ s.floor = s.facts

def check [DecidableEq Fact] (s : Local Fact Owner Consumer Release Runtime)
    (delta : Delta Fact) : Bool := decide (s.facts = delta.prior ∧ s.floor = s.facts)

def apply (s : Local Fact Owner Consumer Release Runtime) (delta : Delta Fact) :
    Local Fact Owner Consumer Release Runtime :=
  {s with facts := delta.next, floor := delta.next}

def prepare [DecidableEq Fact] (s : Local Fact Owner Consumer Release Runtime)
    (delta : Delta Fact) : Option (Local Fact Owner Consumer Release Runtime) :=
  if check s delta then some (apply s delta) else none

variable {Fact Owner Consumer Release Runtime Parent Command : Type}
  {s next : Local Fact Owner Consumer Release Runtime} {delta : Delta Fact}
  {evaluate : Parent → Command → Option Parent} {project : Parent → Fact}
  {prior after : Parent} {command : Command}

theorem stage_exact : stage evaluate project prior command = some delta ↔
    ∃ after, evaluate prior command = some after ∧ delta = ⟨project prior,project after⟩ := by
  cases run : evaluate prior command with
  | none => simp [stage,run]
  | some value => simp [stage,run,eq_comm]

theorem check_exact [DecidableEq Fact] : check s delta = true ↔ Ready s delta := by
  simp [check,Ready]

theorem prepare_exact [DecidableEq Fact] : prepare s delta = some next ↔
    Ready s delta ∧ next = apply s delta := by
  unfold prepare
  split
  · rename_i ready
    simp only [Option.some.injEq]
    exact ⟨fun same => ⟨check_exact.mp ready,same.symm⟩,fun h => h.2.symm⟩
  · rename_i denied
    simp only [reduceCtorEq,false_iff,not_and]
    exact fun ready => False.elim (denied (check_exact.mpr ready))

-- Derived result, not an admission premise. Every observation map and every
-- retained runtime component remains exactly the pre-call child value.
theorem prepare_frame [DecidableEq Fact] (run : prepare s delta = some next) :
    next.facts = delta.next ∧ next.floor = delta.next ∧
    next.observations.owner = s.observations.owner ∧
    next.observations.consumer = s.observations.consumer ∧
    next.observations.release = s.observations.release ∧ next.retained = s.retained := by
  have same := (prepare_exact.mp run).2
  subst next
  exact ⟨rfl,rfl,rfl,rfl,rfl,rfl⟩

theorem prepare_rejects_wrong_prior [DecidableEq Fact] (wrong : s.facts ≠ delta.prior) :
    prepare s delta = none := by simp [prepare,check,wrong]

theorem prepare_rejects_stale_floor [DecidableEq Fact] (stale : s.floor ≠ s.facts) :
    prepare s delta = none := by simp [prepare,check,stale]

theorem prepare_available [DecidableEq Fact] (ready : Ready s delta) :
    ∃ next, prepare s delta = some next :=
  ⟨apply s delta,prepare_exact.mpr ⟨ready,rfl⟩⟩

-- Authority agreement alone is required of the parent stage; child observations
-- are arbitrary and can differ between endpoints. They are never merged.
theorem projected_preparation [DecidableEq Fact]
    (staged : stage evaluate project prior command = some delta)
    (run : prepare s delta = some next) :
    ∃ after, evaluate prior command = some after ∧
      s.facts = project prior ∧ next.facts = project after ∧
      next.floor = project after ∧ next.observations = s.observations ∧
      next.retained = s.retained := by
  obtain ⟨after,evaluated,definition⟩ := stage_exact.mp staged
  subst delta
  obtain ⟨ready,same⟩ := prepare_exact.mp run
  subst next
  exact ⟨after,evaluated,ready.1,rfl,rfl,rfl,rfl⟩

-- An authentic parent transition may leave the endpoint's selected facts equal
-- (or change only its generation); it does not require importing a foreign row.
theorem unaffected_endpoint [DecidableEq Fact]
    (staged : stage evaluate project prior command = some delta)
    (evaluated : evaluate prior command = some after)
    (unaffected : project after = project prior) (run : prepare s delta = some next) :
    next = s := by
  obtain ⟨actual,found,definition⟩ := stage_exact.mp staged
  have sameParent : actual = after := Option.some.inj (found.symm.trans evaluated)
  subst actual
  subst delta
  obtain ⟨ready,same⟩ := prepare_exact.mp run
  rw [same]
  cases s
  simp_all [apply,Ready]

-- Composition with the selected coordinator: all exact prepared values are
-- shown to equal its actual staged successor when it publishes. Local effects
-- are projected from that value and keep each endpoint's observations/runtime.
theorem publication_local_frame [DecidableEq Fact]
    {n revision : Nat} {initial : Parent}
    {coordinator : CoordinatorUse.State n Parent Command} {endpoint : Fin n}
    {pair : Nat × Parent}
    (path : CoordinatorUse.Reached evaluate n revision initial coordinator)
    (publish : CoordinatorUse.Allowed evaluate coordinator .publish)
    (recorded : coordinator.prepared endpoint = some pair)
    (delta_value : delta.next = project pair.2)
    (run : prepare s delta = some next) :
    next.facts = project (CoordinatorUse.apply evaluate coordinator .publish).useState.base.current ∧
    next.observations = s.observations ∧ next.retained = s.retained := by
  have published := (CoordinatorUse.prepared_value_at_publication path publish recorded).2
  have same := (prepare_exact.mp run).2
  subst next
  exact ⟨by simpa only [apply,delta_value,published],rfl,rfl⟩

#print axioms stage_exact
#print axioms prepare_exact
#print axioms prepare_frame
#print axioms projected_preparation
#print axioms unaffected_endpoint
#print axioms publication_local_frame
end MirroreaProofFirst.AuthorityLocalFrame
