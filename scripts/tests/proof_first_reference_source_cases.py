"""Discriminating actual-source checks; each variant uses the real Rust parser."""
from pathlib import Path
import hashlib
import json
import os
import subprocess
import tempfile
import proof_first_reference_source as adapter

SOURCE = (adapter.WORK / 'reference.mir').read_bytes().decode('utf-8')
WORK = Path(tempfile.mkdtemp(prefix='cases-', dir=adapter.WORK))
(adapter.WORK / 'CURRENT_CASES').write_text(str(WORK) + '\n')
results = []
if not __debug__:
    raise SystemExit("optimized Python is not an evidence execution mode")
SESSION_CASES = {"source-cancel-recheck-second-source", "source-session-continue-addition",
                 "source-cancel-expired-repair-stays-failed"}


def check(name, text, assertions=None, adapter_reject=False, extra_sources=None):
    selected = os.environ.get("MIR_W3_CASE_FILTER")
    if selected and selected not in name:
        return
    work = WORK / name
    work.mkdir()
    source = work / 'input.mir'
    source.write_text(text)
    parsed = subprocess.run([str(adapter.PARSER), str(source), '--format', 'json'], capture_output=True, text=True)
    (work / 'PARSE.json').write_text(parsed.stdout)
    (work / 'PARSE.stderr').write_text(parsed.stderr)
    report = json.loads(parsed.stdout)
    # These controls intend semantic/type rejection, not accidental parse errors.
    assert parsed.returncode == 0 and report['accepted'], (name, report['diagnostics'])
    try:
        program, place = adapter.export(report, ['A', 'B', 'C'], text)
    except (ValueError, KeyError) as error:
        if not adapter_reject:
            raise
        receipt = dict(name=name, parser_exit=0, adapter_rejected=True, reason=str(error))
    else:
        assert not adapter_reject, (name, 'unexpected adapter acceptance')
        extras = []
        extra_receipts = []
        for label, extra_text in (extra_sources or {}).items():
            assert label.isidentifier()
            extra_path = work / (label + '.mir')
            extra_path.write_bytes(extra_text.encode('utf-8'))
            extra_parse = subprocess.run([str(adapter.PARSER), str(extra_path), '--format', 'json'], capture_output=True, text=True)
            (work / (label + '-PARSE.json')).write_text(extra_parse.stdout)
            (work / (label + '-PARSE.stderr')).write_text(extra_parse.stderr)
            assert extra_parse.returncode == 0
            extra_program, extra_place = adapter.export(json.loads(extra_parse.stdout), ['A', 'B', 'C'], extra_text)
            extras.append('def ' + label + ' : List Located := [' + ',\n'.join(extra_program) + ']\n')
            extras.append('def ' + label + 'Block : ReferenceContinuation.Program 3 := ⟨' + str(extra_place) + ',' + label + '⟩\n')
            extra_receipts.append(dict(name=label, place=extra_place, source_sha256=hashlib.sha256(extra_path.read_bytes()).hexdigest(), statements=len(extra_program)))
        assertions = '\n'.join(extras) + assertions
        lean = work / 'ActualCase.lean'
        lean.write_text(adapter.generate(program, place, assertions))
        generated_sha = hashlib.sha256(lean.read_bytes()).hexdigest()
        result = subprocess.run(['lean', '--trust=0', lean.name], cwd=work,
                                env=dict(os.environ, LEAN_PATH=str(work) + os.pathsep + str(adapter.WORK)),
                                capture_output=True, text=True, preexec_fn=adapter.cap)
        (work / 'ActualCase.log').write_text(result.stdout + result.stderr)
        receipt = dict(name=name, parser_exit=0, lean_exit=result.returncode, statement_count=len(program), extra_sources=extra_receipts,
                       generated_sha256=generated_sha,
                       log_sha256=hashlib.sha256((work / 'ActualCase.log').read_bytes()).hexdigest())
        (work / 'RESULT.json').write_text(json.dumps(receipt, indent=2) + '\n')
        if hashlib.sha256(lean.read_bytes()).hexdigest() != generated_sha:
            raise RuntimeError('generated case changed during execution: ' + name)
        if result.returncode or 'sorryAx' in result.stdout + result.stderr:
            print(json.dumps(receipt), flush=True)
            print('\n'.join(line for line in result.stdout.splitlines() if 'error' in line), flush=True)
            raise SystemExit(1)
    receipt['scope'] = 'admitted-session' if name in SESSION_CASES else 'source-component-control'
    receipt['source_sha256'] = hashlib.sha256(source.read_bytes()).hexdigest()
    (work / 'RESULT.json').write_text(json.dumps(receipt, indent=2) + '\n')
    results.append(receipt)
    print(name + ' PASS', flush=True)


