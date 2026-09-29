import MixedReferenceAccess
namespace MirroreaProofFirst.MixedReferenceEmbedding
open MixedReferenceAccess

theorem current_exact (s : ManagementEntry.System p a) (r : ReferenceAccess.Request)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) :
 current (MixedManagementEmbedding.system s) r member reader key place =
 some (ReferenceAccess.current s r member reader key place) := rfl

theorem physical_exact (s : ManagementEntry.System p a) (r : ReferenceAccess.Request)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) :
 physicalCheck (MixedManagementEmbedding.system s) r member reader key place =
 ReferenceAccess.physicalCheck s r member reader key place := by
 simp only [physicalCheck,ReferenceAccess.physicalCheck,MixedInstanceState.captureCheck,InstanceState.captureCheck]
 simp only [decide_true,Bool.true_and]
 rfl

theorem ready_exact (s : ManagementEntry.System p a) (r : ReferenceAccess.Request)
 (option : FallbackStatic.OptionDecl) (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) :
 readyCheck (MixedManagementEmbedding.system s) r option member reader key place =
 ReferenceAccess.readyCheck s r option member reader key place := by
 unfold readyCheck ReferenceAccess.readyCheck
 rw [physical_exact]
 have staticEq : MixedFallbackStatic.check (MixedManagementEmbedding.system s).configuration.state r.chain =
   FallbackStatic.check s.configuration.state r.chain := MixedFallbackStatic.pure_check _ _
 rw [staticEq]
 simp [MixedManagementEmbedding.system,MixedCoreEmbedding.config,MixedCatalogEmbedding.state,MixedCatalogEmbedding.embedInstance]
 cases FallbackStatic.check s.configuration.state r.chain <;> rfl

theorem revalidate_exact (s : ManagementEntry.System p a) (policy : CurrentUse.Policy)
 (ctx : ReferenceAccess.Context) (g : ReferenceAccess.Guard) :
 revalidate (MixedManagementEmbedding.system s) policy ctx g = ReferenceAccess.revalidate s policy ctx g := rfl

theorem checkAt_exact (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (r : ReferenceAccess.Request)
 (option : FallbackStatic.OptionDecl) (member : Fin a) (reader key : Fin s.configuration.count)
 (place : Fin p) (g : ReferenceAccess.Guard) :
 checkAt (MixedManagementEmbedding.system s) policy r option member reader key place g =
 ReferenceAccess.checkAt s policy r option member reader key place g := by
 simp only [checkAt,ReferenceAccess.checkAt,current_exact,ready_exact,revalidate_exact]

theorem prepareAt_exact (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (r : ReferenceAccess.Request)
 (option : FallbackStatic.OptionDecl) (member : Fin a) (reader key : Fin s.configuration.count)
 (place : Fin p) :
 prepareAt (MixedManagementEmbedding.system s) policy r option member reader key place =
 ReferenceAccess.prepareAt s policy r option member reader key place := by
 simp only [prepareAt,ReferenceAccess.prepareAt,ready_exact,current_exact,Option.bind_eq_bind,Option.bind_some]
 rfl

theorem resolve_exact (s : ManagementEntry.System p a) (r : ReferenceAccess.Request) :
 resolve (MixedManagementEmbedding.system s) r = ReferenceAccess.resolve s r := rfl

theorem check_exact (s : ManagementEntry.System p a) (policy : CurrentUse.Policy)
 (r : ReferenceAccess.Request) (g : ReferenceAccess.Guard) :
 check (MixedManagementEmbedding.system s) policy r g = ReferenceAccess.check s policy r g := by
 simp only [check,ReferenceAccess.check,resolve_exact,checkAt_exact]
 cases ReferenceAccess.resolve s r with
 | none => rfl
 | some coords => rcases coords with ⟨option,member,reader,key,place⟩; rfl

theorem prepare_exact (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (r : ReferenceAccess.Request) :
 prepare (MixedManagementEmbedding.system s) policy r = ReferenceAccess.prepare s policy r := by
 simp only [prepare,ReferenceAccess.prepare,resolve_exact,prepareAt_exact]
 rfl

theorem saved_exact (s : ManagementEntry.System p a) (policy : CurrentUse.Policy)
 (r : ReferenceAccess.Request) (g : ReferenceAccess.Guard) :
 Saved (MixedManagementEmbedding.system s) policy r g ↔ ReferenceAccess.Saved s policy r g := by
 rw [← MixedReferenceAccess.check_exact,← ReferenceAccess.check_exact,check_exact]

#print axioms current_exact
#print axioms ready_exact
#print axioms checkAt_exact
#print axioms prepareAt_exact
#print axioms check_exact
#print axioms prepare_exact
#print axioms saved_exact
end MirroreaProofFirst.MixedReferenceEmbedding
