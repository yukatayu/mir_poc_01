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
import shutil
import subprocess
import tempfile

from proof_first_composition_source_check import audit_source


FULL_CASES = frozenset(['changed-real-function', 'missing-lineage', 'false-lineage', 'unrelated-terminal', 'reference-as-integer', 'reference-as-instance', 'shadow-reference', 'mutable-reference', 'wrong-provider-type', 'wrong-reference-boundary', 'missing-lease', 'expired-lease', 'released-alias', 'source-pause-revoke-recover', 'source-normalized-then-denied', 'source-no-double-completion', 'source-hold-loss-before-call', 'source-hold-loss-while-pending', 'source-fallback-preserves-hold-origin', 'source-release-dead-hold', 'source-cannot-create-holding-authority', 'source-cancel-recheck-second-source', 'source-cancel-needs-current-authority', 'source-no-cancel-for-committed-normalization-only', 'source-identity-crlf', 'source-identity-multibyte', 'oracle-unrestricted-invocation-resurrection', 'oracle-admissible-head-fresh-evidence-recovery', 'source-session-continue-addition', 'oracle-c-locus-rejoin-current-claim', 'oracle-c-new-member-new-claim', 'source-cancel-expired-repair-stays-failed'])
SESSION_CASES = frozenset([
    "source-cancel-recheck-second-source", "source-session-continue-addition",
    "source-cancel-expired-repair-stays-failed",
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
        if row.get("parser_exit") != 0 or not (row.get("adapter_rejected") is True or row.get("lean_exit") == 0):
            raise ValueError("unexecuted or failed source case")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


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


def memory_limit():
    resource.setrlimit(resource.RLIMIT_AS, (4 * 1024**3, 4 * 1024**3))


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
    args = parser.parse_args()
    if args.with_owner_boundary:
        args.with_publication = True
    repo = Path(__file__).resolve().parents[1]
    work_root = args.work_root.resolve()
    if not work_root.is_dir() or work_root.is_relative_to(repo):
        parser.error('--work-root must be an existing directory outside the repository')
    if shutil.disk_usage(work_root).free < 2 * 1024**3:
        parser.error('at least 2 GiB free space required before the bounded build')
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
                         'MirroreaProofFirstSourceFundingCheckedWork']:
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
    env = dict(os.environ, LEAN_PATH=str(work), CARGO_BUILD_JOBS='1', CARGO_INCREMENTAL='0',
               CARGO_TARGET_DIR=str(work / 'cargo-target'),
               MIR_PROOF_FIRST_PARSER=str(work / 'cargo-target/debug/examples/textual_mir_alpha_parse'))
    # A caller's focused filter must never turn the full acceptance command green.
    env.pop('MIR_W3_CASE_FILTER', None)
    integrity = {str(work / 'MANIFEST.json'): result['manifest_sha256']}

    def seal(path, expected=None):
        data = path.read_bytes() if expected is None else verified_bytes(path, expected)
        integrity[str(path)] = hashlib.sha256(data).hexdigest()
        return data

    def record():
        (work / 'RESULT.json').write_text(json.dumps(result, indent=2) + '\n')

    def run(command, log, cwd=work):
        completed = subprocess.run(command, cwd=cwd, env=env, capture_output=True,
                                   text=True, preexec_fn=memory_limit)
        output = completed.stdout + completed.stderr
        (work / log).write_text(output)
        result['commands'].append(dict(command=command, cwd=str(cwd), exit=completed.returncode,
                                       log=log, log_sha256=sha(work / log)))
        record()
        if completed.returncode or 'sorryAx' in output:
            raise RuntimeError('failed command; inspect ' + str(work / log))
        return output

    record()
    try:
        run(['cargo', 'build', '--locked', '--offline', '-p', 'mir-ast', '--example',
             'textual_mir_alpha_parse', '-j', '1'], 'parser-build.log', repo)
        result['parser_sha256'] = sha(Path(env['MIR_PROOF_FIRST_PARSER']))
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
        (work / 'CompleteAudit.lean').write_text(audit_source(audited) + CONSUMER_CHECKS)
        seal(work / 'CompleteAudit.lean')
        output = run(['lean', '--trust=0', 'CompleteAudit.lean'], 'CompleteAudit.log')
        counts = re.findall(r'AXIOM_AUDIT_OK (\w+) (\d+)', output)
        if {name for name, _ in counts} != set(audited):
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
        result['generated_integrity'] = {str(Path(path).relative_to(work)): value
                                         for path, value in integrity.items()}
        result.update(status='passed', audit_modules=len(counts),
                      owned_declarations=sum(int(count) for _, count in counts))
        if args.with_publication:
            result['publication_status'] = 'passed local model only'
        if args.with_owner_boundary:
            result['owner_boundary_status'] = 'passed general model proofs; native resource/source custody remains separate'
    except Exception as error:
        result.update(status='failed', error=str(error))
        raise
    finally:
        result['finished'] = datetime.datetime.now(datetime.timezone.utc).isoformat()
        record()
    print(json.dumps({key: value for key, value in result.items() if key != 'commands'}), flush=True)


if __name__ == '__main__':
    main()
