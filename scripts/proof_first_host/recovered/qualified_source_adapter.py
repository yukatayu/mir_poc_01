"""External LAB compiler experiment; existing Rust AST, no public API adoption.

referenceAt/instantiateAt are provisional typed library-profile names. Their
last explicit Place argument qualifies the newly produced value. No event,
request ID, receipt, or authority evidence is authored by the source programmer.
"""
from pathlib import Path
import copy
import importlib.util

ROOT = Path(__file__).parent
FROZEN = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('w3_reference', FROZEN / 'proof_first_reference_source.py')
reference = importlib.util.module_from_spec(spec)
spec.loader.exec_module(reference)
QUALIFIED = {'referenceAt': 'reference', 'instantiateAt': 'instantiate'}


def export(report, places, source_text):
    if not report['accepted'] or report['diagnostics']:
        raise ValueError('actual parser rejected')
    expected = reference.base.EFFECTS | reference.EFFECTS
    expected |= {name: (expected[old][0] + ['Place'], expected[old][1]) for name, old in QUALIFIED.items()}
    names = set()
    normalized = copy.deepcopy(report)
    effects, emitted = [], set()
    for effect in report['module']['effects']:
        name = effect['effect_name']
        if name in names or name not in expected:
            raise ValueError('duplicate/unknown typed provider operation')
        names.add(name)
        signature = ([reference.base.ty(p['param_type']) for p in effect['parameters']],
                     reference.base.ty(effect['output']['output_type']))
        if signature != expected[name] or effect['required_capabilities'] != ['CompositionControl'] or effect['failure_row'] != ['Rejected']:
            raise ValueError('qualified provider signature/capability/failure mismatch')
        lower = copy.deepcopy(effect)
        if name in QUALIFIED:
            lower['effect_name'] = QUALIFIED[name]
            lower['parameters'] = lower['parameters'][:-1]
        if lower['effect_name'] not in emitted:
            emitted.add(lower['effect_name'])
            effects.append(lower)
    if len(report['module']['transitions']) != 1:
        raise ValueError('one finite source activation required')
    # Resolve every original operation before lowered declarations can supply a
    # different spelling. In particular, declaring instantiateAt alone cannot
    # declare instantiate for unchanged calls in the source document.
    def check_original(node):
        if isinstance(node, list):
            for child in node:
                check_original(child)
        elif isinstance(node, dict):
            if set(node) == {'Perform'}:
                effect = node['Perform']
                name = effect['effect_name']
                if name not in names:
                    raise ValueError('original provider operation undeclared')
                if len(effect['arguments']) != len(expected[name][0]):
                    raise ValueError('original provider operation arity mismatch')
                if effect['boundary_ref'] != 'lifecycle':
                    raise ValueError('original provider operation boundary mismatch')
            for child in node.values():
                check_original(child)
    check_original(report['module']['transitions'][0]['body'])
    annotations = []
    for statement in normalized['module']['transitions'][0]['body']:
        at = None
        if set(statement) == {'Bind'} and set(statement['Bind']['value']) == {'Perform'}:
            effect = statement['Bind']['value']['Perform']
            name = effect['effect_name']
            if name in QUALIFIED:
                if name not in names or len(effect['arguments']) != len(expected[name][0]):
                    raise ValueError('qualified operation undeclared or wrong arity')
                at = reference.base.literal_place(effect['arguments'][-1], places)
                effect['effect_name'] = QUALIFIED[name]
                effect['arguments'] = effect['arguments'][:-1]
        annotations.append(at)
    normalized['module']['effects'] = effects
    normalized['module']['items'] = [item for item in normalized['module']['items'] if 'Effect' not in item]
    normalized['module']['items'] += [{'Effect': effect} for effect in effects]
    # Reuse W3 typing/arithmetic lowering after this explicit checked desugaring.
    # Original document, byte spans, caller locus and statements are retained.
    items, caller = reference.export(normalized, places, source_text)
    if len(items) != len(annotations):
        raise ValueError('source item/annotation correspondence lost')
    raw = ['⟨' + item + ',' + ('none' if at is None else 'some ' + str(at)) + '⟩'
           for item, at in zip(items, annotations)]
    return raw, caller


