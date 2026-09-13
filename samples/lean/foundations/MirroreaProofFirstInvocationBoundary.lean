import MirroreaProofFirstManagementEntry

namespace MirroreaProofFirst.InvocationBoundary
open InstancePrograms InstanceState CompositionCore WorldProjection CurrentUse

-- A saved invocation is distinct from a persistent source binding. The four
-- identities and submitted witness are copied once, never regenerated when
-- validating a result after state changes. Numeric coordinates are internal.
structure Ticket where
  realm : Nat
  member : Nat
  key : Nat
  place : Nat
  principal : Nat
  id : Nat
  argument : Int
  memberIdentity : RecordIdentity
  locusIdentity : RecordIdentity
  moduleIdentity : RecordIdentity
  operationIdentity : RecordIdentity
  definition : Definition
  arithmeticProfile : Nat
  contractTheoryVersion : Nat
  evidence : CurrentUse.Evidence
  deriving DecidableEq, Repr

def ticket (s : State d p n) (v : AuthorityView a) (member : Fin a) (key : Fin n)
    (place : Fin p) (principal id : Nat) (arg : Int) (e : CurrentUse.Evidence) : Ticket :=
  {realm := s.realm,member := member.val,key := key.val,place := place.val,principal := principal,id := id,argument := arg,
   memberIdentity := (slotRecord s v (.member member)).identity,
   locusIdentity := (slotRecord s v (.locus place)).identity,
   moduleIdentity := (slotRecord s v (.moduleSlot key)).identity,
   operationIdentity := (slotRecord s v (.operation key place)).identity,
   definition := s.definitions (s.instances key).definition,
   arithmeticProfile := 1,contractTheoryVersion := 1,evidence := e}

-- Reconstruct only internal coordinates. The submitted stamp comes from t.
def savedRequest (t : Ticket) (member : Fin a) (key : Fin n) (place : Fin p) : UseRequest (size a p n) :=
  ⟨t.principal,⟨t.realm,encode (.member member),t.memberIdentity⟩,
    ⟨t.realm,encode (.locus place),t.locusIdentity⟩,
    ⟨t.realm,encode (.moduleSlot key),t.moduleIdentity⟩,
    ⟨t.realm,encode (.operation key place),t.operationIdentity⟩,t.id,[.integer t.argument]⟩

theorem saved_fresh (s : State d p n) (v : AuthorityView a) (member : Fin a) (key : Fin n)
    (place : Fin p) (principal id : Nat) (arg : Int) (e : CurrentUse.Evidence) :
    savedRequest (ticket s v member key place principal id arg e) member key place =
      request s v member key place principal id arg := rfl

