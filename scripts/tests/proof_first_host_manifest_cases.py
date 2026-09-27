"""Source-manifest integrity controls only; no process or refinement claim."""
from pathlib import Path
import argparse
import copy
import json
import shutil
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import proof_first_reference_source_check as runner


def main():
    if not __debug__:
        raise SystemExit('control assertions required')
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work-root', type=Path, required=True)
    args = parser.parse_args()
    assert hasattr(runner, 'host_model_modules'), 'host source closure gate is not implemented'
    repo = Path(__file__).resolve().parents[2]
    work_root = args.work_root.resolve()
    assert work_root.is_dir() and not work_root.is_relative_to(repo)
    work = Path(tempfile.mkdtemp(prefix='host-manifest-controls-', dir=work_root))
    manifest_path = Path('docs/proof-first/W4_SOURCE_MANIFEST.json')
    original = json.loads((repo / manifest_path).read_text())
    target = work / manifest_path
    target.parent.mkdir(parents=True)
    for row in original['sources']:
        dst = work / row['target']
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(repo / row['target'], dst)
    target.write_text(json.dumps(original))
    modules = runner.host_model_modules(work)
    assert len(modules) == len(original['sources']) and len(set(modules)) == len(modules)
    results = ['valid source closure accepted']
    for label in ['missing-root', 'missing-dependency', 'duplicate', 'wrong-digest',
                  'path-escape', 'unexpected-root', 'unreachable-extra']:
        bad = copy.deepcopy(original)
        if label == 'missing-root':
            bad['sources'] = [r for r in bad['sources'] if r['module'] != bad['roots'][0]]
        elif label == 'missing-dependency':
            bad['sources'].pop(0)
        elif label == 'duplicate':
            bad['sources'].append(bad['sources'][0])
        elif label == 'wrong-digest':
            bad['sources'][0]['sha256'] = '0' * 64
        elif label == 'path-escape':
            bad['sources'][0]['target'] = '../outside.lean'
        elif label == 'unexpected-root':
            bad['roots'] = bad['roots'][1:]
        else:
            source = work / 'samples/lean/foundations/UnreachableExtra.lean'
            source.write_text('def extra : Nat := 0\n')
            bad['sources'].append(dict(module='UnreachableExtra', target=str(source.relative_to(work)),
                                       sha256=runner.sha(source)))
        target.write_text(json.dumps(bad))
        try:
            runner.host_model_modules(work)
        except ValueError:
            results.append(label + ' rejected')
        else:
            raise AssertionError(label + ' accepted')
    target.write_text(json.dumps(original))
    changed = work / original['sources'][0]['target']
    changed.write_text(changed.read_text() + '\n-- altered after receipt\n')
    try:
        runner.host_model_modules(work)
    except ValueError:
        results.append('changed-source rejected')
    else:
        raise AssertionError('changed source accepted')
    print(json.dumps(dict(scope=__doc__,controls=results,workdir=str(work)), indent=2))


if __name__ == '__main__':
    main()
