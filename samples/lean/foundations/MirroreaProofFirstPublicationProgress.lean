import MirroreaProofFirstPublicationUse

namespace MirroreaProofFirst.PublicationProgress
open Publication

-- Safety alone permits unreachable fences beyond all emitted rounds. This
-- additional fact is derived from emission history, not added to admission.
def Upper (s : Publication.State n) : Prop :=
  s.announced ≤ s.published+1 ∧
    (∀ r ∈ s.stops, r ≤ s.announced) ∧ ∀ i, s.fence i ≤ s.announced

theorem initial_upper (n revision : Nat) : Upper (Publication.initial n revision) := by
  simp [Upper,Publication.initial]

theorem upper_preserved (upper : Upper s)
    (allowed : Publication.Allowed s action) : Upper (Publication.apply s action) := by
  obtain ⟨gap,stops,fences⟩ := upper
  cases action with
  | begin =>
      refine ⟨Nat.le_refl _,?_,?_⟩
      · intro r present
        rcases List.mem_cons.mp present with rfl | prior
        · exact Nat.le_refl _
        · have bound := stops r prior
          change s.announced = s.published at allowed
          change r ≤ s.published+1
          omega
      · intro i
        have bound := fences i
        change s.announced = s.published at allowed
        change s.fence i ≤ s.published+1
        omega
  | freeze i r =>
      refine ⟨gap,stops,?_⟩
      intro j
      by_cases equal : j = i
      · subst j
        have emitted := stops r allowed
        simpa [Publication.apply,Publication.put,Nat.max_le] using And.intro (fences i) emitted
      · simpa [Publication.apply,Publication.put,equal] using fences j
  | acknowledge i r => exact ⟨gap,stops,fences⟩
  | publish => exact ⟨Nat.le_succ _,stops,fences⟩
  | install i r => exact ⟨gap,stops,fences⟩
  | use i => exact ⟨gap,stops,fences⟩

theorem reached_upper (path : Publication.Reached n revision s) : Upper s := by
  induction path with
  | initial => exact initial_upper _ _
  | step prior step ih =>
      cases step with
      | action allowed => exact upper_preserved ih allowed

-- A constructive finite schedule, not fairness or completion of every run.
inductive Sequence : Publication.State n → List (Publication.Action n) → Publication.State n → Prop where
  | nil : Sequence s [] s
  | cons : Publication.Allowed s action → Sequence (Publication.apply s action) rest next →
      Sequence s (action :: rest) next

def run : Publication.State n → List (Publication.Action n) → Option (Publication.State n)
  | s,[] => some s
  | s,action :: rest => (Publication.execute s action).bind (run · rest)

theorem run_exact : run s actions = some next ↔ Sequence s actions next := by
  induction actions generalizing s next with
  | nil => simp [run]; constructor <;> intro equal
           · cases equal; exact .nil
           · cases equal; rfl
  | cons action rest ih =>
      constructor
      · intro accepted
        cases step : Publication.execute s action with
        | none => simp [run,step] at accepted
        | some intermediate =>
            obtain ⟨allowed,rfl⟩ := (Publication.execute_exact _ _ _).mp step
            exact .cons allowed (ih.mp (by simpa [run,step] using accepted))
      · intro sequence
        cases sequence with
        | cons allowed rest =>
            have executed := (Publication.execute_exact s (Publication.apply s action) action).mpr ⟨allowed,rfl⟩
            simpa [run,executed] using ih.mpr rest

theorem sequence_reached (path : Publication.Reached n revision s)
    (sequence : Sequence s actions next) : Publication.Reached n revision next := by
  induction sequence with
  | nil => exact path
  | cons allowed _ ih => exact ih (.step path (.action allowed))

theorem sequence_append (one : Sequence s first middle) (two : Sequence middle second next) :
    Sequence s (first ++ second) next := by
  induction one with
  | nil => exact two
  | cons allowed _ ih => exact .cons allowed (ih two)

