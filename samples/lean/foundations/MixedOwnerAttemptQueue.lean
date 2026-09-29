import MixedOwnerMaterialization
namespace MirroreaProofFirst.MixedOwnerAttemptQueue
open OwnerEffectService

-- Erase ONLY Fin bounds from durable mathematical history, retaining complete
-- operation handles/body/origin/evidence/read values. Not an image/wire API.
structure Record where
 pending : OwnerSavedPending.Saved
 serviceEvidence : CurrentUse.Evidence
 oldValue : Option Int
 value : Int
 reads : List (Nat × Int)
 deriving DecidableEq

def record (write : Write n) : Record :=
 ⟨OwnerSavedPending.save write.pending,write.serviceEvidence,write.oldValue,write.value,write.reads⟩
inductive Result where
 | refused (why : Failure)
 | committed (write : Record)
 deriving DecidableEq

def outcome : Outcome n → Result
 | .refused why => .refused why
 | .committed write => .committed (record write)

def historyAfter (past : List Record) : Result → List Record
 | .committed written => past++[written]
 | .refused _ => past

def id (e : OwnerSavedPending.Saved) : CurrentUse.UseId := e.original.context.useId
def occurrence (e : OwnerSavedPending.Saved) : Nat × Nat × Nat × Nat :=
 (e.original.context.instanceId,e.request.principal,e.origin.activation,e.origin.ordinal)

structure State where
 store : Nat → Option Int
 history : List Record
 queued : Option OwnerSavedPending.Saved
 attempts : List (OwnerSavedPending.Saved × Result)

def Seen (s : State) (e : OwnerSavedPending.Saved) : Prop :=
 ∃ row ∈ s.attempts, id row.1 = id e ∨ occurrence row.1 = occurrence e
instance : Decidable (Seen s e) := inferInstanceAs (Decidable (∃ row ∈ s.attempts, _))

def enqueue (s : State) (e : OwnerSavedPending.Saved) : Option State :=
 if s.queued.isSome ∨ Seen s e then none else some {s with queued := some e}

-- Boundary refusal is no lower service outcome: retain the queued payload.
-- Actual service refusal consumes ONE attempt but produces no acknowledgment.
-- Current gate rechecking is an explicit transition, not an automatic retry,
-- new ID, rollback, completion policy, fairness or availability guarantee.
def advance (ops : FallibleFlow.Arithmetic) (catalog : MixedInstanceState.State d p m)
 (view : WorldProjection.AuthorityView a) (current : Nat → Option (Nat × Nat))
 (s : State) : State × Option Result :=
 match s.queued with
 | none => (s,none)
 | some saved =>
   match OwnerSavedPending.materialize (WorldProjection.size a p m) saved with
   | none => (s,none)
   | some pending =>
     let result := MixedOwnerMaterialization.serve ops catalog view current ⟨s.store,[]⟩ pending
     match result.2 with
     | none => (s,none)
     | some answer =>
       let done := outcome answer
       (⟨result.1.store,
         historyAfter s.history done,
         none,s.attempts++[(saved,done)]⟩,some done)

theorem enqueue_exact : enqueue s e = some next ↔
 s.queued = none ∧ ¬Seen s e ∧ next = {s with queued := some e} := by
 unfold enqueue
 cases h : s.queued <;> by_cases seen : Seen s e <;> simp [seen,eq_comm]

-- General erasure/refinement: executing on the retained store plus an empty
-- temporary history yields EXACTLY the full-history service result after prefix
-- reattachment. No expected write/history is independently manufactured.
theorem lower_history (ops : FallibleFlow.Arithmetic) (world : CurrentUse.World n)
 (registry : Nat → Option Body) (s : Owner n) (pending : Pending n) :
 serve ops world registry s pending =
 let r := serve ops world registry ⟨s.store,[]⟩ pending
 (⟨r.1.store,s.history++r.1.history⟩,r.2) := by
 unfold serve
 split
 · simp_all
 · split
   · split <;> simp_all
   · simp_all

