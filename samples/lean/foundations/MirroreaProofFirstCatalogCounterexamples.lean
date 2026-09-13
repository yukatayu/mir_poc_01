import MirroreaProofFirstSourceHistory

namespace MirroreaProofFirst.CatalogCounterexamples
open InstancePrograms InstanceState CompositionCore CatalogHistory

-- Decisive finite counterexample to inferring edge refinement from old Valid.
def empty : State 0 1 0 := ⟨91,fun _ => 0,Fin.elim0,Fin.elim0,Fin.elim0,fun _ => true⟩
def first := register empty InstancePrograms.Controls.original none
theorem first_catalog : CatalogValid first := register_catalog empty _ none (empty_valid empty)
  ⟨(InstancePrograms.check_exact _).mp (by decide),by simp⟩
def smaller : Definition := {InstancePrograms.Controls.original with
  contract := {InstancePrograms.Controls.contract with inputs := [0]}}
theorem smaller_satisfies : Satisfies smaller := (InstancePrograms.check_exact _).mp (by decide)
def unchecked := register first smaller (some 0)

theorem unchecked_old_valid : Valid unchecked := by
  refine ⟨?_,append_parent_acyclic _ _ first_catalog.versions,?_,?_,?_⟩
  · intro key
    rcases old_or_last key with ⟨old,rfl⟩ | rfl
    · simpa [unchecked,register,IdentityGrowth.extend_old] using first_catalog.definitions old
    · simpa [unchecked,register,extend_last] using smaller_satisfies
  all_goals intro key; exact Fin.elim0 key

theorem unchecked_not_catalog : ¬ CatalogValid unchecked := by
  intro valid
  have refinement := valid.refinement (last 1) (Growth.left 1 1 0) (by rfl)
  have oldMember : (-2 : Int) ∈ (unchecked.definitions (Growth.left 1 1 0)).contract.inputs := by decide
  have lost := refinement.inputs (-2) oldMember
  simp [unchecked,register,extend_last,smaller] at lost

#guard !registrationCheck first smaller (some 0)
#guard (CompositionCore.run (config first) (.register smaller (some 0))).isNone

-- A retained interface with an empty input set also passed the earlier weaker
-- structural predicate. No checked construction creates such an interface.
def live := instantiate first 0 7 [0] none .top
theorem live_catalog : CatalogValid live := instantiate_catalog first 0 7 [0] none .top first_catalog (by simp)
def noInputs : State 1 1 1 := {live with instances := fun key =>
  {live.instances key with interface := {(live.instances key).interface with inputs := []}}}
theorem noInputs_old_valid : Valid noInputs := by
  refine ⟨live_catalog.definitions,live_catalog.versions,?_,live_catalog.placed,live_catalog.parents⟩
  intro key
  exact ⟨by simp [noInputs],(live_catalog.interfaces key).lower,(live_catalog.interfaces key).upper⟩
theorem noInputs_not_catalog : ¬ CatalogValid noInputs := by
  intro valid
  exact valid.inhabited 0 rfl

-- This is a real scope boundary, not an unconditional recovery theorem.
-- The current profile needs a participating control actor. All-locus drain
-- makes every later control request fail, even with unrevoked logical claims.
theorem drained_rejects (s : ManagementEntry.System p a)
    (drained : ∀ place, s.configuration.state.participating place = false)
    (member : Fin a) (place : Fin p) (principal requestId : Nat) (raw : Raw) :
    ManagementEntry.perform s member place principal requestId raw = none := by
  simp [ManagementEntry.perform,ManagementEntry.authorize,ManagementEntry.actorCheck,drained]

-- Smallest alternative obligation, not adopted authority: retain realm/current
-- member/principal/join-claim checks, but require participation only for non-join.
-- That changes admission of a drained realm and needs an explicit implementation
-- and review, not a conclusion smuggled into CurrentActor.
def JoinActorCandidate (s : ManagementEntry.System p a) (member : Fin a) (principal : Nat) : Prop :=
  s.configuration.state.realm = s.view.realm ∧ (s.view.members member).enabled = true ∧
    (s.view.members member).principal = principal

def drainedInitial : ManagementEntry.System 3 1 :=
  {ManagementEntry.Controls.initial with
    configuration := config {ManagementEntry.Controls.initial.configuration.state with participating := fun _ => false}}
example : JoinActorCandidate drainedInitial 0 7 := ⟨rfl,rfl,rfl⟩
#guard (ManagementEntry.perform drainedInitial 0 0 7 0 (.join 0)).isNone
def surviving : ManagementEntry.System 3 1 :=
  {drainedInitial with
    configuration := config {drainedInitial.configuration.state with participating := fun k => k == 1}}
#guard (ManagementEntry.perform surviving 0 1 7 0 (.join 0)).isSome

#print axioms unchecked_old_valid
#print axioms unchecked_not_catalog
#print axioms noInputs_old_valid
#print axioms noInputs_not_catalog
#print axioms drained_rejects
end MirroreaProofFirst.CatalogCounterexamples
