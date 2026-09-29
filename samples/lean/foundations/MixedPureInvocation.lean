import MixedCatalogUse
namespace MirroreaProofFirst.MixedPureInvocation
open InstancePrograms CurrentUse WorldProjection
open MixedCatalogService (operationDefinition)
variable {id : Nat}
abbrev Ticket := InvocationBoundary.Ticket
abbrev savedRequest := @InvocationBoundary.savedRequest
abbrev execute := InvocationBoundary.execute

def request (s : MixedInstanceState.State d p n) (v : AuthorityView a) (member : Fin a)
 (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int) : UseRequest (size a p n) :=
 ⟨principal,⟨s.realm,encode (.member member),(MixedCatalogService.slotRecord s v (.member member)).identity⟩,
  ⟨s.realm,encode (.locus place),(MixedCatalogService.slotRecord s v (.locus place)).identity⟩,
  ⟨s.realm,encode (.moduleSlot key),(MixedCatalogService.slotRecord s v (.moduleSlot key)).identity⟩,
  ⟨s.realm,encode (.operation key place),(MixedCatalogService.slotRecord s v (.operation key place)).identity⟩,
  id,[.integer arg]⟩

def ticket (s : MixedInstanceState.State d p n) (v : AuthorityView a) (member : Fin a)
 (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int) (defn : Definition) (e : Evidence) : Ticket :=
 {realm := s.realm,member := member.val,key := key.val,place := place.val,principal := principal,id := id,argument := arg,
  memberIdentity := (MixedCatalogService.slotRecord s v (.member member)).identity,
  locusIdentity := (MixedCatalogService.slotRecord s v (.locus place)).identity,
  moduleIdentity := (MixedCatalogService.slotRecord s v (.moduleSlot key)).identity,
  operationIdentity := (MixedCatalogService.slotRecord s v (.operation key place)).identity,
  definition := defn,arithmeticProfile := 1,contractTheoryVersion := 1,evidence := e}

def domain (s : MixedInstanceState.State d p n) (key : Fin n) : List Int :=
 match (s.instances key).interface with | .pure c => c.inputs | .owner _ => []

def checkAt (s : MixedInstanceState.State d p n) (v : AuthorityView a) (t : Ticket)
 (member : Fin a) (key : Fin n) (place : Fin p) : Bool :=
 decide (t.member = member.val ∧ t.key = key.val ∧ t.place = place.val) &&
 decide (t.arithmeticProfile = 1 ∧ t.contractTheoryVersion = 1) &&
 decide (.pure t.definition = operationDefinition s key) &&
 (domain s key).contains t.argument &&
 checkUse (MixedCatalogService.world s v) (savedRequest t member key place) t.evidence

def Current (s : MixedInstanceState.State d p n) (v : AuthorityView a) (t : Ticket)
 (member : Fin a) (key : Fin n) (place : Fin p) : Prop :=
 t.member = member.val ∧ t.key = key.val ∧ t.place = place.val ∧
 t.arithmeticProfile = 1 ∧ t.contractTheoryVersion = 1 ∧
 .pure t.definition = operationDefinition s key ∧ t.argument ∈ domain s key ∧
 CurrentUse.CurrentUse (MixedCatalogService.world s v) (savedRequest t member key place)

theorem checkAt_sound (h : checkAt s v t member key place = true) : Current s v t member key place := by
 simp only [checkAt,Bool.and_eq_true,decide_eq_true_eq,List.contains_iff_mem] at h
 exact ⟨h.1.1.1.1.1,h.1.1.1.1.2.1,h.1.1.1.1.2.2,h.1.1.1.2.1,h.1.1.1.2.2,h.1.1.2,h.1.2,checkUse_sound _ _ _ h.2⟩

def check (s : MixedInstanceState.State d p n) (v : AuthorityView a) (t : Ticket) : Bool :=
 match CompositionCore.index a t.member,CompositionCore.index n t.key,CompositionCore.index p t.place with
 | some member,some key,some place => checkAt s v t member key place
 | _,_,_ => false

theorem check_parts (h : check s v t = true) : ∃ member key place, checkAt s v t member key place = true := by
 unfold check at h
 split at h
 · exact ⟨_,_,_,h⟩
 · cases h

def prepare (s : MixedInstanceState.State d p n) (v : AuthorityView a) (member : Fin a)
 (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int) : Option Ticket := do
 let .pure defn := operationDefinition s key | none
 let u := request s v member key place principal id arg
 let e ← CurrentUse.authorize (MixedCatalogService.world s v).authority
   ((MixedCatalogService.world s v).policies u.operation.key) (currentContext (MixedCatalogService.world s v) u)
 let t := ticket s v member key place principal id arg defn e
 if checkAt s v t member key place then some t else none

theorem saved_fresh : savedRequest (ticket s v member key place principal id arg defn e) member key place =
 request s v member key place principal id arg := rfl

theorem prepare_checked (h : prepare s v member key place principal id arg = some t) : check s v t = true := by
 unfold prepare at h
 cases kind : operationDefinition s key with
 | owner _ => simp [kind] at h
 | pure defn =>
   simp only [kind] at h
   cases ha : CurrentUse.authorize (MixedCatalogService.world s v).authority
     ((MixedCatalogService.world s v).policies (request s v member key place principal id arg).operation.key)
     (currentContext (MixedCatalogService.world s v) (request s v member key place principal id arg)) with
   | none => simp [ha] at h
   | some e =>
     simp only [ha,Option.bind_eq_bind,Option.bind_some] at h
     split at h
     · rename_i accepted
       cases h
       simpa only [check,ticket,CompositionCore.index_roundtrip] using accepted
     · cases h

