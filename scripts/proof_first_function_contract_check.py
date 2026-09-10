"""Nonproduction finite evidence from actual LAB parser/checker/runtime, not a replacement executor."""
from pathlib import Path
import argparse, copy, json, subprocess, hashlib, re, os

def invoke(cmd):
    r = subprocess.run([str(x) for x in cmd], capture_output=True, text=True)
    return dict(command=[str(x) for x in cmd], exit_code=r.returncode, stdout=r.stdout, stderr=r.stderr)

def term(expr, param):
    if expr['ty'] != 'Int64':
        raise ValueError('non Int64')
    kind = expr['kind']
    if 'IntLiteral' in kind:
        return '(.integer (' + str(kind['IntLiteral']) + '))'
    if kind.get('Variable') == param:
        return '(.input 0)'
    if 'Binary' in kind:
        b = kind['Binary']
        op = {'Add': 'add', 'Mul': 'mul'}.get(b['op'])
        if op:
            return '(.' + op + ' ' + term(b['left'], param) + ' ' + term(b['right'], param) + ')'
    raise ValueError('outside pure add/multiply fragment')

def export(report, entry):
    if not report['accepted']:
        raise ValueError('checker rejected')
    fs = [f for f in report['module']['functions'] if f['function_name'] == entry]
    if len(fs) != 1:
        raise ValueError('entry not unique')
    f = fs[0]
    if f['parameter']['param_type'] != 'Int64' or f['output_type'] != 'Int64':
        raise ValueError('signature')
    if len(f['body']) != 1 or 'Return' not in f['body'][0]:
        raise ValueError('outside return expression')
    return term(f['body'][0]['Return']['value'], f['parameter']['name'])

def negative_controls(p, checker, valid_report):
    records = []
    for name, body in [('subtraction', 'return x - 1'), ('local_binding', 'let y: Int64 = x\n  return y'), ('wrong_type', 'return true')]:
        src = p / (name + '.mir')
        src.write_text('module ProofFirst.Negative\n\nfn compute(x: Int64) -> Int64 {\n  ' + body + '\n}\n')
        c = invoke([checker, src, '--format', 'json'])
        report = json.loads(c['stdout'])
        if name in ('subtraction', 'local_binding') and not report['accepted']:
            raise AssertionError('control must reach the checked-IR export boundary')
        try:
            export(report, 'compute')
        except ValueError as error:
            records.append(dict(name=name, source_text=src.read_text(), checker=c, rejection=str(error)))
        else:
            raise AssertionError('unsupported expression accepted')
    for name in ['unaccepted', 'missing_entry', 'forged_variable', 'wrong_type']:
        report = copy.deepcopy(valid_report)
        entry = 'first'
        if name == 'unaccepted':
            report['accepted'] = False
        if name == 'missing_entry':
            entry = 'does_not_exist'
        if name == 'forged_variable':
            report['module']['functions'][0]['body'][0]['Return']['value']['kind'] = {'Variable': 'unbound'}
        if name == 'wrong_type':
            report['module']['functions'][0]['body'][0]['Return']['value']['ty'] = 'Bool'
        try:
            export(report, entry)
        except ValueError as error:
            records.append(dict(name=name, classification='deliberate JSON mutation, not checker output', rejection=str(error)))
        else:
            raise AssertionError('invalid IR accepted')
    return records

def named_quote(name):
    return json.dumps(name, ensure_ascii=False)

def named_render_type(value):
    if value in ('int', 'nat', 'handle'):
        return '.' + value
    if isinstance(value, tuple) and len(value) == 2:
        return '(.arrow ' + named_render_type(value[0]) + ' ' + named_render_type(value[1]) + ')'
    raise ValueError('unsupported type-environment entry')

def named_type_of(ast, aliases):
    if ast == 'Int64':
        return '.int'
    if ast == 'UInt64':
        return '.nat'
    if isinstance(ast, dict) and set(ast) == {'Named'}:
        return named_render_type(aliases[ast['Named']])
    raise ValueError('outside supported types')

def named_expression(ast):
    kind = ast['kind']
    if set(kind) == {'IntLiteral'}:
        return '(.integer (' + str(kind['IntLiteral']) + '))'
    if set(kind) == {'Variable'}:
        return '(.var ' + named_quote(kind['Variable']) + ')'
    if set(kind) == {'Binary'}:
        b = kind['Binary']
        op = {'Add': 'add', 'Mul': 'mul'}[b['op']]
        return '(.' + op + ' ' + named_expression(b['left']) + ' ' + named_expression(b['right']) + ')'
    if set(kind) == {'Call'}:
        c = kind['Call']
        if len(c['arguments']) != 1:
            raise ValueError('unary call only')
        return '(.app ' + named_expression(c['callee']) + ' ' + named_expression(c['arguments'][0]) + ')'
    raise ValueError('outside pure expression fragment')

