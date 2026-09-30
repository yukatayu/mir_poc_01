import OwnerStatementAcknowledgment
namespace MirroreaProofFirst.OwnerStatementHeldAuthority
open OwnerStatementLiveCustodian
namespace Bank
export OwnerStatementRegistrySelection (authorityHead Rooted rooted_trans)
end Bank

-- This successor consumer allows a genuine monotone authority-head installation
-- while a source request/result is retained. It leaves the old restricted Path
-- unchanged. Authentic publisher, publication exclusion, physical current view
-- and all-entry provenance are separate obligations, not granted by this model.
def refresh (s : State p a) (view : WorldProjection.AuthorityView a) : Option (State p a) :=
 (Bank.authorityHead s.live view).map fun live => {s with live := live}

theorem refresh_exact : refresh s view = some next ↔
 ∃ live, Bank.authorityHead s.live view = some live ∧ next = {s with live := live} := by
 unfold refresh
 cases run : Bank.authorityHead s.live view <;> simp [eq_comm]

theorem refresh_enabled : (∃ next, refresh s view = some next) ↔
 ReferenceAuthority.Successor s.live.session.state.source.machine.store.core.system.view view := by
 simp only [refresh,Bank.authorityHead,OwnerStatementRegistrySelection.authorityHead,
   MixedOwnerContinuation.authorityHead,MixedOwnerSourceTrace.authorityHead,
   MixedOwnerSourceIssue.authorityHead]
 cases run : MixedReferenceAuthority.install (MixedOwnerSourceIssue.base s.live.session.state.source) view with
 | none => simpa [run] using (MixedReferenceAuthority.install_exact
     (MixedOwnerSourceIssue.base s.live.session.state.source) view)
 | some result => simpa [run] using (MixedReferenceAuthority.install_exact
     (MixedOwnerSourceIssue.base s.live.session.state.source) view)

structure Kept (before after : State p a) : Prop where
 held : after.held = before.held
 designation : after.designation = before.designation
 dispatched : after.dispatched = before.dispatched
 serial : after.nextSerial = before.nextSerial
 owner : after.live.session.state.owner = before.live.session.state.owner
 waiting : after.live.session.state.source.waiting = before.live.session.state.source.waiting
 cursor : after.live.session.cursor = before.live.session.cursor
 program : after.live.session.program = before.live.session.program
 activation : after.live.session.activation = before.live.session.activation
 inbox : after.live.session.state.inbox = before.live.session.state.inbox
 outbox : after.live.session.state.outbox = before.live.session.state.outbox
 bank : after.live.bank = before.live.bank
 packet : after.live.packet = before.live.packet

theorem refresh_keeps (run : refresh s view = some next) : Kept s next := by
 obtain ⟨live,changed,rfl⟩ := refresh_exact.mp run
 unfold OwnerStatementRegistrySelection.authorityHead MixedOwnerContinuation.authorityHead
   MixedOwnerSourceTrace.authorityHead MixedOwnerSourceIssue.authorityHead at changed
 cases installed : MixedReferenceAuthority.install (MixedOwnerSourceIssue.base s.live.session.state.source) view with
 | none => simp [installed] at changed
 | some source =>
   simp only [installed,Option.map_some,Option.some.injEq] at changed
   subst live
   obtain ⟨_,rfl⟩ := MixedReferenceAuthority.install_parts _ _ _ installed
   refine ⟨rfl,rfl,rfl,rfl,rfl,?_,rfl,rfl,rfl,rfl,rfl,rfl,rfl⟩
   cases waiting : s.live.session.state.source.waiting with
   | none => simp [MixedOwnerSourceIssue.merge,MixedOwnerSourceIssue.ownerWaiting,
       MixedOwnerSourceIssue.base,MixedOwnerSourceIssue.pureWaiting,
       MixedReferenceSource.authorityHead,MixedReferenceSource.mark,waiting]
   | some value => cases value <;>
       simp [MixedOwnerSourceIssue.merge,MixedOwnerSourceIssue.ownerWaiting,
         MixedOwnerSourceIssue.base,MixedOwnerSourceIssue.pureWaiting,
         MixedReferenceSource.authorityHead,MixedReferenceSource.mark,waiting]

