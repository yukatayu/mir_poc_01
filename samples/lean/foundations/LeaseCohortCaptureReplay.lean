import LeaseCohortCorrespondence
import CohortStateCorrespondence
import StatementClosureHistory
import CohortHostCaptureValues
import CohortObservationCoverage
import HostReplyPrefix
import SourceEntryCapture
open MirroreaProofFirst
namespace LeaseCohortCaptureReplay
-- Normal teardown and source-program exhaustion are distinct. The default
-- retains the completed-program profile; explicit prefix mode still requires
-- all actual observations, debts, local intervals and teardown to be checked.
-- Empty writer/entry inventories are valid; fixed-fixture counts live outside.
open OwnerCommitJournal
set_option maxHeartbeats 1600000
inductive WireCut where
  | beforeWrite | bodyLost | rawCapture

inductive Event where
  | begin
  | gateEnter (callId : Nat)
  | gateRelease
  | bootstrapReturned
  | cohortCommit
  | cohortObserve (row : Nat) (value : CohortHostCaptureValues.Observation)
  | shutdown
  | finish
  | failedEnd
  | retired (writers : List WriterJournalCaptureReplay.Observation) (snapshot : String)
  | rejectedReentry
  | source (ordinal : Nat)
  | sourceWithEntry (ordinal : Nat) (evidence : SourceEntryCapture.Evidence)
  | sourceClaim (ordinal endpoint : Nat) (before claimed : WriterJournalCaptureReplay.Observation) (snapshot : String)
  | sourceStore (kind : SourceEntryPrefix.StoreKind) (endpoint : Nat) (memory : WriterJournalCaptureReplay.Observation) (snapshot : String)
  | sourceWireClaim (ordinal endpoint : Nat) (before claimed : WriterJournalCaptureReplay.Observation) (snapshot : String)
  | wireStop (source : Bool) (endpoint ordinal : Nat) (cut : WireCut) (writers : List WriterJournalCaptureReplay.Observation) (snapshot : String)
  | owner (endpoint ordinal : Nat) (before : WriterJournalCaptureReplay.Observation)
  | writerStatement (owner ordinal occurrence token : Nat) (kind : String) (memory : WriterJournalCaptureReplay.Observation)
  | writerObserve (owner : Nat) (memory : WriterJournalCaptureReplay.Observation)
  | writerReturn (owner : Nat) (memory : WriterJournalCaptureReplay.Observation)

def observationRows (events : List Event) : List Nat := events.filterMap fun
  | .cohortObserve row _ => some row
  | _ => none

theorem coverage_exact (checked : CohortObservationCoverage.check count (observationRows events) = true) :
    observationRows events = List.range count :=
  CohortObservationCoverage.complete_shape (CohortObservationCoverage.check_exact.mp checked)

