from pathlib import Path
import json,hashlib,copy,datetime
w=Path(__file__).resolve().parent;r=w/'recovered';m=json.loads((w/'COHORT_HOST_PREFIX_INPUTS_FIELD_NORMAL.json').read_text());events=json.loads(Path(m['events_path']).read_text());rows=[i for i,e in enumerate(events) if e['tag']=='cohortObserve'];live=next(i for i in rows if events[i]['observation']['snapshot'] is not None);cases=[]
def emit(label,index,field,value,lifecycle=False):
 x=copy.deepcopy(events);assert x[index]['observation'][field]!=value;x[index]['observation'][field]=value
 p=r/('field-value-control-'+label+'.json')
 with p.open('x') as f:json.dump(x,f,separators=(',',':'));f.write('\n')
 cases.append({'label':label,'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'diagnostic':'actual cohort lifecycle/ordinal mismatch' if lifecycle else 'actual cohort memory mismatch','changed_row':events[index]['row'],'field':field})
emit('BOOTSTRAP_ERASED',live,'bootstrapped',False)
nextSnapshot=next(events[i]['observation']['snapshot'] for i in rows if events[i]['observation']['snapshot'] not in (None,events[live]['observation']['snapshot']))
emit('FOREIGN_ACTUAL_SNAPSHOT',live,'snapshot',nextSnapshot)
for field,label in [('initialized','INITIALIZED_ERASED'),('freezes','FREEZE_ERASED'),('installs','INSTALL_ERASED'),('produced','PRODUCED_ERASED')]:
 i=next(i for i in rows if events[i]['observation'][field]);emit(label,i,field,events[i]['observation'][field][1:])
i=next(i for i in rows if events[i]['observation']['payment'] is not None);emit('PAYMENT_ERASED',i,'payment',None)
for field in ['gate','retired']:emit(field.upper()+'_FLIPPED',live,field,not events[live]['observation'][field],True)
emit('SOURCE_ORDINAL_BORROWED',live,'sourceOrdinal',events[live]['observation']['sourceOrdinal']+1,True)
with (w/'FIELD_VALUE_NEGATIVE_INPUTS.json').open('x') as f:json.dump({'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'scope':'ten typed mutations of actual observed rows; inventories/native raw unchanged; no fake expected state','cases':cases},f,indent=2)
old=(r/'RunFieldCohortControls.lean').read_text();s=old[:old.index('def main :')]+'def main : IO Unit := do\n';normal=(r/'RunCohortHostReplay_FIELD_NORMAL.lean').read_text().split('#eval ',1)[1].strip()
for c in cases:
 command=normal.replace(json.dumps(m['events_path']),json.dumps(c['path']));assert command!=normal
 s+='  check '+json.dumps(c['label'])+' ('+command+') (some '+json.dumps(c['diagnostic'])+')\n'
s+='#eval main\n'
with (r/'RunFieldValueControls.lean').open('x') as f:f.write(s)
print('ten actual field/lifecycle negative controls prepared, NOTRUN')
