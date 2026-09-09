"""Exact polynomial proof certificates, rebuilt from checked operation source.
Certificate production is untrusted. This small checker is part of the research
TCB; it has a hand soundness proof, not a Lean-verified implementation.
No submitted axiom, arbitrary callback, eval, float or solver-success flag.
"""
from __future__ import annotations
import ast
from dataclasses import dataclass
from fractions import Fraction
from itertools import product
from .language import Operation,DomainFailure,digest

class CertificateError(ValueError):pass
@dataclass(frozen=True)
class Poly:
    terms:tuple[tuple[tuple[str,...],Fraction],...]
    def __post_init__(self):
        if type(self.terms)is not tuple or tuple(sorted(self.terms))!=self.terms:raise CertificateError('NonCanonicalPolynomial')
        seen=set()
        for m,c in self.terms:
            if type(m)is not tuple or tuple(sorted(m))!=m or m in seen or not isinstance(c,Fraction) or not c or any(type(v)is not str for v in m):raise CertificateError('NonCanonicalPolynomial')
            seen.add(m)
    @staticmethod
    def make(d):
        z={}
        for m,c in d.items():
            k=tuple(sorted(m));z[k]=z.get(k,Fraction(0))+Fraction(c)
        return Poly(tuple(sorted((m,c) for m,c in z.items() if c)))
    @staticmethod
    def c(c):return Poly.make({():Fraction(c)})
    @staticmethod
    def var(v):return Poly.make({(v,):1})
    def __add__(self,b):
        if not isinstance(b,Poly):b=Poly.c(b)
        d=dict(self.terms)
        for m,c in b.terms:d[m]=d.get(m,Fraction(0))+c
        return Poly.make(d)
    def __neg__(self):return Poly.make({m:-c for m,c in self.terms})
    def __sub__(self,b):return self+(-b if isinstance(b,Poly) else -Poly.c(b))
    def __mul__(self,b):
        if not isinstance(b,Poly):b=Poly.c(b)
        d={}
        for m,c in self.terms:
            for n,e in b.terms:
                k=tuple(sorted(m+n));d[k]=d.get(k,Fraction(0))+c*e
        return Poly.make(d)
    def value(self,env):
        out=Fraction(0)
        for m,c in self.terms:
            for v in m:c*=env[v]
            out+=c
        return out
    def data(self):return [[list(m),str(c)] for m,c in self.terms]
ZERO=Poly.c(0)
@dataclass(frozen=True)
class Certificate:
    rule:str
    args:tuple

def C(rule,*args):return Certificate(rule,args)

def check(cert,ge,eq=(),budget=10000):
    """Return a polynomial proved nonnegative, with a total node budget."""
    left=[budget]
    def rec(c):
        left[0]-=1
        if left[0]<0:raise CertificateError('CertificateBudget')
        if not isinstance(c,Certificate):raise CertificateError('MalformedCertificate')
        r,a=c.rule,c.args
        if r=='nonnegative_constant' and len(a)==1:
            q=Fraction(a[0])
            if q<0:raise CertificateError('NegativeConstant')
            p=Poly.c(q)
        elif r=='premise' and len(a)==1 and type(a[0])is int and 0<=a[0]<len(ge):p=ge[a[0]]
        elif r=='square' and len(a)==1 and isinstance(a[0],Poly):p=a[0]*a[0]
        elif r=='sum' and len(a)==2:p=rec(a[0])+rec(a[1])
        elif r=='product' and len(a)==2:p=rec(a[0])*rec(a[1])
        elif r=='equality_multiple' and len(a)==2 and type(a[0])is int and 0<=a[0]<len(eq) and isinstance(a[1],Poly):p=eq[a[0]]*a[1]
        else:raise CertificateError('UnsupportedRuleOrPremise')
        # A research operational bound, not a completeness claim over all proofs.
        if len(p.terms)>4096 or any(len(m)>128 or c.numerator.bit_length()>16384 or c.denominator.bit_length()>16384 for m,c in p.terms):raise CertificateError('PolynomialBudget')
        return p
    try:return rec(cert)
    except (RecursionError,ZeroDivisionError,TypeError) as exc:raise CertificateError('MalformedOrOversizedCertificate') from exc

