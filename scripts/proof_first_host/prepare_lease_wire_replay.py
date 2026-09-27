"""One actual host/native order feeds proof-carrying wire and writer intervals.
No expected model replies or synthetic completion events are introduced.
"""
from pathlib import Path
import sys,os
if sys.flags.optimize or not __debug__:
    raise RuntimeError('evidence preparer requires Python assertions')
import argparse,datetime,hashlib,json,re
from check_writer_parent_occurrences import check
from check_joint_capture_recipes import check as check_literal,check_nested_rejections
from check_host_commit_capture import check_native_observation_bytes
w=Path(__file__).resolve().parent;rec=w/'recovered';sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
parser=argparse.ArgumentParser();parser.add_argument('tag',nargs='?',default='');parser.add_argument('--capture',choices=['HOST_LEASE_WIRE_PREFIX_'+profile+'_'+cut+suffix for profile in ('ENTRY','OWNER') for cut in ('BEFORE_WRITE','BODY_LOST','RAW_CAPTURE') for suffix in ('','_MIXED')],required=True);options=parser.parse_args()
assert not options.tag or re.fullmatch('[A-Z][A-Z0-9_]*',options.tag)
tag='_'+options.tag if options.tag else ''
path=rec/(options.capture+'.json');r=json.loads(path.read_text());root=Path(r['root'])
input_path=w/('SOURCE_ENTRY_WRITER_JOURNAL_INPUTS'+tag+'.json')
wire_profile=options.capture.startswith('HOST_LEASE_WIRE_PREFIX_')
fault_profile=wire_profile;assert wire_profile
assert r['state']==('failed' if fault_profile else 'passed');inputs=json.loads(input_path.read_text());assert inputs['receipt_sha256']==sha(path)
for row in inputs['raw']:assert sha(Path(row['path']))==row['sha256']
for endpoint in ['source']+['owner'+str(i) for i in range(int(r['source']['numeric_arguments'][1]))]:
 assert sha(Path(r[endpoint]['binary']))==r[endpoint]['binary_sha256']
 if fault_profile:assert r[endpoint]['state']=='reaped' and r[endpoint]['exit']==-9
 else:assert r[endpoint]['exit']==0 and r[endpoint]['stdout_eof'] is True
occurrence=check(r)
check_nested_rejections(r)
check_native_observation_bytes(r)
if wire_profile:
 from check_host_wire_prefix_capture import check as check_wire
 fault_checks=check_wire(r)
elif fault_profile:
 from check_source_entry_fault_capture import check as check_fault
 fault_checks=check_fault(r)
 assert options.capture in ('HOST_COHORT_KNOWN_FAULT_'+fault_checks['cut'],'HOST_COHORT_KNOWN_FAULT_V2_'+fault_checks['cut'],'HOST_COHORT_KNOWN_FAULT_V2_'+fault_checks['cut']+'_MIXED'),'named capture/cut mismatch'
else:check_literal(r);fault_checks=None
def observed(owner):
 assert owner['failed'] is False or (r['wire_fault']['target_profile']=='OWNER' and owner==r['wire_fault']['after_writers'][int(r['wire_fault']['endpoint'][5:])]),'failed flag lacks checked actual writer-stop provenance'
 assert type(owner['credits']) is int and owner['credits']>=0 and type(owner['revision']) is int and owner['revision']>=0 and type(owner['entered']) is bool
 def key(value):
  if value is None:return 'none'
  assert value in r['host_tree_bytes'];return '(some '+json.dumps(value)+')'
 keys=owner['keys']
 if keys is None:shown='none'
 else:
  assert all(len(pair)==2 and all(type(n) is int and n>=0 for n in pair) for pair in keys)
  shown='(some ['+','.join('('+str(x)+','+str(y)+')' for x,y in keys)+'])'
 return '⟨'+','.join([str(owner['credits']),str(owner['revision']),key(owner['image']),shown,key(owner['lease']),str(owner['entered']).lower()])+'⟩'
from bind_source_entry_observations import bind
binder_receipt=r
if wire_profile:
 binder_receipt=dict(r,host_commit_trace=r['host_commit_trace'][:r['wire_fault']['armed_sequence']+1])
entry_intervals,entry_records,entry_files=bind(binder_receipt,rec/('source-entry-journal-trees'+tag),observed,
 allow_retired_prefix=fault_profile and not wire_profile,pending_wire=r['wire_fault'] if wire_profile and r['wire_fault']['target_profile']=='ENTRY' else None)
