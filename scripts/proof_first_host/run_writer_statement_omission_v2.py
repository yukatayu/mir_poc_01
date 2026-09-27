"""Writer stores additionally captured at exact preinstruction and successful-next-instruction cuts.
This is privileged test evidence, not passive production monitoring or a Python/OS proof.
Privileged test tracing of actual selected host lines/returns and real pipes.
Complete cohort store statement boundaries are additionally captured, including idempotent stores. Original sources are retained; private caller-confirmed copies are selected. Return candidates are confirmed by the actual owning caller continuation.
Trace sites observe completed Python statements,
not every instruction; same-line intermediate stores remain inside trusted locks.
This instrumentation is not a passive production observer or an authorization API.
"""
from pathlib import Path
from contextlib import contextmanager
import datetime,hashlib,json,sys,inspect,re,dis
w=Path(__file__).resolve().parent;root=w/'recovered'
sys.path.insert(0,str(root))
import publication_process_check as h
sys.path.insert(0,str(w/'writer-occurrence-red-caller'))
import source_owner_cohort_resource as c
import owner_resource_writer as o
from private_wire_slot import RetainingPeer
h.Peer=RetainingPeer
from cohort_observer_native import native as observed_native
h.native=observed_native
from check_host_commit_capture import check

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
selected={name:sha(root/name) for name in ['publication_process_check.py','source_owner_cohort_resource.py','owner_resource_writer.py']}

def return_items(value):
    return [h.digest(x) for x in value] if type(value) is tuple and all(type(x) is bytes for x in value) else None

