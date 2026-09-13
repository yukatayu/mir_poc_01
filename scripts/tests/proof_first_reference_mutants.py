"""Designated-theorem weakening controls for the freshly built W3 proof cone.

These are proof-script rejection controls, not additional general theorems or
claims that every edited statement is false. Syntax failures alone do not pass.
"""
from pathlib import Path
import hashlib
import json
import os
import re
import resource
import subprocess

WORK = Path(__file__).resolve().parent
OUT = WORK / 'reference-mutants'
OUT.mkdir()

MUTATIONS = [
    ('head-generation-unchecked', 'ReferenceAuthority',
     'decide (old.generation < next.generation)', 'true', 'check_exact'),
    ('revocation-tombstones-dropped', 'ReferenceAuthority',
     'old.authority.revoked.all (next.authority.revoked.contains ·)', 'true', 'check_exact'),
    ('continuation-archive-erased', 'ReferenceContinuation',
     'superseded := archive s :: s.superseded', 'superseded := s.superseded', 'adopt_state'),
    ('cancel-authority-unchecked', 'CompositionMachine',
     'ReferenceCancellationBoundary.check m.system m.events.length t member place principal id permit',
     'true', 'cancel_parts'),
    ('fallback-hold-origin-reset', 'ReferenceMutation',
     'let change := Change.degrade old (newBinding m old.request selected old.holding)',
     'let change := Change.degrade old (newBinding m old.request selected '
     '{old.holding with activation := m.history.length,frontier := m.events.length})',
     'normalize_change_record'),
]


def cap():
    resource.setrlimit(resource.RLIMIT_AS, (4 * 1024**3, 4 * 1024**3))


def theorem_range(text, theorem):
    lines = text.splitlines()
    declaration = r'(?:private\s+)?(?:theorem|def|inductive|structure|namespace|end|#print)\b'
    start = next(i for i, line in enumerate(lines)
                 if re.match(r'(?:private\s+)?theorem ' + re.escape(theorem) + r'\b', line))
    stop = next((i for i in range(start + 1, len(lines)) if re.match(declaration, lines[i])), len(lines))
    return start + 1, stop + 1


results = []
for name, suffix, old, new, theorem in MUTATIONS:
    source = WORK / ('MirroreaProofFirst' + suffix + '.lean')
    text = source.read_text()
    if text.count(old) != 1:
        raise ValueError((name, 'nonunique mutation', text.count(old)))
    mutated = text.replace(old, new)
    start, stop = theorem_range(mutated, theorem)
    target = OUT / (name + '.lean')
    target.write_text(mutated)
    command = ['lean', '--trust=0', target.name]
    run = subprocess.run(command, cwd=OUT, env=dict(os.environ, LEAN_PATH=str(WORK)),
                         capture_output=True, text=True, preexec_fn=cap)
    log = OUT / (name + '.log')
    log.write_text(run.stdout + run.stderr)
    lines = [int(line) for line in re.findall(re.escape(target.name) + r':(\d+):\d+: error:', run.stdout)]
    rejected = run.returncode != 0 and any(start <= line < stop for line in lines)
    results.append(dict(name=name, module=source.stem, theorem=theorem, theorem_lines=[start, stop],
                        original_sha256=hashlib.sha256(source.read_bytes()).hexdigest(),
                        mutant_sha256=hashlib.sha256(target.read_bytes()).hexdigest(),
                        log_sha256=hashlib.sha256(log.read_bytes()).hexdigest(), old=old, new=new,
                        command=command, exit=run.returncode, rejected_at_theorem=rejected))
    (OUT / 'RESULT.json').write_text(json.dumps(dict(status='running', mutations=results), indent=2) + '\n')
    if not rejected:
        raise RuntimeError('not rejected at designated theorem: ' + str(log))
    print(name + ' PASS', flush=True)
(OUT / 'RESULT.json').write_text(json.dumps(dict(status='passed', mutations=results), indent=2) + '\n')
