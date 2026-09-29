import OwnerMetadataStore
import MixedOwnerSourceDeclaration
namespace MirroreaProofFirst.OwnerMetadataSource
open OwnerMetadataRegistry MixedOwnerSourceIssue

-- Compiler/control consumer candidate. The future checked artifact determines
-- change/schema; this entry does not authenticate a raw schema or count as
-- parsed source construction. It consumes the SAME retained source allocator.
def marked (s : State p a) (machine : MixedReferenceExecution.Machine p a) (site : ReferenceSourceData.Site) : State p a :=
 merge s (MixedReferenceSource.mark (base s) machine (some site) .control 1)

theorem marked_waiting : (marked s machine site).waiting = s.waiting := by
 cases held : s.waiting with
 | none => simp [marked,merge,base,ownerWaiting,pureWaiting,MixedReferenceSource.mark,held]
 | some saved => cases saved <;> simp [marked,merge,base,ownerWaiting,pureWaiting,MixedReferenceSource.mark,held]

def changeMetadata (s : State p a) (registry : Registry) (member : Fin a) (place : Fin p)
 (principal : Nat) (site : ReferenceSourceData.Site) (change : Change) : Option (State p a × Registry) := do
 let (machine,next) ← OwnerMetadataStore.execute s.machine registry member place principal s.nextRequest change
 return (marked s machine site,next)

theorem parts (accepted : changeMetadata s registry member place principal site change = some result) :
 ∃ machine next, OwnerMetadataStore.execute s.machine registry member place principal s.nextRequest change = some (machine,next) ∧
 result = (marked s machine site,next) := by
 unfold changeMetadata at accepted
 cases done : OwnerMetadataStore.execute s.machine registry member place principal s.nextRequest change with
 | none => simp [done] at accepted
 | some pair =>
   obtain ⟨machine,next⟩ := pair
   simp only [done,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
   exact ⟨machine,next,rfl,accepted.symm⟩

theorem preserves (valid : MixedOwnerSourceInvariant.Valid s) (agrees : MixedOwnerSourceInvariant.OwnerAgrees s)
 (metadata : Consistent registry)
 (accepted : changeMetadata s registry member place principal site change = some result) :
 MixedOwnerSourceInvariant.Valid result.1 ∧ MixedOwnerSourceInvariant.OwnerAgrees result.1 ∧ Consistent result.2 := by
 obtain ⟨machine,next,done,rfl⟩ := parts accepted
 have preserved := OwnerMetadataStore.execute_preserves valid.1 metadata done
 have bound := OwnerMetadataStore.execute_below valid.2.1 done
 have pending := OwnerMetadataStore.execute_pending done
 refine ⟨⟨preserved.1,bound,?_⟩,?_,preserved.2⟩
 · have wait := marked_waiting (s:=s) (machine:=machine) (site:=site)
   change MixedReferenceSource.PendingAgrees (base (marked s machine site))
   unfold MixedReferenceSource.PendingAgrees base
   rw [wait]
   change MixedReferenceSource.PendingAgrees {base s with machine := machine}
   have classified : machine.pending = s.machine.pending := pending.1
   simpa only [MixedReferenceSource.PendingAgrees,base,classified] using valid.2.2
 · intro waiting held
   rw [marked_waiting] at held
   change machine.store.core.pending = [.owner waiting.saved]
   rw [pending.2]
   exact agrees waiting held

theorem allocated_once (accepted : changeMetadata s registry member place principal site change = some result) :
 result.1.nextRequest = s.nextRequest+1 ∧ result.1.waiting = s.waiting ∧
 result.1.values = s.values ∧ result.1.writes = s.writes := by
 obtain ⟨_,_,_,rfl⟩ := parts accepted
 exact ⟨rfl,marked_waiting,rfl,rfl⟩

theorem actual_occurrence (accepted : changeMetadata s registry member place principal site change = some result) :
 ∃ context,
 result.1.machine.store.events = .metadata context :: s.machine.store.events ∧
 result.1.origins = ⟨some site,.control,s.machine.store.events.length,s.machine.store.events.length+1⟩::s.origins ∧
 context = OwnerMetadataManagement.context (OwnerMetadataStore.state s.machine.store registry)
   member place principal s.nextRequest change := by
 obtain ⟨machine,next,done,rfl⟩ := parts accepted
 obtain ⟨evidence,store,new,committed,equal⟩ := OwnerMetadataStore.execute_parts done
 have occurrence := (OwnerMetadataStore.commit_event committed).1
 change store.events = _ at occurrence
 have machineEq := congrArg Prod.fst equal
 change machine = _ at machineEq
 subst machine
 refine ⟨_,occurrence,?_,rfl⟩
 change _ :: s.origins = _ :: s.origins
 simp only [occurrence,List.length_cons]
 rfl

theorem relative_complete (valid : MixedOwnerSourceInvariant.Valid s)
 (allowed : OwnerMetadataManagement.Allowed (OwnerMetadataStore.state s.machine.store registry)
   member place principal s.nextRequest change)
 (rule : Prepares registry change next) :
 ∃ result, changeMetadata s registry member place principal site change = some result := by
 have fresh := MixedReferenceAllocation.fresh_bound s.machine s.nextRequest
   s.machine.store.core.system.configuration.state.realm principal valid.2.1
 obtain ⟨result,done⟩ := OwnerMetadataStore.execute_complete fresh allowed rule
 exact ⟨(marked s result.1 site,result.2),by simp [changeMetadata,done]⟩

#print axioms marked_waiting
#print axioms preserves
#print axioms allocated_once
#print axioms actual_occurrence
#print axioms relative_complete
end MirroreaProofFirst.OwnerMetadataSource
