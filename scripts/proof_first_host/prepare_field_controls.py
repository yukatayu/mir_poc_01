"""Actual-capture finite negatives for the paired strict consumer. Normalized
mutations preserve raw capture row inventory; no model-created expected values.
"""
from pathlib import Path
import json,copy,hashlib,datetime
w=Path(__file__).resolve().parent;r=w/'recovered';metadata=json.loads((w/'COHORT_HOST_PREFIX_INPUTS_FIELD_NORMAL.json').read_text());events=json.loads(Path(metadata['events_path']).read_text())
indices=[i for i,e in enumerate(events) if e['tag']=='writerStatement'];firstOwner=next(i for i,e in enumerate(events) if e['tag']=='owner');end=next(i for i in range(firstOwner,len(events)) if events[i]['tag']=='writerReturn');first=[i for i in indices if i<end]
release=next(i for i in first if events[i]['kind']=='releaseLease');clear=next(i for i in first if events[i]['kind']=='clearEntered');assert events[release]['memory']==events[clear]['memory'];cases=[]
def emit(label,changed,message):
 token=0
 for e in changed:
  if e['tag']=='writerStatement':e['token']=token;token+=1
 p=r/('field-control-'+label+'.json')
 with p.open('x') as f:json.dump(changed,f,separators=(',',':'));f.write('\n')
 cases.append(dict(label=label,path=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),diagnostic=message))
prefix='physical writer statement differs from same native recipe prefix';finish='writer returned before all actual statement occurrences';identity='physical statement owner/token order'
x=copy.deepcopy(events);del x[release];emit('OMIT_RELEASE_RENUMBERED',x,prefix)
x=[e for i,e in enumerate(copy.deepcopy(events)) if i not in (release,clear)];emit('OMIT_CLEANUP_PAIR_RENUMBERED',x,finish)
x=copy.deepcopy(events);x[release],x[clear]=x[clear],x[release];emit('SWAP_EQUAL_CLEANUP',x,prefix)
x=copy.deepcopy(events);x.insert(release,copy.deepcopy(x[release]));emit('DUPLICATE_EQUAL_RELEASE',x,prefix)
x=copy.deepcopy(events);x[first[0]]['endpoint']=(x[first[0]]['endpoint']+1)%3;emit('BORROW_OTHER_OWNER',x,identity)
x=copy.deepcopy(events);x[first[0]]['memory']['credits']+=1;emit('WRONG_STORE_VALUE',x,prefix)
x=[e for e in copy.deepcopy(events) if e['tag']!='writerStatement'];emit('OMIT_ALL_MARKERS',x,finish)
for field in ('ordinal','occurrence'):
 x=copy.deepcopy(events);x[first[0]][field]+=1;emit('BORROW_'+field.upper(),x,identity)
with (w/'FIELD_NEGATIVE_INPUTS.json').open('x') as f:json.dump(dict(at=datetime.datetime.now(datetime.timezone.utc).isoformat(),scope=__doc__,cases=cases),f,indent=2);f.write('\n')
s='import LeaseCohortCaptureJson\n'+(r/'HostControlHeader.lean').read_text()+'def main : IO Unit := do\n'
normal=(r/'RunCohortHostReplay_FIELD_NORMAL.lean').read_text().split('#eval ',1)[1].strip()
for c in cases:
 command=normal.replace(json.dumps(metadata['events_path']),json.dumps(c['path']));assert command!=normal
 s+='  check '+json.dumps(c['label'])+' ('+command+') (some '+json.dumps(c['diagnostic'])+')\n'
actual=(r/'RunCohortHostReplay_FIELD_OMISSION.lean').read_text().split('#eval ',1)[1].strip()
s+='  check "ACTUAL_PROCESS_OMISSION" ('+actual+') (some '+json.dumps(prefix)+')\n#eval main\n'
with (r/'RunFieldCohortControls.lean').open('x') as f:f.write(s)
print('ten expected-rejection controls prepared; NOT RUN')
