import OwnerStatementResultOrigin
namespace MirroreaProofFirst.OwnerStatementResultLifecycle
open OwnerStatementLiveCustodian OwnerStatementResultCollection
namespace Source
export OwnerStatementSourceCustodian (progress transfer consume invokeAgain refreshAuthority)
end Source

def WaitingFor (s : State p a) (ticket : Ticket) : Prop :=
 ∃ waiting, MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting = some waiting ∧ waiting.saved = ticket.saved

-- This invariant starts at the designated live constructor and follows the
-- actual guarded service. Historical admission alone cannot establish it.
def PostHeld (s : State p a) : Prop :=
 ∀ ticket phase, s.held = some ⟨ticket,phase⟩ → phase ≠ .queued →
 ticket.designation = s.designation ∧ ticket ∈ s.dispatched ∧ WaitingFor s ticket ∧
 (phase = .reported → ∃ written, collect s ticket = some (.committed written) ∧ written.pending = ticket.saved)

theorem ready_waiting (ready : Ready s ticket) : WaitingFor s ticket := by
 have checked := ready.2.2.2
 unfold currentCheck at checked
 cases packet : s.live.packet with
 | none => simp [packet] at checked
 | some value =>
   simp only [packet,ready.2.2.1] at checked
   cases selected : OwnerStatementRegistrySelection.select s.live.session s.live.bank ticket.saved with
   | none => simp [selected] at checked
   | some pair =>
     simp only [selected,Bool.and_eq_true] at checked
     obtain ⟨waiting,saved,held,queued,same,_⟩ := MixedOwnerSourceTrace.service_ready_exact.mp checked.2
     have eqSaved := Option.some.inj (queued.symm.trans ready.2.2.1)
     exact ⟨waiting,by simp [MixedOwnerSourceIssue.ownerWaiting,held],same.trans eqSaved⟩

theorem bank_service_source :
 (OwnerStatementRegistrySelection.service s ops).session.state.source = s.session.state.source := by
 rcases OwnerStatementRegistrySelection.service_route (s:=s) (ops:=ops) with same | ⟨_,_,_,_,_,_,_,_,same⟩
 · rw [same]
 · rw [same]
   rcases OwnerStatementEntryGate.execute_route (live:=OwnerStatementRegistrySelection.view s _) (ops:=ops) with kept | ⟨_,kept⟩
   · rw [kept]; rfl
   · rw [kept]
     rcases OwnerMetadataSession.service_route (s:=OwnerStatementRegistrySelection.view s _) (ops:=ops) with unchanged | ⟨_,_,_,_,_,_,advanced⟩
     · rw [unchanged]; rfl
     · rw [advanced]
       simp only [MixedOwnerContinuation.service,MixedOwnerSourceTrace.service]
       split <;> rfl

theorem start_postHeld : PostHeld (start designation live) := by
 intro ticket phase impossible; cases impossible

theorem stage_postHeld (run : stage capacity s = some (next,ticket)) : PostHeld next := by
 obtain ⟨_,_,_,_,_,rfl⟩ := stage_exact.mp run
 intro other phase held notQueued
 have equal := Held.mk.inj (Option.some.inj held)
 exact False.elim (notQueued equal.2.symm)

theorem execute_postHeld (run : execute s ticket ops = some next) : PostHeld next := by
 obtain ⟨ready,rfl⟩ := execute_exact.mp run
 intro other phase held _
 have equal := Held.mk.inj (Option.some.inj held)
 obtain ⟨rfl,rfl⟩ := equal
 refine ⟨ready.1,by simp,?_,by intro impossible; cases impossible⟩
 have waiting := ready_waiting ready
 simpa only [WaitingFor,bank_service_source] using waiting

theorem report_postHeld (valid : PostHeld s) (run : reportActual s ticket = some next) : PostHeld next := by
 obtain ⟨written,collected,same,held,rfl⟩ := reportActual_exact.mp run
 have previous := valid ticket .unreported held (by decide)
 intro other phase found _
 have equal := Held.mk.inj (Option.some.inj found)
 obtain ⟨rfl,rfl⟩ := equal
 refine ⟨previous.1,previous.2.1,previous.2.2.1,?_⟩
 intro _
 refine ⟨written,collect_exact.mpr ?_,same⟩
 exact ⟨collected.1,Or.inr rfl,collected.2.2⟩

-- This path deliberately replaces the broader historical phase-only report
-- with collection of the actual result. Other source and authority operations
-- are the original functions; there is no new evaluator or authority issuer.
inductive Path (capacity : Nat) (initial : State p a) : State p a → Prop where
 | initial : Path capacity initial initial
 | progress : Path capacity initial s → Source.progress s member principal = some next → Path capacity initial next
 | transfer : Path capacity initial s → Source.transfer s = some next → Path capacity initial next
 | stage : Path capacity initial s → stage capacity s = some (next,ticket) → Path capacity initial next
 | execute : Path capacity initial s → execute s ticket ops = some next → Path capacity initial next
 | report : Path capacity initial s → reportActual s ticket = some next → Path capacity initial next
 | consume : Path capacity initial s → Source.consume s ticket member principal = some next → Path capacity initial next
 | invokeAgain : Path capacity initial s → Source.invokeAgain s = some next → Path capacity initial next
 | authority : Path capacity initial s → Source.refreshAuthority s authority = some next → Path capacity initial next

