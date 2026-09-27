"""Rebuild the finite W3 reference/source candidate in an external work directory.

This is nonproduction mechanization evidence, not Canon or alpha acceptance.
No existing olean or historical parser executable is consumed.
"""
from pathlib import Path
import argparse
import datetime
import hashlib
import json
import os
import re
import resource
import runpy
import shutil
import subprocess
import sys
import tempfile

# Load the inspected source, never a timestamp-compatible cached module.
audit_source = runpy.run_path(str(Path(__file__).resolve().with_name(
    'proof_first_composition_source_check.py')))['audit_source']


FULL_CASES = frozenset(['changed-real-function', 'missing-lineage', 'false-lineage', 'unrelated-terminal', 'reference-as-integer', 'reference-as-instance', 'shadow-reference', 'mutable-reference', 'wrong-provider-type', 'wrong-reference-boundary', 'missing-lease', 'expired-lease', 'released-alias', 'source-pause-revoke-recover', 'source-normalized-then-denied', 'source-no-double-completion', 'source-hold-loss-before-call', 'source-hold-loss-while-pending', 'source-fallback-preserves-hold-origin', 'source-release-dead-hold', 'source-cannot-create-holding-authority', 'source-cancel-recheck-second-source', 'source-cancel-needs-current-authority', 'source-no-cancel-for-committed-normalization-only', 'source-identity-crlf', 'source-identity-multibyte', 'oracle-unrestricted-invocation-resurrection', 'oracle-admissible-head-fresh-evidence-recovery', 'source-session-continue-addition', 'oracle-c-locus-rejoin-current-claim', 'oracle-c-new-member-new-claim', 'source-cancel-expired-repair-stays-failed'])
SESSION_CASES = frozenset([
    "source-cancel-recheck-second-source", "source-session-continue-addition",
    "source-cancel-expired-repair-stays-failed",
])
ADAPTER_REJECTION_CASES = frozenset([
    'mutable-reference', 'wrong-provider-type', 'wrong-reference-boundary', 'missing-lease',
])

def validate_source_cases(record):
    """Full evidence needs exact identities and scope, not a generic passed flag."""
    if record.get("status") != "passed" or record.get("filter") is not None:
        raise ValueError("filtered/incomplete source controls")
    rows = record.get("cases", [])
    names = [row.get("name") for row in rows]
    if len(names) != len(FULL_CASES) or set(names) != FULL_CASES:
        raise ValueError("missing, extra or duplicate source case")
    for row in rows:
        expected = "admitted-session" if row["name"] in SESSION_CASES else "source-component-control"
        if row.get("scope") != expected:
            raise ValueError("incorrect source case scope")
        if row.get("parser_exit") != 0:
            raise ValueError("unexecuted or failed source case")
        if row['name'] in ADAPTER_REJECTION_CASES:
            if row.get('adapter_rejected') is not True or 'lean_exit' in row or not row.get('reason'):
                raise ValueError('required adapter rejection missing or ambiguous')
        elif 'adapter_rejected' in row or row.get('lean_exit') != 0:
            # These consumers use #guard to assert the selected semantic result,
            # including rejection. Their Lean process must itself succeed.
            raise ValueError('required Lean semantic assertions not completed')


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


HOST_MODEL_ROOTS = [
    'LeaseCohortCaptureJson', 'LeaseKnownFaultCaptureJson', 'LeaseWireCaptureJson',
    'MirroreaProofFirstEntryAcquisition', 'ProducedRootedness',
]