def cohort (root trees : String) (realm p a caller member principal scope capacity : Nat)
    (ownerCapacity : Vector Nat p) (expectedObservations : Nat) (events : List Event) (complete : Bool := true)
    (requireProgramComplete : Bool := true) : IO Unit := do
  unless expectedObservations > 0 && CohortObservationCoverage.check expectedObservations (observationRows events) do
    throw (IO.userError "whole-cohort observation inventory missing/duplicated/reordered")
  if hc : caller < p then
   if hm : member < a then
    let assigned : SourceInput.Assignment p a := ⟨realm,⟨⟨caller,hc⟩,⟨member,hm⟩,principal⟩⟩
    let bootstrap ← SourceWorker.decodeFrame (SourceInput.bootstrap a) (← GateCaptureReplay.read root "source" 1 "input")
    let .inr (.inr (vector,.launch program)) ← SourceWorker.decodeFrame (SourceFundingQuery.checkedInput p a)
        (← GateCaptureReplay.read root "source" 2 "input") | throw (IO.userError "first native execution not launch")
    unless (List.finRange p).all (fun i => (vector[i.val]).val == 512) do throw (IO.userError "launch vector not actual fresh owner budgets")
    let capacities : Fin p → Nat := fun i => ownerCapacity[i.val]
    match launched : SourceInput.launch assigned bootstrap program with
    | none => throw (IO.userError "actual source launch rejected")
    | some seed =>
      match launchAt : SourceFundingInput.execute assigned scope bootstrap (PublicationCapacityDriver.initial capacity) (vector,.launch program) with
      | (next,.accepted) =>
        let launchChecked : {s : SharedFundedDriver.State assigned scope bootstrap capacities capacity 512 seed //
            s.mode = .prelude (fun _ => true)} ←
          if bound : ∀ i : Fin p, 512 = (vector[i.val]).val then
            pure (⟨SharedFundedDriver.fromLaunch bound launched launchAt,rfl⟩ :
              {s : SharedFundedDriver.State assigned scope bootstrap capacities capacity 512 seed //
                s.mode = .prelude (fun _ => true)})
          else throw (IO.userError "shared launch vector not fresh owner budgets")
        let initial : SharedFundedDriver.State assigned scope bootstrap capacities capacity 512 seed := launchChecked.val
        have initialFresh : initial.mode = .prelude (fun _ => true) := launchChecked.property
        have initialUnpaid : CohortHostReceipt.paymentOf initial.mode = none := by
          simp [initialFresh,CohortHostReceipt.paymentOf]
        let launchReply ← GateCaptureReplay.read root "source" 2 "output"
        unless launchReply.data.toList == OwnerPacketCodec.encode (PublicationCapacityDriver.reply p a)
            (.accepted,next.source.map SourcePublicationWorker.project) do throw (IO.userError "launch reply differs")
        unless complete do throw (IO.userError "whole-cohort successor currently accepts normal captures only; fault product not yet connected")
        let mut cursor : CohortHostExecution.Cursor (assigned:=assigned) (scope:=scope)
            (bootstrap:=bootstrap) (capacity:=capacities) (sourceBudget:=capacity) (ownerBudget:=512) (seed:=seed) initial := .start initial
        let mut interrupted : Option (HostReplyPrefix.Interrupted initial) := none
        let mut observedPrefixes := 0
        let mut statements : List (OwnerStatementJournal.Observation p a) := []
        let mut statementCount := 0
        let mut strictClosed : List (StatementClosureHistory.Captured p a) := []
        let mut sourceOrdinal := 0
        let mut owners : Fin p → Nat := fun _ => 0
        let mut active := false
        let mut boundaries := 0
        let mut knownWires := 0
        let mut works := 0
        let mut interiors := 0
        let mut normalTeardown := false
        let mut checkedFields := 0
        let mut correspondingPrefixes := 0
        let mut dischargedObservations := 0
        let mut cohortStores := 0
        let mut retained : List OwnerReceipt.Envelope := []
        let mut head : Option (SourceFundingQuery.HeadRequest p × Bool × Nat) := none
        for event in events do
          let joined := cursor.source.getD (SourceEntryPrefix.Certified.start initial)
          if interrupted.isSome then
            match event with
            | .begin | .failedEnd => pure ()
            | _ => throw (IO.userError "semantic event after interrupted wire")
          match event with
          | .begin =>
            unless !active do throw (IO.userError "nested outer span")
            active := true
            head := none
          | .finish =>
            unless joined.state.active.isNone do throw (IO.userError "outer return before source entry stores")
            unless joined.state.joined.active.isNone do throw (IO.userError "outer return before writer caller confirmation")
            unless active do throw (IO.userError "finish outside public span")
            unless !cursor.journal.gate && cursor.journal.pending.isEmpty do
              throw (IO.userError "public end before actual gate release/local stores")
            boundaries := boundaries+1
            active := false
            head := none
          | .gateEnter callId =>
            unless active do throw (IO.userError "gate acquisition outside public span")
            let some next := cursor.advance (.enter callId) | throw (IO.userError "actual outer gate acquisition refused")
            cursor := next
          | .gateRelease =>
            unless active do throw (IO.userError "gate release outside public span")
            match joined.state.joined.history.current.joint with
            | .work _ _ _ _ checked =>
              match checked.phase with
              | .finished envelope =>
                works := works+1
                retained := envelope :: retained
              | _ => throw (IO.userError "outer call ended before consumed source finish")
            | _ => pure ()
            let action := if cursor.journal.retired then CohortHostExecution.Event.releaseRetired else .close
            let some next := cursor.advance action | throw (IO.userError "actual gate release with undischarged local debt")
            cursor := next
          | .bootstrapReturned =>
            unless active && sourceOrdinal == 0 do throw (IO.userError "duplicate or misplaced bootstrap")
            let some next := cursor.advance .bootstrapReturned | throw (IO.userError "bootstrap confirmation outside gate")
            cursor := next
            sourceOrdinal := 1
          | .cohortCommit =>
            let some next := cursor.advance .commit | throw (IO.userError "cohort store without same-history debt")
            cursor := next
            cohortStores := cohortStores+1
          | .cohortObserve _ observation =>
            let memory ← CohortHostCaptureValues.decode p a trees observation
            match cursor with
            | .startup startup =>
              unless CohortHostObservation.memoryEq startup.state.journal.memory memory do
                throw (IO.userError s!"actual cohort memory mismatch at observation {checkedFields}")
            | .live live =>
              if matched : CohortHostObservation.memoryEq live.current.state.cohort.memory memory = true then
                if empty : live.current.state.cohort.pending = [] then
                  have observedMeaning := CohortStateCorrespondence.observed_discharged live initialUnpaid empty matched
                  dischargedObservations := dischargedObservations+1
              else throw (IO.userError s!"actual cohort memory mismatch at observation {checkedFields}")
            unless observation.gate == cursor.journal.gate && observation.retired == cursor.journal.retired &&
                observation.sourceOrdinal == sourceOrdinal do throw (IO.userError "actual cohort lifecycle/ordinal mismatch")
            checkedFields := checkedFields+1
          | .shutdown =>
            unless !active && !normalTeardown && complete && !cursor.journal.gate do
              throw (IO.userError "invalid normal cohort teardown")
            let some next := cursor.advance .retire | throw (IO.userError "teardown retirement refused")
            cursor := next
            normalTeardown := true
          | .rejectedReentry =>
            unless active do throw (IO.userError "reentry outside captured gate")
          | .failedEnd =>
            unless active && (joined.state.joined.stopped || interrupted.isSome) do throw (IO.userError "failed end without actual retirement")
            unless joined.state.joined.active.isNone do throw (IO.userError "fault profile has unfinished writer stores")
            active := false
            head := none
          | .retired observations snapshotFile =>
            unless active && observations.length == p do throw (IO.userError "retirement outside complete observation inventory")
            for i in List.finRange p do
              let some observation := observations[i.val]? | throw (IO.userError "retirement writer missing")
              let memory ← WriterJournalCaptureReplay.decodeObservation p a trees observation
              unless memoryEq memory (joined.state.writers i) do throw (IO.userError "retirement erased writer obligation")
            let snapshot ← SourceEntryCapture.readSnapshot trees snapshotFile p a
            let expected := match joined.state.active with
              | some opened => opened.entry.state.sourceSnapshot
              | none => (SharedWireLifetime.native joined.state.joined.history.current).driver.source.map SourcePublicationWorker.project
            unless SourceEntryCapture.snapshotEq snapshot expected do throw (IO.userError "retirement changed local snapshot")
            let some next := cursor.advance .retire | throw (IO.userError "retirement refused")
            cursor := next
          | .source ordinal | .sourceWithEntry ordinal _ =>
            match event with
            | .sourceWithEntry 2 _ => throw (IO.userError "entry evidence attached to launch")
            | .sourceWithEntry _ _ => throw (IO.userError "coalesced entry evidence requires expanded prefix")
            | _ => pure ()
            unless joined.state.joined.active.isNone do throw (IO.userError "source IO before writer caller confirmation")
            unless active && ordinal == sourceOrdinal+1 do throw (IO.userError "actual source ordinal/order")
            sourceOrdinal := ordinal
            if ordinal == 2 then
              let some next := cursor.advance .launchReturned | throw (IO.userError "launch/bootstrap handoff mismatch")
              cursor := next
            else
              let before := SharedJointDriver.actual joined.state.joined.history.current.joint
              unless before.ordinal+1 == ordinal do throw (IO.userError "proof-carrying source ordinal drift")
              let input ← SourceWorker.decodeFrame (SourceFundingQuery.checkedInput p a)
                (← GateCaptureReplay.read root "source" ordinal "input")
              match joined.state.joined.history.current.joint,input with
              | .closed closed,.inr (.inr (_,.step command)) =>
                unless JointDriver.sourcePreSend closed.actual.joint retained command do
                  throw (IO.userError s!"native source send bypassed pre-send provenance guard at source {ordinal}")
              | _,_ => pure ()
              let (_,response) := SourceFundingQuery.checkedExchange assigned scope bootstrap before.driver input
              let captured ← GateCaptureReplay.read root "source" ordinal "output"
              unless captured.data.toList == OwnerPacketCodec.encode (SourceFundingQuery.checkedReply input) response do
                throw (IO.userError s!"same-driver source reply mismatch {ordinal}")
              match input with
              | .inl request => head := some (request,SourceFundingQuery.matchesHead before.driver request,ordinal)
              | _ => head := none
              let event := match input with
                | .inr (.inr (_,.step (.enter _))) => SourceEntryPrefix.Event.reply input captured.data.toList
                | _ => .ordinary (.source input captured.data.toList)
              let some advanced := cursor.advance (.inner event) | throw (IO.userError s!"shared source/prefix checker refusal {ordinal}")
              cursor := advanced
              knownWires := knownWires+1
          | .sourceClaim ordinal endpoint beforeObservation claimedObservation snapshotFile
          | .sourceWireClaim ordinal endpoint beforeObservation claimedObservation snapshotFile =>
            unless active && ordinal == sourceOrdinal+1 do throw (IO.userError "source claim outside exact next source occurrence")
            let actualInput ← match event with
              | .sourceWireClaim .. => IO.FS.readBinFile (System.FilePath.mk root / "wire-fault" / "input.bin")
              | _ => GateCaptureReplay.read root "source" ordinal "input"
            let .inr (.inr (vector,.step (.enter target))) ← SourceWorker.decodeFrame (SourceFundingQuery.checkedInput p a)
              actualInput | throw (IO.userError "source claim native input is not entry")
            unless endpoint == target.val do throw (IO.userError "source claim endpoint mismatch")
            let before ← WriterJournalCaptureReplay.decodeObservation p a trees beforeObservation
            let claimed ← WriterJournalCaptureReplay.decodeObservation p a trees claimedObservation
            let snapshot ← SourceEntryCapture.readSnapshot trees snapshotFile p a
            let some next := cursor.advance (.inner (.claim target vector before claimed snapshot)) |
              throw (IO.userError "source claim full memory/vector/origin mismatch")
            cursor := next
          | .sourceStore kind endpoint observation snapshotFile =>
            let some opened := joined.state.active | throw (IO.userError "source store outside active entry")
            unless active && endpoint == opened.entry.context.endpoint.val do throw (IO.userError "source store endpoint/call mismatch")
            let memory ← WriterJournalCaptureReplay.decodeObservation p a trees observation
            let snapshot ← SourceEntryCapture.readSnapshot trees snapshotFile p a
            let some next := cursor.advance (.inner (.store kind memory snapshot)) |
              throw (IO.userError "source store prefix order/value mismatch")
            cursor := next
          | .wireStop source endpoint ordinal cut observations snapshotFile =>
            unless active && !complete && joined.state.joined.active.isNone && observations.length == p do
              throw (IO.userError "wire interruption outside exact active prefix")
            for i in List.finRange p do
              let some observation := observations[i.val]? | throw (IO.userError "interrupted writer inventory")
              let memory ← WriterJournalCaptureReplay.decodeObservation p a trees observation
              unless memoryEq memory (joined.state.writers i) do throw (IO.userError "wire interruption changed retained writer memory")
            let snapshot ← SourceEntryCapture.readSnapshot trees snapshotFile p a
            let expected := match joined.state.active with
              | some opened => opened.entry.state.sourceSnapshot
              | none => (SharedWireLifetime.native joined.state.joined.history.current).driver.source.map SourcePublicationWorker.project
            unless SourceEntryCapture.snapshotEq snapshot expected do throw (IO.userError "wire interruption changed local source snapshot")
            let bytes ← IO.FS.readBinFile (System.FilePath.mk root / "wire-fault" / "input.bin")
            let intent : HostReplyPrefix.Intent p a ← if source then do
                unless ordinal == sourceOrdinal+1 do throw (IO.userError "unknown source attempt ordinal")
                let input ← SourceWorker.decodeFrame (SourceFundingQuery.checkedInput p a) bytes
                match input with
                | .inr (.inr (_,.step (.enter target))) =>
                  unless endpoint == target.val do throw (IO.userError "unknown source attempt endpoint")
                  pure (.entry input)
                | _ => throw (IO.userError "unknown source capture is not entry")
              else do
                if hi : endpoint < p then
                  let i : Fin p := ⟨endpoint,hi⟩
                  unless ordinal == owners i+1 do throw (IO.userError "unknown owner attempt ordinal")
                  let command ← SourceWorker.decodeFrame (OwnerEndpointWorker.command p a) bytes
                  match command with
                  | .owner .compute => pure (.owner i command false (joined.state.writers i))
                  | _ => throw (IO.userError "unknown owner capture is not compute")
                else throw (IO.userError "unknown owner endpoint range")
            let some armed := HostReplyPrefix.arm joined intent | throw (IO.userError "unconfirmed request not admissible from SAME anchor")
            let observedBody ← match cut with
              | .beforeWrite => pure []
              | .bodyLost | .rawCapture => do
                let actual ← IO.FS.readBinFile (System.FilePath.mk root / "wire-fault" / "observed-reply.bin")
                unless actual.data.toList == SharedWireLifetime.reply armed.attempt.before intent.request do
                  throw (IO.userError "privileged actual unknown reply differs from SAME native request")
                pure actual.data.toList
            let pending : HostReplyPrefix.Pending armed := match cut with
              | .beforeWrite => .beforeDelivery armed
              | .bodyLost => .afterDelivery armed
              | .rawCapture => .rawStopped armed observedBody
            if stopped : pending.residual.slot.stopped = true then
              interrupted := some ⟨joined,intent,armed,pending,stopped⟩
            else throw (IO.userError "unconfirmed capture not retired")
          | .owner endpoint ordinal observedBefore =>
            unless statements.isEmpty do throw (IO.userError "previous writer statement inventory not consumed")
            unless joined.state.joined.active.isNone do throw (IO.userError "owner IO before prior writer caller confirmation")
            unless active && sourceOrdinal >= 2 do throw (IO.userError "owner outside captured post-launch gate")
            if hi : endpoint < p then
              let i : Fin p := ⟨endpoint,hi⟩
              unless ordinal == owners i+1 do throw (IO.userError "owner ordinal gap")
              owners := GateCaptureReplay.put owners i ordinal
              let earlier := joined.state.joined.history
              let command ← SourceWorker.decodeFrame (OwnerEndpointWorker.command p a)
                (← GateCaptureReplay.read root s!"owner{endpoint}" ordinal "input")
              let before := SharedJointDriver.actual earlier.current.joint
              let nativeResult := OwnerEndpointBudget.transition ⟨realm,i⟩ scope (capacities i) ((SharedWireLifetime.native earlier.current).owners i) command
              let response := nativeResult.2
              let captured ← GateCaptureReplay.read root s!"owner{endpoint}" ordinal "output"
              unless captured.data.toList == OwnerPacketCodec.encode OwnerReservationWorker.replyCodec response do
                throw (IO.userError s!"same-owner reply mismatch {endpoint}/{ordinal}")
              let paid := match command,head with
                | .freeze revision,some (.freeze target queried,true,atOrdinal) => i == target && revision == queried && atOrdinal == before.ordinal
                | _,_ => false
              let memory ← WriterJournalCaptureReplay.decodeObservation p a trees observedBefore
              unless maySend memory command do throw (IO.userError "joined writer lease admission")
              unless dataEq memory.data (project ((SharedWireLifetime.native earlier.current).owners i)) do
                throw (IO.userError "joined writer before-memory mismatch")
              let some advanced := cursor.advance (.inner (.ordinary (.owner i command paid captured.data.toList memory))) |
                throw (IO.userError s!"shared owner checker refusal {endpoint}/{ordinal}")
              cursor := advanced
              knownWires := knownWires+1
              head := none
              match (cursor.source.getD (SourceEntryPrefix.Certified.start initial)).state.joined.history.current.joint with
              | .work _ _ _ _ ⟨.computed _,_,path⟩ =>
                -- At this point the SAME certified path has no finish event.
                have notFinished := WorkOccurrence.computed_no_finish path
                interiors := interiors+1
              | _ => pure ()
            else throw (IO.userError "owner endpoint")
          | .writerStatement owner ordinal occurrence token kind observation =>
            let some opened := joined.state.joined.active | throw (IO.userError "physical statement outside actual writer interval")
            unless owner == opened.bound.writer.owner.val && token == statementCount &&
                ordinal == owners opened.bound.writer.owner && occurrence == joined.state.joined.closed.length do
              throw (IO.userError "physical statement owner/token order")
            let memory ← WriterJournalCaptureReplay.decodeObservation p a trees observation
            let store : Store p a ← match kind with
              | "credits" => pure (.credits memory.data.credits)
              | "image" =>
                let some image := memory.data.image | throw (IO.userError "physical image store has no image")
                pure (.image image)
              | "revision" => pure (.revision memory.data.revision)
              | "keys" => pure (.keys memory.data.keys)
              | "releaseLease" => pure .releaseLease
              | "clearEntered" => pure .clearEntered
              | _ => throw (IO.userError "unknown physical writer statement")
            let observed : OwnerStatementJournal.Observation p a := ⟨store,memory⟩
            let supplied := statements ++ [observed]
            let some _checked := BoundWriterStatements.checkPrefix opened.bound supplied |
              throw (IO.userError "physical writer statement differs from same native recipe prefix")
            statements := supplied
            statementCount := statementCount+1
          | .writerObserve owner observation =>
            let some opened := joined.state.joined.active | throw (IO.userError "joined writer sample outside interval")
            unless owner == opened.bound.writer.owner.val do throw (IO.userError "joined writer borrowed sample")
            let memory ← WriterJournalCaptureReplay.decodeObservation p a trees observation
            let some next := cursor.advance (.inner (.ordinary (.observe opened.bound.writer.owner memory))) | throw (IO.userError "joined writer sample not ordered prefix")
            cursor := next
            observedPrefixes := observedPrefixes+1
          | .writerReturn owner observation =>
            let some opened := joined.state.joined.active | throw (IO.userError "joined writer return outside interval")
            unless owner == opened.bound.writer.owner.val do throw (IO.userError "joined writer borrowed return")
            let memory ← WriterJournalCaptureReplay.decodeObservation p a trees observation
            let some strict := BoundWriterStatements.checkComplete opened.bound statements memory |
              throw (IO.userError "writer returned before all actual statement occurrences")
            -- The pre-confirm cursor supplies both historical occurrence coordinates.
            -- Every certificate is checked again against its actual recorded close
            -- by StatementClosureHistory.certify, with a general pointwise theorem.
            strictClosed := ⟨opened.position,joined.state.joined.events.length,statements⟩ :: strictClosed
            statements := []
            let some completed := cursor.advance (.inner (.ordinary (.confirmed opened.bound.writer.owner memory))) | throw (IO.userError "joined writer incomplete stores at caller continuation")
            cursor := completed
          match cursor with
          | .startup checked =>
            have actualStartup := checked.path.valid
            pure ()
          | .live live =>
            have sameCurrentCorrespondence := LeaseCohortCorrespondence.live_correspondence live initialFresh
            correspondingPrefixes := correspondingPrefixes+1
        let some joined := cursor.source | throw (IO.userError "cohort omitted actual launch")
        unless statements.isEmpty do
          throw (IO.userError "strict writer closure inventory mismatch")
        let some strictHistory := StatementClosureHistory.certify joined.state.joined.closed strictClosed |
          throw (IO.userError "strict statement history does not pair with actual native closes")
        have pointwiseClosed := strictHistory.paired
        IO.println s!"KNOWN_STATE_CORRESPONDENCE_OK prefixes={correspondingPrefixes} dischargedObservations={dischargedObservations}; same current native/receipt history, delayed cohort fields, current idle-owner leases, active owner/entry journals; privileged capture TCB, no general recovery or secrecy claim"
        IO.println s!"STRICT_HISTORY_PAIRED_OK closes={strictHistory.captured.length}; every actual close retains its own ordered statement path and occurrence positions"
        IO.println s!"WRITER_STATEMENTS_OK completedStores={statementCount} strictClosures={strictClosed.length}; actual ordered stores bound to SAME current native receipt, no value-only discharge"
        unless checkedFields > 0 && cohortStores > 0 && normalTeardown do throw (IO.userError "missing whole-cohort observations/teardown")
        IO.println s!"COHORT_FIELDS_OK observations={checkedFields} genericStores={cohortStores}; full typed memory, SAME startup/native/host cursor"
        IO.println s!"SOURCE_ENTRY_JOURNALS_OK actualCapturedIntervals={joined.state.stored.length} normalizedEvents={joined.state.events.length}; pure prefix/full memory handoff, snapshot assignment only, not public return"
        unless joined.state.joined.active.isNone do
          throw (IO.userError "incomplete joined host capture")
        unless !active do throw (IO.userError "unfinished outer span")
        unless sourceOrdinal >= 2 do throw (IO.userError "shared capture omitted launch")
        have generalHistory := joined.joined_valid
        have sourceEntryHistory := joined.valid
        have exactActionProjection := joined.exact_actions
        if let some record := interrupted then
          if complete then throw (IO.userError "unconfirmed wire is not normal completion")
          have rootedPhysical := record.rooted
          have noPromotion := record.no_absorb
          let actual := SharedJointDriver.actual record.anchor.state.joined.history.current.joint
          IO.println s!"SHARED_HOST_UNCONFIRMED_PREFIX_OK writers={record.anchor.state.joined.closed.length} observedPrefixes={observedPrefixes} completedBoundaries={boundaries} settledSourceOrdinal={actual.ordinal} retainedActiveSource={record.anchor.state.active.isSome} knownWires={knownWires} anchoredWireActions={record.anchor.state.joined.history.actions.length} residualWireActions={record.pending.residual.actions.length} physicallyApplied={record.pending.residual.slot.applied} retainedRaw={(SharedReplyRetention.observedBytes record.pending.residual.slot.phase).isSome}; SAME rooted path, no public completion, no second roundtrip, no normal EOF"
        else if joined.state.joined.stopped && !normalTeardown then
          if complete then throw (IO.userError "retired prefix is not normal completion")
          let actual := SharedJointDriver.actual joined.state.joined.history.current.joint
          IO.println s!"SHARED_HOST_RETIRED_PREFIX_OK writers={joined.state.joined.closed.length} observedPrefixes={observedPrefixes} completedBoundaries={boundaries} actualSourceOrdinal={actual.ordinal} retainedActiveSource={joined.state.active.isSome} historicalEntries={joined.state.stored.length} knownWires={knownWires} wireActions={joined.state.joined.history.actions.length}; fault prefix only, no public success or normal EOF"
        else
         unless joined.state.active.isNone do throw (IO.userError "live prefix omitted source stores")
         let .closed closed := joined.state.joined.history.current.joint | throw (IO.userError "interior escaped at capture end")
         let final := closed.actual.joint.actual.source
         if complete && requireProgramComplete then
          let .ordinary := joined.state.joined.history.current.mode | throw (IO.userError "completed shared capture has outstanding funding phase")
          unless final.dispatch.isNone && final.publication.current.privateState.session.remaining.isEmpty &&
              final.publication.current.privateState.session.state.source.waiting.isNone && closed.actual.driver.suffix.isEmpty do
            throw (IO.userError "source not complete")
         IO.println s!"SHARED_HOST_CAPTURE_OK writers={joined.state.joined.closed.length} observedPrefixes={observedPrefixes} boundaries={boundaries} sameDriverWorks={works} computedInterior={interiors} actualSourceOrdinal={closed.actual.ordinal} sourceRemaining={closed.actual.driver.remaining} sharedFundingEvents={joined.state.joined.history.current.events.length} knownWires={knownWires} wireActions={joined.state.joined.history.actions.length}; prefix={ !(complete && requireProgramComplete) }"
      | _ => throw (IO.userError "actual funded launch refused")
   else throw (IO.userError "member index")
  else throw (IO.userError "caller index")
end LeaseCohortCaptureReplay