claimed_rows={x['claimed_sequence']:x for x in entry_records}
store_rows={s['sequence']:(s['kind'],x) for x in entry_records for s in x['stores']}
assert len(claimed_rows)==len(entry_records)
if not fault_profile:assert len(store_rows)==2*len(entry_records)
entry_ordinals={x['ordinal'] for x in entry_records if not x.get('pending_wire')};assert len(entry_ordinals)==len([x for x in entry_records if not x.get('pending_wire')])
claim_uses=set();store_uses=set()
from bind_cohort_store_observations import bind as bind_cohort_stores
from bind_cohort_fault_values import bind as bind_cohort_values
from bind_writer_statement_observations_v2 import bind as bind_writer_statements
writer_commits,writer_statement_intervals=bind_writer_statements(r)
statement_uses=set()
cohort_commits=bind_cohort_stores(r)
cohort_rows,cohort_files=bind_cohort_values(r,rec/('source-entry-journal-trees'+tag))
entry_files+=cohort_files
assert fault_profile,'explicit known-fault profile'
events=[];active=None;previous=None;samples=writers=0;consumed_entries=set()
call_stack=[];gate=False;retired=False;consumed_cohort=set()
assert [x['observation'] for x in r['host_order'] if x['kind']=='observation']==list(range(len(r['host_commit_trace']))),'host-order observation coverage'
for point in r['host_order']:
 if point['kind']=='call_begin':
  call_stack.append(point['call'])
  events.append('.begin' if r['public_spans'][point['call']]['depth']==0 else '.rejectedReentry')
 elif point['kind']=='call_end':
  assert call_stack.pop()==point['call'],'outer call stack'
  span=r['public_spans'][point['call']]
  if span['depth']==0:
   # Earlier completed live refusals retain their normal boundary when an
   # independent later suffix retires the cohort.
   retired_boundary=fault_profile and span['id']>=fault_checks['fault_call']
   if retired_boundary:assert span['outcome']=='raised'
   events.append('.failedEnd' if retired_boundary else '.finish')
 else:
  row=r['host_commit_trace'][point['observation']]
  sequence=row['sequence']
  if row['state']['gate']!=gate:
   assert call_stack and r['public_spans'][call_stack[-1]]['depth']==0,'actual gate edge outside sole outer call'
   events.append('.gateEnter '+str(call_stack[-1]) if row['state']['gate'] else '.gateRelease')
   gate=row['state']['gate']
  if row['state']['retired']!=retired:
   assert call_stack and gate and row['state']['retired'] is True,'known retirement outside held public gate'
   events.append('.retired ['+','.join(observed(x) for x in row['state']['owners'])+'] '+json.dumps('source-snapshot-'+row['state']['source_snapshot']));retired=True
  if sequence in cohort_commits:
   marker=cohort_commits[sequence];consumed_cohort.add(marker['token'])
   if sequence in store_rows:
    assert store_rows[sequence][0]=='snapshot' and marker['kind']=='source_snapshot','two journals share only same snapshot store'
   else:events.append('.cohortCommit')
  if sequence in writer_commits:
   commit=writer_commits[sequence];assert active==commit['owner'],'physical statement before actual native open'
   events.append('.writerStatement '+str(active)+' '+str(commit['custody']['ordinal'])+' '+str(commit['occurrence'])+' '+str(commit['token'])+' '+json.dumps(commit['kind'])+' '+observed(row['state']['owners'][active]))
   statement_uses.add(commit['token'])
  if row['point']=='wire_interrupted':
   assert wire_profile and active is None
   f=r['wire_fault'];target=f['entry_owner'] if f['target_profile']=='ENTRY' else int(f['endpoint'][5:])
   cut={'BEFORE_WRITE':'beforeWrite','BODY_LOST':'bodyLost','RAW_CAPTURE':'rawCapture'}[f['cut']]
   events.append('.wireStop '+str(f['target_profile']=='ENTRY').lower()+' '+str(target)+' '+str(f['attempted_ordinal'])+' .'+cut+' ['+','.join(observed(x) for x in row['state']['owners'])+'] '+json.dumps('source-snapshot-'+row['state']['source_snapshot']))
  elif row['point']=='fault_retired':
   assert fault_profile and active is None
   assert retired and not gate,'fault propagation before actual retirement/cleanup'
  elif sequence in claimed_rows:
   assert active is None
   record=claimed_rows[sequence];owner=record['binding']['owner'];first=r['host_commit_trace'][record['before_sequence']]
   events.append(('.sourceWireClaim ' if record.get('pending_wire') else '.sourceClaim ')+str(record['ordinal'])+' '+str(owner)+' '+observed(first['state']['owners'][owner])+' '+observed(row['state']['owners'][owner])+' '+json.dumps('source-snapshot-'+first['state']['source_snapshot']))
   assert sequence not in claim_uses;claim_uses.add(sequence)
  elif sequence in store_rows:
   assert active is None
   kind,record=store_rows[sequence];owner=record['binding']['owner']
   events.append('.sourceStore .'+kind+' '+str(owner)+' '+observed(row['state']['owners'][owner])+' '+json.dumps('source-snapshot-'+row['state']['source_snapshot']))
   assert sequence not in store_uses;store_uses.add(sequence)
  elif row['point']=='native_return':
   assert active is None
   if row['endpoint']=='source':
    if row['ordinal']==1:events.append('.bootstrapReturned')
    if row['ordinal']>1:
     if row['ordinal'] in entry_ordinals:
      assert row['ordinal'] not in consumed_entries,'source entry consumed twice'
      consumed_entries.add(row['ordinal'])
      events.append('.source '+str(row['ordinal']))
     else:events.append('.source '+str(row['ordinal']))
   else:
    i=int(row['endpoint'][5:]);active=i;previous=observed(row['state']['owners'][i]);writers+=1
    events.append(f'.owner {i} {row["ordinal"]} '+previous)
  elif row['point']=='writer_return':
   assert active is not None
   events.append('.writerReturn '+str(active)+' '+observed(row['state']['owners'][active]));active=None;previous=None
  elif active is not None:
   value=observed(row['state']['owners'][active])
   if value!=previous:events.append('.writerObserve '+str(active)+' '+value);previous=value;samples+=1
  events.append('.cohortObserve '+str(sequence)+' '+cohort_rows[sequence])
