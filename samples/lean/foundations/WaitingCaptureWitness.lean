import MirroreaProofFirstWaitingWork
import PublicCapturedReplay
open MirroreaProofFirst
namespace WaitingCaptureWitness

instance : DecidableEq (PublicationInput.Command p a) := fun x y =>
  if same : OwnerPacketCodec.encode (PublicationInput.command p a) x = OwnerPacketCodec.encode (PublicationInput.command p a) y then
    isTrue (by
      have decoded := congrArg (OwnerPacketCodec.decode (PublicationInput.command p a)) same
      rw [OwnerPacketCodec.roundtrip,OwnerPacketCodec.roundtrip] at decoded
      exact Option.some.inj decoded)
  else isFalse (fun equal => same (congrArg (OwnerPacketCodec.encode (PublicationInput.command p a)) equal))

-- Actual complete driver from startup, carrying its inductively preserved
-- lifecycle certificate. Finite IO witnesses do not replace general proofs.
abbrev Certified (assigned : SourceInput.Assignment p a) (scope : Nat) (seed : SourceInput.Bootstrap a) :=
  {s : PublicationCapacityDriver.State p a // PublicationLifecycle.Invariant assigned scope seed s ∧ s.remaining ≤ 512}

def initial (assigned : SourceInput.Assignment p a) (scope : Nat) (seed : SourceInput.Bootstrap a)
    (capacity : Nat) (bound : capacity ≤ 512) : Certified assigned scope seed :=
  ⟨PublicationCapacityDriver.initial capacity,PublicationLifecycle.initial_invariant assigned scope seed capacity,bound⟩

def advance {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {seed : SourceInput.Bootstrap a} (s : Certified assigned scope seed) (input : SourceFundingQuery.CheckedInput p a) :
    Certified assigned scope seed :=
  ⟨(SourceFundingQuery.checkedExchange assigned scope seed s.val input).1,
    SourceFundingQuery.checked_preserves s.property.1 s.property.2⟩

-- Semantic refusal is a separate actual control, with no constructive claim.
-- Successful semantic entry must independently establish every waiting-entry
-- theorem premise on the ACTUAL retained driver and actual complete vector.
def verify {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
    {seed : SourceInput.Bootstrap a} (s : Certified assigned scope seed)
    (vector : Vector (Fin 513) p) (target : Fin p) (ordinal : Nat) : IO Unit := do
  match present : s.val.source with
  | none => throw (IO.userError "waiting witness has no source")
  | some source =>
    match enteredAt : PublicationInput.execute scope source (.enter target) with
    | none => IO.println s!"ACTUAL_ENTRY_SEMANTIC_REFUSAL ordinal={ordinal}; constructive branch not claimed"
    | some entered =>
      if empty : s.val.suffix = [] then
        if quota : 2 ≤ s.val.remaining then
          if funded : 3 ≤ (vector[target.val]).val then
            if released : PublicationLifecycle.releasePlan entered = [.finish target] then
              if held : entered.publication.held target = none then
                throw (IO.userError "semantic entry retained no release obligation")
              else
                match checked : PublicationCapacity.checkBoundPath assigned.realm scope entered [.finish target] with
                | none => throw (IO.userError "declarative release path fails")
                | some finished =>
                  if quiet : PublicationLifecycle.quietCheck finished = true then
                    have budget : s.val.remaining = (s.val.remaining-2)+2 := by omega
                    have result := WaitingWork.funded_waiting_entry s.property.1 present empty budget enteredAt released held
                      (PublicationCapacity.checkBoundPath_exact.mp checked) (PublicationLifecycle.quiet_exact.mp quiet) funded
                    let certifiedNext : Certified assigned scope seed :=
                      ⟨⟨some entered,s.val.remaining-1,[.finish target]⟩,by
                        have same : s.val.remaining-1 = (s.val.remaining-2)+1 := by omega
                        rw [same]
                        exact result.2.1,by have small := s.property.2; change s.val.remaining-1 ≤ 512; omega⟩
                    -- Runtime output names only public counters, never private image bytes.
                    IO.println s!"ACTUAL_WAITING_THEOREM ordinal={ordinal} actualQuota={s.val.remaining} ownerCredits={(vector[target.val]).val} actualSuffix=0 constructedSuffix={certifiedNext.val.suffix.length}"
                  else throw (IO.userError "release endpoint not quiet")
            else throw (IO.userError "release plan not one matching finish")
          else throw (IO.userError "constructive target funding missing")
        else throw (IO.userError "constructive source quota missing")
      else throw (IO.userError "constructive waiting witness has nonempty actual suffix")
end WaitingCaptureWitness
