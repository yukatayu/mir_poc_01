"""External nonproduction AST adapter; no source or runtime acceptance claim."""
from pathlib import Path
import hashlib
import json
import os
import subprocess
import resource

WORK = Path(__file__).resolve().parent
PARSER = Path(os.environ['MIR_PROOF_FIRST_PARSER'])
q = lambda value: json.dumps(value, ensure_ascii=False)

EFFECTS = {
    'register': (['UnaryInt', 'IntList', 'Int64', 'Int64'], 'DefinitionId'),
    'extend': (['DefinitionId', 'UnaryInt', 'IntList', 'Int64', 'Int64'], 'DefinitionId'),
    'instantiate': (['DefinitionId', 'Placements'], 'InstanceCallable'),
    'retire': (['InstanceCallable'], 'Unit'),
    'reparent': (['InstanceCallable', 'InstanceCallable'], 'Unit'),
    'exchange': (['InstanceCallable', 'DefinitionId'], 'Unit'),
    'leave': (['Place'], 'Unit'),
    'join': (['Place'], 'Unit'),
}

def named(ast):
    k = ast['kind']
    if set(k) != {'Variable'}:
        raise ValueError('a named definition/instance is required')
    return k['Variable']

def number(ast):
    k = ast['kind']
    if set(k) == {'IntLiteral'} and type(k['IntLiteral']) is int:
        result = k['IntLiteral']
    elif set(k) == {'Unary'} and k['Unary']['op'] == 'Negate':
        result = -number(k['Unary']['expr'])
    else:
        raise ValueError('explicit integer annotation required')
    if not -(1 << 63) <= result < (1 << 63):
        raise ValueError('integer annotation is outside Int64')
    return result

def items(ast):
    k = ast['kind']
    if set(k) != {'ArrayLiteral'}:
        raise ValueError('explicit list annotation required')
    return k['ArrayLiteral']

def literal_place(ast, places):
    k = ast['kind']
    if set(k) != {'TextLiteral'} or k['TextLiteral'] not in places:
        raise ValueError('unknown represented locus')
    return places.index(k['TextLiteral'])

def arithmetic(ast, variables, source=False):
    k = ast['kind']
    if set(k) == {'IntLiteral'}:
        return '(.integer (' + str(number(ast)) + '))'
    if set(k) == {'Variable'}:
        if source:
            return '(.read ' + q(k['Variable']) + ')'
        if k['Variable'] not in variables:
            raise ValueError('unresolved lexical name in pure function')
        return variables[k['Variable']]
    if set(k) == {'Unary'} and k['Unary']['op'] == 'Negate':
        return '(.mul (.integer (-1)) ' + arithmetic(k['Unary']['expr'], variables, source) + ')'
    if set(k) == {'Binary'}:
        b = k['Binary']
        op = {'Add': 'add', 'Mul': 'mul'}[b['op']]
        return '(.' + op + ' ' + arithmetic(b['left'], variables, source) + ' ' + arithmetic(b['right'], variables, source) + ')'
    raise ValueError('outside the declared bounded arithmetic source profile')

def function(f):
    if f['input_type'] != 'Int64' or f['output_type'] != 'Int64':
        raise ValueError('selected profile requires Int64 -> Int64')
    names = {f['parameter_name']}
    bindings = []
    body = f['body']
    for stmt in body[:-1]:
        if set(stmt) != {'Let'}:
            raise ValueError('only immutable typed arithmetic lets in unary definitions')
        b = stmt['Let']
        if b['mutable'] or b['ty'] != 'Int64' or b['name'] in names:
            raise ValueError('definition local type/mutability/shadowing error')
        bindings.append((b['name'], arithmetic(b['value'], {n: '(.read ' + q(n) + ')' for n in names})))
        names.add(b['name'])
    if not body or set(body[-1]) != {'Return'}:
        raise ValueError('definition requires an explicit return')
    result = '(.result ' + arithmetic(body[-1]['Return']['value'], {n: '(.read ' + q(n) + ')' for n in names}) + ')'
    for name, expression in reversed(bindings):
        result = '(.letValue ' + q(name) + ' ' + expression + ' ' + result + ')'
    return '(SourceFunction.compile ' + q(f['parameter_name']) + ' ' + result + ')'

def ty(value):
    return value if isinstance(value, str) else value['Named']

