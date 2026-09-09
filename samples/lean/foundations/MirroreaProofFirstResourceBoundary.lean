import Std
namespace MirroreaProofFirst.ResourceBoundary
structure Region where
 block : Nat
 lo : Nat
 hi : Nat
 holder : Nat
 deriving DecidableEq, Repr
structure Handle where
 id : Nat
 region : Region
 deriving DecidableEq, Repr
structure State where
 nextId : Nat
 nextBlock : Nat
 live : Nat → Option Region

def Nonempty (a : Region) : Prop := a.lo < a.hi
def Sep (a b : Region) : Prop :=
 a.block ≠ b.block ∨ a.hi ≤ b.lo ∨ b.hi ≤ a.lo
def Subregion (a b : Region) : Prop :=
 a.block = b.block ∧ b.lo ≤ a.lo ∧ a.hi ≤ b.hi
structure WF (s : State) : Prop where
 bounded : ∀ i r, s.live i = some r → i < s.nextId ∧ r.block < s.nextBlock ∧ Nonempty r
 separated : ∀ i j a b, i ≠ j → s.live i = some a → s.live j = some b → Sep a b

theorem sep_symm {a b : Region} (h : Sep a b) : Sep b a := by
 unfold Sep at *; rcases h with h | h | h
 · exact Or.inl (Ne.symm h)
 · exact Or.inr (Or.inr h)
 · exact Or.inr (Or.inl h)
theorem sep_sub_left {a b c : Region} (h : Sep b c) (sub : Subregion a b) : Sep a c := by
 unfold Sep Subregion at *; rcases sub with ⟨hb,hl,hh⟩
 rcases h with h | h | h
 · exact Or.inl (by simpa [hb] using h)
 · exact Or.inr (Or.inl (Nat.le_trans hh h))
 · exact Or.inr (Or.inr (Nat.le_trans h hl))
theorem sep_sub_right {a b c : Region} (h : Sep a b) (sub : Subregion c b) : Sep a c :=
 sep_symm (sep_sub_left (sep_symm h) sub)

def erase (s : State) (id : Nat) : State :=
 {s with live := fun i => if i = id then none else s.live i}
def push (s : State) (r : Region) (nb : Nat) : State :=
 {nextId := s.nextId+1, nextBlock := nb,
  live := fun i => if i = s.nextId then some r else s.live i}
def newHandle (s : State) (r : Region) : Handle := ⟨s.nextId,r⟩
def Current (s : State) (p : Nat) (h : Handle) : Prop :=
 s.live h.id = some h.region ∧ h.region.holder = p

theorem erase_wf {s : State} (w : WF s) (id : Nat) : WF (erase s id) := by
 constructor
 · intro i r h; simp only [erase] at h ⊢; split at h
   · contradiction
   · exact w.bounded i r h
 · intro i j a b ne ha hb
   simp only [erase] at ha hb
   split at ha
   · contradiction
   · split at hb
     · contradiction
     · exact w.separated i j a b ne ha hb

theorem push_wf {s : State} (w : WF s) (r : Region) (nb : Nat)
 (hn : Nonempty r) (hb : r.block < nb) (old : s.nextBlock ≤ nb)
 (sep : ∀ i a, s.live i = some a → Sep r a) : WF (push s r nb) := by
 constructor
 · intro i a h
   by_cases eq : i = s.nextId
   · simp [push,eq] at h
     subst a; simp [push,eq]; exact ⟨hb,hn⟩
   · simp [push,eq] at h
     obtain ⟨hi,ha,hn'⟩ := w.bounded i a h
     exact ⟨by simp [push]; omega, Nat.lt_of_lt_of_le ha old, hn'⟩
 · intro i j a b ne ha hb'
   by_cases ei : i = s.nextId
   · have ej : j ≠ s.nextId := by omega
     simp [push,ei] at ha; subst a
     simp [push,ej] at hb'
     exact sep j b hb'
   · by_cases ej : j = s.nextId
     · simp [push,ej] at hb'; subst b
       simp [push,ei] at ha
       exact sep_symm (sep i a ha)
     · simp [push,ei] at ha; simp [push,ej] at hb'
       exact w.separated i j a b ne ha hb'

def allocRegion (s : State) (p n : Nat) : Region := ⟨s.nextBlock,0,n,p⟩
def allocateRaw (s : State) (p n : Nat) : State × Handle :=
 let r := allocRegion s p n
 (push s r (s.nextBlock+1), newHandle s r)

theorem allocate_wf {s : State} (w : WF s) (p n : Nat) (positive : 0 < n) :
 WF (allocateRaw s p n).1 := by
 apply push_wf w
 · exact positive
 · simp [allocRegion]
 · omega
 · intro i a ha
   have := (w.bounded i a ha).2.1
   exact Or.inl (by simp [allocRegion]; omega)

def moved (h : Handle) (q : Nat) : Region := {h.region with holder := q}
def moveRaw (s : State) (h : Handle) (q : Nat) : State × Handle :=
 let base := erase s h.id
 let r := moved h q
 (push base r s.nextBlock, newHandle base r)

theorem survivor_separated {s : State} (w : WF s) {h : Handle}
 (live : s.live h.id = some h.region) :
 ∀ i a, (erase s h.id).live i = some a → Sep h.region a := by
 intro i a ha
 simp only [erase] at ha; split at ha
 · contradiction
 · rename_i ne
   exact w.separated h.id i h.region a (Ne.symm ne) live ha

