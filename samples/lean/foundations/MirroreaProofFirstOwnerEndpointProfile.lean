import MirroreaProofFirstOwnerResponseProfile

namespace MirroreaProofFirst.OwnerEndpointProfile
open OwnerResponseProfile

-- Reversible private recipient profile. These checks confer no authority and
-- do not execute owner computations. Reservations/history remain in the old
-- model; rejection does not consume, abandon, restore or fabricate an event.
def ActiveFits (s : OwnerReservation.State p a) : Prop :=
  s.core.capacity ≤ 64 ∧ ∀ ticket, s.active = some ticket →
    ResponseFits s.core.scopeId s.core.revision ticket

def activeCheck (s : OwnerReservation.State p a) : Bool :=
  decide (s.core.capacity ≤ 64) && match s.active with
    | none => true
    | some ticket => responseCheck s.core.scopeId s.core.revision ticket

theorem activeCheck_exact : activeCheck s = true ↔ ActiveFits s := by
  cases active : s.active <;> simp [activeCheck,ActiveFits,active,responseCheck_exact]

def Accepts (capacity : Nat) (state : Option (OwnerEndpoint.State p a))
    (command : OwnerEndpoint.Command p a) : Prop :=
  match state,command with
  | none,.owner (.initialize _) => capacity ≤ 64
  | some s,.owner (.reserve ticket) =>
      s.owner.core.capacity ≤ 64 ∧ ResponseFits s.owner.core.scopeId s.owner.core.revision ticket
  | some s,.owner .compute => ActiveFits s.owner
  | _,_ => True

def check (capacity : Nat) (state : Option (OwnerEndpoint.State p a))
    (command : OwnerEndpoint.Command p a) : Bool :=
  match state,command with
  | none,.owner (.initialize _) => decide (capacity ≤ 64)
  | some s,.owner (.reserve ticket) =>
      decide (s.owner.core.capacity ≤ 64) && responseCheck s.owner.core.scopeId s.owner.core.revision ticket
  | some s,.owner .compute => activeCheck s.owner
  | _,_ => true

theorem check_exact : check capacity state command = true ↔ Accepts capacity state command := by
  cases state <;> cases command with
  | owner command => cases command <;> simp [check,Accepts,responseCheck_exact,activeCheck_exact]
  | freeze revision => simp [check,Accepts]

-- Code15 is a provisional private profile refusal, never a produced reply.
def transition (assigned : OwnerEvaluator.Assignment p) (scopeId capacity : Nat)
    (state : Option (OwnerEndpoint.State p a)) (command : OwnerEndpoint.Command p a) :
    Option (OwnerEndpoint.State p a) × Sum Nat OwnerReceipt.Envelope :=
  if check capacity state command then OwnerEndpoint.transition assigned scopeId capacity state command
  else (state,.inl 15)

theorem admitted_exact (allowed : Accepts capacity state command) :
    transition assigned scopeId capacity state command =
      OwnerEndpoint.transition assigned scopeId capacity state command := by
  simp [transition,check_exact.mpr allowed]

theorem refused_unchanged (refused : ¬ Accepts capacity state command) :
    transition assigned scopeId capacity state command = (state,.inl 15) := by
  have no : check capacity state command ≠ true := fun h => refused (check_exact.mp h)
  simp [transition,no]

theorem refinement :
    transition assigned scopeId capacity state command = (state,.inl 15) ∨
    transition assigned scopeId capacity state command =
      OwnerEndpoint.transition assigned scopeId capacity state command := by
  unfold transition; split
  · exact Or.inr rfl
  · exact Or.inl rfl

theorem owner_step
    (ran : transition assigned scopeId capacity (some s) command = (next,reply)) :
    ∃ value, next = some value ∧ (value.owner = s.owner ∨ OwnerReservation.Step s.owner value.owner) ∧
      s.fence ≤ value.fence := by
  rcases refinement (assigned:=assigned) (scopeId:=scopeId) (capacity:=capacity)
    (state:=some s) (command:=command) with refused | original
  · rw [refused] at ran
    exact ⟨s,(Prod.mk.inj ran).1.symm,Or.inl rfl,Nat.le_refl _⟩
  · rw [original] at ran; exact OwnerEndpoint.owner_step ran

theorem reserve_preserves (valid : ActiveFits s)
    (profile : ResponseFits s.core.scopeId s.core.revision ticket)
    (ran : OwnerReservation.reserve s scopeId ticket = (next,response)) : ActiveFits next := by
  unfold OwnerReservation.reserve at ran
  split at ran
  · cases ran; exact valid
  · split at ran
    · cases ran; exact valid
    · split at ran
      · split at ran <;> cases ran <;> exact valid
      · split at ran
        · cases ran; exact valid
        · cases ran
          exact ⟨valid.1,by intro other same; cases same; exact profile⟩

