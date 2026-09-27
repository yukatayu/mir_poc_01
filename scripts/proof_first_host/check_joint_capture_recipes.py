"""Finite private-call recipe checks supplement the Lean byte/owner replay.

Requires truthful captured public-call metadata. This is not a general Python
simulation or authentication of spans; it makes missing input/recipe/phase
checks explicit and rejects the raw-byte-preserving split found by Oracle34.
"""
from pathlib import Path
import copy, datetime, hashlib, json, sys

w=Path(__file__).resolve().parent
root=w/'recovered'
sys.path.insert(0,str(root))
import publication_process_check as h

def require(condition, message):
    if not condition: raise ValueError(message)

def sha(p): return hashlib.sha256(p.read_bytes()).hexdigest()

def notice(kind,index,revision):
    return h.right(h.nested_right(h.left(h.pair(h.nat(index),h.nat(revision))),2 if kind=='freeze' else 5))

def entry(index): return h.right(h.nested_right(h.left(h.nat(index)),6))
def finish(index): return h.right(h.nested_right(h.right(h.nat(index)),6))

def source_form(tree):
    outer,value=h.un_sum(tree)
    if outer==0:return 'head',value
    inner,value=h.un_sum(value)
    if inner==0:return 'cost',value
    vector,command=h.product_fields(value,2)
    return 'execute',(vector,command)

def input_fact(span):
    kind=span.get('payload_kind');raw=span.get('payload_hex')
    require(kind in ('none','bytes','bytearray'),'missing literal public input kind')
    if kind=='none':
        require(raw is None and span['payload_sha256'] is None,'none input mismatch')
        return None
    require(type(raw) is str,'missing literal payload bytes')
    payload=bytes.fromhex(raw)
    require(h.digest(payload)==span['payload_sha256'],'literal payload digest mismatch')
    require(type(span.get('expect_reply')) is bool,'missing literal reply mode')
    return payload

def return_fact(value):
    if value is None:return dict(kind='none')
    if type(value) is bytes:return dict(kind='bytes',sha256=h.digest(value),size=len(value))
    return dict(kind='tuple',items=[return_fact(x) for x in value])

def check_nested_rejections(receipt):
    """Every non-outer span must have its real selected rejection recipe.
    Call this over the complete capture, including a final failed outer call.
    Host order/parent/gate bookkeeping is checked separately; no native event
    or successful nested return may be erased into rejectedReentry.
    """
    spans=receipt['public_spans'];p=int(receipt['source']['numeric_arguments'][1]);checked=0
    for attempt in spans:
        if attempt['depth']==0:continue
        input_fact(attempt)
        require(attempt['depth']==1 and attempt['start']==attempt['stop'] and attempt['outcome']=='raised','nested public attempt is not a zero-IO rejection')
        kind=attempt['kind'];require(kind in ('source','owner','invoke'),'nested public export kind')
        if kind=='owner':require(type(attempt['endpoint']) is int and 0<=attempt['endpoint']<p,'nested owner inventory')
        else:require(attempt['endpoint'] is None,'nested source export endpoint')
        if kind=='invoke':require(attempt['payload_kind']=='none','nested invoke input')
        error=(attempt.get('error'),attempt.get('message'))
        require(error in [('ValueError','public cohort operation active'),('TypeError','immutable bytes payload required')],'nested rejection outcome')
        if error[0]=='TypeError':require(kind in ('owner','source') and attempt['payload_kind']=='bytearray','nested pre-gate type refusal')
        elif kind!='invoke':require(attempt['payload_kind']=='bytes','nested gate input')
        parent=spans[attempt['parent']]
        require(parent['depth']==0 and parent['kind']=='invoke' and parent['host_start']<attempt['host_start']<attempt['host_stop']<parent['host_stop'],'nested rejection parent lifetime')
        require(parent['start']<=attempt['start']<=parent['stop'],'nested rejection native position')
        checked+=1
    return checked

