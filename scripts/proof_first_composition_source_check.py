"""Fresh actual-parser/Lean W3 source candidate checks; not W3 or alpha acceptance."""
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


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def memory_limit():
    resource.setrlimit(resource.RLIMIT_AS, (4 * 1024**3, 4 * 1024**3))


def audit_source(modules):
    return "\n".join([
        "import Lean", *["import " + name for name in modules],
        "set_option maxRecDepth 10000", "open Lean Elab Command", "run_cmd do",
        "  let env ← getEnv",
        "  let modules : List String := " + json.dumps(modules),
        "  let allowed : List Name := [``propext, ``Classical.choice, ``Quot.sound]",
        "  let mut counts : List (String × Nat) := modules.map (fun name => (name,0))",
        "  for (name, _) in env.constants.toList do",
        "    if let some index := env.getModuleIdxFor? name then",
        "      let owner := env.header.moduleNames[index.toNat]!.toString",
        "      if modules.contains owner then",
        "        counts := counts.map (fun (m,k) => (m,if m == owner then k+1 else k))",
        "        for dependency in (← Lean.collectAxioms name) do",
        "          unless allowed.contains dependency do",
        '            throwError "AXIOM_AUDIT_REJECT {name}: {dependency}"',
        "  for (name,count) in counts do",
        '    if count == 0 then throwError "AXIOM_AUDIT_EMPTY {name}"',
        '    logInfo m!"AXIOM_AUDIT_OK {name} {count}"', "",
    ])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work-root', type=Path, required=True,
                        help='existing external directory for disposable proof/build copies')
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[1]
    work_root = args.work_root.resolve()
    if not work_root.is_dir() or work_root.is_relative_to(repo):
        parser.error('--work-root must be an existing directory outside repository')
    if shutil.disk_usage(work_root).free < 2 * 1024**3:
        parser.error('at least 2 GiB free space required before this small crate build')
    version = subprocess.check_output(['lean', '--version'], text=True)
    if 'version 4.29.1,' not in version:
        raise SystemExit('Lean4.29.1 required for this evidence cut')
    work = Path(tempfile.mkdtemp(prefix='mir-w3-source-', dir=work_root))
    print(work, flush=True)
    foundation = repo / 'samples/lean/foundations'
    modules = []

    def visit(name):
        if name in modules:
            return
        source = foundation / (name + '.lean')
        for dependency in re.findall(r'^import (\w+)$', source.read_text(), re.M):
            if dependency not in {'Lean', 'Std'}:
                visit(dependency)
        modules.append(name)
        shutil.copy2(source, work / source.name)

    for suffix in ['SourceLocators', 'SourceReadNames', 'SourceHistory', 'SourceFunction',
                   'FallbackStaticControls', 'SourceHistoryControls', 'SavedInvocation',
                   'CatalogCounterexamples']:
        visit('MirroreaProofFirst' + suffix)
    source_scripts = ['scripts/proof_first_composition_source.py',
                      'scripts/tests/proof_first_composition_source_cases.py',
                      'scripts/tests/proof_first_composition_frontend_cases.py']
    for name in source_scripts:
        shutil.copy2(repo / name, work / Path(name).name)
    sample = repo / 'samples/clean-near-end/mirrorea-proof-first-composition'
    shutil.copy2(sample / 'main.mir', work / 'main.mir')
    shutil.copytree(sample / 'controls', work / 'controls')
    manifest = {str(p.relative_to(work)): sha(p) for p in sorted(work.rglob('*')) if p.is_file()}
    for name in ['Cargo.lock', 'crates/mir-ast/Cargo.toml', 'crates/mir-ast/src/textual_alpha.rs',
                 'crates/mir-ast/examples/textual_mir_alpha_parse.rs']:
        manifest['repo:' + name] = sha(repo / name)
    (work / 'MANIFEST.json').write_text(json.dumps(manifest, indent=2) + '\n')
    result = dict(classification='W3 source/lifecycle reference candidate; dynamic fallback and final review open',
                  started=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                  workdir=str(work), lean_version=version, rlimit_as=4 * 1024**3,
                  commands=[], status='running')
    env = dict(os.environ, LEAN_PATH=str(work), CARGO_BUILD_JOBS='1', CARGO_INCREMENTAL='0',
               CARGO_TARGET_DIR=str(work / 'cargo-target'),
               MIR_PROOF_FIRST_PARSER=str(work / 'cargo-target/debug/examples/textual_mir_alpha_parse'))

    def record():
        (work / 'RESULT.json').write_text(json.dumps(result, indent=2) + '\n')

    def run(command, log, cwd=work):
        completed = subprocess.run(command, cwd=cwd, env=env, capture_output=True,
                                   text=True, preexec_fn=memory_limit)
        (work / log).write_text(completed.stdout + completed.stderr)
        result['commands'].append(dict(command=command, cwd=str(cwd), exit=completed.returncode,
                                       log=log, log_sha256=sha(work / log)))
        if completed.returncode:
            result['status'] = 'failed'
            record()
            print(completed.stdout + completed.stderr, flush=True)
            raise SystemExit(completed.returncode)
        record()
        return completed.stdout

    record()
    run(['cargo', 'build', '--locked', '--offline', '-p', 'mir-ast', '--example',
         'textual_mir_alpha_parse', '-j', '1'], 'parser-build.log', repo)
    result['parser_sha256'] = sha(Path(env['MIR_PROOF_FIRST_PARSER']))
    for name in modules:
        run(['lean', '--trust=0', '-o', name + '.olean', name + '.lean'], name + '.log')
    print('Fresh proof modules:', len(modules), flush=True)
    for name in source_scripts:
        file = Path(name).name
        run(['python3', file], file + '.log')
        print(file, 'PASS', flush=True)
    run(['lean', '--trust=0', '-o', 'ActualComposition.olean', 'ActualComposition.lean'],
        'ActualComposition-compiled.log')
    audited = modules + ['ActualComposition']
    (work / 'CompleteAudit.lean').write_text(audit_source(audited))
    output = run(['lean', '--trust=0', 'CompleteAudit.lean'], 'CompleteAudit.log')
    counts = re.findall(r'AXIOM_AUDIT_OK (\w+) (\d+)', output)
    if {name for name, _ in counts} != set(audited):
        result['status'] = 'failed'
        record()
        raise SystemExit('missing/extra audited module')
    result.update(status='passed', audit_modules=len(counts),
                  owned_declarations=sum(int(count) for _, count in counts),
                  finished=datetime.datetime.now(datetime.timezone.utc).isoformat())
    record()
    print(json.dumps({key: value for key, value in result.items() if key != 'commands'}), flush=True)


if __name__ == '__main__':
    main()
