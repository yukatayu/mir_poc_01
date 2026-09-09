#!/usr/bin/env python3
"""Run ONE already-verified command, logging output without token-heavy polling.

No job walltime limit. A command may detach itself: process exit does not mean
an oracle response is complete. The caller must use the installed oracle's
actual session/status interface to recover its answer.
"""
from __future__ import annotations
import argparse
import datetime as dt
import json
import os
import signal
import subprocess
import sys
import time
from pathlib import Path

def now()->str: return dt.datetime.now(dt.timezone.utc).isoformat()

def main()->int:
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--directory',type=Path,required=True,help='Fresh output directory; never overwritten.')
    ap.add_argument('--cwd',type=Path,required=True)
    ap.add_argument('--interval-seconds',type=float,default=180)
    ap.add_argument('command',nargs=argparse.REMAINDER)
    a=ap.parse_args(); cmd=a.command[1:] if a.command[:1]==['--'] else a.command
    if not cmd: ap.error('Pass an exact verified argv after --.')
    if a.interval_seconds<180: ap.error('Polling interval must be at least 180 seconds.')
    if not a.cwd.is_dir(): ap.error('Working directory does not exist.')
    d=a.directory.resolve()
    if d.exists(): ap.error('Refusing to overwrite an existing run directory.')
    d.mkdir(parents=True)
    meta={'command':cmd,'cwd':str(a.cwd.resolve()),'started_utc':now(),'status':'starting',
          'poll_interval_seconds':a.interval_seconds,'walltime_limit':None,
          'notice':'process state only; does not certify oracle answer completion'}
    def save()->None:
        tmp=d/'process.json.tmp';tmp.write_text(json.dumps(meta,ensure_ascii=False,indent=2)+'\n',encoding='utf-8');tmp.replace(d/'process.json')
    save(); started=time.monotonic()
    try:
        with (d/'output.log').open('xb') as log:
            p=subprocess.Popen(cmd,cwd=a.cwd,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
            meta.update(pid=p.pid,status='running');save()
            print(f'pid={p.pid} log={d / "output.log"}',flush=True)
            try:
                while True:
                    try: code=p.wait(timeout=a.interval_seconds);break
                    except subprocess.TimeoutExpired:
                        meta.update(last_check_utc=now(),elapsed_seconds=round(time.monotonic()-started,3));save()
                        print(f'process still running; elapsed={int(time.monotonic()-started)}s; next check >= {int(a.interval_seconds)}s',flush=True)
            except KeyboardInterrupt:
                # Explicit human interrupt, never an elapsed-time cutoff.
                if os.name=='posix': os.killpg(p.pid,signal.SIGINT)
                else: p.send_signal(signal.SIGINT)
                meta.update(status='interrupted_by_operator',ended_utc=now());save()
                return 130
    except OSError as e:
        meta.update(status='launch_error',error=f'{type(e).__name__}: {e}',ended_utc=now());save()
        print(meta['error'],file=sys.stderr);return 127
    meta.update(status='process_exited',exit_code=code,ended_utc=now(),elapsed_seconds=round(time.monotonic()-started,3));save()
    print(f'process_exit={code}; confirm oracle session completion separately when applicable',flush=True)
    return code if code>=0 else 128-code
if __name__=='__main__': raise SystemExit(main())
