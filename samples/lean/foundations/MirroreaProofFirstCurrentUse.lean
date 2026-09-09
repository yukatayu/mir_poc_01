import MirroreaProofFirstSupport

/- Task-local candidate. No Canon, issuer trust or production interface is
   selected here. Issued records and current state are explicit inputs.
   Structural equality is not a cryptographic hash assumption. -/
namespace MirroreaProofFirst.CurrentUse

inductive Scalar where
  | integer (value : Int)
  | boolean (value : Bool)
  deriving DecidableEq, Repr

structure Context where
  principal : Nat
  member : Nat
  memberIncarnation : Nat
  locus : Nat
  locusIncarnation : Nat
  moduleKey : Nat
  moduleIncarnation : Nat
  action : Nat
  target : Nat
  targetIncarnation : Nat
  targetRevision : Nat
  instanceId : Nat
  request : Nat
  arguments : List Scalar
  code : Nat
  contract : Nat
  generation : Nat
  deriving DecidableEq, Repr

structure Claim where
  id : Nat
  issuer : Nat
  epoch : Nat
  principal : Nat
  member : Nat
  memberIncarnation : Nat
  instanceId : Nat
  predicate : Nat
  actions : List Nat
  targets : List Nat
  label : Nat
  deriving DecidableEq, Repr

structure Need where
  issuer : Nat
  predicate : Nat
  deriving DecidableEq, Repr

/- No empty conjunction or Top authority branch. Explicit disjunction does
   not change conjunction into an implicit any-of decision. -/
inductive PolicyExpr where
  | leaf (need : Need)
  | both (left right : PolicyExpr)
  | either (left right : PolicyExpr)
  deriving DecidableEq, Repr

structure Policy where
  id : Nat
  version : Nat
  label : Nat
  expression : PolicyExpr
  deriving DecidableEq, Repr

structure Authority where
  issued : List Claim
  revoked : List Nat
  epochs : Nat → Option Nat

/- Independent declarative leaf judgment: an actual issued record, scoped to
   this subject/member/action/target and current issuer generation. -/
def ValidClaim (a : Authority) (κ : Context) (label : Nat) (need : Need)
    (c : Claim) : Prop :=
  c ∈ a.issued ∧ c.id ∉ a.revoked ∧ a.epochs c.issuer = some c.epoch ∧
  c.issuer = need.issuer ∧ c.predicate = need.predicate ∧
  c.principal = κ.principal ∧ c.member = κ.member ∧
  c.memberIncarnation = κ.memberIncarnation ∧ c.instanceId = κ.instanceId ∧
  κ.action ∈ c.actions ∧ κ.target ∈ c.targets ∧ c.label ≤ label

def checkClaim (a : Authority) (κ : Context) (label : Nat) (need : Need)
    (c : Claim) : Bool :=
  a.issued.contains c && !a.revoked.contains c.id &&
  decide (a.epochs c.issuer = some c.epoch) && decide (c.issuer = need.issuer) &&
  decide (c.predicate = need.predicate) && decide (c.principal = κ.principal) &&
  decide (c.member = κ.member) && c.actions.contains κ.action &&
  decide (c.memberIncarnation = κ.memberIncarnation) && decide (c.instanceId = κ.instanceId) &&
  c.targets.contains κ.target && decide (c.label ≤ label)

theorem checkClaim_exact (a : Authority) (κ : Context) (label : Nat)
    (need : Need) (c : Claim) :
    checkClaim a κ label need c = true ↔ ValidClaim a κ label need c := by
  simp [checkClaim, ValidClaim, and_assoc, and_left_comm, and_comm]

inductive Authorized (a : Authority) (κ : Context) (label : Nat) : PolicyExpr → Prop where
  | leaf : ValidClaim a κ label need c → Authorized a κ label (.leaf need)
  | both : Authorized a κ label p → Authorized a κ label q →
      Authorized a κ label (.both p q)
  | left : Authorized a κ label p → Authorized a κ label (.either p q)
  | right : Authorized a κ label q → Authorized a κ label (.either p q)

