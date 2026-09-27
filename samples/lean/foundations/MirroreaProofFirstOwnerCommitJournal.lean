import MirroreaProofFirstOwnerReservationMonitor
import MirroreaProofFirstOwnerImageMonitor
import MirroreaProofFirstOwnerCreditCustody
namespace MirroreaProofFirst.OwnerCommitJournal

-- Statement-level local stores of the selected CreditWriter, after a known
-- typed native reply. These are not atomic native transitions or authorization.
-- The outer caller must bind this receipt to its exact owned wire occurrence,
-- retain the gate, and compose its further cohort stores. This module does not
-- certify those additional premises or the Python bytecode/OS by itself.
structure Data (p a : Nat) where
  credits : Nat
  image : Option (OwnerImage.Image p a)
  revision : Nat
  keys : Option (List (Nat × Nat))

structure Memory (p a : Nat) where
  data : Data p a
  lease : Option InvocationBoundary.Ticket
  entered : Bool

def imageEq : Option (OwnerImage.Image p a) → Option (OwnerImage.Image p a) → Bool
  | none,none => true
  | some left,some right => OwnerEndpoint.sameImage left right
  | _,_ => false

theorem imageEq_exact : imageEq left right = true ↔ left = right := by
  cases left <;> cases right <;> simp [imageEq,OwnerEndpoint.sameImage_exact]

def dataEq (left right : Data p a) : Bool :=
  decide (left.credits = right.credits) && imageEq left.image right.image &&
    decide (left.revision = right.revision) && decide (left.keys = right.keys)

theorem dataEq_exact : dataEq left right = true ↔ left = right := by
  cases left; cases right
  simp only [dataEq,Bool.and_eq_true,decide_eq_true_eq,imageEq_exact,Data.mk.injEq]
  simp only [and_assoc]

def memoryEq (left right : Memory p a) : Bool :=
  dataEq left.data right.data && decide (left.lease = right.lease) && decide (left.entered = right.entered)

theorem memoryEq_exact : memoryEq left right = true ↔ left = right := by
  cases left; cases right
  simp only [memoryEq,Bool.and_eq_true,decide_eq_true_eq,dataEq_exact,Memory.mk.injEq]
  simp only [and_assoc]

-- Admission is separate from postreply store correspondence. This condition
-- matches the selected writer's local leased-command check; authentic staging
-- and source-enter provenance must additionally be supplied by the outer path.
def maySend (memory : Memory p a) (command : OwnerEndpoint.Command p a) : Bool :=
  match memory.lease with
  | none => true
  | some ticket => memory.entered && match command with
      | .owner (.reserve submitted) => decide (submitted = ticket)
      | _ => false

inductive SendAllowed : Memory p a → OwnerEndpoint.Command p a → Prop where
  | unleased : SendAllowed ⟨data,none,entered⟩ command
  | reserved : SendAllowed ⟨data,some ticket,true⟩ (.owner (.reserve ticket))

theorem maySend_exact : maySend memory command = true ↔ SendAllowed memory command := by
  constructor
  · intro checked
    rcases memory with ⟨data,lease,entered⟩
    cases lease with
    | none => exact .unleased
    | some ticket =>
      cases entered with
      | false => simp [maySend] at checked
      | true =>
        cases command with
        | freeze revision => simp [maySend] at checked
        | owner command =>
          cases command <;> simp [maySend] at checked
          cases checked
          exact .reserved
  · intro allowed
    cases allowed <;> simp [maySend]

def project (state : OwnerEndpointBudget.State p a) : Data p a :=
  let current := OwnerImageMonitor.project state.owner
  ⟨state.remaining,current.map Prod.snd,(current.map Prod.fst).getD 0,
    (OwnerReservationMonitor.project state.owner).map Prod.snd⟩

inductive Store (p a : Nat) where
  | credits (value : Nat)
  | image (value : OwnerImage.Image p a)
  | revision (value : Nat)
  | keys (value : Option (List (Nat × Nat)))
  | releaseLease
  | clearEntered

def write (memory : Memory p a) : Store p a → Memory p a
  | .credits value => {memory with data := {memory.data with credits:=value}}
  | .image value => {memory with data := {memory.data with image:=some value}}
  | .revision value => {memory with data := {memory.data with revision:=value}}
  | .keys value => {memory with data := {memory.data with keys:=value}}
  | .releaseLease => {memory with lease:=none}
  | .clearEntered => {memory with entered:=false}

def stores (before : Memory p a) (command : OwnerEndpoint.Command p a)
    (reply : Sum Nat OwnerReceipt.Envelope) : List (Store p a) :=
  (if OwnerCreditCustody.budgetRefused reply then [] else [.credits (before.data.credits-1)]) ++
  (match command,reply with
    | .owner (.initialize image),.inl 10 => [.image image,.revision 0,.keys (some [])]
    | .owner (.reserve ticket),.inl 6 => [.keys (before.data.keys.map (OwnerOccurrence.key ticket :: ·))]
    | .owner (.install revision image),.inl 7 => [.revision revision,.image image]
    | _,_ => []) ++ [.releaseLease,.clearEntered]

