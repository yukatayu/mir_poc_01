"""Real consumer-import and fresh-execution isolation controls for W4 evidence."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import runpy
import subprocess
import tempfile
import sys
import types


def main():
    if not __debug__:
        raise SystemExit('control assertions required')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work-root', required=True, type=Path)
    parser.add_argument('--runner', type=Path, default=Path(__file__).resolve().parents[1] / 'proof_first_host_run.py')
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[2]
    root = args.work_root.resolve()
    assert root.is_dir() and not root.is_relative_to(repo)
    work = Path(tempfile.mkdtemp(prefix='host-execution-controls-', dir=root))
    print(work, flush=True)
    api = runpy.run_path(str(repo / 'scripts/proof_first_reference_source_check.py'))
    runner = args.runner.resolve()
    sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
    before = sha(runner)
    selected = runpy.run_path(str(runner))
    prefix = Path(subprocess.check_output(['lean', '--print-prefix'], text=True).strip()).resolve()
    compiler = prefix / 'bin/lean'
    records = []
    result = dict(status='running', runner_sha256=before, records=records)

    def run(name, source):
        p = work / (name + '.lean')
        p.write_text(source)
        env = {k: v for k, v in os.environ.items() if not k.startswith('LEAN')}
        env['LEAN_PATH'] = str(work)
        r = subprocess.run([str(compiler), '--trust=0', '-j1', str(p)], cwd=work,
                           env=env, capture_output=True, text=True, preexec_fn=api['memory_limit'])
        text = r.stdout + r.stderr
        log = work / (name + '.log')
        log.write_text(text)
        records.append(dict(name=name, exit=r.returncode, source_sha256=sha(p), log_sha256=sha(log)))
        return r, text

    try:
        # A current source may occupy an inherited evidence path after a
        # checkout is placed around that evidence. Its historical digest must
        # be checked before adding its current-source role.
        original = work / 'selected-observer.py'
        original.write_text('retained source\n')
        inherited = {str(original): sha(original)}
        assert 'bind_current_source' in selected, 'current source can overwrite inherited identity'
        add_source = selected['bind_current_source']
        add_source(inherited, original)
        expected = dict(inherited)
        original.write_text('different current source\n')
        try:
            add_source(inherited, original)
        except ValueError:
            assert inherited == expected
            records.append(dict(name='changed inherited/current source alias refused before overwrite'))
        else:
            raise AssertionError('inherited digest overwritten by current source')
        separate = work / 'new-current-source.py'
        separate.write_text('new current source\n')
        add_source(inherited, separate)
        assert inherited[str(separate)] == sha(separate)
        # Execute the real main's startup prefix with an explicit fixture map.
        # Stop at observer import: no invented preparation is accepted and no
        # physical/native E2E result is claimed by this boundary control.
        entry = selected['main']
        context = entry.__globals__
        old_file, old_runpy, old_argv = context['__file__'], context['runpy'], sys.argv
        class ImportReached(Exception):
            pass
        cases = [
            ('symlink-spelling-changed-first', 'spelling', True, True),
            ('direct-observer-same', 'observer', False, False),
            ('direct-observer-changed', 'observer', False, True),
            ('direct-runner-same', 'runner', False, False),
            ('direct-runner-changed', 'runner', False, True),
            ('symlink-spelling-same', 'spelling', True, False),
            ('symlink-spelling-changed', 'spelling', True, True),
            ('symlink-target-same', 'target', True, False),
            ('symlink-target-changed', 'target', True, True),
            ('symlink-new-source', 'none', True, False),
            ('observation-only-same', 'observation', True, False),
            ('observation-only-changed', 'observation', True, True),
            ('hardlink-target-same', 'target', 'hard', False),
            ('hardlink-target-changed', 'target', 'hard', True),
            ('observation-role-equal', 'collision', False, False),
            ('observation-role-conflict', 'collision', False, True),
        ]
        try:
            for name, role, linked, changed in cases:
                startup = work / name
                startup.mkdir()
                current = startup / 'proof_first_host_run.py'
                current.write_bytes(runner.read_bytes())
                observer = startup / 'proof_first_process_observer.py'
                target = startup / 'observer-implementation.py' if linked else observer
                target.write_text('# selected observer implementation\n')
                if linked == 'hard':
                    os.link(target, observer)
                elif linked:
                    observer.symlink_to(target)
                expected_observer = sha(target)
                inherited = {}
                selected_path = {'observer': observer, 'runner': current,
                                 'spelling': observer, 'target': target}.get(role)
                if selected_path is not None:
                    inherited[str(selected_path)] = sha(selected_path)
                fixture = startup / 'fixture'
                fixture.mkdir()
                receipt = fixture / 'RESULT.json'
                receipt.write_text(json.dumps({
                    'status': 'prepared; physical capture/replay NOT RUN',
                    'bindings': inherited}))
                observation = startup / 'OBSERVATION.json'
                observation_rows = {}
                for key in ['producer_source', 'observer_source', 'result', 'log']:
                    item = receipt if key == 'result' else startup / (key + '.txt')
                    if key != 'result':
                        item.write_text('explicit prefix fixture ' + key + '\n')
                    if role == 'observation' and key == 'observer_source':
                        item = target
                    observation_rows[key] = dict(path=str(item), sha256=sha(item))
                observation.write_text(json.dumps(observation_rows))
                if changed:
                    (current if role == 'runner' else target).write_text('# changed current implementation\n')
                if role == 'collision':
                    # Two role declarations use the same current pathname. A
                    # last-write-wins comprehension would lose the old digest
                    # and reach import in the differing-byte case.
                    observation_rows['producer_source'] = dict(path=str(target), sha256=expected_observer)
                    observation_rows['observer_source'] = dict(path=str(target), sha256=sha(target))
                    observation.write_text(json.dumps(observation_rows))
                def stop_before_import(path):
                    assert Path(path) == observer
                    raise ImportReached()
                context['__file__'] = str(current)
                context['runpy'] = types.SimpleNamespace(run_path=stop_before_import)
                sys.argv = [str(current), '--prepared', str(fixture),
                            '--preparation-observation', str(observation)]
                try:
                    entry()
                except ImportReached:
                    assert not changed, name + ': conflicting selected source loaded before preflight'
                except ValueError as error:
                    assert changed and ('selected input' in str(error) or
                                        'inherited consumed input' in str(error)), str(error)
                else:
                    raise AssertionError('startup control unexpectedly completed')
                records.append(dict(name=name, expected='refused before import' if changed else 'import boundary reached',
                                    scope='actual main prefix with explicit fixture; no preparation acceptance or capture'))
                if not changed:
                    # These are the runner's actual helpers and shared exporter;
                    # the full physical workflow separately exercises final export.
                    current_bindings = dict(inherited)
                    bound_key = selected['bind_current_source'](current_bindings, observer)
                    assert bound_key == str(observer.resolve()), 'source binding must return its canonical role key'
                    observed_role = {str(target)} if role in {'observation', 'collision'} else set()
                    retained = set(inherited) | observed_role
                    if observed_role:
                        current_bindings[str(target)] = expected_observer
                    exporter = runpy.run_path(str(repo / 'scripts/proof_first_process_observer.py'))['consumed_bindings']
                    exported = exporter(current_bindings, {bound_key}, retained)
                    assert all(exported[p] == digest for p, digest in inherited.items())
                    if bound_key not in retained:
                        assert bound_key not in exported
                    records.append(dict(name=name + ' canonical export role', retained=sorted(retained)))
            merge = selected['merge_selected_bindings']
            known = {'kept': 'old'}
            try:
                merge(known, {'new': 'added', 'kept': 'conflict'})
            except ValueError:
                assert known == {'kept': 'old'}
            else:
                raise AssertionError('multi-entry conflict partially replaced expectation')
            records.append(dict(name='multi-entry conflict leaves map unchanged'))
        finally:
            context['__file__'], context['runpy'], sys.argv = old_file, old_runpy, old_argv
        source = '#eval IO.println "ACTUAL_CONSUMER_RAN"\n'
        r, text = run('MissingImport', source + api['IMPORT_INSPECTOR'])
        assert r.returncode != 0 and 'ACTUAL_CONSUMER_RAN' in text and 'include `import Lean.Elab.Command`' in text
        render = selected['physical_consumer_source']
        r, text = run('BoundConsumer', render(source, api['IMPORT_INSPECTOR']))
        api['require_completed'](r, 'BoundConsumer')
        assert 'ACTUAL_CONSUMER_RAN' in text
        api['validate_lean_imports'](text, set(), {}, work, prefix)
        prepared = work / 'prepared'
        origin = prepared / 'host'
        origin.mkdir(parents=True)
        source_file = origin / 'worker.py'
        source_file.write_text('bound input\n')
        (origin / 'OLD_CAPTURE.json').write_text('{"status":"passed"}\n')
        bindings = {str(source_file): sha(source_file)}
        native = origin / 'cold-native-2'
        native.mkdir()
        for name in ['RESULT.json', 'MANIFEST.json', 'source-worker']:
            p = native / name
            p.write_text('native preparation ' + name)
            bindings[str(p)] = sha(p)
        stage = selected['stage_host_inputs']
        first = stage(prepared, work / 'attempt1', bindings, api['verified_bytes'])
        second = stage(prepared, work / 'attempt2', bindings, api['verified_bytes'])
        assert set(first) != set(second)
        assert (work / 'attempt1/worker.py').read_bytes() == source_file.read_bytes()
        assert not (work / 'attempt1/OLD_CAPTURE.json').exists()
        for name in ['RESULT.json', 'MANIFEST.json']:
            assert (work / 'attempt1/cold-native-2' / name).read_bytes() == (native / name).read_bytes()
        assert not (work / 'attempt1/cold-native-2/source-worker').exists()
        current_input = selected['current_execution_input']
        actual = work / 'attempt2/worker.py'
        assert current_input(actual, work / 'attempt2') == actual.resolve()
        (work / 'attempt2/old-link').symlink_to(work / 'attempt1/worker.py')
        for stale in [source_file, work / 'attempt1/worker.py', work / 'attempt2/old-link']:
            try:
                current_input(stale, work / 'attempt2')
            except ValueError:
                records.append(dict(name='non-current generated input refused', input=str(stale)))
            else:
                raise AssertionError('stale generated input admitted')
        try:
            stage(prepared, work / 'attempt1', bindings, api['verified_bytes'])
        except FileExistsError:
            pass
        else:
            raise AssertionError('existing execution was overwritten')
        source_file.write_text('drifted input\n')
        try:
            stage(prepared, work / 'attempt3', bindings, api['verified_bytes'])
        except ValueError:
            pass
        else:
            raise AssertionError('changed preparation input was accepted')
        result['status'] = 'passed'
    except BaseException as error:
        result.update(status='failed', error=str(error))
        raise
    finally:
        assert sha(runner) == before
        (work / 'RESULT.json').write_text(json.dumps(result, indent=2) + '\n')


if __name__ == '__main__':
    main()
