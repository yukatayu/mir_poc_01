"""Actual frontend regressions; same source adapter and machine, no expected trace splice."""
from pathlib import Path
import hashlib,json,os,subprocess
from proof_first_composition_source import PARSER, export, generate, memory_limit
W=Path(__file__).resolve().parent
OUT=W/'frontend-controls'; OUT.mkdir(exist_ok=True)
base=(W/'main.mir').read_text(); header=base.split('fn score(')[0]
def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def complete(body, definition='return x'):
    return header+'fn value(x: Int64) -> Int64 {\n  '+definition+'\n}\ntransition build at A requires CompositionControl {\n'+body+'}\n'
setup='''  initial <- perform register(value, [-9223372036854775808, -2, -1, 0, 9223372036854775807], -9223372036854775808, 9223372036854775807) via lifecycle
  base <- perform instantiate(initial, ["A", "B", "C"]) via lifecycle
  extra <- perform instantiate(initial, ["A", "C"]) via lifecycle
'''
cases=[]
for name in ['duplicate-requires','duplicate-output','duplicate-failure','namespace-collision','unicode']:
    text=(W/'controls'/(name+'.mir')).read_text()
    mode='parser-reject' if name.startswith('duplicate-') else 'adapter-reject' if name=='namespace-collision' else 'origin'
    cases.append((name,text,mode,''))
for name,expr,expected in [('negative','-2',-2),('minimum','-9223372036854775808',-(1<<63)),('maximum','9223372036854775807',(1<<63)-1)]:
    cases.append((name,complete(setup+'  actual <- extra('+expr+')\n'),'execute',f'''#guard checked.isSome
#guard outcome.rejected.isNone
#guard SourceAuthoring.lookup outcome.final.source.values "actual" = some (.integer ({expected}) false)
'''))
cases.append(('negative-function',complete('''  initial <- perform register(value, [-2, -1, 0, 1, 2], -2, 2) via lifecycle
  base <- perform instantiate(initial, ["A", "B", "C"]) via lifecycle
  extra <- perform instantiate(initial, ["A", "C"]) via lifecycle
  actual <- extra(-2)
''','let negated: Int64 = -x\n  return negated'),'execute','''#guard checked.isSome
#guard outcome.rejected.isNone
#guard SourceAuthoring.lookup outcome.final.source.values "actual" = some (.integer 2 false)
'''))
cases.append(('negated-minimum-overflow',complete(setup+'  let smallest: Int64 = -9223372036854775808\n  let impossible: Int64 = -smallest\n'),'execute','''#guard checked.isSome
#guard outcome.rejected.map (fun x => x.2) = some .dynamicRejected
#guard outcome.final.writes.length = 4
#guard (SourceAuthoring.lookup outcome.final.source.values "impossible").isNone
'''))
cases.append(('annotation-negated-minimum-overflow',complete(setup.replace('[-9223372036854775808,','[--9223372036854775808,')),'adapter-range',''))
cases.append(('unused-unbound-function',(W/'controls/unused-unbound-function.mir').read_text(),'adapter-lexical',''))
results=[]
for name,text,mode,assertions in cases:
    source=OUT/(name+'.mir'); source.write_text(text)
    r=subprocess.run([str(PARSER),str(source),'--format','json'],capture_output=True,text=True)
    (OUT/(name+'-parse.json')).write_text(r.stdout); report=json.loads(r.stdout)
    record={'case':name,'mode':mode,'source_sha256':sha(source),'parser_exit':r.returncode}
    if mode=='parser-reject':
        assert not report['accepted'] and [d['code'] for d in report['diagnostics']]==['duplicate_effect_member']
    elif mode.startswith('adapter-'):
        assert r.returncode==0
        try: export(report,['A','B','C'],text)
        except ValueError as error:
            expected={'adapter-reject':'collides','adapter-range':'outside Int64','adapter-lexical':'unresolved lexical name'}[mode]
            assert expected in str(error), str(error)
            record['rejection']=str(error)
        else: raise AssertionError(name+' unexpectedly accepted')
    else:
        assert r.returncode==0
        program,place=export(report,['A','B','C'],text)
        if mode=='origin':
            offsets=[len(text[:next(iter(s.values()))['span']['start']].encode('utf-8')) for s in report['module']['transitions'][0]['body']]
            assert any(a!=next(iter(s.values()))['span']['start'] for a,s in zip(offsets,report['module']['transitions'][0]['body']))
            assertions='#guard outcome.rejected.isNone\n#guard outcome.final.writes.reverse.map SourceExecution.Write.startByte = '+str(offsets)+'\n'
        lean=OUT/(name+'.lean'); lean.write_text(generate(program,place,assertions))
        run=subprocess.run(['lean','--trust=0',str(lean)],cwd=W,env={**os.environ,'LEAN_PATH':str(W)},capture_output=True,text=True,preexec_fn=memory_limit)
        (OUT/(name+'-run.log')).write_text(run.stdout+run.stderr); record['lean_exit']=run.returncode; record['lean_sha256']=sha(lean)
        if run.returncode: print(run.stdout+run.stderr,flush=True)
        assert run.returncode==0,name
    record['passed']=True;results.append(record);print(name+': PASS',flush=True)
    (OUT/'RESULT.json').write_text(json.dumps({'parser_sha256':sha(PARSER),'adapter_sha256':sha(W/'proof_first_composition_source.py'),'cases':results},indent=2)+'\n')