theorem move_wf {s : State} (w : WF s) {h : Handle}
 (live : s.live h.id = some h.region) (q : Nat) : WF (moveRaw s h q).1 := by
 obtain ⟨_,hb,hn⟩ := w.bounded h.id h.region live
 apply push_wf (erase_wf w h.id)
 · exact hn
 · exact hb
 · exact Nat.le_refl _
 · intro i a ha
   exact survivor_separated w live i a ha

def leftPart (h : Handle) (k : Nat) : Region := {h.region with hi := k}
def rightPart (h : Handle) (k : Nat) : Region := {h.region with lo := k}
def splitRaw (s : State) (h : Handle) (k : Nat) : State × Handle × Handle :=
 let base := erase s h.id
 let a := leftPart h k
 let b := rightPart h k
 let mid := push base a s.nextBlock
 (push mid b s.nextBlock, newHandle base a, newHandle mid b)

theorem split_wf {s : State} (w : WF s) {h : Handle}
 (live : s.live h.id = some h.region) (k : Nat)
 (interior : h.region.lo < k ∧ k < h.region.hi) : WF (splitRaw s h k).1 := by
 have hb := (w.bounded h.id h.region live).2.1
 have la : Subregion (leftPart h k) h.region := ⟨rfl, Nat.le_refl _,Nat.le_of_lt interior.2⟩
 have rb : Subregion (rightPart h k) h.region := ⟨rfl,Nat.le_of_lt interior.1,Nat.le_refl _⟩
 have mid : WF (push (erase s h.id) (leftPart h k) s.nextBlock) := by
   apply push_wf (erase_wf w h.id) (leftPart h k) s.nextBlock interior.1 hb (Nat.le_refl _)
   intro i a ha; exact sep_sub_left (survivor_separated w live i a ha) la
 apply push_wf mid (rightPart h k) s.nextBlock interior.2 hb (Nat.le_refl _)
 intro i a ha
 by_cases eq : i = (erase s h.id).nextId
 · simp [push,eq] at ha; subst a
   exact Or.inr (Or.inr (Nat.le_refl _))
 · simp [push,eq] at ha
   exact sep_sub_left (survivor_separated w live i a ha) rb

inductive Action where
 | allocate (principal length : Nat)
 | release (principal : Nat) (handle : Handle)
 | move (principal : Nat) (handle : Handle) (receiver : Nat)
 | split (principal : Nat) (handle : Handle) (cut : Nat)
 deriving DecidableEq, Repr

-- Policy is independently supplied, never manufactured by arithmetic or ownership.
inductive Allowed (policy : Nat → Bool) (s : State) : Action → Prop where
 | allocate {p n} : policy p = true → 0 < n → Allowed policy s (.allocate p n)
 | release {p h} : policy p = true → Current s p h → Allowed policy s (.release p h)
 | move {p h q} : policy p = true → Current s p h → Allowed policy s (.move p h q)
 | split {p h k} : policy p = true → Current s p h →
     h.region.lo < k → k < h.region.hi → Allowed policy s (.split p h k)

def currentCheck (s : State) (p : Nat) (h : Handle) : Bool :=
 decide (s.live h.id = some h.region ∧ h.region.holder = p)
def check (policy : Nat → Bool) (s : State) : Action → Bool
 | .allocate p n => policy p && decide (0 < n)
 | .release p h => policy p && currentCheck s p h
 | .move p h _ => policy p && currentCheck s p h
 | .split p h k => policy p && currentCheck s p h && decide (h.region.lo < k ∧ k < h.region.hi)

theorem current_exact (s : State) (p : Nat) (h : Handle) :
 currentCheck s p h = true ↔ Current s p h := by simp [currentCheck,Current]
theorem check_exact (policy : Nat → Bool) (s : State) (a : Action) :
 check policy s a = true ↔ Allowed policy s a := by
 cases a with
 | allocate p n =>
   simp only [check,Bool.and_eq_true,decide_eq_true_eq]
   constructor
   · rintro ⟨h,n⟩; exact .allocate h n
   · intro h; cases h with | allocate h n => exact ⟨h,n⟩
 | release p h =>
   simp only [check,Bool.and_eq_true,current_exact]
   constructor
   · rintro ⟨p,h⟩; exact .release p h
   · intro h; cases h with | release p h => exact ⟨p,h⟩
 | move p h q =>
   simp only [check,Bool.and_eq_true,current_exact]
   constructor
   · rintro ⟨p,h⟩; exact .move p h
   · intro h; cases h with | move p h => exact ⟨p,h⟩
 | split p h k =>
   simp only [check,Bool.and_eq_true,current_exact,decide_eq_true_eq]
   constructor
   · rintro ⟨⟨p,h⟩,l,u⟩; exact .split p h l u
   · intro h; cases h with | split p h l u => exact ⟨⟨p,h⟩,l,u⟩

def raw (s : State) : Action → State × List Handle
 | .allocate p n => let r := allocateRaw s p n; (r.1,[r.2])
 | .release _ h => (erase s h.id,[])
 | .move _ h q => let r := moveRaw s h q; (r.1,[r.2])
 | .split _ h k => let r := splitRaw s h k; (r.1,[r.2.1,r.2.2])
def execute (policy : Nat → Bool) (s : State) (a : Action) : Option (State × List Handle) :=
 if check policy s a then some (raw s a) else none

theorem execution_exists (policy : Nat → Bool) (s : State) (a : Action) :
 (∃ result, execute policy s a = some result) ↔ Allowed policy s a := by
 simp [execute,← check_exact]