@contextmanager
def factory(source_build,source_args,owner_build,owner_args,capture,receipt):
    h.require(sys.gettrace() is None,'existing trace hook must be preserved')
    rows=receipt['host_commit_trace']=[]
    confirmed_native=set();writer_candidates={};writer_bindings={};next_writer_occurrence=0
    spans=receipt['public_spans']=[]
    cohort=None;last=None;tree_cache={};call_stack=[];views={};done=set();snapshot_cache={}
    snapshot_pool=receipt['host_source_snapshots']={}
    receipt['host_tree_bytes']={}
    order=receipt['host_order']=[]
    controls=receipt['host_edge_controls']=[]
    def tick(kind,**details):
        position=len(order);order.append(dict(position=position,kind=kind,**details));return position
    def tree_sha(tree):
        if tree is None:return None
        key=id(tree)
        if key not in tree_cache:
            encoded=h.encode(tree);digest=h.digest(encoded)
            tree_cache[key]=(tree,digest)
            prior=receipt['host_tree_bytes'].setdefault(digest,encoded.hex())
            h.require(prior==encoded.hex(),'host tree byte collision')
        h.require(tree_cache[key][0] is tree,'retained tree identity')
        return tree_cache[key][1]
    immutable_snapshot_parts={}
    def content_key(value):
        # Mutable dict/list nodes are visited every time. Only recursively
        # immutable built-in tuples/scalars may reuse identity-cached keys.
        # Type tags distinguish bool/int and list/tuple, unlike plain ==.
        kind=type(value)
        if value is None or kind in (bool,int,str):return (kind.__name__,value),True
        if kind is tuple:
            prior=immutable_snapshot_parts.get(id(value))
            if prior is not None:
                h.require(prior[0] is value,'retained immutable snapshot part')
                return prior[1],True
            parts=[content_key(child) for child in value]
            key=('tuple',tuple(part[0] for part in parts));fixed=all(part[1] for part in parts)
            if fixed:immutable_snapshot_parts[id(value)]=(value,key)
            return key,fixed
        if kind is list:return ('list',tuple(content_key(child)[0] for child in value)),False
        if kind is dict:
            h.require(all(type(k) is str for k in value),'snapshot string field names')
            return ('dict',tuple((k,content_key(value[k])[0]) for k in sorted(value))),False
        raise TypeError('unsupported snapshot value type')
    def snapshot_id(value):
        if value is None:return None
        key=id(value);content,_=content_key(value)
        if key not in snapshot_cache:
            body=json.dumps(value,sort_keys=True,separators=(',',':')).encode()
            digest=h.digest(body);snapshot_cache[key]=(value,digest,content)
            saved=json.loads(body)
            h.require(digest not in snapshot_pool or snapshot_pool[digest]==saved,'source snapshot digest collision')
            snapshot_pool[digest]=saved
        h.require(snapshot_cache[key][0] is value,'retained snapshot identity')
        h.require(snapshot_cache[key][2]==content,'source snapshot mutated in place')
        return snapshot_cache[key][1]
    def snapshot():
        if cohort is None:return None
        try:
            owners=[]
            for writer in cohort._Cohort__owners:
                owners.append(dict(credits=writer._CreditWriter__credits,entered=writer._CreditWriter__entered,
                    lease=tree_sha(writer._CreditWriter__lease),failed=writer._CreditWriter__failed,
                    image=tree_sha(writer._CreditWriter__image),revision=writer._CreditWriter__revision,
                    keys=None if writer._CreditWriter__reserved_keys is None else list(writer._CreditWriter__reserved_keys)))
            pending=cohort._Cohort__pending_payment
            return dict(owners=owners,ordinal=cohort._Cohort__source.ordinal,
                gate=cohort._Cohort__entry_gate.locked(),retired=cohort._Cohort__retired,
                initialized=sorted(cohort._Cohort__initialized),
                freezes=sorted(cohort._Cohort__freezes),installs=sorted(cohort._Cohort__installs),
                bootstrapped=cohort._Cohort__bootstrapped,source_snapshot=snapshot_id(cohort._Cohort__snapshot),
                pending=None if pending is None else dict(command=h.digest(h.encode(pending[0])),ordinal=pending[1]),
                produced=sorted(h.digest(value) for value in cohort._Cohort__produced))
        except AttributeError:return None
    def append(point,frame,extra=None,force=False):
        nonlocal last
        state=snapshot()
        if state is None:return
        if force or state!=last:
            row=dict(sequence=len(rows),point=point,function=frame.f_code.co_name,line=frame.f_lineno,
                native_position=len(receipt.get('stream_sequence',[])),state=state)
            row.update(extra or {});row['host_position']=tick('observation',observation=len(rows));rows.append(row);last=state
    def ancestor(frame,name):
        while frame is not None:
            if frame.f_code.co_name==name and frame.f_code.co_filename==c.__file__:return frame
            frame=frame.f_back
        return None
    source_entries={};next_source_entry=0
    source_lines,source_first=inspect.getsourcelines(c._Cohort.source_send)
    claim_site=source_first+next(i for i,line in enumerate(source_lines) if 'if not owner.claim(ticket)' in line)
    confirmed_claim_site=source_first+next(i for i,line in enumerate(source_lines) if 'if self.__funded:' in line)
    stored_site=source_first+next(i for i,line in enumerate(source_lines) if 'return reply' in line)
    confirmation_sites={}
    for method in (o.CreditWriter.send,c._Cohort._Cohort__query,c._Cohort.source_send):
        lines,first=inspect.getsourcelines(method)
        for offset,line in enumerate(lines):
            match=re.search(r'\._confirm_return\((reply|result)\)',line)
            if match:confirmation_sites[(method.__code__,first+offset)]=match.group(1)
    h.require(len(confirmation_sites)==4,'complete selected source/writer confirmation sites')
    cohort_sites={};cohort_pending={};next_cohort_store=0
    def register_cohort_store(method,needle,kind,value,guard=lambda fields:True):
        lines,first=inspect.getsourcelines(method)
        found=[first+i for i,line in enumerate(lines) if needle in line]
        h.require(len(found)==1,'unique selected cohort statement site')
        key=(method.__code__,found[0]);h.require(key not in cohort_sites,'duplicate cohort site')
        cohort_sites[key]=(kind,value,guard)
    register_cohort_store(c._Cohort.source_send,'self.__bootstrapped=True','bootstrapped',lambda f:True)
    register_cohort_store(c._Cohort.source_send,'self.__snapshot=snapshot','source_snapshot',lambda f:snapshot_id(f['snapshot']))
    register_cohort_store(c._Cohort.public_owner,'self.__initialized.add(index)','initialized',lambda f:f['index'])
    register_cohort_store(c._Cohort.public_owner,'self.__pending_payment=(payment,self.__source.ordinal)','pending',lambda f:dict(command=h.digest(h.encode(f['payment'])),ordinal=cohort._Cohort__source.ordinal))
    register_cohort_store(c._Cohort.public_owner,'if tree==initialize and decoded==','installs',lambda f:[f['index'],f['current']],lambda f:f['tree']==f['initialize'] and f['decoded']==h.left(h.nat(10)))
    register_cohort_store(c._Cohort.public_owner,'if tree==freeze and decoded==','freezes',lambda f:[f['index'],f['announced']],lambda f:f['tree']==f['freeze'] and f['decoded']==h.left(h.nat(12)))
    register_cohort_store(c._Cohort.public_owner,'if tree==install and decoded==','installs',lambda f:[f['index'],f['current']],lambda f:f['tree']==f['install'] and f['decoded']==h.left(h.nat(7)))
    register_cohort_store(c._Cohort.public_source,'self.__pending_payment=None','pending',lambda f:None)
    register_cohort_store(c._Cohort.invoke_pending,'self.__produced.add(h.encode(envelope))','produced',lambda f:h.digest(h.encode(f['envelope'])))
    receipt['cohort_store_sites']=[dict(function=code.co_name,line=line,kind=spec[0]) for (code,line),spec in cohort_sites.items()]
    writer_pending={};next_writer_store=0
    instructions=list(dis.get_instructions(o.CreditWriter.send,show_caches=False,adaptive=False))
    writer_sites={}
    kinds={'_CreditWriter__credits':'credits','_CreditWriter__image':'image','_CreditWriter__revision':'revision',
        '_CreditWriter__reserved_keys':'keys','_CreditWriter__lease':'releaseLease','_CreditWriter__entered':'clearEntered'}
    for i,instruction in enumerate(instructions):
        kind=None
        if instruction.opname=='STORE_ATTR':kind=kinds.get(instruction.argval)
        elif instruction.opname=='CALL' and any(x.opname=='LOAD_ATTR' and x.argval=='insert' for x in instructions[max(0,i-4):i]):kind='keys'
        if kind is not None:
            writer_sites[instruction.offset]=dict(kind=kind,offset=instruction.offset,next_offset=instructions[i+1].offset,
                opcode=instruction.opname,line=instruction.positions.lineno,column=instruction.positions.col_offset)
    h.require(len(writer_sites)==9,'selected exact writer instruction inventory')
    receipt['writer_statement_sites']=list(writer_sites.values())
    receipt['writer_uncompleted_stores']=[]
    def instruction_event(code,offset):
        nonlocal next_writer_store
        frame=sys._getframe(1)
        h.require(code is o.CreditWriter.send.__code__ and frame.f_code is code,'writer instruction actual frame')
        if id(frame) in writer_pending:
            retained,binding=writer_pending.pop(id(frame));h.require(retained is frame,'writer store retained frame')
            if offset!=binding['store_next_offset']:
                receipt['writer_uncompleted_stores'].append(dict(binding,actual_next_offset=offset))
                append('writer_store_aborted',frame,binding,True)
                raise AssertionError('writer store did not reach its exact next instruction')
            actual=snapshot()['owners'][binding['owner']]
            field={'releaseLease':'lease','clearEntered':'entered'}.get(binding['store_kind'],binding['store_kind'])
            append('writer_store_after',frame,dict(binding,store_value=actual[field],store_after_offset=offset),True)
        site=writer_sites.get(offset)
        if site is not None:
            obj=frame.f_locals['self'];origin,parent,expected,occurrence=writer_bindings[id(obj)]
            h.require(origin is frame and frame.f_back is parent,'writer store actual owning interval')
            peer=obj._CreditWriter__peer
            binding=dict(writer_store=next_writer_store,writer_occurrence=occurrence,writer_frame=id(frame),writer_code=id(code),
                writer_instance=id(obj),owner=expected['index'],endpoint=peer.endpoint,ordinal=peer.ordinal,process_id=peer.process.pid,
                store_kind=site['kind'],store_offset=offset,store_next_offset=site['next_offset'],store_line=site['line'])
            next_writer_store+=1;writer_pending[id(frame)]=(frame,binding)
            append('writer_store_before',frame,binding,True)
    def trace(frame,event,arg):
        nonlocal cohort,next_writer_occurrence,next_source_entry,next_cohort_store
        filename=frame.f_code.co_filename
        if filename not in (c.__file__,o.__file__,h.__file__,RetainingPeer.send.__code__.co_filename):return None
        if event=='line' and frame.f_code is c._Cohort.source_send.__code__ and frame.f_locals.get('owner') is not None:
            obj=frame.f_locals['self'];owner=frame.f_locals['owner'];index=frame.f_locals['target']
            h.require(obj._Cohort__owners[index] is owner,'entry owner association')
            if frame.f_lineno==claim_site:
                h.require(id(frame) not in source_entries,'overlapping source entry frame')
                token=next_source_entry;next_source_entry+=1
                binding=dict(source_entry=token,source_frame=id(frame),source_code=id(frame.f_code),cohort=id(obj),owner=index,writer=id(owner),ticket=tree_sha(frame.f_locals['ticket']))
                source_entries[id(frame)]=(frame,binding)
                append('entry_claim_before',frame,binding,True)
            elif frame.f_lineno in (confirmed_claim_site,stored_site):
                retained,binding=source_entries[id(frame)];h.require(retained is frame,'entry retained frame')
                if frame.f_lineno==confirmed_claim_site:
                    append('entry_claimed',frame,binding,True)
                else:
                    extra=dict(binding,source_ordinal=obj._Cohort__source.ordinal,status=frame.f_locals['status'])
                    append('entry_snapshot_stored',frame,extra,True)
                    del source_entries[id(frame)]
        site=confirmation_sites.get((frame.f_code,frame.f_lineno)) if event=='line' else None
        if site is not None:
            obj=frame.f_locals['self']
            peer=obj._CreditWriter__peer if frame.f_code is o.CreditWriter.send.__code__ else obj._Cohort__source
            identity=(id(peer),peer.ordinal);arg=frame.f_locals[site];slot=peer.unresolved_wire
            h.require(identity not in confirmed_native and slot is not None and slot.ordinal==peer.ordinal and slot.reply==arg,'caller continuation occurrence')
            confirmed_native.add(identity)
            extra=dict(endpoint=peer.endpoint,ordinal=peer.ordinal,process_id=peer.process.pid,
                caller_function=frame.f_code.co_name,caller_frame=id(frame),
                reply_sha256=h.digest(arg) if type(arg) is bytes else None)
            if peer.endpoint.startswith('owner'):
                parent=frame.f_back
                h.require(parent is not None and parent.f_code is c._Cohort.owner_send.__code__,'actual immediate writer parent')
                owning=parent.f_locals['self'];index=parent.f_locals['index']
                h.require(owning is cohort and owning._Cohort__owners[index] is obj,'actual cohort writer association')
                expected=dict(frame=id(parent),code=id(parent.f_code),function='owner_send',cohort=id(owning),index=index,writer=id(obj))
                token=next_writer_occurrence;next_writer_occurrence+=1
                h.require(id(obj) not in writer_bindings,'overlapping parent custody')
                # Retain frame objects throughout this occurrence; IDs cannot be
                # recycled while they serve as the live provenance witness.
                writer_bindings[id(obj)]=(frame,parent,expected,token)
                extra.update(expected_parent=expected,writer_occurrence=token,writer_frame=id(frame),writer_code=id(frame.f_code),writer_instance=id(obj))
            if peer.endpoint=='source' and id(frame) in source_entries:
                retained,binding=source_entries[id(frame)];h.require(retained is frame,'entry reply exact frame')
                extra.update(binding)
            admin=ancestor(frame,'public_owner')
            if admin is not None and admin.f_locals.get('payment') is not None:
                extra['payment']=dict(command=h.digest(h.encode(admin.f_locals['payment'])),ordinal=cohort._Cohort__source.ordinal)
            invoke=ancestor(frame,'invoke_pending');source=ancestor(frame,'source_send')
            if invoke is not None and source is not None and peer.endpoint=='source':
                target=invoke.f_locals.get('target');tree=source.f_locals.get('tree')
                if target is not None and tree==h.right(h.nested_right(h.right(h.nat(target)),6)):
                    extra['work_finish']=[h.digest(arg),h.digest(invoke.f_locals['response'])]
            append('native_return',frame,extra,True)
            if extra.get('work_finish'):
                # Actual last reply is already captured, outer gate still held.
                for kind,action in [('owner',lambda:views['owners'][0].send(h.encode(h.right(h.nat(999))))),
                                    ('source',lambda:views['source'].send(h.encode(h.right(h.left(h.left(h.unit)))))),
                                    ('invoke',lambda:views['source'].invoke_pending())]:
                    before=len(receipt['stream_sequence'])
                    try:action()
                    except ValueError as error:h.require(str(error)=='public cohort operation active','after-finish gate refusal')
                    else:raise AssertionError('after-finish reentry admitted')
                    h.require(len(receipt['stream_sequence'])==before,'after-finish reentry IO')
                    controls.append(dict(kind='after_finish_'+kind,native_position=before,native_io=0))
        obj=frame.f_locals.get('self')
        if type(obj) is c._Cohort:cohort=obj
        if event in ('line','return','exception'):
            point='host_statement'
            extra={};force=False
            if event=='return' and filename==o.__file__ and frame.f_code.co_name=='send' and type(arg) is bytes:
                peer=obj._CreditWriter__peer
                h.require(id(obj) not in writer_candidates,'overlapping writer return candidate')
                extra=dict(reply_sha256=h.digest(arg),endpoint=peer.endpoint,ordinal=peer.ordinal,process_id=peer.process.pid,writer_instance=id(obj),writer_frame=id(frame))
                origin,parent,expected,token=writer_bindings[id(obj)]
                h.require(origin is frame and frame.f_back is parent,'candidate changed actual parent')
                extra.update(expected_parent=expected,writer_occurrence=token,writer_frame=id(frame),writer_code=id(frame.f_code),writer_instance=id(obj))
                writer_candidates[id(obj)]=(dict(extra),arg,len(receipt['stream_sequence']))
                point='writer_return_candidate';force=True
            elif event=='return' and filename==c.__file__ and frame.f_code.co_name=='owner_send' and type(arg) is bytes:
                writer=obj._Cohort__owners[frame.f_locals['index']]
                candidate,reply,position=writer_candidates.pop(id(writer))
                h.require(reply==arg and position==len(receipt['stream_sequence']),'writer caller continuation mismatch')
                peer=writer._CreditWriter__peer
                h.require(candidate['endpoint']==peer.endpoint and candidate['ordinal']==peer.ordinal and candidate['process_id']==peer.process.pid,'writer caller occurrence')
                origin,parent,expected,token=writer_bindings.pop(id(writer))
                h.require(parent is frame and origin.f_back is frame,'confirmation not actual retained parent')
                actual_parent=dict(frame=id(frame),code=id(frame.f_code),function=frame.f_code.co_name,cohort=id(obj),index=frame.f_locals['index'],writer=id(writer))
                h.require(actual_parent==expected and token==candidate['writer_occurrence'],'confirmed parent association mismatch')
                point='writer_return';extra=dict(candidate);extra['caller_frame']=id(frame);extra['caller_function']='owner_send';extra['confirming_parent']=actual_parent;force=True
            elif event=='return' and filename==c.__file__ and frame.f_code.co_name=='invoke_pending' and return_items(arg) is not None:
                point='invoke_return_candidate';extra['return_items']=return_items(arg);force=True
            append(point,frame,extra,force)
        # This observes completion of a selected source statement, not a
        # return-instruction claim. A throwing statement has no after marker.
        # No-op set/snapshot assignments still get their actual boundary.
        if id(frame) in cohort_pending and event in ('line','return'):
            retained,binding=cohort_pending[id(frame)]
            h.require(retained is frame,'cohort store retained frame')
            if event=='return' or frame.f_lineno!=binding['store_line']:
                append('cohort_store_after',frame,binding,True)
                del cohort_pending[id(frame)]
        site=cohort_sites.get((frame.f_code,frame.f_lineno)) if event=='line' else None
        if site is not None:
            kind,value,guard=site
            if guard(frame.f_locals):
                h.require(id(frame) not in cohort_pending and frame.f_locals['self'] is cohort,'actual cohort store custody')
                binding=dict(cohort_store=next_cohort_store,store_frame=id(frame),store_code=id(frame.f_code),store_cohort=id(cohort),store_kind=kind,store_value=value(frame.f_locals),store_line=frame.f_lineno)
                next_cohort_store+=1;cohort_pending[id(frame)]=(frame,binding)
                append('cohort_store_before',frame,binding,True)
        return trace
    def call(kind,index,payload,fn,expect_reply=True):
        row=dict(id=len(spans),parent=call_stack[-1]['id'] if call_stack else None,kind=kind,endpoint=index,start=len(receipt.get('stream_sequence',[])),depth=len(call_stack),
            payload_kind='none' if payload is None else type(payload).__name__,
            payload_hex=None if payload is None else payload.hex(),
            payload_sha256=None if payload is None else h.digest(payload),expect_reply=expect_reply)
        spans.append(row);row['host_start']=tick('call_begin',call=row['id'],parent=row['parent']);call_stack.append(row)
        try:
            value=fn();row['outcome']='returned'
            if kind=='invoke' and row['depth']==0:
                append('invoke_return',sys._getframe(),dict(return_items=return_items(value),confirmed_call=row['id']),True)
            def facts(v):
                if v is None:return dict(kind='none')
                if type(v) is bytes:return dict(kind='bytes',sha256=h.digest(v),size=len(v))
                return dict(kind='tuple',items=[facts(x) for x in v])
            row['return_value']=facts(value);return value
        except BaseException as error:
            row.update(outcome='raised',error=type(error).__name__,message=str(error));raise
        finally:
            row['stop']=len(receipt.get('stream_sequence',[]));row['host_stop']=tick('call_end',call=row['id'],outcome=row['outcome']);h.require(call_stack.pop() is row,'call parent stack')
    class Source:
        def __init__(self,actual):self.actual=actual
        def send(self,payload,expect_reply=True):return call('source',None,payload,lambda:self.actual.send(payload,expect_reply),expect_reply)
        def invoke_pending(self):
            result=call('invoke',None,None,self.actual.invoke_pending)
            if 'provenance' not in done:
                _,envelope=h.un_sum(h.decode(result[1]));scope,ordinal,revision,ticket,value=h.product_fields(envelope,5)
                foreign=h.pair(h.nat(scope[1]+1),h.pair(ordinal,h.pair(revision,h.pair(ticket,value))))
                tests=[('unretained_arrival',h.right(h.right(h.left(foreign))),'source arrival lacks actual owner production')]
                for depth in (2,3,5):tests.append(('unobserved_notice_'+str(depth),h.right(h.nested_right(h.left(h.pair(h.nat(0),h.nat(999))),depth)),'source notification lacks actual owner acknowledgement'))
                for label,command,message in tests:
                    before=len(receipt['stream_sequence'])
                    try:self.send(h.encode(command))
                    except ValueError as error:h.require(str(error)==message,('provenance first refusal',str(error),message))
                    else:raise AssertionError('unretained provenance accepted')
                    h.require(len(receipt['stream_sequence'])==before,'provenance rejection emitted IO')
                    controls.append(dict(kind=label,native_position=before,native_io=0))
                done.add('provenance')
            return result
    class Owner:
        def __init__(self,actual,i):self.actual=actual;self.i=i
        def send(self,payload,expect_reply=True):return call('owner',self.i,payload,lambda:self.actual.send(payload,expect_reply),expect_reply)
    # Local code-object events avoid instrumenting recursive codec/test helpers.
    # Python3.12 sys.monitoring docs: callbacks keep the observed frame on stack.
    monitor=sys.monitoring;tool=3
    h.require(monitor.get_tool(tool) is None,'monitor identifier already owned')
    codes={RetainingPeer._confirm_return.__code__}
    for cls in (c._Cohort,o.CreditWriter):
        for method in vars(cls).values():
            method=getattr(method,'__wrapped__',method)
            if hasattr(method,'__code__'):codes.add(method.__code__)
    def line_event(code,line):
        frame=sys._getframe(1);h.require(frame.f_code is code,'local line frame identity')
        trace(frame,'line',None)
    def return_event(code,offset,value):
        frame=sys._getframe(1);h.require(frame.f_code is code,'local return frame identity')
        trace(frame,'return',value)
    monitor.use_tool_id(tool,'mirrorea-w4-private-host-test')
    monitor.register_callback(tool,monitor.events.LINE,line_event)
    monitor.register_callback(tool,monitor.events.PY_RETURN,return_event)
    monitor.register_callback(tool,monitor.events.INSTRUCTION,instruction_event)
    for code in codes:
        monitor.set_local_events(tool,code,monitor.events.PY_RETURN | monitor.events.LINE | (monitor.events.INSTRUCTION if code is o.CreditWriter.send.__code__ else 0))
    try:
        with c.native_query_cohort(source_build,source_args,owner_build,owner_args,capture,receipt) as (source,owners):
            wrapped_source=Source(source);wrapped_owners=[Owner(owner,i) for i,owner in enumerate(owners)]
            views.update(source=wrapped_source,owners=wrapped_owners)
            yield wrapped_source,wrapped_owners
            h.require(not cohort_pending,'normal capture left uncompleted cohort store statement')
            h.require(not writer_pending and not receipt['writer_uncompleted_stores'],'normal capture left uncompleted writer store')
    finally:
        for code in codes:monitor.set_local_events(tool,code,0)
        monitor.register_callback(tool,monitor.events.LINE,None)
        monitor.register_callback(tool,monitor.events.PY_RETURN,None)
        monitor.register_callback(tool,monitor.events.INSTRUCTION,None)
        monitor.free_tool_id(tool)

