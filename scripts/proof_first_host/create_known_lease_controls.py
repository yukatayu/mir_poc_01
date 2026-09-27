"""Typed mutations of the real NOTIFIED whole-cohort normalized trace."""
from pathlib import Path
from copy import deepcopy
import json,hashlib
w=Path(__file__).resolve().parent;rec=w/'recovered';tag='LEASEFINAL_NOTIFIED'
events=json.loads((rec/('cohort-events_'+tag+'.json')).read_text());source=json.loads((rec/'HOST_LEASE_KNOWN_FAULT_NOTIFIED.json').read_text())
retired=next(i for i,e in enumerate(events) if e['tag']=='retired')
release=next(i for i in range(retired+1,len(events)) if events[i]['tag']=='gateRelease')
probe=next(i for i in range(release+1,len(events)) if events[i]['tag']=='gateEnter')
observed=next(i for i in range(retired+1,len(events)) if events[i]['tag']=='cohortObserve')
controls=[]
def add(label,events,error):
 p=rec/('known-lease-control-'+label+'.json')
 with p.open('x') as f:json.dump(events,f,separators=(',',':'));f.write('\n')
 controls.append(dict(label=label,path=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),error=error))
mutation=deepcopy(events);del mutation[retired];add('OMIT_RETIREMENT',mutation,'actual cohort lifecycle/ordinal mismatch')
mutation=deepcopy(events);item=mutation.pop(release);mutation.insert(retired,item);add('RELEASE_BEFORE_RETIREMENT',mutation,'outer call ended before consumed source finish')
mutation=deepcopy(events);mutation.insert(retired+1,{'tag':'cohortCommit'});add('COMMIT_AFTER_RETIREMENT',mutation,'cohort store without same-history debt')
mutation=deepcopy(events);mutation.insert(retired,{'tag':'cohortCommit'});add('BYPASS_UNSTORED_ENTRY',mutation,'cohort store without same-history debt')
mutation=deepcopy(events);mutation[retired]['writers'][0]['credits']-=1;add('ERASE_RETAINED_WRITER',mutation,'retirement erased writer obligation')
mutation=deepcopy(events);mutation[observed]['observation']['snapshot']=None;add('ERASE_RETIRED_SNAPSHOT',mutation,'actual cohort memory mismatch')
mutation=deepcopy(events);mutation[observed]['observation']['sourceOrdinal']-=1;add('ROLLBACK_NATIVE_ORDINAL',mutation,'actual cohort lifecycle/ordinal mismatch')
mutation=deepcopy(events);del mutation[probe];add('OMIT_RETIRED_PROBE_ACQUIRE',mutation,'actual cohort lifecycle/ordinal mismatch')
mutation=deepcopy(events);del mutation[observed];add('OMIT_RETIREMENT_OBSERVATION',mutation,'whole-cohort observation inventory missing/duplicated/reordered')
mutation=deepcopy(events);end=next(i for i in range(retired+1,len(events)) if events[i]['tag']=='failedEnd');mutation[end]={'tag':'finish'};add('FALSE_PUBLIC_COMPLETION',mutation,'outer return before source entry stores')
# Removing an idempotent release and renumbering later tokens must still
# fail its actual same-recipe statement path, not merely a counter gap.
mutation=deepcopy(events);omitted=next(i for i,e in enumerate(mutation) if e['tag']=='writerStatement' and e['kind']=='releaseLease');del mutation[omitted]
token=0
for e in mutation:
 if e['tag']=='writerStatement':e['token']=token;token+=1
add('OMIT_IDEMPOTENT_RELEASE',mutation,'physical writer statement differs from same native recipe prefix')
mutation=deepcopy(events);i=next(i for i,e in enumerate(mutation) if e['tag']=='writerStatement');mutation[i]['occurrence']+=1;add('BORROW_STATEMENT_OCCURRENCE',mutation,'physical statement owner/token order')
mutation=deepcopy(events);target=source['source_entry_fault']['owner'];mutation[retired]['writers'][target]['lease']=None;add('ERASE_ACTIVE_ENTRY_LEASE',mutation,'retirement erased writer obligation')
mutation=deepcopy(events);i=next(i for i in range(probe+1,len(events)) if mutation[i]['tag']=='cohortObserve');mutation[i]['observation']['bootstrapped']=False;add('MUTATE_RETIRED_PROBE_MEMORY',mutation,'actual cohort memory mismatch')
base='import LeaseKnownFaultCaptureJson\n'+(rec/'HostControlHeader.lean').read_text()
text=base+'def main : IO Unit := do\n'
args=' '.join(source['source']['numeric_arguments']);trees=str(rec/('source-entry-journal-trees_'+tag));count=len(source['host_commit_trace'])
for c in controls:
 text+='  check '+json.dumps(c['label'])+' (LeaseKnownFaultCaptureJson.run '+json.dumps(source['root'])+' '+json.dumps(trees)+' '+json.dumps(c['path'])+' '+args+'\n  ⟨#[8,8,8],by decide⟩ '+str(count)+') (some '+json.dumps(c['error'])+')\n'
text+='#eval main\n'
with (rec/'RunKnownLeaseControls.lean').open('x') as f:f.write(text)
with (w/'KNOWN_LEASE_CONTROLS_INPUTS.json').open('x') as f:json.dump(controls,f,indent=2);f.write('\n')
print(len(controls),'typed real-capture negatives prepared, NOT RUN')
