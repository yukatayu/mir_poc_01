import MixedManagementEntry
import MixedCatalogEmbedding
namespace MirroreaProofFirst.MixedCoreEmbedding

-- General pure-branch correspondence at the actual raw structural entry. This
-- includes refusals; it is not a theorem restricted to admitted commands.
def command : CompositionCore.Command d p n → MixedCompositionCore.Command d p n
 | .register defn prev => .register (.pure defn) prev
 | .instantiate defn owner places parent deps => .instantiate defn owner places parent deps
 | .retire key => .retire key
 | .reparent key parent => .reparent key parent
 | .replace key defn => .replace key defn
 | .leave place => .leave place
 | .join place => .join place

def raw : CompositionCore.Raw → MixedCompositionCore.Raw
 | .register defn prev => .register (.pure defn) prev
 | .instantiate defn owner places parent deps => .instantiate defn owner places parent deps
 | .retire key => .retire key
 | .reparent key parent => .reparent key parent
 | .replace key defn => .replace key defn
 | .leave place => .leave place
 | .join place => .join place

def config (s : CompositionCore.Config p) : MixedCompositionCore.Config p :=
 ⟨s.definitions,s.count,MixedCatalogEmbedding.state s.state⟩
def result (r : CompositionCore.Config p × Option Nat) := (config r.1,r.2)

theorem check_exact (s : InstanceState.State d p n) (c : CompositionCore.Command d p n) :
 MixedCompositionCore.check (MixedCatalogEmbedding.state s) (command c) = CompositionCore.check s c := by
 cases c <;> simp only [command,MixedCompositionCore.check,CompositionCore.check,
  MixedCatalogEmbedding.registration_check,MixedCatalogEmbedding.replacement_check,
  MixedCatalogEmbedding.reparent_check] <;> rfl

theorem outcome_exact (s : InstanceState.State d p n) (c : CompositionCore.Command d p n) :
 MixedCompositionCore.outcome (MixedCatalogEmbedding.state s) (command c) = result (CompositionCore.outcome s c) := by
 cases c <;> simp only [command,MixedCompositionCore.outcome,CompositionCore.outcome,
  result,config,CompositionCore.config,MixedCompositionCore.config,
  MixedCatalogEmbedding.register_exact,MixedCatalogEmbedding.instantiate_exact,
  MixedCatalogEmbedding.retire_exact,MixedCatalogEmbedding.replace_exact,
  MixedCatalogEmbedding.reparent_exact,MixedCatalogEmbedding.leave_exact,MixedCatalogEmbedding.join_exact]

theorem apply_exact (s : InstanceState.State d p n) (c : CompositionCore.Command d p n) :
 MixedCompositionCore.apply (MixedCatalogEmbedding.state s) (command c) = (CompositionCore.apply s c).map result := by
 simp only [MixedCompositionCore.apply,CompositionCore.apply,check_exact,outcome_exact]
 split <;> rfl

theorem index_exact (n key : Nat) : MixedCompositionCore.index n key = CompositionCore.index n key := rfl
theorem optional_exact (n : Nat) (key : Option Nat) :
 MixedCompositionCore.optionalIndex n key = CompositionCore.optionalIndex n key := by
 cases key <;> rfl

theorem formula_exact (n : Nat) (f : Support.Formula Nat) :
 MixedCompositionCore.formula n f = CompositionCore.formula n f := by
 induction f <;> simp_all [MixedCompositionCore.formula,CompositionCore.formula,index_exact]

theorem elaborate_exact (d p n : Nat) (r : CompositionCore.Raw) :
 MixedCompositionCore.elaborate d p n (raw r) = (CompositionCore.elaborate d p n r).map command := by
 cases r with
 | register defn prev =>
   cases h : CompositionCore.optionalIndex d prev <;> simp [raw,MixedCompositionCore.elaborate,CompositionCore.elaborate,optional_exact,h,command]
 | instantiate defn owner places parent deps =>
   have indices : MixedCompositionCore.index p = CompositionCore.index p := rfl
   cases h1 : CompositionCore.index d defn <;>
   cases h2 : places.mapM (CompositionCore.index p) <;>
   cases h3 : CompositionCore.optionalIndex n parent <;>
   cases h4 : CompositionCore.formula n deps <;>
   simp [raw,MixedCompositionCore.elaborate,CompositionCore.elaborate,index_exact,indices,optional_exact,formula_exact,h1,h2,h3,h4,command]
 | retire key =>
   cases h : CompositionCore.index n key <;> simp [raw,MixedCompositionCore.elaborate,CompositionCore.elaborate,index_exact,h,command]
 | reparent key parent =>
   cases h1 : CompositionCore.index n key <;> cases h2 : CompositionCore.optionalIndex n parent <;>
   simp [raw,MixedCompositionCore.elaborate,CompositionCore.elaborate,index_exact,optional_exact,h1,h2,command]
 | replace key defn =>
   cases h1 : CompositionCore.index n key <;> cases h2 : CompositionCore.index d defn <;>
   simp [raw,MixedCompositionCore.elaborate,CompositionCore.elaborate,index_exact,h1,h2,command]
 | leave place =>
   cases h : CompositionCore.index p place <;> simp [raw,MixedCompositionCore.elaborate,CompositionCore.elaborate,index_exact,h,command]
 | join place =>
   cases h : CompositionCore.index p place <;> simp [raw,MixedCompositionCore.elaborate,CompositionCore.elaborate,index_exact,h,command]

theorem run_exact (s : CompositionCore.Config p) (r : CompositionCore.Raw) :
 MixedCompositionCore.run (config s) (raw r) = (CompositionCore.run s r).map result := by
 simp only [MixedCompositionCore.run,CompositionCore.run,config,elaborate_exact]
 cases hc : CompositionCore.elaborate s.definitions p s.count r with
 | none => rfl
 | some c => simpa only [Option.map_some,Option.bind_eq_bind,Option.bind_some] using apply_exact s.state c

#print axioms check_exact
#print axioms outcome_exact
#print axioms apply_exact
#print axioms elaborate_exact
#print axioms run_exact
end MirroreaProofFirst.MixedCoreEmbedding
