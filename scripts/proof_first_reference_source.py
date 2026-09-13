"""Private W3 reference-source adapter; actual Rust AST, same protected engine.

This does not declare a public grammar/provider API or W3 closure. Authority is
an explicit test-realm input to execution; source compilation issues no claims.
"""
from pathlib import Path
import copy
import hashlib
import importlib.util
import json
import os
import resource
import shutil
import tempfile
import subprocess

if not __debug__:
    raise SystemExit("optimized Python is not an evidence execution mode")

WORK = Path(__file__).resolve().parent
PARSER = Path(os.environ['MIR_PROOF_FIRST_PARSER'])
spec = importlib.util.spec_from_file_location('ordinary_source', WORK / 'proof_first_composition_source.py')
base = importlib.util.module_from_spec(spec)
spec.loader.exec_module(base)
q = base.q
EFFECTS = {
    'reference': (['InstanceCallable', 'InstanceCallables', 'Text', 'BoolList', 'IntList'], 'ReferenceCallable'),
    'reacquire': (['ReferenceCallable'], 'Unit'),
    'release': (['ReferenceCallable'], 'Unit'),
}


def text_literal(ast):
    kind = ast['kind']
    if set(kind) != {'TextLiteral'} or not isinstance(kind['TextLiteral'], str):
        raise ValueError('explicit access declaration required')
    return kind['TextLiteral']


def boolean(ast):
    kind = ast['kind']
    if set(kind) != {'BoolLiteral'} or type(kind['BoolLiteral']) is not bool:
        raise ValueError('explicit adjacent lineage annotation required')
    return kind['BoolLiteral']


def export(report, places, source_text):
    if not report['accepted'] or report['diagnostics']:
        raise ValueError('actual parser rejected')
    module = report['module']
    declarations = set()
    for effect in module['effects']:
        name = effect['effect_name']
        if name in declarations or name not in base.EFFECTS | EFFECTS:
            raise ValueError('duplicate or unknown provider operation')
        declarations.add(name)
        expected = (base.EFFECTS | EFFECTS)[name]
        actual = ([base.ty(p['param_type']) for p in effect['parameters']], base.ty(effect['output']['output_type']))
        if actual != expected or effect['required_capabilities'] != ['CompositionControl'] or effect['failure_row'] != ['Rejected']:
            raise ValueError('provider signature/capability/failure mismatch')

    # The old adapter checks the remaining module/function/capability/locus
    # profile and handles ordinary syntax only. It NEVER executes the program.
    ordinary = copy.deepcopy(report)
    ordinary['module']['effects'] = [effect for effect in module['effects'] if effect['effect_name'] in base.EFFECTS]
    ordinary['module']['items'] = [item for item in module['items']
                                   if 'Effect' not in item or item['Effect']['effect_name'] in base.EFFECTS]
    if len(module['transitions']) != 1:
        raise ValueError('one finite transition required')
    ordinary['module']['transitions'][0]['body'] = []
    _, place = base.export(ordinary, places, source_text)
    functions = {item['Function']['function_name'] for item in module['items'] if 'Function' in item}
    result = []
    for statement in module['transitions'][0]['body']:
        node = None
        if set(statement) == {'Let'} and base.ty(statement['Let']['ty']) == 'ReferenceCallable':
            binding = statement['Let']
            if binding['mutable']:
                raise ValueError('reference alias is immutable in this profile')
            node = '(.alias ' + q(binding['name']) + ' ' + q(base.named(binding['value'])) + ')'
        elif set(statement) == {'Bind'} and set(statement['Bind']['value']) == {'Perform'}:
            binding = statement['Bind']
            effect = binding['value']['Perform']
            operation, args = effect['effect_name'], effect['arguments']
            if operation in EFFECTS:
                if operation not in declarations or effect['boundary_ref'] != 'lifecycle' or binding['contract_clauses']:
                    raise ValueError('unresolved or additionally constrained reference operation')
                if len(args) != len(EFFECTS[operation][0]):
                    raise ValueError('reference operation arity mismatch')
                if operation == 'reference':
                    reader = base.named(args[0])
                    targets = [base.named(item) for item in base.items(args[1])]
                    access = text_literal(args[2])
                    edges = [boolean(item) for item in base.items(args[3])]
                    leases = [base.number(item) for item in base.items(args[4])]
                    if len(leases) != len(targets) or any(lease < 0 for lease in leases):
                        raise ValueError('each option requires a nonnegative logical deadline')
                    if len(edges) > max(0, len(targets) - 1):
                        raise ValueError('extra adjacent lineage annotation')
                    options = ['⟨' + q('option' + str(i)) + ',' + q(target) + ',some ' + q(access) + ',.read,' + str(leases[i]) + '⟩'
                               for i, target in enumerate(targets)]
                    links = ['some ⟨' + q('option' + str(i)) + ',' + q('option' + str(i + 1)) + ',' + str(edge).lower() + '⟩'
                             for i, edge in enumerate(edges)]
                    # Missing/false edges remain actual semantic checker input.
                    chain = '⟨' + q(reader) + ',[' + ','.join(options) + '],[' + ','.join(links) + ']⟩'
                    node = '(.acquire ' + q(binding['name']) + ' ' + chain + ')'
                else:
                    node = '(.' + operation + ' ' + q(binding['name']) + ' ' + q(base.named(args[0])) + ')'
        if node is None:
            ordinary['module']['transitions'][0]['body'] = [statement]
            exported, _ = base.export(ordinary, places, source_text)
            assert len(exported) == 1
            start, plain = exported[0][1:-1].split(',', 1)
            result.append('⟨⟨' + q(source_text) + ',' + start + '⟩,.plain ' + plain + '⟩')
            continue
        if binding['name'] in functions:
            raise ValueError('transition binding collides with module function')
        start = binding['span']['start']
        if type(start) is not int or not 0 <= start < len(source_text):
            raise ValueError('invalid actual parser character origin')
        start_byte = len(source_text[:start].encode('utf-8'))
        result.append('⟨⟨' + q(source_text) + ',' + str(start_byte) + '⟩,' + node + '⟩')
    return result, place


