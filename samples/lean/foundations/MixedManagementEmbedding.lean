import MixedCoreEmbedding
namespace MirroreaProofFirst.MixedManagementEmbedding
open MixedCoreEmbedding

def system (s : ManagementEntry.System p a) : MixedManagementEntry.System p a :=
 ⟨config s.configuration,s.view,s.controlPolicy,s.serial,s.used⟩
def context (c : ManagementEntry.Context) : MixedManagementEntry.Context :=
 ⟨c.claimScope,raw c.payload,c.cut,c.memberIdentity,c.locusIdentity⟩
def evidence (e : ManagementEntry.Evidence) : MixedManagementEntry.Evidence :=
 ⟨context e.context,e.policy,e.version,e.witness⟩
def result (r : ManagementEntry.System p a × Option Nat) := (system r.1,r.2)

theorem raw_injective : Function.Injective raw := by
 intro x y h
 cases x <;> cases y <;> simp_all [raw]

theorem context_injective : Function.Injective context := by
 intro x y h
 cases x; cases y
 simp only [context,MixedManagementEntry.Context.mk.injEq] at h
 obtain ⟨rfl,payload,rfl,rfl,rfl⟩ := h
 have same := raw_injective payload
 cases same
 rfl

theorem owner_definition_false (s : ManagementEntry.System p a) (key : Nat) :
 MixedManagementEntry.ownerDefinition (system s) key = false := by
 unfold MixedManagementEntry.ownerDefinition
 cases h : MixedCompositionCore.index s.configuration.definitions key <;>
 simp [system,config,h,MixedCatalogEmbedding.state]

theorem owner_instance_false (s : ManagementEntry.System p a) (key : Nat) :
 MixedManagementEntry.ownerInstance (system s) key = false := by
 unfold MixedManagementEntry.ownerInstance
 cases h : MixedCompositionCore.index s.configuration.count key with
 | none => simp [system,config,h]
 | some k => simpa only [system,config,h,Option.some] using owner_definition_false s (s.configuration.state.instances k).definition.val

theorem action_exact (s : ManagementEntry.System p a) (r : CompositionCore.Raw) :
 MixedManagementEntry.action (system s) (raw r) = ManagementEntry.action r := by
 cases r <;> simp [raw,MixedManagementEntry.action,ManagementEntry.action,owner_definition_false,owner_instance_false]

theorem target_exact (r : CompositionCore.Raw) :
 MixedManagementEntry.target (raw r) = ManagementEntry.target r := by cases r <;> rfl

theorem current_exact (s : ManagementEntry.System p a) (member : Fin a) (place : Fin p) (principal id : Nat) (r : CompositionCore.Raw) :
 MixedManagementEntry.current (system s) member place principal id (raw r) = context (ManagementEntry.current s member place principal id r) := by
 simp only [MixedManagementEntry.current,ManagementEntry.current,action_exact,target_exact]
 rfl

theorem actor_exact (s : ManagementEntry.System p a) (member : Fin a) (place : Fin p) (principal : Nat) :
 MixedManagementEntry.actorCheck (system s) member place principal = ManagementEntry.actorCheck s member place principal := rfl

theorem revalidate_exact (s : ManagementEntry.System p a) (c : ManagementEntry.Context) (e : ManagementEntry.Evidence) :
 MixedManagementEntry.revalidate (system s) (context c) (evidence e) = ManagementEntry.revalidate s c e := by
 simp only [MixedManagementEntry.revalidate,ManagementEntry.revalidate,evidence,system,context]
 have eq : (context e.context = context c) = (e.context = c) := propext context_injective.eq_iff
 change (decide (context e.context = context c) && _ && _ && _) = _
 simp only [eq]
 rfl

theorem check_exact (s : ManagementEntry.System p a) (member : Fin a) (place : Fin p) (principal id : Nat) (r : CompositionCore.Raw) (e : ManagementEntry.Evidence) :
 MixedManagementEntry.check (system s) member place principal id (raw r) (evidence e) = ManagementEntry.check s member place principal id r e := by
 simp only [MixedManagementEntry.check,ManagementEntry.check,actor_exact,current_exact,revalidate_exact]

theorem authorize_exact (s : ManagementEntry.System p a) (member : Fin a) (place : Fin p) (principal id : Nat) (r : CompositionCore.Raw) :
 MixedManagementEntry.authorize (system s) member place principal id (raw r) = (ManagementEntry.authorize s member place principal id r).map evidence := by
 simp only [MixedManagementEntry.authorize,ManagementEntry.authorize,current_exact,actor_exact,action_exact]
 split
 · simp only [context,system,Option.map_map,Function.comp_def,evidence]
 · rfl

theorem after_exact (s : ManagementEntry.System p a) (cfg : CompositionCore.Config p) (principal id : Nat) :
 MixedManagementEntry.after (system s) (config cfg) principal id = system (ManagementEntry.after s cfg principal id) := rfl

theorem commit_exact (s : ManagementEntry.System p a) (member : Fin a) (place : Fin p) (principal id : Nat) (r : CompositionCore.Raw) (e : ManagementEntry.Evidence) :
 MixedManagementEntry.commit (system s) member place principal id (raw r) (evidence e) =
 (ManagementEntry.commit s member place principal id r e).map result := by
 simp only [MixedManagementEntry.commit,ManagementEntry.commit,check_exact]
 have use : MixedManagementEntry.useId (system s) principal id = ManagementEntry.useId s principal id := rfl
 simp only [use,show (system s).used = s.used from rfl]
 split
 · rfl
 · simp only [show (system s).configuration = config s.configuration from rfl,run_exact]
   cases h : CompositionCore.run s.configuration r with
   | none => rfl
   | some pair => cases pair; rfl

theorem perform_exact (s : ManagementEntry.System p a) (member : Fin a) (place : Fin p) (principal id : Nat) (r : CompositionCore.Raw) :
 MixedManagementEntry.perform (system s) member place principal id (raw r) =
 (ManagementEntry.perform s member place principal id r).map result := by
 simp only [MixedManagementEntry.perform,ManagementEntry.perform,authorize_exact]
 cases h : ManagementEntry.authorize s member place principal id r with
 | none => rfl
 | some e => simpa only [Option.map_some,Option.bind_eq_bind,Option.bind_some] using commit_exact s member place principal id r e

theorem head_exact (s : ManagementEntry.System p a) (view : WorldProjection.AuthorityView a) :
 MixedManagementEntry.installAuthorityHead (system s) view = system (ManagementEntry.installAuthorityHead s view) := rfl

#print axioms raw_injective
#print axioms context_injective
#print axioms current_exact
#print axioms revalidate_exact
#print axioms authorize_exact
#print axioms commit_exact
#print axioms perform_exact
#print axioms head_exact
end MirroreaProofFirst.MixedManagementEmbedding
