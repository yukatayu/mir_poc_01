"""Exact least completion for a FINITE, COMPLETE checkpoint-dependency input.
Dependencies are (sender_process, earliest_sender_checkpoint,
                  receiver_process, earliest_receiver_checkpoint).
Each process chooses one checkpoint index in 0..upper[p].
Choosing receiver >= r requires sender >= s. Extra causal dependencies
can use the same representation. This does not discover missing messages.
"""
from __future__ import annotations
from dataclasses import dataclass
from itertools import product

@dataclass(frozen=True)
class Completion:
    selected: tuple[int,...] | None
    raises: tuple[tuple[int,int,int],...]
    conflict: tuple[int,int,int] | None


def complete(upper: tuple[int,...], dependencies: tuple[tuple[int,int,int,int],...],
             required: dict[int,int]) -> Completion:
    if not upper or any(type(u) is not int or u<0 for u in upper):raise ValueError('BadBounds')
    n=len(upper)
    if any(p not in range(n) or c not in range(upper[p]+1) for p,c in required.items()):raise ValueError('BadRequired')
    for sender,s,receiver,r in dependencies:
        if sender not in range(n) or receiver not in range(n) or s<0 or r<0:
            raise ValueError('BadDependency')
    cap=list(upper)
    lower=[0]*n
    for p,c in required.items():lower[p]=c;cap[p]=c
    raises=[]
    while True:
        changed=False
        for sender,s,receiver,r in dependencies:
            if lower[receiver]>=r and lower[sender]<s:
                old=lower[sender]
                lower[sender]=s;raises.append((sender,old,s));changed=True
                if s>cap[sender]:return Completion(None,tuple(raises),(sender,s,cap[sender]))
        if not changed:
            return Completion(tuple(lower),tuple(raises),None)


def satisfies(choice,upper,dependencies,required):
    return (len(choice)==len(upper) and all(0<=x<=u for x,u in zip(choice,upper))
            and all(choice[p]==c for p,c in required.items())
            and all(choice[rp]<rc or choice[sp]>=sc for sp,sc,rp,rc in dependencies))


def exhaustive(upper,dependencies,required):
    return [c for c in product(*(range(u+1) for u in upper)) if satisfies(c,upper,dependencies,required)]


def find_z_path(upper: tuple[int,...], dependencies: tuple[tuple[int,int,int,int],...],
                required: dict[int,int]) -> tuple[int,...] | None:
    """Independent message-graph search for T25's Z-path.

    Only for complete finite horizons: every send/receive is included in some
    final checkpoint, and checkpoint zero is before every such event.
    Returned integers index dependencies. This is not the closure algorithm.
    """
    from collections import deque
    if not upper or any(type(u) is not int or u < 0 for u in upper):
        raise ValueError('BadBounds')
    if any(p not in range(len(upper)) or c not in range(upper[p]+1)
           for p,c in required.items()):
        raise ValueError('BadRequired')
    for sp,a,rp,b in dependencies:
        if (sp not in range(len(upper)) or rp not in range(len(upper))
                or not 1 <= a <= upper[sp] or not 1 <= b <= upper[rp]):
            raise ValueError('IncompleteHorizon')
    queue = deque((i,) for i,(sp,a,_,_) in enumerate(dependencies)
                  if sp in required and a > required[sp])
    visited = {path[-1] for path in queue}
    while queue:
        path = queue.popleft()
        _,_,rp,b = dependencies[path[-1]]
        if rp in required and b <= required[rp]:
            return path
        for j,(sp,a,_,_) in enumerate(dependencies):
            if sp == rp and a >= b and j not in visited:
                visited.add(j)
                queue.append(path + (j,))
    return None