-- Local traversal lemma: all admitted per-participant updates can be taken
-- while retaining earlier participants' postconditions. Repetitions allowed.
private theorem traverse (actions : Fin n → Publication.Action n)
    (invariant : Publication.State n → Prop) (done : Fin n → Publication.State n → Prop)
    (admission : ∀ s i, invariant s → Publication.Allowed s (actions i))
    (preserved : ∀ s i, invariant s → invariant (Publication.apply s (actions i)))
    (established : ∀ s i, invariant s → done i (Publication.apply s (actions i)))
    (retained : ∀ s i j, invariant s → done j s → done j (Publication.apply s (actions i)))
    (items : List (Fin n)) (start : invariant s) :
    ∃ next, Sequence s (items.map actions) next ∧ invariant next ∧
      (∀ i ∈ items, done i next) ∧ (∀ i, done i s → done i next) := by
  induction items generalizing s with
  | nil => exact ⟨s,.nil,start,by simp,fun _ h => h⟩
  | cons i items ih =>
      obtain ⟨next,path,valid,all,keep⟩ := ih (preserved s i start)
      refine ⟨next,.cons (admission s i start) path,valid,?_,?_⟩
      · intro j present
        rcases List.mem_cons.mp present with equal | tail
        · subst j; exact keep i (established s i start)
        · exact all j tail
      · intro j prior
        exact keep j (retained s i j start prior)

def Freezing (revision : Nat) (s : Publication.State n) : Prop :=
  s.published+1 = revision ∧ s.announced = revision ∧ revision ∈ s.stops ∧
    (∀ i, s.fence i ≤ revision)

theorem freeze_all {s : Publication.State n} (ready : Freezing revision s) :
    ∃ next, Sequence s ((List.finRange n).map (fun i => .freeze i revision)) next ∧
      Freezing revision next ∧ ∀ i, (i,revision) ∈ next.frozen ∧ next.fence i = revision := by
  have admission : ∀ (s : Publication.State n) i, Freezing revision s → Publication.Allowed s (.freeze i revision) :=
    fun _ _ h => h.2.2.1
  have preserved : ∀ (s : Publication.State n) i, Freezing revision s → Freezing revision (Publication.apply s (.freeze i revision)) := by
    intro s i h
    refine ⟨h.1,h.2.1,h.2.2.1,?_⟩
    intro j
    by_cases equal : j = i
    · subst j; simp [Publication.apply,Publication.put,Nat.max_eq_right (h.2.2.2 i)]
    · simpa [Publication.apply,Publication.put,equal] using h.2.2.2 j
  have established : ∀ (s : Publication.State n) i, Freezing revision s →
      (i,revision) ∈ (Publication.apply s (.freeze i revision)).frozen ∧
      (Publication.apply s (.freeze i revision)).fence i = revision := by
    intro s i h
    simp [Publication.apply,Publication.put,Nat.max_eq_right (h.2.2.2 i)]
  have retained : ∀ (s : Publication.State n) i j, Freezing revision s →
      ((j,revision) ∈ s.frozen ∧ s.fence j = revision) →
      (j,revision) ∈ (Publication.apply s (.freeze i revision)).frozen ∧
      (Publication.apply s (.freeze i revision)).fence j = revision := by
    intro s i j h prior
    refine ⟨List.mem_cons_of_mem _ prior.1,?_⟩
    by_cases equal : j = i
    · subst j; simp [Publication.apply,Publication.put,prior.2]
    · simpa [Publication.apply,Publication.put,equal] using prior.2
  obtain ⟨next,path,valid,all,_⟩ := traverse (fun i => .freeze i revision) (Freezing revision)
    (fun i s => (i,revision) ∈ s.frozen ∧ s.fence i = revision)
    admission preserved established retained (List.finRange n) ready
  exact ⟨next,path,valid,fun i => all i (List.mem_finRange i)⟩

def Acknowledging (revision : Nat) (s : Publication.State n) : Prop :=
  Freezing revision s ∧ ∀ i, (i,revision) ∈ s.frozen ∧ s.fence i = revision