theorem raw_wf {policy : Nat → Bool} {s : State} (w : WF s) {a : Action}
 (allowed : Allowed policy s a) : WF (raw s a).1 := by
 cases allowed with
 | allocate _ positive => exact allocate_wf w _ _ positive
 | release _ _ => exact erase_wf w _
 | move _ current => exact move_wf w current.1 _
 | split _ current l u => exact split_wf w current.1 _ ⟨l,u⟩
theorem execute_wf {policy : Nat → Bool} {s : State} (w : WF s) {a : Action}
 {result : State × List Handle} (h : execute policy s a = some result) : WF result.1 := by
 unfold execute at h; split at h
 · rename_i accepted; cases h
   exact raw_wf w ((check_exact _ _ _).mp accepted)
 · contradiction

-- Failure leaves the state unchanged in this driver. None is a reference rejection,
-- not a selected public failure payload or disclosure label.
def advance (policy : Nat → Bool) (s : State) (a : Action) : State :=
 match execute policy s a with
 | none => s
 | some out => out.1
def run (policy : Nat → Bool) (s : State) : List Action → State
 | [] => s
 | a::tail => run policy (advance policy s a) tail

theorem advance_wf {policy : Nat → Bool} {s : State} (w : WF s) (a : Action) : WF (advance policy s a) := by
 unfold advance; split
 · exact w
 · rename_i out h; exact execute_wf w h
theorem run_wf {policy : Nat → Bool} {s : State} (w : WF s) (as : List Action) : WF (run policy s as) := by
 induction as generalizing s with
 | nil => exact w
 | cons a tail ih => exact ih (advance_wf w a)

def empty : State := ⟨0,0,fun _ => none⟩
theorem empty_wf : WF empty := by
 constructor
 · intro i r h; contradiction
 · intro i j a b ne ha hb; contradiction

theorem raw_monotone (s : State) (a : Action) :
 s.nextId ≤ (raw s a).1.nextId ∧ s.nextBlock ≤ (raw s a).1.nextBlock := by
 cases a <;> simp [raw,allocateRaw,moveRaw,splitRaw,push,erase] <;> omega
