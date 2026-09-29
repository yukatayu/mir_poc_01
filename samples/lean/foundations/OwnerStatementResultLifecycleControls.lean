import OwnerStatementResultLifecycle
import OwnerStatementSourceCustodianControls
import OwnerStatementResultCollectionControls
namespace MirroreaProofFirst.OwnerStatementResultLifecycleControls
open OwnerStatementLiveCustodian OwnerStatementSourceCustodian OwnerStatementSourceCustodianControls OwnerStatementResultCollection

def runActualOne (capacity fuel : Nat) (s : OwnerStatementLiveCustodian.State p a)
 (member : Fin a) (principal : Nat) : Option (OwnerStatementLiveCustodian.State p a) := do
 let queued ← issue fuel s member principal
 let (staged,ticket) ← stage capacity queued
 let executed ← execute staged ticket (FallibleFlow.signed 63)
 let reported ← reportActual executed ticket
 consume reported ticket member principal

def actualFirst := initial.bind fun s => runActualOne 4 1 s 0 7
def actualSecond := actualFirst.bind fun s => runActualOne 4 3 s 0 7
def again := actualSecond.bind invokeAgain
def third := again.bind fun s => runActualOne 4 3 s 0 7
def fourth := third.bind fun s => runActualOne 4 3 s 0 7
#guard actualFirst.isSome && actualSecond.isSome && fourth.isSome
#guard (actualFirst.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.dispatched.length)) = some (some 190,1,1)
#guard (actualSecond.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.dispatched.length)) = some (some 17,2,2)
#guard (fourth.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.dispatched.length,s.live.session.activation)) = some (some 17,2,4,1)
#guard (OwnerStatementResultCollectionControls.refused.bind fun (s,t) => reportActual s t).isNone

-- Only outer custody is forged: the live world and genuine earlier attempt
-- remain unchanged. Raw history lookup/reporting still cannot establish that
-- the current stopped statement owns this old ticket.
def oldTicket := executed.bind fun (_,old) => do
 let first ← actualFirst
 let next ← issue 3 first 0 7
 return ({next with held := some ⟨old,.unreported⟩},old)
def oldReported := oldTicket.bind fun (s,t) => (reportActual s t).map fun next => (next,t)
#guard oldTicket.isSome && oldReported.isSome
#guard (oldReported.bind fun (s,t) => collect s t |>.map fun result => match result with | .committed w => w.value | _ => 0) = some 190
#guard (oldReported.map fun (s,t) => (MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting).map (fun w => w.saved) == some t.saved) = some false
#guard (oldReported.bind fun (s,t) => consume s t 0 7).isNone

-- This is the old broader operation, retained as a falsifier rather than
-- silently changing its historical meaning.
def phaseOnlyFailure := OwnerStatementResultCollectionControls.refused.bind fun (s,t) => (report s t).map fun next => (next,t)
#guard phaseOnlyFailure.isSome
#guard (phaseOnlyFailure.bind fun (s,t) => collect s t |>.map fun result => match result with | .refused _ => true | _ => false) = some true

theorem progressN_path (run : progressN fuel s member principal = some next) :
 OwnerStatementResultLifecycle.Path capacity s next := by
 induction fuel generalizing s with
 | zero => cases run; exact .initial
 | succ fuel ih =>
   unfold progressN at run
   cases step : progress s member principal with
   | none => simp [step] at run
   | some middle =>
     simp only [step,Option.bind_eq_bind,Option.bind_some] at run
     have tail := ih run
     exact OwnerStatementResultLifecycle.path_trans (.progress .initial step) tail

theorem runActualOne_path (run : runActualOne capacity fuel s member principal = some next) :
 OwnerStatementResultLifecycle.Path capacity s next := by
 unfold runActualOne issue at run
 cases progressed : progressN fuel s member principal with
 | none => simp [progressed] at run
 | some ready =>
   simp only [progressed,Option.bind_eq_bind,Option.bind_some] at run
   cases transferred : OwnerStatementSourceCustodian.transfer ready with
   | none => simp [transferred] at run
   | some queued =>
     simp only [transferred,Option.bind_some] at run
     cases reserved : stage capacity queued with
     | none => simp [reserved] at run
     | some pair =>
       obtain ⟨staged,ticket⟩ := pair
       simp only [reserved,Option.bind_some] at run
       cases dispatched : execute staged ticket (FallibleFlow.signed 63) with
       | none => simp [dispatched] at run
       | some done =>
         simp only [dispatched,Option.bind_some] at run
         cases delivered : reportActual done ticket with
         | none => simp [delivered] at run
         | some response =>
           simp only [delivered,Option.bind_some] at run
           exact .consume (.report (.execute (.stage (.transfer (progressN_path progressed) transferred) reserved) dispatched) delivered) run


#print axioms progressN_path
#print axioms runActualOne_path
end MirroreaProofFirst.OwnerStatementResultLifecycleControls
