#!/usr/bin/env python3
"""Create a frozen, explicitly selected context packet. Does NOT send it.

The denylist is only a guardrail, not a secret scanner. Inspect every selected
file and resulting QUESTION.md before using a verified local oracle wrapper.
"""
from __future__ import annotations
import argparse
import hashlib
import json
import subprocess
from pathlib import Path
BLOCK_PARTS={'.git','.ssh','.oracle','.ask-pro','node_modules','target','.lake','__pycache__','browser-profile'}
BLOCK_NAMES={'auth.json','credentials.json','cookies.json','cookies.txt','secrets.json'}
BLOCK_SUFFIX={'.pem','.key','.p12','.pfx','.sqlite','.sqlite3','.db'}

def main()->int:
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--repo',type=Path,required=True)
    ap.add_argument('--question',type=Path,required=True)
    ap.add_argument('--output',type=Path,required=True,help='New directory only.')
    ap.add_argument('--file',dest='files',action='append',default=[],help='Explicit repository-relative FILE; no directory/glob.')
    args=ap.parse_args();repo=args.repo.resolve();out=args.output.resolve()
    if out.exists():ap.error('Output must not already exist.')
    question=args.question.read_text(encoding='utf-8')
    if not question.strip():ap.error('Question is empty.')
    if len(question.encode('utf-8'))>256*1024:ap.error('Question exceeds 256 KiB; use explicit selected attachments.')
    candidates=[];seen=set()
    for name in args.files:
        rel=Path(name)
        if rel.is_absolute() or '..' in rel.parts or '\\' in name:ap.error(f'Unsafe relative path: {name}')
        if name in seen:ap.error(f'Duplicate file: {name}')
        seen.add(name)
        if any(p in BLOCK_PARTS or p=='.env' or p.startswith('.env.') for p in rel.parts) or rel.suffix.lower() in BLOCK_SUFFIX or rel.name.lower() in BLOCK_NAMES:
            ap.error(f'Potential secret/generated path refused: {name}')
        current=repo
        for component in rel.parts:
            current=current/component
            if current.is_symlink():ap.error(f'Symlink refused: {name}')
        p=(repo/rel).resolve()
        try:p.relative_to(repo)
        except ValueError:ap.error(f'Outside repository: {name}')
        if not p.is_file():ap.error(f'Not a regular file: {name}')
        b=p.read_bytes()
        if len(b)>8*1024*1024:ap.error(f'File exceeds packet guardrail: {name}')
        try:b.decode('utf-8')
        except UnicodeDecodeError:ap.error(f'Nontext file: {name}; attach separately after manual inspection.')
        candidates.append((rel,b))
    if sum(len(b) for _,b in candidates)>24*1024*1024:ap.error('Packet exceeds 24 MiB guardrail; select relevant material.')
    p=subprocess.run(['git','-C',str(repo),'rev-parse','HEAD'],capture_output=True,text=True,check=False)
    if p.returncode:ap.error('Cannot establish repository HEAD.')
    head=p.stdout.strip();out.mkdir(parents=True)
    rows=[]
    for rel,b in candidates:
        q=out/'files'/rel;q.parent.mkdir(parents=True,exist_ok=True);q.write_bytes(b)
        rows.append({'path':rel.as_posix(),'packet_path':q.relative_to(out).as_posix(),'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b)})
    (out/'QUESTION.md').write_text(question,encoding='utf-8')
    meta={'head':head,'files':rows,'question_sha256':hashlib.sha256(question.encode()).hexdigest(),
          'notice':'Exact selected working-file bytes, possibly dirty relative to HEAD. Manual secret review required. No submission was made.'}
    (out/'PACKET_MANIFEST.json').write_text(json.dumps(meta,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({'output':str(out),'head':head,'files':len(rows),'submitted':False},ensure_ascii=False))
    return 0
if __name__=='__main__':raise SystemExit(main())