inductive Witness where
  | leaf (claim : Claim)
  | both (left right : Witness)
  | left (witness : Witness)
  | right (witness : Witness)
  deriving DecidableEq, Repr

/- Checker consumes the submitted tree. It does not run the producer or swap
   a failed old branch for a newly authorized different branch. -/
def checkWitness (a : Authority) (κ : Context) (label : Nat) :
    PolicyExpr → Witness → Bool
  | .leaf n, .leaf c => checkClaim a κ label n c
  | .both p q, .both u v => checkWitness a κ label p u && checkWitness a κ label q v
  | .either p _, .left u => checkWitness a κ label p u
  | .either _ q, .right v => checkWitness a κ label q v
  | _, _ => false

theorem checkWitness_sound (a : Authority) (κ : Context) (label : Nat)
    (p : PolicyExpr) (w : Witness) :
    checkWitness a κ label p w = true → Authorized a κ label p := by
  induction p generalizing w with
  | leaf n =>
    cases w <;> simp only [checkWitness, Bool.false_eq_true, false_implies]
    exact fun h => .leaf ((checkClaim_exact a κ label n _).mp h)
  | both p q hp hq =>
    cases w <;> simp only [checkWitness, Bool.false_eq_true, false_implies]
    intro h
    simp only [Bool.and_eq_true] at h
    exact .both (hp _ h.1) (hq _ h.2)
  | either p q hp hq =>
    cases w <;> simp only [checkWitness, Bool.false_eq_true, false_implies]
    · exact fun h => .left (hp _ h)
    · exact fun h => .right (hq _ h)

def chooseClaim (test : Claim → Bool) : List Claim → Option Claim
  | [] => none
  | c :: cs => if test c then some c else chooseClaim test cs

theorem chooseClaim_sound (test : Claim → Bool) (cs : List Claim) (c : Claim) :
    chooseClaim test cs = some c → c ∈ cs ∧ test c = true := by
  induction cs with
  | nil => simp [chooseClaim]
  | cons d ds ih =>
    simp only [chooseClaim]
    split
    · intro h; cases Option.some.inj h
      exact ⟨by simp, by assumption⟩
    · intro h
      obtain ⟨hm, ht⟩ := ih h
      exact ⟨by simp [hm], ht⟩

theorem chooseClaim_complete (test : Claim → Bool) (cs : List Claim) :
    (∃ c ∈ cs, test c = true) → ∃ c, chooseClaim test cs = some c := by
  induction cs with
  | nil => simp
  | cons d ds ih =>
    intro h
    simp only [chooseClaim]
    split
    · exact ⟨d, rfl⟩
    · rename_i hd
      apply ih
      obtain ⟨c, hc, ht⟩ := h
      rcases List.mem_cons.mp hc with eq | mem
      · subst c; contradiction
      · exact ⟨c, mem, ht⟩

def produce (a : Authority) (κ : Context) (label : Nat) : PolicyExpr → Option Witness
  | .leaf n => (chooseClaim (checkClaim a κ label n) a.issued).map Witness.leaf
  | .both p q =>
      match produce a κ label p, produce a κ label q with
      | some u, some v => some (.both u v)
      | _, _ => none
  | .either p q =>
      match produce a κ label p with
      | some u => some (.left u)
      | none => (produce a κ label q).map Witness.right

theorem produce_checked (a : Authority) (κ : Context) (label : Nat)
    (p : PolicyExpr) (w : Witness) :
    produce a κ label p = some w → checkWitness a κ label p w = true := by
  induction p generalizing w with
  | leaf n =>
    simp only [produce]
    cases h : chooseClaim (checkClaim a κ label n) a.issued with
    | none => simp
    | some c =>
      simp only [Option.map_some, Option.some.injEq]
      intro eq; subst w
      exact (chooseClaim_sound _ _ _ h).2
  | both p q hp hq =>
    simp only [produce]
    cases hu : produce a κ label p <;> cases hv : produce a κ label q <;>
      simp
    intro eq; subst w
    simp [checkWitness, hp _ hu, hq _ hv]
  | either p q hp hq =>
    simp only [produce]
    cases hu : produce a κ label p with
    | some u =>
      simp only [Option.some.injEq]
      intro eq; subst w; exact hp _ hu
    | none =>
      cases hv : produce a κ label q with
      | none => simp
      | some v =>
        simp only [Option.map_some, Option.some.injEq]
        intro eq; subst w; exact hq _ hv