ready = '#guard checked.isSome\n#guard outcome.status = .ready\n'
static = '#guard checked.isNone\n#guard outcome.status = .failed .staticType\n'
elaboration = '#guard checked.isSome\n#guard outcome.status = .failed .elaboration\n#guard outcome.state.machine.store.bindings = []\n'
check('changed-real-function', SOURCE.replace('return x * x + 2', 'return x * x + 7'), ready +
      '#guard lookup outcome.state.values "afterReacquire" = some (.plain (.integer 16 false))\n')
check('missing-lineage', SOURCE.replace('"score", [true]', '"score", []'), elaboration)
check('false-lineage', SOURCE.replace('"score", [true]', '"score", [false]'), elaboration)
check('unrelated-terminal', SOURCE.replace('  linked <- perform reparent(extra, base) via lifecycle\n', '').replace(
      'perform reference(base,', 'perform reference(extra,'), elaboration)
check('reference-as-integer', SOURCE.replace('first <- alias(count)', 'first <- alias(route)'), static +
      '#guard lookup outcome.state.values "first" = none\n#guard outcome.state.machine.store.bindings.length = 1\n')
check('reference-as-instance', SOURCE.replace('retire(extra)', 'retire(route)'), static)
check('shadow-reference', SOURCE.replace('let alias: ReferenceCallable = route', 'let route: Int64 = 2'), static)
check('mutable-reference', SOURCE.replace('let alias: ReferenceCallable', 'let mut alias: ReferenceCallable'), adapter_reject=True)
check('wrong-provider-type', SOURCE.replace('arg3: BoolList', 'arg3: IntList'), adapter_reject=True)
check('wrong-reference-boundary', SOURCE.replace('[100, 100]) via lifecycle', '[100, 100]) via elsewhere'), adapter_reject=True)
check('missing-lease', SOURCE.replace('[100, 100])', '[100])'), adapter_reject=True)
check('expired-lease', SOURCE.replace('[100, 100])', '[0, 0])'),
      '#guard checked.isSome\n#guard outcome.status = .failed .rejected\n#guard outcome.state.machine.store.bindings = []\n')
check('released-alias', SOURCE[:-2] + '  useReleased <- alias(count)\n}\n',
      '#guard checked.isSome\n#guard outcome.status = .failed (.normalization .absent)\n'
      '#guard lookup outcome.state.values "useReleased" = none\n#guard outcome.state.machine.store.bindings = [none]\n')

# These pauses consume the SAME actual parser-produced program and its named
# call. Interleavings are typed external inputs, not source construction claims.
check('source-pause-revoke-recover', SOURCE, ready + '''
def beforeCall :=  run initial 0 0 7 (program.take 8)
def waiting := advance beforeCall.state 0 0 7 (program[8]'(by decide))
#guard beforeCall.status = .ready
#guard waiting.status = .waiting
def lost := ReferenceSourceControls.head waiting.state [40]
def recovered := ReferenceSourceControls.head lost []
#guard (complete lost).status = .failed .rejected
#guard (complete recovered).status = .failed .rejected
#guard (complete recovered).state.machine.pending = waiting.state.machine.pending
#guard lookup (complete recovered).state.values "first" = none
#guard (advance recovered 0 0 7 (program[12]'(by decide))).status = .failed .awaiting
''')
check('source-normalized-then-denied', SOURCE, ready + '''
def beforeCall :=  run initial 0 0 7 (program.take 8)
def lost := ReferenceSourceControls.head beforeCall.state [40,10]
def denied := advance lost 0 0 7 (program[8]'(by decide))
#guard denied.status = .failed .rejected
#guard denied.state.nextRequest = lost.nextRequest+1
#guard denied.state.machine.store.events.length = lost.machine.store.events.length+1
#guard denied.state.writes = lost.writes
#guard lookup denied.state.values "first" = none
#guard ((ReferenceMutation.lookup denied.state.machine.store 0).bind fun b => b.selected.map ReferenceSelection.Choice.index) = some 1
''')
check('source-no-double-completion', SOURCE, ready + '''
def beforeCall :=  run initial 0 0 7 (program.take 8)
def waiting := advance beforeCall.state 0 0 7 (program[8]'(by decide))
def once := complete waiting.state
#guard once.status = .ready
#guard (complete once.state).status = .failed .awaiting
#guard (complete once.state).state.writes = once.state.writes
#guard once.state.machine.pending = []
''')