theorem prepare_complete (kind : operationDefinition s key = .pure defn)
 (use : CurrentUse.CurrentUse (MixedCatalogService.world s v) (request s v member key place principal id arg))
 (input : arg ∈ domain s key) : ∃ t, prepare s v member key place principal id arg = some t := by
 obtain ⟨e,authorized,checked⟩ := authorize_use_complete _ _ use
 have accepted : checkAt s v (ticket s v member key place principal id arg defn e) member key place = true := by
   simp only [checkAt,saved_fresh]
   simp [ticket,kind,input,checked]
 exact ⟨ticket s v member key place principal id arg defn e,by simp [prepare,kind,authorized,accepted]⟩

theorem current_execution (valid : MixedInstanceState.Valid s) (h : check s v t = true) :
 ∃ result, Machine.Executes t.definition.code t.argument result ∧ Output t.definition.contract result := by
 obtain ⟨member,key,place,checked⟩ := check_parts h
 have cur := checkAt_sound checked
 have contract := valid.interfaces key
 have total := valid.definitions (s.instances key).definition
 change MixedOperationDefinitions.Refines (s.instances key).interface (operationDefinition s key).contract at contract
 change MixedOperationDefinitions.Satisfies (operationDefinition s key) at total
 rw [←cur.2.2.2.2.2.1] at contract total
 have input := cur.2.2.2.2.2.2.1
 cases shape : (s.instances key).interface with
 | owner _ => simp [domain,shape] at input
 | pure c =>
   simp only [domain,shape] at input
   simp only [shape,MixedOperationDefinitions.Refines,MixedOperationDefinitions.Definition.contract] at contract
   exact total.total t.argument (contract.inputs t.argument input)

def resultCheck (s : MixedInstanceState.State d p n) (v : AuthorityView a) (t : Ticket) (result : Int) : Bool :=
 check s v t && decide (execute t = some result)

def ResultMeaning (s : MixedInstanceState.State d p n) (v : AuthorityView a) (t : Ticket) (result : Int) : Prop :=
 (∃ member key place, Current s v t member key place) ∧ Machine.Executes t.definition.code t.argument result

theorem result_sound (h : resultCheck s v t result = true) : ResultMeaning s v t result := by
 simp only [resultCheck,Bool.and_eq_true,decide_eq_true_eq] at h
 obtain ⟨member,key,place,checked⟩ := check_parts h.1
 exact ⟨⟨member,key,place,checkAt_sound checked⟩,(Machine.run_exact _ _ _).mp h.2⟩

theorem fresh_result_completes (valid : MixedInstanceState.Valid s) (h : check s v t = true) :
 ∃ result, execute t = some result ∧ resultCheck s v t result = true := by
 obtain ⟨result,exec,_⟩ := current_execution valid h
 have computed := (Machine.run_exact _ _ _).mpr exec
 exact ⟨result,computed,by simp [resultCheck,execute,InvocationBoundary.execute,h,computed]⟩

theorem pure_checkAt (s : InstanceState.State d p n) (v : AuthorityView a) (t : Ticket)
 (member : Fin a) (key : Fin n) (place : Fin p) :
 checkAt (MixedCatalogEmbedding.state s) v t member key place = InvocationBoundary.checkAt s v t member key place := by
 simp only [checkAt,InvocationBoundary.checkAt,MixedCatalogService.pure_world]
 simp [domain,MixedCatalogEmbedding.state,MixedCatalogEmbedding.embedInstance,operationDefinition]

theorem pure_check (s : InstanceState.State d p n) (v : AuthorityView a) (t : Ticket) :
 check (MixedCatalogEmbedding.state s) v t = InvocationBoundary.check s v t := by
 simp only [check,InvocationBoundary.check]
 cases ha : CompositionCore.index a t.member <;> cases hn : CompositionCore.index n t.key <;>
 cases hp : CompositionCore.index p t.place <;> simp only [pure_checkAt]

theorem pure_request (s : InstanceState.State d p n) (v : AuthorityView a) (member : Fin a)
 (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int) :
 request (MixedCatalogEmbedding.state s) v member key place principal id arg = WorldProjection.request s v member key place principal id arg := rfl

theorem pure_prepare (s : InstanceState.State d p n) (v : AuthorityView a) (member : Fin a)
 (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int) :
 prepare (MixedCatalogEmbedding.state s) v member key place principal id arg = InvocationBoundary.prepare s v member key place principal id arg := by
 have kind : operationDefinition (MixedCatalogEmbedding.state s) key = .pure (s.definitions (s.instances key).definition) := rfl
 have same : ∀ e, ticket (MixedCatalogEmbedding.state s) v member key place principal id arg (s.definitions (s.instances key).definition) e = InvocationBoundary.ticket s v member key place principal id arg e := fun _ => rfl
 simp only [prepare,InvocationBoundary.prepare,pure_request,MixedCatalogService.pure_world,kind,same,pure_checkAt]

theorem pure_result (s : InstanceState.State d p n) (v : AuthorityView a) (t : Ticket) (result : Int) :
 resultCheck (MixedCatalogEmbedding.state s) v t result = InvocationBoundary.resultCheck s v t result := by
 simp only [resultCheck,InvocationBoundary.resultCheck,pure_check,execute]

#print axioms prepare_checked
#print axioms prepare_complete
#print axioms current_execution
#print axioms result_sound
#print axioms fresh_result_completes
#print axioms pure_prepare
#print axioms pure_result
end MirroreaProofFirst.MixedPureInvocation
