"""Crash/recovery for EXPLICIT distributed patch activation, not implicit data tx.
Durable preparation and decision; honest coordinator per finite patch cohort;
unknown decision blocks affected resources. No leader election or Byzantine proof.
"""
from __future__ import annotations
from dataclasses import dataclass,replace
from collections import deque
import json
U,C,A='Undecided','Commit','Abort'
OLD,PREPARED,NEW,ABORTED=range(4)
@dataclass(frozen=True)
class State:
    decision:str
    phase:tuple[int,...]
    votes:int
    known:tuple[str,...]
    down:int=0
@dataclass(frozen=True)
class Action:
    name:str
    participant:int=-1
    def text(self):return self.name+(str(self.participant) if self.participant>=0 else '')
def initial(n):
    if n<=0:raise ValueError('NonemptyPatchCohort')
    return State(U,(OLD,)*n,0,(U,)*n)
def active(s,i):return not(s.down>>i&1)
def actions(s,mutant=None):
    n=len(s.phase)
    if active(s,n):
        if s.decision==U:
            yield Action('decide-abort')
            if s.votes==(1<<n)-1:yield Action('decide-commit')
        for i in range(n):
            if not active(s,i):continue
            if s.phase[i]==OLD and s.decision==U:yield Action('prepare',i)
            if s.phase[i]==PREPARED and not(s.votes>>i&1) and s.decision==U:yield Action('vote',i)
            if s.decision!=U and s.known[i]==U:yield Action('learn',i)
    for i in range(n):
        if active(s,i):
            if s.known[i]==C and s.phase[i]==PREPARED:yield Action('install',i)
            if s.known[i]==A and s.phase[i] in (OLD,PREPARED):yield Action('abort-local',i)
            if mutant=='timeout-abort' and s.known[i]==U and s.phase[i]==PREPARED:yield Action('timeout-unfreeze',i)
    for i in range(n+1):yield Action('crash' if active(s,i) else 'recover',i)
def step(s,a,mutant=None):
    n=len(s.phase);i=a.participant;ph=list(s.phase);kn=list(s.known)
    if a.name=='prepare':ph[i]=PREPARED;return replace(s,phase=tuple(ph))
    if a.name=='vote':return replace(s,votes=s.votes|(1<<i))
    if a.name=='learn':kn[i]=s.decision;return replace(s,known=tuple(kn))
    if a.name=='install':ph[i]=NEW;return replace(s,phase=tuple(ph))
    if a.name=='abort-local':ph[i]=ABORTED;return replace(s,phase=tuple(ph))
    if a.name=='decide-commit':return replace(s,decision=C)
    if a.name=='decide-abort':return replace(s,decision=A)
    if a.name=='timeout-unfreeze':ph[i]=OLD;return replace(s,phase=tuple(ph))
    if a.name=='crash':return replace(s,down=s.down|(1<<i))
    if a.name=='recover':
        if mutant=='volatile-prepare' and i<n and ph[i]==PREPARED:ph[i]=OLD
        if mutant=='volatile-decision' and i==n:return replace(s,decision=U,down=s.down&~(1<<i))
        return replace(s,phase=tuple(ph),down=s.down&~(1<<i))
    raise ValueError(a)
def violations(s):
    n=len(s.phase);errors=[]
    if s.decision==U and any(p in (NEW,ABORTED) for p in s.phase):errors.append('active-version-before-decision')
    if s.decision==C and any(p in (OLD,ABORTED) for p in s.phase):errors.append('old-visible-after-commit')
    if s.decision==C and s.votes!=(1<<n)-1:errors.append('commit-without-all-votes')
    if s.decision==A and NEW in s.phase:errors.append('new-visible-after-abort')
    for i,p in enumerate(s.phase):
        if s.known[i]!=U and s.known[i]!=s.decision:errors.append('contradictory-learned-decision')
        if p==NEW and s.known[i]!=C:errors.append('new-without-commit-evidence')
        if s.decision==U and (s.votes>>i&1) and p!=PREPARED:errors.append('vote-without-durable-preparation')
    return errors
def abstract_epoch(s):return 1 if s.decision==C else 0
def read(s,i):
    if not active(s,i) or s.phase[i]==PREPARED:return None
    return 1 if s.phase[i]==NEW else 0
def explore(n=2,mutant=None,retain_states=False):
    start=initial(n);parents={start:None};todo=deque([start]);transitions=0
    while todo:
        s=todo.popleft();bad=violations(s)
        if bad:
            path=[];q=s
            while parents[q] is not None:
                prev,a=parents[q];path.append(a.text());q=prev
            return {'participants':n,'states':len(parents),'transitions':transitions,'safe':False,'counterexample':list(reversed(path)),'violations':bad,'state':repr(s)}
        for i in range(n):
            v=read(s,i)
            if v is not None and v!=abstract_epoch(s):raise AssertionError('AbstractionViolation')
        for a in actions(s,mutant):
            nxt=step(s,a,mutant);transitions+=1
            if nxt not in parents:parents[nxt]=(s,a);todo.append(nxt)
    result={'participants':n,'states':len(parents),'transitions':transitions,'safe':True,'counterexample':None}
    return (result,tuple(parents)) if retain_states else result
def finish_after_recovery(s):
    n=len(s.phase);s=replace(s,down=0)
    if s.decision==U:
        for i in range(n):
            if s.phase[i]==OLD:s=step(s,Action('prepare',i))
            if not(s.votes>>i&1):s=step(s,Action('vote',i))
        s=step(s,Action('decide-commit'))
    for i in range(n):
        if s.known[i]==U:s=step(s,Action('learn',i))
        if s.decision==C and s.phase[i]==PREPARED:s=step(s,Action('install',i))
        elif s.decision==A and s.phase[i] in (OLD,PREPARED):s=step(s,Action('abort-local',i))
    return s
if __name__=='__main__':print(json.dumps([explore(n) for n in (1,2,3)]+[explore(2,m) for m in ('timeout-abort','volatile-prepare','volatile-decision')],indent=2))
