"""Fresh privileged-pipe captures and typed replays of the prepared W4 reference.

Finite physical evidence only. Compiler, capture/normalizer, exclusive pipes and
stable filesystem remain trusted; no authenticated network, recovery or alpha.
"""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import os
import re
import runpy
import subprocess
import sys
import tempfile


def physical_consumer_source(source, inspector):
    # Replay modules need not import the elaborator used by the appended audit.
    return 'import Lean\n' + source + '\n' + inspector


def current_execution_input(path, execution):
    path = Path(path).resolve()
    if not path.is_relative_to(Path(execution).resolve()):
        raise ValueError('generated input escaped selected execution')
    return path


def merge_selected_bindings(bindings, incoming):
    """Different selected identities at the same path are a conflict."""
    for path, digest in incoming.items():
        if path in bindings and bindings[path] != digest:
            raise ValueError('new binding differs from inherited consumed input: ' + path)
    bindings.update(incoming)


def bind_current_source(bindings, path):
    """A current-source role cannot replace an inherited selected identity."""
    path = Path(path).resolve()
    merge_selected_bindings(bindings, {str(path): hashlib.sha256(path.read_bytes()).hexdigest()})
    return str(path)


def stage_host_inputs(prepared, destination, bindings, verified_bytes):
    """Copy only preparation-bound host inputs, never prior capture outputs."""
    destination.mkdir()
    origin = prepared / 'host'
    copied = {}
    for name, digest in bindings.items():
        source = Path(name)
        if not source.is_relative_to(origin):
            continue
        native = origin / 'cold-native-2'
        if source.is_relative_to(native) and source not in {native / 'RESULT.json', native / 'MANIFEST.json'}:
            continue
        target = destination / source.relative_to(origin)
        target.parent.mkdir(parents=True, exist_ok=True)
        with target.open('xb') as f:
            f.write(verified_bytes(source, digest))
        copied[str(target)] = digest
    if not copied:
        raise ValueError('no bound host preparation inputs')
    return copied


