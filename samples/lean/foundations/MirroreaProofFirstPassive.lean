import Std
namespace MirroreaProofFirst.Passive
inductive Action (I R : Type) where
  | domain : I → Action I R
  | observe : R → Action I R

def erase : List (Action I R) → List I
  | [] => []
  | .domain i :: rest => i :: erase rest
  | .observe _ :: rest => erase rest

def domainRun (step : S → I → S) : S → List I → S
  | s, [] => s
  | s, i :: rest => domainRun step (step s i) rest

def instrumented (step : S → I → S) (observe : S → O → R → O) :
    S × O → List (Action I R) → S × O
  | so, [] => so
  | (s,o), .domain i :: rest => instrumented step observe (step s i,o) rest
  | (s,o), .observe r :: rest => instrumented step observe (s,observe s o r) rest

theorem erasure (step : S → I → S) (observe : S → O → R → O)
    (actions : List (Action I R)) (s : S) (o : O) :
    (instrumented step observe (s,o) actions).1 = domainRun step s (erase actions) := by
  induction actions generalizing s o with
  | nil => rfl
  | cons a rest ih => cases a <;> simp only [instrumented,erase,domainRun] <;> apply ih

def liftInputs (is : List I) : List (Action I R) := is.map Action.domain

theorem lift_erases (is : List I) : erase (liftInputs (R := R) is) = is := by
  induction is with
  | nil => rfl
  | cons i rest ih => simp [liftInputs,erase] at *; exact ih

def retain (cap : Nat) (rows : List Row) (row : Row) : List Row :=
  (row :: rows).take cap

def ingest (project : Event → Option Row) (cap : Nat) (rows : List Row) (e : Event) : List Row :=
  match project e with
  | none => rows
  | some row => retain cap rows row

def feed (project : Event → Option Row) (cap : Nat) : List Row → List Event → List Row
  | rows, [] => rows
  | rows, e :: rest => feed project cap (ingest project cap rows e) rest

def feedRows (cap : Nat) : List Row → List Row → List Row
  | rows, [] => rows
  | rows, row :: rest => feedRows cap (retain cap rows row) rest

theorem filter_before_retention (project : Event → Option Row) (cap : Nat)
    (events : List Event) (rows : List Row) :
    feed project cap rows events = feedRows cap rows (events.filterMap project) := by
  induction events generalizing rows with
  | nil => rfl
  | cons e rest ih =>
    cases h : project e <;> simp [feed,ingest,h,feedRows,ih]

theorem retained_bound (project : Event → Option Row) (cap : Nat)
    (events : List Event) (rows : List Row) (h : rows.length ≤ cap) :
    (feed project cap rows events).length ≤ cap := by
  induction events generalizing rows with
  | nil => exact h
  | cons e rest ih =>
    apply ih
    cases hp : project e with
    | none => simpa [ingest,hp] using h
    | some r => simp [ingest,hp,retain,List.length_take]; omega

theorem no_invented_rows (project : Event → Option Row) (cap : Nat)
    (events : List Event) (rows : List Row) (r : Row)
    (h : r ∈ feed project cap rows events) :
    r ∈ rows ∨ ∃ e ∈ events, project e = some r := by
  induction events generalizing rows with
  | nil => exact Or.inl h
  | cons e rest ih =>
    rcases ih _ h with hm | ⟨e',he',hp⟩
    · cases hp : project e with
      | none => exact Or.inl (by simpa [ingest,hp] using hm)
      | some row =>
        have hh : r ∈ row :: rows := List.mem_of_mem_take (by simpa [ingest,hp,retain] using hm)
        rcases List.mem_cons.mp hh with heq | hin
        · subst r; exact Or.inr ⟨e,by simp,hp⟩
        · exact Or.inl hin
    · exact Or.inr ⟨e',by simp [he'],hp⟩

