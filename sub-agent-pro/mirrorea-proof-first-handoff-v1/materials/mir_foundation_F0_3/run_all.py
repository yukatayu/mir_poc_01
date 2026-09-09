#!/usr/bin/env python3
"""Offline reproducibility entry. No repository writes or external downloads.

Use --baselines /path/to/extracted_baselines to also run unchanged F0.1/F0.2
unit suites; baseline archives are deliberately not duplicated in this package.
"""
from __future__ import annotations
import argparse
import json
from pathlib import Path
import platform
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent

def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baselines', type=Path)
    args = parser.parse_args()
    evidence = ROOT / 'evidence'
    evidence.mkdir(exist_ok=True)
    jobs = [
        ('syntax', [sys.executable, '-m', 'compileall', '-q', 'model', 'base', 'tests', 'examples', 'verification'], ROOT),
        ('tests', [sys.executable, '-m', 'unittest', 'discover', '-s', 'tests', '-v'], ROOT),
        ('integration', [sys.executable, 'examples/integrated.py'], ROOT),
        ('orders', [sys.executable, 'verification/explore_interactions.py'], ROOT),
        ('mutants', [sys.executable, 'verification/run_mutants.py'], ROOT),
    ]
    if args.baselines:
        for name, folder, target in (
            ('baseline_F0_1', 'mir_foundation_research', 'model'),
            ('baseline_F0_2', 'mir_foundation_F0_2', 'tests'),
        ):
            path = args.baselines / folder
            if not (path / target).is_dir():
                parser.error(f'Missing extracted baseline: {path / target}')
            jobs.append((name, [sys.executable, '-m', 'unittest', 'discover', '-s', target, '-v'], path))
    rows = []
    for name, command, cwd in jobs:
        start = time.monotonic()
        proc = subprocess.run(command, cwd=cwd, capture_output=True, text=True, timeout=120)
        log = evidence / f'run_{name}.log'
        log.write_text(proc.stdout + proc.stderr, encoding='utf-8')
        row = {'name': name, 'command': command, 'cwd': str(cwd),
               'returncode': proc.returncode, 'elapsed_seconds': round(time.monotonic()-start, 6),
               'log': log.relative_to(ROOT).as_posix()}
        rows.append(row)
        print(name, 'PASS' if proc.returncode == 0 else 'FAIL', flush=True)
        if proc.returncode:
            break
    report = {'python': sys.version, 'platform': platform.platform(), 'jobs': rows,
              'passed': len(rows) == len(jobs) and all(row['returncode'] == 0 for row in rows),
              'scope': 'execution tests, bounded model orders and deliberate falsifiers; not proof-assistant verification'}
    (evidence / 'RUN_MANIFEST.json').write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')
    return 0 if report['passed'] else 1

if __name__ == '__main__':
    raise SystemExit(main())
