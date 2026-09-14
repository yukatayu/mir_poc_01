import MirroreaProofFirstOwnerProjection
import MirroreaProofFirstAuthorityImage
import MirroreaProofFirstPublicationUse

namespace MirroreaProofFirst.OwnerImage
open InstanceState InstancePrograms WorldProjection InvocationBoundary CurrentUse

-- Function-free finite candidate for the current unary owner checker. All
-- structural support records are retained, so no unproved footprint truncation
-- or supplied liveness bit substitutes for the least fixed point computation.
-- This is not a wire codec, disclosure permit or authentic/current publication.
structure StateImage (d p n : Nat) where
  realm : Nat
  incarnations : Vector Nat p
  definitions : Vector Definition d
  predecessors : Vector (Option (Fin d)) d
  instances : Vector (Instance d p n) n
  participating : Vector Bool p

def captureState (s : State d p n) : StateImage d p n :=
  ⟨s.realm,Vector.ofFn s.placeIncarnation,Vector.ofFn s.definitions,
    Vector.ofFn s.predecessors,Vector.ofFn s.instances,Vector.ofFn s.participating⟩

def restoreState (s : StateImage d p n) : State d p n :=
  ⟨s.realm,(fun k => s.incarnations[k.val]),(fun k => s.definitions[k.val]),
    (fun k => s.predecessors[k.val]),(fun k => s.instances[k.val]),
    (fun k => s.participating[k.val])⟩

theorem state_roundtrip (s : State d p n) : restoreState (captureState s) = s := by
  cases s
  simp [captureState,restoreState]

structure ViewImage (a p n : Nat) where
  realm : Nat
  generation : Nat
  members : Vector Member a
  authority : AuthorityImage.Image
  policies : Vector (Vector Policy p) n

def captureView (v : AuthorityView a) : ViewImage a p n :=
  ⟨v.realm,v.generation,Vector.ofFn v.members,AuthorityImage.capture v.authority,
    Vector.ofFn fun k : Fin n => Vector.ofFn fun place : Fin p => v.policies k.val place.val⟩

def restoreView (v : ViewImage a p n) : AuthorityView a :=
  ⟨v.realm,v.generation,(fun k => v.members[k.val]),AuthorityImage.restore v.authority,
    fun key place => match CompositionCore.index n key,CompositionCore.index p place with
      | some k,some l => v.policies[k.val][l.val]
      | _,_ => inactivePolicy⟩

theorem members_roundtrip (v : AuthorityView a) (key : Fin a) :
    (restoreView (captureView (p:=p) (n:=n) v)).members key = v.members key := by
  simp [restoreView,captureView]

theorem policy_roundtrip (v : AuthorityView a) (key : Fin n) (place : Fin p) :
    (restoreView (captureView (p:=p) (n:=n) v)).policies key.val place.val = v.policies key.val place.val := by
  simp [restoreView,captureView,CompositionCore.index_roundtrip]

theorem world_records_roundtrip (s : State d p n) (v : AuthorityView a) :
    (world s (restoreView (captureView (p:=p) (n:=n) v))).records = (world s v).records := by
  funext key
  simp only [world]
  cases decode key <;> simp [slotRecord,members_roundtrip]

theorem support_roundtrip (s : State d p n) (v : AuthorityView a) :
    support s (restoreView (captureView (p:=p) (n:=n) v)) = support s v := by
  unfold support
  congr 1
  funext key
  cases decode key <;> simp [slotEligible,restoreView,captureView]

theorem world_policies_roundtrip (s : State d p n) (v : AuthorityView a) :
    (world s (restoreView (captureView (p:=p) (n:=n) v))).policies = (world s v).policies := by
  funext key
  simp only [world]
  cases decode key <;> simp [policy_roundtrip]

-- Exact original submitted-witness revalidation, not replacement by an
-- existentially valid witness. Unissued epochs cannot influence this result.
theorem revalidate_roundtrip (authority : Authority) (policy : Policy) (ctx : CurrentUse.Context)
    (evidence : Evidence) :
    revalidate (AuthorityImage.restore (AuthorityImage.capture authority)) policy ctx evidence =
      revalidate authority policy ctx evidence := by
  simp [revalidate,AuthorityImage.witness_roundtrip]

theorem world_authority_revalidate (s : State d p n) (v : AuthorityView a)
    (policy : Policy) (ctx : CurrentUse.Context) (evidence : Evidence) :
    revalidate (world s (restoreView (captureView (p:=p) (n:=n) v))).authority policy ctx evidence =
      revalidate (world s v).authority policy ctx evidence := by
  by_cases same : s.realm = v.realm
  · simpa only [world,restoreView,captureView,same,ite_true] using
      revalidate_roundtrip v.authority policy ctx evidence
  · simp only [world,restoreView,captureView,same,ite_false]

