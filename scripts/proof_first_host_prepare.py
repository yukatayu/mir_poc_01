"""Prepare fresh native workers and actual source inputs for the W4 reference.

This consumes a completed, bound --with-host-model run. Exit zero means only
preparation: physical captures, replay controls and W4-B acceptance remain due.
No historical native executable, expected trace or recovery patch is consumed.
This private profile requires the selected toolchain prefix to be disjoint from
both current and historical producer repositories. Overlapping installations
remain outside this checked private profile. Live artifact export uses explicit
source/retained roles, not directory ancestry.
"""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import os
import re
import runpy
import shutil
import subprocess
import sys
import tempfile


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def checked_toolchain_layout(prefix, *repositories):
    prefix = Path(prefix).resolve()
    roots = sorted({str(Path(p).resolve()) for p in repositories})
    if not roots or any(prefix.is_relative_to(p) or Path(p).is_relative_to(prefix)
                        for p in roots):
        raise ValueError('selected toolchain and producer repositories must be disjoint')
    return dict(prefix=str(prefix), repositories=roots)


def host_source_paths(repo, work, supplied_name):
    relative = Path(supplied_name)
    if relative.is_absolute() or '..' in relative.parts or relative.as_posix() != supplied_name:
        raise ValueError('host source needs a canonical repository-relative path')
    supplied = repo / relative
    source = supplied.resolve()
    if not source.is_relative_to(repo) or supplied.is_symlink():
        raise ValueError('host source escaped repository')
    if relative.is_relative_to('scripts/proof_first_host') and relative.suffix == '.py':
        target = work / 'host' / relative.relative_to('scripts/proof_first_host')
    elif relative.parent == Path('samples/lean/host-reference') and relative.suffix == '.lean':
        target = work / relative.name
    elif relative.parent == Path('samples/clean-near-end/mirrorea-proof-first-composition/host-reference'):
        places = {'main.mir': 'qualified-source', 'continue-c.mir': 'source-continuation',
                  'continue-a.mir': 'source-continuation-owner-a'}
        if relative.name not in places:
            raise ValueError('unexpected ordinary source input')
        target = work / 'host/recovered' / places[relative.name] / 'main.mir'
    else:
        raise ValueError('unexpected host source category')
    if not target.resolve().is_relative_to(work.resolve()):
        raise ValueError('host destination escaped fresh work directory')
    return source, target


def native_environment(environ, work, prefix, cache, parser_binary):
    env = {k: v for k, v in environ.items()
           if not k.startswith('PYTHON') and k not in {'LEAN_CC', 'LEAN_SYSROOT', 'LEAN_PATH'}}
    env.update(LEAN_SYSROOT=str(prefix), LEAN_PATH=str(work),
               MIR_PROOF_FIRST_LEAN=str((prefix / 'bin/lean').resolve()),
               MIR_PROOF_FIRST_PARSER=str(parser_binary), PYTHONPYCACHEPREFIX=str(cache),
               PYTHONDONTWRITEBYTECODE='1')
    return env


def read_bound_model(model, observation=None):
    if observation is None:
        raise ValueError('model producer terminal observation required')
    observer = Path(__file__).resolve().with_name('proof_first_process_observer.py')
    runpy.run_path(str(observer))['validate_observation'](
        observation, model / 'RESULT.json', sha(model / 'proof_first_reference_source_check.py'))
    result = json.loads((model / 'RESULT.json').read_text())
    if (result.get('status') != 'passed' or not result.get('compiled_modules')
            or not result.get('execution_binding')
            or not result.get('host_model_status', '').startswith('passed')
            or not result.get('commands') or any(c['exit'] != 0 for c in result['commands'])):
        raise ValueError('completed bound host model required')
    if sha(model / 'MANIFEST.json') != result['manifest_sha256']:
        raise ValueError('bound model manifest changed')
    for name, record in result['compiled_modules'].items():
        source = model / (name + '.lean')
        if record['exit'] != 0 or record['source'] != str(source):
            raise ValueError('model lacks selected successful source build')
        expected = {str(source): record['source_sha256'], **record['outputs']}
        for path, digest in expected.items():
            if Path(path).parent != model or result['execution_binding'].get(path) != digest or sha(Path(path)) != digest:
                raise ValueError('model source/object binding changed: ' + path)
        if sha(model / record['log']) != record['log_sha256']:
            raise ValueError('model build log changed')
    return result