def verify(cert,goal,ge,eq=()):
    if check(cert,tuple(ge),tuple(eq))!=goal:raise CertificateError('WrongConclusion')
    return True

def search_linear_certificate(goal,ge):
    """Untrusted incomplete search. Unknown stays Unknown, never admission."""
    for i,p in enumerate(ge):
        d=goal-p
        if len(d.terms)<=1 and (not d.terms or d.terms[0][0]==()) and (not d.terms or d.terms[0][1]>=0):
            c=C('premise',i)
            return C('sum',c,C('nonnegative_constant',str(d.terms[0][1]))) if d.terms else c
    if len(goal.terms)<=1 and (not goal.terms or goal.terms[0][0]==()) and (not goal.terms or goal.terms[0][1]>=0):return C('nonnegative_constant',str(goal.terms[0][1]) if goal.terms else '0')
    if len(ge)>8:raise CertificateError('SearchUnknown')
    for weights in product(range(3),repeat=len(ge)):
        if not any(weights):continue
        p=ZERO;cs=[]
        for i,w in enumerate(weights):
            if w:p=p+ge[i]*w;cs.append(C('product',C('nonnegative_constant',str(w)),C('premise',i)))
        if p==goal:
            c=cs[0]
            for x in cs[1:]:c=C('sum',c,x)
            return c
    raise CertificateError('SearchUnknown')

def polynomial(e,env):
    if isinstance(e,ast.Constant) and type(e.value)is int:return Poly.c(e.value)
    if isinstance(e,ast.Name) and e.id in env:return env[e.id]
    if isinstance(e,ast.UnaryOp) and isinstance(e.op,ast.USub):return -polynomial(e.operand,env)
    if isinstance(e,ast.BinOp):
        a,b=polynomial(e.left,env),polynomial(e.right,env)
        if isinstance(e.op,ast.Add):return a+b
        if isinstance(e.op,ast.Sub):return a-b
        if isinstance(e.op,ast.Mult):return a*b
    raise CertificateError('OutsidePolynomialProfile')

def condition(e,env,truth):
    if not isinstance(e,ast.Compare) or len(e.ops)!=1:raise CertificateError('UnsupportedGuard')
    a,b=polynomial(e.left,env),polynomial(e.comparators[0],env)
    if isinstance(e.ops[0],ast.Lt):return b-a-1 if truth else a-b
    if isinstance(e.ops[0],ast.LtE):return b-a if truth else a-b-1
    raise CertificateError('UnsupportedGuard')

def substitute_poly(p,env):
    out=ZERO
    for mon,coef in p.terms:
        t=Poly.c(coef)
        for v in mon:
            if v not in env:raise CertificateError('UnboundPolynomialVariable:'+v)
            t=t*env[v]
        out=out+t
    return out

@dataclass
class SymbolicPath:
    env:dict
    ge:list
    result:Poly|None=None

def symbolic_paths(op,invariants,initial_ge=(),guard_checks=None):
    statevars=set(op.reads)|set(op.writes)|{v for p in invariants.values() for m,_ in p.terms for v in m}
    base={n:Poly.var('old.'+n) for n in statevars};base.update({n:Poly.var('arg.'+n) for n,_ in op.sig.params})
    ge=[substitute_poly(p,base) for p in invariants.values()]+list(initial_ge)
    def go(stmts,path):
        paths=[path]
        for s in stmts:
            out=[]
            for p in paths:
                if p.result is not None:out.append(p);continue
                q=SymbolicPath(dict(p.env),list(p.ge))
                if isinstance(s,ast.Return):q.result=polynomial(s.value,q.env);out.append(q)
                elif isinstance(s,(ast.Assign,ast.AnnAssign)):
                    n=(s.targets[0] if isinstance(s,ast.Assign) else s.target).id;q.env[n]=polynomial(s.value,q.env);out.append(q)
                elif isinstance(s,ast.Assert):
                    goal=condition(s.test,q.env,True)
                    if guard_checks is not None:guard_checks.append((tuple(q.ge),goal))
                    q.ge.append(goal);out.append(q)
                elif isinstance(s,ast.If):
                    out+=go(s.body,SymbolicPath(dict(q.env),q.ge+[condition(s.test,q.env,True)]))
                    out+=go(s.orelse,SymbolicPath(dict(q.env),q.ge+[condition(s.test,q.env,False)]))
                elif isinstance(s,ast.Pass):out.append(q)
                else:raise CertificateError('UnsupportedCertificateStatement')
            paths=out
            if len(paths)>1024:raise CertificateError('PathBudget')
        return paths
    if op.kind!='named':raise CertificateError('NamedStraightLineOperationRequired')
    return go(op.body or [],SymbolicPath(base,ge))

