import MirroreaProofFirstOwnerEffectService
import SourceFrontier
namespace MirroreaProofFirst.OwnerEffectReceipt
open CurrentUse OwnerEffectService

structure Statement where
 document : String
 byteOffset : Nat
 operationKey : Nat
 body : Body
 deriving DecidableEq

def statement (e : Pending n) : Statement := ⟨e.origin.document,e.origin.byteOffset,e.request.operation.key.val,e.body⟩

structure Reply (n : Nat) where
 pending : Pending n
 serviceEvidence : Evidence
 deriving DecidableEq

def reply (w : Write n) : Reply n := ⟨w.pending,w.serviceEvidence⟩

-- This result checks exact retained binding; it does not evaluate the owner tree.
-- Carrier authenticity is separate: arbitrary construction of Reply is not a
-- theorem of physical origin. Source/control labels and budgets remain required
-- at the enclosing admitted source session, not granted by this frontier view.
def ackCheck (world : World n) (pending : Pending n) (received : Reply n) : Bool :=
 decide (received.pending = pending) &&
 decide (received.serviceEvidence.context = pending.original.context) &&
 checkUse world pending.request pending.original

def Accepted (world : World n) (pending : Pending n) (received : Reply n) : Prop :=
 received.pending = pending ∧ received.serviceEvidence.context = pending.original.context ∧
 AdmissionPhases.Bound world.authority (world.policies pending.request.operation.key)
   (currentContext world pending.request) pending.original ∧
 CurrentUse.CurrentUse world pending.request

theorem ack_exact : ackCheck world pending received = true ↔ Accepted world pending received := by
 unfold ackCheck Accepted
 simp only [Bool.and_eq_true,decide_eq_true_eq]
 constructor
 · rintro ⟨⟨same,stamp⟩,use⟩
   refine ⟨same,stamp,?_,checkUse_sound _ _ _ use⟩
   simp only [checkUse,Bool.and_eq_true] at use
   exact AdmissionPhases.revalidate_exact.mp use.2
 · rintro ⟨same,stamp,bound,use⟩
   exact ⟨⟨same,stamp⟩,AdmissionPhases.FullUse.current_checks use bound⟩

structure Caller (n : Nat) where
 frontier : SourceFrontier.Cursor Statement
 awaiting : Option (Pending n)
 deriving DecidableEq

def arrive (world : World n) (caller : Caller n) (received : Reply n) : Option (Caller n) := do
 let pending ← caller.awaiting
 if ackCheck world pending received && decide (caller.frontier.stopped = some (statement pending)) then
  return ⟨SourceFrontier.finish caller.frontier,none⟩
 else none

theorem arrive_exact : arrive world caller received = some next ↔
 ∃ pending, caller.awaiting = some pending ∧ Accepted world pending received ∧
 caller.frontier.stopped = some (statement pending) ∧
 next = ⟨SourceFrontier.finish caller.frontier,none⟩ := by
 unfold arrive
 cases waiting : caller.awaiting with
 | none => simp
 | some pending =>
  simp only [Option.bind_eq_bind,Option.bind_some]
  by_cases ok : ackCheck world pending received = true ∧ caller.frontier.stopped = some (statement pending)
  · simp [ok,ack_exact.mp ok.1,eq_comm]
  · have no : ¬(Accepted world pending received ∧ caller.frontier.stopped = some (statement pending)) := by
     simpa [ack_exact] using ok
    simp only [Bool.and_eq_true,decide_eq_true_eq,ok,ite_false,reduceCtorEq,false_iff,Option.some.injEq]
    rintro ⟨_,rfl,accepted,stopped,_⟩
    exact no ⟨accepted,stopped⟩

theorem duplicate_ack_rejected (ok : arrive world caller received = some next) :
 arrive later next duplicate = none := by
 obtain ⟨_,_,_,_,rfl⟩ := arrive_exact.mp ok
 rfl

theorem accepted_frontier_step (ok : arrive world caller received = some next) :
 SourceFrontier.step caller.frontier .receive = some next.frontier := by
 obtain ⟨_,_,_,stopped,rfl⟩ := arrive_exact.mp ok
 simp [SourceFrontier.step,stopped]

theorem accepted_preserves_partition (valid : SourceFrontier.Partition program caller.frontier)
 (ok : arrive world caller received = some next) : SourceFrontier.Partition program next.frontier :=
 SourceFrontier.moves_partition valid (SourceFrontier.step_sound (accepted_frontier_step ok))

theorem changed_service_generation_rejected
 (different : received.serviceEvidence.context.generation ≠ pending.original.context.generation) :
 ackCheck world pending received = false := by
 have ne : received.serviceEvidence.context ≠ pending.original.context :=
  fun h => different (congrArg Context.generation h)
 simp [ackCheck,ne]

theorem wrong_binding_rejected (different : received.pending ≠ pending) :
 ackCheck world pending received = false := by simp [ackCheck,different]

-- The owner state is outside the optional caller result. A rejected ack cannot
-- erase committed state/history. No owner evaluator occurs on this path.
def deliver (world : World n) (owner : Owner n) (caller : Caller n) (received : Reply n) :
 Owner n × Caller n := (owner,(arrive world caller received).getD caller)

theorem delivery_keeps_owner : (deliver world owner caller received).1 = owner := rfl

theorem refused_delivery_unchanged (refused : arrive world caller received = none) :
 deliver world owner caller received = (owner,caller) := by simp [deliver,refused]

theorem commit_then_refusal_retains
 (committed : (serve ops ownerWorld registry owner pending).2 = .committed written)
 (refused : arrive callerWorld caller (reply written) = none) :
 (deliver callerWorld (serve ops ownerWorld registry owner pending).1 caller (reply written)).1.history =
 owner.history ++ [written] ∧
 (deliver callerWorld (serve ops ownerWorld registry owner pending).1 caller (reply written)).2 = caller := by
 exact ⟨(committed_state committed).2,by simp [deliver,refused]⟩

#print axioms ack_exact
#print axioms arrive_exact
#print axioms duplicate_ack_rejected
#print axioms accepted_frontier_step
#print axioms accepted_preserves_partition
#print axioms changed_service_generation_rejected
#print axioms wrong_binding_rejected
#print axioms delivery_keeps_owner
#print axioms refused_delivery_unchanged
#print axioms commit_then_refusal_retains
end MirroreaProofFirst.OwnerEffectReceipt
