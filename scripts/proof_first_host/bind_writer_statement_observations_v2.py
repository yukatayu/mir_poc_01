"""Finite physical store occurrence binder; selected normal postreply writer cut.
Truthful CPython instruction callbacks, exact code/frame custody and exclusive
pipes are premises. Matching post-values alone cannot create an occurrence.
No production observer, arbitrary writer mutation or interrupted store claim.
"""
from copy import deepcopy
FIELDS=('credits','image','revision','keys','lease','entered','failed')
FIELD={'releaseLease':'lease','clearEntered':'entered'}
CUSTODY=('writer_occurrence','owner','endpoint','ordinal','native_position','process_id','writer_frame','writer_code','writer_instance')

def bind(receipt):
 rows=receipt['host_commit_trace'];sites={x['offset']:x for x in receipt['writer_statement_sites']}
 assert len(sites)==9 and not receipt['writer_uncompleted_stores'],'normal exact writer site inventory'
 before={};after={};commits={}
 for i,row in enumerate(rows):
  assert row['sequence']==i,'actual row identity'
  if row['point'] not in ('writer_store_before','writer_store_after'):continue
  token=row['writer_store'];assert type(token) is int and token>=0
  target=before if row['point']=='writer_store_before' else after
  assert token not in target,'duplicate writer store boundary';target[token]=row
  site=sites[row['store_offset']]
  assert (row['store_kind'],row['store_next_offset'],row['store_line'])==(site['kind'],site['next_offset'],site['line']),'unbound writer instruction site'
 assert before and set(before)==set(after)==set(range(len(before))),'missing/repeated writer statement occurrence'
 keys=('writer_store','writer_occurrence','writer_frame','writer_code','writer_instance','owner','endpoint','ordinal','store_kind','store_offset','store_next_offset','store_line','native_position','process_id')
 for token,first in before.items():
  last=after[token];assert {k:first[k] for k in keys}=={k:last[k] for k in keys},'writer store occurrence/frame changed'
  assert first['sequence']<last['sequence'] and last['store_after_offset']==first['store_next_offset'],'store did not complete its next instruction'
  owner=first['owner'];assert first['endpoint']=='owner'+str(owner)
  expected=deepcopy(first['state']);field=FIELD.get(first['store_kind'],first['store_kind'])
  assert field in FIELDS and field!='failed'
  expected['owners'][owner][field]=deepcopy(last['store_value'])
  assert first['state']['gate'] and not first['state']['retired'] and not first['state']['owners'][owner]['failed'],'store outside live writer custody'
  for i in range(first['sequence']+1,last['sequence']+1):
   row=rows[i]
   assert row['native_position']==first['native_position'] and row['state']==expected,('writer store extra mutation/wrong post-value',token,i)
   assert row['point']!='writer_store_before','overlapping physical statements'
  # Retrospective cut: all intervening rows already equal the post-store value,
  # and the actual exact-successor completion marker has been checked above.
  index=first['sequence']+1;assert index not in commits
  commits[index]=dict(token=token,kind=first['store_kind'],value=last['store_value'],owner=owner,occurrence=first['writer_occurrence'],before=first['sequence'],after=last['sequence'],custody={k:first[k] for k in CUSTODY})
 active=None;current=None;intervals=[];consumed=set()
 for i,row in enumerate(rows):
  if row['point']=='native_return' and row['endpoint'].startswith('owner'):
   assert active is None,'overlapping native owner intervals'
   owner=int(row['endpoint'][5:]);active=dict(occurrence=row['writer_occurrence'],owner=owner,ordinal=row['ordinal'],start=i,commits=[],custody={k:row[k] for k in CUSTODY if k!='owner'}|{'owner':owner})
   current=deepcopy(row['state']['owners'][owner])
  if i in commits:
   commit=commits[i];assert active is not None and commit['owner']==active['owner'] and commit['occurrence']==active['occurrence'],'store borrowed from another writer/native occurrence'
   assert commit['custody']==active['custody'],'writer store native-interval provenance mismatch'
   current=deepcopy(row['state']['owners'][active['owner']]);active['commits'].append(commit);consumed.add(commit['token'])
  if active is not None:
   assert row['state']['owners'][active['owner']]==current,('unbound writer transient mutation',i)
  if row['point']=='writer_return':
   assert active is not None and row['writer_occurrence']==active['occurrence'] and row['ordinal']==active['ordinal'],'writer completion wrong occurrence'
   assert {k:row[k] for k in CUSTODY if k!='owner'}|{'owner':active['owner']}==active['custody'],'writer return native-interval provenance mismatch'
   active['stop']=i;intervals.append(active);active=None;current=None
 assert active is None and consumed==set(before),'unfinished/unowned writer statement inventory'
 return commits,intervals
