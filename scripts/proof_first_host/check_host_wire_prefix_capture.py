"""Finite exact attempted-wire provenance/recipe and all post-arm local fields.
Authentic private capture is a premise. A peer ordinal is an attempted IO
counter, not native application or settled source knowledge. No expected return
is fabricated to turn a failed public invocation into a complete recipe.
"""
from pathlib import Path
import copy
from collections import Counter
from check_host_commit_capture import check_host_order,check_gate_lifetime,observation_calls,check_native_observation_bytes
from check_writer_parent_occurrences import check as parent_check
from check_joint_capture_recipes import check as literal_check,input_fact,source_form,entry,h

def check(r):
 from check_joint_capture_recipes import check_nested_rejections
 check_nested_rejections(r)
 assert r['state']=='failed' and r['error'].startswith('WireInterrupted:')
 f=r['wire_fault'];cut=f['cut'];profile=f['target_profile']
 p=int(r['source']['numeric_arguments'][1])
 assert cut in ('BEFORE_WRITE','BODY_LOST','RAW_CAPTURE') and profile in ('ENTRY','OWNER')
 assert r['error']=='WireInterrupted: '+cut,'wire fault label/exception mismatch'
 assert f['retired'] and f['gate_released'] and f['public_refusals']==p+2
 assert f['protocol_calls']==[] and f['observed_descriptors']==8
 check_host_order(r);check_gate_lifetime(r);parents=parent_check(r);calls=observation_calls(r)
 check_native_observation_bytes(r)
 rows=r['host_commit_trace'];spans=r['public_spans'];seq=r['stream_sequence'];root=Path(r['root'])
 failed=[s for s in spans if s.get('error')=='WireInterrupted'];assert len(failed)==1;call=failed[0]
 assert call['kind']=='invoke' and call['depth']==0 and call['outcome']=='raised' and input_fact(call) is None
 assert call['message']==cut,'wire failed-call cut mismatch'
 assert call['stop']==len(seq)==f['native_position']
 arm_rows=[x for x in rows if x['point']=='wire_armed'];fault_rows=[x for x in rows if x['point']=='wire_interrupted'];retired_rows=[x for x in rows if x['point']=='fault_retired']
 assert len(arm_rows)==len(fault_rows)==len(retired_rows)==1
 arm,interrupt,retired=arm_rows[0],fault_rows[0],retired_rows[0]
 assert interrupt['cut']==retired['cut']==cut,'wire marker cut mismatch'
 assert retired['target_profile']==profile,'wire retirement target profile mismatch'
 assert arm['sequence']==f['armed_sequence']<interrupt['sequence']<retired['sequence']
 assert all(calls[x['sequence']]==call['id'] for x in (arm,interrupt,retired))
 assert arm['state']['gate'] and not arm['state']['retired']
 assert not retired['state']['gate'] and retired['state']['retired']
 endpoint=f['endpoint'];ordinal=f['attempted_ordinal'];endpoint_rows=[x for x in seq if x['endpoint']==endpoint]
 assert endpoint_rows and ordinal==endpoint_rows[-1]['index']+1==f['last_returned_ordinal']+1
 assert arm['endpoint']==interrupt['endpoint']==endpoint and arm['attempted_ordinal']==interrupt['attempted_ordinal']==ordinal
 assert arm['process_id']==r[endpoint]['pid']
 origin=arm['wire_origin']
 assert origin==interrupt['wire_origin']==f['wire_origin'],'unknown caller origin changed across retained attempt'
 assert origin['caller_frame']==arm['caller_frame'] and origin['endpoint']==endpoint and origin['process_id']==arm['process_id'],'unknown caller occurrence mismatch'
 if profile=='OWNER':
  i=int(endpoint[5:]);prior=next(x for x in reversed(rows[:arm['sequence']]) if x['point']=='native_return' and x['endpoint']==endpoint)['expected_parent']
  assert origin['owner']==i and origin['writer']==prior['writer'] and origin['cohort']==prior['cohort'],'unknown writer/cohort association'
  assert origin['caller_function']=='send' and origin['parent_function']=='owner_send' and origin['caller_frame']!=origin['parent_frame'],'unknown writer parent boundary'
 else:
  assert origin['caller_function']=='source_send' and origin['caller_frame']==arm['source_frame'] and origin['cohort']==arm['cohort'],'unknown source exact caller'

 payload=(root/'wire-fault/input.bin').read_bytes()
 assert payload==bytes.fromhex(arm['payload_hex']) and h.digest(payload)==arm['payload_sha256']==f['payload_sha256']
 assert len(payload)<=65536 and h.encode(h.decode(payload))==payload
 assert f['before_writers']==arm['state']['owners']
 actual_input=root/endpoint/f'{ordinal:03}-input.bin';actual_output=root/endpoint/f'{ordinal:03}-output.bin'
 observed=root/'wire-fault/observed-reply.bin'
 assert not actual_output.exists(),'unconfirmed reply inserted into settled capture'
 if cut=='BEFORE_WRITE':
  assert f['peer_ordinal']==ordinal-1 and not actual_input.exists() and not observed.exists()
  assert f['observer_reply_sha256'] is None and f['host_raw_sha256'] is None and not f['raw_retained']
 else:
  assert f['peer_ordinal']==ordinal and actual_input.read_bytes()==payload
  assert (root/endpoint/f'{ordinal:03}-input-frame.bin').read_bytes()==len(payload).to_bytes(4,'big')+payload
  body=observed.read_bytes();assert h.digest(body)==f['observer_reply_sha256']
  if cut=='RAW_CAPTURE':assert f['raw_retained'] and f['host_raw_sha256']==f['observer_reply_sha256']
  else:assert not f['raw_retained'] and f['host_raw_sha256'] is None
 # Compare full immutable outstanding and last-confirmed occurrences at the
 # actual injection, propagated boundary and after EACH exported retired probe.
 # This is selected-boundary custody, not instruction-level noninterference.
 previous=ordinal-1
 expected_slot=dict(endpoint=endpoint,process_id=r[endpoint]['pid'],ordinal=ordinal,payload_hex=payload.hex(),expect_reply=True,reply_hex=observed.read_bytes().hex() if cut=='RAW_CAPTURE' else None)
 expected_last=dict(endpoint=endpoint,process_id=r[endpoint]['pid'],ordinal=previous,payload_hex=(root/endpoint/f'{previous:03}-input.bin').read_bytes().hex(),expect_reply=True,reply_hex=(root/endpoint/f'{previous:03}-output.bin').read_bytes().hex())
 custody=dict(outstanding=expected_slot,last_confirmed=expected_last)
 assert r['wire_custody_expected']==custody,'unknown injection complete custody mismatch'
 samples=r['wire_custody_trace']
 assert [x['stage'] for x in samples]==['injection','propagated']+['probe_'+str(i) for i in range(p+2)],'unknown custody observation inventory'
 assert all(x['actual']==custody for x in samples),'unknown retained occurrence changed'
 # All tail observations are checked, including changes later restored.
 expected=copy.deepcopy(arm['state']['owners']);failed_owner=False
 ignored={'owners','ordinal','retired','gate'}
 cohort_fields={k:v for k,v in arm['state'].items() if k not in ignored}
 for row in rows[arm['sequence']+1:]:
  assert row['point'] in ('wire_interrupted','host_statement','fault_retired'),'unexpected local/native completion after arming'
  assert row['native_position']==f['native_position']
  state=row['state'];assert {k:v for k,v in state.items() if k not in ignored}==cohort_fields,'cohort field changed during unresolved IO'
  assert state['ordinal']==(f['peer_ordinal'] if endpoint=='source' else arm['state']['ordinal']),'attempt counter versus settled source ordinal'
  if profile=='OWNER':
   i=int(endpoint[5:]);flag=state['owners'][i]['failed'];assert isinstance(flag,bool) and (not failed_owner or flag)
   if flag and not failed_owner:assert row['function']=='send' and row['sequence']>interrupt['sequence'],'writer retirement before interruption'
   failed_owner=flag;expected[i]['failed']=flag
  assert state['owners']==expected,'unresolved IO changed retained writer fields'
 assert (profile!='OWNER' or failed_owner) and f['after_writers']==expected
 probes=[s for s in spans if s['host_start']>call['host_stop']]
 assert len(probes)==p+2
 assert Counter((s['kind'],s['endpoint']) for s in probes)==Counter([('invoke',None),('source',None)]+[('owner',i) for i in range(p)]),'wire retired probe inventory'
 for probe in probes:
  input_fact(probe)
  assert probe['depth']==0 and probe['start']==probe['stop']==call['stop'] and probe['outcome']=='raised'
  assert (probe['error'],probe['message'])==('RuntimeError','cohort retired')
 # Keep the literal checker unchanged for every fully completed preceding call.
 order=r['host_order'][:call['host_start']];count=sum(e['kind']=='observation' for e in order)
 prefix=dict(r,stream_sequence=seq[:call['start']],host_order=order,host_commit_trace=rows[:count],
   public_spans=[s for s in spans if s['host_stop']<call['host_start']])
 prior=literal_check(prefix)
 def read(event):
  stem=root/event['endpoint']/f"{event['index']:03}"
  return h.decode(Path(str(stem)+'-input.bin').read_bytes()),Path(str(stem)+'-output.bin').read_bytes()
 snapshot=None
 for event in seq[:call['start']]:
  if event['endpoint']=='source' and event['reply']:
   command,response=read(event)
   if source_form(command)[0]=='execute':snapshot=h.capacity_reply(response)[1]
 assert snapshot is not None and snapshot['dispatch'] is None
 ticket=snapshot['source']['pending'];assert ticket is not None
 target=h.product_fields(ticket,15)[3][1];p=int(r['source']['numeric_arguments'][1]);scope=int(r['source']['numeric_arguments'][6]);assert 0<=target<p
 before=rows[count-1]['state'];owner=before['owners'][target]
 assert before['initialized']==list(range(p)) and owner['credits']>=3 and owner['keys'] is not None
 key=[h.product_fields(ticket,15)[i][1] for i in (4,5)]
 assert key not in owner['keys'] and len(owner['keys'])<int(r['owner'+str(target)]['numeric_arguments'][5])
 installed,fence=snapshot['installed'],snapshot['fence']
 assert all(v[0:2]==('node',0) and len(v[2])==p for v in (installed,fence))
 if installed[2][target]==fence[2][target]:
  assert owner['revision']==snapshot['published']
  assert owner['image'] is not None and bytes.fromhex(r['host_tree_bytes'][owner['image']])==h.encode(snapshot['source']['image'])
 def cost(tree):
  if tree[0]!='node':return 1
  fields=0
  for child in reversed(tree[2]):fields=1+max(cost(child),fields)
  return 1+fields
 assert len(h.encode(ticket))+372+12*p+25<=65536 and max(p+2,cost(ticket)+24)+7<=256
 partial=seq[call['start']:call['stop']]
 if profile=='ENTRY':
  assert not partial and endpoint=='source' and f['entry_owner']==target
  form,value=source_form(h.decode(payload));assert form=='execute' and value[1]==entry(target)
  assert arm['ticket']==h.digest(h.encode(ticket)) and arm['owner']==target
 else:
  assert len(partial)==4 and endpoint=='owner'+str(target)
  assert [e['endpoint'] for e in partial]==['source','source',endpoint,endpoint]
  events=[read(e) for e in partial];form,value=source_form(events[0][0]);assert form=='execute' and value[1]==entry(target)
  status,entered=h.capacity_reply(events[0][1]);assert status==0
  place,context_scope,revision,retained=h.product_fields(entered['dispatch'],4)
  assert (place,context_scope,revision,retained)==(h.nat(target),h.nat(scope),h.nat(snapshot['published']),ticket)
  assert events[1][0]==h.left(h.right(h.right(h.nat(target)))) and h.un_bool(h.decode(events[1][1]))
  assert events[2][0]==h.left(h.right(h.left(ticket))) and h.decode(events[2][1])==h.left(h.nat(6))
  assert events[3][0]==h.right(h.nat(snapshot['published']+1)) and h.decode(events[3][1])==h.left(h.nat(2))
  assert h.decode(payload)==h.left(h.right(h.right(h.left(h.unit))))
 return dict(completed_literal_prefix=prior,parents=parents,fault_call=call['id'],known_partial_events=len(partial),
   arm_sequence=arm['sequence'],interrupt_sequence=interrupt['sequence'],retire_sequence=retired['sequence'],
   checked_tail_observations=len(rows)-arm['sequence']-1,scope=__doc__)
