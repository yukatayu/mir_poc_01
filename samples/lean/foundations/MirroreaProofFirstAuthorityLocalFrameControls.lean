import MirroreaProofFirstAuthorityLocalFrame

namespace MirroreaProofFirst.AuthorityLocalFrameControls
open AuthorityLocalFrame

structure Parent where
 program : Nat
 generation : Nat
 sAllowed : Bool
 tAllowed : Bool
 deriving DecidableEq, Repr
structure Fact where
 program : Nat
 generation : Nat
 selectedAllowed : Bool
 deriving DecidableEq, Repr

def prior : Parent := ⟨42,100,true,true⟩
def evaluate (p : Parent) (_ : Unit) : Option Parent :=
 if p.sAllowed then some {p with generation := p.generation+1,sAllowed := false} else none

def project (endpoint : Fin 3) (p : Parent) : Fact :=
 ⟨p.program,p.generation,if endpoint.val = 0 then p.sAllowed else p.tAllowed⟩

def initial (endpoint : Fin 3) : Local Fact Nat (List Nat) (List Nat) (List Nat) :=
 ⟨project endpoint prior,project endpoint prior,
  ⟨7,[2,5],[19,23]⟩,[901,902,903]⟩

def prepared (endpoint : Fin 3) :=
 (stage evaluate (project endpoint) prior ()).bind (prepare (initial endpoint))

#guard (prepared 0).isSome
#guard (prepared 1).isSome
#guard (prepared 2).isSome
#guard (prepared 0).map (fun s => s.facts) = some ⟨42,101,false⟩
#guard (prepared 1).map (fun s => s.facts) = some ⟨42,101,true⟩
#guard (prepared 2).map (fun s => s.facts) = some ⟨42,101,true⟩
#guard (prepared 0).map (fun s => s.observations) = some ⟨7,[2,5],[19,23]⟩
#guard (prepared 0).map (fun s => s.retained) = some [901,902,903]
#guard (prepared 0).map (fun s => s.floor) = (prepared 0).map (fun s => s.facts)

def wrongProgram : Local Fact Nat (List Nat) (List Nat) (List Nat) := {initial 0 with facts := ⟨43,100,true⟩,floor := ⟨43,100,true⟩}
def staleFloor : Local Fact Nat (List Nat) (List Nat) (List Nat) := {initial 0 with floor := ⟨42,101,false⟩}
def wrongPrior : Local Fact Nat (List Nat) (List Nat) (List Nat) := {initial 0 with facts := ⟨42,100,false⟩,floor := ⟨42,100,false⟩}
def delta := stage evaluate (project 0) prior ()
#guard (delta.bind (prepare wrongProgram)).isNone
#guard (delta.bind (prepare staleFloor)).isNone
#guard (delta.bind (prepare wrongPrior)).isNone
#guard (prepared 0).bind (fun s => delta.bind (prepare s)) = none

-- Exact child histories can differ; an endpoint's successful preparation never
-- substitutes the first child's observations, the parent's zeroes, or a merge.
def other := {initial 2 with observations := ⟨11,[6],[70,80,90]⟩,retained := [999]}
def otherPrepared := (stage evaluate (project 2) prior ()).bind (prepare other)
#guard otherPrepared.map (fun s => s.observations) = some ⟨11,[6],[70,80,90]⟩
#guard otherPrepared.map (fun s => s.retained) = some [999]
#guard ((evaluate prior ()).bind fun p => stage evaluate (project 0) p ()).isNone

end MirroreaProofFirst.AuthorityLocalFrameControls
