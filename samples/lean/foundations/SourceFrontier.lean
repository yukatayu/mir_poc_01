import Std
namespace MirroreaProofFirst.SourceFrontier

-- A view of the existing Session's three frontier fields, NOT a scheduler.
-- Status, source state, authority, program/place and archives stay at the caller.
structure Cursor (A : Type) where
 completed : List A
 stopped : Option A
 remaining : List A
 deriving DecidableEq, Repr

def Partition (program : List A) (f : Cursor A) : Prop :=
 program = f.completed ++ f.stopped.toList ++ f.remaining

-- These total record operations are used only under the existing driver's
-- ready/waiting checks. No helper grants permission to acknowledge a failed job.
def begin (f : Cursor A) (item : A) (rest : List A) (immediate : Bool) : Cursor A :=
 if immediate then ⟨f.completed ++ [item],none,rest⟩ else ⟨f.completed,some item,rest⟩
def finish (f : Cursor A) : Cursor A := ⟨f.completed ++ f.stopped.toList,none,f.remaining⟩

-- Independent source-order relation. There is no supplied step-correct callback.
inductive Moves : Cursor A → Cursor A → Prop where
 | immediate : Moves ⟨done,none,item::rest⟩ ⟨done++[item],none,rest⟩
 | suspend : Moves ⟨done,none,item::rest⟩ ⟨done,some item,rest⟩
 | receive : Moves ⟨done,some item,rest⟩ ⟨done++[item],none,rest⟩

inductive Command where
 | begin (immediate : Bool)
 | receive
 deriving DecidableEq, Repr

def step (f : Cursor A) : Command → Option (Cursor A)
 | .begin immediate => match f.stopped, f.remaining with
     | none,item::rest => some (begin f item rest immediate)
     | _,_ => none
 | .receive => match f.stopped with
     | some _ => some (finish f)
     | none => none

theorem step_sound (h : step f command = some next) : Moves f next := by
 cases f with | mk done stopped remaining =>
  cases command with
  | begin immediate =>
   cases stopped <;> cases remaining <;> simp only [step,reduceCtorEq] at h
   case none.cons item rest =>
    cases immediate <;> simp only [begin,Bool.false_eq_true,ite_false,ite_true,Option.some.injEq] at h
    · subst next; exact .suspend
    · subst next; exact .immediate
  | receive =>
   cases stopped <;> simp only [step,reduceCtorEq] at h
   case some item => cases h; exact .receive

theorem step_complete (h : Moves f next) : ∃ c, step f c = some next := by
 cases h with
 | immediate => exact ⟨.begin true,rfl⟩
 | suspend => exact ⟨.begin false,rfl⟩
 | receive => exact ⟨.receive,rfl⟩

theorem step_exact : (∃ c, step f c = some next) ↔ Moves f next :=
 ⟨fun ⟨_,h⟩ => step_sound h,step_complete⟩

theorem moves_partition (valid : Partition program f) (move : Moves f next) :
 Partition program next := by
 cases move <;> simpa [Partition,List.append_assoc] using valid

theorem begin_partition (valid : Partition program f) (idle : f.stopped = none)
 (head : f.remaining = item::rest) : Partition program (begin f item rest immediate) := by
 cases immediate <;> simpa [Partition,begin,idle,head,List.append_assoc] using valid

theorem finish_partition (valid : Partition program f) : Partition program (finish f) := by
 simpa [Partition,finish,List.append_assoc] using valid

theorem begin_while_waiting_rejected (waiting : f.stopped = some item) :
 step f (.begin immediate) = none := by simp [step,waiting]

theorem duplicate_receive_rejected : step (finish f) .receive = none := rfl

theorem receive_keeps_residual : (finish f).remaining = f.remaining := rfl

theorem receive_adds_exact_stopped : (finish f).completed = f.completed ++ f.stopped.toList := rfl

def map (f : A → B) (cursor : Cursor A) : Cursor B :=
 ⟨cursor.completed.map f,cursor.stopped.map f,cursor.remaining.map f⟩

theorem map_begin (f : A → B) : map f (begin cursor item rest immediate) =
 begin (map f cursor) (f item) (rest.map f) immediate := by
 cases immediate <;> simp [map,begin]

theorem map_finish (f : A → B) : map f (finish cursor) = finish (map f cursor) := by
 cases stopped : cursor.stopped <;> simp [map,finish,stopped]

theorem map_retraction (embed : A → B) (project : B → A)
 (retract : ∀ x, project (embed x) = x) : map project (map embed cursor) = cursor := by
 cases cursor
 simp [map,List.map_map,Option.map_map,Function.comp_def,retract]

#print axioms map_begin
#print axioms map_finish
#print axioms map_retraction

#print axioms step_exact
#print axioms moves_partition
#print axioms begin_partition
#print axioms finish_partition
#print axioms begin_while_waiting_rejected
#print axioms duplicate_receive_rejected
end MirroreaProofFirst.SourceFrontier
