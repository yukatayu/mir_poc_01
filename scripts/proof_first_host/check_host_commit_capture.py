"""Check actual local observations tied to native reply positions.
This is a finite discriminator, not a proof of Python/OS or secret noninterference.
"""
import json,sys
from pathlib import Path

def check_native_observation_bytes(receipt):
    """Known-row identities only; applicable to complete and fault prefixes.
    The unresolved attempt is deliberately absent from this settled sequence.
    Equality is capture consistency, not authentication of a serialized PID.
    """
    import hashlib
    rows=[row for row in receipt['host_commit_trace'] if row['point']=='native_return']
    sequence=receipt['stream_sequence'];root=Path(receipt['root'])
    assert len(rows)==len(sequence),'native observation inventory'
    for position,(row,event) in enumerate(zip(rows,sequence),1):
        assert row['native_position']==position and row['endpoint']==event['endpoint'] and row['ordinal']==event['index'],'native occurrence identity'
        assert row['process_id']==receipt[event['endpoint']]['pid'],'native process identity'
        expected=None
        if event['reply']:
            expected=hashlib.sha256((root/event['endpoint']/f"{event['index']:03}-output.bin").read_bytes()).hexdigest()
        assert row['reply_sha256']==expected,('native observation/reply digest',row['sequence'])
    return dict(known_native_observations=len(rows))

def check_host_order(receipt):
    """A truthful total host order is required when native positions coincide."""
    order=receipt['host_order'];observations=receipt['host_commit_trace'];spans=receipt['public_spans']
    assert [r['id'] for r in spans]==list(range(len(spans))),'call identity inventory'
    stack=[];opened=set();closed=set();seen_observations=[];native_position=0;gate=False
    for position,event in enumerate(order):
        assert event['position']==position,'host total order'
        if event['kind']=='call_begin':
            call=event['call'];span=spans[call]
            assert call not in opened and span['host_start']==position,'duplicate or displaced call entry'
            parent=stack[-1] if stack else None
            assert span['parent']==event['parent']==parent and span['depth']==len(stack),'actual parent call identity'
            assert span['start']==native_position,'call/native start order'
            if stack:assert gate,'nested call lacks retained public gate'
            opened.add(call);stack.append(call)
        elif event['kind']=='call_end':
            call=event['call'];span=spans[call]
            assert stack and stack.pop()==call and call not in closed,'call return stack'
            assert span['host_stop']==position and span['outcome']==event['outcome'],'call outcome order'
            assert span['stop']==native_position,'call/native return order'
            closed.add(call)
        elif event['kind']=='observation':
            index=event['observation'];row=observations[index]
            assert index==len(seen_observations) and row['host_position']==position,'observation total order'
            seen_observations.append(index)
            if row['point'] in ('native_return','writer_return','invoke_return') or row['function'] not in ('__init__','retire'):
                assert stack,'operation observation outside its public call'
            if row['point']=='invoke_return':
                span=spans[stack[-1]]
                assert span['kind']=='invoke' and span['depth']==0,'invoke return call identity'
                value=span.get('return_value',{})
                assert value.get('kind')=='tuple' and [x.get('sha256') for x in value['items']]==row['return_items'],'invoke observation/public return identity'
            if row['point']=='native_return':native_position+=1
            assert row['native_position']==native_position,'host/native position coupling'
            gate=row['state']['gate']
        else:raise AssertionError('unknown host order event')
    assert not stack and opened==closed==set(range(len(spans))),'incomplete public call order'
    assert len(seen_observations)==len(observations) and native_position==len(receipt['stream_sequence']),'incomplete host/native order'
    return {span['id']:span for span in spans}

def observation_calls(receipt):
    """Derive occurrence ownership from the already-validated total order."""
    if 'host_order' not in receipt:return None
    stack=[];calls={}
    for event in receipt['host_order']:
        if event['kind']=='call_begin':stack.append(event['call'])
        elif event['kind']=='call_end':stack.pop()
        else:calls[event['observation']]=stack[0] if stack else None
    return calls

