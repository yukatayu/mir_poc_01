import MirroreaProofFirstSourceFundingFrame
import SourceWorkerSupport
import MirroreaProofFirstOwnerInstallationHistory
import MirroreaProofFirstPublicOwnerReplay
open MirroreaProofFirst
namespace PublicCapturedReplay

-- Read-only finite replay of complete actual input/output records, globally
-- ordered by the trusted capture harness. Not a general physical refinement,
-- origin authentication or proof that hostile code cannot access private pipes.
instance : DecidableEq (OwnerImage.Image p a) := fun x y =>
  if same : OwnerPacketCodec.encode (OwnerFullCodec.image p a) x = OwnerPacketCodec.encode (OwnerFullCodec.image p a) y then
    isTrue (by
      have decoded := congrArg (OwnerPacketCodec.decode (OwnerFullCodec.image p a)) same
      rw [OwnerPacketCodec.roundtrip,OwnerPacketCodec.roundtrip] at decoded
      exact Option.some.inj decoded)
  else isFalse (fun equal => same (congrArg (OwnerPacketCodec.encode (OwnerFullCodec.image p a)) equal))

deriving instance DecidableEq for OwnerEvaluator.Assignment
deriving instance DecidableEq for OwnerOccurrence.Record
deriving instance DecidableEq for OwnerOccurrence.State
deriving instance DecidableEq for OwnerReservation.State
deriving instance DecidableEq for OwnerEndpoint.State
deriving instance DecidableEq for OwnerEndpointBudget.State

inductive Event where
  | begin
  | finish
  | rejectedReentry
  | source (ordinal : Nat)
  | owner (endpoint ordinal : Nat)

def file (root endpoint : String) (index : Nat) (kind : String) : String :=
  let number := toString index
  root ++ "/" ++ endpoint ++ "/" ++ String.ofList (List.replicate (3-number.length) '0') ++ number ++ "-" ++ kind ++ ".bin"

def read (root endpoint : String) (index : Nat) (kind : String) : IO ByteArray :=
  IO.FS.readBinFile (file root endpoint index kind)

def put (old : Fin p → α) (i : Fin p) (value : α) : Fin p → α := fun j => if j = i then value else old j

def classify (commands : List (Fin p × OwnerEndpoint.Command p a)) : Option (PublicOwnerReplay.Action p a) :=
  match commands with
  | [] => some .frame
  | [(i,command)] => some (.admin i command)
  | [(i,.owner (.reserve ticket)),(j,.freeze revision),(k,.owner .compute)] =>
      if i == j && i == k then some (.work i ticket revision) else none
  | _ => none

