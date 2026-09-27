"""Nonproduction serial custody of actual source and owner private pipes.

One fresh context owns all peers. Views forward bytes without inventing replies;
source-enter resources and phase notification are internal to that IO path.
Unknown IO outcomes retire the entire context. Known source-entry refusal before
owner IO returns without retiring; required-payment contradictions are fatal.
Funding preservation concerns continuing contexts and matched successful work,
not physical resources after failed or terminated attempts. Host process/pipe isolation,
verified binaries and the trusted supervisor remain assumptions. This is not a
namespace allocator, authenticated network transport or durable recovery API.
"""
from contextlib import contextmanager,ExitStack
import threading
import publication_process_check as h
from owner_resource_writer import CreditWriter

class ResourceRefused(RuntimeError):pass

class _Noncopy:
    __slots__=()
    def __copy__(self):raise TypeError('cohort handle is not copyable')
    def __deepcopy__(self,memo):raise TypeError('cohort handle is not copyable')
    def __reduce_ex__(self,protocol):raise TypeError('cohort handle is not serializable')

class _View(_Noncopy):
    __slots__=('__send',)
    def __init__(self,send):
        if hasattr(self,'_View__send'):raise ValueError('view already initialized')
        self.__send=send
    def send(self,payload,expect_reply=True):
        if type(payload) is not bytes:raise TypeError('immutable bytes payload required')
        return self.__send(payload,expect_reply)

class _SourceView(_View):
    __slots__=('__invoke',)
    def __init__(self,send,invoke):
        super().__init__(send);self.__invoke=invoke
    def invoke_pending(self):return self.__invoke()

