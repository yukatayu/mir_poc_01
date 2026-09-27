"""Typed negative mutations of the newly captured unknown-wire entry profile.
These are finite rejection tests; no generated record replaces actual evidence.
"""
from pathlib import Path
from copy import deepcopy
import json,hashlib
w=Path(__file__).resolve().parent;rec=w/'recovered';tag='LEASEWIRE_ENTRY_RAW_CAPTURE'
source_path=rec/'HOST_LEASE_WIRE_PREFIX_ENTRY_RAW_CAPTURE.json'
source=json.loads(source_path.read_text());events=json.loads((rec/('cohort-events_'+tag+'.json')).read_text())
stop=next(i for i,e in enumerate(events) if e['tag']=='wireStop')
retired=next(i for i,e in enumerate(events) if e['tag']=='retired')
release=next(i for i in range(retired+1,len(events)) if events[i]['tag']=='gateRelease')
observation=next(i for i in range(stop+1,len(events)) if events[i]['tag']=='cohortObserve')
probe=next(i for i in range(release+1,len(events)) if events[i]['tag']=='gateEnter')
controls=[]
def add(label,values,error):
 p=rec/('wire-lease-control-'+label+'.json')
 with p.open('x') as f:json.dump(values,f,separators=(',',':'));f.write('\n')
 controls.append(dict(label=label,path=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),error=error))
m=deepcopy(events);del m[stop];add('OMIT_ACTUAL_STOP',m,'actual cohort lifecycle/ordinal mismatch')
m=deepcopy(events);m.insert(stop+1,{'tag':'cohortCommit'});add('COMMIT_AFTER_UNKNOWN',m,'semantic event after interrupted wire')
m=deepcopy(events);m[observation]['observation']['sourceOrdinal']-=1;add('ROLLBACK_ATTEMPT_COUNTER',m,'actual cohort lifecycle/ordinal mismatch')
m=deepcopy(events);del m[retired];add('OMIT_UNKNOWN_RETIREMENT',m,'actual cohort lifecycle/ordinal mismatch')
m=deepcopy(events);item=m.pop(release);m.insert(retired,item);add('RELEASE_UNKNOWN_BEFORE_RETIRE',m,'unconfirmed release before retirement or duplicate release')
m=deepcopy(events);m.insert(retired+1,deepcopy(m[retired]));add('DOUBLE_UNKNOWN_RETIRE',m,'unconfirmed retirement refused or duplicated')
m=deepcopy(events);m.insert(release+1,deepcopy(m[release]));add('DOUBLE_UNKNOWN_RELEASE',m,'unconfirmed release before retirement or duplicate release')
target=source['wire_fault']['entry_owner']
m=deepcopy(events);m[retired]['writers'][target]['lease']=None;add('ERASE_UNKNOWN_LEASE',m,'retirement erased writer obligation')
m=deepcopy(events);del m[observation];add('OMIT_UNKNOWN_OBSERVATION',m,'whole-cohort observation inventory missing/duplicated/reordered')
m=deepcopy(events);m[stop]['cut']='bodyLost';add('ERASE_CAPTURED_KNOWLEDGE',m,'normalized interruption differs from actual full retained custody')
m=deepcopy(events);i=next(i for i,e in enumerate(m) if e['tag']=='writerStatement' and e['kind']=='releaseLease');del m[i]
token=0
for e in m:
 if e['tag']=='writerStatement':e['token']=token;token+=1
add('OMIT_IDEMPOTENT_RELEASE',m,'physical writer statement differs from same native recipe prefix')
m=deepcopy(events);i=next(i for i,e in enumerate(m) if e['tag']=='writerStatement');m[i]['occurrence']+=1;add('BORROW_STATEMENT_OCCURRENCE',m,'physical statement owner/token order')
m=deepcopy(events);m[retired]['writers'][target]['entered']=True;add('INVENT_ENTERED_CONFIRMATION',m,'retirement erased writer obligation')
m=deepcopy(events);i=next(i for i in range(probe+1,len(m)) if m[i]['tag']=='cohortObserve');m[i]['observation']['bootstrapped']=False;add('MUTATE_RETIRED_PROBE_MEMORY',m,'actual cohort memory mismatch')
m=deepcopy(events);m.insert(probe+1,{'tag':'source','ordinal':source['wire_fault']['attempted_ordinal']+1});add('ADVANCE_AFTER_RETIRED_PROBE',m,'semantic event after interrupted wire')
m=deepcopy(events);del m[max(i for i,e in enumerate(m) if e['tag']=='failedEnd')];add('OMIT_FINAL_FAILED_END',m,'unfinished outer span')
base='import LeaseWireCaptureJson\n'+(rec/'HostControlHeader.lean').read_text()
text=base+'def main : IO Unit := do\n';args=' '.join(source['source']['numeric_arguments']);trees=str(rec/('source-entry-journal-trees_'+tag));count=len(source['host_commit_trace'])
for c in controls:
 text+='  check '+json.dumps(c['label'])+' (LeaseWireCaptureJson.run '+json.dumps(source['root'])+' '+json.dumps(trees)+' '+json.dumps(c['path'])+' '+json.dumps(str(source_path))+' '+args+'\n  ⟨#[8,8,8],by decide⟩ '+str(count)+') (some '+json.dumps(c['error'])+')\n'
text+='#eval main\n'
with (rec/'RunWireLeaseControls.lean').open('x') as f:f.write(text)
with (w/'WIRE_LEASE_CONTROLS_INPUTS.json').open('x') as f:json.dump(controls,f,indent=2);f.write('\n')
print(len(controls),'typed controls prepared; NOT RUN')
