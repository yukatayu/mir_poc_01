"""Nonproduction sole-writer resource custody for one fresh budget owner.

Assumes custody of the selected fresh native pipe and stable verified binary.
No populated import, reconnection or sharing of its raw peer is supported.
Ticket and complete response types are supplied by the verified native codec
under pipe custody; generic Python tree parsing is not a full typed decoder.
Lease custody ends at the reserve reply; an adjacent reserve/compute sequence
needs the outer serial coordinator to retain its exclusion interval.
Reservation slots/keys follow actual reserve6 and never reset on compute/install/abandon.
Startup capacity is supplied by the private fresh-process factory, not a grant.
This Python object does not protect against hostile code in the same interpreter.
It grants no authority and never produces a source or owner response locally.
"""
import threading,weakref

_construct_lock=threading.Lock()
_constructed=weakref.WeakKeyDictionary()
import publication_process_check as h

class CreditWriter:
    __slots__=('__peer','__credits','__lease','__failed','__lock','__entered','__image','__revision','__capacity','__reserved_keys')
    def __init__(self,peer,capacity):
        if type(capacity) is not int or not 0<=capacity<=64:raise ValueError('frozen owner startup capacity required')
        with _construct_lock:
            if hasattr(self,'_CreditWriter__peer'):raise ValueError('owner writer already initialized')
            if peer in _constructed:raise ValueError('owner pipe already has a resource writer')
            if peer.ordinal!=0:raise ValueError('resource monitor requires fresh owner pipe')
            _constructed[peer]=True
        self.__peer=peer;self.__credits=512;self.__lease=None;self.__failed=False;self.__entered=False
        self.__lock=threading.Lock();self.__image=None;self.__revision=0
        self.__capacity=capacity;self.__reserved_keys=None
    def __copy__(self):raise TypeError('owner writer is not copyable')
    def __deepcopy__(self,memo):raise TypeError('owner writer is not copyable')
    def __reduce_ex__(self,protocol):raise TypeError('owner writer is not serializable')
    @property
    def credits(self):
        with self.__lock:return self.__credits
    def matches_current(self,revision,image):
        with self.__lock:
            if self.__failed:raise RuntimeError('owner writer retired')
            return self.__image is not None and self.__revision==revision and self.__image==image
    @staticmethod
    def _key(ticket):
        fields=h.product_fields(ticket,15)
        principal,identity=fields[4],fields[5]
        if any(value[0]!='nat' or type(value[1]) is not int or value[1]<0 for value in (principal,identity)):
            raise ValueError('reservation key type')
        return (principal[1],identity[1])
    def slot_available(self,ticket):
        key=self._key(ticket)
        with self.__lock:
            if self.__failed:raise RuntimeError('owner writer retired')
            return (self.__reserved_keys is not None and key not in self.__reserved_keys
                    and len(self.__reserved_keys)<self.__capacity)
    def claim(self,ticket):
        if ticket is None or not isinstance(ticket,tuple):raise ValueError('immutable typed ticket required')
        with self.__lock:
            if self.__failed:raise RuntimeError('owner writer retired')
            if (self.__lease is not None or self.__credits<2 or self.__reserved_keys is None
                or self._key(ticket) in self.__reserved_keys or len(self.__reserved_keys)>=self.__capacity):return False
            self.__lease=ticket;self.__entered=False
            return True
    def entered(self,ticket):
        # The caller supplies this notification only after the actual matching
        # source enter reply; this method does not authenticate that event.
        with self.__lock:
            if self.__failed:raise RuntimeError('owner writer retired')
            if self.__lease is None or self.__lease!=ticket or self.__entered:raise ValueError('no matching staged writer lease')
            self.__entered=True
    def cancel(self):
        # Only before a source enter has been accepted. The caller owns that
        # ordering obligation; this is not an owner/source terminal event.
        with self.__lock:
            if self.__failed:raise RuntimeError('owner writer retired')
            if self.__entered:raise ValueError('entered writer lease cannot be cancelled')
            self.__lease=None
    def send(self,payload):
        if type(payload) is not bytes:raise TypeError('immutable bytes payload required')
        with self.__lock:
            if self.__failed:raise RuntimeError('owner writer retired')
            if self.__lease is not None:
                required=h.encode(h.left(h.right(h.left(self.__lease))))
                if not self.__entered or payload!=required:raise ValueError('writer leased to another command')
            try:
                command=h.decode(payload)
                reply=self.__peer.send(payload)
                self.__peer._confirm_return(reply)
                tree=h.decode(reply)
                kind,value=h.un_sum(tree)
                if kind==0:
                    if value[0]!='nat' or value[1] not in (1,2,3,4,5,6,7,8,10,11,12,14,15,16):
                        raise ValueError('invalid owner status')
                if tree!=h.left(h.nat(16)):
                    if self.__credits==0:raise ValueError('unfunded native reply contradicts budget monitor')
                    self.__credits-=1
                if tree==h.left(h.nat(10)):
                    outer,body=h.un_sum(command);inner,image=h.un_sum(body)
                    if outer!=0 or inner!=0:raise ValueError('initialization reply/request mismatch')
                    if self.__reserved_keys is not None:raise ValueError('duplicate native initialization success')
                    self.__image=image;self.__revision=0;self.__reserved_keys=[]
                elif tree==h.left(h.nat(6)):
                    outer,body=h.un_sum(command)
                    if outer!=0:raise ValueError('reserve reply/request mismatch')
                    inner,body=h.un_sum(body)
                    if inner!=1:raise ValueError('reserve reply/request mismatch')
                    tag,ticket=h.un_sum(body)
                    if tag!=0:raise ValueError('reserve reply/request mismatch')
                    key=self._key(ticket)
                    if (self.__reserved_keys is None or key in self.__reserved_keys
                        or len(self.__reserved_keys)>=self.__capacity):
                        raise ValueError('native reservation contradicts actual slot monitor')
                    self.__reserved_keys.insert(0,key)
                elif tree==h.left(h.nat(7)):
                    outer,body=h.un_sum(command)
                    if outer!=0:raise ValueError('installation reply/request mismatch')
                    for _ in range(3):
                        tag,body=h.un_sum(body)
                        if tag!=1:raise ValueError('installation reply/request mismatch')
                    tag,body=h.un_sum(body)
                    if tag!=0:raise ValueError('installation reply/request mismatch')
                    revision,image=h.product_fields(body,2)
                    if revision[0]!='nat':raise ValueError('installation revision type')
                    self.__revision=revision[1];self.__image=image
                if self.__lease is not None:self.__lease=None
                self.__entered=False
                return reply
            except BaseException:
                self.__peer._retire()
                self.__failed=True
                raise
