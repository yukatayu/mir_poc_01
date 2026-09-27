"""Actual terminal-exit handoff controls; no model/native/physical E2E claim."""
from pathlib import Path
import argparse
import ast
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
    parser.add_argument('--observer', type=Path, default=Path(__file__).resolve().parents[1] / 'proof_first_process_observer.py')
    parser.add_argument('--model-run', type=Path)
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[2]
    root = args.work_root.resolve()
    assert root.is_dir() and not root.is_relative_to(repo)
    work = Path(tempfile.mkdtemp(prefix='process-observation-controls-', dir=root))
    record = dict(scope=__doc__, controls=[])
    try:
        # Extract the real preparation's final stdout operation. Exercise that
        # operation after saving success, without claiming to rerun native builds.
        source = repo / 'scripts/proof_first_host_prepare.py'
        tree = ast.parse(source.read_text())
        main_node = next(n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == 'main')
        final_print = ast.get_source_segment(source.read_text(), main_node.body[-1])
        assert final_print.startswith('print('), final_print
        producer = work / 'producer.py'
        producer.write_text('from pathlib import Path\nimport json, os, sys\n'
                            'receipt = Path(sys.argv[1])\n'
                            'print(str(receipt.parent), flush=True)\n'
                            'sys.stdin.readline()\n'
                            'receipt.write_text(json.dumps({"status":"prepared; physical capture/replay NOT RUN"}))\n'
                            + final_print + '\n')
        bad_result = work / 'broken-result.json'
        with (work / 'broken-stderr.log').open('wb') as err:
            child = subprocess.Popen([sys.executable, '-E', '-B', str(producer), str(bad_result)],
                                     stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=err)
            started = child.stdout.readline().decode().strip()
            assert started == str(work)
            child.stdout.close()
            child.stdin.write(b'continue\n')
            child.stdin.close()
            terminal = child.wait()
        assert terminal != 0 and json.loads(bad_result.read_text())['status'].startswith('prepared')
        assert 'BrokenPipeError' in (work / 'broken-stderr.log').read_text()
        record['controls'].append(dict(name='actual final stdout fails after successful receipt', exit=terminal))
        observer = args.observer.resolve()
        api = runpy.run_path(str(observer)) if observer.is_file() else {}
        assert 'observe' in api and 'validate_observation' in api, 'terminal-exit observer and downstream gate missing'
        # Genuine files in the two-checkout layout. This exercises the export
        # boundary, not a full native build or a forged successful receipt.
        checkout = work / 'later-checkout'
        paths = [checkout / 'scripts/runner.py',
                 checkout / '.lab/prepared/host/cold-native-2/source-worker',
                 checkout / '.lab/prepared/physical/runner.py',
                 checkout / '.lab/prepared/physical/replay.log',
                 checkout / '.lab/prepared/physical/capture.json',
                 checkout / '.other/model/archive.py',
                 checkout / '.other/observation.json',
                 work / 'external-evidence/observation.json']
        for index, path in enumerate(paths):
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text('selected artifact ' + str(index) + '\n')
        export_sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
        bound = {str(p): export_sha(p) for p in paths}
        original = str(paths[0])
        retained = {str(p) for p in paths[1:3] + paths[5:]}
        export = api['consumed_bindings']
        selected = export(bound, {original}, retained)
        assert selected == {p: h for p, h in bound.items() if p != original}
        assert export(bound, {original}, retained | {original}) == bound
        # Removing all ancestry-related restrictions must not turn unknown role
        # names into silent omissions or accept an unbound consumption claim.
        for provenance, live in [({str(work / 'missing')}, retained),
                                  ({original}, retained | {str(work / 'missing')})]:
            try:
                export(bound, provenance, live)
            except ValueError:
                pass
            else:
                raise AssertionError('unbound export role accepted')
        record['controls'].append('later checkout retains inherited worker/archive/observation and fresh capture/log; dual live role wins')
        # The producer creates an actual successful-looking receipt then exits
        # nonzero. The observation is truthful; the gate must refuse it.
        failing = work / 'failing.py'
        failing.write_text('from pathlib import Path\nimport sys\nPath(sys.argv[1]).write_text("{\\\"status\\\":\\\"passed\\\"}")\nraise SystemExit(7)\n')
        good = work / 'good.py'
        good.write_text('from pathlib import Path\nimport sys\nPath(sys.argv[1]).write_text("{\\\"status\\\":\\\"passed\\\"}")\n')
        sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
        for name, script, expected in [('failed', failing, 7), ('success', good, 0)]:
            result = work / (name + '-result.json')
            log = work / (name + '.log')
            obs = work / (name + '-observation.json')
            command = [sys.executable, '-E', '-B', str(script), str(result)]
            actual = api['observe'](command, work, log, obs, script, lambda: result)
            assert actual == expected
            try:
                api['validate_observation'](obs, result, sha(script))
            except ValueError:
                assert expected != 0, 'successful producer rejected'
            else:
                assert expected == 0, 'nonzero producer admitted by its success JSON'
            record['controls'].append(name + ' producer classified by actual terminal exit')
        obs = work / 'success-observation.json'
        result = work / 'success-result.json'
        saved = json.loads(obs.read_text())
        historical_sha = sha(good)
        good.write_text(good.read_text() + '# later live source repair\n')
        api['validate_observation'](obs, result, historical_sha)
        try:
            api['validate_observation'](obs, result, sha(good))
        except ValueError:
            record['controls'].append('historical source remains usable; cannot relabel as current')
        else:
            raise AssertionError('old execution silently relabelled as repaired source')
        for label, path in [('result', result), ('log', work / 'success.log'),
                            ('producer archive', Path(saved['producer_source']['path'])),
                            ('observer archive', Path(saved['observer_source']['path']))]:
            original = path.read_bytes()
            path.write_bytes(original + b'\nchanged\n')
            try:
                api['validate_observation'](obs, result, historical_sha)
            except ValueError:
                record['controls'].append(label + ' drift rejected')
            else:
                raise AssertionError(label + ' drift admitted')
            finally:
                path.write_bytes(original)
        try:
            api['validate_observation'](obs, bad_result, historical_sha)
        except ValueError:
            record['controls'].append('different result identity rejected')
        else:
            raise AssertionError('unobserved result admitted')
        # A caller must use the supervisor's exit even when its child observation
        # is valid. This is a caller-policy control, not a full three-stage run.
        outer = work / 'late-supervisor.py'
        outer_result, outer_obs = work / 'outer-result.json', work / 'outer-observation.json'
        command = [sys.executable, '-E', '-B', str(good), str(outer_result)]
        outer.write_text('from pathlib import Path\nimport runpy\n'
                         + 'api=runpy.run_path(' + repr(str(observer)) + ')\n'
                         + 'status=api["observe"](' + repr(command) + ',' + repr(str(work))
                         + ',' + repr(str(work / 'outer.log')) + ',' + repr(str(outer_obs))
                         + ',' + repr(str(good)) + ',lambda:Path(' + repr(str(outer_result)) + '))\n'
                         + 'raise SystemExit(status or 9)\n')
        completed = subprocess.run([sys.executable, '-E', '-B', str(outer)], capture_output=True, text=True)
        assert completed.returncode == 9, completed.stderr
        api['validate_observation'](outer_obs, outer_result, sha(good))
        started_next = work / 'next-stage-started'
        if completed.returncode == 0:
            started_next.write_text('started')
        assert not started_next.exists()
        record['controls'].append('caller refuses late supervisor failure despite valid child observation')
        if args.model_run:
            prepare = runpy.run_path(str(repo / 'scripts/proof_first_host_prepare.py'))
            try:
                prepare['read_bound_model'](args.model_run.resolve())
            except ValueError as error:
                assert 'observation' in str(error), str(error)
                record['controls'].append('genuine completed model requires terminal observation')
            else:
                raise AssertionError('model handoff accepts no terminal observation')
        record['status'] = 'passed'
    except BaseException as error:
        record.update(status='failed', error=str(error))
        raise
    finally:
        (work / 'RESULT.json').write_text(json.dumps(record, indent=2) + '\n')
        print(work, flush=True)


if __name__ == '__main__':
    main()