def generate(raw, caller):
    return '''import MirroreaProofFirstQualifiedSession
import MirroreaProofFirstOwnerProjection
import MirroreaProofFirstReferenceSourceControls
import MirroreaProofFirstSourceFunction
open MirroreaProofFirst
open QualifiedSource QualifiedSession ReferenceSourceData
def program : QualifiedSession.Program 3 := ⟨''' + str(caller) + ''',[
''' + ',\n'.join(raw) + ''']⟩
def begun : QualifiedSession.Session 3 1 :=
  ⟨⟨ReferenceSource.initial 91 ReferenceSourceControls.view ReferenceSourceControls.policy,[]⟩,
    0,7,program,[],none,program.items,.ready,[]⟩
theorem admittedLaunch : QualifiedSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy
    ⟨program.caller,0,7⟩ begun := .launch (by rfl) rfl
#print axioms admittedLaunch

def main : IO Unit := do
  let mut session := begun
  let mut replies := 0
  for _ in program.items do
    let before := session
    session := QualifiedSession.tick session
    unless session.status == .ready || session.status == .waiting do
      throw (IO.userError s!"actual qualified source refused: {reprStr session.status}")
    if session.status == .waiting then
      let some saved := session.state.source.waiting | throw (IO.userError "missing private pending")
      unless session.state.source.writes == before.state.source.writes &&
          (QualifiedSession.tick session).state.source.writes == session.state.source.writes &&
          session.completed == before.completed do
        throw (IO.userError "ticket preparation or waiting tick completed a source line")
      unless saved.entry.ticket.place == 2 && saved.entry.ticket.argument == 3 do
        throw (IO.userError "parsed operation coordinate or captured argument lost")
      let cancelled := QualifiedSession.cancel session
      unless cancelled.status == .failed .cancelled && cancelled.state.source.waiting.isNone &&
          cancelled.state.source.writes == session.state.source.writes && cancelled.completed == session.completed do
        throw (IO.userError "source cancellation fabricated a result or lost captured coordinate")
      let some value := OwnerProjection.serve (OwnerProjection.project session.state.source) 2 saved.entry.ticket |
        throw (IO.userError "pure owner model refused source ticket")
      let some next := QualifiedSession.receive session saved.entry.ticket value |
        throw (IO.userError "protected receiver refused")
      unless (QualifiedSession.receive next saved.entry.ticket value).isNone do
        throw (IO.userError "double receipt consumed")
      session := next
      replies := replies+1
  unless session.remaining.isEmpty && session.stopped.isNone && session.completed == program.items &&
      session.state.source.writes.length == program.items.length && replies == 4 do
    throw (IO.userError "source partition/write count mismatch")
  unless lookup session.state.source.values "first" == some (.plain (.integer 10 false)) &&
      lookup session.state.source.values "afterExchange" == some (.plain (.integer 10 false)) &&
      lookup session.state.source.values "afterReacquire" == some (.plain (.integer 11 false)) &&
      lookup session.state.source.values "afterRetire" == some (.plain (.integer 10 false)) do
    throw (IO.userError "source-derived function/fallback results changed")
  let same : QualifiedSession.Program 3 := ⟨0,[]⟩
  let moved : QualifiedSession.Program 3 := ⟨2,[]⟩
  unless (QualifiedSession.continueWith session same).isSome &&
      (QualifiedSession.continueWith session moved).isNone do
    throw (IO.userError "continuation lost same-caller custody boundary")
  IO.println "PARSED_QUALIFIED_MODEL_OK actual Rust AST -> provisional typed allocation qualifier -> checked17-line Session; caller A, C reference lifecycle,4 receipts,10/10/11/10, cancellation/continuation/duplicate controls."
  IO.println "Mathematical owner input only; no real network, authenticated custody, public library contract or W4 closure."
'''