def named_body(statements, aliases):
    if len(statements) == 1 and set(statements[0]) == {'Return'}:
        return named_expression(statements[0]['Return']['value'])
    if len(statements) > 1 and set(statements[0]) == {'Let'}:
        b = statements[0]['Let']
        if b['mutable']:
            raise ValueError('mutable local outside pure bridge')
        return '(.letIn ' + named_quote(b['name']) + ' ' + named_type_of(b['ty'], aliases) + ' ' + named_expression(b['value']) + ' ' + named_body(statements[1:], aliases) + ')'
    raise ValueError('outside typed-let/return fragment')

def named_module(report, aliases, entry, arg):
    if not report['accepted']:
        raise ValueError('parser rejected')
    ast = report['module']
    if any((ast[k] for k in ['imports', 'capabilities', 'effects', 'records', 'transitions'])):
        raise ValueError('non-function module item')
    functions = []
    for item in ast['items']:
        if set(item) != {'Function'}:
            raise ValueError('non-function item')
        functions.append(item['Function'])
    names = [f['function_name'] for f in functions]
    if len(set(names)) != len(names):
        raise ValueError('duplicate module definition')
    if entry not in names:
        raise ValueError('missing entry')
    result = '(.app (.var ' + named_quote(entry) + ') (.integer (' + str(arg) + ')))'
    for f in reversed(functions):
        a = named_type_of(f['input_type'], aliases)
        b = named_type_of(f['output_type'], aliases)
        value = '(.lambda ' + named_quote(f['parameter_name']) + ' ' + a + ' ' + named_body(f['body'], aliases) + ')'
        result = '(.letIn ' + named_quote(f['function_name']) + ' (.arrow ' + a + ' ' + b + ') ' + value + ' ' + result + ')'
    return result

