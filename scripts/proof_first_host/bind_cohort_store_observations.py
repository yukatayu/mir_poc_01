"""Bind each actual selected cohort store statement to its first post-store row.
This finite normalizer trusts the privileged code-site/frame capture. It does not
invent native replies, discharge debt from final equality, or prove Python/OS.
The Lean product separately checks whether this store is actually owed.
"""
from copy import deepcopy

FIELDS=('bootstrapped','source_snapshot','initialized','freezes','installs','produced','pending')

def fields(state):return {key:state[key] for key in FIELDS}

def bind(receipt):
    rows=receipt['host_commit_trace'];before={};after={};seen=set();commits={}
    sites={(s['function'],s['line'],s['kind']) for s in receipt['cohort_store_sites']}
    assert len(sites)==9,'selected store-site inventory'
    for index,row in enumerate(rows):
        assert row['sequence']==index,'cohort row coverage'
        if row['point'] not in ('cohort_store_before','cohort_store_after'):continue
        token=row['cohort_store'];target=before if row['point']=='cohort_store_before' else after
        assert type(token) is int and token>=0 and token not in target,'reused cohort store token'
        target[token]=row
        assert (row['function'],row['store_line'],row['store_kind']) in sites,'unbound cohort store site'
    assert before and set(before)==set(after)==set(range(len(before))),'complete contiguous store boundaries'
    keys=('cohort_store','store_frame','store_code','store_cohort','store_kind','store_value','store_line')
    for token,first in before.items():
        last=after[token]
        assert {k:first[k] for k in keys}=={k:last[k] for k in keys},'cohort store frame/value changed'
        assert first['sequence']<last['sequence'],'cohort store order'
        assert first['function']==last['function'] and last['line']!=first['store_line'],'uncompleted statement marker'
        assert first['state']['gate'] is True and first['state']['retired'] is False,'cohort store outside live gate'
        expected=deepcopy(first['state']);kind=first['store_kind'];value=first['store_value']
        if kind=='bootstrapped':assert value is True;expected[kind]=True
        elif kind in ('source_snapshot','pending'):expected[kind]=deepcopy(value)
        elif kind in ('initialized','freezes','installs','produced'):
            if value not in expected[kind]:expected[kind].append(deepcopy(value));expected[kind].sort()
        else:raise AssertionError('unknown cohort store field')
        first_post=first['sequence']+1
        assert first_post not in commits,'multiple stores at one boundary'
        for index in range(first_post,last['sequence']+1):
            row=rows[index]
            assert row['native_position']==first['native_position'],'native progress inside cohort store'
            assert row['state']==expected,('cohort store changed extra fields or wrong value',token,index)
            assert row['point']!='cohort_store_before','overlapping cohort store statements'
        commits[first_post]=dict(token=token,kind=kind,value=value,before=first['sequence'],after=last['sequence'])
        seen.add(token)
    # Validate every transient full cohort projection, including rows that an
    # older native/writer-only event normalizer otherwise discarded.
    current=dict(bootstrapped=False,source_snapshot=None,initialized=[],freezes=[],installs=[],produced=[],pending=None)
    for index,row in enumerate(rows):
        if index in commits:current=fields(row['state'])
        assert fields(row['state'])==current,('unbound transient cohort mutation',index)
    assert len(seen)==len(commits)
    return commits
