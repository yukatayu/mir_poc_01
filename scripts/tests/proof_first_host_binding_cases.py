"""Real Lean import/object counterexamples for the bounded host-model runner.

These check evidence custody, not a theorem, network behavior or authorization.
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import runpy
import subprocess
import sys
import tempfile


def main():
    if not __debug__:
        raise SystemExit('control assertions required')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work-root', required=True, type=Path)
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[2]
    root = args.work_root.resolve()
    assert root.is_dir() and not root.is_relative_to(repo)
    work = Path(tempfile.mkdtemp(prefix='host-binding-controls-', dir=root))
    print(work, flush=True)
    runner = repo / 'scripts/proof_first_reference_source_check.py'
    runner_digest = hashlib.sha256(runner.read_bytes()).hexdigest()
    api = runpy.run_path(str(runner))
    sha = api['sha']
    prefix = Path(subprocess.check_output(['lean', '--print-prefix'], text=True).strip()).resolve()
    compiler = prefix / 'bin/lean'
    records, controls = [], []

    def run(name, command, cwd=work, lean_path=None):
        env = {k: v for k, v in os.environ.items() if not k.startswith('PYTHON')}
        env['LEAN_PATH'] = str(lean_path or work)
        r = subprocess.run(command, cwd=cwd, env=env, capture_output=True,
                           text=True, preexec_fn=api['memory_limit'])
        output = r.stdout + r.stderr
        log = work / (name + '.log')
        log.write_text(output)
        records.append(dict(name=name, command=list(map(str, command)), exit=r.returncode,
                            log=str(log), log_sha256=sha(log)))
        return r, output

    def reject(name, action, diagnostic):
        try:
            action()
        except ValueError as error:
            assert diagnostic in str(error), (name, str(error))
            controls.append(dict(name=name, diagnostic=str(error)))
        else:
            raise AssertionError('evidence counterexample accepted: ' + name)

    source = work / 'Probe.lean'
    source.write_text('def value : Nat := 7\n')
    r, _ = run('good-build', [str(compiler), '--trust=0', '-j1', '-o', 'Probe.olean', source.name])
    assert r.returncode == 0
    obj = work / 'Probe.olean'
    good = obj.read_bytes()
    builds = {'Probe': dict(exit=0, source=str(source), source_sha256=sha(source),
                           outputs={str(obj): sha(obj)})}
    inspector = '''open Lean in
run_cmd do
  let env ← getEnv
  for name in env.header.moduleNames do
    let file ← findOLean name
    logInfo m!"BOUND_MODULE {name} {file}"
'''
    driver = work / 'Consumer.lean'
    driver.write_text('import Lean\nimport Probe\n#eval IO.println s!"VALUE {value}"\n' + inspector)
    command = [str(compiler), '--trust=0', '-j1', driver.name]
    r, output = run('actual-consumer', command)
    assert r.returncode == 0 and 'VALUE 7' in output
    # First implementation RED is retained with the actual positive calibration.
    try:
        validate = api['validate_lean_imports']
        completed = api['require_completed']
        validate(output, {'Probe'}, builds, work, prefix)
        controls.append(dict(name='actual-positive', status='accepted'))
        reject('missing-import', lambda: validate('', {'Probe'}, builds, work, prefix), 'import inventory')
        local = next(line for line in output.splitlines() if line.startswith('BOUND_MODULE Probe '))
        reject('duplicate-import', lambda: validate(output + '\n' + local, {'Probe'}, builds, work, prefix), 'duplicate')
        reject('missing-required-local', lambda: validate(output, {'Probe', 'Missing'}, builds, work, prefix), 'local import inventory')

        alternate = work / 'alternate'
        alternate.mkdir()
        (alternate / source.name).write_text('def value : Nat := 9\n')
        r, _ = run('alternate-build', [str(compiler), '--trust=0', '-j1', '-o', 'Probe.olean', source.name], alternate)
        assert r.returncode == 0
        obj.write_bytes((alternate / obj.name).read_bytes())
        r, changed = run('stale-compatible-object', command)
        assert r.returncode == 0 and 'VALUE 9' in changed and sha(source) == builds['Probe']['source_sha256']
        reject('stale-compatible-object', lambda: validate(changed, {'Probe'}, builds, work, prefix), 'generated input changed')
        obj.write_bytes(good)
        validate(output, {'Probe'}, builds, work, prefix)

        # Same bytes and result, but an unselected dependency origin.
        (alternate / obj.name).write_bytes(good)
        r, foreign = run('foreign-origin', command, lean_path=alternate)
        assert r.returncode == 0 and 'VALUE 7' in foreign
        reject('foreign-origin', lambda: validate(foreign, {'Probe'}, builds, work, prefix), 'import path')
        source.write_text('def value : Nat := 8\n')
        reject('source-object-mismatch', lambda: validate(output, {'Probe'}, builds, work, prefix), 'generated input changed')
        source.write_text('def value : Nat := 7\n')

        failed = work / 'LateFailure.lean'
        failed.write_text('import Lean\n#eval IO.println "SUCCESS_MARKER"\nexample : False := by decide\n')
        r, late = run('late-failure', [str(compiler), '--trust=0', '-j1', failed.name])
        assert r.returncode != 0 and 'SUCCESS_MARKER' in late
        reject('late-failure', lambda: completed(r, 'late-failure'), 'failed command')
        r, text = run('optimized-runner', [sys.executable, '-E', '-B', '-O', str(runner), '--help'])
        assert r.returncode != 0 and 'optimized Python' in text
        controls.append(dict(name='optimized-runner', status='rejected'))
        status = 'passed'
    except BaseException as error:
        status = 'failed'
        raise
    finally:
        stable_runner = sha(runner) == runner_digest
        receipt = dict(status=status if stable_runner else 'failed', scope=__doc__,
                       runner_sha256=runner_digest, runner_unchanged=stable_runner,
                       compiler_sha256=sha(compiler), records=records, controls=controls)
        (work / 'RESULT.json').write_text(json.dumps(receipt, indent=2) + '\n')
        assert stable_runner, 'runner changed during controls'
    print(json.dumps(receipt), flush=True)


if __name__ == '__main__':
    main()