check('source-hold-loss-before-call', SOURCE, ready + '''
def beforeCall := run initial 0 0 7 (program.take 8)
def lost := ReferenceSourceControls.head beforeCall.state [70]
def rejected := advance lost 0 0 7 (program[8]'(by decide))
#guard rejected.status = .failed (.normalization .ownerNotLive)
#guard rejected.state.machine.store.bindings = lost.machine.store.bindings
#guard rejected.state.machine.store.events = lost.machine.store.events
#guard rejected.state.nextRequest = lost.nextRequest
#guard rejected.state.writes = lost.writes
def recovered := ReferenceSourceControls.head lost []
#guard (advance recovered 0 0 7 (program[8]'(by decide))).status = .failed (.normalization .ownerNotLive)
def reacquired := advance recovered 0 0 7 (program[12]'(by decide))
#guard reacquired.status = .ready
#guard ((ReferenceMutation.lookup reacquired.state.machine.store 0).map fun b => b.request.epoch) = some 1
def retry := run reacquired.state 0 0 7 [program[8]'(by decide)]
#guard retry.status = .ready
#guard lookup retry.state.values "first" = some (.plain (.integer 10 false))
''')
check('source-hold-loss-while-pending', SOURCE, ready + '''
def beforeCall := run initial 0 0 7 (program.take 8)
def waiting := advance beforeCall.state 0 0 7 (program[8]'(by decide))
def lost := ReferenceSourceControls.head waiting.state [70]
def recovered := ReferenceSourceControls.head lost []
#guard (complete lost).status = .failed .rejected
#guard (complete recovered).status = .failed .rejected
#guard (complete recovered).state.machine.pending = waiting.state.machine.pending
#guard (complete recovered).state.writes = waiting.state.writes
#guard lookup (complete recovered).state.values "first" = none
#guard (waiting.state.waiting.map fun saved => InvocationBoundary.resultCheck
  lost.machine.store.core.system.configuration.state lost.machine.store.core.system.view saved.entry.ticket 10) = some true
#guard (waiting.state.waiting.map fun saved => ReferenceExecution.protectionCheck recovered.machine.store saved.entry) = some false
''')
check('source-fallback-preserves-hold-origin', SOURCE, ready + '''
def beforeCall := run initial 0 0 7 (program.take 8)
def oldHolding := (ReferenceMutation.lookup beforeCall.state.machine.store 0).map ReferenceOwner.Binding.holding
def lost := ReferenceSourceControls.head beforeCall.state [40]
def waiting := advance lost 0 0 7 (program[8]'(by decide))
#guard waiting.status = .waiting
#guard ((ReferenceMutation.lookup waiting.state.machine.store 0).map ReferenceOwner.Binding.holding) = oldHolding
#guard ((ReferenceMutation.lookup waiting.state.machine.store 0).bind fun b => b.selected.map ReferenceSelection.Choice.index) = some 1
#guard (complete waiting.state).status = .ready
#guard lookup (complete waiting.state).state.values "first" = some (.plain (.integer 10 false))
''')
check('source-release-dead-hold', SOURCE, ready + '''
def beforeCall := run initial 0 0 7 (program.take 8)
def lost := ReferenceSourceControls.head beforeCall.state [70,40,41,10]
def released := advance lost 0 0 7 (program[16]'(by decide))
#guard released.status = .ready
#guard released.state.machine.store.bindings = [none]
#guard released.state.machine.store.events.length = lost.machine.store.events.length+1
''')
check('source-cannot-create-holding-authority', SOURCE, ready + '''
def noHoldInitial : State 3 1 := ReferenceSource.initial 91
  {ReferenceSourceControls.view with authority := {ReferenceSourceControls.authority with revoked := [70]}}
  ReferenceSourceControls.policy
def rejected := run noHoldInitial 0 0 7 program
#guard rejected.status = .failed .rejected
#guard rejected.state.machine.store.bindings = []
#guard lookup rejected.state.values "route" = none
#guard rejected.state.machine.store.core.system.configuration.count = 2
''')