theorem compute_preserves (valid : ActiveFits s)
    (ran : OwnerReservation.compute s = some (next,reply)) : ActiveFits next := by
  obtain ⟨_,core,_,submitted,rfl⟩ := OwnerReservation.compute_parts ran
  exact ⟨by simpa [submitted_capacity submitted] using valid.1,by simp⟩

theorem install_preserves (valid : ActiveFits s)
    (ran : OwnerReservation.install s revision image = some next) : ActiveFits next := by
  obtain ⟨idle,rfl⟩ := OwnerReservation.install_idle ran
  exact ⟨valid.1,by simp [idle]⟩

theorem worker_preserves {s : OwnerReservation.State p a}
    {command : OwnerReservationWorker.Command p a} (valid : ActiveFits s)
    (reserveProfile : ∀ ticket, command = .reserve ticket →
      ResponseFits s.core.scopeId s.core.revision ticket)
    (ran : OwnerReservationWorker.transition assigned scopeId capacity (some s) command = (next,reply)) :
    ∃ value, next = some value ∧ ActiveFits value := by
  cases command with
  | «initialize» image => exact ⟨s,(Prod.mk.inj ran).1.symm,valid⟩
  | reserve ticket =>
      cases reserved : OwnerReservation.reserve s scopeId ticket with
      | mk value response =>
        simp only [OwnerReservationWorker.transition,reserved,Prod.mk.injEq] at ran
        exact ⟨value,ran.1.symm,reserve_preserves valid (reserveProfile ticket rfl) reserved⟩
  | compute =>
      cases computed : OwnerReservation.compute s with
      | none => simp only [OwnerReservationWorker.transition,computed,Prod.mk.injEq] at ran; exact ⟨s,ran.1.symm,valid⟩
      | some pair =>
        obtain ⟨value,response⟩ := pair
        simp only [OwnerReservationWorker.transition,computed,Prod.mk.injEq] at ran
        exact ⟨value,ran.1.symm,compute_preserves valid computed⟩
  | install revision image =>
      simp only [OwnerReservationWorker.transition] at ran
      split at ran
      · cases installed : OwnerReservation.install s revision image with
        | none => simp only [installed,Prod.mk.injEq] at ran; exact ⟨s,ran.1.symm,valid⟩
        | some value =>
          simp only [installed,Prod.mk.injEq] at ran
          exact ⟨value,ran.1.symm,install_preserves valid installed⟩
      · exact ⟨s,(Prod.mk.inj ran).1.symm,valid⟩
  | abandon => exact ⟨_,(Prod.mk.inj ran).1.symm,valid.1,by simp [OwnerReservation.abandon]⟩

def Invariant : Option (OwnerEndpoint.State p a) → Prop
  | none => True
  | some s => ActiveFits s.owner

theorem original_preserves (valid : Invariant state) (allowed : Accepts capacity state command)
    (ran : OwnerEndpoint.transition assigned scopeId capacity state command = (next,reply)) : Invariant next := by
  cases state with
  | none =>
    cases command with
    | freeze revision => cases ran; trivial
    | owner command =>
      cases command <;> simp only [OwnerEndpoint.transition,OwnerReservationWorker.transition] at ran
      case «initialize» image =>
        split at ran
        · cases ran
          exact ⟨allowed,by simp [OwnerReservation.initial]⟩
        · cases ran; trivial
      all_goals cases ran; trivial
  | some s =>
    cases command with
    | freeze revision =>
      simp only [OwnerEndpoint.transition] at ran
      split at ran <;> cases ran <;> exact valid
    | owner command =>
      simp only [OwnerEndpoint.transition] at ran
      split at ran
      · cases ran; exact valid
      · split at ran
        · cases previous : OwnerReservationWorker.transition assigned scopeId capacity (some s.owner) command with
          | mk owner response =>
            have profile : ∀ ticket, command = .reserve ticket →
                ResponseFits s.owner.core.scopeId s.owner.core.revision ticket := by
              intro ticket same; subst command; exact allowed.2
            obtain ⟨value,equal,good⟩ := worker_preserves valid profile previous
            rw [previous,equal] at ran
            cases ran; exact good
        · cases ran; exact valid