def host_model_modules(repo):
    """Check the preserved source cut, not its theorems or physical provenance."""
    record = json.loads((repo / 'docs/proof-first/W4_SOURCE_MANIFEST.json').read_text())
    if record['roots'] != HOST_MODEL_ROOTS:
        raise ValueError('unexpected host model roots')
    sources = {}
    for row in record['sources']:
        name = row['module']
        if (not re.fullmatch(r'[A-Za-z_]\w*', name) or name in {'Lean', 'Std'}
                or name in sources):
            raise ValueError('invalid or duplicate host module')
        expected = 'samples/lean/foundations/' + name + '.lean'
        if row['target'] != expected:
            raise ValueError('host source path mismatch')
        path = repo / expected
        if (not path.is_file() or path.is_symlink()
                or not path.resolve().is_relative_to((repo / 'samples/lean/foundations').resolve())
                or sha(path) != row['sha256']):
            raise ValueError('host source differs from preserved cut: ' + name)
        sources[name] = path.read_text()
    ordered, visiting = [], set()

    def visit(name):
        if name in {'Lean', 'Std'} or name.startswith(('Lean.', 'Std.')) or name in ordered:
            return
        if name in visiting or name not in sources:
            raise ValueError('cyclic or incomplete host source closure: ' + name)
        visiting.add(name)
        for line in sources[name].splitlines():
            if line.startswith('import '):
                match = re.fullmatch(r'import ([A-Za-z_]\w*(?:\.[A-Za-z_]\w*)*)', line)
                if not match:
                    raise ValueError('unsupported import in preserved source: ' + name)
                visit(match[1])
        visiting.remove(name)
        ordered.append(name)

    for name in HOST_MODEL_ROOTS:
        visit(name)
    if set(ordered) != set(sources):
        raise ValueError('unreachable source in host manifest')
    return ordered


def verified_bytes(path, expected):
    """Consume the bytes compared with the producer receipt, without rereading."""
    data = path.read_bytes()
    if hashlib.sha256(data).hexdigest() != expected:
        raise ValueError('generated input changed: ' + str(path))
    return data


def freeze_consumer(source, expected, target, namespace=None):
    data = verified_bytes(source, expected)
    if namespace is not None:
        lines = data.decode('utf-8').splitlines(keepends=True)
        at = next(i for i, line in enumerate(lines) if not line.startswith('import '))
        data = (''.join(lines[:at]) + 'namespace ' + namespace + '\n' +
                ''.join(lines[at:]) + '\nend ' + namespace + '\n').encode('utf-8')
    target.write_bytes(data)
    return hashlib.sha256(data).hexdigest()


def verify_integrity(entries):
    for path, expected in entries.items():
        verified_bytes(Path(path), expected)


IMPORT_INSPECTOR = '''open Lean in
run_cmd do
  let env ← getEnv
  for name in env.header.moduleNames do
    let file ← findOLean name
    logInfo m!"BOUND_MODULE {name} {file}"
'''


def require_completed(completed, label):
    output = completed.stdout + completed.stderr
    if completed.returncode or 'sorryAx' in output:
        raise ValueError('failed command; inspect ' + label)
    return output


def lean_outputs(path):
    return [p for p in [path, path.with_suffix('.olean.private'),
                       path.with_suffix('.olean.server'), path.with_suffix('.ir')]
            if p.exists()]


def validate_lean_imports(output, expected_local, builds, work, toolchain):
    """Bind this consumer's resolved imports to successful source/object builds.

    The compiler, toolchain libraries and stable non-adversarial filesystem are
    trusted. This is not a proof of the compiler, OS custody or authentication.
    """
    rows = [line.split(' ', 2)[1:] for line in output.splitlines()
            if line.startswith('BOUND_MODULE ')]
    if not rows or any(len(row) != 2 for row in rows):
        raise ValueError('missing/malformed import inventory')
    names = [name for name, _ in rows]
    if len(names) != len(set(names)):
        raise ValueError('duplicate import inventory')
    local, bindings = set(), {}
    for name, path in rows:
        if not re.fullmatch(r'[A-Za-z_]\w*(?:\.[A-Za-z_]\w*)*', name):
            raise ValueError('unsupported import name')
        p = Path(path)
        if name == 'Init' or name.startswith('Init.') or name == 'Lean' or name.startswith('Lean.') or name == 'Std' or name.startswith('Std.'):
            expected = toolchain / 'lib/lean' / (name.replace('.', '/') + '.olean')
            if p != expected or not p.is_file() or p.is_symlink():
                raise ValueError('toolchain import path mismatch: ' + name)
            bindings.update({str(q): sha(q) for q in lean_outputs(p)})
        else:
            local.add(name)
            if p != work / (name + '.olean') or name not in builds:
                raise ValueError('local import path lacks selected build: ' + name)
            record = builds[name]
            source = work / (name + '.lean')
            if record['exit'] != 0 or record['source'] != str(source):
                raise ValueError('import lacks successful source build: ' + name)
            files = lean_outputs(p)
            if not p.is_file() or any(q.is_symlink() for q in files):
                raise ValueError('local import path is missing or indirect: ' + name)
            if set(record['outputs']) != {str(q) for q in files}:
                raise ValueError('compiled output inventory changed: ' + name)
            entries = {str(source): record['source_sha256'], **record['outputs']}
            verify_integrity(entries)
            bindings.update(entries)
    if local != set(expected_local):
        raise ValueError('local import inventory differs from selected source closure')
    return bindings


