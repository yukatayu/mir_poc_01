import Std
namespace MirroreaProofFirst.TrackedValidation
-- Pure read-dependent validation, not ordinary source read snapshot semantics.
inductive Query (K V A : Type) where
  | done (result : A)
  | read (key : K) (next : Option V → Query K V A)

def evaluate (s : K → Option V) : Query K V A → A × List K
  | .done a => (a, [])
  | .read k next => let rest := evaluate s (next (s k)); (rest.1, k :: rest.2)

-- The declarative relation does not mention evaluate or a validator result.
inductive Evaluates (s : K → Option V) : Query K V A → A → List K → Prop where
  | done : Evaluates s (.done a) a []
  | read : Evaluates s (next (s k)) a ks → Evaluates s (.read k next) a (k :: ks)

theorem evaluate_exact (s : K → Option V) (q : Query K V A) (a : A) (ks : List K) :
    evaluate s q = (a, ks) ↔ Evaluates s q a ks := by
  constructor
  · intro h
    induction q generalizing a ks with
    | done v => cases h; exact .done
    | read k next ih =>
      cases e : evaluate s (next (s k)) with
      | mk v vs =>
        simp only [evaluate, e, Prod.mk.injEq] at h
        obtain ⟨rfl, rfl⟩ := h
        exact .read (ih _ _ _ e)
  · intro h
    induction h with
    | done => rfl
    | read _ ih => simp [evaluate, ih]

theorem tracked_reads_suffice (s t : K → Option V) (q : Query K V A)
    (agree : ∀ k ∈ (evaluate s q).2, s k = t k) :
    evaluate s q = evaluate t q := by
  induction q with
  | done a => rfl
  | read k next ih =>
    have hk : s k = t k := agree k (by simp [evaluate])
    have rest : evaluate s (next (s k)) = evaluate t (next (s k)) :=
      ih (s k) (fun j hj => agree j (by simp [evaluate, hj]))
    simp only [evaluate, ← hk, rest]

structure Cell (V : Type) where
  version : Nat
  value : Option V

def write [DecidableEq K] (s : K → Cell V) (key : K) (v : Option V) : K → Cell V :=
  fun k => if k = key then ⟨(s k).version + 1, v⟩ else s k

def writes [DecidableEq K] (s : K → Cell V) : List (K × Option V) → K → Cell V
  | [] => s
  | (k,v) :: rest => writes (write s k v) rest

theorem write_version_mono [DecidableEq K] (s : K → Cell V) (key k : K) (v : Option V) :
    (s k).version ≤ (write s key v k).version := by
  by_cases h : k = key <;> simp [write, h]

theorem writes_version_mono [DecidableEq K] (s : K → Cell V) (ws : List (K × Option V)) (k : K) :
    (s k).version ≤ (writes s ws k).version := by
  induction ws generalizing s with
  | nil => exact Nat.le_refl _
  | cons kv rest ih =>
    exact Nat.le_trans (write_version_mono s kv.1 k kv.2) (ih (write s kv.1 kv.2))

theorem equal_version_same_cell [DecidableEq K] (s : K → Cell V)
    (ws : List (K × Option V)) (k : K)
    (same : (s k).version = (writes s ws k).version) :
    s k = writes s ws k := by
  induction ws generalizing s with
  | nil => rfl
  | cons kv rest ih =>
    have mono := writes_version_mono (write s kv.1 kv.2) rest k
    have unchanged : k ≠ kv.1 := by
      intro eq
      simp only [writes] at same
      subst k
      simp [write] at mono
      omega
    have heq : write s kv.1 kv.2 k = s k := by simp [write, unchanged]
    have tail := ih (write s kv.1 kv.2) (by simpa [writes, heq] using same)
    simpa [writes, heq] using tail

theorem validated_query_preserved [DecidableEq K] (s : K → Cell V)
    (ws : List (K × Option V)) (q : Query K V A)
    (stamps : ∀ k ∈ (evaluate (fun j => (s j).value) q).2,
      (s k).version = (writes s ws k).version) :
    evaluate (fun k => (s k).value) q = evaluate (fun k => (writes s ws k).value) q := by
  apply tracked_reads_suffice
  intro k hk
  exact congrArg Cell.value (equal_version_same_cell s ws k (stamps k hk))

