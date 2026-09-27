"""Finite binder for actual source_send claim/reply/store intervals.
Selected privileged capture is TCB, not authentication. Coalesces only intervals
with no intervening native IO into a checked local-store subtrace. Snapshot
assignment marker is not an inner/outer return. No model-generated expected state.
"""
from pathlib import Path
import json,hashlib
from check_joint_capture_recipes import h,source_form,entry

def canonical(x):return json.dumps(x,sort_keys=True,separators=(',',':')).encode()
def bind(receipt,trees,observed,*,allow_retired_prefix=False,pending_wire=None):
 root=Path(receipt['root']);rows=receipt['host_commit_trace'];snapshot_pool=receipt['host_source_snapshots'];raw_by_snapshot={};p=int(receipt['source']['numeric_arguments'][1]);files=[]
 for event in receipt['stream_sequence']:
  if event['endpoint']!='source' or event['index']<2:continue
  ordinal=event['index'];raw=(root/'source'/f'{ordinal:03}-output.bin').read_bytes()
  if ordinal>2:
   kind,_=source_form(h.decode((root/'source'/f'{ordinal:03}-input.bin').read_bytes()))
   if kind!='execute':continue
  _,snapshot=h.capacity_reply(raw);encoded=h.encode(h.product_fields(h.decode(raw),2)[1]);key=hashlib.sha256(canonical(snapshot)).hexdigest()
  if key in raw_by_snapshot:assert raw_by_snapshot[key][0]==encoded
  raw_by_snapshot[key]=(encoded,snapshot)
 def snap(key):
  assert key is not None and key in snapshot_pool
  value=snapshot_pool[key];assert hashlib.sha256(canonical(value)).hexdigest()==key
  encoded,parsed=raw_by_snapshot[key];assert canonical(value)==canonical(parsed)
  name='source-snapshot-'+key;path=trees/(name+'.bin')
  if path.exists():assert path.read_bytes()==encoded
  else:path.write_bytes(encoded)
  files.append(dict(path=str(path),sha256=hashlib.sha256(encoded).hexdigest()))
  return json.dumps(name)
 active=None;used=set();intervals={};records=[];retired_seen=False
 fields=['source_entry','source_frame','source_code','cohort','owner','writer','ticket']
 def identity(row):return {k:row[k] for k in fields}
 for sequence,row in enumerate(rows):
  assert row['sequence']==sequence,'source entry observation sequence gap'
  point=row['point']
  if active is not None and row['state']['retired']:
   assert allow_retired_prefix,'retired source entry requires explicit prefix profile'
   assert point not in ('native_return','entry_claim_before','entry_claimed','entry_snapshot_stored'),'operation after source-entry retirement'
   first=active['before'];native=active['native'];i=first['owner']
   assert native is not None and active['claimed'] is not None,'retired prefix profile requires an actual known source reply'
   assert row['native_position']==native['native_position'] and row['state']['ordinal']==native['ordinal'],'native progress after entry-local fault'
   expected=list(active['claimed']['state']['owners']);expected[i]=active['last']
   assert row['state']['owners']==expected,'retirement changed retained writer memory'
   assert row['state']['source_snapshot']==first['state']['source_snapshot'],'unfinished entry snapshot changed at retirement'
   assert {k:v for k,v in row['state'].items() if k not in ('owners','ordinal','gate','retired')}=={k:v for k,v in first['state'].items() if k not in ('owners','ordinal','gate','retired')},'retirement changed cohort fields'
   retired_seen=True
   continue
  if point=='entry_claimed':
   assert active is not None and active['claimed'] is None and active['native'] is None,'orphan/duplicate claimed marker'
  if point=='entry_snapshot_stored':
   assert active is not None and active['native'] is not None,'orphan/premature snapshot marker'
  if point=='native_return' and 'source_entry' in row:
   assert active is not None,'orphan source entry native marker'
  if point=='entry_claim_before':
   assert active is None and row['source_entry'] not in used;used.add(row['source_entry'])
   i=row['owner'];assert 0<=i<p and row['state']['owners'][i]['lease'] is None and row['state']['owners'][i]['entered'] is False
   active=dict(binding=identity(row),before=row,claimed=None,native=None,stores=[],store_records=[],last=None)
  elif active is not None:
   first=active['before'];i=first['owner'];state=row['state'];baseline=first['state']
   assert state['gate'] is True and not state['retired']
   assert all(state['owners'][j]==baseline['owners'][j] for j in range(p) if j!=i)
   assert {k:v for k,v in state.items() if k not in ('owners','ordinal','source_snapshot')}=={k:v for k,v in baseline.items() if k not in ('owners','ordinal','source_snapshot')}
   if point=='entry_claimed':
    assert identity(row)==active['binding'] and active['claimed'] is None and active['native'] is None
    assert row['native_position']==first['native_position'] and state['ordinal']==baseline['ordinal']
    assert state['source_snapshot']==baseline['source_snapshot']
    assert state['owners'][i]['lease']==first['ticket'] and state['owners'][i]['entered'] is False
    active['claimed']=row;active['last']=state['owners'][i]
   elif point=='native_return':
    assert identity(row)==active['binding'] and active['claimed'] is not None and active['native'] is None
    assert row['caller_frame']==row['source_frame'] and row['caller_function']=='source_send','source entry caller/frame mismatch'
    assert row['endpoint']=='source' and row['ordinal']==baseline['ordinal']+1 and row['native_position']==first['native_position']+1
    assert state['ordinal']==row['ordinal'],'source entry contemporaneous ordinal mismatch'
    assert state['owners']==active['claimed']['state']['owners'] and state['source_snapshot']==baseline['source_snapshot']
    raw=(root/'source'/f'{row["ordinal"]:03}-input.bin').read_bytes();form,(vector,cmd)=source_form(h.decode(raw));assert form=='execute' and cmd==entry(i)
    active['native']=row
   elif active['native'] is not None:
    native=active['native'];assert row['native_position']==native['native_position'] and state['ordinal']==native['ordinal']
    if point=='entry_snapshot_stored':
     assert identity(row)==active['binding'] and row['source_ordinal']==native['ordinal']
     native_status,_=h.capacity_reply((root/'source'/f'{native["ordinal"]:03}-output.bin').read_bytes())
     assert type(row['status']) is int and row['status']==native_status,'source entry snapshot status/native mismatch'
     assert state['owners'][i]==active['last']
     active['stores'].append('⟨.storeSnapshot,'+observed(state['owners'][i])+','+snap(state['source_snapshot'])+'⟩')
     active['store_records'].append(dict(kind='snapshot',sequence=row['sequence']))
     assert len(active['stores'])==2
     shown='⟨'+observed(first['state']['owners'][i])+','+observed(active['claimed']['state']['owners'][i])+','+snap(baseline['source_snapshot'])+',['+','.join(active['stores'])+']⟩'
     assert native['ordinal'] not in intervals,'duplicate source entry ordinal'
     intervals[native['ordinal']]=shown;records.append(dict(binding=active['binding'],before_sequence=first['sequence'],claimed_sequence=active['claimed']['sequence'],native_sequence=native['sequence'],stored_sequence=row['sequence'],stores=active['store_records'],native_position=native['native_position'],ordinal=native['ordinal'],status=row['status']))
     active=None
    elif state['owners'][i]!=active['last']:
     assert row['function'] in ('entered','cancel') and not active['stores'] and state['source_snapshot']==baseline['source_snapshot']
     kind='notify' if row['function']=='entered' else 'cancel'
     active['store_records'].append(dict(kind=kind,sequence=row['sequence']))
     active['stores'].append('⟨.'+kind+','+observed(state['owners'][i])+','+snap(state['source_snapshot'])+'⟩');active['last']=state['owners'][i]
    else:assert state['source_snapshot']==baseline['source_snapshot']
   else:
    assert row['native_position']==first['native_position'] and state['ordinal']==baseline['ordinal']
    assert state['source_snapshot']==baseline['source_snapshot']
    if active['claimed'] is not None:
     assert state['owners']==active['claimed']['state']['owners'],'target writer changed before native reply'
    else:
     before_owner=baseline['owners'][i];actual_owner=state['owners'][i]
     assert {k:v for k,v in actual_owner.items() if k!='lease'}=={k:v for k,v in before_owner.items() if k!='lease'},'claim prefix changed non-lease fields'
     assert actual_owner['lease'] in (None,first['ticket']),'claim prefix borrowed lease'
 if active is not None:
  first=active['before'];native=active['native']
  snap(first['state']['source_snapshot'])
  if pending_wire is not None:
   assert native is None and active['claimed'] is not None and not active['stores'],'pending wire already known or unclaimed'
   arm=rows[-1];assert arm['point']=='wire_armed' and identity(arm)==active['binding'],'pending wire exact retained claim'
   assert arm['caller_frame']==arm['source_frame'] and arm['endpoint']=='source'
   assert pending_wire['armed_sequence']==arm['sequence'] and pending_wire['target_profile']=='ENTRY'
   assert arm['attempted_ordinal']==pending_wire['attempted_ordinal']==first['state']['ordinal']+1
   raw=(root/'wire-fault/input.bin').read_bytes();form,(_,cmd)=source_form(h.decode(raw))
   assert form=='execute' and cmd==entry(first['owner']) and raw.hex()==arm['payload_hex'],'pending entry actual input'
   records.append(dict(binding=active['binding'],before_sequence=first['sequence'],claimed_sequence=active['claimed']['sequence'],native_sequence=None,stored_sequence=None,stores=[],native_position=first['native_position'],ordinal=arm['attempted_ordinal'],status=None,pending_wire=True))
  else:
   assert allow_retired_prefix and retired_seen,'unfinished live source entry'
   assert native['ordinal'] not in intervals
   status,_=h.capacity_reply((root/'source'/f'{native["ordinal"]:03}-output.bin').read_bytes())
   records.append(dict(binding=active['binding'],before_sequence=first['sequence'],claimed_sequence=active['claimed']['sequence'],native_sequence=native['sequence'],stored_sequence=None,stores=active['store_records'],native_position=native['native_position'],ordinal=native['ordinal'],status=status,retired_prefix=True))
 assert records and (allow_retired_prefix or pending_wire is not None or intervals)
 check_framing(receipt,records)
 return intervals,records,list({x['path']:x for x in files}.values())

