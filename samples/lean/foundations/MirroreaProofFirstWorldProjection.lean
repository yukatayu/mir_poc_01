import MirroreaProofFirstInstanceState

namespace MirroreaProofFirst.WorldProjection
open InstanceState

-- Internal complete enumeration. Members and loci have independent records;
-- each instance adds a module plus one operation slot per represented locus.
-- An unplaced operation stays present but is not eligible. p is a parameter of
-- this finite reference realm, not a final global roster or wire layout.
def size (a p n : Nat) : Nat := a + p + n * (p+1)

inductive Slot (a p n : Nat) where
  | member (key : Fin a)
  | locus (key : Fin p)
  | moduleSlot (key : Fin n)
  | operation (key : Fin n) (place : Fin p)
  deriving DecidableEq, Repr

def pack (key : Fin n) (offset : Fin (p+1)) : Fin (n*(p+1)) :=
  ⟨key.val*(p+1)+offset.val,by
    have bound := Nat.mul_le_mul_right (p+1) (Nat.succ_le_of_lt key.isLt)
    simp only [Nat.succ_mul] at bound
    have offsetBound := offset.isLt
    omega⟩

def unpack (key : Fin (n*(p+1))) : Fin n × Fin (p+1) :=
  (⟨key.val/(p+1),(Nat.div_lt_iff_lt_mul (by omega)).mpr key.isLt⟩,
   ⟨key.val%(p+1),Nat.mod_lt _ (by omega)⟩)

theorem unpack_pack (key : Fin n) (offset : Fin (p+1)) :
    unpack (pack key offset) = (key,offset) := by
  apply Prod.ext <;> apply Fin.ext
  · simp only [unpack,pack]
    rw [Nat.mul_comm key.val (p+1),Nat.mul_add_div (by omega)]
    simp [Nat.div_eq_of_lt offset.isLt]
  · simp [unpack,pack,Nat.mul_add_mod_of_lt offset.isLt]

theorem pack_unpack (key : Fin (n*(p+1))) : pack (unpack key).1 (unpack key).2 = key := by
  apply Fin.ext
  simpa only [unpack,pack,Nat.mul_comm] using Nat.div_add_mod key.val (p+1)

def moduleOffset : Fin (p+1) := ⟨0,by omega⟩
def operationOffset (place : Fin p) : Fin (p+1) := ⟨place.val+1,by have := place.isLt; omega⟩

def blockSlot (key : Fin (n*(p+1))) : Slot a p n :=
  let unit := (unpack key).1
  let offset := (unpack key).2
  if h : offset.val = 0 then .moduleSlot unit
  else .operation unit ⟨offset.val-1,by have := offset.isLt; omega⟩

def encode : Slot a p n → Fin (size a p n)
  | .member key => ⟨key.val,by have := key.isLt; simp only [size]; omega⟩
  | .locus key => ⟨a+key.val,by have := key.isLt; simp only [size]; omega⟩
  | .moduleSlot key => ⟨a+p+(pack key moduleOffset).val,by
      have := (pack key (moduleOffset (p:=p))).isLt; simp only [size]; omega⟩
  | .operation key place => ⟨a+p+(pack key (operationOffset place)).val,by
      have := (pack key (operationOffset place)).isLt; simp only [size]; omega⟩

def decode (key : Fin (size a p n)) : Slot a p n :=
  if h : key.val < a then .member ⟨key.val,h⟩
  else if hp : key.val < a+p then .locus ⟨key.val-a,by omega⟩
  else blockSlot ⟨key.val-(a+p),by have := key.isLt; simp only [size] at this; omega⟩

theorem decode_encode (slot : Slot a p n) : decode (encode slot) = slot := by
  cases slot with
  | member key => simp [decode,encode,key.isLt]
  | locus key =>
    have bound := key.isLt
    simp [decode,encode,show ¬ a+key.val < a by omega,show a+key.val < a+p by omega]
  | moduleSlot key =>
    simp only [encode,decode]
    rw [dif_neg (by omega),dif_neg (by omega)]
    simp only [Nat.add_sub_cancel_left]
    change blockSlot (pack key moduleOffset) = .moduleSlot key
    simp only [blockSlot,unpack_pack,moduleOffset]
    rfl

  | operation key place =>
    simp only [encode,decode]
    rw [dif_neg (by omega),dif_neg (by omega)]
    simp only [Nat.add_sub_cancel_left]
    change blockSlot (pack key (operationOffset place)) = .operation key place
    simp only [blockSlot,unpack_pack,operationOffset]
    simp

theorem encode_injective : Function.Injective (encode (a:=a) (p:=p) (n:=n)) := by
  intro x y h
  have := congrArg decode h
  simpa [decode_encode] using this