theorem raw_old_absent (s : State) (a : Action) (i : Nat)
 (old : i < s.nextId) (absent : s.live i = none) : (raw s a).1.live i = none := by
 have ne : i ≠ s.nextId := by omega
 have ne' : i ≠ s.nextId+1 := by omega
 cases a <;> simp [raw,allocateRaw,moveRaw,splitRaw,push,erase,ne,ne',absent]
theorem advance_monotone (policy : Nat → Bool) (s : State) (a : Action) :
 s.nextId ≤ (advance policy s a).nextId ∧ s.nextBlock ≤ (advance policy s a).nextBlock := by
 by_cases h : check policy s a = true
 · simpa [advance,execute,h] using raw_monotone s a
 · simp [advance,execute,h]
theorem advance_old_absent (policy : Nat → Bool) (s : State) (a : Action) (i : Nat)
 (old : i < s.nextId) (absent : s.live i = none) : (advance policy s a).live i = none := by
 by_cases h : check policy s a = true
 · simpa [advance,execute,h] using raw_old_absent s a i old absent
 · simpa [advance,execute,h] using absent
-- Quantified over all following actions, not only a fixed double-release example.
theorem run_old_absent (policy : Nat → Bool) (s : State) (as : List Action) (i : Nat)
 (old : i < s.nextId) (absent : s.live i = none) : (run policy s as).live i = none := by
 induction as generalizing s with
 | nil => exact absent
 | cons a tail ih =>
   apply ih
   · exact Nat.lt_of_lt_of_le old (advance_monotone policy s a).1
   · exact advance_old_absent policy s a i old absent

def consumed : Action → Option Handle
 | .allocate _ _ => none
 | .release _ h => some h
 | .move _ h _ => some h
 | .split _ h _ => some h

theorem consumed_absent {policy : Nat → Bool} {s : State} (w : WF s) {a : Action}
 (allowed : Allowed policy s a) {h : Handle} (hc : consumed a = some h) :
 (raw s a).1.live h.id = none ∧ h.id < (raw s a).1.nextId := by
 cases allowed with
 | allocate hp hn => contradiction
 | release hp cur =>
   simp only [consumed,Option.some.injEq] at hc; subst h
   have old := (w.bounded _ _ cur.1).1
   simp [raw,erase,old]
 | move hp cur =>
   simp only [consumed,Option.some.injEq] at hc; subst h
   have old := (w.bounded _ _ cur.1).1
   have ne : _ ≠ s.nextId := Nat.ne_of_lt old
   simp [raw,moveRaw,push,erase,ne]; omega
 | split hp cur lo hi =>
   simp only [consumed,Option.some.injEq] at hc; subst h
   have old := (w.bounded _ _ cur.1).1
   have ne : _ ≠ s.nextId := Nat.ne_of_lt old
   have ne' : _ ≠ s.nextId+1 := Nat.ne_of_lt (Nat.lt_trans old (Nat.lt_succ_self _))
   simp [raw,splitRaw,push,erase,ne,ne']; omega

theorem consumed_never_current {policy : Nat → Bool} {s : State} (w : WF s) {a : Action}
 (allowed : Allowed policy s a) {h : Handle} (hc : consumed a = some h)
 (futurePolicy : Nat → Bool) (tail : List Action) (p : Nat) :
 ¬ Current (run futurePolicy (raw s a).1 tail) p h := by
 obtain ⟨absent,old⟩ := consumed_absent w allowed hc
 have after := run_old_absent futurePolicy (raw s a).1 tail h.id old absent
 intro cur; simp [Current,after] at cur

-- Each step receives its own policy input; policy changes are not resource mutations.
def runSchedule (s : State) : List ((Nat → Bool) × Action) → State
 | [] => s
 | (policy,a)::tail => runSchedule (advance policy s a) tail

theorem schedule_wf {s : State} (w : WF s) (steps : List ((Nat → Bool) × Action)) :
 WF (runSchedule s steps) := by
 induction steps generalizing s with
 | nil => exact w
 | cons pair tail ih => exact ih (advance_wf w pair.2)

theorem schedule_old_absent (s : State) (steps : List ((Nat → Bool) × Action)) (i : Nat)
 (old : i < s.nextId) (absent : s.live i = none) : (runSchedule s steps).live i = none := by
 induction steps generalizing s with
 | nil => exact absent
 | cons pair tail ih =>
   apply ih
   · exact Nat.lt_of_lt_of_le old (advance_monotone pair.1 s pair.2).1
   · exact advance_old_absent pair.1 s pair.2 i old absent

theorem consumed_never_current_schedule {policy : Nat → Bool} {s : State} (w : WF s) {a : Action}
 (allowed : Allowed policy s a) {h : Handle} (hc : consumed a = some h)
 (steps : List ((Nat → Bool) × Action)) (p : Nat) :
 ¬ Current (runSchedule (raw s a).1 steps) p h := by
 obtain ⟨absent,old⟩ := consumed_absent w allowed hc
 have after := schedule_old_absent (raw s a).1 steps h.id old absent
 intro cur; simp [Current,after] at cur

-- Extensional geometry and returned capabilities are separate from WF preservation.
def Contains (r : Region) (block offset : Nat) : Prop :=
 r.block = block ∧ r.lo ≤ offset ∧ offset < r.hi

theorem split_exact_cover (h : Handle) (k block offset : Nat)
 (interior : h.region.lo < k ∧ k < h.region.hi) :
 Contains h.region block offset ↔
 Contains (leftPart h k) block offset ∨ Contains (rightPart h k) block offset := by
 unfold Contains leftPart rightPart
 simp only
 omega

theorem split_disjoint_points (h : Handle) (k block offset : Nat) :
 ¬ (Contains (leftPart h k) block offset ∧ Contains (rightPart h k) block offset) := by
 unfold Contains leftPart rightPart
 simp only
 omega

theorem move_exact_cover (h : Handle) (q block offset : Nat) :
 Contains (moved h q) block offset ↔ Contains h.region block offset := Iff.rfl

theorem raw_frame (s : State) (a : Action) (i : Nat) (old : i < s.nextId)
 (untouched : ∀ h, consumed a = some h → i ≠ h.id) :
 (raw s a).1.live i = s.live i := by
 have ne : i ≠ s.nextId := by omega
 have ne' : i ≠ s.nextId+1 := by omega
 cases a with
 | allocate p n => simp [raw,allocateRaw,push,ne]
 | release p h =>
   have hn := untouched h rfl
   simp [raw,erase,hn]
 | move p h q =>
   have hn := untouched h rfl
   simp [raw,moveRaw,push,erase,ne,hn]
 | split p h k =>
   have hn := untouched h rfl
   simp [raw,splitRaw,push,erase,ne,ne',hn]

theorem returned_current (s : State) (a : Action) :
 ∀ h ∈ (raw s a).2, Current (raw s a).1 h.region.holder h := by
 cases a <;> simp [raw,allocateRaw,moveRaw,splitRaw,newHandle,Current,push,erase]

theorem execute_frame {policy : Nat → Bool} {s : State} {a : Action}
 {result : State × List Handle} (ok : execute policy s a = some result)
 (i : Nat) (old : i < s.nextId)
 (untouched : ∀ h, consumed a = some h → i ≠ h.id) :
 result.1.live i = s.live i := by
 unfold execute at ok
 split at ok
 · cases ok; exact raw_frame s a i old untouched
 · contradiction

theorem execute_returned_current {policy : Nat → Bool} {s : State} {a : Action}
 {result : State × List Handle} (ok : execute policy s a = some result) :
 ∀ h ∈ result.2, Current result.1 h.region.holder h := by
 unfold execute at ok
 split at ok
 · cases ok; exact returned_current s a
 · contradiction

#print axioms split_exact_cover
#print axioms split_disjoint_points
#print axioms move_exact_cover
#print axioms raw_frame
#print axioms execute_frame
#print axioms execute_returned_current

namespace Controls
def aliceOnly (p : Nat) : Bool := p == 1
def a : State × Handle := allocateRaw empty 1 12
def halves : State × Handle × Handle := splitRaw a.1 a.2 5
def transferred : State × Handle := moveRaw halves.1 halves.2.2 2
example : check aliceOnly empty (.allocate 1 12) = true := by decide
example : check aliceOnly empty (.allocate 1 0) = false := by decide
example : check aliceOnly empty (.allocate 2 12) = false := by decide
example : check aliceOnly a.1 (.split 1 a.2 5) = true := by decide
example : check aliceOnly a.1 (.split 1 a.2 12) = false := by decide
example : check aliceOnly halves.1 (.release 1 a.2) = false := by decide
example : check aliceOnly halves.1 (.move 1 halves.2.2 2) = true := by decide
-- A real new holder has ownership, but movement does not grant policy authority.
example : currentCheck transferred.1 2 transferred.2 = true := by decide
example : check aliceOnly transferred.1 (.release 2 transferred.2) = false := by decide
example : check (fun _ => true) transferred.1 (.release 2 transferred.2) = true := by decide
example : check (fun _ => true) transferred.1 (.release 1 halves.2.2) = false := by decide
-- Direct raw entry is intentionally unsafe: a non-interior split creates an empty region.
example : ¬ WF (splitRaw a.1 a.2 12).1 := by
 intro w
 have h := (w.bounded 2 (rightPart a.2 12) (by decide)).2.2
 change 12 < 12 at h
 omega
-- An old image can resurrect the handle; the theorem excludes resetting the current head.
example : currentCheck a.1 1 a.2 = true ∧ currentCheck halves.1 1 a.2 = false := by decide
end Controls

#print axioms allocate_wf
#print axioms erase_wf
#print axioms move_wf
#print axioms split_wf
#print axioms check_exact
#print axioms execution_exists
#print axioms execute_wf
#print axioms run_wf
#print axioms raw_monotone
#print axioms consumed_never_current
#print axioms schedule_wf
#print axioms consumed_never_current_schedule
end MirroreaProofFirst.ResourceBoundary

-- Later unreviewed finite-capacity reference extension; no production acceptance.
namespace MirroreaProofFirst.ResourceBoundary.BoundedIdentifiers
structure Limits where
 handles : Nat
 blocks : Nat
 deriving DecidableEq, Repr

def Within (l : Limits) (s : State) : Prop :=
 s.nextId ≤ l.handles ∧ s.nextBlock ≤ l.blocks

def Capacity (l : Limits) (s : State) : Action → Prop
 | .allocate _ _ => s.nextId+1 ≤ l.handles ∧ s.nextBlock+1 ≤ l.blocks
 | .release _ _ => s.nextId ≤ l.handles ∧ s.nextBlock ≤ l.blocks
 | .move _ _ _ => s.nextId+1 ≤ l.handles ∧ s.nextBlock ≤ l.blocks
 | .split _ _ _ => s.nextId+2 ≤ l.handles ∧ s.nextBlock ≤ l.blocks

def capacityCheck (l : Limits) (s : State) : Action → Bool
 | .allocate _ _ => decide (s.nextId+1 ≤ l.handles ∧ s.nextBlock+1 ≤ l.blocks)
 | .release _ _ => decide (s.nextId ≤ l.handles ∧ s.nextBlock ≤ l.blocks)
 | .move _ _ _ => decide (s.nextId+1 ≤ l.handles ∧ s.nextBlock ≤ l.blocks)
 | .split _ _ _ => decide (s.nextId+2 ≤ l.handles ∧ s.nextBlock ≤ l.blocks)

theorem capacity_exact (l : Limits) (s : State) (a : Action) :
 capacityCheck l s a = true ↔ Capacity l s a := by
 cases a <;> simp [capacityCheck,Capacity]

theorem raw_within (l : Limits) (s : State) (a : Action) :
 Within l (raw s a).1 ↔ Capacity l s a := by
 cases a <;> simp [Within,Capacity,raw,allocateRaw,moveRaw,splitRaw,erase,push,Nat.add_assoc]

-- Arithmetic is still mathematical Nat. A host implementation must check before
-- overflowing; computing a wrapped next counter and then comparing is unsound.
def runOne (l : Limits) (policy : Nat → Bool) (s : State) (a : Action) :
 Option (State × List Handle) :=
 if capacityCheck l s a then execute policy s a else none

theorem runOne_exact (l : Limits) (policy : Nat → Bool) (s : State) (a : Action)
 (out : State × List Handle) :
 runOne l policy s a = some out ↔
 Capacity l s a ∧ Allowed policy s a ∧ out = raw s a := by
 simp [runOne,execute,←capacity_exact,←check_exact]
 intro _ _
 exact eq_comm

theorem exists_iff (l : Limits) (policy : Nat → Bool) (s : State) (a : Action) :
 (∃ out, runOne l policy s a = some out) ↔ Capacity l s a ∧ Allowed policy s a := by
 simp [runOne_exact]

theorem preserves {l : Limits} {policy : Nat → Bool} {s : State}
 (w : WF s) {a : Action} {out : State × List Handle}
 (ok : runOne l policy s a = some out) : WF out.1 ∧ Within l out.1 := by
 obtain ⟨cap,allow,rfl⟩ := (runOne_exact _ _ _ _ _).mp ok
 exact ⟨raw_wf w allow,(raw_within _ _ _).mpr cap⟩

def advance (l : Limits) (policy : Nat → Bool) (s : State) (a : Action) : State :=
 match runOne l policy s a with
 | none => s
 | some out => out.1

def schedule (l : Limits) (s : State) : List ((Nat → Bool) × Action) → State
 | [] => s
 | (p,a)::tail => schedule l (advance l p s a) tail

theorem advance_preserves {l : Limits} {policy : Nat → Bool} {s : State}
 (w : WF s) (bounded : Within l s) (a : Action) :
 WF (advance l policy s a) ∧ Within l (advance l policy s a) := by
 unfold advance
 split
 · exact ⟨w,bounded⟩
 · rename_i out h
   exact preserves w h

theorem schedule_preserves {l : Limits} {s : State} (w : WF s) (bounded : Within l s)
 (steps : List ((Nat → Bool) × Action)) :
 WF (schedule l s steps) ∧ Within l (schedule l s steps) := by
 induction steps generalizing s with
 | nil => exact ⟨w,bounded⟩
 | cons pair tail ih =>
   have h := advance_preserves w bounded pair.2 (policy := pair.1)
   exact ih h.1 h.2

theorem advance_monotone (l : Limits) (policy : Nat → Bool) (s : State) (a : Action) :
 s.nextId ≤ (advance l policy s a).nextId := by
 unfold advance
 split
 · exact Nat.le_refl _
 · rename_i out h
   obtain ⟨_,_,rfl⟩ := (runOne_exact _ _ _ _ _).mp h
   exact (raw_monotone s a).1

theorem advance_old_absent (l : Limits) (policy : Nat → Bool) (s : State) (a : Action)
 (i : Nat) (old : i < s.nextId) (absent : s.live i = none) :
 (advance l policy s a).live i = none := by
 unfold advance
 split
 · exact absent
 · rename_i out h
   obtain ⟨_,_,rfl⟩ := (runOne_exact _ _ _ _ _).mp h
   exact raw_old_absent s a i old absent

theorem schedule_old_absent (l : Limits) (s : State)
 (steps : List ((Nat → Bool) × Action)) (i : Nat)
 (old : i < s.nextId) (absent : s.live i = none) :
 (schedule l s steps).live i = none := by
 induction steps generalizing s with
 | nil => exact absent
 | cons pair tail ih =>
   apply ih
   · exact Nat.lt_of_lt_of_le old (advance_monotone l pair.1 s pair.2)
   · exact advance_old_absent l pair.1 s pair.2 i old absent

theorem consumed_never_current {l : Limits} {policy : Nat → Bool} {s : State}
 (w : WF s) {a : Action} {out : State × List Handle}
 (ok : runOne l policy s a = some out) {h : Handle} (hc : consumed a = some h)
 (steps : List ((Nat → Bool) × Action)) (p : Nat) :
 ¬ Current (schedule l out.1 steps) p h := by
 obtain ⟨_,allow,rfl⟩ := (runOne_exact _ _ _ _ _).mp ok
 obtain ⟨absent,old⟩ := consumed_absent w allow hc
 have after := schedule_old_absent l (raw s a).1 steps h.id old absent
 intro cur
 simp [Current,after] at cur

-- A host guard can avoid overflowing addition: validate the current bound before
-- subtracting, then compare the required fresh capacity against the remainder.
def handleNeed : Action → Nat
 | .allocate _ _ | .move _ _ _ => 1
 | .release _ _ => 0
 | .split _ _ _ => 2
def blockNeed : Action → Nat
 | .allocate _ _ => 1
 | _ => 0
def remainingCheck (l : Limits) (s : State) (a : Action) : Bool :=
 if s.nextId ≤ l.handles ∧ s.nextBlock ≤ l.blocks then
   decide (handleNeed a ≤ l.handles-s.nextId ∧ blockNeed a ≤ l.blocks-s.nextBlock)
 else false

theorem remaining_exact (l : Limits) (s : State) (a : Action) :
 remainingCheck l s a = true ↔ Capacity l s a := by
 cases a <;> simp [remainingCheck,handleNeed,blockNeed,Capacity] <;> omega

theorem remaining_same (l : Limits) (s : State) (a : Action) :
 remainingCheck l s a = capacityCheck l s a := by
 have h := (remaining_exact l s a).trans (capacity_exact l s a).symm
 cases hr : remainingCheck l s a <;> cases hc : capacityCheck l s a <;> simp_all

theorem successful_native_bounds {l : Limits} {policy : Nat → Bool} {s : State}
 {a : Action} {out : State × List Handle} (ok : runOne l policy s a = some out)
 (maximum : Nat) (handles : l.handles ≤ maximum) (blocks : l.blocks ≤ maximum) :
 out.1.nextId ≤ maximum ∧ out.1.nextBlock ≤ maximum := by
 obtain ⟨cap,_,rfl⟩ := (runOne_exact _ _ _ _ _).mp ok
 have h := (raw_within l s a).mpr cap
 exact ⟨Nat.le_trans h.1 handles,Nat.le_trans h.2 blocks⟩

namespace Controls
open ResourceBoundary.Controls
def limits : Limits := ⟨4,1⟩
example : capacityCheck ⟨2,1⟩ a.1 (.split 1 a.2 5) = false := by decide
example : capacityCheck ⟨4,0⟩ empty (.allocate 1 12) = false := by decide
example : capacityCheck limits empty (.allocate 1 12) = true := by decide
example : capacityCheck limits a.1 (.split 1 a.2 5) = true := by decide
example : capacityCheck limits halves.1 (.move 1 halves.2.2 2) = true := by decide
example : capacityCheck limits transferred.1 (.split 2 transferred.2 8) = false := by decide
example : capacityCheck limits transferred.1 (.release 2 transferred.2) = true := by decide
-- Resource release does not reset the historical fresh-identifier counters.
example : capacityCheck limits (erase transferred.1 transferred.2.id) (.allocate 2 1) = false := by decide
example : (runOne limits aliceOnly empty (.allocate 1 12)).map
 (fun out => out.2.map (fun h => (h.id,h.region.block,h.region.lo,h.region.hi,h.region.holder))) =
 some [(0,0,0,12,1)] := by decide
example : (runOne limits aliceOnly halves.1 (.move 1 halves.2.2 2)).map
 (fun out => out.2.map (fun h => (h.id,h.region.lo,h.region.hi,h.region.holder))) =
 some [(3,5,12,2)] := by decide
example : runOne limits (fun _ => true) transferred.1 (.split 2 transferred.2 8) = none := by decide
example : (runOne limits (fun _ => true) transferred.1 (.release 2 transferred.2)).map
 (fun out => out.1.live transferred.2.id) = some none := by decide
-- Resetting finite counters after release is an extra transition, not admitted
-- by schedule. It can resurrect an old handle even when spatial WF holds again.
def wrapped : State := {erase a.1 a.2.id with
 nextId := a.1.nextId % 1, nextBlock := a.1.nextBlock % 1}
theorem wrapped_eq_empty : wrapped = empty := by
 simp [wrapped,erase,a,allocateRaw,push,empty,newHandle]
 funext i
 by_cases hi : i = 0 <;> simp [hi]

example : WF (allocateRaw wrapped 1 12).1 := by
 rw [wrapped_eq_empty]
 exact allocate_wf empty_wf 1 12 (by decide)
example : currentCheck (allocateRaw wrapped 1 12).1 1 a.2 = true := by decide
end Controls
#print axioms remaining_exact
#print axioms remaining_same
#print axioms successful_native_bounds
#print axioms capacity_exact
#print axioms raw_within
#print axioms runOne_exact
#print axioms exists_iff
#print axioms preserves
#print axioms schedule_preserves
#print axioms schedule_old_absent
#print axioms consumed_never_current
end MirroreaProofFirst.ResourceBoundary.BoundedIdentifiers

-- Unreviewed finite resource image. Authenticated freshness, whole-state histories,
-- byte decoding/allocation budgets and physical crash persistence remain open.
namespace MirroreaProofFirst.ResourceBoundary.PrivateImage
-- LAB finite mathematical image, not a wire format or trusted recovery head.
structure Image where
 nextId : Nat
 nextBlock : Nat
 entries : List (Option Region)
 deriving DecidableEq

def encode (s : State) : Image :=
 ⟨s.nextId,s.nextBlock,List.ofFn (fun i : Fin s.nextId => s.live i.val)⟩
def decode (image : Image) : State :=
 ⟨image.nextId,image.nextBlock,fun i => (image.entries[i]?).join⟩

theorem outside_none (s : State) (wf : WF s) (i : Nat) (outside : s.nextId ≤ i) :
 s.live i = none := by
 cases e : s.live i with
 | none => rfl
 | some r => have h := (wf.bounded i r e).1; omega

theorem roundtrip (s : State) (wf : WF s) : decode (encode s) = s := by
 cases s with
 | mk ni nb live =>
  apply congrArg (State.mk ni nb)
  funext i
  by_cases h : i < ni
  · simp [encode,h]
  · have gone : live i = none := outside_none ⟨ni,nb,live⟩ wf i (by change ni ≤ i; omega)
    simp [encode,h,gone]
def EntryOK (image : Image) (i : Fin image.entries.length) : Prop :=
 match image.entries[i.val] with
 | none => True
 | some r => r.block < image.nextBlock ∧ Nonempty r

def PairOK (image : Image) (i j : Fin image.entries.length) : Prop :=
 match image.entries[i.val], image.entries[j.val] with
 | some a, some b => i = j ∨ Sep a b
 | _,_ => True

instance (image : Image) (i : Fin image.entries.length) : Decidable (EntryOK image i) := by
 unfold EntryOK Nonempty; split <;> infer_instance
instance (image : Image) (i j : Fin image.entries.length) : Decidable (PairOK image i j) := by
 unfold PairOK Sep; split <;> infer_instance

-- Independent finite structural judgment; no claim that the image is current.
def Valid (image : Image) : Prop :=
 image.entries.length = image.nextId ∧
 (∀ i, EntryOK image i) ∧ (∀ i j, PairOK image i j)

def check (image : Image) : Bool :=
 decide (image.entries.length = image.nextId) &&
 (List.finRange image.entries.length).all (fun i => decide (EntryOK image i)) &&
 (List.finRange image.entries.length).all (fun i =>
  (List.finRange image.entries.length).all (fun j => decide (PairOK image i j)))

theorem check_exact (image : Image) : check image = true ↔ Valid image := by
 simp [check,Valid,List.all_eq_true,and_assoc]

theorem decoded_wf (image : Image) (valid : Valid image) : WF (decode image) := by
 obtain ⟨length,entries,pairs⟩ := valid
 constructor
 · intro i r h
   by_cases inside : i < image.entries.length
   · have entry : image.entries[i] = some r := by simpa [decode,inside] using h
     have bound := entries ⟨i,inside⟩
     simp [EntryOK,entry] at bound
     exact ⟨by simpa [decode,←length] using inside,bound⟩
   · simp [decode,inside] at h
 · intro i j a b distinct ha hb
   have insideI : i < image.entries.length := by
    by_cases inside : i < image.entries.length
    · exact inside
    · simp [decode,inside] at ha
   have insideJ : j < image.entries.length := by
    by_cases inside : j < image.entries.length
    · exact inside
    · simp [decode,inside] at hb
   have atI : image.entries[i] = some a := by simpa [decode,insideI] using ha
   have atJ : image.entries[j] = some b := by simpa [decode,insideJ] using hb
   have pair := pairs ⟨i,insideI⟩ ⟨j,insideJ⟩
   simpa [PairOK,atI,atJ,Fin.ext_iff,distinct] using pair

theorem wf_implies_valid (image : Image) (length : image.entries.length = image.nextId)
 (wf : WF (decode image)) : Valid image := by
 refine ⟨length,?_,?_⟩
 · intro i
   cases entry : image.entries[i.val] with
   | none => simp [EntryOK,entry]
   | some r =>
     have live : (decode image).live i.val = some r := by simp [decode,i.isLt,entry]
     simpa [EntryOK,entry,decode] using (wf.bounded i.val r live).2
 · intro i j
   cases left : image.entries[i.val] with
   | none => simp [PairOK,left]
   | some a =>
    cases right : image.entries[j.val] with
    | none => simp [PairOK,right]
    | some b =>
     by_cases same : i = j
     · simp [PairOK,right,same]
     · have different : i.val ≠ j.val := fun eq => same (Fin.ext eq)
       have liveI : (decode image).live i.val = some a := by simp [decode,i.isLt,left]
       have liveJ : (decode image).live j.val = some b := by simp [decode,j.isLt,right]
       have separated := wf.separated i.val j.val a b different liveI liveJ
       simp [PairOK,left,right,separated]

theorem check_wf_exact (image : Image) : check image = true ↔
 image.entries.length = image.nextId ∧ WF (decode image) := by
 rw [check_exact]
 exact ⟨fun v => ⟨v.1,decoded_wf image v⟩,fun ⟨len,wf⟩ => wf_implies_valid image len wf⟩

def restore (image : Image) : Option State := if check image then some (decode image) else none

theorem restores_encoded (s : State) (wf : WF s) : restore (encode s) = some s := by
 have good : check (encode s) = true := (check_wf_exact _).mpr
  ⟨by simp [encode],by simpa [roundtrip s wf] using wf⟩
 simp [restore,good,roundtrip s wf]

theorem restore_sound (image : Image) (s : State) (accepted : restore image = some s) : WF s := by
 unfold restore at accepted
 split at accepted
 · rename_i good
   cases accepted
   exact ((check_wf_exact _).mp good).2
 · contradiction

-- Limits are independent caller policy, not authority inferred from an image.
def fits (limits : BoundedIdentifiers.Limits) (image : Image) : Bool :=
 decide (image.nextId ≤ limits.handles) && decide (image.nextBlock ≤ limits.blocks)

def restoreWithin (limits : BoundedIdentifiers.Limits) (image : Image) : Option State :=
 if fits limits image then restore image else none

theorem fits_exact (limits : BoundedIdentifiers.Limits) (image : Image) :
 fits limits image = true ↔ BoundedIdentifiers.Within limits (decode image) := by
 simp [fits,BoundedIdentifiers.Within,decode]

theorem restore_within_sound (limits : BoundedIdentifiers.Limits) (image : Image) (s : State)
 (accepted : restoreWithin limits image = some s) :
 WF s ∧ BoundedIdentifiers.Within limits s := by
 unfold restoreWithin at accepted
 split at accepted
 · rename_i budget
   have wf := restore_sound image s accepted
   unfold restore at accepted
   split at accepted
   · cases accepted; exact ⟨wf,(fits_exact _ _).mp budget⟩
   · contradiction
 · contradiction

theorem restores_encoded_within (limits : BoundedIdentifiers.Limits) (s : State)
 (wf : WF s) (within : BoundedIdentifiers.Within limits s) :
 restoreWithin limits (encode s) = some s := by
 have fit : fits limits (encode s) = true := (fits_exact _ _).mpr (by rw [roundtrip s wf]; exact within)
 simp [restoreWithin,fit,restores_encoded s wf]

namespace Controls
open ResourceBoundary.Controls
example : check (encode a.1) = true := by decide
example : check (encode transferred.1) = true := by decide
example : (restoreWithin ⟨0,0⟩ (encode a.1)).isNone = true := by decide
example : (restoreWithin ⟨1,1⟩ (encode a.1)).isSome = true := by decide
-- Reject truncation, out-of-head blocks, empty geometry and overlapping ownership.
example : check ⟨1,1,[]⟩ = false := by decide
example : check ⟨1,0,[some ⟨0,0,12,1⟩]⟩ = false := by decide
example : check ⟨1,1,[some ⟨0,5,5,1⟩]⟩ = false := by decide
example : check ⟨2,1,[some ⟨0,0,12,1⟩,some ⟨0,5,9,2⟩]⟩ = false := by decide
example : (restore ⟨2,1,[some ⟨0,0,12,1⟩,some ⟨0,5,9,2⟩]⟩).isNone = true := by decide
-- Released holes still retain the issuance head; do not renumber live entries.
def released := erase a.1 a.2.id
example : (encode released).entries = [none] ∧ (encode released).nextId = 1 := by decide
example : currentCheck (decode (encode released)) 1 a.2 = false := by decide
example : (decode (encode released)).nextId = 1 := by decide
-- A locally well-formed old image is still accepted: freshness is a separate gate.
example : check (encode a.1) = true ∧ currentCheck (decode (encode a.1)) 1 a.2 = true := by decide
-- Resetting all history is structurally valid and can reuse a prior identifier.
example : check (encode empty) = true := by decide
example : (allocateRaw (decode (encode empty)) 1 12).2 = a.2 := by decide
end Controls
#print axioms roundtrip
#print axioms check_exact
#print axioms decoded_wf
#print axioms wf_implies_valid
#print axioms check_wf_exact
#print axioms restores_encoded
#print axioms restore_sound
#print axioms fits_exact
#print axioms restore_within_sound
#print axioms restores_encoded_within
end MirroreaProofFirst.ResourceBoundary.PrivateImage
