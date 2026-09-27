"""Finite exact-parent binding over authentic privileged observations.
The producer retains the actual writer and parent frames until confirmation.
Serialized IDs and counters alone are not authentication or authority.
"""
from check_writer_occurrences_total import check as check_intervals

def check(receipt):
    result=check_intervals(receipt)
    active=None;seen=set();confirmed=0
    for row in receipt['host_commit_trace']:
        if row['point']=='native_return' and row['endpoint'].startswith('owner'):
            assert active is None,'overlapping parent interval'
            token=row['writer_occurrence'];parent=row['expected_parent']
            assert token not in seen,'writer parent occurrence reused'
            assert parent['function']=='owner_send' and parent['frame']!=row['caller_frame'],'invalid parent origin'
            assert parent['index']==int(row['endpoint'][5:]),'wrong parent owner index'
            seen.add(token);active=row
        elif row['point'] in ('writer_return_candidate','writer_return'):
            assert active is not None,'parent observation lacks origin'
            assert row['writer_occurrence']==active['writer_occurrence'],'borrowed parent occurrence'
            assert row['expected_parent']==active['expected_parent'],'parent origin changed'
            assert row['writer_instance']==active['expected_parent']['writer'],'parent writer association'
            if row['point']=='writer_return':
                assert row['caller_frame']==active['expected_parent']['frame'],'wrong owning caller frame'
                assert row['confirming_parent']==active['expected_parent'],'wrong owning caller identity'
                active=None;confirmed+=1
    assert active is None and confirmed==result['writer_occurrences'],'parent interval incomplete'
    return dict(**result,parent_confirmations=confirmed,parent_scope=__doc__)