theorem path_trans (before : Path capacity initial middle) (after : Path capacity middle final) : Path capacity initial final := by
 induction after with
 | initial => exact before
 | progress _ run ih => exact .progress ih run
 | transfer _ run ih => exact .transfer ih run
 | stage _ run ih => exact .stage ih run
 | execute _ run ih => exact .execute ih run
 | report _ run ih => exact .report ih run
 | consume _ run ih => exact .consume ih run
 | invokeAgain _ run ih => exact .invokeAgain ih run
 | authority _ run ih => exact .authority ih run

theorem path_refines (path : Path capacity initial s) : OwnerStatementSourceCustodian.Path capacity initial s := by
 induction path with
 | initial => exact .initial
 | progress _ run ih => exact .progress ih run
 | transfer _ run ih => exact .transfer ih run
 | stage _ run ih => exact .stage ih run
 | execute _ run ih => exact .execute ih run
 | report _ run ih => exact OwnerStatementSourceCustodian.path_trans ih (reportActual_refines run)
 | consume _ run ih => exact .consume ih run
 | invokeAgain _ run ih => exact .invokeAgain ih run
 | authority _ run ih => exact .authority ih run

theorem path_postHeld (valid : PostHeld initial) (path : Path capacity initial s) : PostHeld s := by
 induction path with
 | initial => exact valid
 | progress _ run ih =>
   obtain ⟨idle,_,rfl⟩ := OwnerStatementSourceCustodian.progress_exact.mp run
   intro _ _ held; simp [idle] at held
 | transfer _ run ih =>
   obtain ⟨idle,_,_,rfl⟩ := OwnerStatementSourceCustodian.transfer_exact.mp run
   intro _ _ held; simp [idle] at held
 | stage _ run ih => exact stage_postHeld run
 | execute _ run ih => exact execute_postHeld run
 | report _ run ih => exact report_postHeld ih run
 | consume _ run ih =>
   obtain ⟨_,_,_,_,_,_,rfl⟩ := OwnerStatementSourceCustodian.consume_exact.mp run
   intro _ _ impossible; cases impossible
 | invokeAgain _ run ih =>
   obtain ⟨idle,_,_,rfl⟩ := OwnerStatementSourceCustodian.invokeAgain_exact.mp run
   intro _ _ held; simp [idle] at held
 | authority _ run ih =>
   obtain ⟨idle,_,_,_,rfl⟩ := OwnerStatementSourceCustodian.refreshAuthority_exact.mp run
   intro _ _ held; simp [idle] at held

theorem rooted_report_origin
 (entered : OwnerStatementRegistryInvariant.Admitted realm authority policy store live)
 (path : Path capacity (start designation live) s)
 (held : s.held = some ⟨ticket,.reported⟩) :
 WaitingFor s ticket ∧ ticket ∈ s.dispatched ∧
 ∃ written, collect s ticket = some (.committed written) ∧
 written.pending = ticket.saved ∧ written ∈ s.live.session.state.owner.history := by
 have actual := path_postHeld start_postHeld path ticket .reported held (by decide)
 obtain ⟨written,collected,same⟩ := actual.2.2.2 rfl
 have admitted := OwnerStatementSourceCustodian.path_admitted entered (path_refines path)
 exact ⟨actual.2.2.1,actual.2.1,written,collected,same,(OwnerStatementResultOrigin.admitted_collected_commit admitted collected).2⟩

theorem wrong_waiting_not_rooted (wrong : ¬WaitingFor s ticket)
 (held : s.held = some ⟨ticket,.reported⟩) : ¬Path capacity (start designation live) s := by
 intro path
 exact wrong (path_postHeld start_postHeld path ticket .reported held (by decide)).2.2.1

theorem rooted_consume_sound
 (entered : OwnerStatementRegistryInvariant.Admitted realm authority policy store live)
 (path : Path capacity (start designation live) s)
 (run : Source.consume s ticket member principal = some next) :
 OwnerStatementCompletion.Accepted s.live.session next.live.session := by
 have admitted := OwnerStatementSourceCustodian.path_admitted entered (path_refines path)
 exact OwnerStatementSourceCustodian.consume_sound (OwnerStatementRegistryInvariant.admitted_valid admitted).1 run

theorem refused_report_not_rooted
 (refused : collect s ticket = some (.refused why))
 (held : s.held = some ⟨ticket,.reported⟩) : ¬Path capacity (start designation live) s := by
 intro path
 obtain ⟨written,committed,_⟩ := (path_postHeld start_postHeld path ticket .reported held (by decide)).2.2.2 rfl
 rw [refused] at committed
 cases committed

#print axioms refused_report_not_rooted
#print axioms ready_waiting
#print axioms bank_service_source
#print axioms path_refines
#print axioms path_postHeld
#print axioms rooted_report_origin
#print axioms wrong_waiting_not_rooted
#print axioms rooted_consume_sound
end MirroreaProofFirst.OwnerStatementResultLifecycle
