import OwnerStatementIdentity
import OwnerStatementCursor
namespace MirroreaProofFirst.OwnerStatementControls
open OwnerStatementIdentity MixedSourceCursor
instance {Item : Type} [DecidableEq Item] (program : List Item) (cursor : Cursor Item) :
 Decidable (OwnerStatementCursor.Partition program cursor) :=
 inferInstanceAs (Decidable (cursor.completed ++ cursor.stopped.toList ++ cursor.remaining = program))
instance {Item : Type} [DecidableEq Item] (status : ReferenceSource.Status) (cursor : Cursor Item) :
 Decidable (OwnerStatementCursor.Shape status cursor) := by
 cases status <;> unfold OwnerStatementCursor.Shape <;> infer_instance

structure Code where
 owner : String
 site : String
 parameters : List String
 body : Nat
 contract : String
 generation : Nat
 label : Nat
 deriving DecidableEq
def scope : Scope := ⟨"checked-program-1","refresh"⟩
def first : Code := ⟨"S","line1",["target","unused"],7,"current-owner",1,2⟩
def second : Code := ⟨"T","line2",["target","unused"],8,"current-owner",1,2⟩
def source : List Code := [first,second,first]
def rows := lower scope 0 source
#guard check scope source rows
#guard (rows.map (fun e => e.key.ordinal)) = [0,1,2]
#guard (rows.map (fun e => e.code.owner)) = ["S","T","S"]
#guard !check scope source rows.reverse
#guard !check scope source (rows++[⟨⟨scope,0⟩,first⟩])
#guard !check scope source (rows.drop 1)
#guard !check {scope with handler := "other"} source rows
#guard !check {scope with program := "other"} source rows
#guard restore scope source rows = some rows
#guard check scope [second] (lower scope 0 [second])

-- Cursor-only finite controls. Nat state here is an opaque marker, not an
-- expected owner log. Actual modeled owner invariant is in Session theorem.
def ready : Outcome Nat := ⟨99,.ready⟩
def waiting : Outcome Nat := ⟨99,.waiting⟩
def failed : Outcome Nat := ⟨99,.failed .rejected⟩
def cursor : Cursor Nat := ⟨[],[1,2,3],none⟩
def pending := (tick ready cursor (fun _ => waiting) ready).cursor
def acknowledged := (tick waiting pending (fun _ => failed) ready).cursor
#guard pending = ⟨[],[2,3],some 1⟩
#guard acknowledged = ⟨[1],[2,3],none⟩
#guard (tick waiting pending (fun _ => failed) waiting).cursor = pending
#guard (tick ready acknowledged (fun _ => failed) ready).cursor = ⟨[1],[3],some 2⟩
#guard (tick failed pending (fun _ => ready) ready).outcome.state = 99
#guard decide (OwnerStatementCursor.Partition [1,2,3] pending)
#guard decide (OwnerStatementCursor.Partition [1,2,3] acknowledged)
-- Essential shape premise: raw ready+stopped state is not admitted.
def incoherent : Cursor Nat := ⟨[],[2],some 1⟩
#guard decide (OwnerStatementCursor.Partition [1,2] incoherent)
#guard !decide (OwnerStatementCursor.Shape .ready incoherent)
#guard !decide (OwnerStatementCursor.Partition [1,2] (tick ready incoherent (fun _ => ready) ready).cursor)
end MirroreaProofFirst.OwnerStatementControls