theorem catalog_history (ops : FallibleFlow.Arithmetic) (catalog : MixedInstanceState.State d p m)
 (view : WorldProjection.AuthorityView a) (s : Owner (WorldProjection.size a p m))
 (pending : Pending (WorldProjection.size a p m)) :
 MixedCatalogService.serve ops catalog view s pending =
 let r := MixedCatalogService.serve ops catalog view ⟨s.store,[]⟩ pending
 (⟨r.1.store,s.history++r.1.history⟩,r.2) := by
 unfold MixedCatalogService.serve
 split
 · split
   · simp
   · exact lower_history _ _ _ _ _
 · simp

theorem guarded_history (ops : FallibleFlow.Arithmetic) (catalog : MixedInstanceState.State d p m)
 (view : WorldProjection.AuthorityView a) (current : Nat → Option (Nat × Nat))
 (s : Owner (WorldProjection.size a p m)) (pending : Pending (WorldProjection.size a p m)) :
 MixedOwnerMaterialization.serve ops catalog view current s pending =
 let r := MixedOwnerMaterialization.serve ops catalog view current ⟨s.store,[]⟩ pending
 (⟨r.1.store,s.history++r.1.history⟩,r.2) := by
 unfold MixedOwnerMaterialization.serve
 cases slot : WorldProjection.decode pending.request.operation.key <;> simp only [slot]
 all_goals try simp
 rename_i key place
 cases kind : MixedCatalogService.operationDefinition catalog key <;> simp only [kind]
 · simp
 · rename_i definition
   have ready : MixedOwnerMaterialization.readyCheck definition current s pending =
     MixedOwnerMaterialization.readyCheck definition current ⟨s.store,[]⟩ pending := rfl
   rw [←ready]
   split
   · rw [catalog_history ops catalog view s pending]
   · simp

variable {ops : FallibleFlow.Arithmetic} {catalog : MixedInstanceState.State d p m}
 {view : WorldProjection.AuthorityView a} {current : Nat → Option (Nat × Nat)} {s : State} {result : Result}

theorem advance_parts (completed : (advance ops catalog view current s).2 = some result) :
 ∃ saved pending answer,
 s.queued = some saved ∧ OwnerSavedPending.save pending = saved ∧
 (MixedOwnerMaterialization.serve ops catalog view current ⟨s.store,[]⟩ pending).2 = some answer ∧
 result = outcome answer ∧
 (advance ops catalog view current s).1.queued = none ∧
 (advance ops catalog view current s).1.attempts = s.attempts++[(saved,result)] ∧
 (advance ops catalog view current s).1.store =
   (MixedOwnerMaterialization.serve ops catalog view current ⟨s.store,[]⟩ pending).1.store ∧
 (advance ops catalog view current s).1.history =
   historyAfter s.history result := by
 unfold advance at completed ⊢
 cases waiting : s.queued with
 | none => simp [waiting] at completed
 | some saved =>
   simp only [waiting] at completed ⊢
   cases loaded : OwnerSavedPending.materialize (WorldProjection.size _ _ _) saved with
   | none => simp [loaded] at completed
   | some pending =>
     simp only [loaded] at completed ⊢
     cases run : (MixedOwnerMaterialization.serve ops catalog view current ⟨s.store,[]⟩ pending).2 with
     | none => simp [run] at completed
     | some answer =>
       simp only [run,Option.some.injEq] at completed ⊢
       subst result
       refine ⟨saved,pending,answer,rfl,OwnerSavedPending.materialize_exact.mp loaded,?_,?_,?_,?_,?_,?_⟩ <;> simp [run]

theorem advance_empty (empty : s.queued = none) : advance ops catalog view current s = (s,none) := by
 simp only [advance,empty]