@dataclass(frozen=True)
class ExportEvidence:
    operation_hash:str
    contract_hash:str
    invariant_hash:str
    proofs:tuple[tuple[Certificate,...],...]

def contract_goals(path,invariants,post):
    if path.result is None:raise CertificateError('IncompleteSuccessPath')
    goals=[substitute_poly(p,path.env) for p in invariants.values()]
    if post is not None:goals.append(substitute_poly(post,{**path.env,'result':path.result}))
    return goals

def contract_identity(invariants,post):return digest([[(n,p.data()) for n,p in sorted(invariants.items())],post.data() if post else None])
def invhash(invs):return digest([(n,p.data()) for n,p in sorted(invs.items())])
def produce_evidence(op,invariants,post=None):
    paths=symbolic_paths(op,invariants)
    proofs=tuple(tuple(search_linear_certificate(g,p.ge) for g in contract_goals(p,invariants,post)) for p in paths)
    return ExportEvidence(op.identity,contract_identity(invariants,post),invhash(invariants),proofs)

def verify_export(op,invariants,post,evidence):
    if not isinstance(evidence,ExportEvidence) or evidence.operation_hash!=op.identity or evidence.contract_hash!=contract_identity(invariants,post) or evidence.invariant_hash!=invhash(invariants):raise CertificateError('EvidenceBindingMismatch')
    paths=symbolic_paths(op,invariants)
    if len(paths)!=len(evidence.proofs):raise CertificateError('MissingPath')
    for p,certs in zip(paths,evidence.proofs):
        goals=contract_goals(p,invariants,post)
        if len(goals)!=len(certs):raise CertificateError('MissingGoal')
        for c,g in zip(certs,goals):verify(c,g,p.ge)
    return True

class ProtectedRegistry:
    def __init__(self,state,invariants):
        self.state=dict(state);self.invariants=dict(invariants);self.operations={}
        if any(p.value(state)<0 for p in invariants.values()):raise CertificateError('InitialInvariantViolation')
    def admit(self,op,evidence,post=None):
        verify_export(op,self.invariants,post,evidence);self.operations[op.sig.name]=(op,evidence,post)
    def invoke(self,name,args,permission=False):
        if not permission:raise DomainFailure('AuthorityDenied')
        op,ev,post=self.operations[name];verify_export(op,self.invariants,post,ev)
        new,v=op.evaluate(self.state,args);self.state=new;return v

@dataclass(frozen=True)
class BoundaryLink:
    client_identity:str
    producer_identity:str
    consumer_identity:str
    variable:str
    established_requirement:Poly
    consequence:Certificate

