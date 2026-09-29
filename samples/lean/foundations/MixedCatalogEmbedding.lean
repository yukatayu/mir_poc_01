import MixedInstanceState
namespace MirroreaProofFirst.MixedCatalogEmbedding
open MixedOperationDefinitions

def embedInstance (i : InstanceState.Instance d p n) : MixedInstanceState.Instance d p n :=
 ⟨i.definition,.pure i.interface,i.owner,i.revision,i.enabled,i.placements,i.parent,i.dependencies⟩
def state (s : InstanceState.State d p n) : MixedInstanceState.State d p n :=
 ⟨s.realm,s.placeIncarnation,(fun key => .pure (s.definitions key)),s.predecessors,
  (fun key => embedInstance (s.instances key)),s.participating⟩

theorem valid_exact (s : InstanceState.State d p n) :
 MixedInstanceState.Valid (state s) ↔ InstanceState.Valid s := by
 constructor
 · intro h; exact ⟨h.definitions,h.versions,h.interfaces,h.placed,h.parents⟩
 · intro h; exact ⟨h.definitions,h.versions,h.interfaces,h.placed,h.parents⟩

theorem snapshot_exact (s : InstanceState.State d p n) :
 MixedInstanceState.snapshot (state s) = InstanceState.snapshot s := rfl

theorem registration_check (s : InstanceState.State d p n) (new : InstancePrograms.Definition)
 (parent : Option (Fin d)) :
 MixedInstanceState.registrationCheck (state s) (.pure new) parent = InstanceState.registrationCheck s new parent := by
 cases parent <;> rfl

theorem register_exact (s : InstanceState.State d p n) (new : InstancePrograms.Definition)
 (parent : Option (Fin d)) :
 MixedInstanceState.register (state s) (.pure new) parent = state (InstanceState.register s new parent) := by
 cases s
 simp only [state,MixedInstanceState.register,InstanceState.register,
   MixedInstanceState.mapDefinition,InstanceState.mapDefinition,embedInstance,IdentityGrowth.extend]
 congr 1
 funext key
 by_cases bound : key.val < d <;> simp [IdentityGrowth.extend,bound]

theorem instantiate_exact (s : InstanceState.State d p n) (definition : Fin d) (owner : Nat)
 (placements : List (Fin p)) (parent : Option (Fin n)) (dependencies : Support.Formula (Fin n)) :
 MixedInstanceState.instantiate (state s) definition owner placements parent dependencies =
 state (InstanceState.instantiate s definition owner placements parent dependencies) := by
 cases s
 simp only [state,MixedInstanceState.instantiate,InstanceState.instantiate,
   MixedInstanceState.mapInstance,InstanceState.mapInstance,embedInstance,IdentityGrowth.extend]
 congr 1
 funext key
 by_cases bound : key.val < n <;> simp [IdentityGrowth.extend,bound,Definition.contract]

theorem retire_exact (s : InstanceState.State d p n) (key : Fin n) :
 MixedInstanceState.retire (state s) key = state (InstanceState.retire s key) := by
 cases s
 simp only [state,MixedInstanceState.retire,InstanceState.retire,
   MixedInstanceState.modify,InstanceState.modify,embedInstance]
 congr 1
 funext k
 split <;> rfl

theorem replace_exact (s : InstanceState.State d p n) (key : Fin n) (definition : Fin d) :
 MixedInstanceState.replace (state s) key definition = state (InstanceState.replace s key definition) := by
 cases s
 simp only [state,MixedInstanceState.replace,InstanceState.replace,
   MixedInstanceState.modify,InstanceState.modify,embedInstance]
 congr 1
 funext k
 split <;> rfl

theorem reparent_exact (s : InstanceState.State d p n) (key : Fin n) (parent : Option (Fin n)) :
 MixedInstanceState.setParent (state s) key parent = state (InstanceState.setParent s key parent) := by
 cases s
 simp only [state,MixedInstanceState.setParent,InstanceState.setParent,
   MixedInstanceState.modify,InstanceState.modify,embedInstance]
 congr 1
 funext k
 split <;> rfl

theorem leave_exact (s : InstanceState.State d p n) (place : Fin p) :
 MixedInstanceState.leave (state s) place = state (InstanceState.leave s place) := rfl
theorem join_exact (s : InstanceState.State d p n) (place : Fin p) :
 MixedInstanceState.join (state s) place = state (InstanceState.join s place) := rfl

theorem replacement_check (s : InstanceState.State d p n) (key : Fin n) (definition : Fin d) :
 MixedInstanceState.replacementCheck (state s) key definition = InstanceState.replacementCheck s key definition := rfl

theorem reparent_check (s : InstanceState.State d p n) (key : Fin n) (parent : Option (Fin n)) :
 MixedInstanceState.reparentCheck (state s) key parent = InstanceState.reparentCheck s key parent := by
 cases parent <;> rfl

#print axioms valid_exact
#print axioms register_exact
#print axioms instantiate_exact
#print axioms retire_exact
#print axioms replace_exact
#print axioms reparent_exact
#print axioms leave_exact
#print axioms join_exact
end MirroreaProofFirst.MixedCatalogEmbedding
