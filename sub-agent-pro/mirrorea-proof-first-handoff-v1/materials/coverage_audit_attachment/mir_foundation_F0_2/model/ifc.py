"""Sequential command IFC over mathematical scalars and a fixed schema.
Termination-insensitive low trace noninterference; no physical timing/capacity
channels or network scheduler theorem. This checker is not silently enabled on
all .mirx inputs: the actor prototype explicitly selects its public-only profile.
"""
from __future__ import annotations
from dataclasses import dataclass
from f01_model.core import E,Expr,CellSpec,INT,BOOL,infer,evaluate,typed_store,StaticError
@dataclass(frozen=True)
class Cmd:
    tag:str
    args:tuple

def Q(tag,*args):return Cmd(tag,args)

def check(c,schema,pc=0):
    t,a=c.tag,c.args
    if t=='skip':return
    if t=='assign':
        x,e=a
        if x not in schema:raise StaticError('UnknownCell')
        info=infer(e,schema)
        if info.ty!=schema[x].ty:raise StaticError('AssignmentType')
        if max(pc,info.label)>schema[x].label:raise StaticError('WriteDown')
        return
    if t=='emit':
        label,e=a;info=infer(e,schema)
        if label not in (0,1) or max(pc,info.label)>label:raise StaticError('DisclosureDown')
        return
    if t=='seq':
        for s in a:check(s,schema,pc)
        return
    if t in ('if','while'):
        guard=a[0];info=infer(guard,schema)
        if info.ty!=BOOL:raise StaticError('GuardType')
        for s in a[1:]:check(s,schema,max(pc,info.label))
        return
    raise StaticError('UnsupportedCommand')

def run(c,schema,initial,budget=10000):
    check(c,schema)
    if not typed_store(schema,initial):raise StaticError('InitialType')
    s=dict(initial);trace=[];fuel=[budget]
    def go(c):
        fuel[0]-=1
        if fuel[0]<0:raise RuntimeError('ResearchExecutionBudget')
        t,a=c.tag,c.args
        if t=='skip':return
        if t=='assign':s[a[0]]=evaluate(a[1],s);return
        if t=='emit':trace.append((a[0],evaluate(a[1],s)));return
        if t=='seq':
            for x in a:go(x)
            return
        if t=='if':go(a[1] if evaluate(a[0],s) else a[2]);return
        if t=='while':
            while evaluate(a[0],s):
                fuel[0]-=1
                if fuel[0]<0:raise RuntimeError('ResearchExecutionBudget')
                go(a[1])
            return
    go(c);return s,trace

def low_view(trace,observer=0):
    # The ordinal is assigned AFTER projection. It does not expose secret row count.
    values=[v for label,v in trace if label<=observer]
    return [(i,v) for i,v in enumerate(values)]