def prove_call_chain(client,task_name,producer,producer_evidence,producer_post,
                     consumer,consumer_requirement,consequence):
    """A source-checked immutable-value bridge, not a verifier-granted capability.
    Exact straight-line shape: x = producer(...); return consumer(x).
    The producer success theorem is independently checked. Other consumer guards,
    notably current state/authority, remain dynamic and are NOT discharged.
    """
    verify_export(producer,{},producer_post,producer_evidence)
    from .language import parse_check_compile
    client=parse_check_compile(client.source,client.name,{s.name:s for s in client.imports.values()})
    f=client.functions[task_name];body=f.node.body
    if len(body)!=2 or not isinstance(body[0],(ast.Assign,ast.AnnAssign)) or not isinstance(body[1],ast.Return):raise CertificateError('UnsupportedBridgeShape')
    assign=body[0];dst=assign.targets[0] if isinstance(assign,ast.Assign) else assign.target
    a,b=assign.value,body[1].value
    if not isinstance(dst,ast.Name) or not isinstance(a,ast.Call) or not isinstance(a.func,ast.Name) or not isinstance(b,ast.Call) or not isinstance(b.func,ast.Name) or len(b.args)!=1 or not isinstance(b.args[0],ast.Name) or b.args[0].id!=dst.id or len(consumer.sig.params)!=1:raise CertificateError('BridgeDataflowMismatch')
    ps=client.imports.get(a.func.id);cs=client.imports.get(b.func.id)
    if ps is None or cs is None or ps.identity!=producer.sig.identity or cs.identity!=consumer.sig.identity:raise CertificateError('BridgeInterfaceMismatch')
    goal=substitute_poly(consumer_requirement,{consumer.sig.params[0][0]:Poly.var('result')})
    verify(consequence,goal,[producer_post])
    return BoundaryLink(client.identity,producer.identity,consumer.identity,dst.id,consumer_requirement,consequence)

# Untrusted proof producers cross a data-only, bounded serialized seam.
def poly_from_data(d):
    if not isinstance(d,list) or len(d)>4096:raise CertificateError('MalformedPolynomial')
    out={}
    for row in d:
        if not isinstance(row,list) or len(row)!=2:raise CertificateError('MalformedPolynomial')
        m,c=row
        if not isinstance(m,list) or len(m)>128 or any(type(v)is not str or len(v)>256 for v in m) or type(c)is not str or len(c)>5000:raise CertificateError('MalformedPolynomial')
        k=tuple(m)
        if k in out or k!=tuple(sorted(k)):raise CertificateError('NonCanonicalPolynomial')
        try:q=Fraction(c)
        except (ValueError,ZeroDivisionError):raise CertificateError('MalformedCoefficient')
        if not q:raise CertificateError('NonCanonicalPolynomial')
        out[k]=q
    p=Poly.make(out)
    if p.data()!=d:raise CertificateError('NonCanonicalPolynomial')
    return p

def cert_data(c):
    if c.rule=='square':return [c.rule,c.args[0].data()]
    if c.rule=='equality_multiple':return [c.rule,c.args[0],c.args[1].data()]
    if c.rule in ('sum','product'):return [c.rule,cert_data(c.args[0]),cert_data(c.args[1])]
    return [c.rule,*c.args]

def cert_from_data(d,budget=None):
    if budget is None:budget=[10000]
    budget[0]-=1
    if budget[0]<0 or not isinstance(d,list) or not d or type(d[0])is not str:raise CertificateError('MalformedCertificate')
    r=d[0]
    if r=='square' and len(d)==2:return C(r,poly_from_data(d[1]))
    if r=='equality_multiple' and len(d)==3 and type(d[1])is int:return C(r,d[1],poly_from_data(d[2]))
    if r in ('sum','product') and len(d)==3:return C(r,cert_from_data(d[1],budget),cert_from_data(d[2],budget))
    if r=='premise' and len(d)==2 and type(d[1])is int:return C(r,d[1])
    if r=='nonnegative_constant' and len(d)==2 and type(d[1])is str and len(d[1])<5000:
        try:q=Fraction(d[1])
        except (ValueError,ZeroDivisionError):raise CertificateError('MalformedCoefficient')
        if str(q)!=d[1] or q<0:raise CertificateError('NonCanonicalCoefficient')
        return C(r,d[1])
    raise CertificateError('UnsupportedCertificateRule')

def encode_evidence(ev):
    import json
    return json.dumps({'operation':ev.operation_hash,'contract':ev.contract_hash,'invariants':ev.invariant_hash,'proofs':[[cert_data(c) for c in path] for path in ev.proofs]},sort_keys=True,separators=(',',':')).encode()

