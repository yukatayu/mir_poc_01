"""Concrete exclusive-region model for a LOCAL theory example.
Not malloc, not C memory, not a claim that every separation logic is integrated.
Authority to run operations and possession of an ownership token are distinct.
"""
from dataclasses import dataclass

@dataclass(frozen=True)
class RegionToken:
    serial: int
    block: int
    start: int
    length: int
    holder: str

class ExclusiveRegions:
    def __init__(self):
        self.next_block=0
        self.next_token=0
        self.live: dict[int,RegionToken]={}
        self.allowed: set[str]=set()

    def authorize(self,principal: str):
        # Trusted policy input, NOT the result of arithmetic verification.
        self.allowed.add(principal)

    def _authorized(self,principal: str):
        if principal not in self.allowed: raise PermissionError('NoEffectPermission')

    def _issue(self,block,start,length,holder):
        t=RegionToken(self.next_token,block,start,length,holder)
        self.next_token+=1;self.live[t.serial]=t
        return t

    def allocate(self,principal: str,n: int):
        self._authorized(principal)
        if type(n) is not int or n<=0:raise ValueError('PositiveLengthRequired')
        block=self.next_block;self.next_block+=1
        t=self._issue(block,0,n,principal);self.assert_separated();return t

    def split(self,principal: str,t:RegionToken,k:int):
        self._authorized(principal)
        if self.live.get(t.serial)!=t or t.holder!=principal:raise PermissionError('StaleOwnership')
        if type(k) is not int or not 0<k<t.length:raise ValueError('InteriorSplitRequired')
        del self.live[t.serial]
        a=self._issue(t.block,t.start,k,principal)
        b=self._issue(t.block,t.start+k,t.length-k,principal)
        self.assert_separated();return a,b

    def release(self,principal: str,t:RegionToken):
        self._authorized(principal)
        if self.live.get(t.serial)!=t or t.holder!=principal:raise PermissionError('StaleOwnership')
        del self.live[t.serial];self.assert_separated()

    def move(self,principal: str,t:RegionToken,new_holder: str):
        self._authorized(principal)
        if self.live.get(t.serial)!=t or t.holder!=principal:raise PermissionError('StaleOwnership')
        del self.live[t.serial]
        out=self._issue(t.block,t.start,t.length,new_holder)
        # No self.allowed mutation: ownership transfer is not a policy grant.
        self.assert_separated();return out

    def assert_separated(self):
        tokens=list(self.live.values())
        assert all(t.length>0 and t.start>=0 for t in tokens)
        for i,a in enumerate(tokens):
            for b in tokens[i+1:]:
                assert a.block!=b.block or a.start+a.length<=b.start or b.start+b.length<=a.start
