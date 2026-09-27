import CohortMemoryProvenance
open MirroreaProofFirst SharedHostCaptureReplay
namespace CohortAdministrativeCorrespondence
open CohortCommitJournal
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Administrative sets are HISTORICAL successful native operations. They are
-- not a claim that old revisions are current or that an owner is still usable.
inductive Fact (p : Nat) where
  | initialized (owner : Fin p)
  | frozen (owner : Fin p) (revision : Nat)
  | installed (owner : Fin p) (revision : Nat)

def Holds (memory : Memory p a) : Fact p → Prop
  | .initialized owner => owner ∈ memory.initialized
  | .frozen owner revision => (owner,revision) ∈ memory.freezes
  | .installed owner revision => (owner,revision) ∈ memory.installs

def Emitted (store : Store p a) (fact : Fact p) : Prop :=
  match store,fact with
  | .initialized owner,.initialized target => target = owner
  | .frozen owner revision,.frozen target version => target = owner ∧ version = revision
  | .installed owner revision,.installed target version => target = owner ∧ version = revision
  | _,_ => False

-- Independent native command/reply classification, not membership in a recipe.
def CommandFact (owner : Fin p) (command : OwnerEndpoint.Command p a)
    (reply : Sum Nat OwnerReceipt.Envelope) (fact : Fact p) : Prop :=
  match command,reply,fact with
  | .owner (.initialize _),.inl 10,.initialized target => target = owner
  | .owner (.initialize _),.inl 10,.installed target version => target = owner ∧ version = 0
  | .freeze revision,.inl 12,.frozen target version => target = owner ∧ version = revision
  | .owner (.install revision _),.inl 7,.installed target version => target = owner ∧ version = revision
  | _,_,_ => False

theorem mem_insert [DecidableEq α] (x y : α) (values : List α) :
    x ∈ CohortCommitJournal.insert y values ↔ x = y ∨ x ∈ values := by
  unfold CohortCommitJournal.insert
  split <;> simp_all

theorem write_holds (memory : Memory p a) (store : Store p a) (fact : Fact p) :
    Holds (write memory store) fact ↔ Holds memory fact ∨ Emitted store fact := by
  cases store <;> cases fact <;> simp [Holds,write,Emitted,mem_insert,Prod.mk.injEq,or_comm]

theorem fold_holds (memory : Memory p a) (stores : List (Store p a)) (fact : Fact p) :
    Holds (stores.foldl write memory) fact ↔ Holds memory fact ∨ ∃ store ∈ stores, Emitted store fact := by
  induction stores generalizing memory with
  | nil => simp
  | cons first rest ih =>
    simp only [List.foldl_cons,ih,write_holds,List.mem_cons]
    constructor
    · rintro ((prior | firstFact) | ⟨store,member,created⟩)
      · exact Or.inl prior
      · exact Or.inr ⟨first,Or.inl rfl,firstFact⟩
      · exact Or.inr ⟨store,Or.inr member,created⟩
    · rintro (prior | ⟨store,(same | member),created⟩)
      · exact Or.inl (Or.inl prior)
      · subst store; exact Or.inl (Or.inr created)
      · exact Or.inr ⟨store,member,created⟩

theorem payment_no_fact (payment : Option (PaidHeadPhase.Pending p)) (fact : Fact p) :
    ¬ ∃ store ∈ payment.toList.map (Store.payment (a:=a)), Emitted store fact := by
  cases payment <;> cases fact <;> simp [Emitted]

theorem owner_recipe_holds (memory : Memory p a) (owner : Fin p)
    (command : OwnerEndpoint.Command p a) (reply : Sum Nat OwnerReceipt.Envelope)
    (payment : Option (PaidHeadPhase.Pending p)) (fact : Fact p) :
    Holds ((recipe memory (.ownerReturned owner command reply payment)).foldl write memory) fact ↔
      Holds memory fact ∨ CommandFact owner command reply fact := by
  rw [fold_holds]
  cases command with
  | freeze revision =>
    cases reply with
    | inr envelope => cases payment <;> cases fact <;> simp [recipe,ownerStores,Emitted,CommandFact]
    | inl code =>
      by_cases success : code = 12
      · subst code; cases payment <;> cases fact <;> simp [recipe,ownerStores,Emitted,CommandFact]
      · cases payment <;> cases fact <;> simp [recipe,ownerStores,Emitted,CommandFact,success]
  | owner command =>
    cases reply with
    | inr envelope => cases command <;> cases payment <;> cases fact <;> simp [recipe,ownerStores,Emitted,CommandFact]
    | inl code =>
      by_cases initialized : code = 10
      · subst code
        by_cases present : owner ∈ memory.initialized
        all_goals cases command <;> cases payment <;> cases fact <;>
          simp_all [recipe,ownerStores,Emitted,CommandFact,Holds,or_comm]
      · by_cases installed : code = 7
        · subst code; cases command <;> cases payment <;> cases fact <;>
            simp [recipe,ownerStores,Emitted,CommandFact]
        · cases command <;> cases payment <;> cases fact <;>
            simp [recipe,ownerStores,Emitted,CommandFact,initialized,installed]

