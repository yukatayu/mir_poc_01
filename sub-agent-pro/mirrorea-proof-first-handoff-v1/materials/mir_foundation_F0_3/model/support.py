"""Finite positive support logic, independent of physical network topology.
The fixed point is inductive support, not an arbitrary self-supporting fixed point.
"""
from __future__ import annotations
from dataclasses import dataclass
from collections.abc import Mapping, Callable

class Invalid(ValueError):
    pass

@dataclass(frozen=True)
class Formula:
    tag: str
    args: tuple = ()

def Top() -> Formula: return Formula('top')
def Bot() -> Formula: return Formula('bottom')
def Ref(key: str) -> Formula: return Formula('ref', (key,))
def All(*parts: Formula) -> Formula: return Formula('all', tuple(parts))
def Any(*parts: Formula) -> Formula: return Formula('any', tuple(parts))

def validate(f: Formula, depth: int = 0) -> None:
    if depth > 128 or not isinstance(f, Formula): raise Invalid('FormulaBudgetOrShape')
    if f.tag in ('top', 'bottom'):
        if f.args: raise Invalid('ConstantArity')
    elif f.tag == 'ref':
        if len(f.args) != 1 or type(f.args[0]) is not str: raise Invalid('ReferenceShape')
    elif f.tag in ('all', 'any'):
        # Empty All and Any deliberately have standard logical meanings here.
        for p in f.args: validate(p, depth + 1)
    else: raise Invalid('UnknownFormula')

def deps(f: Formula) -> frozenset[str]:
    if f.tag == 'ref': return frozenset(f.args)
    if f.tag in ('top', 'bottom'): return frozenset()
    return frozenset().union(*(deps(p) for p in f.args))

def holds(f: Formula, available: set[str] | frozenset[str]) -> bool:
    if f.tag == 'top': return True
    if f.tag == 'bottom': return False
    if f.tag == 'ref': return f.args[0] in available
    if f.tag == 'all': return all(holds(p, available) for p in f.args)
    if f.tag == 'any': return any(holds(p, available) for p in f.args)
    raise Invalid('UnknownFormula')

def derive(forms: Mapping[str, Formula], eligible: set[str] | frozenset[str]):
    """Return least closure AND minimal derivation rounds, for arbitrary finite maps."""
    keys = set(forms)
    if not set(eligible) <= keys: raise Invalid('UnknownEligibleKey')
    for f in forms.values():
        validate(f)
        if not deps(f) <= keys: raise Invalid('UndeclaredSupport')
    live: set[str] = set(); rank: dict[str, int] = {}
    for iteration in range(len(keys) + 1):
        fresh = {k for k in eligible - live if holds(forms[k], live)}
        if not fresh: return frozenset(live), rank
        rank.update({k: iteration for k in fresh}); live |= fresh
    raise AssertionError('FiniteClosureTerminationViolation')

def witness(f: Formula, live: frozenset[str], rank: Mapping[str, int], bound: int) -> frozenset[str]:
    """One acyclic justification for a live node, even with OR support cycles."""
    allowed = frozenset(k for k in live if rank[k] < bound)
    if not holds(f, allowed): raise Invalid('NoEarlierDerivation')
    if f.tag in ('top', 'bottom'): return frozenset()
    if f.tag == 'ref': return frozenset(f.args)
    if f.tag == 'all': return frozenset().union(*(witness(p, live, rank, bound) for p in f.args))
    return next(witness(p, live, rank, bound) for p in f.args if holds(p, allowed))

def dag_order(nodes: set[str], edges: set[tuple[str, str]]) -> tuple[str, ...]:
    """Edges are prerequisite -> dependent. Fail closed on unknown vertices."""
    if any(a not in nodes or b not in nodes for a,b in edges): raise Invalid('UnknownGraphVertex')
    todo = set(nodes); done: list[str] = []
    while todo:
        ready = sorted(n for n in todo if all(a not in todo for a,b in edges if b == n))
        if not ready: raise Invalid('Cycle')
        done.extend(ready); todo.difference_update(ready)
    return tuple(done)

def retire_closure(forms: Mapping[str, Formula], eligible: frozenset[str], seed: frozenset[str]):
    before, _ = derive(forms, eligible)
    after, _ = derive(forms, eligible - seed)
    return before - after


def check_closure(forms: Mapping[str,Formula], eligible, live, ranks) -> bool:
    """Independent finite certificate kernel: no call to derive().

    Ranked introduction proves live <= least(F); closure proves least(F) <= live.
    Exact height equations additionally check the canonical synchronous rounds.
    This verifies *current support*, not policy trust or proof of arbitrary code.
    """
    keys=set(forms);eligible=set(eligible);live=set(live)
    if not eligible<=keys or not live<=eligible or set(ranks)!=live:raise Invalid('ClosureCertificateDomain')
    if any(type(r)is not int or r<0 for r in ranks.values()):raise Invalid('ClosureCertificateRank')
    for f in forms.values():
        validate(f)
        if not deps(f)<=keys:raise Invalid('UndeclaredSupport')
    inf=float('inf')
    def height(f):
        if f.tag=='top':return -1
        if f.tag=='bottom':return inf
        if f.tag=='ref':return ranks.get(f.args[0],inf)
        hs=[height(p) for p in f.args]
        if f.tag=='all':return max(hs,default=-1)
        return min(hs,default=inf)
    for k in live:
        earlier={n for n,r in ranks.items() if r<ranks[k]}
        if not holds(forms[k],earlier):raise Invalid('CircularSupportCertificate')
        if ranks[k]!=height(forms[k])+1:raise Invalid('NonminimalSupportRank')
    for k in eligible-live:
        if holds(forms[k],live):raise Invalid('OmittedDerivableSupport')
    return True