class _Cohort(_Noncopy):
    __slots__=('__source','__owners','__snapshot','__lock','__retired','__bootstrapped','__scope','__freezes','__installs','__produced','__entry_gate','__funded','__queried','__initialized','__pending_payment')
    def __init__(self,source,peers,scope,*,owner_capacities,funded=True,queried=True):
        if hasattr(self,'_Cohort__source'):raise ValueError('cohort already initialized')
        if source.ordinal!=0:raise ValueError('fresh source pipe required')
        if type(owner_capacities) is not tuple or len(owner_capacities)!=len(peers):raise ValueError('frozen owner capacity inventory required')
        self.__source=source;self.__owners=[CreditWriter(peer,owner_capacities[i]) for i,peer in enumerate(peers)]
        self.__snapshot=None;self.__lock=threading.RLock();self.__retired=False
        self.__bootstrapped=False;self.__scope=scope;self.__funded=funded;self.__queried=queried
        self.__freezes=set();self.__installs=set();self.__produced=set();self.__entry_gate=threading.Lock()
        self.__initialized=set();self.__pending_payment=None
    @contextmanager
    def __public_entry(self):
        # Pinned CPython reference profile: intrinsic RLock recursion depth
        # identifies this attempt even if interruption occurs after acquire
        # but before an acquired flag could be stored. Cleanup is uninterrupted.
        # The gate snapshot is taken under that RLock BEFORE gate acquisition;
        # failed reentry must not release a gate already owned by its parent.
        depth_before=self.__lock._recursion_count()
        gate_was_locked=None
        try:
            self.__lock.acquire()
            gate_was_locked=self.__entry_gate.locked()
            if gate_was_locked:
                raise ValueError('public cohort operation active')
            if not self.__entry_gate.acquire(blocking=False):
                raise RuntimeError('private gate acquisition contradicted exclusive custody')
            if self.__retired:raise RuntimeError('cohort retired')
            yield
        except BaseException as error:
            if not isinstance(error,Exception):self.retire()
            raise
        finally:
            if self.__lock._recursion_count()>depth_before:
                if gate_was_locked is False and self.__entry_gate.locked():
                    self.__entry_gate.release()
                while self.__lock._recursion_count()>depth_before:
                    self.__lock.release()
    def views(self):
        return _SourceView(self.public_source,self.invoke_pending),[_View(lambda data,reply=True,i=i:self.public_owner(i,data,reply)) for i in range(len(self.__owners))]
    def __query(self,tree):
        # Called only inside public-entry custody. No snapshot is supplied by
        # the caller; the actual native reply is retained in the raw capture.
        try:
            reply=self.__source.send(h.encode(tree))
            self.__source._confirm_return(reply)
            return h.decode(reply)
        except BaseException:
            self.__source._retire()
            self.__retired=True
            raise
    def __cost(self,index):
        value=self.__query(h.right(h.left(h.nat(index))))
        if value[0]!='nat' or value[1]>1536:
            self.__retired=True;raise ValueError('invalid native source cost')
        return value[1]
    def __is_head(self,kind,index,revision):
        request=h.left(h.pair(h.nat(index),h.nat(revision))) if kind=='freeze' else h.right(h.left(h.pair(h.nat(index),h.nat(revision)))) if kind=='install' else h.right(h.right(h.nat(index)))
        try:return h.un_bool(self.__query(h.left(request)))
        except BaseException:
            self.__retired=True
            raise
    def public_owner(self,index,payload,expect_reply=True):
        entry=self.__public_entry()
        try:
            with entry:
                if type(payload) is not bytes:raise TypeError('immutable bytes payload required')
                if self.__snapshot is None:raise ValueError('owner needs initialized source')
                if self.__queried and self.__pending_payment is not None:raise ValueError('pending owner payment requires matching source notification')
                tree=h.decode(payload);image=self.__snapshot['source']['image']
                current=self.__snapshot['published'];announced=self.__snapshot['announced']
                initialize=h.left(h.left(image))
                freeze=h.right(h.nat(announced))
                install=h.left(h.nested_right(h.left(h.pair(h.nat(current),image)),3))
                if tree not in (initialize,freeze,install):raise ValueError('owner command requires generated coordinator path')
                if not expect_reply:raise ValueError('owner reply required')
                payment=None
                if self.__queried:
                    first=tree==initialize and index not in self.__initialized
                    if tree!=initialize and len(self.__initialized)!=len(self.__owners):
                        raise ValueError('owner initialization prelude incomplete')
                    demand=self.__cost(index)
                    if tree!=initialize:
                        kind,revision=('freeze',announced) if tree==freeze else ('install',current)
                        at_head=self.__is_head(kind,index,revision)
                        if kind=='freeze' and announced>current and not at_head:
                            raise ResourceRefused('future freeze awaits certified source head')
                        if at_head:
                            if demand<1:self.__retired=True;raise ValueError('required owner command has no native cost')
                            depth=2 if kind=='freeze' else 5
                            payment=h.right(h.nested_right(h.left(h.pair(h.nat(index),h.nat(revision))),depth))
                    # A first initialize pays its explicit prelude debt. A matching
                    # head freeze/install pays that one occurrence; extras spend
                    # only slack and never create a refundable payment credit.
                    minimum=demand+(0 if payment is not None else 1)
                    if self.__owners[index].credits<minimum:
                        raise ResourceRefused('administrative debit would consume retained source obligation')
                try:
                    reply=self.owner_send(index,payload,expect_reply);decoded=h.decode(reply)
                    if self.__queried:
                        if first:
                            if decoded!=h.left(h.nat(10)):
                                self.__retired=True;raise ValueError('first initialization did not discharge prelude debt')
                            self.__initialized.add(index)
                        if payment is not None:
                            expected=12 if tree==freeze else 7
                            if decoded!=h.left(h.nat(expected)):
                                self.__retired=True;raise ValueError('required owner payment lacks successful native operation')
                            self.__pending_payment=(payment,self.__source.ordinal)
                    if tree==initialize and decoded==h.left(h.nat(10)):self.__installs.add((index,current))
                    if tree==freeze and decoded==h.left(h.nat(12)):self.__freezes.add((index,announced))
                    if tree==install and decoded==h.left(h.nat(7)):self.__installs.add((index,current))
                    return reply
                except BaseException:
                    self.__retired=True
                    raise
        finally:
            # Caller owns cleanup even if contextlib entry never returns.
            entry.gen.close()
    def public_source(self,payload,expect_reply=True):
        entry=self.__public_entry()
        try:
            with entry:
                if type(payload) is not bytes:raise TypeError('immutable bytes payload required')
                if not self.__bootstrapped:return self.source_send(payload,expect_reply)
                tree=h.decode(payload)
                if self.__queried:
                    if self.__snapshot is not None and len(self.__initialized)!=len(self.__owners):
                        raise ValueError('owner initialization prelude incomplete')
                    if self.__pending_payment is not None:
                        command,position=self.__pending_payment
                        if tree!=command:raise ValueError('pending owner payment requires matching source notification')
                        if self.__source.ordinal!=position:
                            self.__retired=True;raise ValueError('source moved while owner payment pending')
                for i in range(len(self.__owners)):
                    if tree in (h.right(h.nested_right(h.left(h.nat(i)),6)),h.right(h.nested_right(h.right(h.nat(i)),6))):
                        raise ValueError('source work requires generated coordinator path')
                # For these administrative notifications, no caller-created tuple
                # can replace an observed native owner reply. Old notifications
                # with retained facts still reach the source's semantic refusal.
                tag,command=h.un_sum(tree)
                if tag==1:
                    depth=0;body=command
                    while True:
                        side,value=h.un_sum(body)
                        if side==0:break
                        depth+=1;body=value
                        if depth==7:break
                    if depth in (2,3,5):
                        index,revision=h.product_fields(value,2)
                        fact=(index[1],revision[1]);facts=self.__installs if depth==5 else self.__freezes
                        if index[0]!='nat' or revision[0]!='nat' or fact not in facts:
                            raise ValueError('source notification lacks actual owner acknowledgement')
                    if depth==1 and h.encode(value) not in self.__produced:
                        raise ValueError('source arrival lacks actual owner production')
                if not expect_reply:raise ValueError('source reply required after bootstrap')
                try:
                    reply=self.source_send(payload,expect_reply)
                    if self.__queried and self.__pending_payment is not None:
                        status,_=h.capacity_reply(reply)
                        if status!=0:
                            # Under the certified live-state/payment premises this
                            # notification must succeed. A contrary reply is fatal;
                            # retaining its old transport ordinal is not a retry path.
                            self.__retired=True
                            raise ValueError('required source notification unexpectedly refused')
                        self.__pending_payment=None
                    return reply
                except BaseException:
                    self.__retired=True
                    raise
        finally:
            # Caller owns cleanup even if contextlib entry never returns.
            entry.gen.close()
    def invoke_pending(self):
        # The public-entry scope starts before snapshot/credit validation and
        # remains held through the complete generated work interval.
        # Concurrent public views cannot cancel, abandon, consume its result,
        # or spend its final credit between source submission and completion.
        # Private helpers use the RLock without opening a public mutator.
        entry=self.__public_entry()
        try:
            with entry:
                if self.__queried:
                    if self.__pending_payment is not None:raise ValueError('pending owner payment requires matching source notification')
                    if len(self.__initialized)!=len(self.__owners):raise ValueError('owner initialization prelude incomplete')
                if self.__snapshot is None or self.__snapshot['dispatch'] is not None:raise ValueError('no idle pending source')
                ticket=self.__snapshot['source']['pending']
                if ticket is None:raise ValueError('source has no pending work')
                target=h.product_fields(ticket,15)[3][1];owner=self.__owners[target]
                if self.__queried:
                    # A known closed source gate may produce its own framed
                    # refusal. An open logical gate must match actual observed
                    # owner image/revision/fence before any source entry occurs.
                    installed=self.__snapshot['installed'];fence=self.__snapshot['fence']
                    for vector in (installed,fence):
                        if vector[0:2]!=('node',0) or len(vector[2])!=len(self.__owners):
                            self.__retired=True;raise ValueError('invalid native publication vector')
                    if installed[2][target]==fence[2][target]:
                        stopped=max((revision for index,revision in self.__freezes if index==target),default=0)
                        if stopped!=self.__snapshot['published']:
                            raise ResourceRefused('actual owner fence differs from current publication')
                        if not owner.matches_current(self.__snapshot['published'],self.__snapshot['source']['image']):
                            raise ResourceRefused('actual owner image differs from current publication')
                    def cost(tree):
                        if tree[0]!='node':return 1
                        fields=0
                        for child in reversed(tree[2]):fields=1+max(cost(child),fields)
                        return 1+fields
                    count=len(self.__owners)
                    if len(h.encode(ticket))+372+12*count+25>65536 or max(count+2,cost(ticket)+24)+7>256:
                        raise ResourceRefused('funded result carrier lacks pre-computation framing space')
                # Bound to actual fresh startup and successful reservation replies;
                # this check remains inside the outer gate and precedes source IO.
                if not owner.slot_available(ticket):
                    raise ResourceRefused('owner reservation key or slot unavailable before source enter')
                # Preserve the existing driver's active-freeze diagnostic. Three
                # credits cover reserve, this probe and compute; later publication
                # and source completion still require separately funded suffixes.
                if owner.credits<3:raise ResourceRefused('owner work and diagnostic credits unavailable before source enter')
                try:
                    entered=self.source_send(h.encode(h.right(h.nested_right(h.left(h.nat(target)),6))))
                    status,snapshot=h.capacity_reply(entered)
                    if status!=0:
                        if not self.__queried:
                            raise ValueError('source refused generated work entry')
                        # source_send has fully cancelled the staged lease. Only
                        # this known framed branch may leave the fatal region.
                    else:
                        if self.__queried and not self.__is_head('finish',target,0):
                            raise ValueError('generated work lacks next source finish obligation')
                        reserved=self.owner_send(target,h.encode(h.left(h.right(h.left(ticket)))))
                        if h.decode(reserved)!=h.left(h.nat(6)):raise ValueError('owner refused generated reservation')
                        probe=self.owner_send(target,h.encode(h.right(h.nat(snapshot['published']+1))))
                        if h.decode(probe)!=h.left(h.nat(2)):raise ValueError('active freeze diagnostic differs')
                        response=self.owner_send(target,h.encode(h.left(h.right(h.right(h.left(h.unit))))))
                        kind,envelope=h.un_sum(h.decode(response))
                        if kind!=1:raise ValueError('no owner production')
                        scope,ordinal,revision,retained,result=h.product_fields(envelope,5)
                        if scope!=h.nat(self.__scope) or revision!=h.nat(snapshot['published']) or retained!=ticket:
                            raise ValueError('owner production correlation differs')
                        finished=self.source_send(h.encode(h.right(h.nested_right(h.right(h.nat(target)),6))))
                        status,after=h.capacity_reply(finished)
                        if status!=0 or after['held'] is not None:raise ValueError('source refused actual terminal notification')
                        self.__produced.add(h.encode(envelope))
                        return finished,response
                except BaseException:
                    self.__retired=True
                    raise
                raise ResourceRefused('source refused generated work entry before owner IO')
        finally:
            # Caller owns cleanup even if contextlib entry never returns.
            entry.gen.close()
    def retire(self):
        with self.__lock:self.__retired=True
    def owner_send(self,index,payload,expect_reply=True):
        with self.__lock:
            if self.__retired:raise RuntimeError('cohort retired')
            if not expect_reply:raise ValueError('owner reply required')
            # Local scheduling rejections occur before IO and keep the context.
            try:return self.__owners[index].send(payload)
            except ValueError as error:
                if str(error)=='writer leased to another command':raise
                self.__retired=True;raise
            except BaseException:self.__retired=True;raise
    def source_send(self,payload,expect_reply=True):
        with self.__lock:
            if self.__retired:raise RuntimeError('cohort retired')
            if not self.__bootstrapped:
                if expect_reply:raise ValueError('bootstrap has no reply')
                try:
                    result=self.__source.send(payload,False)
                    self.__source._confirm_return(result)
                    self.__bootstrapped=True
                    return result
                except BaseException:self.__source._retire();self.__retired=True;raise
            if not expect_reply:raise ValueError('source reply required after bootstrap')
            tree=h.decode(payload)
            target=None
            for i in range(len(self.__owners)):
                if tree==h.right(h.nested_right(h.left(h.nat(i)),6)):target=i;break
            owner=None;ticket=None
            if target is not None:
                if self.__snapshot is None or self.__snapshot['dispatch'] is not None:
                    raise ValueError('source enter requires idle initialized coordinator')
                ticket=self.__snapshot['source']['pending']
                if ticket is None or h.product_fields(ticket,15)[3]!=h.nat(target):
                    raise ValueError('source enter requires its own waiting ticket')
                owner=self.__owners[target]
                if not owner.claim(ticket):raise ResourceRefused('owner work credits unavailable before source enter')
            try:
                # Bind the whole vector from these retained real owner monitors
                # immediately before submission under the same outer lock.
                # This private carrier is not a caller-supplied quota grant.
                if self.__funded:
                    vector=h.node(0,*(h.nat(owner.credits) for owner in self.__owners))
                    payload=h.encode(h.pair(vector,h.decode(payload)))
                    if self.__queried:payload=h.encode(h.right(h.right(h.decode(payload))))
                reply=self.__source.send(payload)
                self.__source._confirm_return(reply)
                status,snapshot=h.capacity_reply(reply)
                if owner is not None:
                    if status==0:
                        if snapshot is None or snapshot['dispatch'] is None:raise ValueError('accepted source omitted dispatch')
                        endpoint,scope,revision,retained=h.product_fields(snapshot['dispatch'],4)
                        if (endpoint!=h.nat(target) or scope!=h.nat(self.__scope) or retained!=ticket
                            or revision!=h.nat(self.__snapshot['published']) or snapshot['held'] is None):
                            raise ValueError('accepted source dispatch differs from retained request')
                        owner.entered(ticket)
                    else:
                        # Only an actual refused reply can release the staged
                        # lease. An exception/unknown outcome takes retirement.
                        if snapshot!=self.__snapshot:raise ValueError('refused source changed state')
                        owner.cancel()
                self.__snapshot=snapshot
                return reply
            except BaseException:
                self.__source._retire()
                self.__retired=True
                raise

