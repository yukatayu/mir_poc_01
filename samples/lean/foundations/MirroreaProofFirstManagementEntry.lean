import MirroreaProofFirstCompositionCore

namespace MirroreaProofFirst.ManagementEntry
open CurrentUse WorldProjection InstanceState CompositionCore

-- Exact structural payload binding extends the W1 claim scope. The evidence is
-- a logical derivation, not a signature or a cryptographic digest. Policy leaves
-- in this profile constrain subject/member/action/target/label; they do not add
-- arbitrary argument predicates. All layers receive this same full context.
structure Context where
  claimScope : CurrentUse.Context
  payload : Raw
  cut : Nat
  memberIdentity : RecordIdentity
  locusIdentity : RecordIdentity
  deriving DecidableEq, Repr

structure Evidence where
  context : Context
  policy : Nat
  version : Nat
  witness : Witness
  deriving DecidableEq, Repr

structure System (p a : Nat) where
  configuration : Config p
  view : AuthorityView a
  controlPolicy : Nat → Policy
  serial : Nat
  used : List UseId

def Invariant (s : System p a) : Prop := CompositionCore.Invariant s.configuration ∧ s.used.Nodup

def action : Raw → Nat
  | .register .. => 1
  | .instantiate .. => 2
  | .retire .. => 3
  | .reparent .. => 4
  | .replace .. => 6
  | .leave .. => 7
  | .join .. => 8

-- Control targets inhabit an action-specific namespace. Registration and
-- instantiation require a realm-control target 0; existing-instance edits use
-- its instance key, locus transitions its locus key. Invocation action 5 is
-- disjoint and uses WorldProjection's operation key namespace.
def target : Raw → Nat
  | .register .. | .instantiate .. => 0
  | .retire key | .reparent key _ | .replace key _ => key
  | .leave place | .join place => place

def current (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat) (raw : Raw) : Context :=
  let data := s.configuration.state
  {claimScope :=
    {principal := principal, member := member.val,
     memberIncarnation := (s.view.members member).incarnation,
     locus := place.val, locusIncarnation := data.placeIncarnation place,
     moduleKey := 0,moduleIncarnation := 0,action := action raw,target := target raw,
     targetIncarnation := 0,targetRevision := 0,instanceId := data.realm,
     request := id,arguments := [],code := 0,contract := 0,generation := s.view.generation},
   payload := raw,cut := s.serial,
   memberIdentity := ⟨.member,(s.view.members member).incarnation,(s.view.members member).revision⟩,
   locusIdentity := ⟨.locus,data.placeIncarnation place,0⟩}

-- The zero/empty legacy fields above have no hidden code semantics: the entire
-- operation, definition term and contract are bound structurally by payload.
-- The cut binds target revisions and the immutable catalog of this local actor.
def CurrentActor (s : System p a) (member : Fin a) (place : Fin p) (principal : Nat) : Prop :=
  s.configuration.state.realm = s.view.realm ∧ (s.view.members member).enabled = true ∧
  (s.view.members member).principal = principal ∧ s.configuration.state.participating place = true

def actorCheck (s : System p a) (member : Fin a) (place : Fin p) (principal : Nat) : Bool :=
  decide (s.configuration.state.realm = s.view.realm) && (s.view.members member).enabled &&
  decide ((s.view.members member).principal = principal) && s.configuration.state.participating place

theorem actor_exact (s : System p a) (member : Fin a) (place : Fin p) (principal : Nat) :
    actorCheck s member place principal = true ↔ CurrentActor s member place principal := by
  simp [actorCheck,CurrentActor,and_assoc]

def revalidate (s : System p a) (ctx : Context) (e : Evidence) : Bool :=
  let policy := s.controlPolicy ctx.claimScope.action
  decide (e.context = ctx) && decide (e.policy = policy.id) && decide (e.version = policy.version) &&
    checkWitness s.view.authority ctx.claimScope policy.label policy.expression e.witness

def Allowed (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat) (raw : Raw) : Prop :=
  CurrentActor s member place principal ∧
  Authorized s.view.authority (current s member place principal id raw).claimScope
    (s.controlPolicy (action raw)).label (s.controlPolicy (action raw)).expression

def check (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) : Bool :=
  actorCheck s member place principal && revalidate s (current s member place principal id raw) e

def useId (s : System p a) (principal id : Nat) : UseId :=
  ⟨s.configuration.state.realm,principal,id⟩

def after (s : System p a) (cfg : Config p) (principal id : Nat) : System p a :=
  {s with configuration := cfg,serial := s.serial+1,used := useId s principal id :: s.used}

theorem check_sound (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) (checked : check s member place principal id raw e = true) :
    Allowed s member place principal id raw ∧ e.context = current s member place principal id raw := by
  simp only [check,Bool.and_eq_true,actor_exact,revalidate,decide_eq_true_eq] at checked
  exact ⟨⟨checked.1,checkWitness_sound _ _ _ _ _ checked.2.2⟩,checked.2.1.1.1⟩