def effects (before : Data p a) (command : OwnerEndpoint.Command p a)
    (reply : Sum Nat OwnerReceipt.Envelope) : Data p a :=
  let debited := {before with credits := OwnerCreditCustody.observe before.credits reply}
  match command,reply with
  | .owner (.initialize image),.inl 10 => {debited with image:=some image,revision:=0,keys:=some []}
  | .owner (.reserve ticket),.inl 6 => {debited with keys:=before.keys.map (OwnerOccurrence.key ticket :: ·)}
  | .owner (.install revision image),.inl 7 => {debited with revision:=revision,image:=some image}
  | _,_ => debited

theorem stores_effects (before : Memory p a) (command : OwnerEndpoint.Command p a)
    (reply : Sum Nat OwnerReceipt.Envelope) :
    (stores before command reply).foldl write before = ⟨effects before.data command reply,none,false⟩ := by
  cases reply with
  | inr envelope => cases command with
    | freeze revision => rfl
    | owner command => cases command <;> rfl
  | inl code =>
    by_cases ten : code = 10
    · subst code; cases command with
      | freeze revision => rfl
      | owner command => cases command <;> rfl
    by_cases six : code = 6
    · subst code; cases command with
      | freeze revision => rfl
      | owner command => cases command <;> rfl
    by_cases seven : code = 7
    · subst code; cases command with
      | freeze revision => rfl
      | owner command => cases command <;> rfl
    by_cases refused : code = 16
    · subst code; cases command with
      | freeze revision => rfl
      | owner command => cases command <;> rfl
    cases command with
    | freeze revision => simp [stores,effects,write,OwnerCreditCustody.observe,OwnerCreditCustody.budgetRefused,refused]
    | owner command =>
      cases command <;> simp [stores,effects,write,OwnerCreditCustody.observe,OwnerCreditCustody.budgetRefused,ten,six,seven,refused]

-- This correspondence is derived from actual native execution and the three
-- existing proved monitors; a desired successor projection is NOT an input
-- guard on an individual store transition.
theorem effects_project
    (ran : OwnerEndpointBudget.transition assigned scope capacity state command = (next,reply)) :
    effects (project state) command reply = project next := by
  have credits := OwnerCreditCustody.observe_exact ran
  have image := OwnerImageMonitor.budget_exact ran
  have keys := OwnerReservationMonitor.budget_exact ran
  unfold project
  rw [image,keys,←credits]
  cases reply with
  | inr envelope => cases command with
    | freeze revision => rfl
    | owner command => cases command <;> rfl
  | inl code =>
    by_cases ten : code = 10
    · subst code; cases command with
      | freeze revision => rfl
      | owner command => cases command <;> rfl
    by_cases six : code = 6
    · subst code; cases command with
      | freeze revision => rfl
      | owner command => cases command <;> simp [effects,OwnerImageMonitor.observe,OwnerReservationMonitor.observe,Option.map_map,Function.comp_def]
    by_cases seven : code = 7
    · subst code; cases command with
      | freeze revision => rfl
      | owner command => cases command <;> rfl
    cases command with
    | freeze revision => rfl
    | owner command => cases command <;> simp [effects,OwnerImageMonitor.observe,OwnerReservationMonitor.observe,ten,six,seven]

structure Journal (p a : Nat) where
  memory : Memory p a
  pending : List (Store p a)
  stopped : Bool

inductive Action where
  | commit
  | stop
  deriving DecidableEq, Repr

-- The residual list is an operation-bound recipe, not a set of desired final
-- equalities. Each commit performs the actual head assignment and frames every
-- other field. Retirement keeps both the current memory and remaining stores.
inductive Step : Journal p a → Action → Journal p a → Prop where
  | commit : Step ⟨memory,store::rest,false⟩ .commit ⟨write memory store,rest,false⟩
  | stop : Step ⟨memory,pending,false⟩ .stop ⟨memory,pending,true⟩

def advance : Journal p a → Action → Option (Journal p a)
  | ⟨memory,store::rest,false⟩,.commit => some ⟨write memory store,rest,false⟩
  | ⟨memory,pending,false⟩,.stop => some ⟨memory,pending,true⟩
  | _,_ => none

theorem advance_exact : advance s action = some next ↔ Step s action next := by
  constructor
  · intro checked
    rcases s with ⟨memory,pending,stopped⟩
    cases pending <;> cases stopped <;> cases action <;> simp only [advance] at checked <;> try contradiction
    all_goals cases Option.some.inj checked; constructor
  · intro step
    cases step with
    | commit => rfl
    | stop => rename_i memory pending; cases pending <;> rfl

def target (s : Journal p a) : Memory p a := s.pending.foldl write s.memory

theorem preserves_target (step : Step s action after) : target after = target s := by
  cases step <;> rfl

