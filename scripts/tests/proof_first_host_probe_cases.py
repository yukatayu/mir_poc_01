"""Real omitted/duplicated retirement probes using explicitly selected native workers.

These are bounded harness mutations, not a current full-workflow pass or general
all-entry theorem. The selected preparation may be historical; its bound staged
inputs and original executable identities are retained, never relabelled current.
"""
from pathlib import Path
import argparse
import datetime
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
    parser.add_argument('--prepared', required=True, type=Path)
    parser.add_argument('--wire-only', action='store_true', help='run only the added wire discriminator')
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[2]
    root, prepared = args.work_root.resolve(), args.prepared.resolve()
    assert root.is_dir() and not root.is_relative_to(repo)
    source = json.loads((prepared / 'RESULT.json').read_text())
    assert source['status'] == 'prepared; physical capture/replay NOT RUN'
    sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
    api = runpy.run_path(str(repo / 'scripts/proof_first_host_run.py'))
    model = runpy.run_path(str(Path(source['model']) / 'proof_first_reference_source_check.py'))
    work = Path(tempfile.mkdtemp(prefix='host-probe-controls-', dir=root))
    print(work, flush=True)
    record = dict(scope=__doc__, preparation=str(prepared),
                  preparation_sha256=sha(prepared / 'RESULT.json'), cases=[])
    original = "probes=[wrapped_source.invoke_pending,lambda:wrapped_source.send(b''),*[lambda owner=owner:owner.send(b'') for owner in wrapped_owners]]"
    mutants = {
        'omit': original.replace('in wrapped_owners]', 'in wrapped_owners[:-1]]'),
        'duplicate': original.replace('in wrapped_owners]', 'in [wrapped_owners[0],wrapped_owners[1],wrapped_owners[1]]]'),
    }
    try:
        for name, replacement in ([] if args.wire_only else mutants.items()):
            host = work / name
            api['stage_host_inputs'](prepared, host, source['bindings'], model['verified_bytes'])
            for binary in source['native_binaries'].values():
                assert sha(Path(binary['path'])) == binary['sha256']
            harness = host / 'run_lease_known_fault_capture.py'
            before = sha(harness)
            text = harness.read_text()
            assert text.count(original) == 1
            harness.write_text(text.replace(original, replacement))
            cache = work / (name + '-cache')
            cache.mkdir()
            env = {k: v for k, v in os.environ.items() if not k.startswith('PYTHON')}
            env.update(PYTHONDONTWRITEBYTECODE='1', PYTHONPYCACHEPREFIX=str(cache))
            prefix = [sys.executable, '-E', '-B', '-X', 'pycache_prefix=' + str(cache)]
            capture = subprocess.run([*prefix, str(harness), 'UNNOTIFIED'], cwd=host, env=env,
                                     capture_output=True, text=True, preexec_fn=model['memory_limit'])
            (work / (name + '-capture.log')).write_text(capture.stdout + capture.stderr)
            assert capture.returncode == 0, capture.stderr
            raw_path = host / 'recovered/HOST_LEASE_KNOWN_FAULT_UNNOTIFIED.json'
            raw = json.loads(raw_path.read_text())
            probes = [r for r in raw['public_spans'] if r.get('message') == 'cohort retired']
            identities = [(r['kind'], r['endpoint']) for r in probes]
            expected = [('invoke', None), ('source', None), ('owner', 0), ('owner', 1), ('owner', 2)]
            assert identities != expected
            assert len(probes) == (4 if name == 'omit' else 5)
            journal = subprocess.run([*prefix, str(host / 'prepare_lease_fault_writer_journal.py'),
                                      'LEASEFINAL_UNNOTIFIED', '--capture', 'HOST_LEASE_KNOWN_FAULT_UNNOTIFIED'],
                                     cwd=host, env=env, capture_output=True, text=True,
                                     preexec_fn=model['memory_limit'])
            (work / (name + '-journal.log')).write_text(journal.stdout + journal.stderr)
            assert journal.returncode == 0, journal.stderr
            check = subprocess.run([*prefix, str(host / 'prepare_lease_fault_replay.py'),
                                    'LEASEFINAL_UNNOTIFIED', '--capture', 'HOST_LEASE_KNOWN_FAULT_UNNOTIFIED'],
                                   cwd=host, env=env, capture_output=True, text=True,
                                   preexec_fn=model['memory_limit'])
            (work / (name + '-check.log')).write_text(check.stdout + check.stderr)
            assert check.returncode != 0 and 'AssertionError' in check.stderr, check.stderr
            assert 'check_source_entry_fault_capture.py' in check.stderr, check.stderr
            if name == 'duplicate':
                assert 'retired probe export inventory' in check.stderr, check.stderr
            record['cases'].append(dict(name=name, harness_original_sha256=before,
                                        harness_mutant_sha256=sha(harness), capture_exit=capture.returncode,
                                        actual_probe_identities=identities, checker_exit=check.returncode,
                                        raw=str(raw_path), raw_sha256=sha(raw_path)))
        host = work / 'wire-duplicate'
        api['stage_host_inputs'](prepared, host, source['bindings'], model['verified_bytes'])
        for binary in source['native_binaries'].values():
            assert sha(Path(binary['path'])) == binary['sha256']
        harness = host / 'run_lease_wire_capture.py'
        before, text = sha(harness), harness.read_text()
        assert text.count(original) == 1
        harness.write_text(text.replace(original, mutants['duplicate']))
        cache = work / 'wire-cache'
        cache.mkdir()
        env = {k: v for k, v in os.environ.items() if not k.startswith('PYTHON')}
        env.update(PYTHONDONTWRITEBYTECODE='1', PYTHONPYCACHEPREFIX=str(cache))
        prefix = [sys.executable, '-E', '-B', '-X', 'pycache_prefix=' + str(cache)]
        capture = subprocess.run([*prefix, str(harness), 'ENTRY', 'BEFORE_WRITE'], cwd=host,
                                 env=env, capture_output=True, text=True,
                                 preexec_fn=model['memory_limit'])
        (work / 'wire-capture.log').write_text(capture.stdout + capture.stderr)
        assert capture.returncode == 0, capture.stderr
        raw_path = host / 'recovered/HOST_LEASE_WIRE_PREFIX_ENTRY_BEFORE_WRITE.json'
        raw = json.loads(raw_path.read_text())
        identities = [(r['kind'], r['endpoint']) for r in raw['public_spans']
                      if r.get('message') == 'cohort retired']
        assert identities == [('invoke', None), ('source', None), ('owner', 0), ('owner', 1), ('owner', 1)]
        journal = subprocess.run([*prefix, str(host / 'prepare_lease_wire_writer_journal.py'),
                                  'WIRE_DUPLICATE', '--capture', 'HOST_LEASE_WIRE_PREFIX_ENTRY_BEFORE_WRITE'],
                                 cwd=host, env=env, capture_output=True, text=True,
                                 preexec_fn=model['memory_limit'])
        (work / 'wire-journal.log').write_text(journal.stdout + journal.stderr)
        assert journal.returncode != 0, journal.stdout
        assert 'check_host_wire_prefix_capture.py' in journal.stderr, journal.stderr
        assert 'AssertionError: wire retired probe inventory' in journal.stderr, journal.stderr
        record['cases'].append(dict(name='wire-duplicate', harness_original_sha256=before,
                                    harness_mutant_sha256=sha(harness), capture_exit=capture.returncode,
                                    actual_probe_identities=identities, journal_exit=journal.returncode,
                                    normalizer='not run: prerequisite rejected at named inventory',
                                    raw=str(raw_path), raw_sha256=sha(raw_path)))
        record['status'] = 'passed'
    except BaseException as error:
        record.update(status='failed', error=str(error))
        raise
    finally:
        record['finished'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
        (work / 'RESULT.json').write_text(json.dumps(record, indent=2) + '\n')


if __name__ == '__main__':
    main()