def export(report, places, source_text):
    if not isinstance(source_text, str):
        raise ValueError('exact source text is required for UTF-8 origins')
    if not report['accepted'] or report['diagnostics']:
        raise ValueError('actual parser rejected')
    m = report['module']
    if m['imports'] or m['records'] or len(m['transitions']) != 1:
        raise ValueError('selected source profile requires one resolved transition')
    if len(set(places)) != len(places):
        raise ValueError('ambiguous locus names')
    fs = {}
    for item in m['items']:
        if set(item) == {'Function'}:
            f = item['Function']
            if f['function_name'] in fs:
                raise ValueError('duplicate function')
            fs[f['function_name']] = function(f)
        elif set(item) not in ({'Effect'}, {'Capability'}, {'Transition'}):
            raise ValueError('unsupported module item')
    declared = set()
    for e in m['effects']:
        name = e['effect_name']
        if name in declared or name not in EFFECTS:
            raise ValueError('duplicate/unknown typed lifecycle provider operation')
        expected = EFFECTS[name]
        if ([ty(p['param_type']) for p in e['parameters']], ty(e['output']['output_type'])) != expected:
            raise ValueError('provider declaration signature mismatch')
        if e['required_capabilities'] != ['CompositionControl'] or e['failure_row'] != ['Rejected']:
            raise ValueError('provider declaration requires explicit capability/failure contract')
        declared.add(name)
    if [c['capability_name'] for c in m['capabilities']] != ['CompositionControl']:
        raise ValueError('missing or ambiguous capability declaration')
    transition = m['transitions'][0]
    if transition['required_capabilities'] != ['CompositionControl']:
        raise ValueError('transition capability requirement missing')
    if transition['place_ref'] not in places:
        raise ValueError('transition locus is not represented')
    result = []
    for statement in transition['body']:
        if set(statement) == {'Let'}:
            b = statement['Let']
            if b['ty'] != 'Int64':
                raise ValueError('local must be explicitly Int64 in this profile')
            node = '(.localValue ' + q(b['name']) + ' ' + str(b['mutable']).lower() + ' ' + arithmetic(b['value'], {}, True) + ')'
        elif set(statement) == {'Assign'}:
            b = statement['Assign']
            node = '(.assign ' + q(b['name']) + ' ' + arithmetic(b['value'], {}, True) + ')'
        elif set(statement) == {'Bind'}:
            b = statement['Bind']
            if b['contract_clauses']:
                raise ValueError('unimplemented additional contract clause')
            name = q(b['name'])
            if set(b['value']) == {'Perform'}:
                e = b['value']['Perform']
                op, args = e['effect_name'], e['arguments']
                if op not in declared or e['boundary_ref'] != 'lifecycle' or len(args) != len(EFFECTS[op][0]):
                    raise ValueError('unresolved typed provider call')
                if op in ('register', 'extend'):
                    pred = 'none'
                    if op == 'extend':
                        pred = '(some ' + q(named(args[0])) + ')'
                        args = args[1:]
                    code = fs[named(args[0])]
                    domain = '[' + ','.join(str(number(x)) for x in items(args[1])) + ']'
                    contract = '⟨' + domain + ',' + str(number(args[2])) + ',' + str(number(args[3])) + '⟩'
                    node = '(.register ' + name + ' ⟨' + code + ',' + contract + '⟩ ' + pred + ')'
                elif op == 'instantiate':
                    loci = '[' + ','.join(str(literal_place(x, places)) for x in items(args[1])) + ']'
                    node = '(.instantiate ' + name + ' ' + q(named(args[0])) + ' ' + loci + ' none .top)'
                elif op == 'retire':
                    node = '(.retire ' + name + ' ' + q(named(args[0])) + ')'
                elif op == 'reparent':
                    node = '(.reparent ' + name + ' ' + q(named(args[0])) + ' (some ' + q(named(args[1])) + '))'
                elif op == 'exchange':
                    node = '(.replace ' + name + ' ' + q(named(args[0])) + ' ' + q(named(args[1])) + ')'
                else:
                    node = '(.' + op + ' ' + name + ' ' + str(literal_place(args[0], places)) + ')'
            else:
                call = b['value']['Expr']['kind']['Call']
                if len(call['arguments']) != 1:
                    raise ValueError('unary instance invocation required')
                node = '(.invoke ' + name + ' ' + q(named(call['callee'])) + ' ' + arithmetic(call['arguments'][0], {}, True) + ')'
        else:
            raise ValueError('unsupported statement; no silent skip')
        if b['name'] in fs:
            raise ValueError('transition binding collides with a module function')
        start = b['span']['start']
        if type(start) is not int or not 0 <= start < len(source_text):
            raise ValueError('invalid parser character origin')
        start_byte = len(source_text[:start].encode('utf-8'))
        result.append('⟨' + str(start_byte) + ',' + node + '⟩')
    return result, places.index(transition['place_ref'])

def memory_limit():
    resource.setrlimit(resource.RLIMIT_AS, (4*1024**3,4*1024**3))