def check(receipt):
    sequence=receipt['stream_sequence'];capture=Path(receipt['root'])
    p=int(receipt['source']['numeric_arguments'][1]);scope=int(receipt['source']['numeric_arguments'][6])
    cache={}
    def read(position):
        if position in cache:return cache[position]
        row=sequence[position];base=capture/row['endpoint']/f"{row['index']:03}"
        payload=Path(str(base)+'-input.bin').read_bytes()
        response=Path(str(base)+'-output.bin').read_bytes() if row['reply'] else None
        value=dict(row,input=payload,command=h.decode(payload),response=response,reply=h.decode(response) if response else None)
        cache[position]=value
        return value
    ordered_calls=None
    if 'host_order' in receipt:
        from check_host_commit_capture import check_host_order
        ordered_calls=check_host_order(receipt)
    spans=receipt['public_spans'];outer=[r for r in spans if r['depth']==0]
    nested=[r for r in spans if r['depth']!=0]
    require(outer and outer[0]['start']==0 and outer[-1]['stop']==len(sequence),'outer coverage')
    require(all(x['stop']==y['start'] for x,y in zip(outer,outer[1:])),'outer continuity')
    snapshot=None;pending=None;source_ordinal=0;credits=[512]*p;initialized=set()
    slots=[None]*p;capacities=[int(receipt['owner'+str(i)]['numeric_arguments'][5]) for i in range(p)]
    def key(ticket):
        fields=h.product_fields(ticket,15);return (fields[4][1],fields[5][1])
    owner_images=[None]*p;owner_revisions=[0]*p;owner_fences=[0]*p
    observed_freezes=set();observed_installs=set();retained_productions=set()
    def provenance_failure(tree):
        # Guards occur BEFORE native send, including native refusal branches.
        tag,command=h.un_sum(tree)
        if tag!=1:return None
        depth=0;body=command
        while True:
            side,value=h.un_sum(body)
            if side==0:break
            depth+=1;body=value
            if depth==7:break
        if depth in (2,3,5):
            index,revision=h.product_fields(value,2)
            facts=observed_installs if depth==5 else observed_freezes
            if index[0]!='nat' or revision[0]!='nat' or (index[1],revision[1]) not in facts:
                return 'source notification lacks actual owner acknowledgement'
        if depth==1 and h.encode(value) not in retained_productions:
            return 'source arrival lacks actual owner production'
        return None
    def invocation_prelude():
        require(snapshot is not None and len(initialized)==p and snapshot['dispatch'] is None,'invoke phase')
        ticket=snapshot['source']['pending'];require(ticket is not None,'invoke without waiting work')
        target=h.product_fields(ticket,15)[3][1]
        installed,fence=snapshot['installed'],snapshot['fence']
        require(all(v[0:2]==('node',0) and len(v[2])==p for v in (installed,fence)),'invoke native publication vector')
        if installed[2][target]==fence[2][target]:
            require(owner_fences[target]==snapshot['published'],'invoke actual owner fence preflight')
            require(owner_revisions[target]==snapshot['published'] and owner_images[target]==snapshot['source']['image'],'invoke actual owner image preflight')
        def cost(tree):
            if tree[0]!='node':return 1
            fields=0
            for child in reversed(tree[2]):fields=1+max(cost(child),fields)
            return 1+fields
        require(len(h.encode(ticket))+372+12*p+25<=65536 and max(p+2,cost(ticket)+24)+7<=256,'invoke carrier preflight')
        return ticket,target
    completed=0;known_refusal=0;payments=0;notified=0;image_checks=0;attempts=0;query_refusals=0
    for span in outer:
        events=[read(i) for i in range(span['start'],span['stop'])]
        kind=span['kind'];raised=span['outcome']=='raised'
        payload=input_fact(span)
        require(kind in ('source','owner','invoke'),'unknown public call kind')
        if pending is not None:
            command,ordinal=pending
            if events:
                require(kind=='source' and not raised,'pending payment escaped source notification')
                require(source_ordinal==ordinal,'source moved after pending payment')
                require(span['payload_sha256']==h.digest(h.encode(command)),'wrong pending notification payload')
        if not events:
            require(raised,'successful public call omitted all native IO')
            error,message=span.get('error'),span.get('message')
            if error=='TypeError' and message=='immutable bytes payload required':
                require(kind in ('owner','source') and span['payload_kind']!='bytes','false input-type refusal')
            else:
                require(span['payload_kind']==('none' if kind=='invoke' else 'bytes'),'wrong input kind for post-type refusal')
                if error=='ValueError' and message=='bad tree':
                    require(kind=='source' and source_ordinal>0,'decoder failure context')
                    try:h.decode(payload)
                    except ValueError as problem:require(str(problem)=='bad tree','wrong first decoder failure')
                    else:raise ValueError('decodable source falsely refused')
                elif message=='pending owner payment requires matching source notification':
                    require(error=='ValueError' and pending is not None,'false pending-payment refusal')
                    if kind=='source':
                        # public_source decodes BEFORE testing pending payment.
                        require(h.decode(payload)!=pending[0],'matching notification falsely refused as mismatch')
                else:
                    require(pending is None,'unrelated reason while payment pending')
                    if error=='ValueError' and message=='owner command requires generated coordinator path':
                        require(kind=='owner' and snapshot is not None,'owner whitelist refusal context')
                        i=span['endpoint'];require(type(i) is int and 0<=i<p,'owner refusal assignment')
                        image=snapshot['source']['image']
                        allowed=(h.left(h.left(image)),h.right(h.nat(snapshot['announced'])),
                                 h.left(h.nested_right(h.left(h.pair(h.nat(snapshot['published']),image)),3)))
                        require(h.decode(payload) not in allowed,'allowed owner input falsely refused')
                    elif error=='ValueError' and message=='source work requires generated coordinator path':
                        require(kind=='source' and snapshot is not None and len(initialized)==p,'source whitelist refusal context')
                        require(h.decode(payload) in [f(i) for i in range(p) for f in (entry,finish)],'nonwork source input falsely refused')
                    elif error=='ValueError' and message in ('source arrival lacks actual owner production','source notification lacks actual owner acknowledgement'):
                        require(kind=='source' and snapshot is not None and len(initialized)==p,'provenance refusal context')
                        require(provenance_failure(h.decode(payload))==message,'false pre-send provenance refusal')
                    elif error=='ValueError' and message=='no idle pending source':
                        require(kind=='invoke' and len(initialized)==p and (snapshot is None or snapshot['dispatch'] is not None),'false nonidle invocation refusal')
                    elif error=='ResourceRefused' and message in (
                        'owner reservation key or slot unavailable before source enter',
                        'owner work and diagnostic credits unavailable before source enter'):
                        require(kind=='invoke' and len(initialized)==p and snapshot is not None and snapshot['dispatch'] is None,'resource refusal invocation context')
                        ticket,target=invocation_prelude()
                        available=slots[target] is not None and key(ticket) not in slots[target] and len(slots[target])<capacities[target]
                        if message.startswith('owner reservation'):
                            require(not available,'false slot/key refusal')
                        else:require(available and credits[target]<3,'false pre-entry credit refusal')
                    else:raise ValueError('unrecognized literal empty failure reason')
            attempts+=1
            continue
        require(span['payload_kind']==('none' if kind=='invoke' else 'bytes'),'nonbytes call emitted IO')
        if kind=='source':
            require(snapshot is None or len(initialized)==p,'source operation bypassed actual initialization prelude')
            require(len(events)==1 and events[0]['endpoint']=='source','source call recipe')
            event=events[0]
            if event['index']==1:
                require(snapshot is None and not raised and not event['reply'],'bootstrap recipe')
                require(span['expect_reply'] is False,'bootstrap reply mode')
                original=event['input']
            else:
                require(span['expect_reply'] is True,'source reply mode')
                form,value=source_form(event['command'])
                require(form=='execute','public source path used private query')
                vector,command=value;original=h.encode(command)
                require(command not in [f(i) for i in range(p) for f in (entry,finish)],'raw source work escaped public whitelist')
                require(provenance_failure(command) is None,'native source send bypassed pre-send provenance guard')
                require(not raised,'unexpected continuing source exception after IO')
                if pending is not None:
                    require(command==pending[0],'pending command mismatch')
                    require(h.capacity_reply(event['response'])[0]==0,'pending notification refused')
                    pending=None;notified+=1
            require(h.digest(original)==span['payload_sha256'],'public source input not bound to actual bytes')
        elif kind=='owner':
            require(snapshot is not None,'owner before source launch')
            require(span['expect_reply'] is True,'owner reply mode')
            index=span['endpoint'];require(type(index) is int and 0<=index<p,'public owner assignment')
            owner_events=[e for e in events if e['endpoint'].startswith('owner')]
            if not owner_events:
                require(raised and span.get('error')=='ResourceRefused','query-only owner call omitted a refusal')
                require(all(e['endpoint']=='source' for e in events),'query-only refusal endpoint')
                image=snapshot['source']['image'];current=snapshot['published'];announced=snapshot['announced']
                initialize=h.left(h.left(image));freeze=h.right(h.nat(announced))
                install=h.left(h.nested_right(h.left(h.pair(h.nat(current),image)),3))
                choices=[command for command in (initialize,freeze,install) if h.digest(h.encode(command))==span['payload_sha256']]
                require(len(choices)==1,'query-only public input not bound to current allowed image/command')
                command=choices[0];is_initialize=command==initialize
                require(len(events)==(1 if is_initialize else 2),'query-only refusal length')
                require(events[0]['command']==h.right(h.left(h.nat(index))),'query-only wrong cost target')
                demand=events[0]['reply'];require(demand[0]=='nat' and demand[1]<=1536,'query-only cost reply')
                payment=False;future=False
                if not is_initialize:
                    require(len(initialized)==p,'query-only owner bypassed initialization prelude')
                    is_freeze=command==freeze;revision=announced if is_freeze else current
                    head=h.left(h.pair(h.nat(index),h.nat(revision))) if is_freeze else h.right(h.left(h.pair(h.nat(index),h.nat(revision))))
                    require(events[1]['command']==h.left(head),'query-only wrong head target')
                    payment=h.un_bool(events[1]['reply']);future=is_freeze and announced>current and not payment
                if future:
                    require(span.get('message')=='future freeze awaits certified source head','future-head refusal outcome')
                else:
                    require(not payment or demand[1]>=1,'zero required-head demand')
                    require(credits[index]<demand[1]+(0 if payment else 1),'query-only refusal without exhausted slack')
                    require(span.get('message')=='administrative debit would consume retained source obligation','slack refusal outcome')
                # Queries change transport ordinal, not semantic source or owner budgets.
                source_ordinal=events[-1]['index'];query_refusals+=1
                continue
            require(len(owner_events)==1 and events[-1] is owner_events[0],'public owner operation must end with one actual owner exchange')
            owner=owner_events[0]
            require(owner['endpoint']=='owner'+str(index),'public owner wrong endpoint')
            require(h.digest(owner['input'])==span['payload_sha256'],'public owner input not bound to actual bytes')
            image=snapshot['source']['image'];current=snapshot['published'];announced=snapshot['announced']
            initialize=h.left(h.left(image));freeze=h.right(h.nat(announced))
            install=h.left(h.nested_right(h.left(h.pair(h.nat(current),image)),3))
            command=owner['command'];require(command in (initialize,freeze,install),'source image/public owner whitelist mismatch')
            image_checks+=int(command in (initialize,install))
            expected_queries=1 if command==initialize else 2
            require(len(events)==expected_queries+1 and all(e['endpoint']=='source' for e in events[:-1]),'owner private query recipe')
            require(events[0]['command']==h.right(h.left(h.nat(index))),'wrong cost query target')
            demand=events[0]['reply'];require(demand[0]=='nat' and demand[1]<=1536,'cost reply shape')
            payment=None
            if command!=initialize:
                require(len(initialized)==p,'missing initialization prelude')
                is_freeze=command==freeze;revision=announced if is_freeze else current
                expected=h.left(h.pair(h.nat(index),h.nat(revision))) if is_freeze else h.right(h.left(h.pair(h.nat(index),h.nat(revision))))
                require(events[1]['command']==h.left(expected),'wrong head query')
                at_head=h.un_bool(events[1]['reply'])
                require(not(is_freeze and announced>current and not at_head),'uncertified future freeze')
                if at_head:
                    require(demand[1]>=1,'unfunded head payment')
                    payment=notice('freeze' if is_freeze else 'install',index,revision)
                    require(owner['reply']==h.left(h.nat(12 if is_freeze else 7)),'unsuccessful required owner payment')
            require(credits[index]>=demand[1]+(0 if payment is not None else 1),'owner spent retained source obligation')
            require(not raised,'unexpected continuing owner exception after IO')
            if command==initialize and index not in initialized:
                require(owner['reply']==h.left(h.nat(10)),'first initialization did not succeed')
                initialized.add(index)
            if payment is not None:
                pending=(payment,events[-2]['index']);payments+=1
        else:
            require(span['payload_sha256'] is None and snapshot is not None,'invoke metadata')
            require(len(initialized)==p,'invoke bypassed actual initialization prelude')
            ticket,target=invocation_prelude()
            require(slots[target] is not None and key(ticket) not in slots[target] and len(slots[target])<capacities[target],'source enter preceded slot/fresh-key admission')
            require(credits[target]>=3,'source enter preceded three-credit admission')
            require(events[0]['endpoint']=='source','invoke must start at source')
            form,value=source_form(events[0]['command'])
            require(form=='execute' and value[1]==entry(target),'invoke entry not tied to waiting ticket')
            status,entered=h.capacity_reply(events[0]['response'])
            if status!=0:
                require(len(events)==1 and raised and span.get('error')=='ResourceRefused' and span.get('message')=='source refused generated work entry before owner IO','known entry refusal recipe')
                require(entered==snapshot,'entry refusal changed source projection')
                known_refusal+=1
            else:
                require(len(events)==6 and not raised,'invoke must retain one boundary through matching source finish')
                require([e['endpoint'] for e in events]==['source','source',*(['owner'+str(target)]*3),'source'],'invoke endpoint order')
                require(events[1]['command']==h.left(h.right(h.right(h.nat(target)))) and h.un_bool(events[1]['reply']),'missing actual finish-head query')
                endpoint,context_scope,revision,retained=h.product_fields(entered['dispatch'],4)
                require(endpoint==h.nat(target) and context_scope==h.nat(scope) and revision==h.nat(snapshot['published']) and retained==ticket,'entry dispatch correlation')
                require(events[2]['command']==h.left(h.right(h.left(ticket))) and events[2]['reply']==h.left(h.nat(6)),'invoke actual reservation')
                require(events[3]['command']==h.right(h.nat(snapshot['published']+1)) and events[3]['reply']==h.left(h.nat(2)),'invoke active probe')
                require(events[4]['command']==h.left(h.right(h.right(h.left(h.unit)))),'invoke compute command')
                produced,envelope=h.un_sum(events[4]['reply']);require(produced==1,'invoke without actual production')
                got_scope,ordinal,got_revision,got_ticket,result=h.product_fields(envelope,5)
                require(got_scope==context_scope and got_revision==revision and got_ticket==ticket,'compute is not the entered request')
                form,value=source_form(events[5]['command'])
                require(form=='execute' and value[1]==finish(target),'finish target not bound to actual production')
                code,finished=h.capacity_reply(events[5]['response'])
                require(code==0 and finished['held'] is None,'actual terminal notification not accepted')
                retained_productions.add(h.encode(envelope))
                completed+=1
        if not raised:
            expected_return=(events[-1]['response'],events[4]['response']) if kind=='invoke' else events[-1]['response']
            require(span.get('return_value')==return_fact(expected_return),'public return differs from actual native reply')
        for event in events:
            if event['endpoint']=='source':
                source_ordinal=event['index']
                if source_ordinal!=1 and source_form(event['command'])[0]=='execute':
                    _,snapshot=h.capacity_reply(event['response'])
            else:
                i=int(event['endpoint'][5:])
                if event['reply']==h.left(h.nat(10)):
                    require(slots[i] is None,'initialization erased existing keys');slots[i]=[]
                    outer_tag,body=h.un_sum(event['command']);inner,image=h.un_sum(body)
                    require((outer_tag,inner)==(0,0),'initialization request/reply mismatch')
                    owner_images[i]=image;owner_revisions[i]=0
                    observed_installs.add((i,snapshot['published']))
                elif event['reply']==h.left(h.nat(6)):
                    outer_tag,body=h.un_sum(event['command']);inner,body=h.un_sum(body);tag,ticket=h.un_sum(body)
                    require((outer_tag,inner,tag)==(0,1,0),'reserve response/command shape')
                    require(slots[i] is not None and key(ticket) not in slots[i] and len(slots[i])<capacities[i],'actual successful reserve violates slot monitor')
                    slots[i].insert(0,key(ticket))
                if event['reply']==h.left(h.nat(7)):
                    tag,body=h.un_sum(event['command']);require(tag==0,'installation request/reply mismatch')
                    for _ in range(3):
                        tag,body=h.un_sum(body);require(tag==1,'installation request/reply mismatch')
                    tag,body=h.un_sum(body);require(tag==0,'installation request/reply mismatch')
                    revision,image=h.product_fields(body,2);require(revision[0]=='nat','installation revision type')
                    owner_revisions[i]=revision[1];owner_images[i]=image
                    observed_installs.add((i,revision[1]))
                if event['reply']==h.left(h.nat(12)):
                    tag,revision=h.un_sum(event['command']);require(tag==1 and revision[0]=='nat','freeze request/reply mismatch')
                    owner_fences[i]=max(owner_fences[i],revision[1])
                    observed_freezes.add((i,revision[1]))
                if event['reply']!=h.left(h.nat(16)):
                    require(credits[i]>0,'negative actual credit monitor');credits[i]-=1
    require(pending is None,'final pending owner payment')
    for attempt in nested:
        input_fact(attempt)
        require(attempt['depth']==1 and attempt['start']==attempt['stop'] and attempt['outcome']=='raised','nonzero-IO nested public attempt')
        require((attempt.get('error'),attempt.get('message')) in [('ValueError','public cohort operation active'),('TypeError','immutable bytes payload required')],'unclassified pre-acquisition failure')
        if attempt.get('error')=='TypeError':
            require(attempt['kind'] in ('owner','source') and attempt['payload_kind']=='bytearray','false pre-gate type refusal')
        elif attempt['kind']!='invoke':require(attempt['payload_kind']=='bytes','nonbytes input claimed gate refusal')
        if ordered_calls is not None:
            parent=ordered_calls.get(attempt['parent'])
            require(parent is not None and parent['depth']==0 and parent['host_start']<attempt['host_start']<attempt['host_stop']<parent['host_stop'],'nested call outside recorded parent lifetime')
            require(parent['start']<=attempt['start']<=parent['stop'],'nested native position outside parent')
            enclosing=[parent]
        else:
            # Historical strict interior records do not distinguish equal native
            # positions at gate release. New endpoint cuts need explicit order.
            enclosing=[span for span in outer if span['start']<attempt['start']<span['stop']]
        require(len(enclosing)==1 and enclosing[0]['kind']=='invoke','reentry outside invoke recipe')
    return dict(outer=len(outer),completed_work=completed,known_entry_refusals=known_refusal,
                payments=payments,notifications=notified,image_checks=image_checks,zero_io_attempts=attempts,
                reentries=len(nested),credits=credits,query_only_refusals=query_refusals,reserved_keys=slots,capacities=capacities)