-- Version checks are executable and independent of the semantic replay theorem.
def checkStamps (s t : K → Cell V) (keys : List K) : Bool :=
  keys.all fun k => decide ((s k).version = (t k).version)

theorem checkStamps_exact (s t : K → Cell V) (keys : List K) :
    checkStamps s t keys = true ↔ ∀ k ∈ keys, (s k).version = (t k).version := by
  simp [checkStamps]

-- First-key-wins finite overlay; no simultaneous mutation is asserted here.
def overlay [DecidableEq K] (s : K → Option V) (patch : List (K × Option V)) (k : K) : Option V :=
  match patch with
  | [] => s k
  | (j,v) :: rest => if k = j then v else overlay s rest k

theorem overlay_agrees [DecidableEq K] (s t : K → Option V)
    (patch : List (K × Option V)) (k : K) (same : s k = t k) :
    overlay s patch k = overlay t patch k := by
  induction patch with
  | nil => exact same
  | cons kv rest ih =>
    simp only [overlay]
    split
    · rfl
    · exact ih

def requiredKeys [DecidableEq K] (s : K → Cell V) (patch : List (K × Option V))
    (q : Query K V A) : List K :=
  (evaluate (overlay (fun k => (s k).value) patch) q).2 ++ patch.map Prod.fst

theorem checked_overlay_replay [DecidableEq K] (s : K → Cell V)
    (ws patch : List (K × Option V)) (q : Query K V A)
    (checked : checkStamps s (writes s ws) (requiredKeys s patch q) = true) :
    evaluate (overlay (fun k => (s k).value) patch) q =
      evaluate (overlay (fun k => (writes s ws k).value) patch) q := by
  apply tracked_reads_suffice
  intro k hk
  apply overlay_agrees
  apply congrArg Cell.value
  apply equal_version_same_cell
  exact (checkStamps_exact _ _ _).mp checked k (by simp [requiredKeys, hk])

theorem checked_blind_write_unchanged [DecidableEq K] (s : K → Cell V)
    (ws patch : List (K × Option V)) (q : Query K V A)
    (checked : checkStamps s (writes s ws) (requiredKeys s patch q) = true)
    (key : K) (present : key ∈ patch.map Prod.fst) : s key = writes s ws key := by
  apply equal_version_same_cell
  exact (checkStamps_exact _ _ _).mp checked key (by simp [requiredKeys, present])

namespace Controls
-- Absence is a read value. The selected second key depends on the first value.
def initial : Nat → Cell Nat := fun k => if k = 0 then ⟨1, some 7⟩ else ⟨0, none⟩
def query : Query Nat Nat (Option Nat) := .read 0 (fun x => .read (x.getD 2) Query.done)
#guard evaluate (fun k => (initial k).value) query = (none, [0,7])
#guard checkStamps initial (writes initial [(9, some 4)]) [0,7]
#guard !checkStamps initial (writes initial [(7, some 4)]) [0,7]
-- Delete then recreate the same value: value-only validation misses the change.
def aba := writes initial [(0, none), (0, some 7)]
#guard (initial 0).value = (aba 0).value
#guard !checkStamps initial aba [0]
-- Omitting absent/index rows accepts a stale validation result.
#guard checkStamps initial (writes initial [(7, some 4)]) [0]
#guard (evaluate (fun k => (initial k).value) query).1 !=
  (evaluate (fun k => (writes initial [(7, some 4)] k).value) query).1
-- Blind write destination must be tracked even if no query reads it.
def blind : Query Nat Nat Bool := .done true
#guard !checkStamps initial (writes initial [(5, some 3)]) (requiredKeys initial [(5, some 8)] blind)
#guard checkStamps initial (writes initial [(5, some 3)]) []
-- A countermodel outside the write transition: restoring/resetting a stamp.
def resetVersion : Nat → Cell Nat := fun k => if k = 0 then ⟨1, some 99⟩ else initial k
#guard checkStamps initial resetVersion [0]
#guard (initial 0).value != (resetVersion 0).value
end Controls

