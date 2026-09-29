import MixedCancelEmbedding
import MirroreaProofFirstCatalogHistory

namespace MirroreaProofFirst.MixedCatalogHistory
open MixedOperationDefinitions MixedInstanceState MixedCompositionCore

-- Pure invocation retains its inhabited input contract. Owner operations make
-- no totality or inhabited-input claim; dynamic abort remains observable.
def Inhabited : Contract → Prop
 | .pure c => c.inputs ≠ []
 | .owner _ => True

theorem satisfies_inhabited (d : Definition) (h : Satisfies d) : Inhabited d.contract := by
 cases d with
 | pure d => exact h.inhabited
 | owner d => trivial

-- Forward strengthening: earlier Valid did not assert these two properties.
-- They are now proved from actual checked construction and every transition.
structure CatalogValid (s : State d p n) : Prop extends Valid s where
  refinement : ∀ newer older, s.predecessors newer = some older →
    Refines (s.definitions older).contract (s.definitions newer).contract
  inhabited : ∀ k, Inhabited (s.instances k).interface

theorem empty_valid (s : State 0 p 0) : CatalogValid s := by
  refine ⟨⟨?_,?_,?_,?_,?_⟩,?_,?_⟩
  all_goals intro k; exact Fin.elim0 k

theorem register_catalog (s : State d p n) (new : Definition) (previous : Option (Fin d))
    (valid : CatalogValid s) (accepted : Registration s new previous) :
    CatalogValid (register s new previous) := by
  refine ⟨register_valid s new previous valid.toValid ((registration_exact _ _ _).mpr accepted),?_,?_⟩
  · intro newer older edge
    rcases old_or_last newer with ⟨old,rfl⟩ | rfl
    · simp only [register,IdentityGrowth.extend_old] at edge
      obtain ⟨parent,hp,rfl⟩ := mapped_parent _ _ edge
      simpa only [register,IdentityGrowth.extend_old] using valid.refinement old parent hp
    · simp only [register,extend_last] at edge
      obtain ⟨parent,hp,rfl⟩ := mapped_parent _ _ edge
      simpa only [register,IdentityGrowth.extend_old,extend_last] using accepted.2 parent hp
  · exact valid.inhabited

theorem instantiate_catalog (s : State d p n) (definition : Fin d) (owner : Nat)
    (placements : List (Fin p)) (parent : Option (Fin n)) (support : Support.Formula (Fin n))
    (valid : CatalogValid s) (placed : placements ≠ []) :
    CatalogValid (instantiate s definition owner placements parent support) := by
  refine ⟨instantiate_valid _ _ _ _ _ _ valid.toValid placed,valid.refinement,?_⟩
  intro k
  rcases old_or_last k with ⟨old,rfl⟩ | rfl
  · simpa only [instantiate_old,mapInstance] using valid.inhabited old
  · simpa only [instantiate_new] using satisfies_inhabited _ (valid.definitions definition)

theorem retire_catalog (s : State d p n) (key : Fin n) (valid : CatalogValid s) :
    CatalogValid (retire s key) := by
  refine ⟨retire_valid _ _ valid.toValid,valid.refinement,?_⟩
  intro k
  by_cases h : k = key <;> simpa [retire,MixedInstanceState.modify,h] using valid.inhabited k

theorem reparent_catalog (s : State d p n) (key : Fin n) (parent : Option (Fin n))
    (valid : CatalogValid s) (allowed : GraphValidation.Acyclic (ParentEdge (setParent s key parent))) :
    CatalogValid (setParent s key parent) := by
  refine ⟨reparent_valid _ _ _ valid.toValid ((reparent_exact _ _ _ valid.parents).mpr allowed),valid.refinement,?_⟩
  intro k
  by_cases h : k = key <;> simpa [setParent,MixedInstanceState.modify,h] using valid.inhabited k

theorem replace_catalog (s : State d p n) (key : Fin n) (definition : Fin d)
    (valid : CatalogValid s) (allowed : Refines (s.definitions (s.instances key).definition).contract
      (s.definitions definition).contract ∧ Satisfies (s.definitions definition)) :
    CatalogValid (replace s key definition) := by
  refine ⟨replacement_valid _ _ _ valid.toValid ((exchange_exact _ _).mpr allowed),valid.refinement,?_⟩
  intro k
  by_cases h : k = key <;> simpa [replace,MixedInstanceState.modify,h] using valid.inhabited k