REPAIR = SOURCE.split('transition build')[0] + '''transition recover at A requires CompositionControl {
  newEpoch <- perform reacquire(alias) via lifecycle
  recoveredValue <- alias(count)
}
'''
MISSING = SOURCE.split('transition build')[0] + '''transition recover at A requires CompositionControl {
  dependent <- alias(first)
}
'''
check('source-cancel-recheck-second-source', SOURCE, ready + '''
def begun : ReferenceContinuation.Session 3 1 := ⟨initial,programBlock,[],program,none,.ready,[]⟩
#guard (ReferenceContinuation.launch 91 ReferenceSourceControls.view ReferenceSourceControls.policy programBlock).isSome
def paused := ReferenceContinuation.drive 9 begun 0 7
#guard paused.status = .waiting
#guard paused.stopped = some (program[8]'(by decide))
#guard paused.remaining = program.drop 9
def oldView := ReferenceSourceControls.view
def lostView := {oldView with generation := oldView.generation+1,authority := {oldView.authority with revoked := [70]}}
def lost := {paused with state := ReferenceSource.authorityHead paused.state lostView}
#guard (ReferenceContinuation.authorityHead paused lostView).isSome
def newHold := {ReferenceOwnerControls.holdClaim with id := 170}
def restoredView := {lostView with generation := lostView.generation+1,authority := {lostView.authority with issued := newHold :: lostView.authority.issued}}
def restored := {lost with state := ReferenceSource.authorityHead lost.state restoredView}
#guard (ReferenceContinuation.authorityHead lost restoredView).isSome
def failed := ReferenceContinuation.tick restored 0 7
#guard failed.status = .failed .rejected
#guard (ReferenceContinuation.replaceResidual failed repairProgramBlock).isNone
#guard failed.state.waiting.isSome
def cancelled := ReferenceContinuation.cancel failed 0 7
#guard cancelled.status = .failed .cancelled
#guard cancelled.stopped = paused.stopped
#guard cancelled.remaining = paused.remaining
#guard cancelled.state.waiting = none
#guard cancelled.state.machine.pending = []
#guard cancelled.state.machine.store.core.pending = []
#guard cancelled.state.writes = failed.state.writes
#guard cancelled.state.values = failed.state.values
#guard cancelled.state.nextRequest = failed.state.nextRequest+1
#guard lookup cancelled.state.values "first" = none
#guard (ReferenceContinuation.replaceResidual cancelled missingProgramBlock).isNone
#guard (checkProgram 3 [] repairProgram).isNone
#guard (checkProgram 3 (environment cancelled.state.values) repairProgram).isSome
def replacement := ReferenceContinuation.replaceResidual cancelled repairProgramBlock
#guard replacement.isSome
#guard (replacement.map fun next => next.superseded) = some [ReferenceContinuation.archive cancelled]
def repaired := replacement.map fun next => ReferenceContinuation.run next 0 7
#guard (repaired.map fun next => next.status) = some .ready
#guard (repaired.map fun next => lookup next.state.values "recoveredValue") = some (some (.plain (.integer 10 false)))
#guard (repaired.map fun next => lookup next.state.values "first") = some none
#guard (repaired.map fun next => next.state.waiting) = some none
#guard (cancelled.state.origins.head?).map Origin.kind = some .cancellation
#guard (cancelled.state.origins.head?).bind Origin.site = some (program[8]'(by decide)).site
#guard (repaired.bind fun next => next.state.writes.head?).map (fun write => write.site) = some (repairProgram[1]'(by decide)).site
#guard (program[8]'(by decide)).site.document != (repairProgram[1]'(by decide)).site.document

theorem pausedRooted : ReferenceSourceTrace.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy paused.state :=
  ReferenceContinuation.drive_source 9 begun 0 7
theorem lostRooted : ReferenceSourceTrace.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy lost.state :=
  .step pausedRooted .head
theorem restoredRooted : ReferenceSourceTrace.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy restored.state :=
  .step lostRooted .head
theorem failedRooted : ReferenceSourceTrace.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy failed.state :=
  ReferenceSourceTrace.trans _ _ _ restoredRooted (ReferenceContinuation.tick_source restored 0 7)
theorem cancelledRooted : ReferenceSourceTrace.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy cancelled.state :=
  ReferenceSourceTrace.trans _ _ _ failedRooted (ReferenceContinuation.cancel_source failed 0 7)
theorem repairedRooted (next : ReferenceContinuation.Session 3 1)
    (accepted : ReferenceContinuation.replaceResidual cancelled repairProgramBlock = some next) :
    ReferenceSourceTrace.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy
      (ReferenceContinuation.run next 0 7).state := by
  have origin : ReferenceSourceTrace.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy next.state := by
    rw [(ReferenceContinuation.replace_state _ _ _ accepted).1]
    exact cancelledRooted
  exact ReferenceSourceTrace.trans _ _ _ origin (ReferenceContinuation.drive_source _ next 0 7)
def repairedInvariants (next : ReferenceContinuation.Session 3 1)
    (accepted : ReferenceContinuation.replaceResidual cancelled repairProgramBlock = some next) :=
  ReferenceSourceTrace.rooted_source_invariants _ _ _ _ (repairedRooted next accepted)
#print axioms repairedInvariants

#guard (ReferenceContinuation.replaceResidual cancelled elsewhereProgramBlock).isSome
def elsewhere := (ReferenceContinuation.replaceResidual cancelled elsewhereProgramBlock).map fun next => ReferenceContinuation.run next 0 7
#guard (elsewhere.map fun next => next.program.place.val) = some 1
#guard (elsewhere.map fun next => next.status) = some (.failed .rejected)
#guard (elsewhere.map fun next => lookup next.state.values "recoveredValue") = some none

theorem begunSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy begun := .launch (program:=programBlock) (by rfl)
#print axioms begunSession
theorem pausedSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy paused :=
  ReferenceSession.drive_rooted _ _ _ _ _ _ _ begunSession
def pausedView := ReferenceSession.drive_view 9 begun 0 7
theorem lostSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy lost := by
  apply ReferenceSession.Rooted.step pausedSession
  apply ReferenceSession.Step.head (view:=lostView)
  apply ReferenceSession.head_checked
  unfold paused
  rw [pausedView]
  decide
#print axioms lostSession
theorem restoredSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy restored := by
  apply ReferenceSession.Rooted.step lostSession
  apply ReferenceSession.Step.head (view:=restoredView)
  apply ReferenceSession.head_checked
  change ReferenceAuthority.check lostView restoredView = true
  decide
#print axioms restoredSession
theorem failedSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy failed := .step restoredSession .tick
theorem cancelledSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy cancelled := .step failedSession .cancellation
theorem repairedSession (next : ReferenceContinuation.Session 3 1)
    (accepted : ReferenceContinuation.replaceResidual cancelled repairProgramBlock = some next) :
    ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy (ReferenceContinuation.run next 0 7) :=
  ReferenceSession.drive_rooted _ _ _ _ _ _ _ (.step cancelledSession (.replace accepted))
def repairedSessionInvariants (next : ReferenceContinuation.Session 3 1)
    (accepted : ReferenceContinuation.replaceResidual cancelled repairProgramBlock = some next) :=
  ReferenceSession.rooted_invariants _ _ _ _ (repairedSession next accepted)
#print axioms repairedSessionInvariants
''', extra_sources={'repairProgram': REPAIR, 'missingProgram': MISSING, 'elsewhereProgram': REPAIR.replace('recover at A','recover at B')})

