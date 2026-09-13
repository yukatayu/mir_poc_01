"""Actual parser -> generic adapter -> actual checked source execution controls."""
from pathlib import Path
import hashlib
import json
import os
import subprocess
import resource
from proof_first_composition_source import PARSER, export, generate

WORK = Path(__file__).resolve().parent
base = (WORK / 'main.mir').read_text()
header = base.split('transition build at A')[0]
setup = '''  initial <- perform register(score, [0, 1, 2, 3], 1, 100) via lifecycle
  base <- perform instantiate(initial, ["A", "B", "C"]) via lifecycle
  extra <- perform instantiate(initial, ["A", "C"]) via lifecycle
'''
cases = [
    ('assign-dependency', setup + '''  let mut count: Int64 = 2
  count = count + 1
  actual <- extra(count)
''', '''#guard checked.isSome
#guard outcome.rejected.isNone
#guard SourceAuthoring.lookup outcome.final.source.values "actual" = some (.integer 10 false)
#guard (outcome.final.writes.find? (fun w => w.ordinal == 4)).map (fun w => (w.name,w.value,w.inputs)) =
  some ("count",.integer 3 true,[⟨"count",some (.integer 2 true),some 3⟩])
#guard (outcome.final.writes.find? (fun w => w.ordinal == 5)).map (fun w => w.inputs.map SourceExecution.Read.producer) = some [some 2,some 4]
'''),
    ('leave-rejoin', setup + '''  left <- perform leave("C") via lifecycle
  during <- extra(2)
  joined <- perform join("C") via lifecycle
  after <- extra(2)
''', '''#guard checked.isSome
#guard outcome.rejected.isNone
#guard SourceAuthoring.lookup outcome.final.source.values "during" = some (.integer 5 false)
#guard SourceAuthoring.lookup outcome.final.source.values "after" = some (.integer 5 false)
#guard outcome.final.source.machine.system.configuration.state.placeIncarnation 2 = 1
'''),
    ('retire-prefix', setup + '''  retired <- perform retire(extra) via lifecycle
  denied <- extra(2)
  unreachable <- base(2)
''', '''#guard checked.isSome
#guard outcome.rejected.map (fun x => x.2) = some .dynamicRejected
#guard outcome.final.writes.length = 4
#guard outcome.final.source.machine.system.configuration.count = 2
#guard SourceAuthoring.lookup outcome.final.source.values "retired" = some .unit
#guard (SourceAuthoring.lookup outcome.final.source.values "denied").isNone
#guard (SourceAuthoring.lookup outcome.final.source.values "unreachable").isNone
#guard (CompositionCore.index outcome.final.source.machine.system.configuration.count 1).map
  (fun key => (outcome.final.source.machine.system.configuration.state.instances key).enabled) = some false
'''),
    ('overflow-prefix', setup + '''  let mut count: Int64 = 9223372036854775807
  count = count + 1
  unreachable <- base(2)
''', '''#guard checked.isSome
#guard outcome.rejected.map (fun x => x.2) = some .dynamicRejected
#guard outcome.final.writes.length = 4
#guard SourceAuthoring.lookup outcome.final.source.values "count" = some (.integer 9223372036854775807 true)
'''),
    ('wrong-callee-type', setup + '  denied <- initial(2)\n', '''#guard checked.isNone
#guard outcome.rejected.map (fun x => x.2) = some .staticType
#guard outcome.final.writes.length = 3
'''),
    ('immutable-write', setup + '''  let count: Int64 = 2
  count = 3
''', '''#guard checked.isNone
#guard outcome.rejected.map (fun x => x.2) = some .staticType
#guard SourceAuthoring.lookup outcome.final.source.values "count" = some (.integer 2 false)
'''),
    ('out-of-input', setup + '  denied <- extra(4)\n', '''#guard checked.isSome
#guard outcome.rejected.map (fun x => x.2) = some .dynamicRejected
#guard outcome.final.source.machine.pending.isEmpty
#guard outcome.final.writes.length = 3
'''),
    ('duplicate-binding-prefix', setup + '  initial <- perform register(score, [0, 1, 2, 3], 1, 100) via lifecycle\n', '''#guard checked.isNone
#guard outcome.rejected.map (fun x => x.2) = some .staticType
#guard outcome.final.writes.length = 3
#guard outcome.final.source.machine.system.configuration.definitions = 1
#guard outcome.final.source.machine.events.length = 3
#guard outcome.final.source.nextRequest = 3
'''),
    ('renamed-different-logic', setup.replace('score,', 'scoreNext,').replace('initial','code').replace('base','ordinary').replace('extra','addition') + '  observed <- addition(3)\n', '''#guard checked.isSome
#guard outcome.rejected.isNone
#guard SourceAuthoring.lookup outcome.final.source.values "observed" = some (.integer 11 false)
'''),
]

def memory_limit():
    resource.setrlimit(resource.RLIMIT_AS, (4*1024**3,4*1024**3))

results = []
for name, body, assertions in cases:
    source = WORK / ('source-' + name + '.mir')
    source.write_text(header + 'transition build at A requires CompositionControl {\n' + body + '}\n')
    parsed = subprocess.run([str(PARSER), str(source), '--format', 'json'], capture_output=True, text=True)
    (WORK / (name + '-parser.json')).write_text(parsed.stdout)
    if parsed.returncode:
        raise RuntimeError(name + ': actual parser rejected: ' + parsed.stdout + parsed.stderr)
    program, place = export(json.loads(parsed.stdout), ['A','B','C'], source.read_text())
    lean = WORK / ('Control-' + name + '.lean')
    lean.write_text(generate(program, place, assertions))
    checked = subprocess.run(['lean','--trust=0',lean.name],cwd=WORK,
                             env={**os.environ,'LEAN_PATH':str(WORK)},capture_output=True,text=True,preexec_fn=memory_limit)
    (WORK / (name + '-execution.log')).write_text(checked.stdout + checked.stderr)
    results.append({'case':name,'parser_exit':parsed.returncode,'lean_exit':checked.returncode,
                    'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                    'lean_sha256':hashlib.sha256(lean.read_bytes()).hexdigest()})
    (WORK / 'SOURCE_EXECUTION_CONTROLS.json').write_text(json.dumps(results,indent=2)+'\n')
    print(name + ': exit ' + str(checked.returncode),flush=True)
    if checked.returncode:
        print(name + ': ' + checked.stdout + checked.stderr)
(WORK / 'SOURCE_EXECUTION_CONTROLS.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps(results))
raise SystemExit(1 if any(r['lean_exit'] for r in results) else 0)