theorem acknowledge_all {s : Publication.State n} (ready : Acknowledging revision s) :
    ∃ next, Sequence s ((List.finRange n).map (fun i => .acknowledge i revision)) next ∧
      Acknowledging revision next ∧ ∀ i, revision ≤ next.ack i := by
  have admission : ∀ (s : Publication.State n) i, Acknowledging revision s →
      Publication.Allowed s (.acknowledge i revision) := fun _ i h => (h.2 i).1
  have preserved : ∀ (s : Publication.State n) i, Acknowledging revision s →
      Acknowledging revision (Publication.apply s (.acknowledge i revision)) := fun _ _ h => h
  have established : ∀ (s : Publication.State n) i, Acknowledging revision s →
      revision ≤ (Publication.apply s (.acknowledge i revision)).ack i := by
    intro s i _; simpa [Publication.apply,Publication.put] using Nat.le_max_right (s.ack i) revision
  have retained : ∀ (s : Publication.State n) i j, Acknowledging revision s → revision ≤ s.ack j →
      revision ≤ (Publication.apply s (.acknowledge i revision)).ack j := by
    intro s i j _ prior
    by_cases equal : j = i
    · subst j; simpa [Publication.apply,Publication.put] using Nat.le_max_right (s.ack i) revision
    · simpa [Publication.apply,Publication.put,equal] using prior
  obtain ⟨next,path,valid,all,_⟩ := traverse (fun i => .acknowledge i revision) (Acknowledging revision)
    (fun i s => revision ≤ s.ack i) admission preserved established retained (List.finRange n) ready
  exact ⟨next,path,valid,fun i => all i (List.mem_finRange i)⟩

def Installing (revision : Nat) (s : Publication.State n) : Prop :=
  s.published = revision ∧ s.announced = revision ∧ revision ∈ s.certificates ∧ ∀ i, s.fence i = revision

theorem install_all {s : Publication.State n} (ready : Installing revision s) :
    ∃ next, Sequence s ((List.finRange n).map (fun i => .install i revision)) next ∧
      Installing revision next ∧ ∀ i, next.installed i = revision := by
  have admission : ∀ (s : Publication.State n) i, Installing revision s → Publication.Allowed s (.install i revision) := by
    intro s i h; exact ⟨h.2.2.1,Nat.le_of_eq (h.2.2.2 i)⟩
  have preserved : ∀ (s : Publication.State n) i, Installing revision s →
      Installing revision (Publication.apply s (.install i revision)) := fun _ _ h => h
  have established : ∀ (s : Publication.State n) i, Installing revision s →
      (Publication.apply s (.install i revision)).installed i = revision := by
    intro s i _; simp [Publication.apply,Publication.put]
  have retained : ∀ (s : Publication.State n) i j, Installing revision s → s.installed j = revision →
      (Publication.apply s (.install i revision)).installed j = revision := by
    intro s i j _ prior
    by_cases equal : j = i
    · subst j; simp [Publication.apply,Publication.put]
    · simpa [Publication.apply,Publication.put,equal] using prior
  obtain ⟨next,path,valid,all,_⟩ := traverse (fun i => .install i revision) (Installing revision)
    (fun i s => s.installed i = revision) admission preserved established retained (List.finRange n) ready
  exact ⟨next,path,valid,fun i => all i (List.mem_finRange i)⟩

def normal (n revision : Nat) : List (Publication.Action n) :=
  [.begin] ++ (List.finRange n).map (fun i => .freeze i revision) ++
    (List.finRange n).map (fun i => .acknowledge i revision) ++ [.publish] ++
    (List.finRange n).map (fun i => .install i revision)

