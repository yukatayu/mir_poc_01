import OwnerCurrentCorrespondence
open MirroreaProofFirst SharedHostCaptureReplay
namespace OwnerLeasePhaseRelation
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

def leasePair (memory : OwnerCommitJournal.Memory p a) := (memory.lease,memory.entered)
def released : Option InvocationBoundary.Ticket × Bool := (none,false)

def expected (mode : CohortPhase.Mode p) (owner : Fin p) : Option InvocationBoundary.Ticket × Bool :=
  match mode with
  | .entered dispatch => if owner = dispatch.endpoint then (some dispatch.ticket,true) else released
  | _ => released

-- Local active entry/writer journals already constrain the focused owner's
-- full memory. This additional relation constrains EVERY OTHER owner's lease,
-- and links idle stored leases to the SAME actual native funding phase.
def AtOwner (state : SourceEntryPrefix.State base) (owner : Fin p) : Prop :=
  match state.joined.active,state.active with
  | some writer,_ => owner ≠ writer.bound.writer.owner → leasePair (state.writers owner) = released
  | none,some entry => owner ≠ entry.entry.context.endpoint → leasePair (state.writers owner) = released
  | none,none => leasePair (state.writers owner) = expected state.joined.history.current.mode owner

def Aligned (state : SourceEntryPrefix.State base) : Prop := ∀ owner, AtOwner state owner

instance (state : SourceEntryPrefix.State base) (owner : Fin p) : Decidable (AtOwner state owner) := by
  unfold AtOwner
  split <;> infer_instance

def check (state : SourceEntryPrefix.State base) : Bool :=
  (List.finRange p).all (fun owner => decide (AtOwner state owner))

theorem check_exact (state : SourceEntryPrefix.State base) : check state = true ↔ Aligned state := by
  simp [check,Aligned,List.all_eq_true]

theorem released_mode (notEntered : ∀ dispatch, mode ≠ CohortPhase.Mode.entered dispatch) :
    expected mode owner = released := by cases mode <;> simp_all [expected]

theorem fresh_aligned (mode : base.mode = .prelude owed) : Aligned (SourceEntryPrefix.State.start base) := by
  intro owner
  simp [AtOwner,SourceEntryPrefix.State.start,JoinedState.start,SharedWireLifetime.History.start,leasePair,expected,mode,released]

-- This is a checkable research relation, NOT yet a theorem of all operational
-- paths. Full preservation and relative admission remain explicit obligations.
#print axioms check_exact
#print axioms fresh_aligned
end OwnerLeasePhaseRelation