check('source-cancel-needs-current-authority', SOURCE, ready + '''
def beforeCall := run initial 0 0 7 (program.take 8)
def waiting := advance beforeCall.state 0 0 7 (program[8]'(by decide))
def lost := ReferenceSourceControls.head waiting.state [80,70,40,10]
def denied := ReferenceSource.cancel lost 0 0 7
#guard denied.status = .failed .rejected
#guard denied.state.waiting = lost.waiting
#guard denied.state.writes = lost.writes
#guard denied.state.values = lost.values
#guard denied.state.nextRequest = lost.nextRequest
#guard denied.state.machine.pending = lost.machine.pending
#guard denied.state.machine.store.events = lost.machine.store.events
''')
check('source-no-cancel-for-committed-normalization-only', SOURCE, ready + '''
def beforeCall := run initial 0 0 7 (program.take 8)
def lost := ReferenceSourceControls.head beforeCall.state [40,10]
def failed := advance lost 0 0 7 (program[8]'(by decide))
#guard failed.status = .failed .rejected
#guard failed.state.nextRequest = lost.nextRequest+1
#guard failed.state.waiting = none
#guard (ReferenceSource.cancel failed.state 0 0 7).status = .failed .awaiting
#guard (ReferenceSource.cancel failed.state 0 0 7).state.machine.store.events = failed.state.machine.store.events
#guard (ReferenceSource.cancel failed.state 0 0 7).state.nextRequest = failed.state.nextRequest
''')
def offset_assertions(text):
    # Independently expected byte positions from unique source tokens. No AST
    # span or adapter offset is used to construct the expected result.
    raw = text.encode('utf-8')
    return '\n'.join('#guard (program[' + str(index) + "]'(by decide)).site.byteOffset = " + str(raw.index(token.encode('utf-8')))
                     for index, token in [(0, 'initial <-'), (4, 'route <-'), (5, 'let alias:'), (8, 'first <-'), (16, 'done <-')]) + '\n'