def main():
    source = WORK / 'main.mir'
    parsed = subprocess.run([str(PARSER), str(source), '--format', 'json'], capture_output=True, text=True)
    (WORK / 'COMPOSITION_PARSE.json').write_text(parsed.stdout)
    (WORK / 'COMPOSITION_PARSE.stderr').write_text(parsed.stderr)
    if parsed.returncode:
        raise RuntimeError('parser exit ' + str(parsed.returncode))
    program, place = export(json.loads(parsed.stdout), ['A', 'B', 'C'], source.read_text())
    lean = generate(program, place)
    f = WORK / 'ActualComposition.lean'
    f.write_text(lean)
    cmd = ['lean', '--trust=0', f.name]
    checked = subprocess.run(cmd, cwd=WORK, env={**os.environ, 'LEAN_PATH': str(WORK)}, capture_output=True, text=True, preexec_fn=memory_limit)
    (WORK / 'ActualComposition.log').write_text(checked.stdout + checked.stderr)
    record = {'classification': 'Actual Rust parser -> external AST adapter -> checked Lean source execution with actual writes and retained rejection prefix; full W3 remains open.',
              'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(), 'parser_sha256': hashlib.sha256(PARSER.read_bytes()).hexdigest(),
              'adapter_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(), 'generated_sha256': hashlib.sha256(f.read_bytes()).hexdigest(),
              'parser_exit': parsed.returncode, 'lean_command': cmd, 'lean_exit': checked.returncode, 'statement_count': len(program)}
    (WORK / 'ACTUAL_COMPOSITION.json').write_text(json.dumps(record, indent=2)+'\n')
    print(json.dumps(record))
    print(checked.stdout[:1800] if checked.returncode else 'Actual values, source writes and machine trace saved to ActualComposition.log')
    if checked.returncode:
        raise SystemExit(checked.returncode)

def generate(program, place, assertions=None):
    lean = '''import MirroreaProofFirstSourceAuthoring
import MirroreaProofFirstSourceFunction
import MirroreaProofFirstSourceTypes
import MirroreaProofFirstSourceExecution
import MirroreaProofFirstSourceWriteHistory
import MirroreaProofFirstSourceAllocation
import MirroreaProofFirstSourceReadNames
import MirroreaProofFirstSourceHistory
import MirroreaProofFirstSourceLocators
open MirroreaProofFirst
open SourceAuthoring
def program : List Located := [
''' + ',\n'.join(program) + ''']
def authority : CurrentUse.Authority :=
  {ManagementEntry.Controls.view.authority with
    issued := [ManagementEntry.Controls.claim,
      {InvocationBoundary.Controls.invocationClaim with targets := [5,6,7,9,10,11]}]}
def initial : State 3 1 :=
  ⟨⟨{ManagementEntry.Controls.initial with
    view := {ManagementEntry.Controls.view with
      authority := authority
      policies := fun _ _ => InvocationBoundary.Controls.invocationPolicy}},[],[]⟩,[],0,[]⟩
def checked := SourceTypes.checkProgram 3 [] program
def outcome := SourceExecution.run ⟨initial,[]⟩ 0 ''' + str(place) + ''' 7 program
''' + '''theorem initialMachineValid : SourcePreservation.Invariant initial := by
  refine ⟨⟨(CatalogHistory.empty_valid initial.machine.system.configuration.state).toValid,?_⟩,?_,?_⟩
  · simp [initial,ManagementEntry.Controls.initial]
  · simp [initial]
  · intro t member; cases member
def actualMachineEvidence := SourceExecution.run_preserves ⟨initial,[]⟩ 0 ''' + str(place) + ''' 7 program initialMachineValid
def actualCatalogEvidence := SourceHistory.execution_catalog ⟨initial,[]⟩ 0 ''' + str(place) + ''' 7 program
  (CatalogHistory.empty_valid initial.machine.system.configuration.state)
def actualReadEvidence := SourceReadNames.run_present ⟨initial,[]⟩ 0 ''' + str(place) + ''' 7 program
  (by intro w member; cases member)
example : SourceWriteHistory.Invariant outcome.final :=
  SourceWriteHistory.run_preserves ⟨initial,[]⟩ 0 ''' + str(place) + ''' 7 program (SourceWriteHistory.empty initial rfl)
def actualLocatorEvidence :=
  SourceLocators.execution_preserves ⟨initial,[]⟩ 0 ''' + str(place) + ''' 7 program
    ⟨initialMachineValid,by intro row member; cases member⟩
def actualAllocationEvidence :=
  SourceAllocation.execution_preserves ⟨initial,[]⟩ 0 ''' + str(place) + ''' 7 program (SourceAllocation.empty initial rfl rfl)
''' + (assertions if assertions is not None else '''#guard checked.isSome
#guard outcome.rejected.isNone
#guard outcome.final.writes.length = program.length
#eval (outcome.final.source.values,outcome.final.source.machine.system.configuration.definitions,
  outcome.final.source.machine.system.configuration.count,outcome.final.source.machine.system.used,outcome.final.source.origins)
#eval outcome.final.source.machine.events
#eval outcome.final.writes
''')
    return lean

if __name__ == '__main__':
    main()