theorem normal_succeeds (path : Publication.Reached n initialRevision s)
    (settled : s.announced = s.published) :
    ∃ next, run s (normal n (s.published+1)) = some next ∧
      next.published = s.published+1 ∧ next.announced = next.published ∧
      ∀ i, next.installed i = next.published ∧ next.fence i = next.published := by
  let r := s.published+1
  have ready : Freezing r (Publication.apply s .begin) := by
    refine ⟨rfl,rfl,by simp [Publication.apply,r],?_⟩
    intro i
    have bound := (reached_upper path).2.2 i
    change s.fence i ≤ s.published+1
    omega
  obtain ⟨frozen,freezes,freezeReady,frozenAll⟩ := freeze_all ready
  obtain ⟨acked,acks,ackReady,ackAll⟩ := acknowledge_all ⟨freezeReady,frozenAll⟩
  have publishAllowed : Publication.Allowed acked .publish := by
    refine ⟨?_,?_⟩
    · have one := ackReady.1.1; have two := ackReady.1.2.1; omega
    · intro i; rw [ackReady.1.2.1]; exact ackAll i
  have installReady : Installing r (Publication.apply acked .publish) := by
    refine ⟨ackReady.1.2.1,ackReady.1.2.1,?_,fun i => (ackReady.2 i).2⟩
    simp [Publication.apply,ackReady.1.2.1]
  obtain ⟨next,installs,finalReady,installed⟩ := install_all installReady
  have sequence : Sequence s (normal n r) next := by
    unfold normal
    exact sequence_append (sequence_append (sequence_append (sequence_append
      (.cons settled .nil) freezes) acks) (.cons publishAllowed .nil)) installs
  exact ⟨next,run_exact.mpr sequence,finalReady.1,finalReady.2.1.trans finalReady.1.symm,
    fun i => ⟨(installed i).trans finalReady.1.symm,(finalReady.2.2.2 i).trans finalReady.1.symm⟩⟩

#print axioms reached_upper
#print axioms run_exact
#print axioms normal_succeeds
end MirroreaProofFirst.PublicationProgress

namespace MirroreaProofFirst.PublicationPayload

-- These availability facts follow from admitted history. They supply the
-- constructive schedule's payload premises; they are not checker conditions.
structure Available (s : State n Value Command) : Prop where
  prepared : s.prepared.isSome = true ↔ s.barrier.published < s.barrier.announced
  image : ∀ r ∈ s.barrier.certificates, ∃ value, s.images r = some value
  bound : ∀ r value, s.images r = some value → r ≤ s.barrier.published

theorem initial_available (n revision : Nat) (value : Value) :
    Available (initial (Command:=Command) n revision value) := by
  refine ⟨by simp [initial,Publication.initial],?_,?_⟩
  · intro r present
    simp only [initial,Publication.initial,List.mem_singleton] at present
    subst r; exact ⟨value,by simp [initial]⟩
  · intro r v found
    by_cases equal : r = revision
    · subst r; exact Nat.le_refl _
    · simp [initial,equal] at found

theorem available_preserved (old : Available s) (valid : Invariant evaluate s)
    (allowed : Allowed evaluate s action) : Available (apply evaluate s action) := by
  refine ⟨?_,?_,?_⟩
  · cases action with
    | stage command =>
        obtain ⟨value,computed⟩ := allowed.2
        simp [apply,projected,Publication.apply,computed]
    | publish => simp [apply,projected,Publication.apply]
    | freeze i r => exact old.prepared
    | acknowledge i r => exact old.prepared
    | install i r => exact old.prepared
    | use i => exact old.prepared
  · intro r present
    cases action with
    | stage command => exact old.image r present
    | freeze i generation => exact old.image r present
    | acknowledge i generation => exact old.image r present
    | install i generation => exact old.image r present
    | use i => exact old.image r present
    | publish =>
        by_cases equal : r = s.barrier.announced
        · subst r; exact ⟨(s.prepared.map Prod.fst).getD s.current,by simp [apply,write]⟩
        · have prior : r ∈ s.barrier.certificates := by
            simpa [apply,projected,Publication.apply,equal] using present
          obtain ⟨value,found⟩ := old.image r prior
          exact ⟨value,by simpa [apply,write,equal] using found⟩
  · intro r value found
    cases action with
    | stage command => exact old.bound r value found
    | freeze i generation => exact old.bound r value found
    | acknowledge i generation => exact old.bound r value found
    | install i generation => exact old.bound r value found
    | use i => exact old.bound r value found
    | publish =>
        by_cases equal : r = s.barrier.announced
        · subst r; exact Nat.le_refl _
        · have prior : s.images r = some value := by simpa [apply,write,equal] using found
          exact Nat.le_trans (old.bound r value prior) valid.numeric.1

