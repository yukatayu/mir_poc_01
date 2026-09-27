import MirroreaProofFirstSharedWireLifetime
import GateCaptureReplay
open MirroreaProofFirst
namespace SharedCaptureReplay
set_option maxHeartbeats 800000

-- Reads each actual event once. The driver and owners used for native reply
-- comparison are exactly the values in the single proof-carrying state.
-- Captured outer spans, successful stable natives and byte custody are TCB;
-- this does not model unknown pre-reply faults or host instruction interleaving.
def cohort (root : String) (realm p a caller member principal scope capacity : Nat)
    (ownerCapacity : Vector Nat p) (events : List GateCaptureReplay.Event) (complete : Bool := true) : IO Unit := do
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
        let initial : SharedFundedDriver.State assigned scope bootstrap capacities capacity 512 seed ←
          if bound : ∀ i : Fin p, 512 = (vector[i.val]).val then
            pure (SharedFundedDriver.fromLaunch bound launched launchAt)
          else throw (IO.userError "shared launch vector not fresh owner budgets")
        let launchReply ← GateCaptureReplay.read root "source" 2 "output"
        unless launchReply.data.toList == OwnerPacketCodec.encode (PublicationCapacityDriver.reply p a)
            (.accepted,next.source.map SourcePublicationWorker.project) do throw (IO.userError "launch reply differs")
        let mut history : SharedWireLifetime.History (assigned:=assigned) (scope:=scope)
            (bootstrap:=bootstrap) (capacity:=capacities) (sourceBudget:=capacity) (ownerBudget:=512) (seed:=seed) initial := .start initial
        let mut sourceOrdinal := 1
        let mut owners : Fin p → Nat := fun _ => 0
        let mut active := false
        let mut boundaries := 0
        let mut knownWires := 0
        let mut works := 0
        let mut interiors := 0
        let mut retained : List OwnerReceipt.Envelope := []
        let mut head : Option (SourceFundingQuery.HeadRequest p × Bool × Nat) := none
        for event in events do
          match event with
          | .begin =>
            unless !active do throw (IO.userError "nested outer span")
            active := true
            head := none
          | .finish =>
            unless active do throw (IO.userError "finish outside public span")
            match history.current.joint with
            | .work _ _ _ _ checked =>
              match checked.phase with
              | .finished envelope =>
                works := works+1
                retained := envelope :: retained
              | _ => throw (IO.userError "outer call ended before consumed source finish")
            | _ => pure ()
            if sourceOrdinal >= 2 then
              let some closed := SharedWireLifetime.History.finishStep history | throw (IO.userError "unclosed actual work")
              history := closed
            boundaries := boundaries+1
            active := false
            head := none
          | .rejectedReentry =>
            unless active do throw (IO.userError "reentry outside captured gate")
          | .source ordinal =>
            unless active && ordinal == sourceOrdinal+1 do throw (IO.userError "actual source ordinal/order")
            sourceOrdinal := ordinal
            if ordinal != 2 then
              let before := SharedJointDriver.actual history.current.joint
              unless before.ordinal+1 == ordinal do throw (IO.userError "proof-carrying source ordinal drift")
              let input ← SourceWorker.decodeFrame (SourceFundingQuery.checkedInput p a)
                (← GateCaptureReplay.read root "source" ordinal "input")
              match history.current.joint,input with
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
              let some advanced := SharedWireLifetime.History.knownStep history (.source input) captured.data.toList | throw (IO.userError s!"shared source checker refusal {ordinal}")
              history := advanced
              knownWires := knownWires+1
          | .owner endpoint ordinal =>
            unless active && sourceOrdinal >= 2 do throw (IO.userError "owner outside captured post-launch gate")
            if hi : endpoint < p then
              let i : Fin p := ⟨endpoint,hi⟩
              unless ordinal == owners i+1 do throw (IO.userError "owner ordinal gap")
              owners := GateCaptureReplay.put owners i ordinal
              let command ← SourceWorker.decodeFrame (OwnerEndpointWorker.command p a)
                (← GateCaptureReplay.read root s!"owner{endpoint}" ordinal "input")
              let before := SharedJointDriver.actual history.current.joint
              let (_,response) := OwnerEndpointBudget.transition ⟨realm,i⟩ scope (capacities i) (before.joint.actual.owners i) command
              let captured ← GateCaptureReplay.read root s!"owner{endpoint}" ordinal "output"
              unless captured.data.toList == OwnerPacketCodec.encode OwnerReservationWorker.replyCodec response do
                throw (IO.userError s!"same-owner reply mismatch {endpoint}/{ordinal}")
              let paid := match command,head with
                | .freeze revision,some (.freeze target queried,true,atOrdinal) => i == target && revision == queried && atOrdinal == before.ordinal
                | _,_ => false
              let some advanced := SharedWireLifetime.History.knownStep history (.owner i command paid) captured.data.toList | throw (IO.userError s!"shared owner checker refusal {endpoint}/{ordinal}")
              history := advanced
              knownWires := knownWires+1
              head := none
              match history.current.joint with
              | .work _ _ _ _ ⟨.computed _,_,path⟩ =>
                -- At this point the SAME certified path has no finish event.
                have notFinished := WorkOccurrence.computed_no_finish path
                interiors := interiors+1
              | _ => pure ()
            else throw (IO.userError "owner endpoint")
        unless !active do throw (IO.userError "unfinished outer span")
        unless sourceOrdinal >= 2 do throw (IO.userError "shared capture omitted launch")
        let .closed closed := history.current.joint | throw (IO.userError "interior escaped at capture end")
        let final := closed.actual.joint.actual.source
        if complete then
          let .ordinary := history.current.mode | throw (IO.userError "completed shared capture has outstanding funding phase")
          unless final.dispatch.isNone && final.publication.current.privateState.session.remaining.isEmpty &&
              final.publication.current.privateState.session.state.source.waiting.isNone && closed.actual.driver.suffix.isEmpty do
            throw (IO.userError "source not complete")
        IO.println s!"SHARED_CAPTURE_OK boundaries={boundaries} sameDriverWorks={works} computedInterior={interiors} actualSourceOrdinal={closed.actual.ordinal} sourceRemaining={closed.actual.driver.remaining} sharedFundingEvents={history.current.events.length} knownWires={knownWires} wireActions={history.actions.length}; prefix={ !complete }"
      | _ => throw (IO.userError "actual funded launch refused")
   else throw (IO.userError "member index")
  else throw (IO.userError "caller index")
end SharedCaptureReplay