inductive Aligned (project : Event → Option Row) : List Event → List Event → Prop where
  | nil : Aligned project [] []
  | hiddenLeft (h : project e = none) : Aligned project xs ys → Aligned project (e::xs) ys
  | hiddenRight (h : project e = none) : Aligned project xs ys → Aligned project xs (e::ys)
  | visible (h : project e = some r) (h' : project e' = some r) :
      Aligned project xs ys → Aligned project (e::xs) (e'::ys)

theorem aligned_projection (h : Aligned project xs ys) :
    xs.filterMap project = ys.filterMap project := by
  induction h with
  | nil => rfl
  | hiddenLeft hp _ ih => simp [hp,ih]
  | hiddenRight hp _ ih => simp [hp,ih]
  | visible hp hp' _ ih => simp [hp,hp',ih]

theorem two_trace_retention (project : Event → Option Row) (h : Aligned project xs ys) (cap : Nat) (rows : List Row) :
    feed project cap rows xs = feed project cap rows ys := by
  rw [filter_before_retention,filter_before_retention,aligned_projection h]

#print axioms erasure
#print axioms lift_erases
#print axioms filter_before_retention
#print axioms retained_bound
#print axioms no_invented_rows
#print axioms aligned_projection
#print axioms two_trace_retention

-- Concrete two-level domain: private steps may inspect both values and change
-- private data; public arithmetic depends only on public state/input. This is
-- a bounded research fragment, not a source-language typing theorem.
inductive Event2 where
  | publicValue : Nat → Event2
  | privateValue : Nat → Event2
  deriving DecidableEq, Repr

def lowProject : Event2 → Option Nat
  | .publicValue n => some n
  | .privateValue _ => none

inductive Command where
  | publicAdd : Nat → Command
  | privateUpdate : (Nat → Nat → Nat) → Command

def publicInputs : List Command → List Nat
  | [] => []
  | .publicAdd n :: cs => n :: publicInputs cs
  | .privateUpdate _ :: cs => publicInputs cs

def trace2 : Nat → Nat → List Command → List Event2
  | _, _, [] => []
  | l, h, .publicAdd n :: cs => .publicValue (l+n) :: trace2 (l+n) h cs
  | l, h, .privateUpdate f :: cs => .privateValue (f l h) :: trace2 l (f l h) cs

def publicTrace : Nat → List Nat → List Nat
  | _, [] => []
  | l, n :: ns => (l+n) :: publicTrace (l+n) ns

theorem concrete_projection (cs : List Command) (l h : Nat) :
    (trace2 l h cs).filterMap lowProject = publicTrace l (publicInputs cs) := by
  induction cs generalizing l h with
  | nil => rfl
  | cons c cs ih => cases c <;> simp [trace2,publicInputs,publicTrace,lowProject,ih]

theorem concrete_two_run (xs ys : List Command) (l h h' : Nat)
    (sameInputs : publicInputs xs = publicInputs ys) (cap : Nat) :
    feed lowProject cap [] (trace2 l h xs) = feed lowProject cap [] (trace2 l h' ys) := by
  rw [filter_before_retention,filter_before_retention,concrete_projection,
    concrete_projection,sameInputs]

namespace Controls
def publicOnly : List Command := [.publicAdd 2,.publicAdd 3]
def withSecrets : List Command := [.privateUpdate (fun l h => l+h+7),.publicAdd 2,
  .privateUpdate (fun l h => l*h),.publicAdd 3]
#guard feed lowProject 2 [] (trace2 10 42 withSecrets) == [15,12]
#guard feed lowProject 2 [] (trace2 10 900 publicOnly) == [15,12]
-- Minimal bad order: newest mixed event consumes capacity before redaction.
def bad (cap : Nat) (events : List Event2) :=
  (events.reverse.take cap).filterMap lowProject
#guard bad 1 [.publicValue 7] == [7]
#guard bad 1 [.publicValue 7,.privateValue 99] == []
#guard feed lowProject 1 [] [.publicValue 7,.privateValue 99] == [7]
-- Raw row indices expose the insertion count even when values are redacted.
#guard ([Event2.publicValue 7].zipIdx.filterMap (fun (e,i) => (lowProject e).map (fun n => (i,n)))) == [(0,7)]
#guard ([Event2.privateValue 99,.publicValue 7].zipIdx.filterMap (fun (e,i) => (lowProject e).map (fun n => (i,n)))) == [(1,7)]
-- A readonly projector may still leak; erasure alone never proves secrecy.
#guard (instrumented (fun (s : Nat) (i : Nat) => s+i)
  (fun s (_ : List Nat) (_ : Unit) => [s]) (99,[]) [.observe ()]).1 == 99
#guard (instrumented (fun (s : Nat) (i : Nat) => s+i)
  (fun s (_ : List Nat) (_ : Unit) => [s]) (99,[]) [.observe ()]).2 == [99]