theorem use_roundtrip (s : State d p n) (v : AuthorityView a)
    (request : UseRequest (size a p n)) (evidence : Evidence) :
    checkUse (world s (restoreView (captureView (p:=p) (n:=n) v))) request evidence =
      checkUse (world s v) request evidence := by
  have supported : (world s (restoreView (captureView (p:=p) (n:=n) v))).support =
      (world s v).support := support_roundtrip s v
  simp only [checkUse,checkHandle,currentContext,world_records_roundtrip,
    world_policies_roundtrip,supported]
  exact congrArg (fun last =>
    checkHandle (world s v) .member request.member && checkHandle (world s v) .locus request.locus &&
    checkHandle (world s v) .module request.moduleHandle && checkHandle (world s v) .operation request.operation &&
    decide (((world s v).records request.member.key).principal = request.principal) &&
    decide (((world s v).records request.operation.key).home = request.locus.key) &&
    decide (((world s v).records request.operation.key).moduleKey = request.moduleHandle.key) && last)
    (world_authority_revalidate s v ((world s v).policies request.operation.key)
      (currentContext (world s v) request) evidence)

theorem checkAt_roundtrip (s : State d p n) (v : AuthorityView a) (ticket : Ticket)
    (member : Fin a) (key : Fin n) (place : Fin p) :
    checkAt (restoreState (captureState s)) (restoreView (captureView (p:=p) (n:=n) v))
      ticket member key place = checkAt s v ticket member key place := by
  simp [state_roundtrip,checkAt,use_roundtrip]

structure Image (p a : Nat) where
  definitions : Nat
  count : Nat
  state : StateImage definitions p count
  view : ViewImage a p count

def capture (input : OwnerProjection.Input p a) : Image p a :=
  ⟨input.configuration.definitions,input.configuration.count,
    captureState input.configuration.state,captureView input.view⟩

def restore (image : Image p a) : OwnerProjection.Input p a :=
  ⟨⟨image.definitions,image.count,restoreState image.state⟩,restoreView image.view⟩

theorem check_roundtrip (input : OwnerProjection.Input p a) (assigned : Fin p) (ticket : Ticket) :
    OwnerProjection.check (restore (capture input)) assigned ticket = OwnerProjection.check input assigned ticket := by
  simp only [OwnerProjection.check,InvocationBoundary.check,restore,capture]
  split <;> simp [checkAt_roundtrip]

theorem serve_roundtrip (input : OwnerProjection.Input p a) (assigned : Fin p) (ticket : Ticket) :
    OwnerProjection.serve (restore (capture input)) assigned ticket = OwnerProjection.serve input assigned ticket := by
  simp [OwnerProjection.serve,check_roundtrip]

-- Ghost publication values may contain the private caller state; an endpoint
-- representation contains only its explicitly selected owner image. This
-- equality is a refinement obligation for authenticated install/guard custody,
-- not a field supplied by an untrusted frame or a permission to disclose it.
theorem held_serve_current {evaluate : Value → Command → Option Value}
    {publication : PublicationUse.State n Value Command}
    (project : Value → OwnerProjection.Input p a)
    (path : PublicationUse.Reached evaluate n revision initial publication)
    (endpoint : Fin n) (held : Nat × Value) (image : Image p a)
    (retained : publication.held endpoint = some held)
    (represented : image = capture (project held.2))
    (assigned : Fin p) (ticket : Ticket) :
    held.1 = publication.base.barrier.published ∧
      OwnerProjection.serve (restore image) assigned ticket =
        OwnerProjection.serve (project publication.base.current) assigned ticket := by
  obtain ⟨revisionAt,current⟩ := PublicationUse.held_value_current path endpoint held retained
  exact ⟨revisionAt,by rw [represented,serve_roundtrip,current]⟩

#print axioms state_roundtrip
#print axioms members_roundtrip
#print axioms policy_roundtrip
#print axioms world_records_roundtrip
#print axioms support_roundtrip
#print axioms world_policies_roundtrip
#print axioms revalidate_roundtrip
#print axioms world_authority_revalidate
#print axioms use_roundtrip
#print axioms checkAt_roundtrip
#print axioms check_roundtrip
#print axioms serve_roundtrip
#print axioms held_serve_current
end MirroreaProofFirst.OwnerImage
