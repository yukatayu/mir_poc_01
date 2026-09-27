import UnconfirmedLeaseCorrespondence
import StatementClosureHistory
import CohortWireEvidence
import CohortFaultProgress
import CohortRetiredProbe
import CohortHostCaptureValues
import CohortObservationCoverage
import HostReplyPrefix
import SourceEntryCapture
open MirroreaProofFirst
namespace LeaseWireCaptureReplay
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
    (ownerCapacity : Vector Nat p) (expectedObservations : Nat) (actualFault : CohortWireEvidence.Evidence) (events : List Event) (complete : Bool := true) : IO Unit := do
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
        if complete then throw (IO.userError "unknown-wire successor requires explicit incomplete capture")
        let mut cursor : CohortHostExecution.Cursor (assigned:=assigned) (scope:=scope)
            (bootstrap:=bootstrap) (capacity:=capacities) (sourceBudget:=capacity) (ownerBudget:=512) (seed:=seed) initial := .start initial
        let mut interrupted : Option (CohortFaultProgress.State initial) := none
        let mut probes : Option (CohortRetiredProbe.Certified initial) := none
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
        let mut retiredObservations := 0
        let mut unconfirmedObservations := 0
        let mut cohortStores := 0
        let mut retained : List OwnerReceipt.Envelope := []
        let mut head : Option (SourceFundingQuery.HeadRequest p × Bool × Nat) := none
        for event in events do
          let joined : SourceEntryPrefix.Certified initial := match interrupted with
            | some state => state.current.source
            | none => cursor.source.getD (SourceEntryPrefix.Certified.start initial)
          let journal : CohortCommitJournal.State p a := match probes with
            | none => match interrupted with
              | some state => state.current.current.state.cohort
              | none => cursor.journal
            | some probe => probe.journal
          if interrupted.isSome then
            match event with
            | .begin | .failedEnd | .cohortObserve .. | .retired .. | .gateEnter .. | .gateRelease => pure ()
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
            unless !journal.retired && !journal.gate && journal.pending.isEmpty do
              throw (IO.userError "public end before actual gate release/local stores")
            boundaries := boundaries+1
            active := false
            head := none
          | .gateEnter callId =>
            unless active do throw (IO.userError "gate acquisition outside public span")
            if journal.retired then
              let probe ← match probes with
                | some existing => pure existing
                | none =>
                  let live : CohortHostExecution.Live initial ← match interrupted with
                    | some state => pure state.current
                    | none => match cursor with
                      | .startup _ => throw (IO.userError "retired probe before launch")
                      | .live live => pure live
                  let some opened := CohortRetiredProbe.openProbe live | throw (IO.userError "retired probe before actual cleanup")
                  pure opened
              let some next := probe.advance (.acquire callId) | throw (IO.userError "retired physical guard acquisition refused")
              probes := some next
            else
              let some next := cursor.advance (.enter callId) | throw (IO.userError "actual outer gate acquisition refused")
              cursor := next
          | .gateRelease =>
            unless active do throw (IO.userError "gate release outside public span")
            if let some probe := probes then
              let some next := probe.advance .release | throw (IO.userError "retired physical guard release refused")
              probes := some next
            else if let some state := interrupted then
              let some next := state.release | throw (IO.userError "unconfirmed release before retirement or duplicate release")
              interrupted := some next
            else
              if !journal.retired then
                match joined.state.joined.history.current.joint with
                | .work _ _ _ _ checked =>
                  match checked.phase with
                  | .finished envelope =>
                    works := works+1
                    retained := envelope :: retained
                  | _ => throw (IO.userError "outer call ended before consumed source finish")
                | _ => pure ()
              let action := if journal.retired then CohortHostExecution.Event.releaseRetired else .close
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
            match probes with
            | some probe =>
              if matched : CohortHostObservation.memoryEq probe.journal.memory memory = true then
                have retainedCorrespondence := RetiredLeaseCorrespondence.probe_correspondence probe initialFresh
                if empty : probe.journal.pending = [] then
                  have observedMeaning := RetiredLeaseCorrespondence.observed_discharged probe initialFresh empty matched
                  dischargedObservations := dischargedObservations+1
                retiredObservations := retiredObservations+1
              else throw (IO.userError s!"actual cohort memory mismatch at observation {checkedFields}")
            | none =>
              match interrupted with
              | some stopped =>
                if matched : CohortHostObservation.memoryEq stopped.current.current.state.cohort.memory memory = true then
                  have observedMeaning := UnconfirmedLeaseCorrespondence.observed_last_known stopped initialFresh matched
                  unconfirmedObservations := unconfirmedObservations+1
                else throw (IO.userError s!"actual cohort memory mismatch at observation {checkedFields}")
              | none =>
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
            unless observation.gate == journal.gate && observation.retired == journal.retired &&
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
            unless active && journal.retired && !journal.gate && journal.call.isNone && joined.state.joined.stopped do throw (IO.userError "failed end without actual retirement and cleanup")
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
            if let some state := interrupted then
              let some next := state.retire | throw (IO.userError "unconfirmed retirement refused or duplicated")
              interrupted := some next
            else
              throw (IO.userError "wire retirement without retained actual unknown occurrence")
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
            let .live live := cursor | throw (IO.userError "unknown request before actual launch")
            let some armed := CohortHostFailure.arm live intent | throw (IO.userError "unconfirmed request not admissible from SAME full host anchor")
            let observedBody ← match cut with
              | .beforeWrite => pure []
              | .bodyLost | .rawCapture => do
                let actual ← IO.FS.readBinFile (System.FilePath.mk root / "wire-fault" / "observed-reply.bin")
                unless actual.data.toList == SharedWireLifetime.reply armed.wire.attempt.before intent.request do
                  throw (IO.userError "privileged actual unknown reply differs from SAME native request")
                pure actual.data.toList
            let stopped : CohortHostFailure.Stopped armed := match cut with
              | .beforeWrite => .beforeDelivery armed
              | .bodyLost => .afterDelivery armed
              | .rawCapture => .raw armed observedBody
            let proposed : CohortWireEvidence.Evidence := ⟨source,endpoint,ordinal,
              stopped.pending.residual.slot.applied,bytes.data.toList,
              SharedReplyRetention.observedBytes stopped.pending.residual.slot.phase⟩
            unless CohortWireEvidence.check actualFault proposed do
              throw (IO.userError "normalized interruption differs from actual full retained custody")
            interrupted := some (.stopped ⟨live,intent,armed,stopped⟩)
            -- Attempt bookkeeping is not settled source knowledge. The actual
            -- peer increments only after readiness, including capture failure.
            if source then
              match cut with
              | .beforeWrite => pure ()
              | .bodyLost | .rawCapture => sourceOrdinal := ordinal
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
          match interrupted with
          | some stopped =>
            have sameKnownCorrespondence := UnconfirmedLeaseCorrespondence.correspondence stopped initialFresh
            correspondingPrefixes := correspondingPrefixes+1
          | none =>
            match cursor with
            | .startup checked =>
              have actualStartup := checked.path.valid
              pure ()
            | .live live =>
              have sameCurrentCorrespondence := LeaseCohortCorrespondence.live_correspondence live initialFresh
              have allOwnerMemory := OwnerLeaseCompleteCorrespondence.complete sameCurrentCorrespondence
              correspondingPrefixes := correspondingPrefixes+1
        let some finalFault := interrupted | throw (IO.userError "unknown-wire capture omitted actual interruption")
        match finalFault with
        | .released _ => pure ()
        | _ => throw (IO.userError "unknown-wire capture omitted retirement/release")
        let joined := finalFault.current.source
        unless statements.isEmpty do
          throw (IO.userError "strict writer closure inventory mismatch")
        let some strictHistory := StatementClosureHistory.certify joined.state.joined.closed strictClosed |
          throw (IO.userError "strict statement history does not pair with actual native closes")
        have pointwiseClosed := strictHistory.paired
        IO.println s!"LAST_KNOWN_CORRESPONDENCE_OK prefixes={correspondingPrefixes} dischargedObservations={dischargedObservations}; same last-known native/receipt history; unknown physical residual remains separate, delayed cohort fields, current idle-owner leases, active owner/entry journals; privileged capture TCB, no general recovery or secrecy claim"
        IO.println s!"STRICT_HISTORY_PAIRED_OK closes={strictHistory.captured.length}; every actual close retains its own ordered statement path and occurrence positions"
        IO.println s!"WRITER_STATEMENTS_OK completedStores={statementCount} strictClosures={strictClosed.length}; actual ordered stores bound to SAME current native receipt, no value-only discharge"

        let journal : CohortCommitJournal.State p a := match probes with
          | none => finalFault.current.current.state.cohort
          | some probe => probe.journal
        unless checkedFields > 0 && cohortStores > 0 && !normalTeardown && journal.retired &&
            !journal.gate && journal.call.isNone do throw (IO.userError "missing whole-cohort fault observations/cleanup")
        if let some probe := probes then
          have retainedMemory := probe.keeps
          have fullStateValid := probe.valid
          IO.println "RETIRED_PROBES_OK actual physical gates; frozen full host origin and unfinished debt retained"
        IO.println s!"UNCONFIRMED_CORRESPONDENCE_OK observations={unconfirmedObservations} retiredProbeObservations={retiredObservations}; last-known host memory with separately retained actual wire custody"
        IO.println s!"COHORT_WIRE_FIELDS_OK observations={checkedFields} genericStores={cohortStores}; full typed memory, SAME startup/native/host cursor"
        IO.println s!"SOURCE_ENTRY_JOURNALS_OK actualCapturedIntervals={joined.state.stored.length} normalizedEvents={joined.state.events.length}; pure prefix/full memory handoff, snapshot assignment only, not public return"
        unless joined.state.joined.active.isNone do
          throw (IO.userError "incomplete joined host capture")
        unless !active do throw (IO.userError "unfinished outer span")
        unless sourceOrdinal >= 2 do throw (IO.userError "shared capture omitted launch")
        have generalHistory := joined.joined_valid
        have sourceEntryHistory := joined.valid
        have exactActionProjection := joined.exact_actions
        have rootedPhysical := finalFault.rooted
        have noPromotion := finalFault.no_absorb
        have retainedFullMemory := finalFault.keeps
        let origin := finalFault.origin
        let actual := SharedJointDriver.actual origin.anchor.source.state.joined.history.current.joint
        IO.println s!"COHORT_WIRE_RETIRED_PREFIX_OK writers={joined.state.joined.closed.length} observedPrefixes={observedPrefixes} completedBoundaries={boundaries} settledSourceOrdinal={actual.ordinal} attemptedSourceCounter={sourceOrdinal} retainedActiveSource={joined.state.active.isSome} knownWires={knownWires} residualWireActions={origin.stopped.pending.residual.actions.length} physicallyApplied={finalFault.slot.applied} retainedRaw={(SharedReplyRetention.observedBytes finalFault.slot.phase).isSome}; SAME full host, rooted unknown residual, actual retire/release/probes; no normal EOF"
      | _ => throw (IO.userError "actual funded launch refused")
   else throw (IO.userError "member index")
  else throw (IO.userError "caller index")
end LeaseWireCaptureReplay