def main():
    if not __debug__:
        raise SystemExit('physical evidence requires assertions')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--prepared', required=True, type=Path)
    parser.add_argument('--preparation-observation', required=True, type=Path,
                        help='external terminal-exit observation from proof_first_process_observer.py')
    args = parser.parse_args()
    work = args.prepared.resolve()
    prepared = json.loads((work / 'RESULT.json').read_text())
    if prepared['status'] != 'prepared; physical capture/replay NOT RUN':
        raise ValueError('completed fresh native/source preparation required')
    sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
    producer = work / 'proof_first_host_prepare.py'
    observer = Path(__file__).resolve().with_name('proof_first_process_observer.py')
    observation = args.preparation_observation.resolve()
    bindings = dict(prepared['bindings'])
    # Read selected observation identities only as additional consistency
    # expectations. This does not validate its source or terminal-exit claim.
    declared_observation = json.loads(observation.read_text())
    for key in ['producer_source', 'observer_source', 'result', 'log']:
        row = declared_observation[key]
        merge_selected_bindings(bindings, {row['path']: row['sha256']})
    merge_selected_bindings(bindings, {str(observation): sha(observation),
                                      str(work / 'RESULT.json'): sha(work / 'RESULT.json')})
    retained = set(bindings)
    # Keep every selected spelling: resolving a current symlink must not erase
    # the expectation at the path through which it will be consumed. Check
    # locally, before loading the helper that fully validates the observation.
    for name, expected in bindings.items():
        if sha(Path(name)) != expected:
            raise ValueError('selected input changed before observer import: ' + name)
    provenance_only = {bind_current_source(bindings, __file__),
                       bind_current_source(bindings, observer)}
    observer_api = runpy.run_path(str(observer))
    if prepared['producer_source'] != dict(path=str(producer), sha256=prepared['bindings'][str(producer)]):
        raise ValueError('selected preparation source snapshot differs')
    observed = observer_api['validate_observation'](
        observation, work / 'RESULT.json', prepared['bindings'][str(producer)])
    incoming = observer_api['observation_inputs'](observed)
    merge_selected_bindings(incoming, {str(observation): sha(observation),
                                      str(work / 'RESULT.json'): sha(work / 'RESULT.json')})
    merge_selected_bindings(bindings, incoming)
    retained.update(incoming)
    model = Path(prepared['model'])
    api_path = model / 'proof_first_reference_source_check.py'
    for p in [api_path, model / 'proof_first_composition_source_check.py']:
        if sha(p) != bindings[str(p)]:
            raise ValueError('physical runner helper changed')
    api = runpy.run_path(str(api_path))
    api['verify_integrity'](bindings)
    parent = json.loads((model / 'RESULT.json').read_text())
    compiler = Path(parent['compiler']['path'])
    prefix = compiler.parent.parent
    layout = prepared.get('toolchain_layout')
    if not layout or layout['prefix'] != str(prefix):
        raise ValueError('preparation lacks selected disjoint toolchain layout')
    runpy.run_path(str(producer))['checked_toolchain_layout'](
        prefix, Path(__file__).resolve().parents[1],
        Path(observed['producer']['path']).parents[1], *layout['repositories'])
    reused_toolchain = prepared.get('reused_toolchain_bindings', {})
    if (not reused_toolchain or reused_toolchain.get(str(compiler)) != parent['compiler']['sha256']
            or any(bindings.get(p) != h for p, h in reused_toolchain.items())
            or sha(compiler) != parent['compiler']['sha256']):
        raise ValueError('selected consumed toolchain identity differs')
    execution = Path(tempfile.mkdtemp(prefix='physical-', dir=work))
    print(execution, flush=True)
    producer_source = observer_api['archive_source'](__file__, execution / Path(__file__).name)
    observer_source = observer_api['archive_source'](observer, execution / observer.name)
    for row in [producer_source, observer_source]:
        bindings[row['path']] = row['sha256']
    output_path = execution / 'HOST_EXECUTION.json'
    host, rec = execution / 'host', execution / 'host/recovered'
    staged = stage_host_inputs(work, host, prepared['bindings'], api['verified_bytes'])
    bindings.update(staged)
    replay = host / 'replays'
    replay.mkdir()
    builds = {}
    for name, row in parent['compiled_modules'].items():
        builds[name] = dict(exit=0, source=str(work / (name + '.lean')), source_sha256=row['source_sha256'],
                            outputs={str(work / Path(p).name): digest for p, digest in row['outputs'].items()})
    result = dict(status='running', scope=__doc__, started=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  commands=[], profiles=[], controls=[], staged_inputs=staged, producer_source=producer_source,
                  preparation=str(work / 'RESULT.json'),
                  preparation_sha256=sha(work / 'RESULT.json'))
    env = {k: v for k, v in os.environ.items()
           if not k.startswith('PYTHON') and k not in {'LEAN_CC', 'LEAN_SYSROOT'}}
    env.update(LEAN_PATH=str(work), LEAN_SYSROOT=str(prefix),
               MIR_PROOF_FIRST_LEAN=str(compiler.resolve()), PYTHONDONTWRITEBYTECODE='1')

    def save():
        output_path.write_text(json.dumps(result, indent=2) + '\n')

    def bind(p, expected=None):
        p = Path(p)
        digest = sha(p)
        if expected is not None and digest != expected:
            raise ValueError('physical evidence changed: ' + str(p))
        if str(p) in bindings and bindings[str(p)] != digest:
            raise ValueError('physical binding changed: ' + str(p))
        bindings[str(p)] = digest
        return digest

    def read(p):
        bind(p)
        return json.loads(api['verified_bytes'](p, bindings[str(p)]))

    def run(command, label):
        api['verify_integrity'](bindings)
        r = subprocess.run(list(map(str, command)), cwd=host, env=env, capture_output=True,
                           text=True, preexec_fn=api['memory_limit'])
        text = r.stdout + r.stderr
        log = replay / (label + '.log')
        with log.open('x') as f:
            f.write(text)
        result['commands'].append(dict(command=list(map(str, command)), exit=r.returncode,
                                       log=str(log), log_sha256=bind(log)))
        save()
        api['require_completed'](r, str(log))
        api['verify_integrity'](bindings)
        return text

    def python(script, arguments, label):
        cache = Path(tempfile.mkdtemp(prefix='python-cache-', dir=replay))
        env['PYTHONPYCACHEPREFIX'] = str(cache)
        text = run([sys.executable, '-E', '-B', '-X', 'pycache_prefix=' + str(cache), host / script, *arguments], label)
        if list(cache.rglob('*')):
            raise ValueError('physical child wrote bytecode')
        return text

    def lean(original, label, markers=(), rejects=()):
        source = api['verified_bytes'](original, bind(original)).decode()
        target = replay / (label + '.lean')
        with target.open('x') as f:
            f.write(physical_consumer_source(source, api['IMPORT_INSPECTOR']))
        bind(target)
        roots = [line.split()[1] for line in source.splitlines() if line.startswith('import ')]
        text = run([compiler, '--trust=0', '-j1', target], label)
        actual = api['validate_lean_imports'](text, api['local_import_closure'](roots, work), builds, work, prefix)
        for path, digest in actual.items():
            bind(path, digest)
        if any(marker not in text for marker in markers):
            raise ValueError('physical consumer omitted a required check: ' + label)
        names = re.findall(r'^CONTROL_EXPECTED_REJECT (\w+):', text, re.M)
        if names != list(rejects) or 'CONTROL_EXPECTED_ACCEPT ' in text:
            raise ValueError('physical negative-control inventory differs: ' + label)
        result['controls'].extend(dict(suite=label, label=name) for name in names)
        api['verify_integrity'](bindings)
        save()

    basic = {'__main__', 'check_host_commit_capture', 'check_writer_occurrences', 'check_writer_parent_occurrences'}
    total = {'__main__', 'check_host_commit_capture', 'check_writer_occurrences_total', 'check_writer_parent_occurrences_total'}
    normal_more = {'bind_cohort_store_observations', 'bind_cohort_values', 'bind_source_entry_observations',
                   'bind_writer_statement_observations_v2', 'check_joint_capture_recipes', 'owner_process_limits',
                   'publication_process_check', 'paired_statement_cohort_event_json'}
    known_more = {'bind_cohort_fault_observations', 'bind_cohort_fault_values', 'bind_cohort_store_observations',
                  'bind_source_entry_observations', 'bind_writer_statement_observations_v2', 'check_joint_capture_recipes',
                  'check_source_entry_fault_capture', 'lease_fault_event_json', 'owner_process_limits', 'publication_process_check'}
    wire_journal = basic | {'check_host_wire_prefix_capture', 'check_joint_capture_recipes', 'owner_process_limits', 'publication_process_check'}
    wire_more = {'bind_cohort_fault_values', 'bind_cohort_store_observations', 'bind_source_entry_observations',
                 'bind_writer_statement_observations_v2', 'lease_wire_event_json'}

    def inputs(tag, label, script, stem, key, expected, extra=()):
        python(script, [tag, '--capture', label, *extra], tag + '-' + Path(script).stem)
        record = read(host / (stem + '_' + tag + '.json'))
        py = record['python']
        if py['optimize'] != 0 or not py['dont_write_bytecode'] or not py['cache_prefix']:
            raise ValueError('unbound Python execution profile')
        imports = {row['module']: row for row in record['python_imports']}
        if len(imports) != len(record['python_imports']) or set(imports) != expected:
            raise ValueError('actual Python import inventory differs: ' + script)
        for module, row in imports.items():
            selected = host / script if module == '__main__' else (rec if module in {'publication_process_check', 'owner_process_limits'} else host) / (module + '.py')
            if row['origin'] != str(selected) or row['path'] != str(selected.resolve()):
                raise ValueError('actual Python import path differs: ' + module)
            bind(selected, row['sha256'])
        for row in record[key]:
            p = current_execution_input(row['path'], execution)
            bind(p, row['sha256'])
        bind(host / script, record['generator_sha256'])
        bind(rec / (label + '.json'), record['receipt_sha256'])
        return record

    profiles = [
        ('normal', 'FIELD_NORMAL', 'HOST_WRITER_STATEMENT_CAPTURE_V2', 'run_writer_statement_capture_v2.py', []),
        ('omission', 'FIELD_OMISSION', 'HOST_WRITER_STATEMENT_OMISSION_V2', 'run_writer_statement_omission_v2.py', []),
        ('prefix', 'SOURCE_ONLY_PREFIX', 'HOST_SOURCE_ONLY_STATEMENT_CAPTURE', 'run_source_only_statement_capture.py', []),
    ]
    for cut in ['UNNOTIFIED', 'NOTIFIED', 'SNAPSHOT', 'NOTIFIED_MIXED']:
        profiles.append(('known', 'LEASEFINAL_' + cut, 'HOST_LEASE_KNOWN_FAULT_' + cut,
                         'run_lease_known_fault_capture.py', cut.split('_')))
    for endpoint in ['ENTRY', 'OWNER']:
        for cut in ['BEFORE_WRITE', 'BODY_LOST', 'RAW_CAPTURE', 'RAW_CAPTURE_MIXED']:
            mixed = cut.endswith('_MIXED')
            arguments = [endpoint, cut.removesuffix('_MIXED')] + (['MIXED', 'MIXED'] if mixed else [])
            profiles.append(('wire', 'LEASEWIRE_' + endpoint + '_' + cut, 'HOST_LEASE_WIRE_PREFIX_' + endpoint + '_' + cut,
                             'run_lease_wire_capture.py', arguments))
    try:
        save()
        for kind, tag, label, harness, arguments in profiles:
            python(harness, arguments, tag + '-capture')
            capture = read(host / (label + '_RESULT.json'))
            raw = read(rec / (label + '.json'))
            bind(rec / (label + '.json'), capture['receipt_sha256'])
            bind(host / harness, capture['harness_sha256'])
            fault = kind in {'known', 'wire'}
            mode = capture['python']
            if mode['optimize'] != 0 or not mode['dont_write_bytecode'] or not mode['cache_prefix']:
                raise ValueError('capture interpreter profile differs')
            expected_capture = {'__main__', 'publication_process_check', 'owner_process_limits',
                                'source_owner_cohort_resource', 'owner_resource_writer', 'private_wire_slot',
                                'cohort_observer_native', 'check_host_commit_capture'}
            expected_capture |= ({'check_writer_occurrences_total', 'check_writer_parent_occurrences_total'}
                                 if kind == 'prefix' else {'check_writer_occurrences', 'check_writer_parent_occurrences'})
            if kind in {'normal', 'omission', 'prefix'} or kind == 'wire' and arguments[0] == 'ENTRY':
                expected_capture.add('check_joint_capture_recipes')
            if fault:
                expected_capture.add('bind_writer_statement_observations_v2')
                if capture.get('normal_EOF') is not False:
                    raise ValueError('fault producer claimed normal EOF')
            imports = {row['module']: row for row in capture['python_imports']}
            if len(imports) != len(capture['python_imports']) or set(imports) != expected_capture:
                raise ValueError('capture actual import inventory differs: ' + tag)
            caller = host / ('writer-occurrence-red-caller' if kind == 'omission' else 'entry-handoff')
            for name, row in imports.items():
                selected = (host / harness if name == '__main__' else
                            (rec if name in {'publication_process_check', 'owner_process_limits'} else
                             caller if name in {'source_owner_cohort_resource', 'owner_resource_writer', 'private_wire_slot'} else host) / (name + '.py'))
                if row['origin'] != str(selected) or row['path'] != str(selected.resolve()):
                    raise ValueError('capture actual import path differs: ' + name)
                bind(selected, row['sha256'])
            if raw['state'] != ('failed' if fault else 'passed'):
                raise ValueError('wrong actual capture state')
            for endpoint in ['source', 'owner0', 'owner1', 'owner2']:
                child = raw[endpoint]
                binary = prepared['native_binaries']['source' if endpoint == 'source' else 'owner']
                if child['binary'] != binary['path'] or child['binary_sha256'] != binary['sha256'] or child['exit'] != (-9 if fault else 0) or child['state'] != 'reaped':
                    raise ValueError('native child identity/status mismatch')
                if not fault and child.get('stdout_eof') is not True:
                    raise ValueError('normal child lacked complete EOF')
            raw_root = Path(raw['root'])
            if raw_root.parent != rec:
                raise ValueError('unexpected actual capture root')
            for p in raw_root.rglob('*'):
                if p.is_file():
                    bind(p)
            if kind == 'wire' and (not raw['wire_fault']['retired'] or raw['wire_fault']['protocol_calls'] or raw['wire_fault']['observed_descriptors'] != 8 or raw['wire_fault']['public_refusals'] != 5):
                raise ValueError('actual wire retirement/probe inventory incomplete')
            if kind in {'normal', 'omission'}:
                journal, normalizer, expected_j, expected_r = 'prepare_paired_statement_journal.py', 'prepare_field_cohort_replay.py', basic, basic | normal_more
                decoder, runner, extra = 'LeaseCohortCaptureJson', 'RunCohortHostReplay_', []
            elif kind == 'prefix':
                journal, normalizer, expected_j = 'prepare_total_statement_journal.py', 'prepare_total_field_replay.py', total
                expected_r = total | (normal_more - {'bind_source_entry_observations', 'bind_writer_statement_observations_v2'}) | {'bind_source_entry_observations_total', 'bind_writer_statement_observations_total'}
                decoder, runner, extra = 'LeaseCohortCaptureJson', 'RunCohortHostReplay_', ['--prefix-only']
            elif kind == 'known':
                journal, normalizer, expected_j, expected_r = 'prepare_lease_fault_writer_journal.py', 'prepare_lease_fault_replay.py', basic, basic | known_more
                decoder, runner, extra = 'LeaseKnownFaultCaptureJson', 'RunCohortKnownFaultReplay_', []
            else:
                journal, normalizer, expected_j, expected_r = 'prepare_lease_wire_writer_journal.py', 'prepare_lease_wire_replay.py', wire_journal, wire_journal | wire_more
                decoder, runner, extra = 'LeaseWireCaptureJson', 'RunCohortWireReplay_', []
            inputs(tag, label, journal, 'SOURCE_ENTRY_WRITER_JOURNAL_INPUTS', 'raw', expected_j)
            normalized = inputs(tag, label, normalizer, 'COHORT_HOST_PREFIX_INPUTS', 'source_snapshot_files', expected_r, extra)
            bind(rec / (decoder + '.lean'), normalized['decoder_sha256'])
            bind(rec / (decoder.replace('Json', 'Replay') + '.lean'), normalized['consumer_sha256'])
            bind(current_execution_input(normalized['events_path'], execution), normalized['events_sha256'])
            original = rec / (runner + tag + '.lean')
            bind(original, normalized['runner_sha256'])
            count = len(raw['host_commit_trace'])
            stores = sum(row['point'] == 'writer_store_after' for row in raw['host_commit_trace'])
            if normalized['cohort_observations'] != count or normalized['writer_statements'] != stores:
                raise ValueError('actual/normalized observation inventory differs')
            markers = ['STRICT_HISTORY_PAIRED_OK', 'WRITER_STATEMENTS_OK completedStores=' + str(stores)]
            markers += (['LAST_KNOWN_CORRESPONDENCE_OK', 'UNCONFIRMED_CORRESPONDENCE_OK', 'retiredProbeObservations=10', 'COHORT_WIRE_RETIRED_PREFIX_OK'] if kind == 'wire' else ['KNOWN_STATE_CORRESPONDENCE_OK'])
            if kind == 'known':
                markers += ['RETIRED_CORRESPONDENCE_OK observations=10', 'SHARED_HOST_RETIRED_PREFIX_OK']
            if kind == 'prefix':
                markers += ['prefix=true']
            if kind != 'omission':
                lean(original, tag + '-replay', markers)
            result['profiles'].append(dict(kind=kind, tag=tag, capture_sha256=sha(rec / (label + '.json')),
                                           observations=count, stores=stores, result='pending negative replay' if kind == 'omission' else 'checked finite replay'))
            save()
        for script, driver, metadata in [
            ('prepare_field_controls.py', 'RunFieldCohortControls', 'FIELD_NEGATIVE_INPUTS.json'),
            ('prepare_field_value_controls.py', 'RunFieldValueControls', 'FIELD_VALUE_NEGATIVE_INPUTS.json'),
            ('create_known_lease_controls.py', 'RunKnownLeaseControls', 'KNOWN_LEASE_CONTROLS_INPUTS.json'),
            ('create_wire_lease_controls.py', 'RunWireLeaseControls', 'WIRE_LEASE_CONTROLS_INPUTS.json'),
        ]:
            python(script, [], driver + '-prepare')
            rows = read(host / metadata)
            rows = rows['cases'] if isinstance(rows, dict) else rows
            for row in rows:
                bind(current_execution_input(row['path'], execution), row['sha256'])
            names = [row['label'] for row in rows]
            if script == 'prepare_field_controls.py':
                names.append('ACTUAL_PROCESS_OMISSION')
            if len(names) != len(set(names)):
                raise ValueError('duplicate negative-control identity within suite')
            lean(rec / (driver + '.lean'), driver, rejects=names)
            if script == 'prepare_field_controls.py':
                next(row for row in result['profiles'] if row['kind'] == 'omission')['result'] = 'actual captured omission rejected by typed replay'
        # The same actual zero-writer prefix is accepted as a prefix and refused
        # as a complete program. Delete each actual store while keeping all rows.
        metadata = read(host / 'COHORT_HOST_PREFIX_INPUTS_SOURCE_ONLY_PREFIX.json')
        events = read(Path(metadata['events_path']))
        positions = [i for i, row in enumerate(events) if row['tag'] == 'cohortCommit']
        if len(positions) != 2:
            raise ValueError('source-only actual store inventory changed')
        action = (rec / 'RunCohortHostReplay_SOURCE_ONLY_PREFIX.lean').read_text().split('#eval ', 1)[1].strip()
        body = 'import LeaseCohortCaptureJson\n' + (rec / 'HostControlHeader.lean').read_text() + 'def main : IO Unit := do\n'
        names = ['OMIT_BOOTSTRAP_STORE', 'OMIT_SOURCE_SNAPSHOT_STORE']
        for name, position in zip(names, positions):
            changed = rec / ('source-only-control-' + name + '.json')
            with changed.open('x') as f:
                json.dump(events[:position] + events[position+1:], f)
            bind(changed)
            selected = action.replace(json.dumps(metadata['events_path']), json.dumps(str(changed)))
            body += '  check ' + json.dumps(name) + ' (' + selected + ') (some "actual cohort memory mismatch")\n'
        if not action.endswith(' false'):
            raise ValueError('explicit source-prefix completion switch missing')
        names.append('PREFIX_NOT_PROGRAM_COMPLETION')
        body += '  check "PREFIX_NOT_PROGRAM_COMPLETION" (' + action[:-6] + ' true) (some "completed shared capture has outstanding funding phase")\n#eval main\n'
        controls = rec / 'RunSourcePrefixControls.lean'
        with controls.open('x') as f:
            f.write(body)
        lean(controls, controls.stem, rejects=names)
        if len(result['profiles']) != 15 or len(result['controls']) != 53:
            raise ValueError('selected complete profile/control inventory differs')
        api['verify_integrity'](bindings)
        consumed = observer_api['consumed_bindings'](bindings, provenance_only, retained)
        result.update(status='passed selected finite private-pipe profiles; W4-B review and wider W4 remain open',
                      export_roles=dict(provenance_only=sorted(provenance_only), retained=sorted(retained)),
                      bindings=consumed, provenance_bindings={p: h for p, h in bindings.items() if p not in consumed})
    except BaseException as error:
        result.update(status='failed', error=str(error))
        raise
    finally:
        result['finished'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
        save()
    print('selected physical captures and typed controls passed; no wider W4/alpha acceptance', flush=True)


if __name__ == '__main__':
    main()
