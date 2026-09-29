import MirroreaProofFirstReferenceSourceData
import MixedFallbackStatic
import MixedCatalogEmbedding
namespace MirroreaProofFirst.MixedReferenceSourceData
open ReferenceSourceData
-- Existing names/values/source grammar/type checker are reused without executing
-- the old engine. Resolution must extract a real PURE interface; no dummy one.
def optionDecl (decl : OptionSyntax) (key : Fin n) (contract : InstancePrograms.Contract) : FallbackStatic.OptionDecl :=
 ⟨decl.name,key.val,decl.declaredAccess,decl.capability,contract,decl.leaseUntil⟩

def resolveOption (s : MixedInstanceState.State d p n) (values : Values) (decl : OptionSyntax) :
 Option FallbackStatic.OptionDecl := do
 let raw ← instanceKey values decl.target
 let key ← CompositionCore.index n raw
 let .pure contract := (s.instances key).interface | none
 return optionDecl decl key contract

def OptionElaborates (s : MixedInstanceState.State d p n) (values : Values) (decl : OptionSyntax)
 (option : FallbackStatic.OptionDecl) : Prop :=
 ∃ (key : Fin n) (contract : InstancePrograms.Contract),
  instanceKey values decl.target = some key.val ∧ (s.instances key).interface = .pure contract ∧
  option = optionDecl decl key contract

theorem option_exact (s : MixedInstanceState.State d p n) (values : Values) (decl : OptionSyntax)
 (option : FallbackStatic.OptionDecl) : resolveOption s values decl = some option ↔ OptionElaborates s values decl option := by
 constructor
 · intro resolved
   unfold resolveOption at resolved
   cases named : instanceKey values decl.target with
   | none => simp [named] at resolved
   | some raw =>
     simp only [named,Option.bind_eq_bind,Option.bind_some] at resolved
     cases indexed : CompositionCore.index n raw with
     | none => simp [indexed] at resolved
     | some key =>
       simp only [indexed,Option.bind_some] at resolved
       cases kind : (s.instances key).interface with
       | owner _ => simp [kind] at resolved
       | pure contract =>
         have equal := CompositionCore.index_sound _ indexed
         exact ⟨key,contract,by simpa [equal] using named,kind,by simpa [kind] using resolved.symm⟩
 · rintro ⟨key,contract,named,kind,rfl⟩
   simp [resolveOption,named,CompositionCore.index_roundtrip,kind]

def resolveOptions (s : MixedInstanceState.State d p n) (values : Values) : List OptionSyntax → Option (List FallbackStatic.OptionDecl)
  | [] => some []
  | item :: rest => do return (← resolveOption s values item) :: (← resolveOptions s values rest)

inductive OptionsElaborate (s : MixedInstanceState.State d p n) (values : Values) :
    List OptionSyntax → List FallbackStatic.OptionDecl → Prop where
  | nil : OptionsElaborate s values [] []
  | cons : OptionElaborates s values item option → OptionsElaborate s values rest options →
      OptionsElaborate s values (item :: rest) (option :: options)

theorem options_exact (s : MixedInstanceState.State d p n) (values : Values) (decl : List OptionSyntax)
    (options : List FallbackStatic.OptionDecl) :
    resolveOptions s values decl = some options ↔ OptionsElaborate s values decl options := by
  induction decl generalizing options with
  | nil => constructor <;> intro h <;> cases h <;> constructor
  | cons item rest ih =>
      constructor
      · intro resolved
        unfold resolveOptions at resolved
        cases hr : resolveOption s values item with
        | none => simp [hr] at resolved
        | some option =>
            simp only [hr,Option.bind_eq_bind,Option.bind_some] at resolved
            cases ht : resolveOptions s values rest with
            | none => simp [ht] at resolved
            | some tail =>
                simp only [ht,Option.bind_some,Option.pure_def,Option.some.injEq] at resolved
                subst options
                exact .cons ((option_exact _ _ _ _).mp hr) ((ih _).mp ht)
      · intro elaborated
        cases elaborated with
        | cons head tail => simp [resolveOptions,(option_exact _ _ _ _).mpr head,(ih _).mpr tail]