crlf = SOURCE.replace('\n','\r\n')
check('source-identity-crlf', crlf, ready + offset_assertions(crlf))
multibyte = SOURCE.replace('transition build', '// 日本語とλの位置確認\ntransition build')
check('source-identity-multibyte', multibyte, ready + offset_assertions(multibyte))

# This is an observed counterexample in the deliberately unrestricted head
# relation, not successful conformance to the admissible authority profile.
check('oracle-unrestricted-invocation-resurrection', SOURCE, ready + '''
def beforeCall := run initial 0 0 7 (program.take 8)
def waiting := advance beforeCall.state 0 0 7 (program[8]'(by decide))
def lost := ReferenceSourceControls.head waiting.state [10]
def restored := ReferenceSourceControls.head lost []
#guard waiting.status = .waiting
#guard (complete lost).status = .failed .rejected
#guard (complete restored).status = .ready
#guard lookup (complete restored).state.values "first" = some (.plain (.integer 10 false))
''')

check('oracle-admissible-head-fresh-evidence-recovery', SOURCE, ready + '''
def beforeCall := run initial 0 0 7 (program.take 8)
def waiting := advance beforeCall.state 0 0 7 (program[8]'(by decide))
def oldView := waiting.state.machine.store.core.system.view
def revokedView := {oldView with generation := oldView.generation+1,authority := {oldView.authority with revoked := [10]}}
#guard ReferenceAuthority.check oldView revokedView
#guard (ReferenceAuthority.install waiting.state revokedView).isSome
def lost := ReferenceSource.authorityHead waiting.state revokedView
#guard (complete lost).status = .failed .rejected
#guard (ReferenceAuthority.install lost oldView).isNone
#guard (ReferenceAuthority.install lost {oldView with generation := revokedView.generation+1}).isNone
def freshClaim := {InvocationBoundary.Controls.invocationClaim with id := 110,targets := List.range 32}
def freshView := {revokedView with generation := revokedView.generation+1,authority := {revokedView.authority with issued := freshClaim :: revokedView.authority.issued}}
#guard ReferenceAuthority.check revokedView freshView
#guard (ReferenceAuthority.install lost freshView).isSome
def recovered := ReferenceSource.authorityHead lost freshView
#guard (complete recovered).status = .failed .rejected
#guard freshView.authority.revoked.contains 10
def cancelled := ReferenceSource.cancel recovered 0 0 7
#guard cancelled.status = .ready
#guard lookup cancelled.state.values "first" = none
def reacquired := advance cancelled.state 0 0 7 (program[12]'(by decide))
#guard reacquired.status = .ready
def retried := run reacquired.state 0 0 7 [program[8]'(by decide)]
#guard retried.status = .ready
#guard lookup retried.state.values "first" = some (.plain (.integer 10 false))
-- An unrelated new generation also ends the old H epoch; this is conservative
-- invalidation, not a claim that action-14 survives all authority updates.
def unrelatedView := {oldView with generation := oldView.generation+1}
#guard ReferenceAuthority.check oldView unrelatedView
#guard (complete (ReferenceSource.authorityHead waiting.state unrelatedView)).status = .failed .rejected
''')