inductive Runs (base : Journal p a) : Journal p a → Prop where
  | nil : Runs base base
  | step : Runs base s → Step s action next → Runs base next

theorem runs_target (path : Runs base next) : target next = target base := by
  induction path with
  | nil => rfl
  | step prior step ih => exact (preserves_target step).trans ih

theorem Runs.trans (left : Runs base middle) (right : Runs middle after) : Runs base after := by
  induction right with
  | nil => exact left
  | step prior step ih => exact .step ih step

theorem discharge (memory : Memory p a) (pending : List (Store p a)) :
    Runs ⟨memory,pending,false⟩ ⟨pending.foldl write memory,[],false⟩ := by
  induction pending generalizing memory with
  | nil => exact .nil
  | cons store rest ih =>
      have first : Step ⟨memory,store::rest,false⟩ .commit ⟨write memory store,rest,false⟩ := .commit
      exact (Runs.step Runs.nil first).trans (ih (write memory store))

theorem completed_exact
    (binding : before.data = project state)
    (ran : OwnerEndpointBudget.transition assigned scope capacity state command = (next,reply))
    (path : Runs ⟨before,stores before command reply,false⟩ ⟨after,[],stopped⟩) :
    after.data = project next ∧ after.lease = none ∧ after.entered = false := by
  have exact := runs_target path
  change after = (stores before command reply).foldl write before at exact
  rw [stores_effects,binding,effects_project ran] at exact
  cases exact
  exact ⟨rfl,rfl,rfl⟩

theorem no_commit_after_stop : advance (⟨memory,pending,true⟩ : Journal p a) .commit = none := by
  cases pending <;> rfl

theorem stop_retains (step : Step s .stop after) :
    after.memory = s.memory ∧ after.pending = s.pending := by
  cases step; exact ⟨rfl,rfl⟩

-- Finite sampled-memory consumer. The cursor is existential compatible-prefix
-- evidence; equal/idempotent stores can make a sampled memory ambiguous. Actual
-- program-point binding is separate and must not be inferred from this search.
def seek (memory : Memory p a) (pending : List (Store p a)) (observed : Memory p a) : Option Nat :=
  if memoryEq memory observed then some 0 else
    match pending with
    | [] => none
    | store::rest => (seek (write memory store) rest observed).map Nat.succ

theorem seek_sound (checked : seek memory pending observed = some count) :
    count ≤ pending.length ∧
    observed = (pending.take count).foldl write memory ∧
    Runs ⟨memory,pending,false⟩ ⟨observed,pending.drop count,false⟩ := by
  induction pending generalizing memory count with
  | nil =>
      simp only [seek] at checked
      split at checked
      · rename_i equal
        cases Option.some.inj checked
        have same := memoryEq_exact.mp equal
        subst observed
        exact ⟨by simp,rfl,.nil⟩
      · cases checked
  | cons store rest ih =>
      simp only [seek] at checked
      split at checked
      · rename_i equal
        cases Option.some.inj checked
        have same := memoryEq_exact.mp equal
        subst observed
        exact ⟨by simp,rfl,.nil⟩
      · cases found : seek (write memory store) rest observed with
        | none => simp [found] at checked
        | some index =>
          simp only [found,Option.map_some,Option.some.injEq] at checked
          subst count
          obtain ⟨bound,atMemory,path⟩ := ih found
          refine ⟨by simp; omega,?_,?_⟩
          · simpa only [List.take_succ_cons,List.foldl_cons] using atMemory
          · exact (Runs.step Runs.nil Step.commit).trans path

-- Every actual finite prefix has a recognizable sampled memory, including zero
-- effective field changes; the result need not be that unique instruction count.
theorem seek_complete (memory : Memory p a) (pending : List (Store p a)) (count : Nat)
    (bound : count ≤ pending.length) :
    (seek memory pending ((pending.take count).foldl write memory)).isSome = true := by
  induction pending generalizing memory count with
  | nil =>
      have zero : count = 0 := by simpa using bound
      subst count
      simp [seek,memoryEq_exact.mpr rfl]
  | cons store rest ih =>
      cases count with
      | zero => simp [seek,memoryEq_exact.mpr rfl]
      | succ count =>
        have tailBound : count ≤ rest.length := by simpa using bound
        have restFound := ih (write memory store) count tailBound
        simp only [List.take_succ_cons,List.foldl_cons]
        unfold seek
        split
        · rfl
        · simpa only [Option.isSome_map] using restFound

#print axioms maySend_exact
#print axioms imageEq_exact
#print axioms dataEq_exact
#print axioms memoryEq_exact
#print axioms seek_sound
#print axioms seek_complete
#print axioms stores_effects
#print axioms effects_project
#print axioms advance_exact
#print axioms preserves_target
#print axioms runs_target
#print axioms discharge
#print axioms completed_exact
#print axioms no_commit_after_stop
#print axioms stop_retains
end MirroreaProofFirst.OwnerCommitJournal