theorem reached_available (path : Reached evaluate n revision value s) : Available s := by
  induction path with
  | initial => exact initial_available _ _ _
  | step prior step ih =>
      cases step with
      | action allowed => exact available_preserved ih (reached_invariant prior) allowed

theorem reached_numeric (path : Reached evaluate n revision value s) :
    Publication.Reached n revision s.barrier := by
  induction path with
  | initial => exact .initial
  | step prior step ih =>
      cases step with
      | action allowed => rw [apply_projection]; exact .step ih (.action allowed.1)

theorem historical_image_immutable (path : Reached evaluate n revision initial s)
    (allowed : Allowed evaluate s action) (found : s.images r = some value) :
    (apply evaluate s action).images r = some value := by
  cases action with
  | stage command => exact found
  | freeze i generation => exact found
  | acknowledge i generation => exact found
  | install i generation => exact found
  | use i => exact found
  | publish =>
      have bound := (reached_available path).bound r value found
      have newer := allowed.1.1
      have different : r ≠ s.barrier.announced := by omega
      simpa [apply,write,different] using found

#print axioms reached_available
#print axioms historical_image_immutable
end MirroreaProofFirst.PublicationPayload

namespace MirroreaProofFirst.PublicationProgress

def lift (command : Command) : Publication.Action n → PublicationUse.Action n Command
  | .begin => .administrative (.stage command)
  | .freeze i r => .administrative (.freeze i r)
  | .acknowledge i r => .administrative (.acknowledge i r)
  | .publish => .administrative .publish
  | .install i r => .administrative (.install i r)
  | .use i => .administrative (.use i)

theorem lift_projection (command : Command) (s : PublicationUse.State n Value Command)
    (action : Publication.Action n) :
    (PublicationUse.apply evaluate s (lift command action)).base.barrier =
      Publication.apply s.base.barrier action := by
  cases action <;> rfl

theorem lift_free (command : Command) (s : PublicationUse.State n Value Command)
    (action : Publication.Action n) (free : ∀ i, s.held i = none) :
    ∀ i, (PublicationUse.apply evaluate s (lift command action)).held i = none := by
  cases action <;> exact free

theorem lift_admitted (path : PublicationUse.Reached evaluate n revision initial s)
    (free : ∀ i, s.held i = none) (allowed : Publication.Allowed s.base.barrier action)
    (begin : action = .begin → ∃ value, evaluate s.base.current command = some value) :
    PublicationUse.Allowed evaluate s (lift command action) := by
  have available := PublicationPayload.reached_available (PublicationUse.reached_payload path)
  cases action with
  | begin => exact ⟨⟨allowed,begin rfl⟩,True.intro⟩
  | freeze i r => exact ⟨⟨allowed,True.intro⟩,free i⟩
  | acknowledge i r => exact ⟨⟨allowed,True.intro⟩,True.intro⟩
  | publish =>
      have somePrepared := available.prepared.mpr allowed.1
      exact ⟨⟨allowed,Option.isSome_iff_exists.mp somePrepared⟩,True.intro⟩
  | install i r => exact ⟨⟨allowed,available.image r allowed.1⟩,free i⟩
  | use i => exact ⟨⟨allowed,True.intro⟩,True.intro⟩

def useRun (evaluate : Value → Command → Option Value) :
    PublicationUse.State n Value Command → List (PublicationUse.Action n Command) →
      Option (PublicationUse.State n Value Command)
  | s,[] => some s
  | s,action :: rest => (PublicationUse.execute evaluate s action).bind (useRun evaluate · rest)