theorem no_second_advance (completed : (advance ops catalog view current s).2 = some result) :
 advance nextOps nextCatalog nextView nextCurrent (advance ops catalog view current s).1 =
 ((advance ops catalog view current s).1,none) := by
 obtain ⟨_,_,_,_,_,_,_,empty,_,_⟩ := advance_parts completed
 exact advance_empty empty

theorem completed_blocks_same_id (completed : (advance ops catalog view current s).2 = some result)
 (queued : s.queued = some saved) (same : id other = id saved) :
 enqueue (advance ops catalog view current s).1 other = none := by
 obtain ⟨entry,_,_,atQueue,_,_,_,_,attempts,_⟩ := advance_parts completed
 have equal : entry = saved := Option.some.inj (atQueue.symm.trans queued)
 have seen : Seen (advance ops catalog view current s).1 other := by
   refine ⟨(entry,result),?_,Or.inl ?_⟩
   · rw [attempts]; simp
   · rw [equal,same]
 simp [enqueue,seen]

theorem completed_blocks_same_occurrence (completed : (advance ops catalog view current s).2 = some result)
 (queued : s.queued = some saved) (same : occurrence other = occurrence saved) :
 enqueue (advance ops catalog view current s).1 other = none := by
 obtain ⟨entry,_,_,atQueue,_,_,_,_,attempts,_⟩ := advance_parts completed
 have equal : entry = saved := Option.some.inj (atQueue.symm.trans queued)
 have seen : Seen (advance ops catalog view current s).1 other := by
   refine ⟨(entry,result),?_,Or.inr ?_⟩
   · rw [attempts]; simp
   · rw [equal,same]
 simp [enqueue,seen]

theorem not_attempted_retains (absent : (advance ops catalog view current s).2 = none) :
 (advance ops catalog view current s).1 = s := by
 unfold advance at absent ⊢
 split at absent
 · simp_all
 · split at absent
   · simp_all
   · dsimp only at absent ⊢
     split at absent
     · simp_all
     · cases absent

def Invariant (s : State) : Prop :=
 (s.attempts.map fun row => id row.1).Nodup ∧
 (s.attempts.map fun row => occurrence row.1).Nodup ∧
 ∀ e, s.queued = some e → ¬Seen s e

theorem enqueue_preserves (valid : Invariant s) (accepted : enqueue s e = some next) : Invariant next := by
 obtain ⟨_,fresh,rfl⟩ := enqueue_exact.mp accepted
 refine ⟨valid.1,valid.2.1,?_⟩
 intro other same
 cases same
 exact fresh

theorem advance_preserves (valid : Invariant s) : Invariant (advance ops catalog view current s).1 := by
 cases result : (advance ops catalog view current s).2 with
 | none => simpa [not_attempted_retains result] using valid
 | some answer =>
   obtain ⟨e,_,_,waiting,_,_,_,empty,attempts,_⟩ := advance_parts result
   have fresh := valid.2.2 e waiting
   have freshId : id e ∉ s.attempts.map (fun row => id row.1) := by
     intro member
     obtain ⟨row,present,equal⟩ := List.mem_map.mp member
     exact fresh ⟨row,present,Or.inl equal⟩
   have freshOccurrence : occurrence e ∉ s.attempts.map (fun row => occurrence row.1) := by
     intro member
     obtain ⟨row,present,equal⟩ := List.mem_map.mp member
     exact fresh ⟨row,present,Or.inr equal⟩
   unfold Invariant
   rw [attempts]
   simp only [List.map_append,List.map_cons,List.map_nil]
   refine ⟨List.nodup_append.mpr ⟨valid.1,by simp,?_⟩,
     List.nodup_append.mpr ⟨valid.2.1,by simp,?_⟩,?_⟩
   · intro key member other singleton equal
     have value : other = id e := by simpa using singleton
     exact freshId ((equal.trans value) ▸ member)
   · intro key member other singleton equal
     have value : other = occurrence e := by simpa using singleton
     exact freshOccurrence ((equal.trans value) ▸ member)
   · intro other atQueue
     rw [empty] at atQueue
     cases atQueue

