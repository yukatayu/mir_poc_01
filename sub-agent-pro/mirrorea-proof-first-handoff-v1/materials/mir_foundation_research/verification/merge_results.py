"""Validate and merge only the four disjoint batches used for this run."""
import hashlib
import json
from pathlib import Path

root = Path(__file__).resolve().parent
manifest = json.loads((root/'manifest.json').read_text())
results = []
for lo,hi in [(0,20),(20,40),(40,56),(56,61)]:
    batch = json.loads((root/f'results_{lo}_{hi}.json').read_text())
    if [r['id'] for r in batch] != [m['id'] for m in manifest[lo:hi]]:
        raise SystemExit(f'Batch inventory mismatch: {lo}:{hi}')
    results.extend(batch)
if len(results) != len(manifest):
    raise SystemExit('Incomplete result inventory')
for m,r in zip(manifest,results):
    if r['id'] != m['id'] or r['expected'] != m['expected']:
        raise SystemExit(f'Result identity mismatch: {m["id"]}')
    if r['actual'] != m['expected'] or r['passed'] is not True:
        raise SystemExit(f'Failed result: {m["id"]}')
    digest = hashlib.sha256((root/'vcs'/m['file']).read_bytes()).hexdigest()
    if digest != r['input_sha256']:
        raise SystemExit(f'Input changed since execution: {m["id"]}')
    log = (root/'logs'/Path(m['file']).with_suffix('.txt').name).read_text()
    if not log.strip() or log.splitlines()[0].strip() != r['actual'] or '(error' in log:
        raise SystemExit(f'Output log mismatch: {m["id"]}')
(root/'results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
(root/'summary.txt').write_text('Executed as four disjoint batches; input digests rechecked.\n' +
    '\n'.join(f"{r['id']}: {r['actual']} PASS" for r in results) +
    f'\n{len(results)}/{len(manifest)} expected results\n')
print(f'{len(results)} actual results and input digests verified; merged.')