LATER = SOURCE.split('transition build')[0] + '''transition addLater at A requires CompositionControl {
  late <- perform instantiate(initial, ["A", "C"]) via lifecycle
  lateValue <- late(count)
}
'''
check('source-session-continue-addition', SOURCE, ready + '''
def begun : ReferenceContinuation.Session 3 1 := ⟨initial,programBlock,[],program,none,.ready,[]⟩
def built := ReferenceContinuation.run begun 0 7
#guard built.status = .ready
#guard built.remaining = []
#guard built.stopped = none
#guard built.completed = program
def following := ReferenceContinuation.continueWith built laterProgramBlock
#guard following.isSome
#guard (following.map fun next => next.superseded) = some [ReferenceContinuation.archive built]
def added := following.map fun next => ReferenceContinuation.run next 0 7
#guard (added.map fun next => next.status) = some .ready
#guard (added.map fun next => next.state.machine.store.core.system.configuration.count) = some 3
#guard (added.map fun next => lookup next.state.values "lateValue") = some (some (.plain (.integer 10 false)))
#guard (added.map fun next => next.superseded) = some [ReferenceContinuation.archive built]
theorem begunSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy begun := .launch (program:=programBlock) (by rfl)
theorem builtSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy built := ReferenceSession.drive_rooted _ _ _ _ _ _ _ begunSession
theorem continuedSession (next : ReferenceContinuation.Session 3 1)
    (accepted : ReferenceContinuation.continueWith built laterProgramBlock = some next) :
    ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy (ReferenceContinuation.run next 0 7) :=
  ReferenceSession.drive_rooted _ _ _ _ _ _ _ (.step builtSession (.continueWith accepted))
def continuedInvariants (next : ReferenceContinuation.Session 3 1)
    (accepted : ReferenceContinuation.continueWith built laterProgramBlock = some next) :=
  ReferenceSession.rooted_invariants _ _ _ _ (continuedSession next accepted)
#print axioms continuedInvariants
''', extra_sources={'laterProgram': LATER})

check('oracle-c-locus-rejoin-current-claim', SOURCE, ready + '''
def beforeCall := run initial 0 0 7 (program.take 8)
def waiting := advance beforeCall.state 0 0 7 (program[8]'(by decide))
def left := ReferenceSource.controlInput waiting.state 0 1 7 (.leave 0)
#guard left.isSome
#guard (left.map fun (state,_) => (ReferenceSource.cancel state 0 0 7).status) = some (.failed .rejected)
def rejoined := left.bind fun (state,_) => ReferenceSource.controlInput state 0 1 7 (.join 0)
#guard rejoined.isSome
#guard (rejoined.map fun (state,_) => state.machine.store.core.system.configuration.state.placeIncarnation 0) = some 1
#guard (rejoined.map fun (state,_) => state.machine.store.core.system.view.authority.issued) = some ReferenceSourceControls.authority.issued
#guard (rejoined.map fun (state,_) => (complete state).status) = some (.failed .rejected)
#guard (rejoined.map fun (state,_) => (ReferenceSource.cancel state 0 0 7).status) = some .ready
#guard (rejoined.map fun (state,_) => (ReferenceSource.cancel state 0 0 7).state.machine.pending) = some []
''')
check('oracle-c-new-member-new-claim', SOURCE, ready + '''
def beforeCall := run initial 0 0 7 (program.take 8)
def waiting := advance beforeCall.state 0 0 7 (program[8]'(by decide))
def oldView := ReferenceSourceControls.view
def memberView := {oldView with generation := oldView.generation+1,members := fun key => {(oldView.members key) with incarnation := (oldView.members key).incarnation+1}}
#guard ReferenceAuthority.check oldView memberView
def changed := ReferenceSource.authorityHead waiting.state memberView
#guard (ReferenceSource.cancel changed 0 0 7).status = .failed .rejected
def newC := {ReferenceOwnerControls.ownerClaim with id := 280,predicate := 60,actions := [15],targets := List.range 128,memberIncarnation := (memberView.members 0).incarnation}
def grantedView := {memberView with generation := memberView.generation+1,authority := {memberView.authority with issued := newC :: memberView.authority.issued}}
#guard ReferenceAuthority.check memberView grantedView
def newlyGranted := ReferenceSource.authorityHead changed grantedView
#guard (ReferenceSource.cancel newlyGranted 0 0 7).status = .ready
#guard (ReferenceSource.cancel newlyGranted 0 0 7).state.machine.pending = []
#guard lookup (ReferenceSource.cancel newlyGranted 0 0 7).state.values "first" = none
''')

