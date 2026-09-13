import MirroreaProofFirstReferenceAccessHistory

namespace MirroreaProofFirst.ReferenceHolding
open ReferenceAccess CurrentUse WorldProjection

-- Two explicitly different LAB lifetime interpretations, not equivalent
-- implementations or Canon-approved rights. No issuer authority is created.
-- H separates retention from future acquisition. H2 couples retention to the
-- scoped acquisition-capability leaves, without replaying an acquire permit.
inductive Entitlement where
  | separate | acquisition
  deriving DecidableEq, Repr
def action : Entitlement → Nat
  | .separate => 14 | .acquisition => 9

structure Context where
  entitlement : Entitlement
  scope : CurrentUse.Context
  request : Request
  memberIdentity : RecordIdentity
  locusIdentity : RecordIdentity
  deriving DecidableEq, Repr

structure Guard where
  context : Context
  policy : Nat
  version : Nat
  witness : Witness
  deriving DecidableEq, Repr

-- Stable binding descriptor: never the mutable selected choice/code, the
-- selected-access activation, a one-shot occurrence id, or semantic serial.
-- The exact structural descriptor is distinct from issuer scope: the existing
-- finite claim language grants action/target permission over admitted payloads,
-- not a predicate inspecting every chain/epoch field.
def current (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request)
    (member : Fin a) (place : Fin p) : Context :=
  let data := s.configuration.state
  {entitlement := entitlement,
   scope :=
    {principal := r.principal,member := member.val,memberIncarnation := (s.view.members member).incarnation,
     locus := place.val,locusIncarnation := data.placeIncarnation place,
     moduleKey := r.chain.reader,moduleIncarnation := 0,action := action entitlement,
     target := r.binding,targetIncarnation := r.epoch,targetRevision := r.lineage,
     instanceId := data.realm,request := r.binding,arguments := [],code := 0,contract := 0,
     generation := s.view.generation},
   request := r,
   memberIdentity := ⟨.member,(s.view.members member).incarnation,(s.view.members member).revision⟩,
   locusIdentity := ⟨.locus,data.placeIncarnation place,0⟩}

def OwnerAt (s : ManagementEntry.System p a) (r : Request) (member : Fin a) (place : Fin p) : Prop :=
  ManagementEntry.CurrentActor s member place r.principal ∧
    r.member = member.val ∧ r.place = place.val ∧ r.optionIndex = 0

def ownerCheck (s : ManagementEntry.System p a) (r : Request) (member : Fin a) (place : Fin p) : Bool :=
  ManagementEntry.actorCheck s member place r.principal &&
    decide (r.member = member.val ∧ r.place = place.val ∧ r.optionIndex = 0)

theorem owner_exact (s : ManagementEntry.System p a) (r : Request) (member : Fin a) (place : Fin p) :
    ownerCheck s r member place = true ↔ OwnerAt s r member place := by
  simp only [ownerCheck,Bool.and_eq_true,ManagementEntry.actor_exact,decide_eq_true_eq,OwnerAt]

def SavedAt (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request)
    (member : Fin a) (place : Fin p) (guard : Guard) : Prop :=
  let policy := s.controlPolicy (action entitlement)
  OwnerAt s r member place ∧ guard.context = current entitlement s r member place ∧
    guard.policy = policy.id ∧ guard.version = policy.version ∧
    WitnessMeaning s.view.authority (current entitlement s r member place).scope policy.label policy.expression guard.witness

def checkAt (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request)
    (member : Fin a) (place : Fin p) (guard : Guard) : Bool :=
  let policy := s.controlPolicy (action entitlement)
  ownerCheck s r member place && decide (guard.context = current entitlement s r member place) &&
    decide (guard.policy = policy.id) && decide (guard.version = policy.version) &&
    checkWitness s.view.authority (current entitlement s r member place).scope policy.label policy.expression guard.witness

theorem checkAt_exact (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request)
    (member : Fin a) (place : Fin p) (guard : Guard) :
    checkAt entitlement s r member place guard = true ↔ SavedAt entitlement s r member place guard := by
  simp only [checkAt,Bool.and_eq_true,owner_exact,decide_eq_true_eq,witness_exact,SavedAt,and_assoc]

def AllowedAt (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request)
    (member : Fin a) (place : Fin p) : Prop :=
  let policy := s.controlPolicy (action entitlement)
  OwnerAt s r member place ∧
    Authorized s.view.authority (current entitlement s r member place).scope policy.label policy.expression

