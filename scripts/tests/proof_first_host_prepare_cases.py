"""Staging confinement and actual leanc environment controls; no physical claim."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import runpy
import subprocess
import tempfile


def main():
    if not __debug__:
        raise SystemExit('control assertions required')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work-root', required=True, type=Path)
    parser.add_argument('--toolchain', required=True, type=Path)
    parser.add_argument('--runner', type=Path, default=Path(__file__).resolve().parents[1] / 'proof_first_host_prepare.py')
    args = parser.parse_args()
    repo = Path(__file__).resolve().parents[2]
    work_root = args.work_root.resolve()
    assert work_root.is_dir() and not work_root.is_relative_to(repo)
    work = Path(tempfile.mkdtemp(prefix='host-prepare-controls-', dir=work_root))
    runner = args.runner.resolve()
    sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
    before = sha(runner)
    api = runpy.run_path(str(runner))
    record = dict(scope=__doc__, runner=str(runner), runner_sha256=before, controls=[])
    try:
        assert 'checked_toolchain_layout' in api, 'explicit toolchain layout gate missing'
        layout = api['checked_toolchain_layout']
        valid = layout(work / 'toolchain', work / 'repo', work / 'historical-repo')
        assert valid['prefix'] == str(work / 'toolchain')
        for label, prefix, repositories in [
            ('toolchain-under-repo', work / 'repo/toolchain', [work / 'repo']),
            ('repo-under-prefix', work, [work / 'repo']),
            ('same-root', work, [work]),
            ('historical-checkout-overlap', work / 'old', [work / 'repo', work / 'old/repo']),
        ]:
            try:
                layout(prefix, *repositories)
            except ValueError as error:
                assert 'disjoint' in str(error)
                record['controls'].append(label + ' refused as unsupported layout')
            else:
                raise AssertionError(label + ' silently accepted')
        record['controls'].append('disjoint current and historical repositories supported')
        assert 'host_source_paths' in api, 'staging confinement gate missing'
        assert 'native_environment' in api, 'native environment selection missing'
        manifest = json.loads((repo / 'docs/proof-first/W4_HOST_MANIFEST.json').read_text())
        destinations = []
        for row in manifest['sources']:
            source, target = api['host_source_paths'](repo, work / 'stage', row['target'])
            assert source == (repo / row['target']).resolve() and source.is_file()
            assert target.is_relative_to(work / 'stage')
            destinations.append(target)
        assert len(destinations) == len(set(destinations)) == len(manifest['sources'])
        record['controls'].append('all selected sources have distinct confined destinations')
        for target in [
            'scripts/proof_first_host/../../docs/proof-first/W4_HOST_MANIFEST.json',
            'scripts/proof_first_host/../proof_first_host/run_writer_statement_capture_v2.py',
            '/tmp/outside.py', 'scripts/proof_first_host//outside.py',
            'scripts/proof_first_host/./outside.py', 'scripts/foreign.py',
            'samples/lean/host-reference/nested/SourceInputControls.lean',
        ]:
            try:
                api['host_source_paths'](repo, work / 'stage', target)
            except ValueError:
                record['controls'].append('rejected ' + target)
            else:
                raise AssertionError('unsafe or ambiguous staging accepted: ' + target)
        fake = work / 'unselected-compiler'
        fake.write_text('#!/bin/sh\nprintf "UNSELECTED_CC_USED\\n"\nexit 23\n')
        fake.chmod(0o700)
        toolchain = args.toolchain.resolve()
        injected = dict(os.environ, LEAN_CC=str(fake), LEAN_SYSROOT=str(work / 'foreign-sysroot'))
        command = [str(toolchain / 'bin/leanc'), '-v', '-fsyntax-only', '-x', 'c', '/dev/null']
        red_env = dict(injected, LEAN_SYSROOT=str(toolchain))
        red = subprocess.run(command, env=red_env, capture_output=True, text=True)
        assert red.returncode == 23 and 'UNSELECTED_CC_USED' in red.stdout
        selected = api['native_environment'](injected, work / 'stage', toolchain, work / 'cache', work / 'parser')
        green = subprocess.run(command, env=selected, capture_output=True, text=True)
        output = green.stdout + green.stderr
        (work / 'selected-compiler.log').write_text(output)
        assert green.returncode == 0 and 'UNSELECTED_CC_USED' not in output, output
        assert str(toolchain / 'bin/clang') in output, output
        assert selected['LEAN_SYSROOT'] == str(toolchain) and 'LEAN_CC' not in selected
        record['controls'].append('actual leanc ignores injected compiler/sysroot and uses selected clang')
        assert sha(runner) == before, 'runner changed during controls'
        record['status'] = 'passed'
    except BaseException as error:
        record.update(status='failed', error=str(error))
        raise
    finally:
        (work / 'RESULT.json').write_text(json.dumps(record, indent=2) + '\n')
        print(work, flush=True)


if __name__ == '__main__':
    main()
