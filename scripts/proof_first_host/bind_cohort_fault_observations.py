"""Bind actual completed stores and explicitly interrupted attempted statements.
Rows are never omitted. The aborted before/abort pair contributes observations,
but cannot discharge a store debt. Privileged capture truthfulness is a premise.
"""
from copy import deepcopy
from bind_cohort_store_observations import bind as completed, fields
from check_source_entry_fault_capture import check as check_fault

KEYS=('cohort_store','store_frame','store_code','store_cohort','store_kind','store_value','store_line')

def bind(receipt):
    fault=check_fault(receipt)
    pending=receipt['source_entry_fault']['uncompleted_cohort_stores']
    rows=receipt['host_commit_trace']
    aborted=[row for row in rows if row['point']=='cohort_store_aborted']
    assert len(pending)==len(aborted)==(1 if fault['cut']=='NOTIFIED' else 0),'interrupted store inventory'
    normalized=deepcopy(receipt)
    for binding,stop in zip(pending,aborted):
        assert {key:stop[key] for key in KEYS}==binding,'interrupted store binding'
        assert stop['cut']==fault['cut'] and stop['function']=='source_send','interrupted source site'
        assert stop['store_kind']=='source_snapshot' and stop['line']==stop['store_line'],'interrupted selected statement'
        token=stop['cohort_store']
        before=[row for row in rows if row['point']=='cohort_store_before' and row['cohort_store']==token]
        assert len(before)==1,'interrupted statement has one before marker'
        first=before[0]
        assert {key:first[key] for key in KEYS}==binding,'interrupted before binding'
        assert stop['sequence']==first['sequence']+1,'interrupted statement boundary'
        assert first['state']==stop['state'] and first['native_position']==stop['native_position'],'interrupted statement changed state'
        assert stop['state']['gate'] is True and stop['state']['retired'] is False,'interrupt after gate cleanup'
        assert not any(row['point']=='cohort_store_after' and row['cohort_store']==token for row in rows),'interrupted statement falsely completed'
        assert token==sum(row['point']=='cohort_store_after' for row in rows),'interrupted statement not last store'
        assert stop['sequence']<fault['retirement_sequence'],'interrupt after retirement'
        for row in rows[stop['sequence']:]:
            assert fields(row['state'])==fields(stop['state']),'post-interruption cohort mutation'
            assert row['native_position']==stop['native_position'],'post-interruption native execution'
        # Only classify this already bound attempted statement; all original
        # sequence numbers, row values and actual native associations remain.
        normalized['host_commit_trace'][first['sequence']]['point']='cohort_store_unexecuted'
    commits=completed(normalized)
    assert len(rows)==len(normalized['host_commit_trace'])
    return commits