theorem lift_passive_sequence {base nextBase : Publication.State n} (command : Command)
    (sequence : Sequence base actions nextBase) (passive : Publication.Action.begin ∉ actions)
    (s : PublicationUse.State n Value Command) (same : s.base.barrier = base)
    (path : PublicationUse.Reached evaluate n revision initial s) (free : ∀ i, s.held i = none) :
    ∃ next, useRun evaluate s (actions.map (lift command)) = some next ∧
      next.base.barrier = nextBase ∧ PublicationUse.Reached evaluate n revision initial next ∧
      ∀ i, next.held i = none := by
  induction sequence generalizing s with
  | nil => exact ⟨s,rfl,same,path,free⟩
  | @cons base action rest next allowed tail ih =>
      subst base
      have notBegin : action ≠ .begin := by intro equal; apply passive; simp [equal]
      have permitted := lift_admitted (command:=command) path free allowed (fun equal => False.elim (notBegin equal))
      have executed := (PublicationUse.execute_exact _ _ _ _).mpr ⟨permitted,rfl⟩
      have restPassive : Publication.Action.begin ∉ rest := fun present => passive (List.mem_cons_of_mem _ present)
      obtain ⟨result,runResult,projection,reached,noHeld⟩ := ih restPassive
        (PublicationUse.apply evaluate s (lift command action)) (lift_projection _ _ _)
        (.step path (.action permitted)) (lift_free _ _ _ free)
      exact ⟨result,by simpa [useRun,executed] using runResult,projection,reached,noHeld⟩

#print axioms lift_admitted
#print axioms lift_passive_sequence
end MirroreaProofFirst.PublicationProgress

namespace MirroreaProofFirst.PublicationProgress

def PreparedResult (command : Command) (value : Value) (s : PublicationUse.State n Value Command) : Prop :=
  s.base.prepared = some (value,command) ∨ (s.base.current = value ∧ s.base.prepared = none)

theorem passive_result (notBegin : action ≠ Publication.Action.begin)
    (result : PreparedResult command value s) :
    PreparedResult command value (PublicationUse.apply evaluate s (lift command action)) := by
  cases action with
  | begin => exact False.elim (notBegin rfl)
  | freeze i r => exact result
  | acknowledge i r => exact result
  | install i r => exact result
  | use i => exact result
  | publish =>
      apply Or.inr
      rcases result with prepared | ⟨current,prepared⟩ <;>
        simp_all [PublicationUse.apply,lift,PublicationPayload.apply]

theorem passive_run_result {actions : List (Publication.Action n)}
    {s next : PublicationUse.State n Value Command} (passive : Publication.Action.begin ∉ actions)
    (result : PreparedResult command value s)
    (accepted : useRun evaluate s (actions.map (lift command)) = some next) :
    PreparedResult command value next := by
  induction actions generalizing s next with
  | nil => cases accepted; exact result
  | cons action actions ih =>
      cases executed : PublicationUse.execute evaluate s (lift command action) with
      | none => simp [useRun,executed] at accepted
      | some intermediate =>
          obtain ⟨_,rfl⟩ := (PublicationUse.execute_exact _ _ _ _).mp executed
          have notBegin : action ≠ .begin := by intro equal; apply passive; simp [equal]
          exact ih (fun present => passive (List.mem_cons_of_mem _ present))
            (passive_result (evaluate:=evaluate) notBegin result)
            (by simpa [useRun,executed] using accepted)

def normalTail (n revision : Nat) : List (Publication.Action n) :=
  (List.finRange n).map (fun i => .freeze i revision) ++
    (List.finRange n).map (fun i => .acknowledge i revision) ++ [.publish] ++
    (List.finRange n).map (fun i => .install i revision)

theorem normal_eq (n revision : Nat) : normal n revision = .begin :: normalTail n revision := by
  simp [normal,normalTail]

theorem normalTail_passive (n revision : Nat) : Publication.Action.begin ∉ normalTail n revision := by
  simp [normalTail]