theorem leave_catalog (s : State d p n) (place : Fin p) (valid : CatalogValid s) :
    CatalogValid (leave s place) := ⟨leave_valid _ _ valid.toValid,valid.refinement,valid.inhabited⟩
theorem join_catalog (s : State d p n) (place : Fin p) (valid : CatalogValid s) :
    CatalogValid (join s place) := ⟨join_valid _ _ valid.toValid,valid.refinement,valid.inhabited⟩

def Invariant (cfg : Config p) : Prop := CatalogValid cfg.state

theorem outcome_catalog (s : State d p n) (cmd : Command d p n)
    (valid : CatalogValid s) (allowed : Allowed s cmd) : Invariant (outcome s cmd).1 := by
  cases cmd with
  | register => exact register_catalog _ _ _ valid allowed
  | instantiate => exact instantiate_catalog _ _ _ _ _ _ valid allowed
  | retire => exact retire_catalog _ _ valid
  | reparent => exact reparent_catalog _ _ _ valid allowed.2
  | replace => exact replace_catalog _ _ _ valid allowed.2
  | leave => exact leave_catalog _ _ valid
  | join => exact join_catalog _ _ valid

theorem run_parts (cfg : Config p) (raw : Raw) (next : Config p × Option Nat)
    (valid : MixedCompositionCore.Invariant cfg) (accepted : MixedCompositionCore.run cfg raw = some next) :
    ∃ cmd : Command cfg.definitions p cfg.count, erase cmd = raw ∧ Allowed cfg.state cmd ∧ next = outcome cfg.state cmd := by
  unfold MixedCompositionCore.run at accepted
  cases hc : elaborate cfg.definitions p cfg.count raw with
  | none => simp [hc] at accepted
  | some cmd =>
      have applied : MixedCompositionCore.apply cfg.state cmd = some next := by simpa [hc] using accepted
      obtain ⟨allowed,eq⟩ := (apply_exact _ _ _ valid).mp applied
      exact ⟨cmd,elaborate_sound _ hc,allowed,eq⟩

theorem run_catalog (cfg : Config p) (raw : Raw) (next : Config p × Option Nat)
    (valid : Invariant cfg) (accepted : MixedCompositionCore.run cfg raw = some next) : Invariant next.1 := by
  obtain ⟨cmd,_,allowed,rfl⟩ := run_parts _ _ _ valid.toValid accepted
  exact outcome_catalog _ _ valid allowed

def definitionAt (cfg : Config p) (key : Nat) : Option Definition :=
  (index cfg.definitions key).map cfg.state.definitions

-- Inclusion preserves the actual old definition, not merely slot counts or a
-- digest chosen by the requester. This is local-history identity, not fork identity.
def Extends (before after : Config p) : Prop :=
  before.definitions ≤ after.definitions ∧ before.count ≤ after.count ∧
    ∀ key : Fin before.definitions, definitionAt after key.val = some (before.state.definitions key)

theorem definitionAt_key (cfg : Config p) (key : Fin cfg.definitions) :
    definitionAt cfg key.val = some (cfg.state.definitions key) := by simp [definitionAt,index_roundtrip]

theorem extends_refl (cfg : Config p) : Extends cfg cfg :=
  ⟨Nat.le_refl _,Nat.le_refl _,definitionAt_key cfg⟩

theorem extends_trans (a b c : Config p) (ab : Extends a b) (bc : Extends b c) : Extends a c := by
  refine ⟨Nat.le_trans ab.1 bc.1,Nat.le_trans ab.2.1 bc.2.1,?_⟩
  intro key
  let middle : Fin b.definitions := ⟨key.val,Nat.lt_of_lt_of_le key.isLt ab.1⟩
  have stable : b.state.definitions middle = a.state.definitions key := by
    have eq := ab.2.2 key
    have atMiddle := definitionAt_key b middle
    rw [atMiddle] at eq
    exact Option.some.inj eq
  rw [← stable]
  exact bc.2.2 middle

theorem outcome_extends (s : State d p n) (cmd : Command d p n) : Extends (config s) (outcome s cmd).1 := by
  cases cmd with
  | register new previous =>
      refine ⟨by simp [outcome,config],Nat.le_refl _,?_⟩
      intro key
      have h := definitionAt_key (config (register s new previous)) (Growth.left d 1 key)
      change definitionAt (config (register s new previous)) (Growth.left d 1 key).val =
        some ((register s new previous).definitions (Growth.left d 1 key)) at h
      rw [register_old_definition] at h
      exact h
  | instantiate definition owner places parent support =>
      exact ⟨Nat.le_refl _,by simp [outcome,config],definitionAt_key (config s)⟩
  | retire => exact extends_refl (config s)
  | reparent => exact extends_refl (config s)
  | replace => exact extends_refl (config s)
  | leave => exact extends_refl (config s)
  | join => exact extends_refl (config s)