theorem produce_complete (a : Authority) (κ : Context) (label : Nat)
    (p : PolicyExpr) (h : Authorized a κ label p) :
    ∃ w, produce a κ label p = some w := by
  induction h with
  | @leaf n c hc =>
    obtain ⟨d, hd⟩ := chooseClaim_complete (checkClaim a κ label n) a.issued
      ⟨c, hc.1, (checkClaim_exact a κ label n c).mpr hc⟩
    exact ⟨.leaf d, by simp [produce, hd]⟩
  | both _ _ hp hq =>
    obtain ⟨u, hu⟩ := hp
    obtain ⟨v, hv⟩ := hq
    exact ⟨.both u v, by simp [produce, hu, hv]⟩
  | left _ hp =>
    obtain ⟨u, hu⟩ := hp
    exact ⟨.left u, by simp [produce, hu]⟩
  | right _ hq =>
    obtain ⟨v, hv⟩ := hq
    simp only [produce]
    split
    · exact ⟨_, rfl⟩
    · exact ⟨.right v, by simp [hv]⟩

structure Evidence where
  policy : Nat
  version : Nat
  context : Context
  witness : Witness
  deriving DecidableEq, Repr

def revalidate (a : Authority) (p : Policy) (current : Context) (old : Evidence) : Bool :=
  decide (old.policy = p.id) && decide (old.version = p.version) &&
  decide (old.context = current) && checkWitness a current p.label p.expression old.witness

theorem revalidate_sound (a : Authority) (p : Policy) (current : Context) (old : Evidence)
    (h : revalidate a p current old = true) :
    old.policy = p.id ∧ old.version = p.version ∧ old.context = current ∧
    Authorized a current p.label p.expression := by
  simp only [revalidate, Bool.and_eq_true, decide_eq_true_eq] at h
  exact ⟨h.1.1.1, h.1.1.2, h.1.2, checkWitness_sound _ _ _ _ _ h.2⟩

theorem context_change_rejected (a : Authority) (p : Policy) (current : Context)
    (old : Evidence) (different : old.context ≠ current) :
    revalidate a p current old = false := by
  simp [revalidate, different]

theorem authorization_iff_produced (a : Authority) (κ : Context) (label : Nat)
    (p : PolicyExpr) : Authorized a κ label p ↔ ∃ w, produce a κ label p = some w := by
  constructor
  · exact produce_complete a κ label p
  · rintro ⟨w, hw⟩
    exact checkWitness_sound a κ label p w (produce_checked a κ label p w hw)

theorem no_claim_no_authorization (a : Authority) (κ : Context) (label : Nat)
    (p : PolicyExpr) (empty : a.issued = []) : ¬ Authorized a κ label p := by
  intro h
  induction h with
  | leaf hc => simp [ValidClaim, empty] at hc
  | both _ _ ih _ => exact ih
  | left _ ih => exact ih
  | right _ ih => exact ih

/- Current-use reference boundary. Finite indices are internal coordinates,
   not external identities. Record kinds, incarnations and revisions remain
   separate from the policy generation and the instance identity. -/
inductive RecordKind where
  | member | locus | module | operation
  deriving DecidableEq, Repr

structure RecordIdentity where
  kind : RecordKind
  incarnation : Nat
  revision : Nat
  deriving DecidableEq, Repr

structure Record (n : Nat) where
  identity : RecordIdentity
  principal : Nat
  home : Fin n
  moduleKey : Fin n
  action : Nat
  code : Nat
  contract : Nat

structure Handle (n : Nat) where
  instanceId : Nat
  key : Fin n
  identity : RecordIdentity
  deriving DecidableEq, Repr

