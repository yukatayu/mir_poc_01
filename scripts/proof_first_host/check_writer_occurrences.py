"""Finite occurrence/frame check for caller-confirmed writer samples.
Trusts privileged capture provenance; equality of IDs/values grants no authority.
Only the active writer interval is framed, not arbitrary outside host mutators.
"""
from check_host_commit_capture import observation_calls,check_host_order

def check(receipt):
 check_host_order(receipt);calls=observation_calls(receipt);active=None;candidate=None;count=framed=0
 for row in receipt['host_commit_trace']:
  if active is not None:
   owner=int(active['endpoint'][5:]);old=active['state']['owners'];now=row['state']['owners']
   assert len(old)==len(now),'writer cohort inventory'
   assert all(now[i]==old[i] for i in range(len(old)) if i!=owner),'nonlocal owner changed during writer interval'
   assert calls[row['sequence']]==calls[active['sequence']],'writer sample public call borrowed'
   assert row['native_position']==active['native_position'],'writer sample native occurrence changed'
   framed+=1
  if row['point']=='native_return':
   assert active is None,'native IO before writer caller confirmation'
   endpoint=row['endpoint'];event=receipt['stream_sequence'][row['native_position']-1]
   assert endpoint==event['endpoint'] and row['ordinal']==event['index'],'native event identity'
   assert row['process_id']==receipt[endpoint]['pid'],'native live process identity'
   if endpoint.startswith('owner'):
    assert row['caller_function']=='send','writer native return caller'
    active=row;candidate=None
  elif row['point']=='writer_return_candidate':
   assert active is not None and candidate is None,'orphan/repeated writer return candidate'
   assert row['caller_frame'] if 'caller_frame' in row else True
   assert row['writer_frame']==active['caller_frame'],'return candidate not actual originating writer frame'
   for field in ['endpoint','ordinal','process_id','reply_sha256']:assert row[field]==active[field],'writer candidate borrowed occurrence'
   candidate=row
  elif row['point']=='writer_return':
   assert active is not None and candidate is not None,'writer return lacks imminent candidate/caller continuation'
   for field in ['endpoint','ordinal','process_id','reply_sha256','writer_instance','writer_frame']:
    assert row[field]==candidate[field],'writer confirmed return borrowed occurrence'
   assert row['caller_function']=='owner_send' and row['caller_frame']!=row['writer_frame'],'writer confirmation must be in actual owning caller'
   active=None;candidate=None;count+=1
 assert active is None and candidate is None and count>0,'unfinished/nonvacuous writer interval'
 return dict(writer_occurrences=count,framed_samples=framed,scope=__doc__)
