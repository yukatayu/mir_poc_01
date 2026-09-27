"""Stage actual parser-derived source consumers in the fresh reference workdir.

Preparation only: this does not execute a consumer or issue authority. The seed
and qualified library profile are private finite test inputs, not public APIs.
"""
from pathlib import Path
import hashlib
import importlib.util
import json
import os
import subprocess
import sys

if not __debug__:
    raise SystemExit('source preparation requires assertions')
host = Path(__file__).resolve().parent
root = host.parent
rec = host / 'recovered'
parser = Path(os.environ['MIR_PROOF_FIRST_PARSER']).resolve()
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
spec = importlib.util.spec_from_file_location('qualified_adapter', rec / 'qualified_source_adapter.py')
adapter = importlib.util.module_from_spec(spec)
spec.loader.exec_module(adapter)
inputs = [rec / 'qualified-source/main.mir', rec / 'source-continuation/main.mir',
          rec / 'source-continuation-owner-a/main.mir']
bindings = {str(p): sha(p) for p in [parser, Path(__file__), *inputs,
            Path(adapter.__file__), Path(adapter.reference.__file__), Path(adapter.reference.base.__file__)]}
commands, generated = [], []
for index, source in enumerate(inputs):
    command = [str(parser), str(source), '--format', 'json']
    run = subprocess.run(command, capture_output=True)
    (source.parent / 'PARSE.json').write_bytes(run.stdout)
    (source.parent / 'parser.stderr').write_bytes(run.stderr)
    commands.append(dict(command=command, exit=run.returncode, source_sha256=sha(source),
                         parse_sha256=sha(source.parent / 'PARSE.json')))
    if run.returncode:
        raise RuntimeError('actual parser rejected ' + str(source))
    raw, caller = adapter.export(json.loads(run.stdout), ['A', 'B', 'C'], source.read_text())
    text = adapter.generate(raw, caller)
    if index == 0:
        name = 'ParsedQualified'
    else:
        name = 'SourceContinuation' + ('C' if index == 1 else 'A')
        text = 'import MirroreaProofFirstSourceInput\n' + text.split('def begun :')[0] + '''
def exportContinuation : IO Unit := do
  let bytes := OwnerPacketCodec.encode (SourceInput.input 3 1) (.step (.continueWith program))
  unless SourceCodec.compactFits bytes do throw (IO.userError "continuation payload preflight")
  unless (OwnerPacketCodec.decodeAt (SourceInput.input 3 1) 256 bytes).isSome do
    throw (IO.userError "continuation payload decode")
  IO.FS.writeBinFile ''' + json.dumps(str(source.parent / 'continue.bin')) + ''' ⟨bytes.toArray⟩
  IO.println s!"ACTUAL_CONTINUATION_ENCODED statements={program.items.length},bytes={bytes.length}"
#eval exportContinuation
'''
    target = root / (name + '.lean')
    with target.open('x') as f:
        f.write(text)
    generated.append(dict(module=name, source=str(source), path=str(target), sha256=sha(target)))
for path, digest in bindings.items():
    if sha(Path(path)) != digest:
        raise RuntimeError('source preparation input changed: ' + path)
with (host / 'SOURCE_PREPARATION.json').open('x') as f:
    json.dump(dict(status='prepared, not executed', bindings=bindings,
                   commands=commands, generated=generated), f, indent=2)
    f.write('\n')
print('actual source consumers prepared; execution remains required', flush=True)
