"""External-effect crash knowledge model. Start records cannot roll back I/O.
No automatic physical retry after Start, even if the call might not have begun.
An external receipt protocol may strengthen this, but is not assumed here.
"""
from dataclasses import dataclass,replace
@dataclass(frozen=True)
class EffectState:
    durable:str='Absent'   # Absent, Started, Outcome
    phase:str='Idle'       # Idle, Ready, Called, Returned
    up:bool=True
    calls:int=0           # environment fact, NOT known by requester/owner journal
    volatile_result:int|None=None
    retained_result:int|None=None
    requester_result:int|None=None
    authorized:bool=True

def act(s,a):
    if a=='crash':return replace(s,up=False,phase='Idle',volatile_result=None)
    if a=='recover':return replace(s,up=True)
    if a=='revoke':
        if s.phase in ('Ready','Called'):raise ValueError('OwnerGateSerialized')
        return replace(s,authorized=False)
    if not s.up:raise ValueError('Down')
    if a=='start':
        if s.durable!='Absent' or not s.authorized:raise ValueError('NotFreshOrAuthorized')
        return replace(s,durable='Started',phase='Ready')
    if a=='call':
        if s.phase!='Ready':raise ValueError('NoOneUseInvocation')
        return replace(s,phase='Called',calls=s.calls+1)
    if a=='return':
        if s.phase!='Called':raise ValueError('NoCall')
        return replace(s,phase='Returned',volatile_result=42)
    if a=='retain':
        if s.phase!='Returned':raise ValueError('NoReturnedResult')
        return replace(s,durable='Outcome',retained_result=s.volatile_result,phase='Idle')
    if a=='deliver':
        if s.durable!='Outcome' or not s.authorized:raise ValueError('NoCurrentRelease')
        return replace(s,requester_result=s.retained_result)
    if a=='retry':
        if s.durable=='Absent':return act(s,'start')
        # Replay a retained result if available; Started alone stays uncertain.
        return act(s,'deliver') if s.durable=='Outcome' and s.authorized else s
    raise ValueError('UnknownAction')

def visible_knowledge(s):return s.durable,s.retained_result,s.requester_result,s.up