def named_controls(workdir, parser_binary, checker_binary, lean_path):
    base = 'module ProofFirst.HigherOrder\nfn square(x: Int64) -> Int64 { return x * x }\nfn apply(f: UnaryInt) -> Int64 { return f(4) }\nfn main(x: Int64) -> Int64 {\n  let selected: UnaryInt = square\n  return apply(selected) + x\n}\n'
    returned = 'module ProofFirst.Returned\nfn square(x: Int64) -> Int64 { return x * x }\nfn identity(f: UnaryInt) -> UnaryInt { return f }\nfn main(x: Int64) -> Int64 { return identity(square)(x) }\n'
    cases = [('base', base, {'UnaryInt': ('int', 'int')}, 'main', 19), ('renamed', base.replace('square', 'transform').replace('apply', 'consume').replace('main', 'launch').replace('UnaryInt', 'Procedure'), {'Procedure': ('int', 'int')}, 'launch', 19), ('returned', returned, {'UnaryInt': ('int', 'int')}, 'main', 9), ('wrong_result_type', base.replace('fn square(x: Int64) -> Int64', 'fn square(x: Int64) -> UInt64'), {'UnaryInt': ('int', 'int')}, 'main', None), ('wrong_type_environment', base, {'UnaryInt': 'nat'}, 'main', None), ('unbound', base.replace('apply(selected) + x', 'apply(selected) + missing'), {'UnaryInt': ('int', 'int')}, 'main', None)]
    records = []
    for name, source, aliases, entry, expected in cases:
        source_file = workdir / ('named_' + name + '.mir')
        source_file.write_text(source)
        parsed = invoke([parser_binary, source_file, '--format', 'json'])
        if parsed['exit_code'] != 0:
            raise AssertionError(parsed)
        program = named_module(json.loads(parsed['stdout']), aliases, entry, 3)
        lean = 'import MirroreaProofFirstPureHandleFunctions\nopen MirroreaProofFirst.PureHandleFunctions.NamedElaboration\ndef program : Source 0 := ' + program + '\n'
        if expected is None:
            lean += 'example : check [] program = none := by decide\n#eval (check [] program).isSome\n'
        else:
            lean += f'example : integerResult 200 program = some {expected} := by decide\n#eval integerResult 200 program\n'
        lean_file = workdir / ('Named_' + name + '.lean')
        lean_file.write_text(lean)
        command = ['lean', '--trust=0', lean_file.name]
        result = subprocess.run(command, cwd=workdir, env=dict(os.environ, LEAN_PATH=str(lean_path)), capture_output=True, text=True)
        records.append(dict(name=name, source_text=source, type_environment=aliases, parser=parsed, lean_source=lean, kernel_command=command, kernel_exit=result.returncode, kernel_stdout=result.stdout, kernel_stderr=result.stderr))
        if result.returncode != 0:
            raise AssertionError(records[-1])
    comparison = invoke([checker_binary, workdir / 'named_base.mir', '--format', 'json'])
    return dict(classification='Actual existing parser to named Lean reference; mathematical integer model, not existing runtime support or general Int64 refinement. Type-name environment is supplied, not a Core primitive or authority.', parser_sha256=hashlib.sha256(parser_binary.read_bytes()).hexdigest(), cases=records, existing_checker_comparison=comparison)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--workdir', type=Path, required=True)
    parser.add_argument('--lean-path', type=Path, required=True)
    repo = Path(__file__).resolve().parents[1]
    parser.add_argument('--checker', type=Path, default=repo / 'target/debug/examples/full_system_v1_check')
    parser.add_argument('--runner', type=Path, default=repo / 'target/debug/examples/mir_full_system_v1_session')
    parser.add_argument('--parser', type=Path, default=repo / 'target/debug/examples/textual_mir_alpha_parse')
    args = parser.parse_args()
    p = args.workdir.resolve()
    lean_path = args.lean_path.resolve()
    check = args.checker.resolve()
    run = args.runner.resolve()
    parser_binary = args.parser.resolve()
    if not (lean_path / 'MirroreaProofFirstContractExport.olean').is_file():
        parser.error('--lean-path requires the compiled proof dependency cone')
    if not check.is_file() or not run.is_file() or (not parser_binary.is_file()):
        parser.error('build the existing LAB parser, checker and runner first')
    p.mkdir(parents=False, exist_ok=False)
    cases = [('square', 'first', 'x', 'x * x + 1', -3, True), ('renamed', 'unrelated_name', 'value', 'value * value + 1', -3, True), ('shifted', 'shift', 'y', '(y + 2) * (y + 2) + 1', -3, True), ('nested', 'fourth', 'n', '(n * n) * (n * n) + 1', -2, True), ('linear', 'scale', 'z', 'z * 3 + 1', 2, False)]
    records = []
    lean = ['import MirroreaProofFirstContractExport', 'namespace ActualSourceControls', 'open MirroreaProofFirst.LocalContract MirroreaProofFirst.ContractExport', 'open MirroreaProofFirst.LocalContract.ProductNormalization']
    for name, entry, param, body, arg, symbolic in cases:
        src = p / (name + '.mir')
        src.write_text('module ProofFirst.Functions\n\nfn ' + entry + '(' + param + ': Int64) -> Int64 {\n  return ' + body + '\n}\n')
        c = invoke([check, src, '--format', 'json'])
        if not c['exit_code'] == 0:
            raise AssertionError(c)
        ir = json.loads(c['stdout'])
        t = export(ir, entry)
        r = invoke([run, src, '--entry', entry, '--input', str(arg), '--format', 'json'])
        if not r['exit_code'] == 0:
            raise AssertionError(r)
        actual = json.loads(r['stdout'])['runtime']
        if not actual['accepted']:
            raise AssertionError("validation failed: actual['accepted']")
        out = actual['output']['summary']
        m = re.fullmatch('Int64\\((-?\\d+)\\)', out)
        if not m:
            raise AssertionError(out)
        z = int(m[1])
        if not z > 0:
            raise AssertionError('validation failed: z > 0')
        lean += [f'def {name} : Term := {t}', f'example : CheckedArithmetic.evaluate (-9223372036854775808) 9223372036854775807 [{arg}] (normalize {name}) = some {z} := by decide', f'example : scopeCheck 1 (normalize {name}) = true := by decide', f'example : CheckedArithmetic.evaluate (-9223372036854775808) 9223372036854775807 [{arg}] (normalize {name}) ≠ some {z + 1} := by decide']
        if symbolic:
            lean += [f'example : (match normalize {name} with | .add (.square a) (.integer 1) => check [] (normalize {name}) (.add (.square a) (.integer 1)) | _ => false) = true := by decide']
        records.append(dict(name=name, source_sha256=hashlib.sha256(src.read_bytes()).hexdigest(), source_text=src.read_text(), checker=c, runtime=r, exported_term=t, actual_result=z, symbolic=symbolic))
    lean += ['end ActualSourceControls']
    f = p / 'ActualSourceControls.lean'
    f.write_text('\n'.join(lean) + '\n')
    k = subprocess.run(['lean', '--trust=0', f.name], cwd=p, env=dict(os.environ, LEAN_PATH=str(lean_path)), capture_output=True, text=True)
    negatives = negative_controls(p, check, json.loads(records[0]['checker']['stdout']))
    named = named_controls(p, parser_binary, check, lean_path)
    result = dict(named_reference=named, negative_controls=negatives, classification='Finite real-source/checker/runtime correspondence; JSON exporter and parser/runtime are TCB, no general Rust refinement or Mir E2E', binary_hashes={str(x): hashlib.sha256(x.read_bytes()).hexdigest() for x in [check, run]}, cases=records, kernel=dict(exit_code=k.returncode, stdout=k.stdout, stderr=k.stderr))
    (p / 'ACTUAL_SOURCE_BRIDGE.json').write_text(json.dumps(result, indent=2))
    print('existing-runtime cases', len(records), 'named-reference cases', len(named['cases']), 'kernel', k.returncode)
    print(k.stdout)
    if not k.returncode == 0:
        raise AssertionError('validation failed: k.returncode == 0')
if __name__ == '__main__':
    main()
