"""Nonproduction private retaining wrapper over the selected real Peer IO.

Single exclusive ordered writer is a PRECONDITION supplied by cohort custody;
this is not hostile-Python isolation, network auth, durable recovery or a retry
API. An unknown IO keeps its immutable request and last returned occurrence.
A fully read raw reply is retained before optional capture writes. It is NOT yet
a typed/validated reply or a completed caller return: unresolved_wire.reply keeps
those bytes if later work fails; last_returned names only send returns confirmed by the owning caller.
Return-instruction events are candidates, not evidence of caller continuation.
Bootstrap write-without-reply stays labelled unacknowledged, never as proof of
native semantic completion. This adapter creates no response or native state.
"""
from dataclasses import dataclass
import publication_process_check as h

@dataclass(frozen=True,slots=True)
class WireOccurrence:
    endpoint:str
    process_id:int
    ordinal:int
    payload:bytes
    expect_reply:bool
    reply:bytes|None=None

class RetainingPeer(h.Peer):
    def __init__(self,*args,**kwargs):
        if hasattr(self,'_RetainingPeer__constructed'):raise ValueError('native peer already initialized')
        self.__constructed=True
        super().__init__(*args,**kwargs)
        self.__outstanding=None
        self.__last_returned=None
        self.__failed=False
        self.__read_phase=None
        self.__return_candidate=False
    def __copy__(self):raise TypeError('native peer is not copyable')
    def __deepcopy__(self,memo):raise TypeError('native peer is not copyable')
    def __reduce_ex__(self,protocol):raise TypeError('native peer is not serializable')
    @property
    def unresolved_wire(self):return self.__outstanding
    @property
    def last_returned_wire(self):return self.__last_returned
    @property
    def wire_retired(self):return self.__failed
    def read(self,count):
        # Bound to the selected base Peer's header/body framing sequence. The
        # inherited reader returns only a complete chunk; partial reads and
        # read exceptions do not establish receipt. Capture/decoding happen later.
        phase=self.__read_phase
        data=None
        try:
            data=super().read(count)
            if phase=='header':
                if count!=4:raise ValueError('private reply header shape')
                self.__read_phase=int.from_bytes(data,'big')
            elif type(phase) is int:
                if count!=phase:raise ValueError('private reply body shape')
                occurrence=self.__outstanding
                if occurrence is None:raise RuntimeError('private reply without occurrence')
                self.__outstanding=WireOccurrence(occurrence.endpoint,occurrence.process_id,occurrence.ordinal,
                    occurrence.payload,occurrence.expect_reply,data)
                self.__read_phase=None
            return data
        except BaseException:
            # Preserve a complete body already assigned to the local bytes
            # variable even if its normal slot publication was interrupted.
            # An exception INSIDE the inherited reader still means unknown;
            # this is not an atomic OS-consume-to-Python-store guarantee.
            if type(phase) is int and type(data) is bytes and len(data)==phase:
                occurrence=self.__outstanding
                if occurrence is not None:
                    self.__outstanding=WireOccurrence(occurrence.endpoint,occurrence.process_id,occurrence.ordinal,
                        occurrence.payload,occurrence.expect_reply,data)
                    self.__read_phase=None
            raise
    def send(self,payload,expect_reply=True):
        if self.__failed:raise RuntimeError('native peer retired')
        if self.__outstanding is not None:raise RuntimeError('native peer request outstanding')
        if type(payload) is not bytes:raise TypeError('immutable bytes payload required')
        h.require(len(payload)<=65536,'native experiment check failed')
        if type(expect_reply) is not bool:raise TypeError('reply expectation must be boolean')
        occurrence=WireOccurrence(self.endpoint,self.process.pid,self.ordinal+1,payload,expect_reply)
        try:
            self.__outstanding=occurrence
            self.__read_phase='header' if expect_reply else None
            reply=super().send(payload,expect_reply)
            occurrence=WireOccurrence(occurrence.endpoint,occurrence.process_id,occurrence.ordinal,
                occurrence.payload,occurrence.expect_reply,reply)
            self.__outstanding=occurrence
            self.__return_candidate=True
            return reply
        except BaseException:
            # The sole injected interruption has already occurred; repeated
            # interruption of this cleanup is outside the trusted host profile.
            self.__failed=True
            if self.__outstanding is None:self.__outstanding=occurrence
            raise

    def _confirm_return(self,reply):
        # Only the owning caller invokes this after its send call actually
        # continues. This is private occurrence custody, not authentication or
        # a source-visible receipt API. A return hook alone cannot confirm it.
        if self.__failed:raise RuntimeError('native peer retired')
        occurrence=self.__outstanding
        if not self.__return_candidate or occurrence is None or reply!=occurrence.reply:
            raise RuntimeError('no matching returned native occurrence')
        self.__last_returned=occurrence
        self.__outstanding=None
        self.__return_candidate=False
    def _retire(self):
        # Called by the owning caller's exceptional unwind, including a
        # callee return-hook failure outside that callee's exception table.
        self.__failed=True
