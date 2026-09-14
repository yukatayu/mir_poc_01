import Std

namespace MirroreaProofFirst.Publication

-- Exploratory generation-fence protocol, not a semantic authority issuer.
-- One fresh realm and fixed finite participant set. Full head/payload binding,
-- authentic delivery, runtime exclusion, restart and Session refinement are
-- separate obligations. Retained packets permit arbitrary duplicate/reordering.
structure State (n : Nat) where
  published : Nat
  announced : Nat
  installed : Fin n → Nat
  fence : Fin n → Nat
  ack : Fin n → Nat
  stops : List Nat
  frozen : List (Fin n × Nat)
  certificates : List Nat

def initial (n generation : Nat) : State n :=
  ⟨generation,generation,fun _ => generation,fun _ => generation,
    fun _ => 0,[],[],[generation]⟩

def put (values : Fin n → Nat) (i : Fin n) (value : Nat) : Fin n → Nat :=
  fun j => if j = i then value else values j

inductive Action (n : Nat) where
  | begin
  | freeze (i : Fin n) (generation : Nat)
  | acknowledge (i : Fin n) (generation : Nat)
  | publish
  | install (i : Fin n) (generation : Nat)
  | use (i : Fin n)

-- Declarative permission does not contain the desired invariant or checker.
def Allowed (s : State n) : Action n → Prop
  | .begin => s.announced = s.published
  | .freeze _ g => g ∈ s.stops
  | .acknowledge i g => (i,g) ∈ s.frozen
  | .publish => s.published < s.announced ∧ ∀ i, s.announced ≤ s.ack i
  | .install i g => g ∈ s.certificates ∧ s.fence i ≤ g
  | .use i => s.installed i = s.fence i

def check (s : State n) : Action n → Bool
  | .begin => decide (s.announced = s.published)
  | .freeze _ g => s.stops.contains g
  | .acknowledge i g => s.frozen.contains (i,g)
  | .publish => decide (s.published < s.announced) &&
      (List.finRange n).all (fun i => decide (s.announced ≤ s.ack i))
  | .install i g => s.certificates.contains g && decide (s.fence i ≤ g)
  | .use i => decide (s.installed i = s.fence i)

theorem check_exact (s : State n) (action : Action n) :
    check s action = true ↔ Allowed s action := by
  cases action <;> simp [check,Allowed,List.all_eq_true]

def apply (s : State n) : Action n → State n
  | .begin => {s with announced := s.published + 1, stops := (s.published + 1) :: s.stops}
  | .freeze i g => {s with fence := put s.fence i (max (s.fence i) g), frozen := (i,g) :: s.frozen}
  | .acknowledge i g => {s with ack := put s.ack i (max (s.ack i) g)}
  | .publish => {s with published := s.announced, certificates := s.announced :: s.certificates}
  | .install i g => {s with installed := put s.installed i g}
  | .use _ => s

def execute (s : State n) (action : Action n) : Option (State n) :=
  if check s action then some (apply s action) else none

inductive Step : State n → State n → Prop where
  | action : Allowed s action → Step s (apply s action)

theorem execute_exact (s next : State n) (action : Action n) :
    execute s action = some next ↔ Allowed s action ∧ next = apply s action := by
  simp only [execute]
  split
  · rename_i accepted
    constructor
    · intro equal
      exact ⟨(check_exact _ _).mp accepted,(Option.some.inj equal).symm⟩
    · intro pair
      exact congrArg some pair.2.symm
  · rename_i denied
    constructor
    · intro impossible; cases impossible
    · intro pair
      exact False.elim (denied ((check_exact _ _).mpr pair.1))

-- Concrete inductive strengthening, not a field of State or Step premise.
def Invariant (s : State n) : Prop :=
  s.published ≤ s.announced ∧
  (∀ i, s.installed i ≤ s.published ∧ s.published ≤ s.fence i ∧ s.ack i ≤ s.fence i) ∧
  (∀ pair ∈ s.frozen, pair.2 ≤ s.fence pair.1) ∧
  (∀ g ∈ s.certificates, g ≤ s.published)

theorem initial_invariant (n generation : Nat) : Invariant (initial n generation) := by
  simp [Invariant,initial]