structure World (n : Nat) where
  instanceId : Nat
  generation : Nat
  support : Support.Snapshot n
  records : Fin n → Record n
  authority : Authority
  policies : Fin n → Policy

def CurrentHandle (s : World n) (kind : RecordKind) (h : Handle n) : Prop :=
  h.instanceId = s.instanceId ∧ h.identity = (s.records h.key).identity ∧
  h.identity.kind = kind ∧
  Support.Grounded s.support.forms (fun k => s.support.eligible k = true) h.key

def checkHandle (s : World n) (kind : RecordKind) (h : Handle n) : Bool :=
  decide (h.instanceId = s.instanceId) && decide (h.identity = (s.records h.key).identity) &&
  decide (h.identity.kind = kind) && Support.snapshotLive s.support h.key

theorem checkHandle_exact (s : World n) (kind : RecordKind) (h : Handle n) :
    checkHandle s kind h = true ↔ CurrentHandle s kind h := by
  simp only [checkHandle, Bool.and_eq_true, decide_eq_true_eq,
    Support.snapshot_live_exact, CurrentHandle, and_assoc]

structure UseRequest (n : Nat) where
  principal : Nat
  member : Handle n
  locus : Handle n
  moduleHandle : Handle n
  operation : Handle n
  request : Nat
  arguments : List Scalar

/- Code, contract, instance and generation are reconstructed from the state;
   they are not copied out of the submitted old evidence. The operation's
   module/locus are separately checked against the requested handles. -/
def currentContext (s : World n) (u : UseRequest n) : Context :=
  { principal := u.principal, member := u.member.key.val,
    memberIncarnation := (s.records u.member.key).identity.incarnation,
    locus := u.locus.key.val, locusIncarnation := (s.records u.locus.key).identity.incarnation,
    moduleKey := u.moduleHandle.key.val,
    moduleIncarnation := (s.records u.moduleHandle.key).identity.incarnation,
    action := (s.records u.operation.key).action, target := u.operation.key.val,
    targetIncarnation := (s.records u.operation.key).identity.incarnation,
    targetRevision := (s.records u.operation.key).identity.revision,
    instanceId := s.instanceId, request := u.request, arguments := u.arguments,
    code := (s.records u.operation.key).code,
    contract := (s.records u.operation.key).contract, generation := s.generation }

def CurrentUse (s : World n) (u : UseRequest n) : Prop :=
  CurrentHandle s .member u.member ∧ CurrentHandle s .locus u.locus ∧
  CurrentHandle s .module u.moduleHandle ∧ CurrentHandle s .operation u.operation ∧
  (s.records u.member.key).principal = u.principal ∧
  (s.records u.operation.key).home = u.locus.key ∧
  (s.records u.operation.key).moduleKey = u.moduleHandle.key ∧
  Authorized s.authority (currentContext s u) (s.policies u.operation.key).label
    (s.policies u.operation.key).expression

def checkUse (s : World n) (u : UseRequest n) (e : Evidence) : Bool :=
  checkHandle s .member u.member && checkHandle s .locus u.locus &&
  checkHandle s .module u.moduleHandle && checkHandle s .operation u.operation &&
  decide ((s.records u.member.key).principal = u.principal) &&
  decide ((s.records u.operation.key).home = u.locus.key) &&
  decide ((s.records u.operation.key).moduleKey = u.moduleHandle.key) &&
  revalidate s.authority (s.policies u.operation.key) (currentContext s u) e

theorem checkUse_sound (s : World n) (u : UseRequest n) (e : Evidence)
    (h : checkUse s u e = true) : CurrentUse s u := by
  simp only [checkUse, Bool.and_eq_true, checkHandle_exact, decide_eq_true_eq] at h
  obtain ⟨⟨⟨⟨⟨⟨⟨hm, hl⟩, hmod⟩, hop⟩, hsubject⟩, hhome⟩, hmodule⟩, hauth⟩ := h
  exact ⟨hm, hl, hmod, hop, hsubject, hhome, hmodule,
    (revalidate_sound _ _ _ _ hauth).2.2.2⟩

