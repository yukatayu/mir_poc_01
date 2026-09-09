"""Optimistic validation with tracked reads INCLUDING absence and query-index rows.
A cell version survives deletion; value equality alone does not discharge a stamp.
The verifier is trusted code. Untrusted plugins cannot directly publish deltas.
Repeated reads retain their first version: a changed version aborts validation.
Each (version, value) lookup is an abstract atomic read; real threads need a
lock or equivalent record-read contract, not two unsynchronized Python dicts.
"""
from __future__ import annotations
from dataclasses import dataclass
from copy import deepcopy
from typing import Any, Callable
from .support import Invalid
from .identity import exact

ABSENT = ('F03.absent',)

@dataclass(frozen=True)
class Preparation:
    owner: str
    reads: tuple[tuple[str, int], ...]
    writes: tuple[tuple[str, Any], ...]
    result: Any

class View:
    def __init__(self, store, patch):
        self._store = store; self._patch = deepcopy(patch); self.reads = {}
    def get(self, key: str):
        if key in self._patch: return deepcopy(self._patch[key])
        stamp = self._store.versions.get(key, 0)
        if key in self.reads and self.reads[key] != stamp:
            raise Invalid('UnstablePreparationRead')
        self.reads[key] = stamp
        return deepcopy(self._store.values.get(key, ABSENT))
    def require(self, key: str):
        value = self.get(key)
        if value == ABSENT: raise Invalid('Absent:' + key)
        return value

class Store:
    def __init__(self, values=None):
        self.values = deepcopy(values or {}); self.versions = {k:1 for k in self.values}
        self.issued = {}; self.counter = 0
    def write(self, patch):
        # Atomic logical record update; no physical atomicity is claimed.
        for k,v in deepcopy(patch).items():
            self.versions[k] = self.versions.get(k, 0) + 1
            if v == ABSENT: self.values.pop(k, None)
            else: self.values[k] = v
    def prepare(self, owner: str, patch: dict, validator: Callable[[View], Any]):
        view = View(self, patch)
        # Blind writes need write-write conflict protection as well.
        for key in patch: view.reads[key] = self.versions.get(key, 0)
        result = validator(view)
        prep = Preparation(owner, tuple(sorted(view.reads.items())), tuple(sorted(deepcopy(patch).items())), deepcopy(result))
        self.counter += 1
        token = (owner, self.counter)
        self.issued[token] = deepcopy(prep)
        return token, prep
    def commit(self, token, prep: Preparation, principal: str, authorize: Callable[[str, Preparation], bool]):
        if not exact(self.issued.get(token), prep): raise Invalid('UnboundPreparation')
        if principal != prep.owner or not authorize(principal, prep): raise Invalid('CurrentAuthorizationDenied')
        if any(self.versions.get(k,0) != v for k,v in prep.reads): raise Invalid('StalePreparation')
        # Current authorization and write are one abstract control transition.
        self.write(dict(prep.writes)); del self.issued[token]
        return deepcopy(prep.result)


def validate_new_edge(view: View, source: str, target: str):
    """Validate no target ->* source path after the prospective update.
    Each outgoing adjacency list is an indexed row, including empty lists.
    """
    seen=set(); todo=[target]
    while todo:
        x=todo.pop()
        if x==source: raise Invalid('Cycle')
        if x in seen: continue
        seen.add(x)
        outgoing=view.get('out/'+x)
        if outgoing == ABSENT: outgoing=()
        if not isinstance(outgoing,tuple): raise Invalid('MalformedAdjacency')
        todo.extend(outgoing)
    return tuple(sorted(seen))