label=sys.argv[1] if len(sys.argv)>1 else 'HOST_WRITER_STATEMENT_OMISSION_V2'
h.require(label in ('HOST_WRITER_STATEMENT_OMISSION_V2',),'explicit capture label')
path=root/(label+'.json')
h.require(not path.exists(),'fresh host commit capture required')
h.main(root/'source-continuation',path.name,'REBUILT_OWNER_BUILD.json','REBUILT_SOURCE_BUILD.json',512,factory)
receipt=json.loads(path.read_text())
result=check(receipt)
from check_writer_parent_occurrences import check as check_parent
result['parent_binding']=check_parent(receipt)
h.require(all(receipt[name]['exit']==0 and receipt[name]['stdout_eof'] for name in ['source','owner0','owner1','owner2']),'native close incomplete')
h.require(selected=={name:sha(root/name) for name in selected},'selected code changed')
result['observer_native_sha256']=sha(w/'cohort_observer_native.py')
result['observer_wall_limit_seconds']=60
result.update(status='passed',at=datetime.datetime.now(datetime.timezone.utc).isoformat(),receipt=str(path),
    receipt_sha256=sha(path),harness_sha256=sha(Path(__file__)),checker_sha256=sha(w/'check_host_commit_capture.py'),candidate_sha256=sha(Path(RetainingPeer.__module__ and sys.modules[RetainingPeer.__module__].__file__)),selected=selected,caller_sources={p.name:sha(p) for p in (w/'writer-occurrence-red-caller').glob('*.py')},host_tree_count=len(receipt['host_tree_bytes']),scope=__doc__)
result['python']=dict(executable=sys.executable,optimize=sys.flags.optimize,cache_prefix=sys.pycache_prefix,dont_write_bytecode=sys.dont_write_bytecode)
result['python_imports']=[dict(module=name,origin=str(Path(module.__file__).absolute()),path=str(Path(module.__file__).resolve()),sha256=sha(Path(module.__file__).resolve())) for name,module in sorted(sys.modules.items()) if getattr(module,'__file__',None) and (Path(module.__file__).absolute().is_relative_to(w) or Path(module.__file__).resolve().is_relative_to(w))]
with (w/(label+'_RESULT.json')).open('x') as f:f.write(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
