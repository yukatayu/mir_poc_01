"""Actual runner import/cache controls; not semantic or physical E2E evidence."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import py_compile
import runpy
import subprocess
import sys
import tempfile


def main():
    if not __debug__:
        raise SystemExit('control assertions required')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work-root', type=Path, required=True)
    parser.add_argument('--runner', type=Path, default=Path(__file__).resolve().parents[1] / 'proof_first_reference_source_check.py')
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[2]
    root = args.work_root.resolve()
    assert root.is_dir() and not root.is_relative_to(repo)
    work = Path(tempfile.mkdtemp(prefix='host-runner-controls-', dir=root))
    helper = work / 'proof_first_composition_source_check.py'
    original = (repo / 'scripts' / helper.name).read_bytes()
    assert b'AXIOM_AUDIT_OK' in original
    poisoned = original.replace(b'AXIOM_AUDIT_OK', b'AXIOM_AUDIT_NO')
    assert len(poisoned) == len(original)
    helper.write_bytes(poisoned)
    stamp = helper.stat()
    py_compile.compile(str(helper), doraise=True,
                       invalidation_mode=py_compile.PycInvalidationMode.TIMESTAMP)
    helper.write_bytes(original)
    os.utime(helper, ns=(stamp.st_atime_ns, stamp.st_mtime_ns))
    runner = work / 'proof_first_reference_source_check.py'
    runner.write_bytes(args.runner.read_bytes())
    env = {k: v for k, v in os.environ.items() if not k.startswith('PYTHON')}
    records = []

    def run(label, module):
        command = [sys.executable, '-E', '-B', '-c',
                   'import ' + module + ' as r; print(r.audit_source(["Probe"]))']
        result = subprocess.run(command, cwd=work, env=env, capture_output=True, text=True)
        (work / (label + '.log')).write_text(result.stdout + result.stderr)
        records.append(dict(label=label, command=command, exit=result.returncode))
        assert result.returncode == 0, label
        return result.stdout

    # Calibrate that the timestamp/length-compatible cache is actually consumed
    # by a normal import even with -B, which prevents writes but not reads.
    baseline = run('ordinary-import', 'proof_first_composition_source_check')
    assert 'AXIOM_AUDIT_NO' in baseline and 'AXIOM_AUDIT_OK' not in baseline
    actual = run('runner-import', 'proof_first_reference_source_check')
    passed = 'AXIOM_AUDIT_OK' in actual and 'AXIOM_AUDIT_NO' not in actual
    # Two truthful routes to the installed compiler. A stable relative PATH
    # chooses different routes solely because the child changes directory.
    compiler = Path(subprocess.check_output(['lean', '--print-prefix'], text=True).strip()) / 'bin/lean'
    launched = work / 'selected-route'
    first, second, child_cwd = work / 'tools', work / 'alternate', work / 'child'
    for directory in [first, second, child_cwd]:
        directory.mkdir()
    import shlex
    for directory, label in [(first, 'A'), (second, 'B')]:
        wrapper = directory / 'lean'
        wrapper.write_text('#!/bin/sh\nprintf ' + shlex.quote(label) + ' >> ' + shlex.quote(str(launched))
                           + '\nexec ' + shlex.quote(str(compiler)) + ' "$@"\n')
        wrapper.chmod(0o700)
    relative_env = dict(env, PATH='./tools:' + str(second) + ':/usr/bin:/bin')
    for cwd in [work, child_cwd]:
        child = subprocess.run(['lean', '--version'], cwd=cwd, env=relative_env, capture_output=True, text=True)
        assert child.returncode == 0 and 'version 4.29.1,' in child.stdout
    assert launched.read_text() == 'AB', 'relative PATH counterexample did not select different routes'
    api = runpy.run_path(str(runner))
    assert 'tool_environment' in api, 'nested absolute compiler/environment selection missing'
    selected = api['tool_environment'](dict(relative_env, LEAN_CC='foreign', LEAN_SYSROOT='foreign'), compiler)
    assert selected['MIR_PROOF_FIRST_LEAN'] == str(compiler.resolve())
    assert selected['LEAN_SYSROOT'] == str(compiler.resolve().parent.parent) and 'LEAN_CC' not in selected
    child = subprocess.run([selected['MIR_PROOF_FIRST_LEAN'], '--version'], cwd=child_cwd,
                           env=selected, capture_output=True, text=True)
    assert child.returncode == 0 and 'version 4.29.1,' in child.stdout
    assert launched.read_text() == 'AB', 'selected child still went through relative PATH'
    # Exercise discovery itself with interfering inherited variables. Only the
    # installed compiler is available; this is not a two-version counterexample.
    chosen, checked_version = api['select_compiler'](
        dict(env, PATH=str(first) + ':/usr/bin:/bin', LEAN_SYSROOT='foreign', LEAN_CC='foreign'))
    assert chosen == compiler.resolve() and 'version 4.29.1,' in checked_version
    assert launched.read_text() == 'ABA', 'version query did not bypass the discovery route'
    records.append(dict(label='actual selected compiler bootstrap/version under normalized environment', exit=0))
    records.append(dict(label='actual relative PATH counterexample and selected absolute compiler', exit=0))
    receipt = dict(scope=__doc__, workdir=str(work), status='passed' if passed else 'failed',
                   counterexample_calibrated=True, source_sha256=hashlib.sha256(original).hexdigest(),
                   runner_sha256=hashlib.sha256(runner.read_bytes()).hexdigest(), records=records)
    (work / 'RESULT.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt), flush=True)
    if not passed:
        raise AssertionError('runner loaded stale audit code despite matching source bytes')


if __name__ == '__main__':
    main()