def _query_arguments(source_args,owner_args):
    # Freeze exactly the values that will be passed to the fresh processes.
    # This binds slot/dimensions/realm/scope, not global namespace authority.
    source=tuple(source_args);owners=tuple(tuple(args) for args in owner_args)
    def numbers(args,count):
        if len(args)!=count or any(type(value) is not str or not value or
            not value.isascii() or not value.isdecimal() for value in args):
            raise ValueError('native numeric argument shape')
        return tuple(int(value) for value in args)
    realm,places,members,caller,member,principal,scope,capacity=numbers(source,8)
    if places>64 or members>64 or capacity>512 or caller>=places or member>=members:
        raise ValueError('source startup profile mismatch')
    if len(owners)!=places:raise ValueError('owner vector inventory mismatch')
    for index,args in enumerate(owners):
        orealm,op,oa,place,oscope,ocapacity=numbers(args,6)
        if (orealm,op,oa,place,oscope)!=(realm,places,members,index,scope) or ocapacity>64:
            raise ValueError('owner startup assignment/profile mismatch')
    return source,owners

@contextmanager
def native_query_cohort(source_build,source_args,owner_build,owner_args,root,receipt):
    """Candidate guard for reserved suffix resources, with privileged queries.

    Complete actual initialization before further source operations. A matching
    head payment is retained until its truthful source notification. Unrelated
    administration spends only slack; unknown/invalid IO retires the cohort.
    Physical current authority, namespace and whole-system refinement remain open.
    """
    source_args,owner_args=_query_arguments(source_args,owner_args)
    with ExitStack() as stack:
        source=stack.enter_context(h.native(source_build,source_args,'source',root,receipt))
        peers=[stack.enter_context(h.native(owner_build,args,f'owner{i}',root,receipt)) for i,args in enumerate(owner_args)]
        cohort=_Cohort(source,peers,int(source_args[6]),
            owner_capacities=tuple(int(args[5]) for args in owner_args),funded=True,queried=True)
        try:yield cohort.views()
        finally:cohort.retire()