def check_gate_lifetime(receipt):
    """One observed acquisition/release interval per OUTER public call.

    The public span begins before gate acquisition and ends after release.
    This finite capture check does not turn call-stack membership into a gate.
    Retirement forbids later native IO; cleanup observations may still follow.
    """
    calls=observation_calls(receipt)
    phases={span['id']:'before' for span in receipt['public_spans'] if span['depth']==0}
    retired=False
    for row in receipt['host_commit_trace']:
        state=row['state'];call=calls[row['sequence']]
        assert isinstance(state['gate'],bool) and isinstance(state['retired'],bool),'gate/retired observation type'
        if row['point']=='native_return':
            assert not retired and not state['retired'],'native IO after cohort retirement'
        assert not retired or state['retired'],'cohort retirement reversed'
        retired=state['retired']
        if call is None:
            assert not state['gate'],'public gate held outside public occurrence'
            continue
        phase=phases[call]
        if state['gate']:
            assert phase!='released','public gate reacquired inside same call'
            phase='held'
        elif phase=='held':phase='released'
        if row['point'] in ('native_return','writer_return'):
            assert phase=='held' and state['gate'],'native/writer boundary without retained gate'
        phases[call]=phase
    assert all(phase!='held' for phase in phases.values()),'public call ended with held gate'