theorem checkUse_complete (s : World n) (u : UseRequest n) (h : CurrentUse s u) :
    ∃ e, checkUse s u e = true := by
  obtain ⟨hm, hl, hmod, hop, hsubject, hhome, hmodule, hauth⟩ := h
  obtain ⟨w, hw⟩ := produce_complete _ _ _ _ hauth
  refine ⟨⟨(s.policies u.operation.key).id, (s.policies u.operation.key).version,
    currentContext s u, w⟩, ?_⟩
  have checked := produce_checked _ _ _ _ _ hw
  simp [checkUse, (checkHandle_exact s .member u.member).mpr hm,
    (checkHandle_exact s .locus u.locus).mpr hl,
    (checkHandle_exact s .module u.moduleHandle).mpr hmod,
    (checkHandle_exact s .operation u.operation).mpr hop, hsubject, hhome, hmodule,
    revalidate, checked]

theorem retired_locus_rejected (s : World n) (u : UseRequest n) (e : Evidence)
    (retired : s.support.eligible u.locus.key = false) : checkUse s u e = false := by
  cases h : checkUse s u e with
  | false => rfl
  | true =>
    have live := (checkUse_sound s u e h).2.1.2.2.2
    have eligible := live.1
    simp [retired] at eligible

theorem stale_incarnation_rejected (s : World n) (kind : RecordKind) (h : Handle n)
    (stale : h.identity.incarnation ≠ (s.records h.key).identity.incarnation) :
    checkHandle s kind h = false := by
  have ne : h.identity ≠ (s.records h.key).identity := fun eq => stale (congrArg RecordIdentity.incarnation eq)
  simp [checkHandle, ne]

def authorize (a : Authority) (p : Policy) (κ : Context) : Option Evidence :=
  (produce a κ p.label p.expression).map fun w => ⟨p.id, p.version, κ, w⟩

theorem authorize_checked (a : Authority) (p : Policy) (κ : Context) (e : Evidence)
    (h : authorize a p κ = some e) : revalidate a p κ e = true := by
  simp only [authorize] at h
  cases hw : produce a κ p.label p.expression with
  | none => simp [hw] at h
  | some w =>
    simp only [hw, Option.map_some, Option.some.injEq] at h
    subst e
    simp [revalidate, produce_checked _ _ _ _ _ hw]

theorem authorize_use_complete (s : World n) (u : UseRequest n)
    (h : CurrentUse s u) :
    ∃ e, authorize s.authority (s.policies u.operation.key) (currentContext s u) = some e ∧
      checkUse s u e = true := by
  obtain ⟨hm, hl, hmod, hop, hsubject, hhome, hmodule, hauth⟩ := h
  obtain ⟨w, hw⟩ := produce_complete _ _ _ _ hauth
  let e : Evidence := ⟨(s.policies u.operation.key).id,
    (s.policies u.operation.key).version, currentContext s u, w⟩
  have produced : authorize s.authority (s.policies u.operation.key)
      (currentContext s u) = some e := by simp [authorize, hw, e]
  refine ⟨e, produced, ?_⟩
  simp [checkUse, (checkHandle_exact s .member u.member).mpr hm,
    (checkHandle_exact s .locus u.locus).mpr hl,
    (checkHandle_exact s .module u.moduleHandle).mpr hmod,
    (checkHandle_exact s .operation u.operation).mpr hop, hsubject, hhome, hmodule,
    authorize_checked _ _ _ _ produced]

/- This ledger records admission decisions, not execution, state mutation or
   delivery. Unique decision keys do not prove linear execution consumption.
   Its serialized mathematical step still requires a physical mechanism. -/
structure UseId where
  instanceId : Nat
  principal : Nat
  request : Nat
  deriving DecidableEq, Repr

def Context.useId (κ : Context) : UseId := ⟨κ.instanceId, κ.principal, κ.request⟩

structure Decision where
  context : Context
  deriving DecidableEq, Repr