check('source-cancel-expired-repair-stays-failed', SOURCE.replace('[100, 100])', '[6, 6])'), '''
#guard checked.isSome
def begun : ReferenceContinuation.Session 3 1 := ⟨initial,programBlock,[],program,none,.ready,[]⟩
def paused := ReferenceContinuation.drive 9 begun 0 7
#guard paused.status = .waiting
#guard paused.state.machine.store.core.system.serial = 5
#guard (complete paused.state).status = .ready
def cancelled := ReferenceContinuation.cancel paused 0 7
#guard cancelled.status = .failed .cancelled
#guard cancelled.state.machine.store.core.system.serial = 6
#guard cancelled.state.machine.pending = []
#guard cancelled.state.machine.store.bindings = paused.state.machine.store.bindings
def replacement := ReferenceContinuation.replaceResidual cancelled repairProgramBlock
#guard replacement.isSome
def repaired := replacement.map fun next => ReferenceContinuation.run next 0 7
#guard (repaired.map fun next => next.status) = some (.failed .rejected)
#guard (repaired.map fun next => next.stopped) = some (some (repairProgram[0]'(by decide)))
#guard (repaired.map fun next => next.remaining) = some (repairProgram.drop 1)
#guard (repaired.map fun next => next.state.values) = some cancelled.state.values
#guard (repaired.map fun next => next.state.writes) = some cancelled.state.writes
#guard (repaired.map fun next => next.state.machine.pending) = some []
#guard (repaired.map fun next => next.state.machine.store.events) = some cancelled.state.machine.store.events
#guard (repaired.map fun next => lookup next.state.values "first") = some none
#guard (repaired.map fun next => lookup next.state.values "recoveredValue") = some none
#guard (repaired.map fun next => next.superseded) = some [ReferenceContinuation.archive cancelled]
theorem begunSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy begun :=
  .launch (program:=programBlock) (by rfl)
theorem pausedSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy paused :=
  ReferenceSession.drive_rooted _ _ _ _ _ _ _ begunSession
theorem cancelledSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy cancelled :=
  .step pausedSession .cancellation
theorem repairedSession (next : ReferenceContinuation.Session 3 1)
    (accepted : ReferenceContinuation.replaceResidual cancelled repairProgramBlock = some next) :
    ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy (ReferenceContinuation.run next 0 7) :=
  ReferenceSession.drive_rooted _ _ _ _ _ _ _ (.step cancelledSession (.replace accepted))
#print axioms repairedSession
''', extra_sources={'repairProgram': REPAIR})

selected = os.environ.get('MIR_W3_CASE_FILTER')
status = 'filtered' if selected else 'passed'
if not results:
    status = 'failed-empty-filter'
summary = dict(status=status, workdir=str(WORK), cases=results,
               parser_sha256=hashlib.sha256(adapter.PARSER.read_bytes()).hexdigest(),
               adapter_sha256=hashlib.sha256(Path(adapter.__file__).read_bytes()).hexdigest(),
               case_runner_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
               filter=os.environ.get('MIR_W3_CASE_FILTER'))
(WORK / 'RESULT.json').write_text(json.dumps(summary, indent=2) + '\n')
print(json.dumps(dict(status=status, cases=len(results), workdir=str(WORK))), flush=True)
if not results:
    raise SystemExit(1)