#print axioms checkStamps_exact
#print axioms checked_overlay_replay
#print axioms checked_blind_write_unchanged

#print axioms evaluate_exact
#print axioms tracked_reads_suffice
#print axioms equal_version_same_cell
#print axioms validated_query_preserved


-- Each observation contains the value and version from one logical atomic read.
-- Observed is a trace semantics, not a claim that an external reader is trusted.
inductive Observed : Query K V A → A → List (K × Cell V) → Prop where
  | done : Observed (.done a) a []
  | read : Observed (next c.value) a rs → Observed (.read k next) a ((k,c) :: rs)

theorem observed_replay (q : Query K V A) (a : A) (rs : List (K × Cell V))
    (h : Observed q a rs) (s : K → Option V)
    (values : ∀ k c, (k,c) ∈ rs → c.value = s k) :
    Evaluates s q a (rs.map Prod.fst) := by
  induction h with
  | done => exact .done
  | @read next a rs k c h ih =>
    have head : c.value = s k := values k c (by simp)
    have tail := ih (fun j d hd => values j d (by simp [hd]))
    have tailCurrent : Evaluates s (next (s k)) a (rs.map Prod.fst) := by simpa [head] using tail
    exact Evaluates.read tailCurrent

-- For each observed row, the current store must be reachable by the declared
-- version-retaining write transitions from that row's actual observation state.
-- Different rows may be read at different times; no global preparation snapshot.
def RowOrigins [DecidableEq K] (current : K → Cell V) (rs : List (K × Cell V)) : Prop :=
  ∀ k c, (k,c) ∈ rs → ∃ before ws,
    before k = c ∧ writes before ws = current

def checkObservedStamps (current : K → Cell V) (rs : List (K × Cell V)) : Bool :=
  rs.all fun (k,c) => decide (c.version = (current k).version)

theorem observed_validation_sound [DecidableEq K]
    (q : Query K V A) (a : A) (rs : List (K × Cell V))
    (h : Observed q a rs) (current : K → Cell V)
    (origins : RowOrigins current rs)
    (checked : checkObservedStamps current rs = true) :
    evaluate (fun k => (current k).value) q = (a, rs.map Prod.fst) := by
  apply (evaluate_exact _ _ _ _).mpr
  apply observed_replay q a rs h
  intro k c present
  obtain ⟨before, ws, hb, hw⟩ := origins k c present
  have stamps : ∀ entry ∈ rs, entry.2.version = (current entry.1).version := by
    simpa [checkObservedStamps] using checked
  have cell := equal_version_same_cell before ws k (by simpa [hb, hw] using stamps (k,c) present)
  simpa [hb, hw] using congrArg Cell.value cell

-- Interleaving semantics: reads are atomic Cell reads, writes are the concrete
-- incrementing operation, and a finite completion has an actual origin trace.
inductive PreparationTrace {K V A : Type} [DecidableEq K] :
    (K → Cell V) → Query K V A → (K → Cell V) → A → List (K × Cell V) → Prop where
  | done : PreparationTrace s (.done a) s a []
  | read : PreparationTrace s (next (s k).value) t a rs →
      PreparationTrace s (.read k next) t a ((k,s k) :: rs)
  | mutate : PreparationTrace (write s k v) q t a rs →
      PreparationTrace s q t a rs

theorem preparation_reachable [DecidableEq K]
    {s t : K → Cell V} {q : Query K V A} {a : A} {rs : List (K × Cell V)}
    (h : PreparationTrace s q t a rs) : ∃ ws, writes s ws = t := by
  induction h with
  | done => exact ⟨[], rfl⟩
  | read _ ih => exact ih
  | @mutate s k v q t a rs h ih =>
    obtain ⟨ws, hw⟩ := ih
    exact ⟨(k,v) :: ws, hw⟩