structure Machine (n : Nat) where
  world : World n
  decisions : List Decision

def tryAdmit (m : Machine n) (u : UseRequest n) (e : Evidence) : Machine n :=
  let κ := currentContext m.world u
  if m.decisions.any (fun d => decide (d.context.useId = κ.useId)) then m
  else if checkUse m.world u e then
    { m with decisions := ⟨κ⟩ :: m.decisions }
  else m

def retire (m : Machine n) (k : Fin n) : Machine n :=
  { m with world := { m.world with support :=
      { m.world.support with eligible := fun j => m.world.support.eligible j && decide (j ≠ k) } } }

def revoke (m : Machine n) (claimId : Nat) : Machine n :=
  { m with world := { m.world with authority :=
      { m.world.authority with revoked := claimId :: m.world.authority.revoked } } }

theorem tryAdmit_sound (m : Machine n) (u : UseRequest n) (e : Evidence)
    (changed : (tryAdmit m u e).decisions ≠ m.decisions) :
    CurrentUse m.world u ∧
    (tryAdmit m u e).decisions = ⟨currentContext m.world u⟩ :: m.decisions := by
  by_cases seen : m.decisions.any (fun d => decide
      (d.context.useId = (currentContext m.world u).useId)) = true
  · simp [tryAdmit, seen] at changed
  · by_cases checked : checkUse m.world u e = true
    · exact ⟨checkUse_sound _ _ _ checked, by simp [tryAdmit, seen, checked]⟩
    · simp [tryAdmit, seen, checked] at changed

theorem tryAdmit_does_not_issue (m : Machine n) (u : UseRequest n) (e : Evidence) :
    (tryAdmit m u e).world = m.world := by
  dsimp only [tryAdmit]
  split
  · rfl
  · split <;> rfl

theorem tryAdmit_retains_history (m : Machine n) (u : UseRequest n) (e : Evidence)
    (d : Decision) (old : d ∈ m.decisions) : d ∈ (tryAdmit m u e).decisions := by
  dsimp only [tryAdmit]
  split
  · exact old
  · split
    · exact List.mem_cons_of_mem _ old
    · exact old

theorem duplicate_admission_unchanged (m : Machine n) (u : UseRequest n) (e : Evidence)
    (d : Decision) (present : d ∈ m.decisions)
    (same : d.context.useId = (currentContext m.world u).useId) :
    tryAdmit m u e = m := by
  have seen : m.decisions.any (fun x => decide (x.context.useId = (currentContext m.world u).useId)) = true := by
    simp only [List.any_eq_true, decide_eq_true_eq]
    exact ⟨d, present, same⟩
  simp [tryAdmit, seen]

theorem retire_preserves_history (m : Machine n) (k : Fin n) :
    (retire m k).decisions = m.decisions := rfl

theorem revoke_preserves_history (m : Machine n) (c : Nat) :
    (revoke m c).decisions = m.decisions := rfl

theorem retire_stops_locus_use (m : Machine n) (u : UseRequest n) (e : Evidence) :
    checkUse (retire m u.locus.key).world u e = false := by
  apply retired_locus_rejected
  simp [retire]

def UniqueDecisions (m : Machine n) : Prop :=
  (m.decisions.map fun d => d.context.useId).Nodup

theorem tryAdmit_unique (m : Machine n) (u : UseRequest n) (e : Evidence)
    (unique : UniqueDecisions m) : UniqueDecisions (tryAdmit m u e) := by
  dsimp only [tryAdmit]
  split
  · exact unique
  · rename_i unseen
    split
    · have absent : (currentContext m.world u).useId ∉
          m.decisions.map (fun d => d.context.useId) := by
        intro present
        obtain ⟨d, hd, eq⟩ := List.mem_map.mp present
        apply unseen
        simp only [List.any_eq_true, decide_eq_true_eq]
        exact ⟨d, hd, eq⟩
      simpa [UniqueDecisions, absent] using unique
    · exact unique