def generate(program, place, assertions, session=False):
    # Adversarial component controls deliberately use the broader source-entry
    # relation. The main demonstration consumes the checked session boundary.
    execution = '''def outcome := run initial 0 ''' + str(place) + ''' 7 program
theorem sourceRooted : ReferenceSourceTrace.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy outcome.state :=
  ReferenceSourceTrace.run_reached initial 0 ''' + str(place) + ''' 7 program
'''
    if session:
        execution = '''def begun : ReferenceContinuation.Session 3 1 := ⟨initial,programBlock,[],program,none,.ready,[]⟩
theorem admittedLaunch : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy begun :=
  .launch (program:=programBlock) (by rfl)
def finished := ReferenceContinuation.run begun 0 7
def outcome : Outcome 3 1 := ⟨finished.state,finished.status⟩
theorem admittedSession : ReferenceSession.Rooted 91 ReferenceSourceControls.view ReferenceSourceControls.policy finished :=
  ReferenceSession.drive_rooted _ _ _ _ _ _ _ admittedLaunch
def sessionMetadata := ReferenceSession.rooted_metadata _ _ _ _ admittedSession
def sessionContinuation := ReferenceSession.rooted_continuing _ _ _ _ admittedSession
def sourceRooted := ReferenceSession.rooted_source _ _ _ _ admittedSession
#print axioms admittedSession
#print axioms sessionMetadata
#print axioms sessionContinuation
'''
    return '''import MirroreaProofFirstReferenceSourceProvenance
import MirroreaProofFirstReferenceChronology
import MirroreaProofFirstReferenceContinuation
import MirroreaProofFirstReferenceSession
import MirroreaProofFirstReferenceAuthority
import MirroreaProofFirstReferenceSourceOrigins
import MirroreaProofFirstReferenceSourceControls
import MirroreaProofFirstSourceFunction
open MirroreaProofFirst
open ReferenceSourceData ReferenceSource
def program : List Located := [
''' + ',\n'.join(program) + ''']
def programBlock : ReferenceContinuation.Program 3 := ⟨''' + str(place) + ''',program⟩
-- Explicit test-realm authority is separate from parsed source and its proof.
def initial : State 3 1 := ReferenceSource.initial 91 ReferenceSourceControls.view ReferenceSourceControls.policy
def checked := checkProgram 3 [] program
''' + execution + '''
def sourceInvariants := ReferenceSourceTrace.rooted_source_invariants _ _ _ _ sourceRooted
def sourceLocators := ReferenceSourceLocators.rooted_values _ _ _ _ sourceRooted
def sourceProvenance := ReferenceSourceProvenance.rooted_invariant _ _ _ _ sourceRooted
def sourceEventOrigins := ReferenceSourceOrigins.rooted_no_fabrication _ _ _ _ sourceRooted
def bindingEventOrigins := ReferenceOrigins.rooted_origins _ _ _ _ sourceRooted
def sourceChronology := ReferenceChronology.rooted_source_history _ _ _ _ sourceRooted
''' + assertions + '''
#eval outcome.status
#eval outcome.state.values
#eval outcome.state.machine.store.events
#eval outcome.state.writes
#eval outcome.state.origins
#print axioms sourceInvariants
#print axioms sourceLocators
#print axioms sourceProvenance
#print axioms sourceChronology
#print axioms sourceEventOrigins
#print axioms bindingEventOrigins
'''


