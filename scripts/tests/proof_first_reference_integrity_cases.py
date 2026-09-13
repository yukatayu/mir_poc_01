"""Integrity/consumer negatives using copies of actual generated execution files.

Not semantic proofs or E2E results. No original evidence file is modified.
"""
from pathlib import Path
import argparse
import importlib.util
import json
import os
import subprocess
import sys
import tempfile

here = Path(__file__).resolve().parent
sys.path.insert(0, str(here if (here / 'proof_first_reference_source_check.py').exists()
                       else here.parent))
from proof_first_reference_source_check import freeze_consumer, sha, verify_integrity, memory_limit


def main():
    if not __debug__:
        raise SystemExit('optimized Python is not an evidence execution mode')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--evidence-dir', type=Path, required=True)
    args = parser.parse_args()
    evidence = args.evidence_dir.resolve()
    work = Path(tempfile.mkdtemp(prefix='integrity-controls-', dir=evidence))
    cases = Path((evidence / 'CURRENT_CASES').read_text().strip())
    rows = {r['name']: r for r in json.loads((cases / 'RESULT.json').read_text())['cases']}
    original_main = Path((evidence / 'CURRENT_RUN').read_text().strip())
    main_receipt = json.loads((original_main / 'ACTUAL_REFERENCE.json').read_text())
    originals = [('main', original_main / 'ActualReference.lean', main_receipt['generated_sha256'], None),
                 ('recovery', cases / 'source-cancel-recheck-second-source/ActualCase.lean',
                  rows['source-cancel-recheck-second-source']['generated_sha256'], 'ActualRecovery'),
                 ('addition', cases / 'source-session-continue-addition/ActualCase.lean',
                  rows['source-session-continue-addition']['generated_sha256'], 'ActualAddition')]
    results = []

    def rejected(name, action):
        try:
            action()
        except ValueError as error:
            if 'generated input changed:' not in str(error):
                raise
            results.append(dict(name=name, status='rejected', diagnostic=str(error)))
        else:
            raise RuntimeError('integrity control accepted: ' + name)

    for name, original, expected, namespace in originals:
        source, target = work / (name + '.lean'), work / (name + '-frozen.lean')
        source.write_bytes(original.read_bytes())
        frozen = freeze_consumer(source, expected, target, namespace)
        verify_integrity({str(source): expected, str(target): frozen})
        source.write_text('def placeholder : Nat := 0\n')
        rejected(name + '-receipt-handoff', lambda: freeze_consumer(source, expected, target, namespace))
        if sha(target) != frozen:
            raise RuntimeError('rejected handoff modified target')
        rejected(name + '-original-drift', lambda: verify_integrity({str(source): expected}))
        target.write_text('def placeholder : Nat := 0\n')
        rejected(name + '-frozen-drift', lambda: verify_integrity({str(target): frozen}))
    for name in ['MANIFEST.json', 'CompleteAudit.lean']:
        original = evidence / name
        target = work / name
        target.write_bytes(original.read_bytes())
        expected = sha(target)
        verify_integrity({str(target): expected})
        target.write_bytes(target.read_bytes() + b'\n')
        rejected(name + '-drift', lambda: verify_integrity({str(target): expected}))

    # Actual parser -> existing structural generator, then deliberately choose
    # the broad branch. The consumer contract must reject missing Session proofs.
    os.environ['MIR_PROOF_FIRST_PARSER'] = str(evidence / 'cargo-target/debug/examples/textual_mir_alpha_parse')
    spec = importlib.util.spec_from_file_location('integrity_adapter', evidence / 'proof_first_reference_source.py')
    adapter = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(adapter)
    source = (original_main / 'main.mir').read_bytes().decode('utf-8')
    parsed = json.loads((original_main / 'ACTUAL_PARSE.json').read_text())
    program, place = adapter.export(parsed, ['A', 'B', 'C'], source)
    component = work / 'component'
    component.mkdir()
    (component / 'ActualReference.lean').write_text(adapter.generate(
        program, place, '#guard checked.isSome\n#guard outcome.status = .ready\n', session=False))
    env = dict(os.environ, LEAN_PATH=str(component) + os.pathsep + str(evidence))

    def lean(cwd, args):
        result = subprocess.run(['lean', '--trust=0'] + args, cwd=cwd, env=env,
                                capture_output=True, text=True, preexec_fn=memory_limit)
        (cwd / (args[-1] + '.log')).write_text(result.stdout + result.stderr)
        return result

    compiled = lean(component, ['-o', 'ActualReference.olean', 'ActualReference.lean'])
    if compiled.returncode or 'sorryAx' in compiled.stdout + compiled.stderr:
        raise RuntimeError('component control failed before consumer-contract check')
    (component / 'ConsumerContract.lean').write_text(
        'import ActualReference\n#check admittedLaunch\n#check admittedSession\n')
    checked = lean(component, ['ConsumerContract.lean'])
    if checked.returncode == 0 or 'Unknown identifier' not in checked.stdout or 'admittedSession' not in checked.stdout:
        raise RuntimeError('missing Session consumer witness did not fail as intended')
    results.append(dict(name='component-default-contract', status='rejected', lean_exit=checked.returncode))

    # The same actual parsed type error accepted as a negative by the component
    # runner must fail the actual fresh Session launch witness.
    invalid = cases / 'reference-as-integer'
    invalid_text = (invalid / 'input.mir').read_bytes().decode('utf-8')
    program, place = adapter.export(json.loads((invalid / 'PARSE.json').read_text()),
                                    ['A', 'B', 'C'], invalid_text)
    invalid_code = adapter.generate(program, place, '', session=True)
    (component / 'InvalidSession.lean').write_text(invalid_code)
    checked = lean(component, ['InvalidSession.lean'])
    launch_line = next(i for i, line in enumerate(invalid_code.splitlines(), 1)
                       if '.launch (program:=' in line)
    if checked.returncode == 0 or ('InvalidSession.lean:' + str(launch_line) + ':') not in checked.stdout:
        raise RuntimeError('invalid source did not fail at Session launch witness')
    results.append(dict(name='invalid-session-launch', status='rejected', lean_exit=checked.returncode))
    receipt = dict(status='passed', classification='actual generated copies; integrity/consumer negative controls only',
                   workdir=str(work), controls=results)
    (work / 'RESULT.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps(receipt), flush=True)


if __name__ == '__main__':
    main()