theorem preparation_observed [DecidableEq K]
    {s t : K → Cell V} {q : Query K V A} {a : A} {rs : List (K × Cell V)}
    (h : PreparationTrace s q t a rs) : Observed q a rs := by
  induction h with
  | done => exact .done
  | read _ ih => exact .read ih
  | mutate _ ih => exact ih

theorem preparation_origins [DecidableEq K]
    {s t : K → Cell V} {q : Query K V A} {a : A} {rs : List (K × Cell V)}
    (h : PreparationTrace s q t a rs) : RowOrigins t rs := by
  induction h with
  | done => intro k c present; simp at present
  | @read s next k t a rs h ih =>
    intro j c present
    rcases List.mem_cons.mp present with head | tail
    · obtain ⟨rfl, rfl⟩ := Prod.mk.inj head
      obtain ⟨ws, hw⟩ := preparation_reachable h
      exact ⟨s, ws, rfl, hw⟩
    · exact ih j c tail
  | mutate _ ih => exact ih

theorem interleaved_validation_sound [DecidableEq K]
    {s t : K → Cell V} {q : Query K V A} {a : A} {rs : List (K × Cell V)}
    (h : PreparationTrace s q t a rs)
    (checked : checkObservedStamps t rs = true) :
    evaluate (fun k => (t k).value) q = (a, rs.map Prod.fst) :=
  observed_validation_sound q a rs (preparation_observed h) t
    (preparation_origins h) checked

namespace InterleavingControls
def start : Nat → Cell Nat := fun _ => ⟨0, some 0⟩
def finish := write start 1 (some 8)
def query : Query Nat Nat (Option Nat × Option Nat) :=
  .read 0 fun x => .read 1 fun y => .done (x,y)
def observations : List (Nat × Cell Nat) := [(0,start 0), (1,finish 1)]
example : PreparationTrace start query finish (some 0, some 8) observations := by
  apply PreparationTrace.read
  apply PreparationTrace.mutate (k := 1) (v := some 8)
  apply PreparationTrace.read
  exact PreparationTrace.done
#guard checkObservedStamps finish observations
#guard (evaluate (fun k => (start k).value) query).1 != (some 0, some 8)
#guard (evaluate (fun k => (finish k).value) query).1 = (some 0, some 8)
#guard !checkObservedStamps (write finish 0 (some 3)) observations
-- Torn read: old value paired with a newer stamp passes stamps alone. This
-- fabricated observation has no PreparationTrace origin for the alleged read.
def forged : List (Nat × Cell Nat) := [(1, ⟨1,some 0⟩)]
#guard checkObservedStamps finish forged
#guard ((forged[0]).2).value != (finish 1).value
-- The rejection uses the general replay theorem, not a search over traces.
theorem forged_has_no_origin (s : Nat → Cell Nat) :
    ¬ PreparationTrace s (.read 1 Query.done) finish (some 0) forged := by
  intro h
  have replay := interleaved_validation_sound h (by decide)
  have result := congrArg Prod.fst replay
  have unequal : (evaluate (fun k => (finish k).value) (.read 1 Query.done)).1 ≠ some 0 := by decide
  exact unequal result
end InterleavingControls

#print axioms preparation_reachable
#print axioms preparation_observed
#print axioms preparation_origins
#print axioms interleaved_validation_sound

#print axioms observed_replay
#print axioms observed_validation_sound

-- Logical atomic publication. The order implements the existing first-key-wins
-- overlay even for duplicate keys; it is not a physical synchronization proof.
def publish [DecidableEq K] (s : K → Cell V) : List (K × Option V) → K → Cell V
  | [] => s
  | (k,v) :: rest => write (publish s rest) k v

theorem publish_values [DecidableEq K] (s : K → Cell V)
    (patch : List (K × Option V)) (k : K) :
    (publish s patch k).value = overlay (fun j => (s j).value) patch k := by
  induction patch with
  | nil => rfl
  | cons kv rest ih =>
    simp only [publish, write, overlay]
    split
    · rfl
    · exact ih