-- A general useful schedule for nonempty cohorts and any evaluable command,
-- including complete Session/outcome snapshots. No fairness, remote owner
-- availability, issuer authority or implicit ordinary-read atomicity is proved.
theorem normal_payload_succeeds {s : PublicationUse.State n Value Command}
    (nonempty : 0 < n) (path : PublicationUse.Reached evaluate n revision initial s)
    (settled : s.base.barrier.announced = s.base.barrier.published)
    (free : ∀ i, s.held i = none) (computed : evaluate s.base.current command = some value) :
    ∃ next, useRun evaluate s ((normal n (s.base.barrier.published+1)).map (lift command)) = some next ∧
      PublicationUse.Reached evaluate n revision initial next ∧
      next.base.current = value ∧ next.base.barrier.published = s.base.barrier.published+1 ∧
      next.base.barrier.announced = next.base.barrier.published ∧
      (∀ i, next.base.cached i = value ∧ next.held i = none ∧
        PublicationUse.check evaluate next (.enter i) = true) ∧
      ∃ i, PublicationUse.check evaluate next (.enter i) = true := by
  have numericPath := PublicationPayload.reached_numeric (PublicationUse.reached_payload path)
  obtain ⟨nextBase,normalRun,published,settledNext,openAll⟩ := normal_succeeds numericPath settled
  have whole := run_exact.mp normalRun
  rw [normal_eq] at whole
  cases whole with
  | cons beginAllowed tail =>
      let staged := PublicationUse.apply evaluate s (lift command .begin)
      have permitted := lift_admitted (command:=command) path free beginAllowed (fun _ => ⟨value,computed⟩)
      have stagedRun : PublicationUse.execute evaluate s (lift command .begin) = some staged :=
        (PublicationUse.execute_exact _ _ _ _).mpr ⟨permitted,rfl⟩
      have stagedPath : PublicationUse.Reached evaluate n revision initial staged := .step path (.action permitted)
      have prepared : PreparedResult command value staged := by
        apply Or.inl
        simp [staged,lift,PublicationUse.apply,PublicationPayload.apply,computed]
      obtain ⟨next,continued,projection,nextPath,nextFree⟩ := lift_passive_sequence command tail
        (normalTail_passive _ _) staged (lift_projection _ _ _) stagedPath (lift_free _ _ _ free)
      have result := passive_run_result (normalTail_passive _ _) prepared continued
      have nextSettled : next.base.barrier.announced = next.base.barrier.published := by
        rw [projection]; exact settledNext
      have noPrepared : next.base.prepared.isSome = false := by
        have availability := (PublicationPayload.reached_available (PublicationUse.reached_payload nextPath)).prepared
        have notGap : ¬ next.base.barrier.published < next.base.barrier.announced := by rw [nextSettled]; omega
        cases present : next.base.prepared.isSome with
        | false => rfl
        | true => exact False.elim (notGap (availability.mp present))
      have current : next.base.current = value := by
        rcases result with preparedNext | ⟨equal,_⟩
        · rw [preparedNext] at noPrepared; cases noPrepared
        · exact equal
      have all : ∀ i, next.base.cached i = value ∧ next.held i = none ∧
          PublicationUse.check evaluate next (.enter i) = true := by
        intro i
        have gate : next.base.barrier.installed i = next.base.barrier.fence i := by
          rw [projection]; exact (openAll i).1.trans (openAll i).2.symm
        have payloadAllowed : PublicationPayload.Allowed evaluate next.base (.use i) := ⟨gate,True.intro⟩
        have checkUse := (PublicationPayload.check_exact _ _ _).mpr payloadAllowed
        have cached := PublicationPayload.enabled_payload_current (PublicationUse.reached_payload nextPath) i checkUse
        exact ⟨cached.trans current,nextFree i,
          (PublicationUse.check_exact _ _ _).mpr ⟨payloadAllowed,nextFree i⟩⟩
      refine ⟨next,?_,nextPath,current,?_,nextSettled,all,?_⟩
      · simpa [normal_eq,useRun,stagedRun] using continued
      · rw [projection]; exact published
      · exact ⟨⟨0,nonempty⟩,(all ⟨0,nonempty⟩).2.2⟩

#print axioms passive_run_result
#print axioms normal_payload_succeeds
end MirroreaProofFirst.PublicationProgress
