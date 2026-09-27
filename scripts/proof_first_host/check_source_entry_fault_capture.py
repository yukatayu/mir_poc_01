"""Finite actual known-entry fault boundary, separate from complete call recipe.
The unchanged literal checker covers every preceding complete outer call. The
last actual call is a one-native-reply prefix and all later probes emit no IO.
Privileged capture truthfulness remains a premise; no general Python theorem.
"""
from check_host_commit_capture import check_host_order,check_gate_lifetime
from check_joint_capture_recipes import check as literal_check,input_fact,source_form,entry,h
from pathlib import Path
from collections import Counter
import hashlib,json

def check(receipt):
 from check_joint_capture_recipes import check_nested_rejections
 check_nested_rejections(receipt)
 assert receipt['state']=='failed' and receipt['error'].startswith('SourceEntryInterrupted:')
 fault=receipt['source_entry_fault'];assert fault['cut'] in ('UNNOTIFIED','NOTIFIED','SNAPSHOT')
 cut=fault['cut'];p=int(receipt['source']['numeric_arguments'][1])
 assert receipt['error']=='SourceEntryInterrupted: '+cut,'fault label/exception mismatch'
 assert type(fault['entered']) is bool and fault['entered'] is (cut!='UNNOTIFIED'),'named fault entered phase'
 assert type(fault['snapshot_stored']) is bool and fault['snapshot_stored'] is (cut=='SNAPSHOT'),'named fault snapshot flag'
 assert fault['retired'] and fault['gate_released'] and fault['public_refusals']==p+2
 assert fault['protocol_calls']==[] and fault['observed_descriptors']==8
 check_host_order(receipt);check_gate_lifetime(receipt)
 spans=receipt['public_spans'];failed=[s for s in spans if s.get('error')=='SourceEntryInterrupted']
 assert len(failed)==1;call=failed[0]
 assert call['depth']==0 and call['kind']=='invoke' and call['outcome']=='raised' and input_fact(call) is None
 assert call['message']==cut,'fault call label mismatch'
 assert call['stop']==call['start']+1==fault['native_position']==len(receipt['stream_sequence'])
 event=receipt['stream_sequence'][call['start']]
 assert event==dict(endpoint='source',index=fault['source_ordinal'],reply=True)
 root=Path(receipt['root']);ordinal=fault['source_ordinal']
 form,(_,command)=source_form(h.decode((root/'source'/f'{ordinal:03}-input.bin').read_bytes()))
 status,actual_snapshot=h.capacity_reply((root/'source'/f'{ordinal:03}-output.bin').read_bytes())
 assert form=='execute' and command==entry(fault['owner']) and status==0
 probes=[s for s in spans if s['host_start']>call['host_stop']]
 assert len(probes)==p+2
 assert Counter((s['kind'],s['endpoint']) for s in probes)==Counter([('invoke',None),('source',None)]+[('owner',i) for i in range(p)]),'retired probe export inventory'
 for probe in probes:
  input_fact(probe)
  assert probe['depth']==0 and probe['outcome']=='raised' and probe['start']==probe['stop']==call['stop']
  assert probe['error']=='RuntimeError' and probe['message']=='cohort retired'
 marker=[row for row in receipt['host_commit_trace'] if row['point']=='fault_retired']
 assert len(marker)==1;marker=marker[0]
 assert call['host_start']<marker['host_position']<call['host_stop'] and marker['cut']==fault['cut']
 assert marker['state']['retired'] and not marker['state']['gate']
 writer=marker['state']['owners'][fault['owner']]
 assert writer['lease']==fault['ticket'] and writer['entered']==fault['entered']
 rows=receipt['host_commit_trace']
 native=[x for x in rows if x['point']=='native_return' and x['endpoint']=='source' and x['ordinal']==ordinal]
 assert len(native)==1;native=native[0]
 assert native['native_position']==fault['native_position'] and native['owner']==fault['owner'] and native['ticket']==fault['ticket'],'fault entry occurrence binding'
 before=[x for x in rows if x['point']=='entry_claim_before' and x['source_entry']==native['source_entry']]
 assert len(before)==1;before=before[0]
 assert before['source_frame']==native['source_frame'] and before['ticket']==fault['ticket'] and before['owner']==fault['owner']
 interval=rows[native['sequence']+1:marker['sequence']]
 stores=[];previous=False
 for row in interval:
  if row['point']=='entry_snapshot_stored':
   assert row['source_entry']==native['source_entry'] and row['owner']==fault['owner']
   stores.append('snapshot')
  flag=row['state']['owners'][fault['owner']]['entered']
  if flag!=previous:
   assert row['function']=='entered' and flag is True,'fault notify occurrence'
   stores.append('notify');previous=flag
 expected={'UNNOTIFIED':[],'NOTIFIED':['notify'],'SNAPSHOT':['notify','snapshot']}[cut]
 assert stores==expected,'named fault local store sequence'
 if cut=='SNAPSHOT':
  encoded=json.dumps(actual_snapshot,sort_keys=True,separators=(',',':')).encode()
  assert marker['state']['source_snapshot']==hashlib.sha256(encoded).hexdigest(),'stored fault snapshot/native mismatch'
 else:assert marker['state']['source_snapshot']==before['state']['source_snapshot'],'unstored fault changed snapshot'
 before_order=receipt['host_order'][:call['host_start']]
 count=sum(e['kind']=='observation' for e in before_order)
 prefix=dict(receipt,stream_sequence=receipt['stream_sequence'][:call['start']],host_order=before_order,
   host_commit_trace=receipt['host_commit_trace'][:count],public_spans=[s for s in spans if s['host_stop']<call['host_start']])
 prior=literal_check(prefix)
 return dict(completed_literal_prefix=prior,fault_call=call['id'],retirement_sequence=marker['sequence'],
   source_ordinal=ordinal,known_accepted=True,stores=stores,cut=cut,later_zero_io_probes=len(probes),scope=__doc__)
