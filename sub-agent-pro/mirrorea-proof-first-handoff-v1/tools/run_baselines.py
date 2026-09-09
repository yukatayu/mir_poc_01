#!/usr/bin/env python3
"""Copy historical packages to a fresh work area and run unit suites serially.

No downloads, package installs, Git changes, proof-status promotion, or oracle calls.
Only unit suites are executed; their own finite test timeouts remain their contracts.
"""
from __future__ import annotations
import argparse
import datetime as dt
import hashlib
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]

def inside(p:Path, parent:Path)->bool:
    try: p.resolve().relative_to(parent.resolve()); return True
    except ValueError: return False

def main()->int:
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--work-root',type=Path,required=True,help='New directory; must not already exist.')
    ap.add_argument('--evidence-root',type=Path,required=True,help='New directory outside the bundle.')
    args=ap.parse_args(); work=args.work_root.resolve(); evidence=args.evidence_root.resolve()
    if inside(work,ROOT) or inside(evidence,ROOT) or inside(work,evidence) or inside(evidence,work):
        ap.error('Work and evidence directories must be separate and outside the read-only bundle.')
    if work.exists() or evidence.exists():
        ap.error('Refusing to overwrite an existing working/evidence directory.')
    work.mkdir(parents=True); evidence.mkdir(parents=True)
    targets=[('F0.1','mir_foundation_research','model'),('F0.2','mir_foundation_F0_2','tests'),('F0.3','mir_foundation_F0_3','tests')]
    rows=[]
    for label, folder, target in targets:
        source=ROOT/'materials'/folder; dest=work/folder
        shutil.copytree(source,dest,ignore=shutil.ignore_patterns('__pycache__','*.pyc'))
        argv=[sys.executable,'-m','unittest','discover','-s',target,'-v']
        logfile=evidence/(label.replace('.','_')+'.log')
        started=dt.datetime.now(dt.timezone.utc).isoformat(); t=time.monotonic()
        env=dict(os.environ); env['PYTHONDONTWRITEBYTECODE']='1';env['PYTHONHASHSEED']='0'
        with logfile.open('wb') as log:
            p=subprocess.run(argv,cwd=dest,env=env,stdout=log,stderr=subprocess.STDOUT,check=False)
        raw=logfile.read_bytes(); output=raw.decode('utf-8',errors='replace')
        matches=re.findall(r'Ran (\d+) tests? in ',output)
        count=int(matches[-1]) if matches else None
        row={'package':label,'command':argv,'working_copy':str(dest),'started_utc':started,
             'elapsed_seconds':round(time.monotonic()-t,3),'exit_code':p.returncode,'test_count':count,
             'log':logfile.name,'log_sha256':hashlib.sha256(raw).hexdigest(),
             'passed':p.returncode==0 and count is not None and count>0}
        rows.append(row); print(label,'PASS' if row['passed'] else 'FAIL',f'tests={count}',flush=True)
    result={'created_utc':dt.datetime.now(dt.timezone.utc).isoformat(),'python':sys.version,'platform':platform.platform(),
            'passed':all(x['passed'] for x in rows),'runs':rows,
            'not_executed':['SMT inputs','Lean/Rocq proofs','live target repository tests','oracle','actual network or storage integration'],
            'scope':'fresh inherited unit-suite reproduction; no new research theorem or independent review'}
    (evidence/'BASELINE_RERUN.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    return 0 if result['passed'] else 1
if __name__=='__main__': raise SystemExit(main())
