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
    for name, body in [('subtraction', 'return x - 1'), ('local_binding', 'let y = x\n  return y'), ('wrong_type', 'return true')]:
        src = p / (name + '.mir')
        src.write_text('module ProofFirst.Negative\n\nfn compute(x: Int64) -> Int64 {\n  ' + body + '\n}\n')
        c = invoke([checker, src, '--format', 'json'])
        report = json.loads(c['stdout'])
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

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--workdir', type=Path, required=True)
    parser.add_argument('--lean-path', type=Path, required=True)
    repo = Path(__file__).resolve().parents[1]
    parser.add_argument('--checker', type=Path, default=repo / 'target/debug/examples/full_system_v1_check')
    parser.add_argument('--runner', type=Path, default=repo / 'target/debug/examples/mir_full_system_v1_session')
    args = parser.parse_args()
    p = args.workdir.resolve()
    lean_path = args.lean_path.resolve()
    check = args.checker.resolve()
    run = args.runner.resolve()
    if not (lean_path / 'MirroreaProofFirstContractExport.olean').is_file():
        parser.error('--lean-path requires the compiled proof dependency cone')
    if not check.is_file() or not run.is_file():
        parser.error('build the existing LAB checker and runner first')
    p.mkdir(parents=False, exist_ok=False)
    cases = [('square', 'first', 'x', 'x * x + 1', -3, True), ('renamed', 'unrelated_name', 'value', 'value * value + 1', -3, True), ('shifted', 'shift', 'y', '(y + 2) * (y + 2) + 1', -3, True), ('nested', 'fourth', 'n', '(n * n) * (n * n) + 1', -2, True), ('linear', 'scale', 'z', 'z * 3 + 1', 2, False)]
    records = []
    lean = ['import MirroreaProofFirstContractExport', 'namespace ActualSourceControls', 'open MirroreaProofFirst.LocalContract MirroreaProofFirst.ContractExport', 'open MirroreaProofFirst.LocalContract.ProductNormalization']
    for name, entry, param, body, arg, symbolic in cases:
        src = p / (name + '.mir')
        src.write_text('module ProofFirst.Functions\n\nfn ' + entry + '(' + param + ': Int64) -> Int64 {\n  return ' + body + '\n}\n')
        c = invoke([check, src, '--format', 'json'])
        assert c['exit_code'] == 0, c
        ir = json.loads(c['stdout'])
        t = export(ir, entry)
        r = invoke([run, src, '--entry', entry, '--input', str(arg), '--format', 'json'])
        assert r['exit_code'] == 0, r
        actual = json.loads(r['stdout'])['runtime']
        assert actual['accepted']
        out = actual['output']['summary']
        m = re.fullmatch('Int64\\((-?\\d+)\\)', out)
        assert m, out
        z = int(m[1])
        assert z > 0
        lean += [f'def {name} : Term := {t}', f'example : CheckedArithmetic.evaluate (-9223372036854775808) 9223372036854775807 [{arg}] (normalize {name}) = some {z} := by decide', f'example : scopeCheck 1 (normalize {name}) = true := by decide', f'example : CheckedArithmetic.evaluate (-9223372036854775808) 9223372036854775807 [{arg}] (normalize {name}) ≠ some {z + 1} := by decide']
        if symbolic:
            lean += [f'example : (match normalize {name} with | .add (.square a) (.integer 1) => check [] (normalize {name}) (.add (.square a) (.integer 1)) | _ => false) = true := by decide']
        records.append(dict(name=name, source_sha256=hashlib.sha256(src.read_bytes()).hexdigest(), source_text=src.read_text(), checker=c, runtime=r, exported_term=t, actual_result=z, symbolic=symbolic))
    lean += ['end ActualSourceControls']
    f = p / 'ActualSourceControls.lean'
    f.write_text('\n'.join(lean) + '\n')
    k = subprocess.run(['lean', '--trust=0', f.name], cwd=p, env=dict(os.environ, LEAN_PATH=str(lean_path)), capture_output=True, text=True)
    negatives = negative_controls(p, check, json.loads(records[0]['checker']['stdout']))
    result = dict(negative_controls=negatives, classification='Finite real-source/checker/runtime correspondence; JSON exporter and parser/runtime are TCB, no general Rust refinement or Mir E2E', binary_hashes={str(x): hashlib.sha256(x.read_bytes()).hexdigest() for x in [check, run]}, cases=records, kernel=dict(exit_code=k.returncode, stdout=k.stdout, stderr=k.stderr))
    (p / 'ACTUAL_SOURCE_BRIDGE.json').write_text(json.dumps(result, indent=2))
    print('cases', len(records), 'kernel', k.returncode)
    print(k.stdout)
    assert k.returncode == 0
if __name__ == '__main__':
    main()
