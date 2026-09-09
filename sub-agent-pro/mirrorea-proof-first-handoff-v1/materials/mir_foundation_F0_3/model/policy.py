"""Multi-issuer policy evaluation. Authentication assertions are ideal issued
records. Proof certificates NEVER create claims, grants or trust roots here.
"""
from __future__ import annotations
from dataclasses import dataclass, fields, is_dataclass
from .support import Formula, Invalid, All, Any, Ref

from .identity import exact

@dataclass(frozen=True)
class Context:
    principal: str
    member: str
    action: str
    target: str
    instance: str
    request: str
    arguments: tuple
    code: str
    contract: str

@dataclass(frozen=True)
class Claim:
    key: str
    issuer: str
    issuer_epoch: int
    principal: str
    member: str
    predicate: str
    actions: frozenset[str]
    targets: frozenset[str]
    label: int = 0

@dataclass(frozen=True)
class Need:
    issuer: str
    predicate: str

@dataclass(frozen=True)
class Policy:
    key: str
    version: int
    expression: Formula
    needs: tuple[tuple[str, Need], ...]
    label: int = 0

@dataclass(frozen=True)
class PolicyEvidence:
    policy: str
    version: int
    context: Context
    used: tuple[str, ...]
    branch: tuple

class Authority:
    def __init__(self):
        self.epochs={}; self.claims={}; self.revoked=set(); self.policies={}; self.counters={}
    def add_issuer(self, issuer):
        if issuer in self.epochs: raise Invalid('IssuerAlreadyExists')
        self.epochs[issuer]=0
    def issue(self, issuer, principal, member, predicate, actions, targets, label=0):
        if issuer not in self.epochs: raise Invalid('UnknownIssuer')
        if type(label)is not int or label<0 or not actions or not targets:raise Invalid('ClaimScopeOrLabel')
        counter=(issuer,label)
        n=self.counters.get(counter,0)+1; self.counters[counter]=n
        key=f'{label}:{issuer}:{n}'
        c=Claim(key,issuer,self.epochs[issuer],principal,member,predicate,frozenset(actions),frozenset(targets),label)
        self.claims[key]=c
        return c
    def retire_issuer_epoch(self, issuer):
        if issuer not in self.epochs: raise Invalid('UnknownIssuer')
        self.epochs[issuer]+=1
    def install_policy(self, policy):
        from .support import validate,deps
        validate(policy.expression)
        if type(policy.label)is not int or policy.label<0:raise Invalid('PolicyLabel')
        if policy.expression.tag in ('top','bottom'): raise Invalid('ExplicitNonemptyPolicyRequired')
        needs=dict(policy.needs)
        if len(needs)!=len(policy.needs) or not deps(policy.expression)<=set(needs):raise Invalid('UndeclaredPolicyPredicate')
        # Policy All/Any may not contain empty lists or top: explicit branch requirements.
        def shape(f):
            if f.tag=='ref':return
            if f.tag not in ('all','any') or not f.args:raise Invalid('EmptyPolicyBranch')
            for x in f.args:shape(x)
        shape(policy.expression)
        if any(n.issuer not in self.epochs for n in needs.values()):raise Invalid('UnknownIssuer')
        old=self.policies.get(policy.key)
        if old and policy.version!=old.version+1:raise Invalid('PolicyVersionNotSuccessor')
        if not old and policy.version!=0:raise Invalid('InitialPolicyVersion')
        self.policies[policy.key]=policy
    def evaluate(self, key, ctx: Context, supplied: tuple[Claim,...]):
        if key not in self.policies:raise Invalid('UnknownPolicy')
        p=self.policies[key]; needs=dict(p.needs)
        def leaf(name):
            need=needs[name]
            for c in supplied:
                if not exact(self.claims.get(c.key),c) or c.key in self.revoked:continue
                if (c.issuer,c.predicate)!=(need.issuer,need.predicate):continue
                if self.epochs.get(c.issuer)!=c.issuer_epoch:continue
                if (c.principal,c.member)!=(ctx.principal,ctx.member):continue
                if ctx.action not in c.actions or ctx.target not in c.targets:continue
                if c.label>p.label:continue
                return (c.key,),('leaf',name,c.key)
            raise Invalid('PolicyDenied')
        def go(f):
            if f.tag=='ref':return leaf(f.args[0])
            if f.tag=='all':
                parts=[go(q) for q in f.args]
                return tuple(k for ks,_ in parts for k in ks),('all',tuple(b for _,b in parts))
            for i,q in enumerate(f.args):
                try:
                    ids,b=go(q);return ids,('any',i,b)
                except Invalid:pass
            raise Invalid('PolicyDenied')
        used,branch=go(p.expression)
        return PolicyEvidence(p.key,p.version,ctx,tuple(sorted(set(used))),branch)
    def revalidate(self, ev:PolicyEvidence, ctx:Context):
        if not exact(ev.context,ctx):raise Invalid('AuthorizationContextMismatch')
        if ev.policy not in self.policies or self.policies[ev.policy].version!=ev.version:raise Invalid('StalePolicy')
        supplied=tuple(self.claims[k] for k in ev.used if k in self.claims)
        now=self.evaluate(ev.policy,ctx,supplied)
        if not exact(now,ev):raise Invalid('ChangedPolicyEvidence')
        return now