def initial (store : Nat → Option Int) : State := ⟨store,[],none,[]⟩
theorem initial_invariant : Invariant (initial store) := by simp [Invariant,initial]

theorem history_retained : ∃ suffix,
 (advance ops catalog view current s).1.history = s.history ++ suffix := by
 cases run : (advance ops catalog view current s).2 with
 | none => exact ⟨[],by rw [not_attempted_retains run]; simp⟩
 | some result =>
   obtain ⟨_,_,_,_,_,_,_,_,_,_,history⟩ := advance_parts run
   cases result with
   | refused why => exact ⟨[],by simpa using history⟩
   | committed written => exact ⟨[written],history⟩

theorem actual_commit (committed : (advance ops catalog view current s).2 = some (.committed written)) :
 ∃ saved pending actual, s.queued = some saved ∧ OwnerSavedPending.save pending = saved ∧
 (MixedOwnerMaterialization.serve ops catalog view current ⟨s.store,[]⟩ pending).2 = some (.committed actual) ∧
 written = record actual ∧ written.pending = saved ∧
 (advance ops catalog view current s).1.history = s.history++[written] := by
 obtain ⟨saved,pending,answer,waiting,loaded,run,equal,_,_,_,history⟩ := advance_parts committed
 cases answer with
 | refused why => cases equal
 | committed actual =>
   have same : written = record actual := Result.committed.inj equal
   obtain ⟨_,lower⟩ := MixedOwnerMaterialization.serve_admission.mp run
   obtain ⟨_,_,_,_,_,_,meaning⟩ := MixedCatalogService.committed_catalog lower
   have pendingEq := congrArg Write.pending meaning.2.2.2
   exact ⟨saved,pending,actual,waiting,loaded,run,same,by rw [same]; exact (congrArg OwnerSavedPending.save pendingEq).trans loaded,history⟩

-- Relative completeness against independent materialization/admission and
-- declarative service semantics, not a successful-advance premise. This excludes
-- an all-refusing queue for precisely this finite arithmetic/current-use profile.
theorem committed_complete
 (queued : s.queued = some saved)
 (materialized : OwnerSavedPending.materialize (WorldProjection.size a p m) saved = some pending)
 (admitted : MixedOwnerMaterialization.Admitted catalog current ⟨s.store,[]⟩ pending)
 (meaning : Commits ops (MixedCatalogService.world catalog view) (MixedCatalogService.registry catalog)
   ⟨s.store,[]⟩ pending written) :
 (advance ops catalog view current s).2 = some (.committed (record written)) := by
 obtain ⟨key,place,definition,slot,kind,_⟩ := admitted
 have catalogRun : (MixedCatalogService.serve ops catalog view ⟨s.store,[]⟩ pending).2 = .committed written := by
   simpa [MixedCatalogService.serve,slot,kind] using committed_exact.mpr meaning
 have ready : MixedOwnerMaterialization.Admitted catalog current ⟨s.store,[]⟩ pending :=
   ⟨key,place,definition,slot,kind,‹MixedOwnerMaterialization.Ready definition current ⟨s.store,[]⟩ pending›⟩
 have run := MixedOwnerMaterialization.serve_admission.mpr ⟨ready,catalogRun⟩
 simp [advance,queued,materialized,run,outcome]

#print axioms committed_complete
#print axioms enqueue_exact
#print axioms not_attempted_retains
#print axioms enqueue_preserves
#print axioms advance_preserves
#print axioms initial_invariant
#print axioms history_retained
#print axioms actual_commit
#print axioms catalog_history
#print axioms lower_history
#print axioms guarded_history
#print axioms advance_parts
#print axioms no_second_advance
#print axioms completed_blocks_same_id
#print axioms completed_blocks_same_occurrence
end MirroreaProofFirst.MixedOwnerAttemptQueue
