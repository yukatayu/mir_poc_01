#!/usr/bin/env python3
"""Offline integrity/coverage check. Does not verify mathematical claims or authorize work."""
from __future__ import annotations
import argparse
import hashlib
import json
import stat
import sys
import zipfile
from pathlib import Path, PurePosixPath

def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()

def safe_relative(name: str) -> PurePosixPath:
    p = PurePosixPath(name)
    if not name or p.is_absolute() or '..' in p.parts or '\\' in name or '\x00' in name:
        raise ValueError(f'Unsafe path: {name!r}')
    return p

def verify(root: Path) -> dict:
    failures: list[str] = []
    manifest = json.loads((root / 'MANIFEST.json').read_text(encoding='utf-8'))
    expected: set[str] = set()
    for row in manifest['files']:
        rel = safe_relative(row['path']).as_posix()
        if rel in expected:
            failures.append(f'duplicate manifest entry: {rel}')
        expected.add(rel)
        p = root / rel
        if p.is_symlink() or not p.is_file():
            failures.append(f'missing/nonregular: {rel}')
            continue
        b = p.read_bytes()
        if len(b) != row['size'] or digest(b) != row['sha256']:
            failures.append(f'content mismatch: {rel}')
    actual = {p.relative_to(root).as_posix() for p in root.rglob('*')
              if p.is_file() and '__pycache__' not in p.parts and p.relative_to(root).as_posix() != 'MANIFEST.json'}
    extras = sorted(actual - expected)
    if extras:
        failures.append(f'unexpected files: {extras[:15]}')
    if any(p.is_symlink() for p in root.rglob('*')):
        failures.append('symlink inside distribution')
    archives = json.loads((root/'provenance/INPUT_ARCHIVES.json').read_text(encoding='utf-8'))
    original_files = 0
    for row in archives:
        p = root/'archives'/row['archive']
        if digest(p.read_bytes()) != row['sha256']:
            failures.append(f'original archive changed: {row["archive"]}')
        with zipfile.ZipFile(p) as z:
            if z.testzip() is not None:
                failures.append(f'ZIP CRC error: {row["archive"]}')
            names = set()
            for info in z.infolist():
                q = safe_relative(info.filename)
                if info.filename in names:
                    failures.append(f'duplicate ZIP entry: {info.filename}')
                names.add(info.filename)
                if stat.S_ISLNK(info.external_attr >> 16):
                    failures.append(f'ZIP symlink: {info.filename}')
                if info.is_dir():
                    continue
                target = root/'materials'/q
                if not target.is_file() or target.read_bytes() != z.read(info):
                    failures.append(f'expanded original differs: {info.filename}')
                original_files += 1
    master = json.loads((root/'materials/mirrorea_system_map_v1/requirements.json').read_text(encoding='utf-8'))
    trace = json.loads((root/'materials/mir_foundation_F0_3/REQUIREMENT_TRACE.json').read_text(encoding='utf-8'))
    req = {x['id'] for x in master['requirements']}
    if len(req) != 119 or req != {x['id'] for x in trace['requirements']}:
        failures.append('119-requirement identity mismatch')
    for k, n in [('decisions',30),('proof_targets',18),('scenarios',24),('goals',7),('alpha_conditions',8)]:
        if len(master[k]) != n:
            failures.append(f'wrong {k} count')
    if digest((root/'materials/mirrorea_system_map_v1/requirements.json').read_bytes()) != trace['input_requirements_sha256']:
        failures.append('F0.3 trace is bound to a different master requirement file')
    delegation = json.loads((root/'requirement_work_map.json').read_text(encoding='utf-8'))
    if req != {x['id'] for x in delegation['requirements']}:
        failures.append('work mapping loses requirements')
    work = json.loads((root/'workstreams.json').read_text(encoding='utf-8'))
    ids = {x['id'] for x in work['workstreams']}
    for w in work['workstreams']:
        if not set(w['depends_on']) <= ids:
            failures.append(f'unknown work dependency: {w["id"]}')
    # Validate the planning DAG, not semantic graphs of Mir itself.
    todo={w['id']:set(w['depends_on']) for w in work['workstreams']}
    done=set()
    while todo:
        ready={k for k,v in todo.items() if v <= done}
        if not ready:
            failures.append('workstream ordering has a cycle'); break
        done |= ready
        for k in ready: del todo[k]
    info = json.loads((root/'provenance/PROMPT.json').read_text(encoding='utf-8'))
    if digest((root/'RUN_CODEX_MIRROREA.md').read_bytes()) != info['sha256']:
        failures.append('packaged prompt differs from recorded output prompt')
    return {'passed':not failures,'manifest_files':len(expected),'original_expanded_files':original_files,
            'requirements':len(req),'decisions':len(master['decisions']),'scenarios':len(master['scenarios']),
            'failures':failures,'scope':'byte integrity and traceability only; not proof correctness, source authentication or product acceptance'}

def main() -> int:
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--root',type=Path,default=Path(__file__).resolve().parents[1])
    ap.add_argument('--json-output',type=Path)
    ap.add_argument('--prompt-file',type=Path,help='Optional separate prompt output; detect mismatched handoff versions.')
    args=ap.parse_args()
    try:
        result=verify(args.root.resolve())
        if args.prompt_file and args.prompt_file.read_bytes() != (args.root/'RUN_CODEX_MIRROREA.md').read_bytes():
            result['passed']=False; result['failures'].append('separate prompt file differs from bundle copy')
    except (OSError,ValueError,KeyError,zipfile.BadZipFile) as e:
        result={'passed':False,'failures':[f'{type(e).__name__}: {e}']}
    text=json.dumps(result,ensure_ascii=False,indent=2)+'\n'
    print(text,end='')
    if args.json_output:
        args.json_output.parent.mkdir(parents=True,exist_ok=True)
        args.json_output.write_text(text,encoding='utf-8')
    return 0 if result['passed'] else 1
if __name__=='__main__': raise SystemExit(main())
