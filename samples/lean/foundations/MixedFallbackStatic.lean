import MixedCatalogHistory
import MirroreaProofFirstFallbackStatic
namespace MirroreaProofFirst.MixedFallbackStatic
open MixedInstanceState CompositionCore FallbackStatic
-- Existing pure reference-chain syntax and shape judgment are retained.
-- Owner-tagged interfaces never stand in for a pure callable contract.
def OptionResolved (s : State d p n) (option : OptionDecl) : Prop :=
  ∃ key : Fin n, option.target = key.val ∧ MixedOperationDefinitions.Contract.pure option.contract = (s.instances key).interface
def optionCheck (s : State d p n) (option : OptionDecl) : Bool :=
  match index n option.target with
  | none => false
  | some key => decide (MixedOperationDefinitions.Contract.pure option.contract = (s.instances key).interface)

theorem option_exact (s : State d p n) (option : OptionDecl) : optionCheck s option = true ↔ OptionResolved s option := by
  constructor
  · intro accepted
    unfold optionCheck at accepted
    cases h : index n option.target with
    | none => simp [h] at accepted
    | some key => exact ⟨key,(index_sound _ h).symm,by simpa [h] using accepted⟩
  · rintro ⟨key,target,contract⟩
    simp [optionCheck,target,index_roundtrip,contract]

-- The terminal target is the reader itself or a genuine ancestor in the SAME
-- instance parent graph. Arbitrary unrelated long-lived constants do not count.
def Dominates (s : State d p n) (reader target : Nat) : Prop :=
  ∃ r t : Fin n, reader = r.val ∧ target = t.val ∧ GraphValidation.Path (ParentEdge s) r t
def dominatesCheck (s : State d p n) (reader target : Nat) : Bool :=
  match index n reader,index n target with
  | some r,some t => GraphValidation.reachable (fun a b => decide ((s.instances a).parent = some b)) r t
  | _,_ => false

theorem dominates_exact (s : State d p n) (reader target : Nat) :
    dominatesCheck s reader target = true ↔ Dominates s reader target := by
  constructor
  · intro checked
    unfold dominatesCheck at checked
    cases hr : index n reader <;> cases ht : index n target <;> simp [hr,ht] at checked
    rename_i r t
    refine ⟨r,t,(index_sound _ hr).symm,(index_sound _ ht).symm,?_⟩
    have path := (GraphValidation.reachable_exact _ _ _).mp checked
    simpa only [decide_eq_true_eq,ParentEdge] using path
  · rintro ⟨r,t,er,et,path⟩
    simp only [dominatesCheck,er,et,index_roundtrip]
    apply (GraphValidation.reachable_exact _ _ _).mpr
    simpa only [decide_eq_true_eq,ParentEdge] using path

def Lifetime (s : State d p n) (chain : Chain) : Prop :=
  ∃ last, chain.options.getLast? = some last ∧ Dominates s chain.reader last.target
def lifetimeCheck (s : State d p n) (chain : Chain) : Bool :=
  match chain.options.getLast? with | none => false | some last => dominatesCheck s chain.reader last.target
theorem lifetime_exact (s : State d p n) (chain : Chain) : lifetimeCheck s chain = true ↔ Lifetime s chain := by
  cases h : chain.options.getLast? <;> simp [lifetimeCheck,Lifetime,h,dominates_exact]

def Admissible (s : State d p n) (chain : Chain) : Prop :=
  Shape chain ∧ (∀ option ∈ chain.options, OptionResolved s option) ∧ Lifetime s chain
def check (s : State d p n) (chain : Chain) : Except Error Unit :=
  match checkShape chain with
  | .error reason => .error reason
  | .ok _ =>
      if !(chain.options.all (optionCheck s)) then .error .unresolved
      else if !lifetimeCheck s chain then .error .lifetime else .ok ()

theorem check_exact (s : State d p n) (chain : Chain) : check s chain = .ok () ↔ Admissible s chain := by
  unfold check
  cases hs : checkShape chain with
  | error reason =>
      have bad : ¬ Shape chain := by intro h; have := (checkShape_exact chain).mpr h; simp [hs] at this
      simp [Admissible,bad]
  | ok unit =>
      cases unit
      have resolved : chain.options.all (optionCheck s) = true ↔
          ∀ option ∈ chain.options, OptionResolved s option := by simp only [List.all_eq_true,option_exact]
      simp only [Admissible,(checkShape_exact chain).mp hs,true_and,← resolved,← lifetime_exact]
      cases chain.options.all (optionCheck s) <;> cases lifetimeCheck s chain <;> simp


theorem pure_option (s : InstanceState.State d p n) (option : OptionDecl) :
 optionCheck (MixedCatalogEmbedding.state s) option = FallbackStatic.optionCheck s option := by
 unfold optionCheck FallbackStatic.optionCheck
 cases CompositionCore.index n option.target <;> simp [MixedCatalogEmbedding.state,MixedCatalogEmbedding.embedInstance]

theorem pure_dominates (s : InstanceState.State d p n) (reader target : Nat) :
 dominatesCheck (MixedCatalogEmbedding.state s) reader target = FallbackStatic.dominatesCheck s reader target := rfl

theorem pure_lifetime (s : InstanceState.State d p n) (chain : Chain) :
 lifetimeCheck (MixedCatalogEmbedding.state s) chain = FallbackStatic.lifetimeCheck s chain := rfl

theorem pure_check (s : InstanceState.State d p n) (chain : Chain) :
 check (MixedCatalogEmbedding.state s) chain = FallbackStatic.check s chain := by
 have eq : optionCheck (MixedCatalogEmbedding.state s) = FallbackStatic.optionCheck s := funext (pure_option s)
 simp only [check,FallbackStatic.check,eq,pure_lifetime]
 rfl

theorem pure_admissible (s : InstanceState.State d p n) (chain : Chain) :
 Admissible (MixedCatalogEmbedding.state s) chain ↔ FallbackStatic.Admissible s chain := by
 rw [← check_exact,← FallbackStatic.check_exact,pure_check]

#print axioms check_exact
#print axioms option_exact
#print axioms pure_check
#print axioms pure_admissible
end MirroreaProofFirst.MixedFallbackStatic