theorem publish_is_writes [DecidableEq K] (s : K → Cell V)
    (patch : List (K × Option V)) : ∃ ws, writes s ws = publish s patch := by
  have append (s : K → Cell V) (xs ys : List (K × Option V)) :
      writes s (xs ++ ys) = writes (writes s xs) ys := by
    induction xs generalizing s with
    | nil => rfl
    | cons kv rest ih => exact ih (write s kv.1 kv.2)
  induction patch with
  | nil => exact ⟨[], rfl⟩
  | cons kv rest ih =>
    obtain ⟨ws, hw⟩ := ih
    exact ⟨ws ++ [kv], by simp [append, writes, hw, publish]⟩

def commit [DecidableEq K] (prepared current : K → Cell V)
    (patch : List (K × Option V)) (q : Query K V Bool) : Option (K → Cell V) :=
  if (evaluate (overlay (fun k => (prepared k).value) patch) q).1 &&
      checkStamps prepared current (requiredKeys prepared patch q)
  then some (publish current patch) else none

theorem commit_rechecks_result [DecidableEq K] (s : K → Cell V)
    (ws patch : List (K × Option V)) (q : Query K V Bool) (out : K → Cell V)
    (accepted : commit s (writes s ws) patch q = some out) :
    (evaluate (fun k => (out k).value) q).1 = true := by
  simp only [commit] at accepted
  split at accepted
  next condition =>
    have hc : (evaluate (overlay (fun k => (s k).value) patch) q).1 = true ∧
        checkStamps s (writes s ws) (requiredKeys s patch q) = true := by simpa using condition
    have replay := checked_overlay_replay s ws patch q hc.2
    have eq : publish (writes s ws) patch = out := Option.some.inj accepted
    subst out
    have values : (fun k => (publish (writes s ws) patch k).value) =
        overlay (fun k => (writes s ws k).value) patch := funext (publish_values _ _)
    rw [values, ← replay]
    exact hc.1
  next => cases accepted

namespace CommitControls
def start : Nat → Cell Nat := fun _ => ⟨0, none⟩
def present : Query Nat Nat Bool := .read 2 (fun v => .done v.isSome)
#guard (commit start start [(2,some 8)] present).isSome
#guard (commit start (write start 9 (some 4)) [(2,some 8)] present).isSome
#guard (commit start (write start 2 (some 4)) [(2,some 8)] present).isNone
#guard (commit start start [] present).isNone
#guard (publish start [(2,some 8),(2,some 9)] 2).value = some 8
-- An unprotected write after validation can invalidate the checked poststate.
#guard (evaluate (fun k => (write (publish start [(2,some 8)]) 2 none k).value) present).1 = false
end CommitControls
#print axioms publish_values
#print axioms publish_is_writes
#print axioms commit_rechecks_result

def writeCount [DecidableEq K] (k : K) : List (K × Option V) → Nat
  | [] => 0
  | (j,_) :: rest => (if k = j then 1 else 0) + writeCount k rest

theorem writes_version_exact [DecidableEq K] (s : K → Cell V)
    (ws : List (K × Option V)) (k : K) :
    (writes s ws k).version = (s k).version + writeCount k ws := by
  induction ws generalizing s with
  | nil => rfl
  | cons kv rest ih =>
    simp only [writes, ih, writeCount]
    by_cases h : k = kv.1 <;> simp [write, h, Nat.add_assoc]

theorem checkStamps_no_writes [DecidableEq K] (s : K → Cell V)
    (ws : List (K × Option V)) (ks : List K) :
    checkStamps s (writes s ws) ks = true ↔ ∀ k ∈ ks, writeCount k ws = 0 := by
  rw [checkStamps_exact]
  constructor
  · intro h k hk
    have eq := h k hk
    rw [writes_version_exact] at eq
    omega
  · intro h k hk
    rw [writes_version_exact, h k hk, Nat.add_zero]
#print axioms writes_version_exact
#print axioms checkStamps_no_writes

-- Values read from one immutable proposed patch; stamps/origins from actual rows.
def patchedValue [DecidableEq K] (patch : List (K × Option V)) (k : K) (c : Cell V) : Option V :=
  overlay (fun _ => c.value) patch k