theorem collection_kept (run : refresh s view = some next) :
 OwnerStatementResultCollection.collect next ticket = OwnerStatementResultCollection.collect s ticket := by
 have kept := refresh_keeps run
 simp only [OwnerStatementResultCollection.collect,kept.designation,kept.held,kept.owner]

theorem postHeld_kept (valid : OwnerStatementResultLifecycle.PostHeld s)
 (run : refresh s view = some next) : OwnerStatementResultLifecycle.PostHeld next := by
 have kept := refresh_keeps run
 intro ticket phase held notQueued
 rw [kept.held] at held
 have prior := valid ticket phase held notQueued
 refine ⟨prior.1.trans kept.designation.symm,?_,?_,?_⟩
 · simpa only [kept.dispatched] using prior.2.1
 · simpa only [OwnerStatementResultLifecycle.WaitingFor,kept.waiting] using prior.2.2.1
 · intro reported
   obtain ⟨written,collected,same⟩ := prior.2.2.2 reported
   exact ⟨written,(collection_kept run).trans collected,same⟩

-- Blocks of original guarded source/service/report operations can interleave
-- with authentic head changes. This is not the old idle-only Path relabelled.
inductive Path (capacity : Nat) (initial : State p a) : State p a → Prop where
 | initial : Path capacity initial initial
 | original : Path capacity initial s → OwnerStatementResultLifecycle.Path capacity s next → Path capacity initial next
 | authority : Path capacity initial s → refresh s view = some next → Path capacity initial next

theorem path_refines (path : Path capacity initial s) : Bank.Rooted initial.live s.live := by
 induction path with
 | initial => exact .initial
 | original _ old ih =>
   exact Bank.rooted_trans ih (OwnerStatementSourceCustodian.path_refines
     (OwnerStatementResultLifecycle.path_refines old))
 | authority _ run ih =>
   obtain ⟨live,changed,rfl⟩ := refresh_exact.mp run
   exact .authority ih changed

theorem path_admitted (entered : OwnerStatementRegistryInvariant.Admitted realm authority policy store initial.live)
 (path : Path capacity initial s) : OwnerStatementRegistryInvariant.Admitted realm authority policy store s.live := by
 obtain ⟨origin,launched,prior⟩ := entered
 exact ⟨origin,launched,Bank.rooted_trans prior (path_refines path)⟩

theorem path_postHeld (valid : OwnerStatementResultLifecycle.PostHeld initial)
 (path : Path capacity initial s) : OwnerStatementResultLifecycle.PostHeld s := by
 induction path with
 | initial => exact valid
 | original _ run ih => exact OwnerStatementResultLifecycle.path_postHeld ih run
 | authority _ run ih => exact postHeld_kept ih run

theorem rooted_report_origin
 (entered : OwnerStatementRegistryInvariant.Admitted realm authority policy store live)
 (path : Path capacity (start designation live) s)
 (held : s.held = some ⟨ticket,.reported⟩) :
 OwnerStatementResultLifecycle.WaitingFor s ticket ∧ ticket ∈ s.dispatched ∧
 ∃ written, OwnerStatementResultCollection.collect s ticket = some (.committed written) ∧
 written.pending = ticket.saved ∧ written ∈ s.live.session.state.owner.history := by
 have actual := path_postHeld OwnerStatementResultLifecycle.start_postHeld path ticket .reported held (by decide)
 obtain ⟨written,collected,same⟩ := actual.2.2.2 rfl
 exact ⟨actual.2.2.1,actual.2.1,written,collected,same,
   (OwnerStatementResultOrigin.admitted_collected_commit (path_admitted entered path) collected).2⟩

theorem rooted_consume_sound
 (entered : OwnerStatementRegistryInvariant.Admitted realm authority policy store live)
 (path : Path capacity (start designation live) s)
 (run : OwnerStatementSourceCustodian.consume s ticket member principal = some next) :
 OwnerStatementCompletion.Accepted s.live.session next.live.session := by
 exact OwnerStatementSourceCustodian.consume_sound
   (OwnerStatementRegistryInvariant.admitted_valid (path_admitted entered path)).1 run

#print axioms refresh_exact
#print axioms refresh_enabled
#print axioms refresh_keeps
#print axioms collection_kept
#print axioms postHeld_kept
#print axioms path_refines
#print axioms path_admitted
#print axioms path_postHeld
#print axioms rooted_report_origin
#print axioms rooted_consume_sound
end MirroreaProofFirst.OwnerStatementHeldAuthority