inductive Action (n : Nat) where
  | admit (request : UseRequest n) (evidence : Evidence)
  | retire (key : Fin n)
  | revoke (claimId : Nat)

def step (m : Machine n) : Action n → Machine n
  | .admit u e => tryAdmit m u e
  | .retire k => retire m k
  | .revoke c => revoke m c

def run (m : Machine n) : List (Action n) → Machine n
  | [] => m
  | a :: as => run (step m a) as

theorem step_unique (m : Machine n) (a : Action n) (unique : UniqueDecisions m) :
    UniqueDecisions (step m a) := by
  cases a with
  | admit u e => exact tryAdmit_unique m u e unique
  | retire k => exact unique
  | revoke c => exact unique

theorem run_unique (m : Machine n) (as : List (Action n)) (unique : UniqueDecisions m) :
    UniqueDecisions (run m as) := by
  induction as generalizing m with
  | nil => exact unique
  | cons a as ih => exact ih (step m a) (step_unique m a unique)

theorem step_retains_history (m : Machine n) (a : Action n) (d : Decision)
    (old : d ∈ m.decisions) : d ∈ (step m a).decisions := by
  cases a with
  | admit u e => exact tryAdmit_retains_history m u e d old
  | retire k => exact old
  | revoke c => exact old

theorem run_retains_history (m : Machine n) (as : List (Action n)) (d : Decision)
    (old : d ∈ m.decisions) : d ∈ (run m as).decisions := by
  induction as generalizing m with
  | nil => exact old
  | cons a as ih => exact ih (step m a) (step_retains_history m a d old)

/- Historical support certificates remain valid mathematical facts about old
   inputs. They are intentionally not accepted as the current World argument.
   Which operation steps require these four handles is a selected reference
   profile obligation; this does not prove every F0.3/Rust entry uses checkUse. -/

namespace Controls

def claimA : Claim :=
  ⟨1, 10, 0, 3, 0, 1, 7, 11, [5], [3], 0⟩
def claimB : Claim := { claimA with id := 2, issuer := 20, predicate := 22 }
def auth : Authority :=
  ⟨[claimA, claimB], [], fun i => if i = 10 ∨ i = 20 then some 0 else none⟩
def policy : Policy :=
  ⟨4, 0, 0, .both (.leaf ⟨10, 11⟩) (.leaf ⟨20, 22⟩)⟩
def world : World 4 :=
  { instanceId := 7, generation := 0,
    support := ⟨fun _ => .top, fun _ => true⟩,
    records := fun k =>
      { identity := ⟨match k.val with | 0 => .member | 1 => .locus | 2 => .module | _ => .operation, 1, 0⟩,
        principal := 3, home := 1, moduleKey := 2, action := 5, code := 100, contract := 200 },
    authority := auth, policies := fun _ => policy }
def handle (k : Fin 4) : Handle 4 := ⟨7, k, (world.records k).identity⟩
def request : UseRequest 4 :=
  ⟨3, handle 0, handle 1, handle 2, handle 3, 9, [.integer 41]⟩
def evidence : Evidence :=
  ⟨4, 0, currentContext world request, .both (.leaf claimA) (.leaf claimB)⟩

#guard checkUse world request evidence
#guard produce auth (currentContext world request) 0 policy.expression = some evidence.witness
#guard !checkUse { world with authority := { auth with issued := [] } } request evidence
#guard !checkUse { world with authority := { auth with revoked := [2] } } request evidence
#guard !checkUse { world with authority := { auth with epochs := fun _ => some 1 } } request evidence
#guard !checkUse { world with policies := fun _ => { policy with version := 1 } } request evidence
#guard !checkUse { world with generation := 1 } request evidence
#guard !checkUse { world with instanceId := 8 } request evidence
#guard !checkUse world { request with arguments := [.boolean true] } evidence
#guard !checkUse world { request with request := 10 } evidence
#guard !checkUse world { request with principal := 4 } evidence
#guard !checkUse world { request with locus := handle 2 } evidence
#guard !checkUse world request { evidence with witness := .left (.leaf claimA) }