theorem preserves (valid : Invariant state)
    (ran : transition assigned scopeId capacity state command = (next,reply)) : Invariant next := by
  unfold transition at ran
  split at ran
  · rename_i allowed; exact original_preserves valid (check_exact.mp allowed) ran
  · cases ran; exact valid

inductive Runs (assigned : OwnerEvaluator.Assignment p) (scopeId capacity : Nat) :
    Option (OwnerEndpoint.State p a) → Prop where
  | fresh : Runs assigned scopeId capacity none
  | next : Runs assigned scopeId capacity state →
      transition assigned scopeId capacity state command = (next,reply) →
      Runs assigned scopeId capacity next

theorem rooted_profile (path : Runs assigned scopeId capacity state) : Invariant state := by
  induction path with
  | fresh => trivial
  | next _ ran ih => exact preserves ih ran

#print axioms activeCheck_exact
#print axioms check_exact
#print axioms admitted_exact
#print axioms refused_unchanged
#print axioms refinement
#print axioms owner_step
#print axioms reserve_preserves
#print axioms compute_preserves
#print axioms install_preserves
#print axioms worker_preserves
#print axioms original_preserves
#print axioms preserves
#print axioms rooted_profile

-- Output readability follows from the actual successful owner computation,
-- without interpreting a predicted result as a physical production witness.
theorem computed_readable {s next : OwnerReservation.State p a}
    {record : OwnerOccurrence.Record p a} (valid : ActiveFits s)
    (computed : OwnerReservation.compute s = some (next,.produced record)) :
    WireFits OwnerReservationWorker.replyCodec 256 (.inr (OwnerReceipt.project record)) ∧
    WireFits (PublicationInput.input p a) 256 (.step (.arrival (OwnerReceipt.project record))) := by
  obtain ⟨ticket,core,active,submitted,_⟩ := OwnerReservation.compute_parts computed
  obtain ⟨value,result,range⟩ := computed_result_range computed
  have ordinal := produced_ordinal_bounded submitted valid.1
  have profile := valid.2 ticket active
  have recordAt := (OwnerOccurrence.produced_parts submitted).2.2.2.2.1
  have projected : OwnerReceipt.project record =
      ⟨s.core.scopeId,record.ordinal,s.core.revision,ticket,.value value⟩ := by
    simp only [OwnerReceipt.project,recordAt] at result ⊢
    simp only [result]
  rw [projected]
  exact ⟨owner_reply_readable profile ordinal range,arrival_readable profile ordinal range⟩

theorem status_readable (small : code < 16) :
    WireFits OwnerReservationWorker.replyCodec 256 (.inl code) := by
  have fits := fits_natural (show code < 2^4 from small) (by decide : 4 ≤ 128) (textLimit:=256)
  exact fits_wire (fits_inl fits OwnerReceipt.codec) (by decide) (by decide)

theorem worker_readable {s : OwnerReservation.State p a}
    {command : OwnerReservationWorker.Command p a}
    (computeProfile : command = .compute → ActiveFits s)
    (ran : OwnerReservationWorker.transition assigned scopeId capacity (some s) command = (next,reply)) :
    WireFits OwnerReservationWorker.replyCodec 256 reply := by
  cases command with
  | «initialize» image => cases ran; exact status_readable (by decide)
  | reserve ticket =>
      cases reserved : OwnerReservation.reserve s scopeId ticket with
      | mk value response =>
        simp only [OwnerReservationWorker.transition,reserved] at ran
        cases ran
        cases response <;> exact status_readable (by decide)
  | compute =>
      cases computed : OwnerReservation.compute s with
      | none =>
        simp only [OwnerReservationWorker.transition,computed] at ran
        cases ran; exact status_readable (by decide)
      | some pair =>
        obtain ⟨value,response⟩ := pair
        simp only [OwnerReservationWorker.transition,computed] at ran
        cases response with
        | produced record =>
          cases ran; exact (computed_readable (computeProfile rfl) computed).1
        | denied => cases ran; exact status_readable (by decide)
        | duplicate => cases ran; exact status_readable (by decide)
        | conflict => cases ran; exact status_readable (by decide)
        | exhausted => cases ran; exact status_readable (by decide)
  | install revision image =>
      simp only [OwnerReservationWorker.transition] at ran
      split at ran
      · cases installed : OwnerReservation.install s revision image with
        | none => simp only [installed] at ran; cases ran; exact status_readable (by decide)
        | some value => simp only [installed] at ran; cases ran; exact status_readable (by decide)
      · cases ran; exact status_readable (by decide)
  | abandon => cases ran; exact status_readable (by decide)