def authorize (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat) (raw : Raw) : Option Evidence :=
  let ctx := current s member place principal id raw
  let policy := s.controlPolicy (action raw)
  if actorCheck s member place principal then
    (produce s.view.authority ctx.claimScope policy.label policy.expression).map
      fun w => ⟨ctx,policy.id,policy.version,w⟩
  else none

theorem authorize_checked (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) (created : authorize s member place principal id raw = some e) :
    check s member place principal id raw e = true := by
  unfold authorize at created
  split at created
  · rename_i actor
    cases hp : produce s.view.authority (current s member place principal id raw).claimScope
        (s.controlPolicy (action raw)).label (s.controlPolicy (action raw)).expression with
    | none => simp [hp] at created
    | some witness =>
        simp only [hp,Option.map_some,Option.some.injEq] at created
        subst e
        simp [check,actor,revalidate,current]
        exact produce_checked _ _ _ _ _ hp
  · cases created

theorem authorize_complete (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (allowed : Allowed s member place principal id raw) :
    ∃ e, authorize s member place principal id raw = some e := by
  obtain ⟨w,hw⟩ := produce_complete _ _ _ _ allowed.2
  exact ⟨⟨current s member place principal id raw,(s.controlPolicy (action raw)).id,
    (s.controlPolicy (action raw)).version,w⟩,
    by simp [authorize,(actor_exact _ _ _ _).mpr allowed.1,hw]⟩

def commit (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) : Option (System p a × Option Nat) := do
  if s.used.contains (useId s principal id) || !check s member place principal id raw e then none else do
    let (cfg,created) ← CompositionCore.run s.configuration raw
    return (after s cfg principal id,created)

theorem commit_parts (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) (next : System p a × Option Nat)
    (accepted : commit s member place principal id raw e = some next) :
    check s member place principal id raw e = true ∧
    ∃ cfg created, CompositionCore.run s.configuration raw = some (cfg,created) ∧
      next = (after s cfg principal id,created) := by
  unfold commit at accepted
  split at accepted
  · cases accepted
  · rename_i h
    have checked : check s member place principal id raw e = true := by
      have parts : s.used.contains (useId s principal id) = false ∧
          check s member place principal id raw e = true := by simpa using h
      exact parts.2
    cases hr : CompositionCore.run s.configuration raw with
    | none => simp [hr] at accepted
    | some result =>
        obtain ⟨cfg,created⟩ := result
        simp only [hr] at accepted
        exact ⟨checked,cfg,created,rfl,(Option.some.inj accepted).symm⟩

theorem commit_fresh (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) (next : System p a × Option Nat)
    (accepted : commit s member place principal id raw e = some next) :
    useId s principal id ∉ s.used := by
  unfold commit at accepted
  split at accepted
  · cases accepted
  · rename_i h
    have parts : useId s principal id ∉ s.used ∧ check s member place principal id raw e = true := by
      simpa using h
    exact parts.1

theorem commit_preserves (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) (next : System p a × Option Nat)
    (valid : Invariant s) (accepted : commit s member place principal id raw e = some next) :
    Invariant next.1 ∧ Allowed s member place principal id raw ∧ next.1.serial = s.serial+1 ∧
      next.1.view = s.view := by
  have fresh := commit_fresh s member place principal id raw e next accepted
  obtain ⟨hc,cfg,created,hr,rfl⟩ := commit_parts s member place principal id raw e next accepted
  exact ⟨⟨run_preserves s.configuration raw (cfg,created) valid.1 hr,
    List.nodup_cons.mpr ⟨fresh,valid.2⟩⟩,
    (check_sound s member place principal id raw e hc).1,rfl,rfl⟩

theorem changed_payload_rejected (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) (changed : e.context.payload ≠ raw) :
    check s member place principal id raw e = false := by
  have different : e.context ≠ current s member place principal id raw := by
    intro eq; exact changed (congrArg Context.payload eq)
  simp [check,revalidate,different]

theorem changed_cut_rejected (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) (changed : e.context.cut ≠ s.serial) :
    check s member place principal id raw e = false := by
  have different : e.context ≠ current s member place principal id raw := by
    intro eq; exact changed (congrArg Context.cut eq)
  simp [check,revalidate,different]

theorem commit_no_double_apply (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) (next : System p a × Option Nat)
    (accepted : commit s member place principal id raw e = some next) :
    commit next.1 member place principal id raw e = none := by
  obtain ⟨hc,cfg,created,hr,rfl⟩ := commit_parts s member place principal id raw e next accepted
  have bound := congrArg Context.cut (check_sound s member place principal id raw e hc).2
  have stale : e.context.cut ≠ s.serial+1 := by change e.context.cut = s.serial at bound; omega
  have rejected := changed_cut_rejected (after s cfg principal id)
    member place principal id raw e stale
  simp [commit,rejected]

-- Even a newly produced witness, another command or another actor locus cannot
-- make an already committed realm/principal/request identifier new again.
theorem committed_request_rejected (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) (e : Evidence) (next : System p a × Option Nat)
    (accepted : commit s member place principal id raw e = some next)
    (otherMember : Fin a) (otherPlace : Fin p) (other : Raw) (fresh : Evidence) :
    commit next.1 otherMember otherPlace principal id other fresh = none := by
  obtain ⟨hc,cfg,created,hr,rfl⟩ := commit_parts s member place principal id raw e next accepted
  have realm := run_realm s.configuration raw (cfg,created) hr
  have same : useId (after s cfg principal id) principal id = useId s principal id := by
    simp only [useId,after]
    rw [realm]
  have seen : (after s cfg principal id).used.contains (useId (after s cfg principal id) principal id) = true := by
    rw [same]
    simp [after]
  simp only [commit,seen,Bool.true_or,ite_true]

-- Fresh source execution is a separate entry from submitting a saved permit.
-- Id allocation belongs to the caller's linear execution state, not source.
def perform (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : Raw) : Option (System p a × Option Nat) := do
  let e ← authorize s member place principal id raw
  commit s member place principal id raw e

theorem perform_complete (s : System p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (c : Command s.configuration.definitions p s.configuration.count)
    (valid : Invariant s) (structural : CompositionCore.Allowed s.configuration.state c)
    (auth : Allowed s member place principal id (erase c)) (fresh : useId s principal id ∉ s.used) :
    ∃ next, perform s member place principal id (erase c) = some next := by
  obtain ⟨e,he⟩ := authorize_complete s member place principal id (erase c) auth
  have checked := authorize_checked s member place principal id (erase c) e he
  have core := (CompositionCore.apply_exact s.configuration.state c
    (outcome s.configuration.state c) valid.1).mpr ⟨structural,rfl⟩
  have run : CompositionCore.run s.configuration (erase c) = some (outcome s.configuration.state c) := by
    simp [CompositionCore.run,elaborate_roundtrip,core]
  refine ⟨(after s (outcome s.configuration.state c).1 principal id,(outcome s.configuration.state c).2),?_⟩
  simp [perform,he,commit,checked,fresh,run]

namespace Controls
def claim : Claim :=
  ⟨9,4,0,7,0,1,91,22,[1,2,3,4,6,7,8],[0,1,2],0⟩
def policy : Policy := ⟨21,1,0,.leaf ⟨4,22⟩⟩
def view : AuthorityView 1 :=
  ⟨91,0,fun _ => ⟨7,1,0,true⟩,⟨[claim],[],fun issuer => if issuer = 4 then some 0 else none⟩,
    fun _ _ => policy⟩
def initial : System 3 1 :=
  ⟨config (⟨91,fun _ => 0,Fin.elim0,Fin.elim0,Fin.elim0,fun _ => true⟩ : State 0 3 0),view,
    fun _ => policy,0,[]⟩
def registerCode := Raw.register InstancePrograms.Controls.original none
def instantiateCode := Raw.instantiate 0 7 [0,1,2] none .top
def registerRun := perform initial 0 0 7 0 registerCode
#guard registerRun.isSome
#guard (registerRun.map fun r => r.1.configuration.definitions) = some 1
#guard (registerRun.map Prod.snd) = some (some 0)
def instantiateRun := registerRun.bind fun r => perform r.1 0 0 7 1 instantiateCode
#guard instantiateRun.isSome
#guard (instantiateRun.map fun r => r.1.configuration.count) = some 1
def addedRun := instantiateRun.bind fun r => perform r.1 0 0 7 2 (.instantiate 0 7 [0,2] none .top)
#guard (addedRun.map fun r => r.1.configuration.count) = some 2
def retiredRun := addedRun.bind fun r => perform r.1 0 0 7 3 (.retire 1)
#guard retiredRun.isSome
-- A fresh authorization cannot reuse the same request id after commit.
#guard (registerRun.bind fun r => perform r.1 0 0 7 0 registerCode).isNone
#guard (perform initial 0 0 8 0 registerCode).isNone
#guard (perform {initial with view := {view with authority := emptyAuthority}} 0 0 7 0 registerCode).isNone
#guard (perform {initial with view := {view with authority := {view.authority with revoked := [9]}}}
  0 0 7 0 registerCode).isNone
#guard (perform {initial with view := {view with realm := 92}} 0 0 7 0 registerCode).isNone
#guard (authorize initial 0 0 7 0 registerCode).isSome
#guard ((authorize initial 0 0 7 0 registerCode).bind fun e =>
  commit initial 0 0 7 0 (.register InstancePrograms.Controls.replacement none) e).isNone
end Controls

-- Observed issuer-head changes are environment inputs, not powers produced by
-- typing. W4 must authenticate/order those inputs; no raw update is exported to
-- source. The old view is retained in history outside this W3 in-memory cut.
def installAuthorityHead (s : System p a) (next : AuthorityView a) : System p a :=
  {s with view := next,serial := s.serial+1}
theorem head_preserves (s : System p a) (next : AuthorityView a) (valid : Invariant s) :
    Invariant (installAuthorityHead s next) := valid

#print axioms actor_exact
#print axioms check_sound
#print axioms authorize_checked
#print axioms authorize_complete
#print axioms commit_preserves
#print axioms changed_payload_rejected
#print axioms commit_no_double_apply
#print axioms committed_request_rejected
#print axioms perform_complete
#print axioms head_preserves
end MirroreaProofFirst.ManagementEntry