theorem run_extends (cfg : Config p) (raw : Raw) (next : Config p × Option Nat)
    (valid : MixedCompositionCore.Invariant cfg) (accepted : MixedCompositionCore.run cfg raw = some next) : Extends cfg next.1 := by
  obtain ⟨cmd,_,_,rfl⟩ := run_parts _ _ _ valid accepted
  exact outcome_extends cfg.state cmd

theorem commit_catalog (s : MixedManagementEntry.System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : MixedManagementEntry.Evidence) (next : MixedManagementEntry.System p a × Option Nat)
    (valid : Invariant s.configuration) (accepted : MixedManagementEntry.commit s member place principal id raw e = some next) :
    Invariant next.1.configuration ∧ Extends s.configuration next.1.configuration := by
  obtain ⟨_,cfg,created,hr,rfl⟩ := MixedManagementEntry.commit_parts _ _ _ _ _ _ _ _ accepted
  exact ⟨run_catalog _ _ _ valid hr,run_extends _ _ _ valid.toValid hr⟩

theorem perform_exact (s : MixedManagementEntry.System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (next : MixedManagementEntry.System p a × Option Nat) (valid : MixedManagementEntry.Invariant s) :
    MixedManagementEntry.perform s member place principal id raw = some next ↔
      MixedManagementEntry.Allowed s member place principal id raw ∧ MixedManagementEntry.useId s principal id ∉ s.used ∧
      ∃ cmd : Command s.configuration.definitions p s.configuration.count,
        erase cmd = raw ∧ Allowed s.configuration.state cmd ∧
        next = (MixedManagementEntry.after s (outcome s.configuration.state cmd).1 principal id,(outcome s.configuration.state cmd).2) := by
  constructor
  · intro accepted
    unfold MixedManagementEntry.perform at accepted
    cases he : MixedManagementEntry.authorize s member place principal id raw with
    | none => simp [he] at accepted
    | some e =>
        have committed : MixedManagementEntry.commit s member place principal id raw e = some next := by simpa [he] using accepted
        have fresh := MixedManagementEntry.commit_fresh _ _ _ _ _ _ _ _ committed
        obtain ⟨checked,cfg,created,hr,eq⟩ := MixedManagementEntry.commit_parts _ _ _ _ _ _ _ _ committed
        have allowed := (MixedManagementEntry.check_sound _ _ _ _ _ _ _ checked).1
        obtain ⟨cmd,eraseEq,structural,ceq⟩ := run_parts _ _ _ valid.1 hr
        exact ⟨allowed,fresh,cmd,eraseEq,structural,by simpa [← ceq] using eq⟩
  · rintro ⟨auth,fresh,cmd,rfl,allowed,rfl⟩
    obtain ⟨e,he⟩ := MixedManagementEntry.authorize_complete s member place principal id (erase cmd) auth
    have checked := MixedManagementEntry.authorize_checked _ _ _ _ _ _ _ he
    have applied := (apply_exact s.configuration.state cmd (outcome s.configuration.state cmd) valid.1).mpr ⟨allowed,rfl⟩
    have ran : MixedCompositionCore.run s.configuration (erase cmd) = some (outcome s.configuration.state cmd) := by
      simp [MixedCompositionCore.run,elaborate_roundtrip,applied]
    simp [MixedManagementEntry.perform,he,MixedManagementEntry.commit,checked,fresh,ran]


theorem pure_valid_exact (s : InstanceState.State d p n) :
 CatalogValid (MixedCatalogEmbedding.state s) ↔ CatalogHistory.CatalogValid s := by
 constructor
 · intro h
   exact ⟨(MixedCatalogEmbedding.valid_exact s).mp h.toValid,h.refinement,h.inhabited⟩
 · intro h
   exact ⟨(MixedCatalogEmbedding.valid_exact s).mpr h.toValid,h.refinement,h.inhabited⟩

#print axioms outcome_catalog
#print axioms outcome_extends
#print axioms run_catalog
#print axioms commit_catalog
#print axioms perform_exact
#print axioms pure_valid_exact
end MirroreaProofFirst.MixedCatalogHistory