theorem encode_block_val (key : Fin (n*(p+1))) :
    (encode (blockSlot (a:=a) key)).val = a+p+key.val := by
  unfold blockSlot
  dsimp only
  split
  · rename_i h
    simp only [encode]
    have offset : (moduleOffset : Fin (p+1)) = (unpack key).2 := by
      apply Fin.ext
      exact h.symm
    rw [offset,pack_unpack]
  · rename_i h
    simp only [encode]
    have offset : operationOffset (⟨(unpack key).2.val-1,by
        have := (unpack key).2.isLt; omega⟩ : Fin p) = (unpack key).2 := by
      apply Fin.ext
      simp only [operationOffset]
      have := (unpack key).2.isLt
      omega
    rw [offset,pack_unpack]

theorem encode_decode (key : Fin (size a p n)) : encode (decode key) = key := by
  apply Fin.ext
  unfold decode
  split
  · rfl
  · split
    · simp only [encode]; omega
    · rw [encode_block_val]
      change a+p+(key.val-(a+p))=key.val
      omega

def liftSlot : Slot a p n → Slot a p (n+1)
  | .member key => .member key
  | .locus key => .locus key
  | .moduleSlot key => .moduleSlot (Growth.left n 1 key)
  | .operation key place => .operation (Growth.left n 1 key) place

theorem size_succ : size a p (n+1) = size a p n + (p+1) := by
  simp [size,Nat.succ_mul,Nat.add_assoc]

def oldKey (key : Fin (size a p n)) : Fin (size a p (n+1)) :=
  ⟨key.val,by rw [size_succ]; have := key.isLt; omega⟩

theorem encode_lift (slot : Slot a p n) : encode (liftSlot slot) = oldKey (encode slot) := by
  apply Fin.ext
  cases slot <;> rfl

theorem decode_old (key : Fin (size a p n)) :
    decode (oldKey key) = liftSlot (decode key) := by
  have h := decode_encode (liftSlot (decode key))
  rw [encode_lift,encode_decode] at h
  exact h

structure Member where
  principal : Nat
  incarnation : Nat
  revision : Nat
  enabled : Bool

-- An actual logical membership/authority store, never constructed from the
-- requesting principal. Its authenticity/current head remains a W4 mechanism.
structure AuthorityView (a : Nat) where
  realm : Nat
  generation : Nat
  members : Fin a → Member
  authority : CurrentUse.Authority
  policies : Nat → Nat → CurrentUse.Policy

def emptyAuthority : CurrentUse.Authority := ⟨[],[],fun _ => none⟩
def inactivePolicy : CurrentUse.Policy := ⟨0,0,0,.leaf ⟨0,0⟩⟩

def slotRecord (s : State d p n) (v : AuthorityView a) : Slot a p n → CurrentUse.Record (size a p n)
  | .member key =>
    {identity := ⟨.member,(v.members key).incarnation,(v.members key).revision⟩,
     principal := (v.members key).principal,home := encode (.member key),moduleKey := encode (.member key),
     action := 0,code := 0,contract := 0}
  | .locus place =>
    {identity := ⟨.locus,s.placeIncarnation place,0⟩,
     principal := 0,home := encode (.locus place),moduleKey := encode (.locus place),
     action := 0,code := 0,contract := 0}
  | .moduleSlot key =>
    {identity := ⟨.module,1,(s.instances key).revision⟩,
     principal := (s.instances key).owner,home := encode (.moduleSlot key),moduleKey := encode (.moduleSlot key),
     action := 0,code := (s.instances key).definition.val,contract := (s.instances key).definition.val}
  | .operation key place =>
    {identity := ⟨.operation,1,(s.instances key).revision⟩,
     principal := (s.instances key).owner,home := encode (.locus place),moduleKey := encode (.moduleSlot key),
     action := 5,code := (s.instances key).definition.val,contract := (s.instances key).definition.val}

-- This is the unary invocation profile only. Nonoperation action/code/contract
-- fields and member/locus self-links are unused by its four-handle use judgment.
-- A definition slot names an immutable (code, contract) pair in this model; it
-- is not the final public code or contract identifier or a source of authority.
def slotForm (s : State d p n) : Slot a p n → Support.Formula (Fin (size a p n))
  | .member _ | .locus _ => .top
  | .moduleSlot key => Growth.mapFormula (fun k => encode (.moduleSlot k)) ((snapshot s).forms key)
  | .operation key place => .both (.ref (encode (.moduleSlot key))) (.ref (encode (.locus place)))