def check_framing(receipt,records):
 """Validate ALL selected writer observations before they can be erased.
 Only owner-journal stores and the already checked source claim/store prefixes
 may update current memory. Intermediate claim assignment is framed by the
 same actual claim interval; it is not a completed public operation. Native
 ordinal is checked independently at every selected observation. This finite
 normalizer is part of the capture TCB, not a theorem about arbitrary Python.
 """
 from copy import deepcopy
 p=int(receipt['source']['numeric_arguments'][1]);rows=receipt['host_commit_trace']
 current=[dict(credits=512,revision=0,image=None,keys=None,lease=None,entered=False,failed=False) for _ in range(p)]
 starts={r['before_sequence']:r for r in records};claims={r['claimed_sequence']:r for r in records}
 stores={s['sequence']:r for r in records for s in r['stores']}
 active_writer=None;claim=None;ordinal=0;checked=0
 for sequence,row in enumerate(rows):
  assert row['sequence']==sequence,'framing sequence'
  values=row['state']['owners'];assert len(values)==p,'framing inventory'
  assert type(row['state']['ordinal']) is int and row['state']['ordinal']>=0,'framing ordinal type'
  for value in values:
   assert set(value)=={'credits','revision','image','keys','lease','entered','failed'},'framing writer schema'
   assert type(value['credits']) is int and 0<=value['credits']<=512 and type(value['revision']) is int and value['revision']>=0,'framing writer numbers'
   assert type(value['entered']) is bool and type(value['failed']) is bool,'framing writer booleans'
   for name in ('image','lease'):
    assert value[name] is None or (type(value[name]) is str and value[name] in receipt['host_tree_bytes']),'framing writer tree reference'
   assert value['keys'] is None or (type(value['keys']) is list and all(type(pair) is list and len(pair)==2 and all(type(n) is int and n>=0 for n in pair) for pair in value['keys'])),'framing writer key schema'
  if row['point']=='native_return' and row['endpoint']=='source':
   assert row['ordinal']==ordinal+1,'framing source ordinal gap'
   ordinal=row['ordinal']
  assert row['state']['ordinal']==ordinal,('framing contemporaneous ordinal',sequence)
  if sequence in starts:
   assert claim is None and active_writer is None
   claim=starts[sequence]
  expected=deepcopy(current)
  if claim is not None and sequence<=claim['claimed_sequence']:
   i=claim['binding']['owner'];ticket=claim['binding']['ticket']
   assert values[i]['lease'] in (current[i]['lease'],ticket),'framing claim lease'
   expected[i]['lease']=values[i]['lease']
  if sequence in claims:
   assert claim is claims[sequence]
   i=claim['binding']['owner'];expected[i]=values[i];claim=None
  elif sequence in stores:
   i=stores[sequence]['binding']['owner'];expected[i]=values[i]
  elif active_writer is not None:
   expected[active_writer]=values[active_writer]
  assert values==expected,('unmodeled writer observation',sequence)
  current=deepcopy(values);checked+=1
  if row['point']=='native_return' and row['endpoint'].startswith('owner'):
   assert active_writer is None and claim is None
   active_writer=int(row['endpoint'][5:])
  elif row['point']=='writer_return':
   assert active_writer is not None
   active_writer=None
 assert active_writer is None and claim is None,'unfinished framing interval'
 return dict(observations=checked,source_ordinal=ordinal,scope=check_framing.__doc__)