def mixedClaim : Claim := { claimB with principal := 4 }
def mixedAuthority : Authority := { auth with issued := [claimA, mixedClaim] }
#guard produce mixedAuthority (currentContext world request) 0 policy.expression = none
#guard !checkUse { world with authority := mixedAuthority } request
  { evidence with witness := .both (.leaf claimA) (.leaf mixedClaim) }
#guard !checkUse world request
  { evidence with witness := .both (.leaf claimA) (.leaf { claimB with label := 1 }) }

def alternative : PolicyExpr := .either (.leaf ⟨99, 99⟩) (.leaf ⟨20, 22⟩)
#guard produce auth (currentContext world request) 0 alternative = some (.right (.leaf claimB))

def retiredWorld : World 4 :=
  { world with support := { world.support with eligible := fun k => k != 1 } }
#guard Support.checkSnapshot world.support (Support.snapshotLive world.support)
  (Support.snapshotRank world.support)
#guard !checkUse retiredWorld request evidence
-- The old pure certificate still checks; that does not make the current use legal.
#guard Support.checkSnapshot world.support (Support.snapshotLive world.support)
  (Support.snapshotRank world.support)

def rejoinedWorld : World 4 :=
  { world with records := fun k => if k = 0 then
      { world.records k with identity := ⟨.member, 2, 0⟩ } else world.records k }
def freshMemberRequest : UseRequest 4 :=
  { request with member := ⟨7, 0, (rejoinedWorld.records 0).identity⟩ }
#guard !checkUse rejoinedWorld request evidence
#guard !checkUse rejoinedWorld freshMemberRequest evidence
-- Even new evidence cannot reuse old-incarnation claims.
#guard produce auth (currentContext rejoinedWorld freshMemberRequest) 0 policy.expression = none

def newClaimA : Claim := { claimA with id := 3, memberIncarnation := 2 }
def newClaimB : Claim := { claimB with id := 4, memberIncarnation := 2 }
def freshWorld : World 4 :=
  { rejoinedWorld with authority := { auth with issued := [newClaimA, newClaimB] } }
def freshEvidence : Evidence :=
  ⟨4, 0, currentContext freshWorld freshMemberRequest, .both (.leaf newClaimA) (.leaf newClaimB)⟩
#guard checkUse freshWorld freshMemberRequest freshEvidence

def recodedWorld : World 4 :=
  { world with records := fun k => if k = 3 then
      { world.records k with code := 101 } else world.records k }
#guard !checkUse recodedWorld request evidence

def machine : Machine 4 := ⟨world, []⟩
#guard (tryAdmit machine request evidence).decisions.length = 1
#guard (tryAdmit (tryAdmit machine request evidence) request evidence).decisions.length = 1
#guard (retire (tryAdmit machine request evidence) 1).decisions.length = 1
#guard (tryAdmit (retire machine 1) request evidence).decisions.length = 0
#guard (tryAdmit (revoke machine 2) request evidence).decisions.length = 0

end Controls

#print axioms checkClaim_exact
#print axioms checkWitness_sound
#print axioms chooseClaim_sound
#print axioms chooseClaim_complete
#print axioms produce_checked
#print axioms produce_complete
#print axioms revalidate_sound
#print axioms context_change_rejected
#print axioms authorization_iff_produced
#print axioms no_claim_no_authorization
#print axioms checkHandle_exact
#print axioms checkUse_sound
#print axioms checkUse_complete
#print axioms retired_locus_rejected
#print axioms stale_incarnation_rejected
#print axioms authorize_checked
#print axioms authorize_use_complete
#print axioms tryAdmit_sound
#print axioms tryAdmit_does_not_issue
#print axioms tryAdmit_retains_history
#print axioms duplicate_admission_unchanged
#print axioms retire_preserves_history
#print axioms revoke_preserves_history
#print axioms retire_stops_locus_use
#print axioms tryAdmit_unique
#print axioms step_unique
#print axioms run_unique
#print axioms step_retains_history
#print axioms run_retains_history

end MirroreaProofFirst.CurrentUse