def cohort (root : String) (realm p a caller member principal scope capacity : Nat)
    (ownerCapacity : Vector Nat p) (events : List Event) : IO Unit := do
  unless p ≤ 64 && a ≤ 64 && capacity ≤ 512 && ownerCapacity.toList.all (· ≤ 64) do
    throw (IO.userError "native startup bounds")
  if hc : caller < p then
    if hm : member < a then
      let assigned : SourceInput.Assignment p a := ⟨realm,⟨⟨caller,hc⟩,⟨member,hm⟩,principal⟩⟩
      let bootstrap ← read root "source" 1 "input"
      let seed ← SourceWorker.decodeFrame (SourceInput.bootstrap a) bootstrap
      let mut source := PublicationCapacityDriver.initial (p:=p) (a:=a) capacity
      let mut owners : Fin p → OwnerEndpointBudget.State p a := fun _ => OwnerEndpointBudget.initial 512
      let assignments : Fin p → OwnerEvaluator.Assignment p := fun i => ⟨realm,i⟩
      let capacities : Fin p → Nat := fun i => ownerCapacity[i.val]
      let mut boundary := PublicOwnerReplay.initial (a:=a) assignments scope capacities 512
      let mut active := false
      let mut boundaryCount := 0
      let mut reentryCount := 0
      let mut ownerCommands : List (Fin p × OwnerEndpoint.Command p a) := []
      let mut installs : Fin p → List Nat := fun _ => []
      let mut freezes : Fin p → List Nat := fun _ => []
      let mut produced : List OwnerReceipt.Envelope := []
      let mut sourceCount := 1
      let mut ownerCount : Fin p → Nat := fun _ => 0
      let mut enteredCount := 0
      let mut queryCount := 0
      let mut executeCount := 0
      let mut vectorCount := 0
      for event in events do
        match event with
        | .begin =>
          unless !active do throw (IO.userError "nested outer boundary")
          active := true
          ownerCommands := []
        | .finish =>
          unless active do throw (IO.userError "completion without outer entry")
          let some action := classify ownerCommands | throw (IO.userError "public owner command shape")
          let some next := PublicOwnerReplay.advance boundary action | throw (IO.userError "public boundary checker refused")
          for i in List.finRange p do
            unless decide (next.val i = owners i) do throw (IO.userError "boundary state differs from actual byte replay")
            unless !OwnerEndpointBudget.held (next.val i).owner do throw (IO.userError "nonidle continuing boundary")
          boundary := next
          boundaryCount := boundaryCount+1
          active := false
        | .rejectedReentry =>
          unless active && (List.finRange p).any (fun i => OwnerEndpointBudget.held (owners i).owner) do
            throw (IO.userError "reentry was not during actual reserved work")
          reentryCount := reentryCount+1
        | .source ordinal =>
          unless active do throw (IO.userError "source IO outside outer operation")
          unless ordinal == sourceCount+1 do throw (IO.userError "source ordinal gap")
          sourceCount := ordinal
          let bytes ← read root "source" ordinal "input"
          let command ← SourceWorker.decodeFrame (SourceFundingQuery.checkedInput p a) bytes
          match command with
          | .inr (.inr (balances,input)) =>
            for i in List.finRange p do
              unless (balances[i.val]).val == (owners i).remaining do
                throw (IO.userError s!"actual balance vector mismatch at source{ordinal}/owner{i.val}")
            vectorCount := vectorCount+1
            match input with
            | .step (.enter i) =>
              unless !OwnerEndpointBudget.held (owners i).owner do
                throw (IO.userError "source entry while actual owner active")
              enteredCount := enteredCount+1
            | .step (.install i revision) =>
              unless (installs i).contains revision do throw (IO.userError "unobserved source install notice")
            | .step (.freeze i revision) | .step (.acknowledge i revision) =>
              unless (freezes i).contains revision do throw (IO.userError "unobserved source freeze notice")
            | .step (.arrival envelope) =>
              unless produced.contains envelope do throw (IO.userError "source arrival without actual production")
            | _ => pure ()
            executeCount := executeCount+1
          | _ => queryCount := queryCount+1
          let (next,response) := SourceFundingQuery.checkedExchange assigned scope seed source command
          let actual ← read root "source" ordinal "output"
          let expected := OwnerPacketCodec.encode (SourceFundingQuery.checkedReply command) response
          unless actual.data.toList == expected do throw (IO.userError s!"exact source bytes mismatch {ordinal}")
          source := next
        | .owner endpoint ordinal =>
          unless active do throw (IO.userError "owner IO outside outer operation")
          if hi : endpoint < p then
            let i : Fin p := ⟨endpoint,hi⟩
            unless ordinal == ownerCount i+1 do throw (IO.userError "owner ordinal gap")
            ownerCount := put ownerCount i ordinal
            let bytes ← read root s!"owner{endpoint}" ordinal "input"
            unless bytes.size ≤ 65536 && OwnerPayload.digitCheck 128 0 bytes.data.toList &&
                OwnerPayload.textCheck 256 bytes.data.toList do throw (IO.userError "owner input preflight")
            let some command := OwnerPacketCodec.decodeAt (OwnerEndpointWorker.command p a) 256 bytes.data.toList |
              throw (IO.userError "typed owner decode")
            ownerCommands := ownerCommands ++ [(i,command)]
            let (next,response) := OwnerEndpointBudget.transition ⟨realm,i⟩ scope ownerCapacity[i.val] (owners i) command
            let actual ← read root s!"owner{endpoint}" ordinal "output"
            unless actual.data.toList == OwnerPacketCodec.encode OwnerReservationWorker.replyCodec response do
              throw (IO.userError s!"exact owner bytes mismatch {endpoint}:{ordinal}")
            installs := put installs i (OwnerInstallationHistory.observe (installs i) command response)
            match command,response with
            | .freeze revision,.inl 12 => freezes := put freezes i (revision :: freezes i)
            | .owner .compute,.inr envelope => produced := envelope :: produced
            | _,_ => pure ()
            owners := put owners i next
          else throw (IO.userError "owner outside actual cohort")
      unless !active && boundaryCount > 0 do throw (IO.userError "incomplete public boundary trace")
      let some final := source.source | throw (IO.userError "source never launched")
      let session := final.publication.current.privateState.session
      unless source.suffix.isEmpty && final.dispatch.isNone && session.status == .ready &&
          session.remaining.isEmpty && session.state.source.waiting.isNone &&
          final.publication.barrier.announced == final.publication.barrier.published do
        throw (IO.userError "source not complete")
      for i in List.finRange p do
        let some actual := (owners i).owner | throw (IO.userError "actual owner never initialized")
        unless actual.owner.active.isNone && actual.fence == final.publication.barrier.published &&
            actual.owner.core.revision == final.publication.barrier.published &&
            final.publication.barrier.installed i == final.publication.barrier.published &&
            (final.publication.held i).isNone &&
            OwnerPacketCodec.encode (OwnerFullCodec.image p a) actual.owner.core.image ==
              OwnerPacketCodec.encode (OwnerFullCodec.image p a) (CustodyPublication.image final.publication.current) do
          throw (IO.userError "final actual owner/image/phase differs from source")
      unless enteredCount > 0 && produced.length > 0 && queryCount > 0 && vectorCount > 0 do
        throw (IO.userError "capture does not exercise work/production/query/funding")
      IO.println s!"PUBLIC_CAPTURE_OK boundaries={boundaryCount} reentries={reentryCount} source={sourceCount} owners={List.finRange p |>.map ownerCount} entries={enteredCount} produced={produced.length} queries={queryCount} executions={executeCount} vectors={vectorCount} sourceRemaining={source.remaining}"
    else throw (IO.userError "source member assignment")
  else throw (IO.userError "source caller assignment")
end PublicCapturedReplay