#guard (instrumented (fun (s : Nat) (i : Nat) => s+i)
  (fun s (_ : List Nat) (_ : Unit) => s::[]) (0,[])
  [.observe (),.domain 3,.observe (),.domain 4]).1 == 7
end Controls
#print axioms concrete_projection
#print axioms concrete_two_run
private theorem append_take_cap (xs ys : List A) (cap : Nat) :
    (xs ++ ys.take cap).take cap = (xs ++ ys).take cap := by
  rw [List.take_append, List.take_append, List.take_take]
  congr 1
  congr 1
  omega

theorem feedRows_exact (cap : Nat) (events rows : List Row) (h : rows.length ≤ cap) :
    feedRows cap rows events = (events.reverse ++ rows).take cap := by
  induction events generalizing rows with
  | nil => simpa [feedRows] using (List.take_of_length_le h).symm
  | cons e es ih =>
    rw [feedRows, ih]
    · simpa [retain,List.reverse_cons,List.append_assoc] using
        append_take_cap es.reverse (e::rows) cap
    · simp [retain,List.length_take]; omega

theorem retained_exact (project : Event → Option Row) (cap : Nat)
    (events : List Event) (rows : List Row) (h : rows.length ≤ cap) :
    feed project cap rows events = ((events.filterMap project).reverse ++ rows).take cap := by
  rw [filter_before_retention,feedRows_exact cap _ _ h]

namespace ReviewControls
#guard feed lowProject 1 [] (trace2 0 0 [Command.publicAdd 0]) == [0]
#guard feed lowProject 1 [] (trace2 0 1 []) == []
-- Completed low input lists agree; response cuts at the first command do not.
def short : List Command := [.publicAdd 7]
def delayed : List Command := [.privateUpdate (fun _ _ => 0),.publicAdd 7]
#guard publicInputs short == publicInputs delayed
#guard feed lowProject 1 [] (trace2 0 0 (short.take 1)) == [7]
#guard feed lowProject 1 [] (trace2 0 0 (delayed.take 1)) == []
-- Retaining prior rows under a different policy requires an additional premise.
#guard feed lowProject 1 [99] [.privateValue 3] == [99]
#guard feed lowProject 0 [7] [] == [7]
#guard feed lowProject 0 [] [.publicValue 7] == []
-- Hidden entries affect latest-before-filter even with no capacity truncation.
def latestThenProject (events : List Event2) := events.getLast?.bind lowProject
#guard latestThenProject [.publicValue 7] == some 7
#guard latestThenProject [.publicValue 7,.privateValue 9] == none
#guard ([Event2.publicValue 7,.privateValue 9].filterMap lowProject).getLast? == some 7
-- Public-step mutation exposes high even with unchanged public input.
def badPublic (l h n : Nat) : List Event2 := [.publicValue (l+n+h)]
#guard feed lowProject 1 [] (badPublic 0 0 0) == [0]
#guard feed lowProject 1 [] (badPublic 0 1 0) == [1]
-- Exact retention keeps multiplicity and newest order, rather than set origin.
#guard feed lowProject 3 [4] [.publicValue 7,.publicValue 7,.publicValue 8] == [8,7,7]
end ReviewControls
#print axioms feedRows_exact
#print axioms retained_exact
end MirroreaProofFirst.Passive