def slotEligible (s : State d p n) (v : AuthorityView a) : Slot a p n → Bool
  | .member key => (v.members key).enabled && decide (s.realm = v.realm)
  | .locus place => s.participating place
  | .moduleSlot key => (snapshot s).eligible key
  | .operation key place => (s.instances key).placements.contains place && s.participating place

def support (s : State d p n) (v : AuthorityView a) : Support.Snapshot (size a p n) :=
  ⟨fun key => slotForm s (decode key),fun key => slotEligible s v (decode key)⟩

def world (s : State d p n) (v : AuthorityView a) : CurrentUse.World (size a p n) :=
  {instanceId := s.realm,generation := v.generation,support := support s v,
   records := fun key => slotRecord s v (decode key),
   authority := if s.realm = v.realm then v.authority else emptyAuthority,
   policies := fun key => match decode key with
     | .operation unit place => v.policies unit.val place.val
     | _ => inactivePolicy}

@[simp] theorem world_record (s : State d p n) (v : AuthorityView a) (slot : Slot a p n) :
    (world s v).records (encode slot) = slotRecord s v slot := by simp [world,decode_encode]

@[simp] theorem support_form (s : State d p n) (v : AuthorityView a) (slot : Slot a p n) :
    (support s v).forms (encode slot) = slotForm s slot := by simp [support,decode_encode]

@[simp] theorem support_eligible (s : State d p n) (v : AuthorityView a) (slot : Slot a p n) :
    (support s v).eligible (encode slot) = slotEligible s v slot := by simp [support,decode_encode]

theorem module_live (s : State d p n) (v : AuthorityView a) (key : Fin n) :
    Support.snapshotLive (support s v) (encode (.moduleSlot key)) = Support.snapshotLive (snapshot s) key := by
  apply Bool.eq_iff_iff.mpr
  rw [Support.snapshot_live_exact,Support.snapshot_live_exact]
  exact Growth.grounded_embedding (fun k => encode (.moduleSlot k)) _ _ _ _
    (fun _ => by simp [slotForm]) (fun _ => by simp [slotEligible]) key

theorem authority_retained (s : State d p n) (v : AuthorityView a) (same : s.realm = v.realm) :
    (world s v).authority = v.authority := by simp [world,same]

theorem realm_mismatch_denies (s : State d p n) (v : AuthorityView a) (different : s.realm ≠ v.realm)
    (request : CurrentUse.UseRequest (size a p n)) (evidence : CurrentUse.Evidence) :
    CurrentUse.checkUse (world s v) request evidence = false := by
  cases checked : CurrentUse.checkUse (world s v) request evidence with
  | false => rfl
  | true =>
    have allowed := (CurrentUse.checkUse_sound _ _ _ checked).2.2.2.2.2.2.2
    exact False.elim (CurrentUse.no_claim_no_authorization _ _ _ _
      (by simp [world,different,emptyAuthority]) allowed)

theorem live_equation (s : Support.Snapshot n) (key : Fin n) :
    Support.snapshotLive s key = (s.eligible key && Support.eval (Support.snapshotLive s) (s.forms key)) := by
  apply Bool.eq_iff_iff.mpr
  rw [Support.snapshot_live_exact,Bool.and_eq_true,Support.eval_correct]
  have h : Support.Holds (fun k => Support.snapshotLive s k = true) (s.forms key) ↔
      Support.Holds (Support.Grounded s.forms (fun k => s.eligible k = true)) (s.forms key) := by
    have eq : (fun k => Support.snapshotLive s k = true) =
        Support.Grounded s.forms (fun k => s.eligible k = true) := by
      funext k; exact propext (Support.snapshot_live_exact s k)
    rw [eq]
  rw [h]
  exact ⟨fun live => ⟨live.1,Support.proof_in_closed (Support.grounded_closed _ _) live.2⟩,
    fun live => Support.grounded_closed _ _ _ live.1 live.2⟩

@[simp] theorem member_live (s : State d p n) (v : AuthorityView a) (key : Fin a) :
    Support.snapshotLive (support s v) (encode (.member key)) =
      ((v.members key).enabled && decide (s.realm = v.realm)) := by
  rw [live_equation,support_form,support_eligible]
  simp [slotForm,slotEligible,Support.eval]

@[simp] theorem locus_live (s : State d p n) (v : AuthorityView a) (place : Fin p) :
    Support.snapshotLive (support s v) (encode (.locus place)) = s.participating place := by
  rw [live_equation,support_form,support_eligible]
  simp [slotForm,slotEligible,Support.eval]

