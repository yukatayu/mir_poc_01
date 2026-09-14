import MirroreaProofFirstSupportTable
import MirroreaProofFirstGraphTable
import MirroreaProofFirstOwnerPayload

namespace MirroreaProofFirst.OwnerTableEvaluator
open CurrentUse InstanceState WorldProjection InvocationBoundary

-- Preserve the separate definition-version and instance-parent DAG checks.
-- Only reachability evaluation changes; live cross-kind support remains its
-- own least-fixed-point check below, and may contain grounded cycles.
def checkState (s : State d p n) : Bool :=
  (List.finRange d).all (fun k => InstancePrograms.check (s.definitions k)) &&
  GraphTable.checkAcyclic (fun a b => decide (s.predecessors a = some b)) &&
  (List.finRange n).all (fun k => InstancePrograms.refinementCheck (s.instances k).interface
    (s.definitions (s.instances k).definition).contract) &&
  (List.finRange n).all (fun k => decide ((s.instances k).placements ≠ [])) &&
  GraphTable.checkAcyclic (fun a b => decide ((s.instances a).parent = some b))

theorem checkState_same (s : State d p n) : checkState s = OwnerValidity.checkState s := by
  simp only [checkState,OwnerValidity.checkState,GraphTable.check_same]

def validity (image : OwnerImage.Image p a) : Bool :=
  checkState (OwnerImage.restore image).configuration.state

theorem validity_same (image : OwnerImage.Image p a) : validity image = OwnerValidity.check image := by
  exact checkState_same _

def checkHandle (available : Fin n → Bool) (s : World n) (kind : RecordKind) (h : Handle n) : Bool :=
  decide (h.instanceId = s.instanceId) && decide (h.identity = (s.records h.key).identity) &&
  decide (h.identity.kind = kind) && available h.key

-- A single finite table is shared by the four original handle checks. All
-- exact request/evidence/auth-context conditions and their order are retained.
def checkUse (s : World n) (u : UseRequest n) (e : Evidence) : Bool :=
  let table := SupportTable.rounds s.support n
  let available := fun k : Fin n => table[k.val]
  checkHandle available s .member u.member && checkHandle available s .locus u.locus &&
  checkHandle available s .module u.moduleHandle && checkHandle available s .operation u.operation &&
  decide ((s.records u.member.key).principal = u.principal) &&
  decide ((s.records u.operation.key).home = u.locus.key) &&
  decide ((s.records u.operation.key).moduleKey = u.moduleHandle.key) &&
  revalidate s.authority (s.policies u.operation.key) (currentContext s u) e

theorem use_exact (s : World n) (u : UseRequest n) (e : Evidence) :
    checkUse s u e = CurrentUse.checkUse s u e := by
  simp [checkUse,checkHandle,SupportTable.rounds_exact,CurrentUse.checkUse,CurrentUse.checkHandle,Support.snapshotLive]

def checkAt (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (member : Fin a) (key : Fin n) (place : Fin p) : Bool :=
  decide (t.member = member.val ∧ t.key = key.val ∧ t.place = place.val) &&
  decide (t.arithmeticProfile = 1 ∧ t.contractTheoryVersion = 1) &&
  decide (t.definition = s.definitions (s.instances key).definition) &&
  (s.instances key).interface.inputs.contains t.argument &&
  checkUse (world s v) (savedRequest t member key place) t.evidence

theorem checkAt_exact (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (member : Fin a) (key : Fin n) (place : Fin p) :
    checkAt s v t member key place = InvocationBoundary.checkAt s v t member key place := by
  simp [checkAt,InvocationBoundary.checkAt,use_exact]

def check (input : OwnerProjection.Input p a) (assigned : Fin p) (t : Ticket) : Bool :=
  decide (t.place = assigned.val) &&
    match CompositionCore.index a t.member,CompositionCore.index input.configuration.count t.key,
        CompositionCore.index p t.place with
    | some member,some key,some place => checkAt input.configuration.state input.view t member key place
    | _,_,_ => false

theorem check_exact (input : OwnerProjection.Input p a) (assigned : Fin p) (t : Ticket) :
    check input assigned t = OwnerProjection.check input assigned t := by
  simp only [check,OwnerProjection.check,InvocationBoundary.check]
  split <;> simp_all [checkAt_exact]

def run (assigned : OwnerEvaluator.Assignment p) (image : OwnerImage.Image p a)
    (ticket : Ticket) : OwnerEvaluator.Result :=
  if image.state.realm ≠ assigned.realm ∨ image.view.realm ≠ assigned.realm then .rejected .wrongRealm
  else if !validity image then .rejected .invalidConfiguration
  else if !check (OwnerImage.restore image) assigned.place ticket then .rejected .unauthorized
  else match InvocationBoundary.execute ticket with
    | none => .rejected .executionFailure
    | some result => .value result

theorem run_exact (assigned : OwnerEvaluator.Assignment p) (image : OwnerImage.Image p a) (ticket : Ticket) :
    run assigned image ticket = OwnerEvaluator.run assigned image ticket := by
  simp only [run,OwnerEvaluator.run,check_exact,validity_same]
  rfl

def payload (assigned : OwnerEvaluator.Assignment p) (a : Nat) (bytes : List UInt8) : Sum Nat OwnerEvaluator.Result :=
  if bytes.length > OwnerPayload.maxBytes then .inl 1
  else if !OwnerPayload.digitCheck OwnerPayload.maxDigits 0 bytes then .inl 3
  else if !OwnerPayload.textCheck OwnerPayload.maxTextScalars bytes then .inl 3
  else match OwnerPacketCodec.decodeAt (OwnerFullCodec.request p a) OwnerPayload.maxFuel bytes with
  | none => .inl 0
  | some request =>
      if request.1.definitions > OwnerPayload.maxRecords || request.1.count > OwnerPayload.maxRecords then .inl 2
      else .inr (run assigned request.1 request.2)

theorem payload_exact (assigned : OwnerEvaluator.Assignment p) (a : Nat) (bytes : List UInt8) :
    payload assigned a bytes = OwnerPayload.payload assigned a bytes := by
  simp only [payload,OwnerPayload.payload,run_exact]
  rfl

#print axioms use_exact
#print axioms checkState_same
#print axioms validity_same
#print axioms checkAt_exact
#print axioms check_exact
#print axioms run_exact
#print axioms payload_exact
end MirroreaProofFirst.OwnerTableEvaluator