def local_import_closure(roots, work):
    seen = set()

    def visit(name):
        if name.split('.')[0] in {'Init', 'Lean', 'Std'} or name in seen:
            return
        seen.add(name)
        for line in (work / (name + '.lean')).read_text().splitlines():
            if line.startswith('import '):
                for dependency in line.split()[1:]:
                    visit(dependency)

    for name in roots:
        visit(name)
    return seen


CONSUMER_CHECKS = '''
-- These names must belong to the actual freshly generated consumers.
#check admittedLaunch
#check admittedSession
#check ActualRecovery.repairedSession
#check ActualRecovery.repairedSessionInvariants
#check ActualAddition.continuedSession
#check ActualAddition.continuedInvariants
#check ActualExpiredRepair.repairedSession
'''



def tool_environment(environ, compiler):
    """One selected absolute compiler for direct and owned nested consumers."""
    compiler = Path(compiler).resolve()
    env = {k: v for k, v in environ.items() if k not in {'LEAN_CC', 'LEAN_SYSROOT'}}
    env.update(MIR_PROOF_FIRST_LEAN=str(compiler), LEAN_SYSROOT=str(compiler.parent.parent))
    return env


def select_compiler(environ):
    """Check the ultimately selected compiler under its execution environment."""
    discovery = {k: v for k, v in environ.items() if k not in {'LEAN_CC', 'LEAN_SYSROOT'}}
    prefix = Path(subprocess.check_output(['lean', '--print-prefix'], env=discovery,
                                         text=True).strip()).resolve()
    compiler = (prefix / 'bin/lean').resolve()
    before = sha(compiler)
    selected = tool_environment(discovery, compiler)
    version = subprocess.check_output([str(compiler), '--version'], env=selected, text=True).strip()
    if sha(compiler) != before or 'version 4.29.1,' not in version:
        raise ValueError('selected unchanged Lean4.29.1 required for this evidence profile')
    return compiler, version