theorem apply_preserves (s : State n) (action : Action n)
    (valid : Invariant s) (allowed : Allowed s action) : Invariant (apply s action) := by
  obtain ⟨announced,nodes,frozen,certificates⟩ := valid
  cases action with
  | begin =>
      exact ⟨Nat.le_succ _,nodes,frozen,certificates⟩
  | freeze i g =>
      refine ⟨announced,?_,?_,certificates⟩
      · intro j
        by_cases equal : j = i
        · subst j
          simp only [apply,put,ite_true]
          obtain ⟨l,p,a⟩ := nodes i
          exact ⟨l,Nat.le_trans p (Nat.le_max_left _ _),Nat.le_trans a (Nat.le_max_left _ _)⟩
        · simpa [apply,put,equal] using nodes j
      · intro pair present
        rcases List.mem_cons.mp present with rfl | prior
        · simp only [apply,put,ite_true]
          exact Nat.le_max_right _ _
        · by_cases equal : pair.1 = i
          · have old := frozen pair prior
            rw [equal] at old
            simpa only [apply,put,equal,ite_true] using Nat.le_trans old (Nat.le_max_left (s.fence i) g)
          · simpa [apply,put,equal] using frozen pair prior
  | acknowledge i g =>
      refine ⟨announced,?_,frozen,certificates⟩
      intro j
      by_cases equal : j = i
      · subst j
        simp only [apply,put,ite_true]
        refine ⟨(nodes i).1,(nodes i).2.1,?_⟩
        have one := (nodes i).2.2
        have two := frozen (i,g) allowed
        dsimp only at two
        omega
      · simpa [apply,put,equal] using nodes j
  | publish =>
      obtain ⟨newer,allAck⟩ := allowed
      refine ⟨Nat.le_refl _,?_,frozen,?_⟩
      · intro i
        exact ⟨Nat.le_trans (nodes i).1 announced,Nat.le_trans (allAck i) (nodes i).2.2,(nodes i).2.2⟩
      · intro g present
        rcases List.mem_cons.mp present with rfl | old
        · exact Nat.le_refl _
        · exact Nat.le_trans (certificates g old) announced
  | install i g =>
      refine ⟨announced,?_,frozen,certificates⟩
      intro j
      by_cases equal : j = i
      · subst j
        simp only [apply,put,ite_true]
        exact ⟨certificates g allowed.1,(nodes i).2⟩
      · simpa [apply,put,equal] using nodes j
  | use i => exact ⟨announced,nodes,frozen,certificates⟩

inductive Reached (n generation : Nat) : State n → Prop where
  | initial : Reached n generation (Publication.initial n generation)
  | step : Reached n generation s → Step s next → Reached n generation next

theorem reached_invariant (path : Reached n generation s) : Invariant s := by
  induction path with
  | initial => exact initial_invariant _ _
  | step _ step ih =>
      cases step with
      | action permitted => exact apply_preserves _ _ ih permitted

theorem enabled_current (path : Reached n generation s) (i : Fin n)
    (enabled : check s (.use i) = true) : s.installed i = s.published := by
  have same : s.installed i = s.fence i := (check_exact _ _).mp enabled
  have bounds := (reached_invariant path).2.1 i
  omega

theorem stale_install_rejected (path : Reached n generation s) (i : Fin n) (g : Nat)
    (stale : g < s.published) : execute s (.install i g) = none := by
  have bound := (reached_invariant path).2.1 i
  have denied : ¬ s.fence i ≤ g := by omega
  simp [execute,check,denied]

theorem publish_excludes_old_use (s : State n) (valid : Invariant s)
    (allowed : Allowed s .publish) (i : Fin n) : check (apply s .publish) (.use i) = false := by
  have bounds := valid.2.1 i
  have stopped := allowed.2 i
  have newer := allowed.1
  have different : s.installed i ≠ s.fence i := by omega
  simp [apply,check,different]

theorem initial_use_allowed (n generation : Nat) (i : Fin n) :
    execute (initial n generation) (.use i) = some (initial n generation) := by
  simp [execute,check,initial,apply]

-- Unsafe comparison, never an admitted Step. The caller's local gate is
-- unchanged when only the global/owner publication is advanced.
def ownerOnlyPublish (s : State n) (generation : Nat) : State n :=
  {s with published := generation}

theorem owner_only_gap (s : State n) (i : Fin n) (generation : Nat)
    (openGate : Allowed s (.use i)) (current : s.installed i = s.published)
    (newer : s.published < generation) :
    check (ownerOnlyPublish s generation) (.use i) = true ∧
      (ownerOnlyPublish s generation).installed i ≠ (ownerOnlyPublish s generation).published := by
  have gate : s.installed i = s.fence i := openGate
  have distinct : s.installed i ≠ generation := by omega
  constructor
  · simp [ownerOnlyPublish,check,gate]
  · exact distinct

#print axioms check_exact
#print axioms execute_exact
#print axioms apply_preserves
#print axioms reached_invariant
#print axioms enabled_current
#print axioms stale_install_rejected
#print axioms publish_excludes_old_use
#print axioms initial_use_allowed
#print axioms owner_only_gap

end MirroreaProofFirst.Publication