@[simp] theorem operation_live (s : State d p n) (v : AuthorityView a) (key : Fin n) (place : Fin p) :
    Support.snapshotLive (support s v) (encode (.operation key place)) =
      ((s.instances key).placements.contains place && s.participating place &&
        Support.snapshotLive (snapshot s) key) := by
  rw [live_equation,support_form,support_eligible]
  simp [slotForm,slotEligible,Support.eval,module_live,Bool.and_left_comm,Bool.and_comm]

def handle (s : State d p n) (v : AuthorityView a) (slot : Slot a p n) :
    CurrentUse.Handle (size a p n) := ⟨s.realm,encode slot,(slotRecord s v slot).identity⟩

-- Fresh capture is an invocation entry. Saved requests retain all four handles;
-- they must never be reconstructed from a later state to validate old evidence.
def request (s : State d p n) (v : AuthorityView a) (member : Fin a)
    (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int) : CurrentUse.UseRequest (size a p n) :=
  ⟨principal,handle s v (.member member),handle s v (.locus place),
    handle s v (.moduleSlot key),handle s v (.operation key place),id,[.integer arg]⟩

theorem check_fresh_handle (s : State d p n) (v : AuthorityView a) (slot : Slot a p n)
    (kind : CurrentUse.RecordKind) :
    CurrentUse.checkHandle (world s v) kind (handle s v slot) =
      (decide ((slotRecord s v slot).identity.kind = kind) && Support.snapshotLive (support s v) (encode slot)) := by
  simp [CurrentUse.checkHandle,handle,world,decode_encode]
  rfl

theorem fresh_use_exact (s : State d p n) (v : AuthorityView a) (member : Fin a)
    (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int) (e : CurrentUse.Evidence) :
    CurrentUse.checkUse (world s v) (request s v member key place principal id arg) e =
      ((v.members member).enabled && decide (s.realm = v.realm) &&
       captureCheck s (capture s key place) key place && decide ((v.members member).principal = principal) &&
       CurrentUse.revalidate (world s v).authority (v.policies key.val place.val)
         (CurrentUse.currentContext (world s v) (request s v member key place principal id arg)) e) := by
  simp only [CurrentUse.checkUse,request,check_fresh_handle]
  simp [slotRecord,world,decode_encode,member_live,locus_live,module_live,operation_live,
    handle,captureCheck,Bool.and_assoc,Bool.and_left_comm,Bool.and_comm]

theorem saved_rejoin_rejected (s : State d p n) (v : AuthorityView a) (member : Fin a)
    (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int) (e : CurrentUse.Evidence) :
    CurrentUse.checkUse (world (join (leave s place) place) v)
      (request s v member key place principal id arg) e = false := by
  have stale := CurrentUse.stale_incarnation_rejected (world (join (leave s place) place) v)
    .locus (handle s v (.locus place)) (by simp [handle,world_record,slotRecord,join,leave])
  simp only [CurrentUse.checkUse,request,stale,Bool.and_false,Bool.false_and]

theorem saved_revision_rejected (s t : State d p n) (v : AuthorityView a) (member : Fin a)
    (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int) (e : CurrentUse.Evidence)
    (changed : (s.instances key).revision ≠ (t.instances key).revision) :
    CurrentUse.checkUse (world t v) (request s v member key place principal id arg) e = false := by
  have different : (slotRecord s v (.operation key place)).identity ≠
      (slotRecord t v (.operation key place)).identity := by
    intro eq
    exact changed (congrArg CurrentUse.RecordIdentity.revision eq)
  have stale : CurrentUse.checkHandle (world t v) .operation (handle s v (.operation key place)) = false := by
    simp [CurrentUse.checkHandle,handle,world_record,different]
  simp only [CurrentUse.checkUse,request,stale,Bool.and_false,Bool.false_and]

namespace Controls
#guard (encode (a:=2) (p:=3) (n:=1) (.moduleSlot 0)).val = 5
#guard (encode (a:=2) (p:=3) (n:=2) (.moduleSlot 1)).val = 9
#guard (encode (a:=2) (p:=3) (n:=2) (.operation 1 2)).val = 12
#guard decode (a:=2) (p:=3) (n:=2) ⟨12,by decide⟩ = .operation 1 2
end Controls

#print axioms unpack_pack
#print axioms pack_unpack
#print axioms decode_encode
#print axioms encode_injective
#print axioms encode_decode
#print axioms encode_lift
#print axioms decode_old
#print axioms world_record
#print axioms module_live
#print axioms authority_retained
#print axioms realm_mismatch_denies
#print axioms live_equation
#print axioms operation_live
#print axioms fresh_use_exact
#print axioms saved_rejoin_rejected
#print axioms saved_revision_rejected
end MirroreaProofFirst.WorldProjection
