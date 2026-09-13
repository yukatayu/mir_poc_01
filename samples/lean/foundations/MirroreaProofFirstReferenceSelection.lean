import MirroreaProofFirstReferenceAccessHistory

namespace MirroreaProofFirst.ReferenceSelection
open ReferenceAccess

structure Choice where
  index : Nat
  guard : Guard
  deriving DecidableEq, Repr

def atIndex (request : Request) (index : Nat) : Request := {request with optionIndex := index}

def AdmissibleIndex (s : ManagementEntry.System p a) (policy : CurrentUse.Policy)
    (request : Request) (index : Nat) : Prop :=
  ∃ option member reader key place, ReferenceAccess.Allowed s policy (atIndex request index) option member reader key place

theorem admissible_iff_prepared (s : ManagementEntry.System p a) (policy : CurrentUse.Policy)
    (request : Request) (index : Nat) :
    AdmissibleIndex s policy request index ↔ ∃ guard, prepare s policy (atIndex request index) = some guard := by
  constructor
  · exact prepare_complete _ _ _
  · rintro ⟨guard,produced⟩
    obtain ⟨option,member,reader,key,place,allowed,_⟩ := check_sound _ _ _ _ (prepare_checked _ _ _ _ produced)
    exact ⟨option,member,reader,key,place,allowed⟩

def pick (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (request : Request) : List Nat → Option Choice
  | [] => none
  | index :: rest => match prepare s policy (atIndex request index) with
      | some guard => some ⟨index,guard⟩
      | none => pick s policy request rest

-- An independent leftmost judgment excludes every skipped admissible option.
-- The returned exact guard remains part of the success meaning.
inductive First (s : ManagementEntry.System p a) (policy : CurrentUse.Policy)
    (request : Request) : List Nat → Choice → Prop where
  | here {guard : Guard} : Saved s policy (atIndex request index) guard → First s policy request (index :: rest) ⟨index,guard⟩
  | later : ¬ AdmissibleIndex s policy request index → First s policy request rest choice →
      First s policy request (index :: rest) choice

theorem none_iff (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (request : Request) (indices : List Nat) :
    pick s policy request indices = none ↔ ∀ index ∈ indices, ¬ AdmissibleIndex s policy request index := by
  induction indices with
  | nil => simp [pick]
  | cons index rest ih =>
      cases hp : prepare s policy (atIndex request index) with
      | none =>
          have denied : ¬ AdmissibleIndex s policy request index := by
            rw [admissible_iff_prepared]; simp [hp]
          simp [pick,hp,ih,denied]
      | some guard =>
          have allowed := (admissible_iff_prepared _ _ _ _).mpr ⟨guard,hp⟩
          simp [pick,hp,allowed]

theorem pick_sound (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (request : Request)
    (indices : List Nat) (choice : Choice) (picked : pick s policy request indices = some choice) :
    First s policy request indices choice := by
  induction indices with
  | nil => simp [pick] at picked
  | cons index rest ih =>
      cases hp : prepare s policy (atIndex request index) with
      | none =>
          have denied : ¬ AdmissibleIndex s policy request index := by rw [admissible_iff_prepared]; simp [hp]
          exact .later denied (ih (by simpa [pick,hp] using picked))
      | some guard =>
          have eq : choice = ⟨index,guard⟩ := by simpa [pick,hp] using picked.symm
          subst choice
          exact .here ((check_exact _ _ _ _).mp (prepare_checked _ _ _ _ hp))

theorem pick_complete (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (request : Request)
    (indices : List Nat) (existsOption : ∃ index ∈ indices, AdmissibleIndex s policy request index) :
    ∃ choice, pick s policy request indices = some choice := by
  cases selected : pick s policy request indices with
  | some choice => exact ⟨choice,rfl⟩
  | none =>
      obtain ⟨index,mem,allowed⟩ := existsOption
      exact False.elim ((none_iff _ _ _ _).mp selected index mem allowed)

theorem first_member (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (request : Request)
    (indices : List Nat) (choice : Choice) (selected : First s policy request indices choice) :
    choice.index ∈ indices ∧ Saved s policy (atIndex request choice.index) choice.guard := by
  induction selected with
  | here saved => exact ⟨by simp,saved⟩
  | later _ _ ih => exact ⟨by simp [ih.1],ih.2⟩

def candidates (request : Request) (start : Nat) : List Nat :=
  (List.range request.chain.options.length).filter (fun index => decide (start ≤ index))

theorem candidates_exact (request : Request) (start index : Nat) :
    index ∈ candidates request start ↔ start ≤ index ∧ index < request.chain.options.length := by
  simp [candidates,and_comm]

def search (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (request : Request) (start : Nat) : Option Choice :=
  pick s policy request (candidates request start)

theorem search_bounds (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (request : Request)
    (start : Nat) (choice : Choice) (selected : search s policy request start = some choice) :
    start ≤ choice.index ∧ choice.index < request.chain.options.length ∧
      Saved s policy (atIndex request choice.index) choice.guard := by
  have evidence := first_member _ _ _ _ _ (pick_sound _ _ _ _ _ selected)
  have bounds := (candidates_exact _ _ _).mp evidence.1
  exact ⟨bounds.1,bounds.2,evidence.2⟩

theorem search_after_strict (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (request : Request)
    (old : Nat) (choice : Choice) (selected : search s policy request (old+1) = some choice) : old < choice.index :=
  (search_bounds _ _ _ _ _ selected).1

theorem search_complete (s : ManagementEntry.System p a) (policy : CurrentUse.Policy) (request : Request)
    (start : Nat) (existsOption : ∃ index, start ≤ index ∧ index < request.chain.options.length ∧
      AdmissibleIndex s policy request index) : ∃ choice, search s policy request start = some choice := by
  obtain ⟨index,lower,upper,allowed⟩ := existsOption
  exact pick_complete _ _ _ _ ⟨index,(candidates_exact _ _ _).mpr ⟨lower,upper⟩,allowed⟩

#print axioms admissible_iff_prepared
#print axioms none_iff
#print axioms pick_sound
#print axioms pick_complete
#print axioms first_member
#print axioms candidates_exact
#print axioms search_bounds
#print axioms search_after_strict
#print axioms search_complete
end MirroreaProofFirst.ReferenceSelection