def memory_limit():
    resource.setrlimit(resource.RLIMIT_AS, (4 * 1024**3, 4 * 1024**3))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def main():
    if not __debug__:
        raise SystemExit('optimized Python is not an evidence execution mode')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work-root', type=Path, required=True,
                        help='existing directory outside the repository')
    parser.add_argument('--with-publication', action='store_true',
                        help='also check the nonproduction W4 publication model; no network claim')
    parser.add_argument('--with-owner-boundary', action='store_true',
                        help='also check W4 qualified owner/codec/resource proofs; implies publication, no native/network claim')
    parser.add_argument('--with-host-model', action='store_true',
                        help='also rebuild the preserved W4 host correspondence proof cone; implies owner boundary, no fresh native/network evidence')
    args = parser.parse_args()
    if args.with_host_model:
        args.with_owner_boundary = True
    if args.with_owner_boundary:
        args.with_publication = True
    repo = Path(__file__).resolve().parents[1]
    work_root = args.work_root.resolve()
    if not work_root.is_dir() or work_root.is_relative_to(repo):
        parser.error('--work-root must be an existing directory outside the repository')
    if shutil.disk_usage(work_root).free < 2 * 1024**3:
        parser.error('at least 2 GiB free space required before the bounded build')
    if args.with_host_model:
        selected_compiler, version = select_compiler(os.environ)
    else:
        version = subprocess.check_output(['lean', '--version'], text=True).strip()
    if 'version 4.29.1,' not in version:
        raise SystemExit('Lean4.29.1 required for this evidence profile')
    work = Path(tempfile.mkdtemp(prefix='mir-w3-reference-', dir=work_root))
    print(work, flush=True)
    foundation = repo / 'samples/lean/foundations'
    modules, visiting = [], set()

    def visit(name):
        if args.with_publication and name == 'ActualReference':
            # This dependency is produced from the freshly built parser below.
            return
        if name in modules:
            return
        if name in visiting:
            raise ValueError('cyclic Lean import: ' + name)
        visiting.add(name)
        source = foundation / (name + '.lean')
        text = source.read_text()
        for dependency in re.findall(r'^import (\w+)$', text, re.M):
            if dependency not in {'Lean', 'Std'}:
                visit(dependency)
        visiting.remove(name)
        modules.append(name)
        shutil.copy2(source, work / source.name)

    roots = sorted(p.stem for p in foundation.glob('MirroreaProofFirstReference*.lean'))
    if not roots:
        raise ValueError('no reference proof modules')
    for name in roots + ['MirroreaProofFirstSourceFunction']:
        visit(name)
    reference_module_count = len(modules)
    if args.with_publication:
        publication_roots = sorted(p.stem for p in foundation.glob('MirroreaProofFirstPublication*.lean'))
        if not publication_roots:
            raise ValueError('no publication proof modules')
        for name in publication_roots + ['MirroreaProofFirstReceivedResultControls']:
            visit(name)
        if args.with_owner_boundary:
            for name in ['MirroreaProofFirstOwnerEndpointBudget', 'MirroreaProofFirstRoutedOwner',
                         'MirroreaProofFirstSourceFundingAdministration',
                         'MirroreaProofFirstSourceFundingCheckedWork',
                         'MirroreaProofFirstSourceRegistration']:
                visit(name)
            if args.with_host_model:
                for name in host_model_modules(repo):
                    visit(name)
    source_scripts = ['scripts/proof_first_composition_source.py',
                      'scripts/proof_first_composition_source_check.py',
                      'scripts/proof_first_reference_source_check.py',
                      'scripts/proof_first_reference_source.py',
                      'scripts/tests/proof_first_reference_source_cases.py',
                      'scripts/tests/proof_first_reference_integrity_cases.py',
                      'scripts/tests/proof_first_reference_mutants.py']
    if args.with_publication:
        source_scripts.append('scripts/tests/proof_first_publication_mutants.py')
    for name in source_scripts:
        shutil.copy2(repo / name, work / Path(name).name)
    shutil.copy2(repo / 'samples/clean-near-end/mirrorea-proof-first-composition/reference.mir',
                 work / 'reference.mir')
    # Pin the complete local parser crate, workspace manifest/lock, and harness,
    # rather than just the CLI example. Dependencies are locked, offline Cargo.
    parser_sources = sorted(p for p in (repo / 'crates/mir-ast').rglob('*')
                            if p.is_file() and 'target' not in p.parts)
    repo_inputs = parser_sources + [repo / 'Cargo.toml', repo / 'Cargo.lock',
                                   Path(__file__).resolve(),
                                   repo / 'scripts/proof_first_composition_source_check.py']
    if args.with_host_model:
        repo_inputs.append(repo / 'docs/proof-first/W4_SOURCE_MANIFEST.json')
    manifest = {str(p.relative_to(work)): sha(p) for p in sorted(work.iterdir()) if p.is_file()}
    manifest.update({'repo:' + str(p.relative_to(repo)): sha(p) for p in repo_inputs})
    (work / 'MANIFEST.json').write_text(json.dumps(manifest, indent=2) + '\n')
    result = dict(classification='finite W3 LAB source/reference candidate; no distributed or durable claim',
                  status='running', started=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  workdir=str(work), lean_version=version, rlimit_as=4 * 1024**3,
                  manifest_sha256=sha(work / 'MANIFEST.json'), reference_modules=len(roots), commands=[])
    if args.with_publication:
        result['classification'] = 'W3 reference plus W4 LAB publication model; no physical/distributed/durable claim'
        result['publication_status'] = 'running'
    if args.with_owner_boundary:
        result['classification'] = 'W3 reference plus W4 qualified owner/codec/resource proof model; no native/distributed/durable claim'
        result['owner_boundary_status'] = 'running'
    if args.with_host_model:
        result['classification'] = 'W3 source checks plus preserved W4 host correspondence model; no fresh native/network/durable evidence or W4-B acceptance'
        result['host_model_status'] = 'running'
    env = dict(os.environ, LEAN_PATH=str(work), CARGO_BUILD_JOBS='1', CARGO_INCREMENTAL='0',
               CARGO_TARGET_DIR=str(work / 'cargo-target'),
               MIR_PROOF_FIRST_PARSER=str(work / 'cargo-target/debug/examples/textual_mir_alpha_parse'))
    # A caller's focused filter must never turn the full acceptance command green.
    env.pop('MIR_W3_CASE_FILTER', None)
    integrity = {str(work / 'MANIFEST.json'): result['manifest_sha256']}
    builds = {}
    if args.with_host_model:
        compiler = selected_compiler
        toolchain = compiler.parent.parent
        env = tool_environment(env, compiler)
        integrity[str(Path(sys.executable).resolve())] = sha(Path(sys.executable).resolve())
        integrity[str(compiler)] = sha(compiler)
        for name, expected in manifest.items():
            path = repo / name[5:] if name.startswith('repo:') else work / name
            integrity[str(path)] = expected
        # -B prevents writes, not reads. Every child gets a fresh cache location;
        # the parent audit helper above is executed directly from source.
        env = {k: v for k, v in env.items() if not k.startswith('PYTHON')}
        cache = work / 'empty-python-cache'
        cache.mkdir()
        env.update(PYTHONPYCACHEPREFIX=str(cache), PYTHONDONTWRITEBYTECODE='1')
        result['compiler'] = dict(path=str(compiler), sha256=sha(compiler), version=version,
                                  version_command=[str(compiler), '--version'])

    def seal(path, expected=None):
        data = path.read_bytes() if expected is None else verified_bytes(path, expected)
        integrity[str(path)] = hashlib.sha256(data).hexdigest()
        return data

    def record():
        (work / 'RESULT.json').write_text(json.dumps(result, indent=2) + '\n')

    def run(command, log, cwd=work):
        output_path = None
        if args.with_host_model and command[0] == 'lean':
            command = [str(compiler), '-j1', *command[1:]]
            if '-o' in command:
                output_path = cwd / command[command.index('-o') + 1]
                if lean_outputs(output_path):
                    raise ValueError('fresh compilation has existing output: ' + str(output_path))
        if args.with_host_model:
            verify_integrity(integrity)
            if command[0] == 'python3':
                command = [sys.executable, '-E', '-B', '-X', 'pycache_prefix=' + str(cache), *command[1:]]
        completed = subprocess.run(command, cwd=cwd, env=env, capture_output=True,
                                   text=True, preexec_fn=memory_limit)
        output = completed.stdout + completed.stderr
        (work / log).write_text(output)
        result['commands'].append(dict(command=command, cwd=str(cwd), exit=completed.returncode,
                                       log=log, log_sha256=sha(work / log)))
        record()
        require_completed(completed, str(work / log))
        if args.with_host_model:
            verify_integrity(integrity)
            seal(work / log)
            if output_path is not None:
                if not output_path.is_file():
                    raise ValueError('successful compiler omitted required object')
                source = cwd / command[-1]
                entry = dict(exit=completed.returncode, source=str(source),
                             source_sha256=sha(source), outputs={})
                for p in lean_outputs(output_path):
                    seal(p)
                    entry['outputs'][str(p)] = integrity[str(p)]
                seal(source)
                entry.update(command=command, log=log, log_sha256=sha(work / log))
                builds[output_path.stem] = entry
                result['compiled_modules'] = builds
                record()
        return output

    record()
    try:
        if args.with_host_model:
            # Pin standard-library objects before any selected model compiles.
            probe = work / 'ToolchainImports.lean'
            probe.write_text('import Lean\nimport Std\n' + IMPORT_INSPECTOR)
            seal(probe)
            output = run(['lean', '--trust=0', probe.name], 'toolchain-imports.log')
            integrity.update(validate_lean_imports(output, set(), {}, work, toolchain))
        run(['cargo', 'build', '--locked', '--offline', '-p', 'mir-ast', '--example',
             'textual_mir_alpha_parse', '-j', '1'], 'parser-build.log', repo)
        result['parser_sha256'] = sha(Path(env['MIR_PROOF_FIRST_PARSER']))
        if args.with_host_model:
            seal(Path(env['MIR_PROOF_FIRST_PARSER']))
        for name in modules[:reference_module_count]:
            run(['lean', '--trust=0', '-o', name + '.olean', name + '.lean'], name + '.log')
        print('Fresh proof modules:', len(modules), flush=True)
        run(['python3', 'proof_first_reference_source.py'], 'actual-source.log')
        source_run = Path(seal(work / 'CURRENT_RUN').decode().strip()).resolve()
        if not source_run.is_relative_to(work):
            raise ValueError('actual source run escaped work directory')
        main_receipt = json.loads(seal(source_run / 'ACTUAL_REFERENCE.json'))
        if main_receipt['parser_exit'] != 0 or main_receipt['lean_exit'] != 0:
            raise ValueError('main consumer did not execute successfully')
        main_original = source_run / 'ActualReference.lean'
        main_frozen = work / 'ActualReference.lean'
        integrity[str(main_original)] = main_receipt['generated_sha256']
        integrity[str(main_frozen)] = freeze_consumer(
            main_original, main_receipt['generated_sha256'], main_frozen)
        run(['python3', 'proof_first_reference_source_cases.py'], 'source-cases.log')
        cases = Path(seal(work / 'CURRENT_CASES').decode().strip()).resolve()
        if not cases.is_relative_to(work):
            raise ValueError('source cases escaped work directory')
        case_result = json.loads(seal(cases / 'RESULT.json'))
        validate_source_cases(case_result)
        result['source_cases'] = len(case_result['cases'])
        result['source_cases_sha256'] = sha(cases / 'RESULT.json')
        case_rows = {row['name']: row for row in case_result['cases']}
        for row in case_rows.values():
            if 'generated_sha256' in row:
                seal(cases / row['name'] / 'ActualCase.lean', row['generated_sha256'])
        # Freeze the actual multi-source session consumers for the complete
        # declaration audit. Namespace wrapping is the sole transformation.
        consumers = ['ActualReference']
        generated = {}
        for name, case in [('ActualRecovery', 'source-cancel-recheck-second-source'),
                           ('ActualAddition', 'source-session-continue-addition'),
                           ('ActualExpiredRepair', 'source-cancel-expired-repair-stays-failed')]:
            source = cases / case / 'ActualCase.lean'
            target = work / (name + '.lean')
            expected = case_rows[case]['generated_sha256']
            frozen_sha = freeze_consumer(source, expected, target, name)
            integrity[str(target)] = frozen_sha
            generated[name] = dict(source=str(source.relative_to(work)), source_sha256=expected,
                                   frozen_sha256=frozen_sha, transform='verified bytes; namespace wrapping only')
            consumers.append(name)
        result['generated_consumers'] = generated
        for name in consumers:
            run(['lean', '--trust=0', '-o', name + '.olean', name + '.lean'], name + '-compiled.log')
        if args.with_publication:
            for name in modules[reference_module_count:]:
                run(['lean', '--trust=0', '-o', name + '.olean', name + '.lean'], name + '.log')
            for stem, module, namespace in [
                    ('Source', 'PublicationSourceControls', 'PublicationSourceControl'),
                    ('Outcome', 'PublicationOutcomeControls', 'PublicationOutcomeControls'),
                    ('Received', 'ReceivedResultControls', 'ReceivedResultControls'),
                    ('Execution', 'PublicationExecutionControls', 'PublicationExecutionControls'),
                    ('Projection', 'PublicationProjectionControls', 'PublicationProjectionControls')]:
                driver = work / ('RunPublication' + stem + '.lean')
                driver.write_text('import MirroreaProofFirst' + module + '\n'
                                  'def main := MirroreaProofFirst.' + namespace + '.main\n'
                                  '#print axioms main\n')
                seal(driver)
                run(['lean', '--trust=0', '--run', driver.name], driver.stem + '.log')
            run(['python3', 'proof_first_publication_mutants.py'], 'publication-mutants.log')
            publication_mutants = json.loads(seal(work / 'publication-mutants/RESULT.json'))
            if len(publication_mutants) != 8 or not all(row['rejected_at_theorem'] for row in publication_mutants):
                raise ValueError('publication mutation controls incomplete')
            result['publication_modules'] = modules[reference_module_count:]
            result['publication_proof_mutants'] = len(publication_mutants)
        audited = modules + consumers
        # Bound each audit's working set without dropping any owned module.
        groups = ([modules[i:i + 8] for i in range(0, len(modules), 8)] + [consumers]
                  if args.with_host_model else [audited])
        counts = []
        for i, group in enumerate(groups):
            # The existing consumer-integrity controls inspect CompleteAudit.
            # Keep that final contract while batching the preceding model audit.
            name = 'CompleteAudit' if i == len(groups) - 1 else 'CompleteAudit' + str(i)
            checks = CONSUMER_CHECKS if i == len(groups) - 1 else ''
            inspector = IMPORT_INSPECTOR if args.with_host_model else ''
            (work / (name + '.lean')).write_text(audit_source(group) + checks + inspector)
            seal(work / (name + '.lean'))
            output = run(['lean', '--trust=0', name + '.lean'], name + '.log')
            if args.with_host_model:
                bound = validate_lean_imports(output, local_import_closure(group, work),
                                              builds, work, toolchain)
                for path, digest in bound.items():
                    if path in integrity and integrity[path] != digest:
                        raise ValueError('import changed during execution: ' + path)
                    integrity[path] = digest
            current = re.findall(r'AXIOM_AUDIT_OK (\w+) (\d+)', output)
            if len(current) != len(group) or {n for n, _ in current} != set(group):
                raise ValueError('missing, extra or duplicate audited module')
            counts.extend(current)
        if len(counts) != len(audited) or {name for name, _ in counts} != set(audited):
            raise ValueError('missing or extra audited module')
        # This test mutates only its own copies of real generated consumers.
        integrity_result = json.loads(run(['python3', 'proof_first_reference_integrity_cases.py',
                                          '--evidence-dir', str(work)], 'reference-integrity.log'))
        if integrity_result['status'] != 'passed':
            raise ValueError('integrity/consumer controls incomplete')
        result['integrity_controls'] = len(integrity_result['controls'])
        run(['python3', 'proof_first_reference_mutants.py'], 'reference-mutants.log')
        mutants = json.loads((work / 'reference-mutants/RESULT.json').read_text())
        if mutants['status'] != 'passed':
            raise ValueError('weakening controls incomplete')
        result['weakening_controls'] = len(mutants['mutations'])
        result['weakening_controls_sha256'] = sha(work / 'reference-mutants/RESULT.json')
        for name, expected in manifest.items():
            path = repo / name[5:] if name.startswith('repo:') else work / name
            if sha(path) != expected:
                raise ValueError('input changed during execution: ' + name)
        verify_integrity(integrity)
        if args.with_host_model and list(cache.rglob('*')):
            raise ValueError('evidence child wrote Python bytecode')
        result['generated_integrity'] = {str(Path(path).relative_to(work)): value
                                         for path, value in integrity.items()
                                         if Path(path).is_relative_to(work)}
        if args.with_host_model:
            result['execution_binding'] = integrity
        result.update(status='passed', audit_modules=len(counts),
                      owned_declarations=sum(int(count) for _, count in counts))
        if args.with_publication:
            result['publication_status'] = 'passed local model only'
        if args.with_owner_boundary:
            result['owner_boundary_status'] = 'passed general model proofs; native resource/source custody remains separate'
        if args.with_host_model:
            result['host_model_status'] = 'passed preserved model proofs; physical capture/replay binding and W4-B closure remain separate'
    except Exception as error:
        result.update(status='failed', error=str(error))
        raise
    finally:
        result['finished'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
        record()
    print(json.dumps({key: value for key, value in result.items() if key != 'commands'}), flush=True)


if __name__ == '__main__':
    main()