def main():
    if not __debug__:
        raise SystemExit('preparation requires assertions')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work-root', type=Path, required=True)
    parser.add_argument('--model-run', type=Path, required=True)
    parser.add_argument('--model-observation', type=Path, required=True,
                        help='external terminal-exit observation from proof_first_process_observer.py')
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[1]
    work_root, model = args.work_root.resolve(), args.model_run.resolve()
    if not work_root.is_dir() or work_root.is_relative_to(repo) or model.is_relative_to(repo):
        parser.error('existing external work root and completed model directory required')
    if shutil.disk_usage(work_root).free < 4 * 1024**3:
        parser.error('4 GiB free space required before native preparation')
    parent = read_bound_model(model, args.model_observation.resolve())
    model_observation = json.loads(args.model_observation.read_text())
    layout = checked_toolchain_layout(Path(parent['compiler']['path']).parent.parent,
                                      repo, Path(model_observation['producer']['path']).parents[1])
    work = Path(tempfile.mkdtemp(prefix='mir-w4-host-', dir=work_root))
    print(work, flush=True)
    host = work / 'host'
    rec = host / 'recovered'
    native = host / 'cold-native-2'
    native.mkdir(parents=True)
    rec.mkdir()
    api_path = model / 'proof_first_reference_source_check.py'
    if sha(api_path) != parent['execution_binding'][str(api_path)]:
        raise ValueError('bound model runner source changed')
    audit_helper = model / 'proof_first_composition_source_check.py'
    if sha(audit_helper) != parent['execution_binding'][str(audit_helper)]:
        raise ValueError('bound model audit helper changed')
    api = runpy.run_path(str(api_path))
    observer_path = Path(__file__).resolve().with_name('proof_first_process_observer.py')
    observer_api = runpy.run_path(str(observer_path))
    producer_source = observer_api['archive_source'](__file__, work / Path(__file__).name)
    observer_source = observer_api['archive_source'](observer_path, work / observer_path.name)
    bindings = {str(model / 'RESULT.json'): sha(model / 'RESULT.json'),
                str(Path(__file__).resolve()): sha(Path(__file__).resolve()), str(api_path): sha(api_path),
                str(audit_helper): sha(audit_helper), str(observer_path): sha(observer_path),
                producer_source['path']: producer_source['sha256'],
                observer_source['path']: observer_source['sha256']}
    provenance_only = {str(Path(__file__).resolve()), str(observer_path)}
    retained = set(bindings) - provenance_only
    observation = args.model_observation.resolve()
    bindings[str(observation)] = sha(observation)
    observation_bindings = observer_api['observation_inputs'](json.loads(observation.read_text()))
    bindings.update(observation_bindings)
    retained.update([str(observation), *observation_bindings])
    source_manifest = repo / 'docs/proof-first/W4_HOST_MANIFEST.json'
    manifest = json.loads(source_manifest.read_text())
    bindings[str(source_manifest)] = sha(source_manifest)
    provenance_only.add(str(source_manifest))
    frozen_manifest = work / source_manifest.name
    frozen_manifest.write_bytes(source_manifest.read_bytes())
    if sha(frozen_manifest) != bindings[str(source_manifest)]:
        raise ValueError('host manifest changed while archiving')
    bindings[str(frozen_manifest)] = sha(frozen_manifest)
    copied, builds = [], {}
    result = dict(status='running', scope=__doc__, started=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  workdir=str(work), model=str(model), producer_source=producer_source,
                  toolchain_layout=layout,
                  commands=[], generated_consumers=[], native_binaries={})

    def save():
        (work / 'RESULT.json').write_text(json.dumps(result, indent=2) + '\n')

    def bind(path, expected=None):
        digest = sha(path)
        if expected is not None and digest != expected:
            raise ValueError('preparation input differs: ' + str(path))
        if str(path) in bindings and bindings[str(path)] != digest:
            raise ValueError('preparation binding changed: ' + str(path))
        bindings[str(path)] = digest
        return digest

    def copy(source, target, digest):
        bind(source, digest)
        if target.exists():
            raise ValueError('staging would overwrite existing input: ' + str(target))
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(source.read_bytes())
        bind(target, digest)
        copied.append(dict(source=str(source), target=str(target), sha256=digest))

    def command(args, label, cwd=work, expected_exit=0):
        api['verify_integrity'](bindings)
        completed = subprocess.run(list(map(str, args)), cwd=cwd, env=env, capture_output=True,
                                   text=True, preexec_fn=api['memory_limit'])
        output = completed.stdout + completed.stderr
        log = work / (label + '.log')
        with log.open('x') as f:
            f.write(output)
        result['commands'].append(dict(command=list(map(str, args)), cwd=str(cwd), exit=completed.returncode,
                                       log=str(log), log_sha256=bind(log)))
        save()
        if completed.returncode != expected_exit or 'sorryAx' in output:
            raise RuntimeError('preparation command failed: ' + str(log))
        api['verify_integrity'](bindings)
        return output

    def lean(source, label, output=False, run=False, cwd=work, c_output=None):
        bind(source)
        # IO consumers may use a different cwd; their module identity must still
        # match the flat import name used by the selected source closure.
        args = [compiler, '--trust=0', '-j1', '-R', source.parent]
        if output:
            obj = source.with_suffix('.olean')
            if obj.exists():
                raise ValueError('fresh object would overwrite prior output')
            args += ['-o', obj]
        if c_output is not None:
            args += ['-c', c_output]
        if run:
            args.append('--run')
        args.append(source)
        text = command(args, label, cwd)
        if output:
            if not obj.is_file():
                raise ValueError('compiler omitted native object')
            outputs = {str(p): bind(p) for p in api['lean_outputs'](obj)}
            builds[source.stem] = dict(exit=0, source=str(source), source_sha256=bind(source), outputs=outputs)
        if c_output is not None:
            bind(c_output)
        return text

    try:
        # Only successful generated consumers/model modules are copied; each
        # object retains its producing source and parent execution receipt.
        for name, row in parent['compiled_modules'].items():
            source = model / (name + '.lean')
            copy(source, work / source.name, row['source_sha256'])
            outputs = {}
            for path, digest in row['outputs'].items():
                target = work / Path(path).name
                copy(Path(path), target, digest)
                outputs[str(target)] = digest
            builds[name] = dict(exit=0, source=str(work / source.name), source_sha256=row['source_sha256'], outputs=outputs)
        for name in ['proof_first_reference_source.py', 'proof_first_composition_source.py']:
            copy(model / name, work / name, parent['execution_binding'][str(model / name)])
        for row in manifest['sources']:
            source, target = host_source_paths(repo, work, row['target'])
            copy(source, target, row['sha256'])
            provenance_only.add(str(source))
        copy(work / 'HostControlHeader.lean', rec / 'HostControlHeader.lean', sha(work / 'HostControlHeader.lean'))
        # Preparers bind the current decoder sources, never a saved old reader.
        for name in ['WriterJournalCaptureReplay', 'LeaseCohortCaptureReplay', 'LeaseCohortCaptureJson',
                     'LeaseKnownFaultCaptureReplay', 'LeaseKnownFaultCaptureJson',
                     'LeaseWireCaptureReplay', 'LeaseWireCaptureJson']:
            copy(work / (name + '.lean'), rec / (name + '.lean'), sha(work / (name + '.lean')))
        compiler = Path(parent['compiler']['path'])
        prefix = compiler.parent.parent
        leanc = compiler.with_name('leanc')
        bind(compiler, parent['compiler']['sha256'])
        bind(leanc)
        for name in ['clang', 'ld.lld']:
            bind(prefix / 'bin' / name)
        for directory in [prefix / 'include', prefix / 'lib/clang']:
            for p in directory.rglob('*'):
                if p.is_file():
                    bind(p)
        for path, digest in parent['execution_binding'].items():
            if Path(path).is_relative_to(prefix):
                bind(Path(path), digest)
        for p in (prefix / 'lib/lean').glob('*.a'):
            bind(p)
        for p in (prefix / 'lib/lean').glob('*.so'):
            bind(p)
        parser_binary = model / 'cargo-target/debug/examples/textual_mir_alpha_parse'
        bind(parser_binary, parent['parser_sha256'])
        cache = work / 'empty-python-cache'
        cache.mkdir()
        env = native_environment(os.environ, work, prefix, cache, parser_binary)
        result['native_toolchain'] = dict(sysroot=str(prefix), compiler=str(prefix / 'bin/clang'),
                                           linker=str(prefix / 'bin/ld.lld'),
                                           scope='selected bundled compiler/linker/headers and Lean libraries bound; OS loader/system libraries remain TCB')
        entries = {'source': 'SourceFundingQueryWorker', 'owner': 'OwnerBudgetWorker'}
        closures = {kind: api['local_import_closure']([name], work) for kind, name in entries.items()}
        order, seen = [], set()

        def visit(name):
            if name.split('.')[0] in {'Init', 'Lean', 'Std'} or name in seen:
                return
            for line in (work / (name + '.lean')).read_text().splitlines():
                if line.startswith('import '):
                    for dep in line.split()[1:]:
                        visit(dep)
            seen.add(name)
            order.append(name)

        for name in entries.values():
            visit(name)
        for name in order:
            copy(work / (name + '.lean'), native / (name + '.lean'), sha(work / (name + '.lean')))
        native_manifest = dict(entries=entries, modules=order, compiler_sha256=sha(compiler),
                               leanc_sha256=sha(leanc), sources={n: sha(native / (n + '.lean')) for n in order})
        (native / 'MANIFEST.json').write_text(json.dumps(native_manifest, indent=2) + '\n')
        bind(native / 'MANIFEST.json')
        env['LEAN_PATH'] = str(native)
        for name in order:
            lean(native / (name + '.lean'), name + '-native-lean', output=True, c_output=native / (name + '.c'))
            command([leanc, '-O0', '-c', native / (name + '.c'), '-o', native / (name + '.o')], name + '-native-c')
            bind(native / (name + '.o'))
        binaries = {}
        for kind, names in closures.items():
            binary = native / (kind + '-worker')
            command([leanc, '-o', binary, *[native / (n + '.o') for n in order if n in names]], kind + '-link')
            binaries[kind] = dict(path=str(binary), sha256=bind(binary), modules=len(names))
        # Each entry exports main; inspect them in separate environments.
        # Their shared proof sources match the bound model byte for byte.
        for kind, entry in entries.items():
            audit = native / (kind + 'EntryAudit.lean')
            audit.write_text(api['audit_source']([entry]) + api['IMPORT_INSPECTOR'])
            out = lean(audit, kind + '-native-entry-audit')
            counts = re.findall(r'AXIOM_AUDIT_OK (\w+) (\d+)', out)
            if len(counts) != 1 or counts[0][0] != entry:
                raise ValueError('native entry declaration audit incomplete')
            bound = api['validate_lean_imports'](out, closures[kind], builds, native, prefix)
            for path, digest in bound.items():
                bind(Path(path), digest)
        native_result = dict(status='passed', scope='fresh native build only; captures not run',
                             manifest_sha256=sha(native / 'MANIFEST.json'), binaries=binaries,
                             builds={name: builds[name] for name in order})
        (native / 'RESULT.json').write_text(json.dumps(native_result, indent=2) + '\n')
        bind(native / 'RESULT.json')
        for kind in entries:
            p = rec / ('REBUILT_' + kind.upper() + '_BUILD.json')
            p.write_text(json.dumps(dict(binary=binaries[kind]['path'], binary_sha256=binaries[kind]['sha256'],
                                        cold_build=str(native / 'RESULT.json'), cold_build_sha256=sha(native / 'RESULT.json')), indent=2) + '\n')
            bind(p)
        env['LEAN_PATH'] = str(work)
        command([sys.executable, '-E', '-B', '-X', 'pycache_prefix=' + str(cache),
                 host / 'prepare_source_inputs.py'], 'prepare-source')
        prepared = json.loads((host / 'SOURCE_PREPARATION.json').read_text())
        bind(host / 'SOURCE_PREPARATION.json')
        for row in prepared['generated']:
            bind(Path(row['path']), row['sha256'])
        lean(work / 'ParsedQualified.lean', 'qualified-source', output=True, run=True)
        lean(work / 'SourceInputControls.lean', 'source-inputs', output=True, cwd=rec)
        for name in ['SourceContinuationC', 'SourceContinuationA']:
            lean(work / (name + '.lean'), name)
        for p in [rec / 'source-bootstrap-actual.bin', rec / 'source-launch-actual.bin',
                  rec / 'source-continuation/continue.bin', rec / 'source-continuation-owner-a/continue.bin']:
            bind(p)
        if list(cache.rglob('*')):
            raise ValueError('source preparation wrote Python bytecode')
        api['verify_integrity'](bindings)
        consumed = observer_api['consumed_bindings'](bindings, provenance_only, retained)
        reused_toolchain = {p: h for p, h in bindings.items() if Path(p).is_relative_to(prefix)}
        if any(consumed.get(p) != h for p, h in reused_toolchain.items()):
            raise ValueError('consumed toolchain binding omitted from preparation')
        result.update(status='prepared; physical capture/replay NOT RUN', native_binaries=binaries,
                      generated_consumers=prepared['generated'], copied=copied, bindings=consumed,
                      export_roles=dict(provenance_only=sorted(provenance_only), retained=sorted(retained)),
                      reused_toolchain_bindings=reused_toolchain,
                      provenance_bindings={p: h for p, h in bindings.items() if p not in consumed})
    except BaseException as error:
        result.update(status='failed', error=str(error))
        raise
    finally:
        result['finished'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
        save()
    print('fresh workers and actual source inputs prepared; physical validation remains required', flush=True)


if __name__ == '__main__':
    main()