theorem finished_frames (memory : Memory p a) (envelope : Option OwnerReceipt.Envelope) (fact : Fact p) :
    Holds (CohortHostDebt.receiptResult memory (envelope.toList.map Receipt.workFinished)) fact ↔
      Holds memory fact := by
  cases envelope <;> cases fact <;> rfl

theorem source_frames (before after : SharedWireLifetime.History base)
    (input : SourceFundingQuery.CheckedInput p a) (memory : Memory p a) (fact : Fact p) :
    Holds (CohortHostDebt.receiptResult memory (CohortHostReceipt.sourceReceipts before after input)) fact ↔
      Holds memory fact := by
  rcases input with head | (owner | request)
  · rfl
  · rfl
  · simp only [CohortHostReceipt.sourceReceipts,List.singleton_append,CohortHostDebt.receiptResult]
    rw [finished_frames]
    cases paid : CohortHostReceipt.paymentOf before.current.mode <;>
      cases fact <;> simp [recipe,write,Holds]

-- Independent actual historical records: the relation does not inspect the
-- local store recipe or assume its result. Each record retains nativeReply.
def Corresponds (memory : Memory p a) (closed : List (RecordedClosed base)) : Prop :=
  ∀ fact, Holds memory fact ↔ ∃ record ∈ closed,
    CommandFact record.writer.bound.writer.owner record.writer.bound.writer.command
      record.writer.bound.writer.reply fact

theorem add_record (prior : Corresponds memory closed) (record : RecordedClosed base) :
    Corresponds
      ((recipe memory (CohortHostReceipt.ownerReceipt record.writer.bound)).foldl write memory)
      (record::closed) := by
  intro fact
  rw [CohortHostReceipt.ownerReceipt,owner_recipe_holds,prior fact]
  simp only [List.mem_cons,exists_eq_or_imp]
  exact or_comm

theorem interpret_corresponds (step : SourceEntryPrefix.Step before event after)
    (prior : Corresponds memory before.joined.closed) :
    Corresponds (CohortMemoryProvenance.interpret before after event memory) after.joined.closed := by
  cases step with
  | ordinary live idle allowed binding joinedRun =>
    cases SharedHostCaptureReplay.advance_sound joinedRun with
    | source =>
      intro fact
      exact (source_frames _ _ _ _ _).trans (prior fact)
    | owner => exact prior
    | observe => exact prior
    | confirmed live active same ran =>
      rename_i opened owner closed observed
      have added := add_record prior ⟨opened.position,before.joined.events.length,closed⟩
      have bound := finish_bound ran
      simpa only [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,active,
        CohortHostDebt.receiptResult,List.foldl_nil,bound] using added
    | localComplete => exact prior
    | retire => exact prior
  | claim => exact prior
  | reply live active binding localRun wireRun =>
    cases SharedHostCaptureReplay.advance_sound wireRun with
    | source =>
      intro fact
      exact (source_frames _ _ _ _ _).trans (prior fact)
  | store => exact prior
  | retireIdle live idle wireRun => cases SharedHostCaptureReplay.advance_sound wireRun; exact prior
  | retireActive live active wireRun localRun => cases SharedHostCaptureReplay.advance_sound wireRun; exact prior

theorem history_corresponds
    (path : CohortMemoryProvenance.NativeHistory first initial last result)
    (initialFacts : Corresponds initial first.joined.closed) :
    Corresponds result last.joined.closed := by
  induction path with
  | nil => exact initialFacts
  | step prior native ih => exact interpret_corresponds native ih