def resolveChain (s : MixedInstanceState.State d p n) (values : Values) (decl : ChainSyntax) : Option FallbackStatic.Chain := do
  let reader ← instanceKey values decl.reader
  let _ ← CompositionCore.index n reader
  let options ← resolveOptions s values decl.options
  return ⟨reader,options,decl.edges⟩

def ChainElaborates (s : MixedInstanceState.State d p n) (values : Values) (decl : ChainSyntax) (chain : FallbackStatic.Chain) : Prop :=
  (∃ reader : Fin n, instanceKey values decl.reader = some reader.val ∧ chain.reader = reader.val) ∧
  OptionsElaborate s values decl.options chain.options ∧ chain.edges = decl.edges

theorem chain_exact (s : MixedInstanceState.State d p n) (values : Values) (decl : ChainSyntax) (chain : FallbackStatic.Chain) :
    resolveChain s values decl = some chain ↔ ChainElaborates s values decl chain := by
  constructor
  · intro resolved
    unfold resolveChain at resolved
    cases named : instanceKey values decl.reader with
    | none => simp [named] at resolved
    | some raw =>
        simp only [named,Option.bind_eq_bind,Option.bind_some] at resolved
        cases indexed : CompositionCore.index n raw with
        | none => simp [indexed] at resolved
        | some reader =>
            simp only [indexed,Option.bind_some] at resolved
            cases options : resolveOptions s values decl.options with
            | none => simp [options] at resolved
            | some result =>
                simp only [options,Option.bind_some,Option.pure_def,Option.some.injEq] at resolved
                subst chain
                have equal := CompositionCore.index_sound _ indexed
                exact ⟨⟨reader,by simpa [equal] using named,equal.symm⟩,(options_exact _ _ _ _).mp options,rfl⟩
  · rintro ⟨⟨reader,named,equal⟩,options,edges⟩
    have record : chain = ⟨reader.val,chain.options,decl.edges⟩ := by cases chain; simp_all
    simp only [resolveChain,named,Option.bind_eq_bind,Option.bind_some,CompositionCore.index_roundtrip,
      (options_exact _ _ _ _).mpr options,Option.pure_def]
    exact congrArg some record.symm

theorem pure_option (s : InstanceState.State d p n) (values : Values) (decl : OptionSyntax) :
 resolveOption (MixedCatalogEmbedding.state s) values decl = ReferenceSourceData.resolveOption s values decl := by
 unfold resolveOption ReferenceSourceData.resolveOption
 cases named : instanceKey values decl.target with
 | none => rfl
 | some raw =>
   simp only [Option.bind_eq_bind,Option.bind_some]
   cases indexed : CompositionCore.index n raw with
   | none => rfl
   | some key => rfl

theorem pure_options (s : InstanceState.State d p n) (values : Values) (decls : List OptionSyntax) :
 resolveOptions (MixedCatalogEmbedding.state s) values decls = ReferenceSourceData.resolveOptions s values decls := by
 induction decls with
 | nil => rfl
 | cons item rest ih => simp only [resolveOptions,ReferenceSourceData.resolveOptions,pure_option,ih]

theorem pure_chain (s : InstanceState.State d p n) (values : Values) (decl : ChainSyntax) :
 resolveChain (MixedCatalogEmbedding.state s) values decl = ReferenceSourceData.resolveChain s values decl := by
 unfold resolveChain ReferenceSourceData.resolveChain
 simp only [pure_options]

-- Kind extraction only elaborates the descriptor. The actual catalog definition,
-- lifetime floor, selected access and holding gates are separate consumers.
#print axioms option_exact
#print axioms options_exact
#print axioms chain_exact
#print axioms pure_option
#print axioms pure_chain
end MirroreaProofFirst.MixedReferenceSourceData