def cap():
    resource.setrlimit(resource.RLIMIT_AS, (4 * 1024**3, 4 * 1024**3))


def main():
    work = Path(tempfile.mkdtemp(prefix='run-', dir=WORK))
    (WORK / 'CURRENT_RUN').write_text(str(work) + '\n')
    source = work / 'main.mir'
    shutil.copy2(WORK / 'reference.mir', source)
    shutil.copy2(Path(__file__), work / 'proof_first_reference_source.py.frozen')
    shutil.copy2(WORK / 'proof_first_composition_source.py', work / 'ordinary_source.py.frozen')
    parsed = subprocess.run([str(PARSER), str(source), '--format', 'json'], capture_output=True, text=True)
    (work / 'ACTUAL_PARSE.json').write_text(parsed.stdout)
    (work / 'ACTUAL_PARSE.stderr').write_text(parsed.stderr)
    if parsed.returncode:
        raise RuntimeError('parser exit ' + str(parsed.returncode))
    program, place = export(json.loads(parsed.stdout), ['A', 'B', 'C'], source.read_bytes().decode('utf-8'))
    assertions = '''#guard checked.isSome
#guard outcome.status = .ready
#guard outcome.state.writes.length = program.length
#guard lookup outcome.state.values "first" = some (.plain (.integer 10 false))
#guard lookup outcome.state.values "afterExchange" = some (.plain (.integer 10 false))
#guard lookup outcome.state.values "afterReacquire" = some (.plain (.integer 11 false))
#guard lookup outcome.state.values "afterRetire" = some (.plain (.integer 10 false))
#guard outcome.state.machine.store.bindings = [none]
#guard outcome.state.machine.pending = []
'''
    lean = work / 'ActualReference.lean'
    lean.write_text(generate(program, place, assertions, session=True))
    generated_sha = hashlib.sha256(lean.read_bytes()).hexdigest()
    result = subprocess.run(['lean', '--trust=0', '-o', 'ActualReference.olean', lean.name], cwd=work,
                            env=dict(os.environ, LEAN_PATH=str(work) + os.pathsep + str(WORK)),
                            capture_output=True, text=True, preexec_fn=cap)
    (work / 'ActualReference.log').write_text(result.stdout + result.stderr)
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    receipt = dict(workdir=str(work), classification='actual Rust parser -> checked reference source -> same protected engine; component execution evidence only',
                   parser_exit=parsed.returncode, lean_exit=result.returncode, statement_count=len(program),
                   source_sha256=sha(source), parser_sha256=sha(PARSER), adapter_sha256=sha(Path(__file__)),
                   ordinary_adapter_sha256=sha(WORK / 'proof_first_composition_source.py'),
                   generated_sha256=generated_sha, log_sha256=sha(work / 'ActualReference.log'))
    (work / 'ACTUAL_REFERENCE.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt))
    if sha(lean) != generated_sha:
        raise RuntimeError('generated main source changed during execution')
    if result.returncode or 'sorryAx' in result.stdout + result.stderr:
        print(result.stdout[:5000] + result.stderr[:1000])
        raise SystemExit(result.returncode or 1)


if __name__ == '__main__':
    main()