-- Even an arbitrary supplied state cannot bypass the pre-compute profile.
-- This does not authorize populated import or make such a state rooted.
theorem every_reply_readable
    (ran : transition assigned scopeId capacity state command = (next,reply)) :
    WireFits OwnerReservationWorker.replyCodec 256 reply := by
  unfold transition at ran
  split at ran
  · rename_i checked
    have allowed := check_exact.mp checked
    cases state with
    | none =>
      cases command with
      | freeze revision => cases ran; exact status_readable (by decide)
      | owner command =>
        cases command <;> simp only [OwnerEndpoint.transition,OwnerReservationWorker.transition] at ran
        case «initialize» image => split at ran <;> cases ran <;> exact status_readable (by decide)
        all_goals cases ran; exact status_readable (by decide)
    | some s =>
      cases command with
      | freeze revision =>
        simp only [OwnerEndpoint.transition] at ran
        split at ran <;> cases ran <;> exact status_readable (by decide)
      | owner command =>
        simp only [OwnerEndpoint.transition] at ran
        split at ran
        · cases ran; exact status_readable (by decide)
        · split at ran
          · cases previous : OwnerReservationWorker.transition assigned scopeId capacity (some s.owner) command with
            | mk owner response =>
              have readable := worker_readable (s:=s.owner) (command:=command)
                (by intro same; subst command; exact allowed) previous
              rw [previous] at ran
              exact (Prod.mk.inj ran).2 ▸ readable
          · cases ran; exact status_readable (by decide)
  · cases ran; exact status_readable (by decide)

-- Positive progress derives both old reservation and actual model production
-- from independent admission/freshness/room premises and rooted inventory.
-- No assumed successful schedule or readable output occurs in the premises.
theorem enabled_roundtrip {s : OwnerEndpoint.State p a}
    (path : OwnerReservation.Steps (OwnerReservation.initial initialAssigned initialScope initialImage initialCapacity) s.owner)
    (small : s.owner.core.capacity ≤ 64)
    (profile : ResponseFits s.owner.core.scopeId s.owner.core.revision ticket)
    (current : s.owner.core.revision = s.fence)
    (scope : scopeId = s.owner.core.scopeId)
    (admitted : OwnerEvaluator.Admitted s.owner.core.assigned s.owner.core.image ticket)
    (idle : s.owner.active = none) (fresh : OwnerReservation.hasKey s.owner ticket = false)
    (room : s.owner.reserved.length < s.owner.core.capacity) :
    ∃ (reserved computed : OwnerEndpoint.State p a) (record : OwnerOccurrence.Record p a),
      transition assigned scopeId capacity (some s) (.owner (.reserve ticket)) = (some reserved,.inl 6) ∧
      transition assigned scopeId capacity (some reserved) (.owner .compute) = (some computed,.inr (OwnerReceipt.project record)) ∧
      WireFits OwnerReservationWorker.replyCodec 256 (.inr (OwnerReceipt.project record)) ∧
      WireFits (PublicationInput.input p a) 256 (.step (.arrival (OwnerReceipt.project record))) := by
  obtain ⟨owner,reserved⟩ := OwnerReservation.admissible_reservation_progress scope admitted idle fresh room
  obtain ⟨later,record,computed⟩ := OwnerReservation.rooted_reserved_computes path reserved
  have before : ActiveFits s.owner := ⟨small,by simp [idle]⟩
  have after := reserve_preserves before profile reserved
  have core := OwnerReservation.reserve_core reserved
  refine ⟨⟨owner,s.fence⟩,⟨later,s.fence⟩,record,?_,?_,(computed_readable after computed).1,(computed_readable after computed).2⟩
  · rw [admitted_exact (show Accepts capacity (some s) (.owner (.reserve ticket)) from ⟨small,profile⟩)]
    simp [OwnerEndpoint.transition,OwnerEndpoint.confirms,OwnerEndpoint.permitted,current,
      OwnerReservationWorker.transition,reserved,OwnerReservationWorker.responseCode]
  · rw [admitted_exact (show Accepts capacity (some (⟨owner,s.fence⟩ : OwnerEndpoint.State p a)) (.owner .compute) from after)]
    have same : owner.core.revision = s.fence := by rw [core,current]
    simp [OwnerEndpoint.transition,OwnerEndpoint.confirms,OwnerEndpoint.permitted,same,
      OwnerReservationWorker.transition,computed]

#print axioms computed_readable
#print axioms status_readable
#print axioms worker_readable
#print axioms every_reply_readable
#print axioms enabled_roundtrip
end MirroreaProofFirst.OwnerEndpointProfile