theorem patchedValue_exact [DecidableEq K] (patch : List (K × Option V))
    (s : K → Cell V) (k : K) :
    patchedValue patch k (s k) = overlay (fun j => (s j).value) patch k :=
  overlay_agrees _ _ patch k rfl

inductive PatchObserved {K V A : Type} [DecidableEq K] (patch : List (K × Option V)) :
    Query K V A → A → List (K × Cell V) → Prop where
  | done : PatchObserved patch (.done a) a []
  | read : PatchObserved patch (next (patchedValue patch k c)) a rs →
      PatchObserved patch (.read k next) a ((k,c) :: rs)

inductive PatchPreparation {K V A : Type} [DecidableEq K] (patch : List (K × Option V)) :
    (K → Cell V) → Query K V A → (K → Cell V) → A → List (K × Cell V) → Prop where
  | done : PatchPreparation patch s (.done a) s a []
  | read : PatchPreparation patch s (next (patchedValue patch k (s k))) t a rs →
      PatchPreparation patch s (.read k next) t a ((k,s k) :: rs)
  | mutate : PatchPreparation patch (write s k v) q t a rs →
      PatchPreparation patch s q t a rs

theorem patch_preparation_reachable [DecidableEq K]
    {patch : List (K × Option V)} {s t : K → Cell V} {q : Query K V A}
    {a : A} {rs : List (K × Cell V)} (h : PatchPreparation patch s q t a rs) :
    ∃ ws, writes s ws = t := by
  induction h with
  | done => exact ⟨[],rfl⟩
  | read _ ih => exact ih
  | @mutate s k v q t a rs h ih =>
    obtain ⟨ws,hw⟩ := ih
    exact ⟨(k,v)::ws,hw⟩

theorem patch_preparation_origins [DecidableEq K]
    {patch : List (K × Option V)} {s t : K → Cell V} {q : Query K V A}
    {a : A} {rs : List (K × Cell V)} (h : PatchPreparation patch s q t a rs) :
    RowOrigins t rs := by
  induction h with
  | done => intro k c present; simp at present
  | @read s next k t a rs h ih =>
    intro j c present
    rcases List.mem_cons.mp present with head | tail
    · obtain ⟨rfl,rfl⟩ := Prod.mk.inj head
      obtain ⟨ws,hw⟩ := patch_preparation_reachable h
      exact ⟨s,ws,rfl,hw⟩
    · exact ih j c tail
  | mutate _ ih => exact ih

theorem patch_preparation_observed [DecidableEq K]
    {patch : List (K × Option V)} {s t : K → Cell V} {q : Query K V A}
    {a : A} {rs : List (K × Cell V)} (h : PatchPreparation patch s q t a rs) :
    PatchObserved patch q a rs := by
  induction h with
  | done => exact .done
  | read _ ih => exact .read ih
  | mutate _ ih => exact ih

theorem patch_observed_replay [DecidableEq K]
    {patch : List (K × Option V)} {q : Query K V A} {a : A} {rs : List (K × Cell V)}
    (h : PatchObserved patch q a rs) (t : K → Cell V)
    (values : ∀ k c, (k,c) ∈ rs → c = t k) :
    Evaluates (overlay (fun k => (t k).value) patch) q a (rs.map Prod.fst) := by
  induction h with
  | done => exact .done
  | @read next a rs k c h ih =>
    have head := values k c (by simp)
    have tail := ih (fun j d hd => values j d (by simp [hd]))
    have eq : patchedValue patch k c = overlay (fun k => (t k).value) patch k := by
      rw [head]; exact patchedValue_exact patch t k
    rw [eq] at tail
    exact .read tail

