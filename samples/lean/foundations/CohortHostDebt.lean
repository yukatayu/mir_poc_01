import CohortHostReceipt
open MirroreaProofFirst SharedHostCaptureReplay
namespace CohortHostDebt
open CohortCommitJournal
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Receipt succession is evaluated against the result of its preceding stores.
-- The list is local store debt, not executed stores or additional native IO.
def compileReceipts (memory : Memory p a) : List (Receipt p a) → List (Store p a)
  | [] => []
  | receipt::rest =>
    let first := recipe memory receipt
    first ++ compileReceipts (first.foldl write memory) rest

def receiptResult (memory : Memory p a) : List (Receipt p a) → Memory p a
  | [] => memory
  | receipt::rest => receiptResult ((recipe memory receipt).foldl write memory) rest

theorem compileReceipts_exact (memory : Memory p a) (receipts : List (Receipt p a)) :
    (compileReceipts memory receipts).foldl write memory = receiptResult memory receipts := by
  induction receipts generalizing memory with
  | nil => rfl
  | cons receipt rest ih => simp only [compileReceipts,List.foldl_append,receiptResult]; exact ih _

-- No arbitrary external receipt is supplied here. Source receipts come from
-- the SAME before/after wire prefix; owner receipts from the current bound
-- writer, at confirmation. SourceEntryPrefix.Step is required by the product.
def receipts (before after : SourceEntryPrefix.State base) : SourceEntryPrefix.Event p a → List (Receipt p a)
  | .ordinary (.source input _) | .reply input _ =>
    CohortHostReceipt.sourceReceipts before.joined.history after.joined.history input
  | .ordinary (.confirmed _ _) =>
    match before.joined.active with
    | some opened => [CohortHostReceipt.ownerReceipt opened.bound]
    | none => []
  | _ => []

inductive Kind where
  | plain | notify | snapshot | forbidden
  deriving DecidableEq

def kind : SourceEntryPrefix.Event p a → Kind
  | .store .snapshot _ _ => .snapshot
  | .store .notify _ _ | .store .cancel _ _ => .notify
  | .retire | .ordinary .retire | .ordinary .localComplete => .forbidden
  | _ => .plain

-- Inner native/local actions require actual outer gate ownership. The owning
-- product connects enter/release to the separate acquisition/restoration model.
-- An entry's snapshot store consumes BOTH its source-entry barrier and the
-- corresponding cohort debt once; notify/cancel may precede that store.
def advance (before after : SourceEntryPrefix.State base) (s : State p a)
    (event : SourceEntryPrefix.Event p a) : Option (State p a) :=
  if s.retired || !s.gate || s.call.isNone then none else
  match kind event with
  | .forbidden => none
  | .plain =>
    match s.pending with
    | [] => some {s with pending:=compileReceipts s.memory (receipts before after event)}
    | _::_ => none
  | .notify =>
    match s.pending with
    | .snapshot _ :: _ => some s
    | _ => none
  | .snapshot =>
    match event,s.pending with
    | .store .snapshot _ observed,.snapshot expected::rest =>
      if SourceEntryCapture.snapshotEq observed expected then
        some {s with memory:=write s.memory (.snapshot expected),pending:=rest}
      else none
    | _,_ => none

-- Declarative operational rules, independent of the executable case split.
-- The lower source-entry Step supplies the cross-writer notify/cancel barrier;
-- this relation supplies the independent full cohort debt and gate condition.
inductive Step (before after : SourceEntryPrefix.State base) :
    State p a → SourceEntryPrefix.Event p a → State p a → Prop where
  | plain (live : s.retired = false) (gate : s.gate = true) (call : s.call = some callId)
      (empty : s.pending = []) (classification : kind event = .plain) :
      Step before after s event {s with pending:=compileReceipts s.memory (receipts before after event)}
  | notify (live : s.retired = false) (gate : s.gate = true) (call : s.call = some callId)
      (owed : s.pending = .snapshot value::rest) (classification : kind event = .notify) :
      Step before after s event s
  | snapshot (live : s.retired = false) (gate : s.gate = true) (call : s.call = some callId)
      (owed : s.pending = .snapshot value::rest) :
      Step before after s (.store .snapshot writer value)
        {s with memory:=write s.memory (.snapshot value),pending:=rest}

theorem advance_complete (step : Step before after s event next) :
    advance before after s event = some next := by
  cases step <;> simp_all [advance,kind,SourceEntryCapture.snapshotEq_exact]

theorem advance_sound (checked : advance before after s event = some next) :
    Step before after s event next := by
  unfold advance at checked
  split at checked
  · cases checked
  · rename_i live
    have guards : (s.retired = false ∧ s.gate = true) ∧ s.call ≠ none := by
      simpa using live
    cases call : s.call with
    | none => simp [call] at guards
    | some callId =>
      cases classification : kind event with
      | forbidden => simp [classification] at checked
      | plain =>
        cases owed : s.pending with
        | cons => simp [classification,owed] at checked
        | nil =>
          simp only [classification,owed] at checked
          cases Option.some.inj checked
          exact .plain guards.1.1 guards.1.2 call owed classification
      | notify =>
        cases owed : s.pending with
        | nil => simp [classification,owed] at checked
        | cons store rest =>
          cases store <;> simp only [classification,owed] at checked <;> try contradiction
          cases Option.some.inj checked
          exact .notify guards.1.1 guards.1.2 call owed classification
      | snapshot =>
        cases event <;> simp only [kind] at classification <;> try contradiction
        case ordinary inner => cases inner <;> cases classification
        rename_i storeKind writer observed
        cases storeKind <;> simp only [kind] at classification <;> try contradiction
        cases owed : s.pending with
        | nil => simp [kind,owed] at checked
        | cons store rest =>
          cases store <;> simp only [kind,owed] at checked <;> try contradiction
          split at checked
          · rename_i equal
            have equal := SourceEntryCapture.snapshotEq_exact.mp equal
            subst observed
            cases Option.some.inj checked
            exact .snapshot guards.1.1 guards.1.2 call owed
          · cases checked

theorem advance_exact : advance before after s event = some next ↔ Step before after s event next :=
  ⟨advance_sound,advance_complete⟩

theorem preserves (valid : WellFormed s) (step : Step before after s event next) : WellFormed next := by
  cases step <;> simp_all [WellFormed]

theorem frames_gate (step : Step before after s event next) :
    next.call = s.call ∧ next.gate = s.gate ∧ next.retired = s.retired := by
  cases step <;> exact ⟨rfl,rfl,rfl⟩

theorem snapshot_target (step : Step before after s (.store .snapshot writer value) next) :
    target next = target s := by
  cases step with
  | plain _ _ _ _ classification => cases classification
  | notify _ _ _ _ classification => cases classification
  | snapshot live gate call owed => simp [target,owed]

#print axioms compileReceipts_exact
#print axioms advance_exact
#print axioms preserves
#print axioms frames_gate
#print axioms snapshot_target
end CohortHostDebt