def check(receipt, *, require_discharged=True):
    assert 'host_order' in receipt,'missing occurrence host order'
    check_host_order(receipt)
    check_native_observation_bytes(receipt)
    calls=observation_calls(receipt)
    rows=receipt.get('host_commit_trace',[])
    assert rows,'missing actual host commit observations'
    assert all(r['sequence']==i for i,r in enumerate(rows)),'local observation order'
    replies=[r for r in rows if r['point']=='native_return' and r.get('reply_sha256')]
    assert len(replies)==len(receipt['stream_sequence'])-1,'native reply coverage'
    # Independently derive occurrence labels from the SAME actual raw bytes.
    sys.path.insert(0,str(Path(receipt['root']).parent))
    import publication_process_check as h
    from check_joint_capture_recipes import source_form,notice,finish
    native=[]
    for event in receipt['stream_sequence']:
        base=Path(receipt['root'])/event['endpoint']/f"{event['index']:03}"
        payload=Path(str(base)+'-input.bin').read_bytes()
        response=Path(str(base)+'-output.bin').read_bytes() if event['reply'] else None
        native.append((event,payload,response))
    native_rows=[r for r in rows if r['point']=='native_return']
    assert len(native_rows)==len(native),'native return coverage'
    production={}
    for position,(row,(event,payload,response)) in enumerate(zip(native_rows,native),1):
        assert (row['endpoint'],row['ordinal'],row['native_position'])==(event['endpoint'],event['index'],position),'native occurrence identity'
        assert row['reply_sha256']==(None if response is None else h.digest(response)),'native reply byte identity'
        payment=None;terminal=None
        if event['endpoint'].startswith('owner') and response is not None:
            index=int(event['endpoint'][5:]);tree=h.decode(payload);reply=h.decode(response)
            if reply in [h.left(h.nat(12)),h.left(h.nat(7))]:
                kind='freeze' if reply==h.left(h.nat(12)) else 'install'
                if kind=='freeze':revision=tree[2][0][1]
                else:
                    value=tree
                    for _ in range(5):value=h.un_sum(value)[1]
                    revision=h.product_fields(value,2)[0][1]
                head=h.left(h.left(h.pair(h.nat(index),h.nat(revision)))) if kind=='freeze' else h.left(h.right(h.left(h.pair(h.nat(index),h.nat(revision)))))
                previous,request,answer=native[position-2]
                if previous['endpoint']=='source' and h.decode(request)==head and h.decode(answer)==h.right(h.unit):
                    payment=dict(command=h.digest(h.encode(notice(kind,index,revision))),ordinal=previous['index'])
        if event['endpoint']=='source' and response is not None:
            form,value=source_form(h.decode(payload))
            if form=='execute' and value[1] in [finish(i) for i in range(int(receipt['source']['numeric_arguments'][1]))]:
                previous,request,answer=native[position-2]
                assert previous['endpoint'].startswith('owner') and h.decode(request)==h.left(h.right(h.right(h.left(h.unit)))),'finish lacks adjacent actual compute'
                terminal=[h.digest(response),h.digest(answer)]
                kind,envelope=h.un_sum(h.decode(answer))
                assert kind==1,'finish lacks actual production reply'
                production[position]=h.digest(h.encode(envelope))
        assert row.get('payment')==payment,'actual payment occurrence omitted or invented'
        assert row.get('work_finish')==terminal,'actual finish occurrence omitted or invented'
    # A small occurrence journal checks exact baseline and frame conditions.
    # Native reply capture, local writer commit and Python pre-return are distinct.
    # Both profiles require closed public spans and discharged local commits.
    # The completed profile additionally discharges the retained notification.
    # This is NOT an admission rule for unknown IO or native-reply fault prefixes.
    p=int(receipt['source']['numeric_arguments'][1])
    credits=[512]*p;source_ordinal=0;writer_due=None;retention_due=None;produced=[]
    payment_due=None;pending_value=None;clear_due=None
    for row in rows:
        if row['point']=='native_return':
            assert writer_due is None,'new native IO before writer return'
            assert retention_due is None,'new native IO before production retention'
            assert payment_due is None and clear_due is None,'new native IO before pending local commit'
            event,payload,response=native[row['native_position']-1]
            if pending_value is not None:
                assert event['endpoint']=='source' and event['index']==pending_value['ordinal']+1,'pending notification occurrence'
                form,value=source_form(h.decode(payload))
                assert form=='execute' and h.digest(h.encode(value[1]))==pending_value['command'],'pending notification command'
                assert h.capacity_reply(response)[0]==0,'pending notification lacks accepted native reply'
                clear_due=dict(position=row['native_position'],call=calls[row['sequence']])
            if event['endpoint']=='source':
                assert event['index']==source_ordinal+1,'source occurrence gap'
                source_ordinal=event['index']
            else:
                owner=int(event['endpoint'][5:]);expected=list(credits)
                if h.decode(response)!=h.left(h.nat(16)):expected[owner]-=1
                writer_due=dict(expected=expected,reply=h.digest(response),
                    position=row['native_position'],call=None if calls is None else calls[row['sequence']])
            if row['native_position'] in production:
                retention_due=dict(expected=sorted(set(produced)|{production[row['native_position']]}),
                    position=row['native_position'],call=None if calls is None else calls[row['sequence']])
            if row.get('payment'):
                assert pending_value is None,'payment overwrites retained occurrence'
                payment_due=dict(expected=row['payment'],position=row['native_position'],call=calls[row['sequence']])
        actual=[owner['credits'] for owner in row['state']['owners']]
        if actual!=credits:
            assert writer_due is not None and row['point']!='native_return' and actual==writer_due['expected'],'host credit baseline or exact owner frame'
            credits=actual
        assert actual==credits,'host credit baseline'
        assert row['state']['ordinal']==source_ordinal,'host source ordinal identity'
        if row['point']=='writer_return':
            assert writer_due is not None,'writer return lacks native occurrence'
            assert row['native_position']==writer_due['position'] and credits==writer_due['expected'],'writer return before exact debit commit'
            assert row['reply_sha256']==writer_due['reply'],'writer/native reply identity'
            if calls is not None:assert calls[row['sequence']]==writer_due['call'],'writer/native public call identity'
            writer_due=None
        if row['state']['produced']!=produced:
            assert retention_due is not None and row['state']['produced']==retention_due['expected'],'exact production retention/frame'
            assert row['point']!='native_return' and row['native_position']==retention_due['position'],'production local commit occurrence'
            if calls is not None:assert calls[row['sequence']]==retention_due['call'],'production/public call identity'
            produced=row['state']['produced'];retention_due=None
        observed_pending=row['state']['pending']
        if observed_pending!=pending_value:
            if payment_due is not None:
                assert row['state']['gate'],'payment commit without retained gate'
                assert observed_pending==payment_due['expected'] and writer_due is None and row['point']!='native_return','pending commit precedes writer return or differs'
                assert row['native_position']==payment_due['position'] and calls[row['sequence']]==payment_due['call'],'pending commit occurrence identity'
                pending_value=observed_pending;payment_due=None
            elif clear_due is not None:
                assert row['state']['gate'],'pending clear commit without retained gate'
                assert observed_pending is None and row['point']!='native_return','pending notification clear differs'
                assert row['native_position']==clear_due['position'] and calls[row['sequence']]==clear_due['call'],'pending clear occurrence identity'
                pending_value=None;clear_due=None
            else:raise AssertionError('pending occurrence changed before matching notification')
        if payment_due is not None:
            assert row['state']['gate'],'gate released with payment commit debt'
        if clear_due is not None:
            assert row['state']['gate'],'gate released with pending clear debt'
    assert writer_due is None and retention_due is None and payment_due is None and clear_due is None,'unfinished local commit journal'
    if require_discharged:
        assert pending_value is None,'completed host capture retains pending notification'
    pays=works=0
    for i,r in enumerate(rows):
        if r['point']!='native_return' or not r.get('payment'):continue
        # Actual native success has occurred; the private monitor is still old.
        target=r['endpoint'];assert target.startswith('owner')
        owner=int(target[5:]);before=r['state']['owners'][owner]['credits']
        assert r['state']['gate'] and r['state']['pending'] is None,'payment reply gap'
        later=rows[i+1:]
        writer=next(x for x in later if x['point']=='writer_return')
        assert writer['native_position']==r['native_position'],'intervening native IO before writer commit'
        assert writer['reply_sha256']==r['reply_sha256'],'payment writer/native reply identity'
        assert writer['state']['owners'][owner]['credits']==before-1,'writer debit commit'
        assert writer['state']['pending'] is None and writer['state']['gate'],'writer/cohort commit gap'
        pending=next(x for x in later if x['state']['pending'] is not None)
        assert pending['native_position']==r['native_position'],'payment ordinal moved'
        assert pending['state']['pending']==r['payment'],'wrong pending occurrence'
        assert pending['state']['gate'],'gate released before payment retained'
        assert writer['sequence']<pending['sequence'],'pending commit precedes writer return'
        if calls is not None:
            assert calls[r['sequence']]==calls[writer['sequence']]==calls[pending['sequence']],'payment commit/public call identity'
        pays+=1
    for i,r in enumerate(rows):
        if r['point']!='native_return' or not r.get('work_finish'):continue
        assert r['state']['gate'],'finish reply after gate release'
        produced=r['state']['produced']
        later=rows[i+1:]
        retained=next(x for x in later if x['state']['produced']!=produced)
        assert retained['native_position']==r['native_position'],'IO before production retained'
        assert retained['state']['produced']==sorted(set(produced)|{production[r['native_position']]}),'exact production retention'
        assert retained['state']['gate'],'gate released before production retained'
        released=next(x for x in later if not x['state']['gate'])
        returned=next(x for x in later if x['point']=='invoke_return')
        assert retained['sequence']<released['sequence']<=returned['sequence'],'retain/release/return order'
        assert returned['native_position']==r['native_position'],'IO before public return'
        assert returned['return_items']==r['work_finish'],'actual result tuple differs'
        assert returned['state']['produced']==retained['state']['produced'],'production retained through return'
        if calls is not None:
            assert calls[r['sequence']]==calls[retained['sequence']]==calls[released['sequence']]==calls[returned['sequence']],'work commit/public call identity'
        works+=1
    assert pays and works,'vacuous payment/work capture'
    check_gate_lifetime(receipt)
    return dict(payments=pays,works=works,native_replies=len(replies),observations=len(rows),
        profile='completed' if require_discharged else 'closed_boundary_prefix',pending_notification=pending_value)

if __name__=='__main__':
    receipt=json.loads(Path(sys.argv[1]).read_text())
    print(json.dumps(check(receipt)))