def checkAt (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (member : Fin a) (key : Fin n) (place : Fin p) : Bool :=
  decide (t.member = member.val ∧ t.key = key.val ∧ t.place = place.val) &&
  decide (t.arithmeticProfile = 1 ∧ t.contractTheoryVersion = 1) &&
  decide (t.definition = s.definitions (s.instances key).definition) &&
  (s.instances key).interface.inputs.contains t.argument &&
  checkUse (world s v) (savedRequest t member key place) t.evidence

def Current (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (member : Fin a) (key : Fin n) (place : Fin p) : Prop :=
  t.member = member.val ∧ t.key = key.val ∧ t.place = place.val ∧
  t.arithmeticProfile = 1 ∧ t.contractTheoryVersion = 1 ∧
  t.definition = s.definitions (s.instances key).definition ∧
  t.argument ∈ (s.instances key).interface.inputs ∧
  CurrentUse.CurrentUse (world s v) (savedRequest t member key place)

theorem checkAt_sound (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (member : Fin a) (key : Fin n) (place : Fin p) (checked : checkAt s v t member key place = true) :
    Current s v t member key place := by
  simp only [checkAt,Bool.and_eq_true,decide_eq_true_eq,List.contains_iff_mem] at checked
  exact ⟨checked.1.1.1.1.1,checked.1.1.1.1.2.1,checked.1.1.1.1.2.2,
    checked.1.1.1.2.1,checked.1.1.1.2.2,checked.1.1.2,checked.1.2,
    checkUse_sound _ _ _ checked.2⟩

def check (s : State d p n) (v : AuthorityView a) (t : Ticket) : Bool :=
  match index a t.member,index n t.key,index p t.place with
  | some member,some key,some place => checkAt s v t member key place
  | _,_,_ => false

theorem check_parts (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (checked : check s v t = true) :
    ∃ member key place, checkAt s v t member key place = true := by
  unfold check at checked
  split at checked
  · exact ⟨_,_,_,checked⟩
  · cases checked

def prepare (s : State d p n) (v : AuthorityView a) (member : Fin a) (key : Fin n)
    (place : Fin p) (principal id : Nat) (arg : Int) : Option Ticket := do
  let u := request s v member key place principal id arg
  let e ← CurrentUse.authorize (world s v).authority ((world s v).policies u.operation.key)
    (currentContext (world s v) u)
  let t := ticket s v member key place principal id arg e
  if checkAt s v t member key place then some t else none

theorem prepare_checked (s : State d p n) (v : AuthorityView a) (member : Fin a) (key : Fin n)
    (place : Fin p) (principal id : Nat) (arg : Int) (t : Ticket)
    (prepared : prepare s v member key place principal id arg = some t) : check s v t = true := by
  unfold prepare at prepared
  cases he : CurrentUse.authorize (world s v).authority
      ((world s v).policies (request s v member key place principal id arg).operation.key)
      (currentContext (world s v) (request s v member key place principal id arg)) with
  | none => simp [he] at prepared
  | some e =>
      simp only [he] at prepared
      change (if checkAt s v (ticket s v member key place principal id arg e) member key place then
        some (ticket s v member key place principal id arg e) else none) = some t at prepared
      split at prepared
      · rename_i accepted
        cases prepared
        simpa only [check,ticket,index_roundtrip] using accepted
      · cases prepared

theorem prepare_complete (s : State d p n) (v : AuthorityView a) (member : Fin a) (key : Fin n)
    (place : Fin p) (principal id : Nat) (arg : Int)
    (use : CurrentUse.CurrentUse (world s v) (request s v member key place principal id arg))
    (domain : arg ∈ (s.instances key).interface.inputs) :
    ∃ t, prepare s v member key place principal id arg = some t := by
  obtain ⟨e,authorized,checked⟩ := authorize_use_complete _ _ use
  have accepted : checkAt s v (ticket s v member key place principal id arg e) member key place = true := by
    simp only [checkAt,saved_fresh]
    simp [ticket,domain,checked]
  exact ⟨ticket s v member key place principal id arg e,by simp [prepare,authorized,accepted]⟩

theorem grown_checkAt (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (definition : Fin d) (owner : Nat) (placements : List (Fin p))
    (parent : Option (Fin n)) (dependencies : Support.Formula (Fin n))
    (member : Fin a) (key : Fin n) (place : Fin p) :
    checkAt (instantiate s definition owner placements parent dependencies) v t member (Growth.left n 1 key) place =
      checkAt s v t member key place := by
  simp only [checkAt,savedRequest,CurrentUse.checkUse,CurrentUse.checkHandle,CurrentUse.currentContext,
    world,decode_encode,slotRecord,member_live,locus_live,module_live,operation_live,
    instantiate_old,instantiate_old_live,InstanceState.mapInstance]
  rfl

theorem current_execution (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (valid : Valid s) (checked : check s v t = true) :
    ∃ result, Machine.Executes t.definition.code t.argument result ∧
      Output t.definition.contract result := by
  obtain ⟨member,key,place,h⟩ := check_parts s v t checked
  have cur := checkAt_sound s v t member key place h
  have argument : t.argument ∈ (s.definitions (s.instances key).definition).contract.inputs :=
    (valid.interfaces key).inputs t.argument cur.2.2.2.2.2.2.1
  obtain ⟨result,exec,output⟩ := (valid.definitions (s.instances key).definition).total t.argument argument
  exact ⟨result,cur.2.2.2.2.2.1 ▸ exec,cur.2.2.2.2.2.1 ▸ output⟩

-- Actual Mir arithmetic is executed. A delivered result is checked against
-- this exact bounded definition, never joined to a separate expected JSON.
def execute (t : Ticket) : Option Int := Machine.run t.definition.code t.argument
def resultCheck (s : State d p n) (v : AuthorityView a) (t : Ticket) (result : Int) : Bool :=
  check s v t && decide (execute t = some result)

def ResultMeaning (s : State d p n) (v : AuthorityView a) (t : Ticket) (result : Int) : Prop :=
  (∃ member key place, Current s v t member key place) ∧
    Machine.Executes t.definition.code t.argument result

theorem result_sound (s : State d p n) (v : AuthorityView a) (t : Ticket) (result : Int)
    (checked : resultCheck s v t result = true) : ResultMeaning s v t result := by
  simp only [resultCheck,Bool.and_eq_true,decide_eq_true_eq] at checked
  obtain ⟨member,key,place,h⟩ := check_parts s v t checked.1
  exact ⟨⟨member,key,place,checkAt_sound s v t member key place h⟩,(Machine.run_exact _ _ _).mp checked.2⟩

theorem fresh_result_completes (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (valid : Valid s) (checked : check s v t = true) :
    ∃ result, execute t = some result ∧ resultCheck s v t result = true := by
  obtain ⟨result,exec,_⟩ := current_execution s v t valid checked
  have computed := (Machine.run_exact _ _ _).mpr exec
  exact ⟨result,computed,by simp [resultCheck,execute,checked,computed]⟩

theorem version_erasure_rejected (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (bad : t.arithmeticProfile ≠ 1 ∨ t.contractTheoryVersion ≠ 1) : check s v t = false := by
  unfold check
  split
  · rcases bad with bad | bad <;> simp [checkAt,bad]
  · rfl

namespace Controls
open InstanceState.Controls
def invocationClaim : Claim := ⟨10,4,0,7,0,1,91,23,[5],[6,7,8,10,11,12],0⟩
def invocationPolicy : Policy := ⟨31,1,0,.leaf ⟨4,23⟩⟩
def view : AuthorityView 1 :=
  {ManagementEntry.Controls.view with
    authority := {ManagementEntry.Controls.view.authority with issued := [invocationClaim]},
    policies := fun _ _ => invocationPolicy}
def prepared := prepare two view 0 1 2 7 10 2
#guard prepared.isSome
#guard (prepared.bind execute) = some 5
#guard (prepared.map fun t => resultCheck two view t 5) = some true
#guard (prepared.map fun t => resultCheck two view t 6) = some false
#guard (prepared.map fun t => check replaced view t) = some false
#guard (prepared.map fun t => check (join (leave two 2) 2) view t) = some false
#guard (prepared.map fun t => check two {view with authority := {view.authority with revoked := [10]}} t) = some false
#guard (prepared.map fun t => check two {view with members := fun k => {view.members k with revision := 1}} t) = some false
#guard (prepare two view 0 1 1 7 11 2).isNone
#guard (prepare two view 0 1 2 7 11 3).isNone
end Controls

#print axioms saved_fresh
#print axioms checkAt_sound
#print axioms prepare_checked
#print axioms prepare_complete
#print axioms grown_checkAt
#print axioms current_execution
#print axioms result_sound
#print axioms fresh_result_completes
#print axioms version_erasure_rejected
end MirroreaProofFirst.InvocationBoundary