def decode_evidence(data):
    import json
    if type(data)is not bytes or len(data)>1024*1024:raise CertificateError('EvidenceSize')
    def pairs(items):
        out={}
        for k,v in items:
            if k in out:raise CertificateError('DuplicateField')
            out[k]=v
        return out
    try:
        d=json.loads(data,object_pairs_hook=pairs)
        if not isinstance(d,dict) or set(d)!={'operation','contract','invariants','proofs'}:raise CertificateError('EvidenceFields')
        for k in ('operation','contract','invariants'):
            if type(d[k])is not str or len(d[k])!=64 or any(c not in '0123456789abcdef' for c in d[k]):raise CertificateError('InvalidIdentity')
        if not isinstance(d['proofs'],list) or len(d['proofs'])>1024:raise CertificateError('ProofPaths')
        budget=[10000];paths=[]
        for path in d['proofs']:
            if not isinstance(path,list):raise CertificateError('ProofPath')
            paths.append(tuple(cert_from_data(c,budget) for c in path))
        return ExportEvidence(d['operation'],d['contract'],d['invariants'],tuple(paths))
    except (UnicodeError,ValueError,TypeError,RecursionError) as exc:
        if isinstance(exc,CertificateError):raise
        raise CertificateError('MalformedEvidence') from exc

@dataclass(frozen=True)
class TotalEvidence:
    operation_hash:str
    contract_hash:str
    invariant_hash:str
    preconditions:tuple[Poly,...]
    guard_proofs:tuple[Certificate,...]
    proofs:tuple[tuple[Certificate,...],...]

def total_obligations(op,invariants,post,preconditions):
    statevars=set(op.reads)|set(op.writes)|{v for p in invariants.values() for m,_ in p.terms for v in m}
    env={n:Poly.var('old.'+n) for n in statevars};env.update({n:Poly.var('arg.'+n) for n,_ in op.sig.params})
    pres=tuple(substitute_poly(p,env) for p in preconditions)
    guards=[];paths=symbolic_paths(op,invariants,pres,guards)
    return guards,paths

def total_identity(invariants,post,preconditions):return digest(['total-owner-body',contract_identity(invariants,post),[p.data() for p in preconditions]])

def produce_total_evidence(op,invariants,post,preconditions=()):
    guards,paths=total_obligations(op,invariants,post,preconditions)
    gs=tuple(search_linear_certificate(g,ge) for ge,g in guards)
    ps=tuple(tuple(search_linear_certificate(g,p.ge) for g in contract_goals(p,invariants,post)) for p in paths)
    return TotalEvidence(op.identity,total_identity(invariants,post,preconditions),invhash(invariants),tuple(preconditions),gs,ps)

def verify_total(op,invariants,post,expected_pre,evidence):
    if not isinstance(evidence,TotalEvidence) or evidence.operation_hash!=op.identity or evidence.contract_hash!=total_identity(invariants,post,expected_pre) or evidence.invariant_hash!=invhash(invariants) or evidence.preconditions!=tuple(expected_pre):raise CertificateError('TotalEvidenceBindingMismatch')
    guards,paths=total_obligations(op,invariants,post,expected_pre)
    if len(guards)!=len(evidence.guard_proofs) or len(paths)!=len(evidence.proofs):raise CertificateError('MissingTotalityObligation')
    for (ge,g),c in zip(guards,evidence.guard_proofs):verify(c,g,ge)
    for p,cs in zip(paths,evidence.proofs):
        goals=contract_goals(p,invariants,post)
        if len(goals)!=len(cs):raise CertificateError('MissingGoal')
        for g,c in zip(goals,cs):verify(c,g,p.ge)
    return True

def verify_contract(op,invariants,post,evidence):
    if isinstance(evidence,TotalEvidence):return verify_total(op,invariants,post,evidence.preconditions,evidence)
    return verify_export(op,invariants,post,evidence)
