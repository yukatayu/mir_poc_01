import MirroreaProofFirstSourceEntryJournal
import WriterJournalCaptureReplay
open MirroreaProofFirst
namespace SourceEntryCapture
open OwnerCommitJournal SourceEntryJournal
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

def snapshotEq (x y : Option (SourcePublicationWorker.PrivateOutput p a)) : Bool :=
  OwnerPacketCodec.encode (OwnerCodecTree.optional (SourcePublicationWorker.output p a)) x ==
  OwnerPacketCodec.encode (OwnerCodecTree.optional (SourcePublicationWorker.output p a)) y

theorem snapshotEq_exact {x y : Option (SourcePublicationWorker.PrivateOutput p a)} : snapshotEq x y = true ↔ x = y := by
  constructor
  · intro same
    have bytes : OwnerPacketCodec.encode (OwnerCodecTree.optional (SourcePublicationWorker.output p a)) x =
        OwnerPacketCodec.encode (OwnerCodecTree.optional (SourcePublicationWorker.output p a)) y := by
      simpa only [snapshotEq,beq_iff_eq] using same
    have decoded := congrArg (OwnerPacketCodec.decode (OwnerCodecTree.optional (SourcePublicationWorker.output p a))) bytes
    simpa only [OwnerPacketCodec.roundtrip,Option.some.injEq] using decoded
  · intro same; subst y; simp [snapshotEq]