theorem interleaved_patch_replay [DecidableEq K]
    {patch : List (K × Option V)} {s t : K → Cell V} {q : Query K V A}
    {a : A} {rs : List (K × Cell V)} (h : PatchPreparation patch s q t a rs)
    (checked : checkObservedStamps t rs = true) :
    evaluate (overlay (fun k => (t k).value) patch) q = (a,rs.map Prod.fst) := by
  apply (evaluate_exact _ _ _ _).mpr
  apply patch_observed_replay (patch_preparation_observed h) t
  intro k c present
  obtain ⟨before,ws,hb,hw⟩ := patch_preparation_origins h k c present
  have stamps : ∀ e ∈ rs, e.2.version = (t e.1).version := by
    simpa [checkObservedStamps] using checked
  have cell := equal_version_same_cell before ws k (by simpa [hb,hw] using stamps (k,c) present)
  simpa [hb,hw] using cell

#print axioms patch_preparation_reachable
#print axioms patch_preparation_origins
#print axioms patch_preparation_observed
#print axioms patch_observed_replay
#print axioms interleaved_patch_replay

theorem patch_publication_replay [DecidableEq K]
    {patch : List (K × Option V)} {s t : K → Cell V} {q : Query K V A}
    {a : A} {rs : List (K × Cell V)} (h : PatchPreparation patch s q t a rs)
    (checked : checkObservedStamps t rs = true) :
    evaluate (fun k => (publish t patch k).value) q = (a,rs.map Prod.fst) := by
  have values : (fun k => (publish t patch k).value) =
      overlay (fun k => (t k).value) patch := funext (publish_values _ _)
  rw [values]
  exact interleaved_patch_replay h checked

namespace PatchTraceControls
def start : Nat → Cell Nat := fun _ => ⟨0,none⟩
def patch : List (Nat × Option Nat) := [(0,some 7)]
def finish := write start 1 (some 8)
def query : Query Nat Nat (Option Nat × Option Nat) :=
  .read 0 fun x => .read 1 fun y => .done (x,y)
def observations : List (Nat × Cell Nat) := [(0,start 0),(1,finish 1)]
example : PatchPreparation patch start query finish (some 7,some 8) observations := by
  apply PatchPreparation.read
  apply PatchPreparation.mutate (k := 1) (v := some 8)
  apply PatchPreparation.read
  exact PatchPreparation.done
#guard checkObservedStamps finish observations
#guard (evaluate (fun k => (publish finish patch k).value) query).1 = (some 7,some 8)
#guard (evaluate (fun k => (publish start patch k).value) query).1 != (some 7,some 8)
-- Changing the patch between preparation and publication violates the identity premise.
#guard (evaluate (fun k => (publish finish [(0,some 99)] k).value) query).1 != (some 7,some 8)
#guard !checkObservedStamps (write finish 0 (some 3)) observations
end PatchTraceControls
#print axioms patch_publication_replay

namespace ReviewControls
def start : Nat → Cell Nat := fun _ => ⟨0,some 0⟩
def finish := write start 0 (some 1)
def repeated : Query Nat Nat Bool := .read 0 fun x => .read 0 fun y => .done (x != y)
def observations : List (Nat × Cell Nat) := [(0,start 0),(0,finish 0)]
example : PreparationTrace start repeated finish true observations := by
  apply PreparationTrace.read
  apply PreparationTrace.mutate (k := 0) (v := some 1)
  apply PreparationTrace.read
  exact PreparationTrace.done
#guard !checkObservedStamps finish observations
#guard checkObservedStamps finish [(0,finish 0)]
#guard !(evaluate (fun k => (finish k).value) repeated).1
#guard checkStamps start start []
#guard (commit start start [] (.done false)).isNone
#guard (publish start [(0,some 8),(0,some 9)] 0).version = 2
#guard (publish start [(0,some 8),(0,some 9)] 1).value = (start 1).value
-- A reset/current-state forgery lies outside the required writes history.
def fabricated : Nat → Cell Nat := fun k => if k = 0 then ⟨0,some 1⟩ else start k
def isZero : Query Nat Nat Bool := .read 0 fun v => .done (v == some 0)
#guard (commit start fabricated [] isZero).isSome
#guard !(evaluate (fun k => (fabricated k).value) isZero).1
end ReviewControls
end MirroreaProofFirst.TrackedValidation