def prepareAt (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request)
    (member : Fin a) (place : Fin p) : Option Guard := do
  if !ownerCheck s r member place then none else do
    let policy := s.controlPolicy (action entitlement)
    let ctx := current entitlement s r member place
    let witness ← produce s.view.authority ctx.scope policy.label policy.expression
    return ⟨ctx,policy.id,policy.version,witness⟩

theorem prepareAt_checked (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request)
    (member : Fin a) (place : Fin p) (guard : Guard)
    (produced : prepareAt entitlement s r member place = some guard) :
    checkAt entitlement s r member place guard = true := by
  unfold prepareAt at produced
  split at produced
  · cases produced
  · rename_i owner
    have ownerAt : ownerCheck s r member place = true := by simpa using owner
    dsimp only at produced
    cases generated : produce s.view.authority (current entitlement s r member place).scope
        (s.controlPolicy (action entitlement)).label (s.controlPolicy (action entitlement)).expression with
    | none => simp [generated] at produced
    | some witness =>
        simp only [generated,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at produced
        subst guard
        simp [checkAt,ownerAt,produce_checked _ _ _ _ _ generated]

theorem prepareAt_complete (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request)
    (member : Fin a) (place : Fin p) (allowed : AllowedAt entitlement s r member place) :
    ∃ guard, prepareAt entitlement s r member place = some guard := by
  obtain ⟨witness,generated⟩ := produce_complete _ _ _ _ allowed.2
  exact ⟨⟨current entitlement s r member place,(s.controlPolicy (action entitlement)).id,
    (s.controlPolicy (action entitlement)).version,witness⟩,
    by simp [prepareAt,(owner_exact _ _ _ _).mpr allowed.1,generated]⟩

theorem saved_allowed (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request)
    (member : Fin a) (place : Fin p) (guard : Guard) (saved : SavedAt entitlement s r member place guard) :
    AllowedAt entitlement s r member place := by
  exact ⟨saved.1,checkWitness_sound _ _ _ _ _ ((witness_exact _ _ _ _ _).mpr saved.2.2.2.2)⟩

def check (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request) (guard : Guard) : Bool :=
  match CompositionCore.index a r.member, CompositionCore.index p r.place with
  | some member,some place => checkAt entitlement s r member place guard
  | _,_ => false

def Saved (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request) (guard : Guard) : Prop :=
  ∃ member place, SavedAt entitlement s r member place guard

theorem check_exact (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request) (guard : Guard) :
    check entitlement s r guard = true ↔ Saved entitlement s r guard := by
  constructor
  · intro checked
    unfold check at checked
    split at checked
    · rename_i member place _ _
      exact ⟨member,place,(checkAt_exact _ _ _ _ _ _).mp checked⟩
    · cases checked
  · rintro ⟨member,place,saved⟩
    have memberAt := saved.1.2.1
    have placeAt := saved.1.2.2.1
    simp [check,memberAt,placeAt,CompositionCore.index_roundtrip,(checkAt_exact _ _ _ _ _ _).mpr saved]

def prepare (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request) : Option Guard := do
  let member ← CompositionCore.index a r.member
  let place ← CompositionCore.index p r.place
  prepareAt entitlement s r member place

theorem prepare_checked (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request) (guard : Guard)
    (produced : prepare entitlement s r = some guard) : check entitlement s r guard = true := by
  unfold prepare at produced
  cases memberAt : CompositionCore.index a r.member with
  | none => simp [memberAt] at produced
  | some member =>
      cases placeAt : CompositionCore.index p r.place with
      | none => simp [memberAt,placeAt] at produced
      | some place =>
          simp only [memberAt,placeAt,Option.bind_eq_bind,Option.bind_some] at produced
          exact (check_exact _ _ _ _).mpr ⟨member,place,(checkAt_exact _ _ _ _ _ _).mp (prepareAt_checked _ _ _ _ _ _ produced)⟩

theorem prepare_complete (entitlement : Entitlement) (s : ManagementEntry.System p a) (r : Request)
    (allowed : ∃ member place, AllowedAt entitlement s r member place) :
    ∃ guard, prepare entitlement s r = some guard := by
  obtain ⟨member,place,meaning⟩ := allowed
  obtain ⟨guard,generated⟩ := prepareAt_complete _ _ _ _ _ meaning
  exact ⟨guard,by simp [prepare,meaning.1.2.1,meaning.1.2.2.1,CompositionCore.index_roundtrip,generated]⟩

def Continuous (entitlement : Entitlement) (request : Request) (guard : Guard)
    (history : List (ReferenceAccessHistory.Frame p a)) : Prop :=
  ∀ frame ∈ history, Saved entitlement frame.system request guard

def continuousCheck (entitlement : Entitlement) (request : Request) (guard : Guard)
    (history : List (ReferenceAccessHistory.Frame p a)) : Bool :=
  history.all (fun frame => check entitlement frame.system request guard)

theorem continuous_exact (entitlement : Entitlement) (request : Request) (guard : Guard)
    (history : List (ReferenceAccessHistory.Frame p a)) :
    continuousCheck entitlement request guard history = true ↔ Continuous entitlement request guard history := by
  simp only [continuousCheck,List.all_eq_true,check_exact,Continuous]

-- The origin bound excludes a future/empty suffix. It does not by itself prove
-- that the origin was installed by acquisition: the store's rooted occurrence
-- correspondence must establish that independently before this gate is used.
def Live (entitlement : Entitlement) (request : Request) (guard : Guard) (origin : Nat)
    (history : List (ReferenceAccessHistory.Frame p a)) : Prop :=
  origin < history.length ∧ Continuous entitlement request guard (history.drop origin)

def liveCheck (entitlement : Entitlement) (request : Request) (guard : Guard) (origin : Nat)
    (history : List (ReferenceAccessHistory.Frame p a)) : Bool :=
  decide (origin < history.length) && continuousCheck entitlement request guard (history.drop origin)

theorem live_exact (entitlement : Entitlement) (request : Request) (guard : Guard) (origin : Nat)
    (history : List (ReferenceAccessHistory.Frame p a)) :
    liveCheck entitlement request guard origin history = true ↔ Live entitlement request guard origin history := by
  simp only [liveCheck,Bool.and_eq_true,decide_eq_true_eq,continuous_exact,Live]

theorem future_origin_rejected (entitlement : Entitlement) (request : Request) (guard : Guard) (origin : Nat)
    (history : List (ReferenceAccessHistory.Frame p a)) (future : history.length ≤ origin) :
    liveCheck entitlement request guard origin history = false := by
  simp [liveCheck,Nat.not_lt.mpr future]

theorem introduced_live (entitlement : Entitlement) (request : Request) (guard : Guard)
    (past : List (ReferenceAccessHistory.Frame p a)) (frame : ReferenceAccessHistory.Frame p a)
    (admitted : Saved entitlement frame.system request guard) :
    Live entitlement request guard past.length (past ++ [frame]) := by
  refine ⟨by simp,?_⟩
  simpa [Continuous] using admitted

theorem loss_persists (entitlement : Entitlement) (request : Request) (guard : Guard) (origin : Nat)
    (past extra : List (ReferenceAccessHistory.Frame p a))
    (lost : continuousCheck entitlement request guard (past.drop origin) = false) :
    liveCheck entitlement request guard origin (past ++ extra) = false := by
  unfold liveCheck
  rw [List.drop_append]
  simp only [continuousCheck,List.all_append] at lost ⊢
  simp [lost]

theorem failed_frame_rejects (entitlement : Entitlement) (request : Request) (guard : Guard) (origin : Nat)
    (history : List (ReferenceAccessHistory.Frame p a)) (frame : ReferenceAccessHistory.Frame p a)
    (included : frame ∈ history.drop origin) (invalid : ¬ Saved entitlement frame.system request guard) :
    liveCheck entitlement request guard origin history = false := by
  cases checked : liveCheck entitlement request guard origin history with
  | false => rfl
  | true => exact False.elim (invalid (((live_exact _ _ _ _ _).mp checked).2 frame included))

theorem changed_descriptor_rejected (entitlement : Entitlement) (s : ManagementEntry.System p a)
    (request : Request) (guard : Guard) (different : guard.context.request ≠ request) :
    check entitlement s request guard = false := by
  cases checked : check entitlement s request guard with
  | false => rfl
  | true =>
      obtain ⟨member,place,saved⟩ := (check_exact _ _ _ _).mp checked
      exact False.elim (different (congrArg (fun ctx : Context => ctx.request) saved.2.1))

theorem empty_authority_rejected (entitlement : Entitlement) (s : ManagementEntry.System p a)
    (request : Request) (guard : Guard) (empty : s.view.authority.issued = []) :
    check entitlement s request guard = false := by
  cases checked : check entitlement s request guard with
  | false => rfl
  | true =>
      obtain ⟨member,place,saved⟩ := (check_exact _ _ _ _).mp checked
      exact False.elim (no_claim_no_authorization _ _ _ _ empty (saved_allowed _ _ _ _ _ _ saved).2)

-- Unrelated catalog/instance changes and serial movement need not destroy
-- holding. Only the listed owner-relevant components and policy are fixed;
-- no equality of configuration, target code, or entire machine is assumed.
theorem context_stable (entitlement : Entitlement) (before after : ManagementEntry.System p a)
    (request : Request) (member : Fin a) (place : Fin p)
    (viewAt : after.view = before.view)
    (realmAt : after.configuration.state.realm = before.configuration.state.realm)
    (placeAt : after.configuration.state.placeIncarnation place = before.configuration.state.placeIncarnation place) :
    current entitlement after request member place = current entitlement before request member place := by
  simp [current,viewAt,realmAt,placeAt]

theorem savedAt_stable (entitlement : Entitlement) (before after : ManagementEntry.System p a)
    (request : Request) (member : Fin a) (place : Fin p) (guard : Guard)
    (viewAt : after.view = before.view)
    (realmAt : after.configuration.state.realm = before.configuration.state.realm)
    (placeAt : after.configuration.state.placeIncarnation place = before.configuration.state.placeIncarnation place)
    (participationAt : after.configuration.state.participating place = before.configuration.state.participating place)
    (policyAt : after.controlPolicy (action entitlement) = before.controlPolicy (action entitlement)) :
    SavedAt entitlement after request member place guard ↔ SavedAt entitlement before request member place guard := by
  have contextAt := context_stable entitlement before after request member place viewAt realmAt placeAt
  simp only [SavedAt,contextAt,policyAt,viewAt,OwnerAt,ManagementEntry.CurrentActor,realmAt,participationAt]

structure Stamp where
  guard : Guard
  activation : Nat
  frontier : Nat
  deriving DecidableEq, Repr

-- A historical admission fact remains true even after the right is revoked.
-- Live is checked separately; putting Live in structural storage invariants
-- would incorrectly forbid actual authority-loss transitions.
structure Admitted (history : List (ReferenceAccessHistory.Frame p a)) (eventCount : Nat)
    (request : Request) (stamp : Stamp) : Prop where
  activation_bound : stamp.activation < history.length
  frontier_bound : stamp.frontier < eventCount
  saved : ∃ frame, history[stamp.activation]? = some frame ∧ Saved .separate frame.system request stamp.guard

theorem admitted_extend (history extra : List (ReferenceAccessHistory.Frame p a)) (before after : Nat)
    (request : Request) (stamp : Stamp) (admitted : Admitted history before request stamp)
    (events : before ≤ after) : Admitted (history ++ extra) after request stamp := by
  refine ⟨?_,Nat.lt_of_lt_of_le admitted.frontier_bound events,?_⟩
  · have bound := admitted.activation_bound; simp only [List.length_append]; omega
  · obtain ⟨frame,atOrigin,saved⟩ := admitted.saved
    exact ⟨frame,by simpa [List.getElem?_append_left admitted.activation_bound] using atOrigin,saved⟩

theorem fresh_admitted (history : List (ReferenceAccessHistory.Frame p a)) (frontier : Nat)
    (frame : ReferenceAccessHistory.Frame p a) (request : Request) (guard : Guard)
    (saved : Saved .separate frame.system request guard) :
    Admitted (history ++ [frame]) (frontier+1) request ⟨guard,history.length,frontier⟩ := by
  exact ⟨by simp,by simp,frame,by simp,saved⟩

#print axioms admitted_extend
#print axioms fresh_admitted
#print axioms context_stable
#print axioms savedAt_stable
#print axioms live_exact
#print axioms introduced_live
#print axioms loss_persists
#print axioms failed_frame_rejects
#print axioms changed_descriptor_rejected
#print axioms empty_authority_rejected
#print axioms check_exact
#print axioms prepare_checked
#print axioms prepare_complete
#print axioms saved_allowed
end MirroreaProofFirst.ReferenceHolding