-- Executable start consumes actual before/after-claim observations. The native
-- history and complete source waiting ticket are supplied by the shared wire
-- consumer, not by an independent projected model or expected fixture JSON.
structure Open (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  context : Context base
  state : State base
  path : SourceEntryJournal.Runs context (initial context) state
  valid : Invariant context state
  beforeClaim : Memory p a
  claimed : OwnerLeasePrefix.claim beforeClaim (capacity context.endpoint) context.ticket = some context.staged
  source : PublicationInput.State p a
  sourcePresent : (SharedWireLifetime.native context.before.current).driver.source = some source
  waiting : (SourcePublicationWorker.project source).source.pending = some context.ticket
  beforeSnapshot : context.oldSnapshot = some (SourcePublicationWorker.project source)
  beforeData : context.staged.data = project ((SharedWireLifetime.native context.before.current).owners context.endpoint)

def start (history : SharedWireLifetime.History base) (endpoint : Fin p) (vector : Vector (Fin 513) p)
    (before staged : Memory p a) (observedSnapshot : Option (SourcePublicationWorker.PrivateOutput p a)) : Option (Open base) := do
  if before.entered then none else do
  if binding : dataEq before.data (project ((SharedWireLifetime.native history.current).owners endpoint)) = true then
    match present : (SharedWireLifetime.native history.current).driver.source with
    | none => none
    | some source =>
      match pending : (SourcePublicationWorker.project source).source.pending with
      | none => none
      | some ticket =>
        if origin : (SourcePublicationWorker.project source).dispatch.isNone && decide (ticket.place = endpoint.val) then
         match made : OwnerLeasePrefix.claim before (capacity endpoint) ticket with
         | none => none
         | some expected =>
           if same : memoryEq staged expected = true then
             if snapshotAt : snapshotEq observedSnapshot (some (SourcePublicationWorker.project source)) = true then
               have stagedAt := memoryEq_exact.mp same
               have claimed : OwnerLeasePrefix.claim before (capacity endpoint) ticket = some staged := by rw [stagedAt]; exact made
               let context : Context base := ⟨history,endpoint,ticket,vector,staged,observedSnapshot⟩
               some ⟨context,initial context,.nil,claimed_initial claimed,before,claimed,source,present,pending,
                 snapshotEq_exact.mp snapshotAt,(OwnerLeasePrefix.claim_preserves claimed).1.trans (dataEq_exact.mp binding)⟩
             else none
           else none
        else none
  else none

def Open.advance (opened : Open base) (action : SourceEntryJournal.Action) : Option (Open base) :=
  match checked : SourceEntryJournal.advance opened.context opened.state action with
  | none => none
  | some state =>
    let step := SourceEntryJournal.advance_sound checked
    some {opened with state:=state,path:=opened.path.step step,valid:=SourceEntryJournal.preserves opened.valid step}

-- Observation equality compares complete typed values, not hashes. A supplied
-- local action is separately checked against the independent phase transition.
def Open.observe (opened : Open base) (action : SourceEntryJournal.Action)
    (writer : Memory p a) (sourceSnapshot : Option (SourcePublicationWorker.PrivateOutput p a)) : Option (Open base) := do
  let next ← opened.advance action
  if memoryEq writer next.state.writer && snapshotEq sourceSnapshot next.state.sourceSnapshot then some next else none

theorem Open.accepted_dispatch (opened : Open base) (phase : AcceptedPhase opened.state.phase) :
    ∃ enteredSource,
      (SharedWireLifetime.native opened.state.history.current).driver.source = some enteredSource ∧
      enteredSource.dispatch = some ⟨opened.context.endpoint,⟨scope,opened.source.publication.barrier.installed opened.context.endpoint⟩,opened.context.ticket⟩ :=
  SourceEntryJournal.accepted_dispatch opened.valid phase opened.sourcePresent opened.waiting

#print axioms snapshotEq_exact
#print axioms start
#print axioms Open.advance
#print axioms Open.accepted_dispatch

-- Serialized observation input is generated only from actual host fields.
-- The final marker witnesses snapshot assignment, NOT source_send/public return.
inductive StoreKind where
  | notify | cancel | storeSnapshot
structure StoreObservation where
  kind : StoreKind
  writer : WriterJournalCaptureReplay.Observation
  sourceSnapshot : String
structure Evidence where
  beforeClaim : WriterJournalCaptureReplay.Observation
  claimed : WriterJournalCaptureReplay.Observation
  oldSnapshot : String
  stores : List StoreObservation

def readSnapshot (trees file : String) (p a : Nat) : IO (Option (SourcePublicationWorker.PrivateOutput p a)) := do
  SourceWorker.decodeFrame (OwnerCodecTree.optional (SourcePublicationWorker.output p a))
    (← IO.FS.readBinFile (trees ++ "/" ++ file ++ ".bin"))

def check (history : SharedWireLifetime.History base) (endpoint : Fin p) (vector : Vector (Fin 513) p)
    (bytes : List UInt8) (trees : String) (evidence : Evidence) : IO (Open base) := do
  let before ← WriterJournalCaptureReplay.decodeObservation p a trees evidence.beforeClaim
  let staged ← WriterJournalCaptureReplay.decodeObservation p a trees evidence.claimed
  let oldSnapshot ← readSnapshot trees evidence.oldSnapshot p a
  let some begun := start history endpoint vector before staged oldSnapshot |
    throw (IO.userError "source entry before/claim/native snapshot binding")
  let some known := begun.advance (.reply bytes) | throw (IO.userError "source entry known reply mismatch")
  let mut opened := known
  for store in evidence.stores do
    let memory ← WriterJournalCaptureReplay.decodeObservation p a trees store.writer
    let snapshot ← readSnapshot trees store.sourceSnapshot p a
    let action := match store.kind with
      | .notify => SourceEntryJournal.Action.notify
      | .cancel => .cancel
      | .storeSnapshot => .storeSnapshot
    let some next := opened.observe action memory snapshot | throw (IO.userError "source entry store order/value mismatch")
    opened := next
  unless !opened.state.retired && (opened.state.phase == .acceptedStored || opened.state.phase == .refusedStored) do
    throw (IO.userError "source entry captured snapshot assignment incomplete")
  return opened

-- Relative completeness of the executable start uses its actual resource,
-- native-data, and source-ticket conditions, not the desired journal invariant.
def StartAllowed (history : SharedWireLifetime.History base) (endpoint : Fin p)
    (before staged : Memory p a) (observedSnapshot : Option (SourcePublicationWorker.PrivateOutput p a)) : Prop :=
  before.entered = false ∧ before.data = project ((SharedWireLifetime.native history.current).owners endpoint) ∧
  ∃ source ticket, (SharedWireLifetime.native history.current).driver.source = some source ∧
    (SourcePublicationWorker.project source).source.pending = some ticket ∧
    (SourcePublicationWorker.project source).dispatch.isNone = true ∧ ticket.place = endpoint.val ∧
    OwnerLeasePrefix.claim before (capacity endpoint) ticket = some staged ∧
    observedSnapshot = some (SourcePublicationWorker.project source)

theorem start_complete (allowed : StartAllowed history endpoint before staged observedSnapshot) :
    (start history endpoint vector before staged observedSnapshot).isSome = true := by
  obtain ⟨inactive,binding,source,ticket,present,pending,idle,place,claimed,atSnapshot⟩ := allowed
  have dataMatches := dataEq_exact.mpr binding
  have memoryMatches := memoryEq_exact.mpr (Eq.refl staged)
  have snapshotMatches := snapshotEq_exact.mpr atSnapshot
  simp only [start,inactive,Bool.false_eq_true,↓reduceIte,dataMatches,↓reduceDIte]
  split
  · rename_i absent
    rw [present] at absent
    cases absent
  · rename_i actual sourceAt
    have sameSource := Option.some.inj (sourceAt.symm.trans present)
    subst actual
    split
    · rename_i absent
      rw [pending] at absent
      cases absent
    · rename_i actualTicket ticketAt
      have sameTicket := Option.some.inj (ticketAt.symm.trans pending)
      subst actualTicket
      simp only [idle,place,decide_true,Bool.and_self,↓reduceDIte]
      split
      · rename_i absent
        rw [claimed] at absent
        cases absent
      · rename_i expected made
        have sameMemory := Option.some.inj (made.symm.trans claimed)
        subst expected
        simp only [memoryMatches,snapshotMatches,↓reduceDIte,Option.isSome_some]

#print axioms start_complete

theorem start_sound {history : SharedWireLifetime.History base} {endpoint : Fin p}
    {vector : Vector (Fin 513) p} {before staged : Memory p a}
    {observedSnapshot : Option (SourcePublicationWorker.PrivateOutput p a)} {opened : Open base}
    (checked : start history endpoint vector before staged observedSnapshot = some opened) :
    StartAllowed history endpoint before staged observedSnapshot ∧
    opened.context.before = history ∧ opened.context.endpoint = endpoint ∧
    opened.context.vector = vector ∧ opened.state = initial opened.context ∧
    opened.context.staged = staged ∧ opened.beforeClaim = before ∧ opened.context.oldSnapshot = observedSnapshot := by
  unfold start at checked
  split at checked
  · cases checked
  · rename_i notEntered
    have inactive : before.entered = false := by cases flag : before.entered <;> simp_all
    split at checked
    · rename_i binding
      split at checked
      · cases checked
      · rename_i source present
        split at checked
        · cases checked
        · rename_i ticket pending
          split at checked
          · rename_i origin
            have admitted : (SourcePublicationWorker.project source).dispatch.isNone = true ∧ ticket.place = endpoint.val := by simpa using origin
            split at checked
            · cases checked
            · rename_i expected made
              split at checked
              · rename_i same
                split at checked
                · rename_i snapshotAt
                  cases Option.some.inj checked
                  have claimed : OwnerLeasePrefix.claim before (capacity endpoint) ticket = some staged := by
                    rw [memoryEq_exact.mp same]; exact made
                  exact ⟨⟨inactive,dataEq_exact.mp binding,source,ticket,present,pending,admitted.1,admitted.2,claimed,
                    snapshotEq_exact.mp snapshotAt⟩,rfl,rfl,rfl,rfl,rfl,rfl,rfl⟩
                · cases checked
              · cases checked
          · cases checked
    · cases checked

theorem start_exact : (start history endpoint vector before staged observedSnapshot).isSome = true ↔
    StartAllowed history endpoint before staged observedSnapshot := by
  constructor
  · intro hasValue
    cases got : start history endpoint vector before staged observedSnapshot with
    | none => simp [got] at hasValue
    | some opened => exact (start_sound got).1
  · exact start_complete

#print axioms start_sound
#print axioms start_exact

theorem start_wrong_endpoint {history : SharedWireLifetime.History base} {endpoint : Fin p}
    (present : (SharedWireLifetime.native history.current).driver.source = some source)
    (pending : (SourcePublicationWorker.project source).source.pending = some ticket)
    (wrong : ticket.place ≠ endpoint.val) :
    start history endpoint vector before staged observed = none := by
  cases checked : start history endpoint vector before staged observed with
  | none => rfl
  | some opened =>
    obtain ⟨_,_,actual,actualTicket,atSource,atTicket,_,atPlace,_,_⟩ := (start_sound checked).1
    have sameSource := Option.some.inj (atSource.symm.trans present)
    subst actual
    have sameTicket := Option.some.inj (atTicket.symm.trans pending)
    subst actualTicket
    exact False.elim (wrong atPlace)

theorem start_dispatched {history : SharedWireLifetime.History base}
    (present : (SharedWireLifetime.native history.current).driver.source = some source)
    (busy : (SourcePublicationWorker.project source).dispatch.isNone = false) :
    start history endpoint vector before staged observed = none := by
  cases checked : start history endpoint vector before staged observed with
  | none => rfl
  | some opened =>
    obtain ⟨_,_,actual,_,atSource,_,idle,_,_,_⟩ := (start_sound checked).1
    have sameSource := Option.some.inj (atSource.symm.trans present)
    subst actual
    rw [busy] at idle
    cases idle

#print axioms start_wrong_endpoint
#print axioms start_dispatched

theorem Open.advance_sound (checked : Open.advance before action = some after) :
    after.context = before.context ∧ SourceEntryJournal.Step before.context before.state action after.state := by
  unfold Open.advance at checked
  split at checked
  · cases checked
  · rename_i next ran
    cases Option.some.inj checked
    exact ⟨rfl,SourceEntryJournal.advance_sound ran⟩

theorem Open.observe_sound (checked : Open.observe before action memory observed = some after) :
    after.context = before.context ∧ SourceEntryJournal.Step before.context before.state action after.state ∧
    after.state.writer = memory ∧ after.state.sourceSnapshot = observed := by
  unfold Open.observe at checked
  cases ran : before.advance action with
  | none => simp [ran] at checked
  | some next =>
    simp only [ran] at checked
    change (if memoryEq memory next.state.writer && snapshotEq observed next.state.sourceSnapshot then some next else none) = some after at checked
    split at checked
    · rename_i values
      have same : memoryEq memory next.state.writer = true ∧ snapshotEq observed next.state.sourceSnapshot = true := by
        simpa using values
      cases Option.some.inj checked
      exact ⟨(Open.advance_sound ran).1,(Open.advance_sound ran).2,
        (memoryEq_exact.mp same.1).symm,(snapshotEq_exact.mp same.2).symm⟩
    · cases checked

theorem Open.advance_complete (before : Open base)
    (step : SourceEntryJournal.Step before.context before.state action next) :
    ∃ after, before.advance action = some after ∧ after.state = next := by
  have checked := SourceEntryJournal.advance_complete step
  unfold Open.advance
  split
  · rename_i noStep; rw [checked] at noStep; cases noStep
  · rename_i state ran
    have same := Option.some.inj (ran.symm.trans checked)
    exact ⟨_,rfl,same⟩

#print axioms Open.advance_sound
#print axioms Open.observe_sound
#print axioms Open.advance_complete
end SourceEntryCapture
