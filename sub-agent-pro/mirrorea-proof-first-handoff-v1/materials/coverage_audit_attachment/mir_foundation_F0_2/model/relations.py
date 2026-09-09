"""Late-evaluated relation DAGs. Semantic invalidation and sample gaps differ.
Only admitted, exact-epoch, coherent logical samples are evaluated. No renderer,
physical-clock accuracy, authenticity or arbitrary constraint solver is claimed.
"""
from __future__ import annotations
from dataclasses import dataclass,replace
from fractions import Fraction
from .language import DomainFailure,StaticError

@dataclass(frozen=True)
class Anchor:
    name:str
    incarnation:int
@dataclass(frozen=True)
class Sample:
    anchor:Anchor
    frontier:int
    value:Fraction
    label:int=0
@dataclass(frozen=True)
class Term:
    tag:str
    args:tuple

def K(v):return Term('constant',(Fraction(v),))
def Ref(n):return Term('ref',(n,))
def Add(a,b):return Term('add',(a,b))
def Scale(k,a):return Term('scale',(Fraction(k),a))
def refs(t):
    if t.tag=='constant':return set()
    if t.tag=='ref':return {t.args[0]}
    if t.tag=='add':return refs(t.args[0])|refs(t.args[1])
    if t.tag=='scale':return refs(t.args[1])
    raise StaticError('UnknownRelationTerm')
def evaluate(t,env):
    if t.tag=='constant':return t.args[0]
    if t.tag=='ref':return env[t.args[0]]
    if t.tag=='add':return evaluate(t.args[0],env)+evaluate(t.args[1],env)
    if t.tag=='scale':return t.args[0]*evaluate(t.args[1],env)
    raise StaticError('UnknownRelationTerm')
@dataclass
class Binding:
    candidates:tuple[Anchor,...]
    lineage:int
    cursor:int=0
    def resolve(self,live):
        n=len(self.candidates)
        if not 0<=self.cursor<=n:raise StaticError('BadFallbackCursor')
        while self.cursor<n and self.candidates[self.cursor] not in live:self.cursor+=1
        return self.candidates[self.cursor] if self.cursor<n else None
class RelationSystem:
    def __init__(self,bindings,definitions,definition_labels=None):
        self.definition_labels=dict(definition_labels or {})
        if not set(self.definition_labels)<=set(definitions) or any(type(v)is not int or v<0 for v in self.definition_labels.values()):raise StaticError("DefinitionLabel")
        self.bindings=dict(bindings);self.definitions=dict(definitions);self.used_lineages={b.lineage for b in bindings.values()}
        if set(bindings)&set(definitions):raise StaticError('RelationNameCollision')
        todo=set(definitions);known=set(bindings);order=[]
        while todo:
            ready=sorted(n for n in todo if refs(definitions[n])<=known)
            if not ready:raise StaticError('RelationCycleOrMissingDependency')
            for n in ready:order.append(n);todo.remove(n);known.add(n)
        self.order=tuple(order)
    def invalidate(self,live,permission=False):
        if not permission:raise DomainFailure('RelationAuthorityDenied')
        for b in self.bindings.values():b.resolve(live)
    def reacquire(self,name,candidates,new_lineage,permission=False):
        if not permission:raise DomainFailure('RelationAuthorityDenied')
        if new_lineage in self.used_lineages:raise DomainFailure('ReusedLineage')
        self.used_lineages.add(new_lineage);self.bindings[name]=Binding(tuple(candidates),new_lineage)
    def project(self,samples,live,frontier,observer=0):
        env={};labels={};selected={}
        for n,b in self.bindings.items():
            if b.cursor>=len(b.candidates):raise DomainFailure('NoSemanticAnchor')
            a=b.candidates[b.cursor]
            if a not in live:raise DomainFailure('StaleSemanticBinding')
            sample=samples.get(a)
            if sample is None:raise DomainFailure('PresentationGap')
            if sample.anchor!=a or sample.frontier!=frontier:raise DomainFailure('IncoherentPresentation')
            if sample.label>observer:raise DomainFailure('VisibilityDenied')
            env[n]=sample.value;labels[n]=sample.label;selected[n]=(a,b.lineage,b.cursor)
        for n in self.order:
            env[n]=evaluate(self.definitions[n],env)
            labels[n]=max(self.definition_labels.get(n,0),max((labels[r] for r in refs(self.definitions[n])),default=0))
            if labels[n]>observer:raise DomainFailure('VisibilityDenied')
        return env,labels,selected
