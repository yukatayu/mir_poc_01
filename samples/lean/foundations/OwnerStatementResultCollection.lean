import OwnerStatementSourceCustodian
namespace MirroreaProofFirst.OwnerStatementResultCollection
open OwnerStatementLiveCustodian
abbrev Row := OwnerSavedPending.Saved × MixedOwnerAttemptQueue.Result

-- Collection reads the actual lower attempt history. Ambiguity is refused;
-- saved equality includes arguments, origin, code/body and original evidence.
-- It neither evaluates an expression nor creates authority or a source tick.
def lookup (rows : List Row) (saved : OwnerSavedPending.Saved) : Option MixedOwnerAttemptQueue.Result :=
 match rows.filter (fun row => decide (row.1 = saved)) with
 | [row] => some row.2
 | _ => none

-- Independent declarative rule: exactly one matching actual attempt, with all
-- other rows unequal to the full saved request. This includes refused outcomes.
def Unique (rows : List Row) (saved : OwnerSavedPending.Saved) (answer : MixedOwnerAttemptQueue.Result) : Prop :=
 ∃ before after, rows = before ++ (saved,answer) :: after ∧
 (∀ row ∈ before, row.1 ≠ saved) ∧ (∀ row ∈ after, row.1 ≠ saved)

theorem lookup_filter : lookup rows saved = some answer ↔
 rows.filter (fun row => decide (row.1 = saved)) = [(saved,answer)] := by
 unfold lookup
 cases filtered : rows.filter (fun row => decide (row.1 = saved)) with
 | nil => simp
 | cons head tail =>
   cases tail with
   | cons other rest => simp
   | nil =>
     have member : head ∈ rows.filter (fun row => decide (row.1 = saved)) := by rw [filtered]; simp
     have same : head.1 = saved := by simpa using (List.mem_filter.mp member).2
     cases head
     simp_all

theorem lookup_exact : lookup rows saved = some answer ↔ Unique rows saved answer := by
 rw [lookup_filter]
 simp only [List.filter_eq_cons_iff,List.filter_eq_nil_iff,decide_eq_true_eq,Unique]
 simp

def collect (s : State p a) (ticket : Ticket) : Option MixedOwnerAttemptQueue.Result :=
 if ticket.designation = s.designation ∧
 (s.held = some ⟨ticket,.unreported⟩ ∨ s.held = some ⟨ticket,.reported⟩) then
 lookup s.live.session.state.owner.attempts ticket.saved else none

def Collectible (s : State p a) (ticket : Ticket) (answer : MixedOwnerAttemptQueue.Result) : Prop :=
 ticket.designation = s.designation ∧
 (s.held = some ⟨ticket,.unreported⟩ ∨ s.held = some ⟨ticket,.reported⟩) ∧
 Unique s.live.session.state.owner.attempts ticket.saved answer

theorem collect_exact : collect s ticket = some answer ↔ Collectible s ticket answer := by
 unfold collect Collectible
 split <;> simp_all [lookup_exact]

theorem collected_actual (run : collect s ticket = some answer) :
 (ticket.saved,answer) ∈ s.live.session.state.owner.attempts := by
 obtain ⟨_,_,before,after,equal,_,_⟩ := collect_exact.mp run
 rw [equal]; simp

theorem queued_no_collect (held : s.held = some ⟨ticket,.queued⟩) : collect s other = none := by
 simp [collect,held,Held.mk.injEq]

theorem wrong_custodian (wrong : ticket.designation ≠ s.designation) : collect s ticket = none := by
 simp [collect,wrong]

-- Only a retained actual successful write can enter successful source report.
-- This is historical reporting, not current authorization of another effect.
-- Current source consumption remains the original Bank tick and its checker.
def reportActual (s : State p a) (ticket : Ticket) : Option (State p a) := do
 let answer ← collect s ticket
 match answer with
 | .refused _ => none
 | .committed written =>
   if written.pending = ticket.saved then report s ticket else none

theorem reportActual_exact : reportActual s ticket = some next ↔
 ∃ written, Collectible s ticket (.committed written) ∧ written.pending = ticket.saved ∧
 s.held = some ⟨ticket,.unreported⟩ ∧ next = {s with held := some ⟨ticket,.reported⟩} := by
 unfold reportActual
 cases result : collect s ticket with
 | none => simp [←collect_exact,result]
 | some answer =>
   cases answer with
   | refused why => simp [←collect_exact,result]
   | committed written =>
     simp only [Option.bind_eq_bind,Option.bind_some]
     by_cases same : written.pending = ticket.saved
     · simp [same,←collect_exact,result,report_exact]
     · simp [same,←collect_exact,result]

theorem reportActual_refines (run : reportActual s ticket = some next) :
 OwnerStatementSourceCustodian.Path capacity s next := by
 obtain ⟨_,_,_,held,rfl⟩ := reportActual_exact.mp run
 exact .report .initial (report_exact.mpr ⟨held,rfl⟩)

theorem reportActual_keeps (run : reportActual s ticket = some next) :
 next.live = s.live ∧ next.dispatched = s.dispatched ∧ next.nextSerial = s.nextSerial := by
 obtain ⟨_,_,_,_,rfl⟩ := reportActual_exact.mp run
 exact ⟨rfl,rfl,rfl⟩

theorem refused_not_success (result : collect s ticket = some (.refused why)) : reportActual s ticket = none := by
 simp [reportActual,result]

theorem reportActual_no_execute (run : reportActual s ticket = some next) : execute next other ops = none := by
 obtain ⟨_,_,_,_,rfl⟩ := reportActual_exact.mp run
 simp [execute,Held.mk.injEq]

theorem collection_does_not_enable_repeat (done : execute before ticket ops = some s) :
 execute s other moreOps = none := executed_no_repeat done

-- A genuine lower advance appends the uniquely keyed result under the existing
-- queue invariant. Collection therefore cannot be an all-refusing checker.
theorem lookup_actual_advance
 (valid : MixedOwnerAttemptQueue.Invariant owner)
 (queued : owner.queued = some saved)
 (completed : (MixedOwnerAttemptQueue.advance ops catalog view current owner).2 = some answer) :
 lookup (MixedOwnerAttemptQueue.advance ops catalog view current owner).1.attempts saved = some answer := by
 obtain ⟨entry,_,_,atQueue,_,_,_,_,attempts,_⟩ := MixedOwnerAttemptQueue.advance_parts completed
 have equal : entry = saved := Option.some.inj (atQueue.symm.trans queued)
 apply lookup_exact.mpr
 refine ⟨owner.attempts,[],?_,?_,by simp⟩
 · rw [attempts,equal]
 · intro row member same
   have fresh := valid.2.2 saved queued
   exact fresh ⟨row,member,Or.inl (congrArg MixedOwnerAttemptQueue.id same)⟩

#print axioms lookup_actual_advance

#print axioms lookup_exact
#print axioms collect_exact
#print axioms collected_actual
#print axioms reportActual_exact
#print axioms reportActual_refines
#print axioms reportActual_keeps
#print axioms refused_not_success
#print axioms reportActual_no_execute
end MirroreaProofFirst.OwnerStatementResultCollection
