import MirroreaProofFirstOwnerFundingCursor

namespace MirroreaProofFirst.SourceFundingInput
open OwnerCodecTree
open PublicationCapacityDriver (State Status)

-- Privileged candidate carrier, not source syntax or a public wire contract.
-- Finite credits must come from the sole retained real owner monitors. The
-- codec preserves the entire vector and actual source input; it authenticates
-- neither them nor the source/owner namespace. No populated source state is an
-- input. Stable selected native owner starts at512 and never refills.
abbrev Request (p a : Nat) := Vector (Fin 513) p × PublicationInput.Input p a

def request (p a : Nat) : Codec (Request p a) :=
  product (vector (finite 513) p) (PublicationInput.input p a)

def credits (value : Request p a) : Fin p → Nat := fun i => (value.1[i.val]).val

-- Initial checked source image must still be installed in each fresh owner.
-- Its first successful initialization spends one actual owner transition.
-- Later source commands do not reinitialize owners. Repeated/reconnected
-- initialization is not licensed by this arithmetic admission condition.
def initialDebit (old : State p a) : Fin p → Nat := fun _ => if old.source.isNone then 1 else 0

def execute (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (old : State p a) (value : Request p a) : State p a × Status :=
  let (next,status) := PublicationLifecycle.transitionFast assigned scopeId seed old value.2
  if status = .accepted then
    if OwnerFundingCursor.chargeCheck next.suffix 0 (credits value) (initialDebit old)
    then (next,status) else (old,.profileRefused)
  else (next,status)

def Admissible (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (old next : State p a) (value : Request p a) : Prop :=
  PublicationLifecycle.transitionFast assigned scopeId seed old value.2 = (next,.accepted) ∧
  ∀ owner, initialDebit old owner + PublicationOwnerBudget.demand next.suffix owner ≤ credits value owner

theorem accepted_exact : execute assigned scopeId seed old value = (next,.accepted) ↔
    Admissible assigned scopeId seed old next value := by
  cases ran : PublicationLifecycle.transitionFast assigned scopeId seed old value.2 with
  | mk actual status =>
    cases status <;> simp [execute,ran,Admissible]
    by_cases enough : OwnerFundingCursor.chargeCheck actual.suffix 0 (credits value) (initialDebit old) = true
    · simp [enough,eq_comm]
      intro same; subst next
      simpa [OwnerFundingCursor.ChargeAdmissible] using OwnerFundingCursor.charge_exact.mp enough
    · have no : ¬ (∀ owner, initialDebit old owner + PublicationOwnerBudget.demand actual.suffix owner ≤ credits value owner) := by
        intro allowed
        apply enough
        apply OwnerFundingCursor.charge_exact.mpr
        simpa [OwnerFundingCursor.ChargeAdmissible] using allowed
      simp [enough]
      intro same; subst next; simpa using no

theorem admitted_progress (admitted : Admissible assigned scopeId seed old next value) :
    execute assigned scopeId seed old value = (next,.accepted) := accepted_exact.mpr admitted

theorem refused_unchanged
    (candidate : PublicationLifecycle.transitionFast assigned scopeId seed old value.2 = (next,.accepted))
    (short : ∃ owner, credits value owner < initialDebit old owner + PublicationOwnerBudget.demand next.suffix owner) :
    execute assigned scopeId seed old value = (old,.profileRefused) := by
  have denied : ¬ OwnerFundingCursor.chargeCheck next.suffix 0 (credits value) (initialDebit old) = true := by
    intro yes
    have enough := OwnerFundingCursor.charge_exact.mp yes
    obtain ⟨owner,short⟩ := short
    have bound := enough owner
    simp at bound
    omega
  simp [execute,candidate,denied]

theorem initial_reserved (accepted : execute assigned scopeId seed old value = (next,.accepted))
    (fresh : old.source = none) :
    ∀ owner, 1 + PublicationOwnerBudget.demand next.suffix owner ≤ credits value owner := by
  have funded := (accepted_exact.mp accepted).2
  simpa [initialDebit,fresh] using funded

theorem accepted_covered (accepted : execute assigned scopeId seed old value = (next,.accepted)) :
    PublicationOwnerBudget.Covered next.suffix (fun owner => credits value owner-initialDebit old owner) := by
  intro owner
  have funded := (accepted_exact.mp accepted).2 owner
  change PublicationOwnerBudget.demand next.suffix owner ≤ credits value owner-initialDebit old owner
  omega

theorem preserves (valid : PublicationLifecycle.Invariant assigned scopeId seed old) :
    PublicationLifecycle.Invariant assigned scopeId seed (execute assigned scopeId seed old value).1 := by
  have original := PublicationLifecycle.transitionFast_preserves valid (input:=value.2)
  cases ran : PublicationLifecycle.transitionFast assigned scopeId seed old value.2 with
  | mk next status =>
    rw [ran] at original
    cases status with
    | accepted =>
      by_cases enough : OwnerFundingCursor.chargeCheck next.suffix 0 (credits value) (initialDebit old) = true
      · simpa [execute,ran,enough] using original
      · simpa [execute,ran,enough] using valid
    | semanticRefused => simpa [execute,ran] using original
    | profileRefused => simpa [execute,ran] using original

-- Whole reply framing/readability is inherited for every new outcome, not
-- just the newly accepted path. The extra input carrier does not enlarge it.
theorem reply_readable {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a} {old : State p a} {value : Request p a}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old) :
    let result := execute assigned scopeId seed old value
    let bytes := OwnerPacketCodec.encode (PublicationCapacityDriver.reply p a)
      (result.2,result.1.source.map SourcePublicationWorker.project)
    bytes.length ≤ 65536 ∧ SourceCodec.compactFits bytes = true ∧
      OwnerPacketCodec.decodeAt (PublicationCapacityDriver.reply p a) 256 bytes =
        some (result.2,result.1.source.map SourcePublicationWorker.project) := by
  have after := preserves valid (value:=value)
  exact ⟨PublicationLifecycle.bounded_reply after _,PublicationLifecycle.readable_reply after _⟩

-- The full typed request is retained by successful decoding. No default or
-- truncated vector, alternate entry, snapshot/import or supplied success flag.
theorem request_exact : OwnerPacketCodec.decode (request p a) bytes = some value ↔
    bytes = OwnerPacketCodec.encode (request p a) value := OwnerPacketCodec.exact _ _ _

theorem bounded_request (value : Request p a)
    (fits : OwnerTreeBytes.treeCost ((request p a).encode value) ≤ fuel) :
    OwnerPacketCodec.decodeAt (request p a) fuel (OwnerPacketCodec.encode (request p a) value) = some value :=
  OwnerPacketCodec.bounded_roundtrip _ _ _ fits

#print axioms accepted_exact
#print axioms admitted_progress
#print axioms refused_unchanged
#print axioms initial_reserved
#print axioms accepted_covered
#print axioms preserves
#print axioms reply_readable
#print axioms request_exact
#print axioms bounded_request
end MirroreaProofFirst.SourceFundingInput
