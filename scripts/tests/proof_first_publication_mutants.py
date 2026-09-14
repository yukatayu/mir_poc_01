"""W4 publication-component proof sensitivity, not production evidence."""
from pathlib import Path
import hashlib
import json
import os
import re
import resource
import subprocess

if not __debug__:
    raise SystemExit("optimized Python is not an evidence execution mode")

root = Path(__file__).resolve().parent
output = root / 'publication-mutants'
output.mkdir()
mutations = [
    ('unfrozen-ack', 's.frozen.contains (i,g)', 'true', 'check_exact'),
    ('stale-install', 's.certificates.contains g && decide (s.fence i ≤ g)',
     's.certificates.contains g', 'check_exact'),
    ('missing-fence', 'fence := put s.fence i (max (s.fence i) g)',
     'fence := s.fence', 'apply_preserves'),
    ('publish-without-ack-bound', 'published := s.announced, certificates :=',
     'published := s.announced + 1, certificates :=', 'apply_preserves'),
    ('reject-all-use', '| .use i => decide (s.installed i = s.fence i)',
     '| .use _ => false', 'check_exact'),
]
mutations = [('MirroreaProofFirstPublication', *row) for row in mutations] + [
    ('MirroreaProofFirstPublicationExecution', 'waiting-publication-bypass',
     '| .tick .. => decide (s.status = .ready)', '| .tick .. => true', 'ready_exact'),
    ('MirroreaProofFirstPublicationExecution', 'erased-reply-ordinal',
     '⟨pair.1,⟨s.replies.length,command,pair.2⟩ :: s.replies⟩',
     '⟨pair.1,⟨0,command,pair.2⟩ :: s.replies⟩', 'snapshot_exact'),
    ('MirroreaProofFirstPublicationExecution', 'reject-all-received-results',
     '| .received entry value => (ReceivedResult.arrive s entry value).map fun next => (next,.received value)',
     '| .received _ _ => none', 'evaluateResult_complete'),
]

def cap():
    resource.setrlimit(resource.RLIMIT_AS, (4 * 1024**3, 4 * 1024**3))

results = []
for module, name, old, new, theorem in mutations:
    original = root / (module + '.lean')
    source = original.read_text()
    if source.count(old) != 1:
        raise ValueError("nonunique mutation anchor: " + name)
    changed = source.replace(old, new)
    lines = changed.splitlines()
    begin = next(i for i, line in enumerate(lines) if line.startswith('theorem ' + theorem + ' '))
    end = next((i for i in range(begin + 1, len(lines))
                if re.match(r'(theorem|def|inductive|structure|#print|end)\b', lines[i])), len(lines))
    target = output / (name + '.lean')
    target.write_text(changed)
    run = subprocess.run(['lean', '--trust=0', target.name], cwd=output,
                         env=dict(os.environ, LEAN_PATH=str(root)), preexec_fn=cap,
                         text=True, capture_output=True)
    text = run.stdout + run.stderr
    (output / (name + '.log')).write_text(text)
    errors = [int(i) for i in re.findall(re.escape(target.name) + r':(\d+):\d+: error(?:\([^)]*\))?:', text)]
    found = run.returncode != 0 and any(begin + 1 <= i <= end for i in errors)
    results.append(dict(name=name, theorem=theorem, old=old, new=new,
                        exit=run.returncode, rejected_at_theorem=found,
                        original_sha256=hashlib.sha256(original.read_bytes()).hexdigest(),
                        mutant_sha256=hashlib.sha256(target.read_bytes()).hexdigest(),
                        log_sha256=hashlib.sha256(text.encode()).hexdigest()))
    (output / 'RESULT.json').write_text(json.dumps(results, indent=2) + '\n')
    if not found:
        raise RuntimeError('designated proof did not reject ' + name)
    print(name, 'PASS', flush=True)
