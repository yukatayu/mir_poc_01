import CohortMemoryProvenance
open MirroreaProofFirst SharedHostCaptureReplay
namespace CohortProductionCorrespondence
open CohortCommitJournal CohortHostReceipt
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

def framesProduced : Store p a → Bool
  | .produced _ => false
  | _ => true

theorem write_produced (frame : framesProduced store = true) :
    (write memory store).produced = memory.produced := by
  cases store <;> simp_all [framesProduced,write]

theorem fold_produced (stores : List (Store p a))
    (frame : ∀ store ∈ stores, framesProduced store = true) (memory : Memory p a) :
    (stores.foldl write memory).produced = memory.produced := by
  induction stores generalizing memory with
  | nil => rfl
  | cons first rest ih =>
    exact (ih (fun store member => frame store (List.mem_cons_of_mem first member))
      (write memory first)).trans (write_produced (frame first List.mem_cons_self))

theorem owner_store_frames (member : store ∈ ownerStores memory owner command reply payment) :
    framesProduced store = true := by
  unfold ownerStores at member
  simp only [List.mem_append] at member
  rcases member with (initialized | paid) | tagged
  · split at initialized
    · split at initialized <;> simp_all [framesProduced]
    · simp at initialized
  · obtain ⟨value,_,same⟩ := List.mem_map.mp paid
    subst store
    rfl
  · split at tagged <;> simp_all [framesProduced]

theorem owner_recipe_produced (memory : Memory p a) (owner : Fin p)
    (command : OwnerEndpoint.Command p a) (reply : Sum Nat OwnerReceipt.Envelope)
    (payment : Option (PaidHeadPhase.Pending p)) :
    ((recipe memory (.ownerReturned owner command reply payment)).foldl write memory).produced =
      memory.produced := fold_produced _ (fun _ member => owner_store_frames member) memory

theorem finished_produced (memory : Memory p a) (selected : Option OwnerReceipt.Envelope)
    (envelope : OwnerReceipt.Envelope) :
    envelope ∈ (CohortHostDebt.receiptResult memory (selected.toList.map Receipt.workFinished)).produced ↔
      envelope ∈ memory.produced ∨ selected = some envelope := by
  cases selected with
  | none => simp [CohortHostDebt.receiptResult]
  | some value =>
    by_cases present : value ∈ memory.produced
    all_goals simp_all [CohortHostDebt.receiptResult,recipe,write,CohortCommitJournal.insert,or_comm,eq_comm]

theorem source_produced (before after : SharedWireLifetime.History base)
    (request : SourceFundingInput.Request p a) (memory : Memory p a) (envelope : OwnerReceipt.Envelope) :
    envelope ∈ (CohortHostDebt.receiptResult memory (sourceReceipts before after (.inr (.inr request)))).produced ↔
      envelope ∈ memory.produced ∨ finishedEnvelope after.current.joint = some envelope := by
  simp only [sourceReceipts,List.singleton_append,CohortHostDebt.receiptResult]
  rw [finished_produced]
  cases paid : paymentOf before.current.mode <;> simp [recipe,write]

-- The retained envelope came from an actual native compute and matching source
-- finish inside a full rooted wire history extending to this SAME current one.
-- It is not public completion, delivery, current availability, or authority.
def ProducedAt (state : SourceEntryPrefix.State base) (envelope : OwnerReceipt.Envelope) : Prop :=
  ∃ history : SharedWireLifetime.History base,
    Extends history state.joined.history ∧ FinishedProduction history.current.joint envelope

theorem ProducedAt.extend (prior : ProducedAt before envelope)
    (later : Extends before.joined.history after.joined.history) : ProducedAt after envelope := by
  obtain ⟨history,inside,production⟩ := prior
  exact ⟨history,inside.trans later,production⟩

theorem interpret_produced (before after : SourceEntryPrefix.State base)
    (event : SourceEntryPrefix.Event p a) (memory : Memory p a)
    (present : envelope ∈ (CohortMemoryProvenance.interpret before after event memory).produced) :
    envelope ∈ memory.produced ∨ finishedEnvelope after.joined.history.current.joint = some envelope := by
  cases event with
  | ordinary event =>
    cases event with
    | source input bytes =>
      rcases input with head | (owner | request)
      · exact Or.inl present
      · exact Or.inl present
      · exact (source_produced before.joined.history after.joined.history request memory envelope).mp present
    | confirmed owner observed =>
      cases active : before.joined.active with
      | none => simpa [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,active,
          CohortHostDebt.receiptResult] using Or.inl present
      | some opened =>
        exact Or.inl (by simpa [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,active,
          CohortHostDebt.receiptResult,ownerReceipt,owner_recipe_produced] using present)
    | owner | observe | localComplete | retire => exact Or.inl present
  | reply input bytes =>
    rcases input with head | (owner | request)
    · exact Or.inl present
    · exact Or.inl present
    · exact (source_produced before.joined.history after.joined.history request memory envelope).mp present
  | claim | store | retire => exact Or.inl present

theorem interpret_origin {before after : SourceEntryPrefix.State base} {memory : Memory p a}
    (step : SourceEntryPrefix.Step before event after)
    (prior : ∀ envelope ∈ memory.produced, ProducedAt before envelope) :
    ∀ envelope ∈ (CohortMemoryProvenance.interpret before after event memory).produced,
      ProducedAt after envelope := by
  intro envelope present
  rcases interpret_produced before after event memory present with old | actual
  · exact (prior envelope old).extend (SourceEntryPrefix.step_extends step)
  · exact ⟨after.joined.history,Extends.refl _,finishedEnvelope_origin actual⟩

theorem history_origin {first last : SourceEntryPrefix.State base} {initial result : Memory p a}
    (path : CohortMemoryProvenance.NativeHistory first initial last result)
    (initialProduction : ∀ envelope ∈ initial.produced, ProducedAt first envelope) :
    ∀ envelope ∈ result.produced, ProducedAt last envelope := by
  induction path with
  | nil => exact initialProduction
  | step prior native ih => exact interpret_origin native ih

theorem live_origin (live : CohortHostExecution.Live base) :
    ∀ envelope ∈ (target live.current.state.cohort).produced,
      ProducedAt live.current.state.inner envelope := by
  apply history_origin (CohortMemoryProvenance.live_origin live)
  simp [target,CohortHostStartup.anchor,CohortHostStartup.empty,write]

theorem discharged_origin (live : CohortHostExecution.Live base)
    (empty : live.current.state.cohort.pending = []) :
    ∀ envelope ∈ live.current.state.cohort.memory.produced,
      ProducedAt live.current.state.inner envelope := by
  simpa [target,empty] using live_origin live

-- Any actual selected finished envelope appears in the receipt target for an
-- arbitrary before-memory. This is not a fixed-example decide or all-reject.
theorem finished_present (before after : SharedWireLifetime.History base)
    (request : SourceFundingInput.Request p a) (memory : Memory p a)
    (actual : finishedEnvelope after.current.joint = some envelope) :
    envelope ∈ (CohortHostDebt.receiptResult memory (sourceReceipts before after (.inr (.inr request)))).produced :=
  (source_produced _ _ _ _ _).mpr (Or.inr actual)

#print axioms owner_recipe_produced
#print axioms finished_produced
#print axioms source_produced
#print axioms interpret_produced
#print axioms interpret_origin
#print axioms history_origin
#print axioms live_origin
#print axioms discharged_origin
#print axioms finished_present
end CohortProductionCorrespondence
