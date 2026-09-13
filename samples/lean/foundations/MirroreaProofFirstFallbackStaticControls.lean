import MirroreaProofFirstFallbackStatic

namespace MirroreaProofFirst.FallbackStaticControls
open FallbackStatic InstanceState

def resultIs (actual expected : Except Error Unit) : Bool :=
  match actual,expected with
  | .ok _,.ok _ => true
  | .error a,.error b => decide (a = b)
  | _,_ => false

def primary : OptionDecl := ⟨"extra",1,some "score",.read,InstancePrograms.Controls.contract,100⟩
def terminal : OptionDecl := ⟨"base",0,some "score",.read,InstancePrograms.Controls.contract,100⟩
def edge : EdgeDecl := ⟨"extra","base",true⟩
def chain : Chain := ⟨0,[primary,terminal],[some edge]⟩
def state := InstanceState.Controls.two

-- A reader may try a shorter-lived additional instance and end at itself.
#guard resultIs (check state chain) (.ok ())
#guard resultIs (check state {chain with options := [terminal], edges := []}) (.ok ())
#guard resultIs (check state {chain with options := [primary,{terminal with leaseUntil := 0}]}) (.ok ())
#guard resultIs (check (retire state 1) chain) (.ok ())
#guard resultIs (checkShape {chain with options := []}) (.error .malformed)
#guard resultIs (checkShape {chain with options := [terminal],edges := []}) (.ok ())
#guard resultIs (check state {chain with options := [{primary with declaredAccess := none},terminal]}) (.error .underdeclared)
#guard resultIs (check state {chain with edges := [none]}) (.error .underdeclared)
#guard resultIs (check state {chain with edges := []}) (.error .underdeclared)
#guard resultIs (check state {chain with edges := [some {edge with predecessor := "not-extra"}]}) (.error .malformed)
#guard resultIs (check state {chain with edges := [some {edge with successor := "not-base"}]}) (.error .malformed)
#guard resultIs (check state {chain with edges := [some {edge with sameLineage := false}]}) (.error .malformed)
#guard resultIs (check state {chain with options := [primary,{terminal with declaredAccess := some "different"}]}) (.error .malformed)
#guard resultIs (check state {chain with options := [primary,{terminal with capability := .readWrite}]}) (.error .malformed)
#guard resultIs (check state {chain with options := [{primary with capability := .readWrite},terminal]}) (.ok ())
#guard resultIs (check state {chain with options := [primary,{terminal with contract := ⟨[0],101,200⟩}]}) (.error .malformed)
#guard resultIs (check state {chain with options := [primary,{terminal with contract := ⟨[0],1,100⟩}]}) (.error .outsideProfile)
#guard resultIs (check state {chain with options := [{primary with target := 99},terminal]}) (.error .unresolved)
#guard resultIs (check state {chain with reader := 99}) (.error .lifetime)
#guard resultIs (check state {chain with options := [primary,{terminal with target := 1}]}) (.error .lifetime)
#guard resultIs (check state {chain with options := [primary,{terminal with name := "extra"}]}) (.error .malformed)
#guard resultIs (check state {chain with edges := [some edge,some edge]}) (.error .malformed)

-- The terminal is a real ancestor here; losing that relation is observable to
-- the checker. A future binding-aware mutator must preserve this obligation.
def linked := setParent state 1 (some 0)
def childReader := {chain with reader := 1}
#guard resultIs (check linked childReader) (.ok ())
#guard resultIs (check (setParent linked 1 none) childReader) (.error .lifetime)
#guard dominatesCheck linked 1 0
#guard !dominatesCheck state 1 0
-- Inspect all declared edges before applying the narrower contract profile.
def middle : OptionDecl := {terminal with name := "middle", contract := {terminal.contract with inputs := [0]}}
def wrongEdge : Option EdgeDecl := some ⟨"wrong","base",true⟩
def maskedChain : Chain := ⟨0,[primary,middle,terminal],[some ⟨"extra","middle",true⟩,wrongEdge]⟩
#guard resultIs (checkShape maskedChain) (.error .malformed)
#guard resultIs (checkShape {maskedChain with options := [{primary with declaredAccess := none},middle,terminal]}) (.error .malformed)
#guard resultIs (checkShape {maskedChain with edges := [some ⟨"extra","middle",true⟩,none]}) (.error .underdeclared)
#guard resultIs (checkShape {maskedChain with edges := [some ⟨"extra","middle",true⟩,some ⟨"middle","base",true⟩]}) (.error .outsideProfile)
example : MalformedLinks maskedChain.options maskedChain.edges := by
  apply MalformedLinks.tail
  apply MalformedLinks.head
  exact ⟨"score","score",⟨"wrong","base",true⟩,rfl,rfl,rfl,Or.inr (Or.inl (by decide))⟩
end MirroreaProofFirst.FallbackStaticControls
