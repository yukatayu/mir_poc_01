"""Check the packaged file inventory. It provides integrity, not authenticity."""
from pathlib import Path
import hashlib,json,sys
root=Path(__file__).resolve().parents[1]
manifest=json.loads((root/'provenance/artifact_manifest.json').read_text())
errors=[]
for item in manifest['files']:
    rel=Path(item['path'])
    if rel.is_absolute() or '..' in rel.parts:
        errors.append('Unsafe manifest path');continue
    p=root/rel
    if not p.is_file() or p.stat().st_size!=item['bytes'] or hashlib.sha256(p.read_bytes()).hexdigest()!=item['sha256']:
        errors.append(str(rel))
print(json.dumps({'checked':len(manifest['files']),'mismatches':errors,'integrity_only':True},indent=2))
sys.exit(bool(errors))