assert not call_stack and not gate and retired and consumed_cohort==set(range(len(cohort_commits)))
assert active is None and writers==occurrence['writer_occurrences'] and samples>0
assert statement_uses==set(range(len(writer_commits))),'physical statement coverage'
assert consumed_entries==entry_ordinals,'source entry evidence left unconsumed'
assert claim_uses==set(claimed_rows) and store_uses==set(store_rows),'source entry local marker coverage'
args=r['source']['numeric_arguments'];caps=[int(r['owner'+str(i)]['numeric_arguments'][-1]) for i in range(int(args[1]))];trees=rec/('source-entry-journal-trees'+tag)
from lease_wire_event_json import convert
values=[convert(event) for event in events]
assert [x['row'] for x in values if x['tag']=='cohortObserve']==list(range(len(r['host_commit_trace'])))
events_path=rec/('cohort-events'+tag+'.json')
with events_path.open('x') as f:json.dump(values,f,separators=(',',':'));f.write('\n')
runner=rec/('RunCohortWireReplay'+tag+'.lean');text='import LeaseWireCaptureJson\n#eval LeaseWireCaptureJson.run '+json.dumps(str(root))+' '+json.dumps(str(trees))+' '+json.dumps(str(events_path))+' '+json.dumps(str(path))+' '+' '.join(args)+'\n  ⟨#['+','.join(map(str,caps))+'],by decide⟩ '+str(len(r['host_commit_trace']))+'\n'
with runner.open('x') as f:f.write(text)
out=dict(python=dict(executable=sys.executable,optimize=sys.flags.optimize,version=sys.version,cache_prefix=sys.pycache_prefix,dont_write_bytecode=sys.dont_write_bytecode),python_imports=[dict(module=name,origin=os.path.abspath(module.__file__),path=str(Path(module.__file__).resolve()),sha256=sha(Path(module.__file__).resolve())) for name,module in sorted(sys.modules.items()) if getattr(module,"__file__",None) and (Path(os.path.abspath(module.__file__)).is_relative_to(w) or Path(module.__file__).resolve().is_relative_to(w))],at=datetime.datetime.now(datetime.timezone.utc).isoformat(),status='prepared, not executed',receipt_sha256=sha(path),generator_sha256=sha(Path(__file__)),consumer_sha256=sha(rec/'LeaseWireCaptureReplay.lean'),decoder_sha256=sha(rec/'LeaseWireCaptureJson.lean'),converter_sha256=sha(w/'lease_wire_event_json.py'),events_path=str(events_path),events_sha256=sha(events_path),runner_sha256=sha(runner),occurrence_checker_sha256=sha(w/'check_writer_parent_occurrences.py'),literal_checker_sha256=sha(w/'check_joint_capture_recipes.py'),writer_input_sha256=sha(input_path),source_entry_binder_sha256=sha(w/'bind_source_entry_observations.py'),writer_statement_binder_sha256=sha(w/'bind_writer_statement_observations_v2.py'),writer_statements=len(writer_commits),writer_statement_intervals=writer_statement_intervals,cohort_stores=len(cohort_commits),cohort_observations=len(cohort_rows),source_entries=entry_records,source_snapshot_files=entry_files,events=len(events),writer_intervals=writers,observed_prefixes=samples,fault_checks=fault_checks,scope=__doc__)
with (w/('COHORT_HOST_PREFIX_INPUTS'+tag+'.json')).open('x') as f:json.dump(out,f,indent=2);f.write('\n')
print(json.dumps(out))