theorem live_corresponds (live : CohortHostExecution.Live base) :
    Corresponds (target live.current.state.cohort) live.current.state.inner.joined.closed := by
  apply history_corresponds (CohortMemoryProvenance.live_origin live)
  intro fact
  cases fact <;> simp [Holds,target,CohortHostStartup.anchor,CohortHostStartup.empty,write,
    SourceEntryPrefix.State.start,JoinedState.start]

theorem discharged_corresponds (live : CohortHostExecution.Live base)
    (empty : live.current.state.cohort.pending = []) :
    Corresponds live.current.state.cohort.memory live.current.state.inner.joined.closed := by
  simpa [target,empty] using live_corresponds live

-- The witness belongs to this SAME actual joined close list, its complete wire
-- history extends to the current history, and its native transition is retained.
-- No fresh proof object grants current-use authority for that historical fact.
theorem live_native_fact_iff (live : CohortHostExecution.Live base) (fact : Fact p) :
    Holds (target live.current.state.cohort) fact ↔
      ∃ record ∈ live.current.state.inner.joined.closed,
        CommandFact record.writer.bound.writer.owner record.writer.bound.writer.command
          record.writer.bound.writer.reply fact ∧
        Extends record.writer.bound.later live.current.state.inner.joined.history ∧
        OwnerEndpointBudget.transition ⟨assigned.realm,record.writer.bound.writer.owner⟩ scope
          (capacity record.writer.bound.writer.owner)
          ((SharedWireLifetime.native record.writer.bound.earlier.current).owners record.writer.bound.writer.owner)
          record.writer.bound.writer.command =
            (record.writer.bound.writer.nativeAfter,record.writer.bound.writer.reply) := by
  constructor
  · intro holds
    obtain ⟨record,member,created⟩ := (live_corresponds live fact).mp holds
    exact ⟨record,member,created,live.source.joined_valid.2 record member,record.writer.bound.nativeReply⟩
  · rintro ⟨record,member,created,_,_⟩
    exact (live_corresponds live fact).mpr ⟨record,member,created⟩

theorem absent_native_fact (live : CohortHostExecution.Live base) (fact : Fact p)
    (absent : ∀ record ∈ live.current.state.inner.joined.closed,
      ¬ CommandFact record.writer.bound.writer.owner record.writer.bound.writer.command
        record.writer.bound.writer.reply fact) :
    ¬ Holds (target live.current.state.cohort) fact := by
  intro present
  obtain ⟨record,member,created⟩ := (live_corresponds live fact).mp present
  exact absent record member created

-- Relative positive admission: every actual successful historical operation is
-- represented; the relation cannot be satisfied by silently discarding records.
theorem actual_fact_present (live : CohortHostExecution.Live base)
    (member : record ∈ live.current.state.inner.joined.closed)
    (created : CommandFact record.writer.bound.writer.owner record.writer.bound.writer.command
      record.writer.bound.writer.reply fact) :
    Holds (target live.current.state.cohort) fact :=
  (live_corresponds live fact).mpr ⟨record,member,created⟩

theorem initialized_install_zero (owner target : Fin p) (image : OwnerImage.Image p a) (version : Nat) :
    CommandFact owner (.owner (.initialize image)) (.inl 10) (.installed target version) ↔
      target = owner ∧ version = 0 := Iff.rfl

theorem closed_corresponds (live : CohortHostExecution.Live base)
    (returned : CohortHostPrefix.Step live.current.state .close after) :
    Corresponds after.cohort.memory after.inner.joined.closed := by
  apply history_corresponds (CohortMemoryProvenance.returned_memory live.current.path returned)
  intro fact
  cases fact <;> simp [Holds,target,CohortHostStartup.anchor,CohortHostStartup.empty,write,
    SourceEntryPrefix.State.start,JoinedState.start]

#print axioms write_holds
#print axioms fold_holds
#print axioms payment_no_fact
#print axioms owner_recipe_holds
#print axioms source_frames
#print axioms add_record
#print axioms interpret_corresponds
#print axioms history_corresponds
#print axioms live_corresponds
#print axioms discharged_corresponds
#print axioms live_native_fact_iff
#print axioms absent_native_fact
#print axioms actual_fact_present
#print axioms initialized_install_zero
#print axioms closed_corresponds
end CohortAdministrativeCorrespondence
