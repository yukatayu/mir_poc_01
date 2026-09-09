"""F0.3 integrated semantic control reference, NOT a production runtime.

Trusted root commands include admin_* plus supervisor checkpoint/recover.
These and present() are host-internal APIs, not guest-visible endpoints. End-user requests
can only be generated from a checked task request instruction. Guest Python is
never evaluated. Source contracts reuse the *unchanged* F0.2 arithmetic checker.
All mutations below are abstract atomic transitions. Distributed realization is
an obligation, not supplied by Python's execution or deepcopy.
"""
from __future__ import annotations
from dataclasses import dataclass, field, fields, is_dataclass, replace
from copy import deepcopy
import ast, hashlib, json
from typing import Any
from .support import Formula, Top, Ref, All, derive, deps, dag_order, check_closure, Invalid
from .policy import Authority, Context, Policy, PolicyEvidence, Claim
from .identity import exact
from base.language import parse_check_compile, Program, Operation, DomainFailure, tyval, binop
from base.certificates import Poly, TotalEvidence, verify_total, verify_contract


def canonical(x):
    if x is None or type(x) in (int,str,bool): return x
    if isinstance(x, ast.AST): return {'AST':ast.dump(x,include_attributes=False)}
    if is_dataclass(x): return {'type':type(x).__name__,'fields':{f.name:canonical(getattr(x,f.name)) for f in fields(x)}}
    if isinstance(x, Authority): return {'Authority':canonical(x.__dict__)}
    if isinstance(x,dict):
        pairs=[(canonical(k),canonical(v)) for k,v in x.items()]
        return {'map':sorted(pairs,key=lambda p:json.dumps(p[0],sort_keys=True))}
    if isinstance(x,(set,frozenset)):
        return {'set':sorted([canonical(v) for v in x],key=lambda v:json.dumps(v,sort_keys=True))}
    if isinstance(x,tuple): return {'tuple':[canonical(v) for v in x]}
    if isinstance(x,list): return [canonical(v) for v in x]
    # Fraction in polynomial certificates, no arbitrary object serialization.
    from fractions import Fraction
    if isinstance(x,Fraction): return {'rational':[x.numerator,x.denominator]}
    raise Invalid('UnsupportedCheckpointComponent:'+type(x).__name__)


def fingerprint(x):
    return hashlib.sha256(json.dumps(canonical(x),sort_keys=True,separators=(',',':')).encode()).hexdigest()


@dataclass(frozen=True)
class Node:
    key:str
    kind:str
    label:int
    support:Formula
    enabled:bool=True
    revision:int=0


@dataclass(frozen=True)
class Access:
    node:str
    anchor:str
    authorization:PolicyEvidence


@dataclass
class Binding:
    node:str
    candidates:tuple[str,...]
    cursor:int=0
    lineage:int=0
    policy:str=''


@dataclass(frozen=True)
class RelationExpr:
    tag:str
    args:tuple


def relation_refs(e):
    if not isinstance(e,RelationExpr):raise Invalid('RelationExpressionShape')
    if e.tag=='const' and len(e.args)==1 and type(e.args[0]) is int:return frozenset()
    if e.tag=='ref' and len(e.args)==1 and type(e.args[0]) is str:return frozenset(e.args)
    if e.tag=='add' and len(e.args)==2:return relation_refs(e.args[0])|relation_refs(e.args[1])
    if e.tag=='scale' and len(e.args)==2 and type(e.args[0]) is int:return relation_refs(e.args[1])
    raise Invalid('UnsupportedRelationExpression')


@dataclass(frozen=True)
class Sample:
    node:str
    lineage:int
    presentation_context:str
    value:int


@dataclass
class CellState:
    node:str
    program:str
    name:str
    ty:str
    value:Any
    version:int=0


@dataclass
class BoundOperation:
    operation:Operation
    module:str
    locus:str
    cells:dict[str,str]
    policy:str
    label:int
    invariants:dict
    evidence:Any
    source_identity:str
    @property
    def assurance(self):return 'TotalBody' if isinstance(self.evidence,TotalEvidence) else 'Partial' if self.evidence is not None else 'TypeOnly'
    @property
    def contract(self):return fingerprint([self.operation.sig.identity,self.invariants,self.assurance])
    @property
    def code(self):return fingerprint([self.operation.identity,self.source_identity,self.module,self.cells,self.contract])


@dataclass
class Request:
    context:Context
    operation:str
    evidence:PolicyEvidence
    dependencies:tuple[str,...]
    label:int
    site:tuple[str,str,int]
    status:str='Pending'
    result:Any=None
    failure:str|None=None


@dataclass
class Activity:
    name:str
    principal:str
    member:str
    program:str
    function:str
    env:dict
    supplied:tuple[Claim,...]
    label:int=0
    pc:int=0
    stack:list=field(default_factory=list)
    waiting:tuple[str,str]|None=None
    result:Any=None
    failure:str|None=None
    done:bool=False


@dataclass(frozen=True)
class Observation:
    label:int
    kind:str
    payload:tuple


@dataclass(frozen=True)
class PreparedReparent:
    identity:str
    principal:str
    member:str
    target:str
    support:Formula
    tracked:tuple[tuple[str,int],...]
    authorization:PolicyEvidence
    label:int


@dataclass
class Data:
    instance:str
    nodes:dict[str,Node]=field(default_factory=dict)
    live:frozenset[str]=frozenset()
    rank:dict[str,int]=field(default_factory=dict)
    names:dict=field(default_factory=dict)
    memberships:dict[str,str]=field(default_factory=dict)
    cells:dict[str,CellState]=field(default_factory=dict)
    programs:dict[str,Program]=field(default_factory=dict)
    modules:dict[str,str]=field(default_factory=dict)
    locus_bindings:dict[str,dict[str,str]]=field(default_factory=dict)
    operations:dict[str,BoundOperation]=field(default_factory=dict)
    accesses:dict[str,Access]=field(default_factory=dict)
    bindings:dict[str,Binding]=field(default_factory=dict)
    relations:dict[str,RelationExpr]=field(default_factory=dict)
    preparations:dict[str,PreparedReparent]=field(default_factory=dict)
    requests:dict[str,Request]=field(default_factory=dict)
    activities:dict[str,Activity]=field(default_factory=dict)
    counters:dict=field(default_factory=dict)
    auth:Authority=field(default_factory=Authority)
    events:list[Observation]=field(default_factory=list)
    observer_grants:dict[str,tuple[str,int]]=field(default_factory=dict)


@dataclass(frozen=True)
class Checkpoint:
    instance:str
    position:int
    digest:str
    components:tuple[str,...]
    image:Data


COMPONENTS=tuple(f.name for f in fields(Data))


class Kernel:
    def __init__(self,instance='instance-1'):
        self.data=Data(instance)
        self.journal=[]   # trusted atomic prefix, containing complete images
        self.snapshots={}
        self.observers={} # intentionally outside authoritative state
        self._append()

    def _append(self):
        previous=self.journal[-1][0] if self.journal else 'genesis'
        image=deepcopy(self.data)
        digest=fingerprint([previous,image])
        self.journal.append((digest,previous,image))

    @staticmethod
    def _eligible(d:Data):
        enabled={k for k,n in d.nodes.items() if n.enabled}
        for k,access in d.accesses.items():
            ev=access.authorization
            try:
                if d.memberships.get(ev.context.principal)!=ev.context.member:raise Invalid('StaleAccessMember')
                if access.anchor not in d.nodes:raise Invalid('AccessTargetMissing')
                current=replace(ev.context,code=str(d.nodes[access.anchor].revision))
                d.auth.revalidate(ev,current)
            except Invalid:enabled.discard(k)
        return frozenset(enabled)

    @staticmethod
    def _normalize(d:Data):
        d.live,d.rank=derive({k:n.support for k,n in d.nodes.items()},Kernel._eligible(d))
        # Semantic invalidation only; sample loss is not an input.
        for b in d.bindings.values():
            while b.cursor<len(b.candidates) and b.candidates[b.cursor] not in d.live:b.cursor+=1

    @staticmethod
    def _wf(d:Data):
        if set(vars(d))!=set(COMPONENTS):raise Invalid('UndeclaredStateComponent')
        check_closure({k:n.support for k,n in d.nodes.items()},Kernel._eligible(d),d.live,d.rank)
        live=d.live
        for k,n in d.nodes.items():
            if n.key!=k or type(n.label)is not int or n.label<0:raise Invalid('NodeShape')
            if any(d.nodes[p].label>n.label for p in deps(n.support)):raise Invalid('HighSupportToLow')
        # The structural graphs of each declared kind are DAGs. Cross-kind
        # justification is the least fixed point, never coinductive self-trust.
        for kind in {n.kind for n in d.nodes.values()}:
            ns={k for k,n in d.nodes.items() if n.kind==kind}
            es={(p,k) for k in ns for p in deps(d.nodes[k].support) if p in ns}
            dag_order(ns,es)
        for p,m in d.memberships.items():
            if m not in d.nodes or d.nodes[m].kind!='member':raise Invalid('MembershipShape')
        for key,access in d.accesses.items():
            if key!=access.node or access.anchor not in d.nodes or d.nodes[key].kind!='access':raise Invalid('AccessShape')
            ev=access.authorization
            if ev.context.instance!=d.instance or ev.context.target!=access.anchor or ev.context.action!='anchor_use':raise Invalid('AccessContext')
            if d.auth.policies[ev.policy].label>d.nodes[key].label:raise Invalid('AccessPolicyFlow')
        for b in d.bindings.values():
            if b.node not in d.nodes or not b.candidates or b.cursor<0 or b.cursor>len(b.candidates):raise Invalid('BindingShape')
            if any(c not in d.nodes or d.nodes[c].label>d.nodes[b.node].label for c in b.candidates):raise Invalid('BindingInformationFlow')
            if b.cursor<len(b.candidates) and b.candidates[b.cursor] not in live:raise Invalid('BindingNotNormalized')
        relation_keys=set(d.bindings)|set(d.relations)
        relation_edges=set()
        for key,expr in d.relations.items():
            refs=relation_refs(expr)
            if not refs<=relation_keys or key not in d.nodes:raise Invalid('UnknownRelationDependency')
            if any(d.nodes[r].label>d.nodes[key].label for r in refs):raise Invalid('RelationInformationFlow')
            relation_edges.update((r,key) for r in refs)
        dag_order(relation_keys,relation_edges)
        for c in d.cells.values():
            if c.node not in d.nodes or d.nodes[c.node].kind!='state' or tyval(c.value)!=c.ty:raise Invalid('CellShape')
        for q,b in d.operations.items():
            if q!=b.operation.sig.name or b.module not in d.nodes or b.locus not in d.nodes:raise Invalid('OperationShape')
            if b.policy not in d.auth.policies or d.auth.policies[b.policy].label>b.label:raise Invalid('OperationPolicyFlow')
            if d.nodes[b.module].label>b.label or d.nodes[b.locus].label>b.label:raise Invalid('OperationSupportFlow')
            for name in b.operation.reads|b.operation.writes:
                if name not in b.cells or b.cells[name] not in d.cells:raise Invalid('OperationCellBinding')
            if any(d.nodes[b.cells[r]].label>b.label for r in b.operation.reads|b.operation.writes):raise Invalid('OperationDependencyLabel')
            if any(d.nodes[b.cells[w]].label<b.label for w in b.operation.writes):raise Invalid('WriteDown')
            if b.invariants:
                state={n:d.cells[k].value for n,k in b.cells.items()}
                if any(poly.value(state)<0 for poly in b.invariants.values()):raise Invalid('ProtectedInvariant')
        for pid,prep in d.preparations.items():
            if prep.identity!=pid or prep.authorization.context.instance!=d.instance or prep.target not in d.nodes:raise Invalid('PreparationBinding')
        for key,r in d.requests.items():
            if r.context.request!=key:raise Invalid('RequestIdentity')
            if r.status not in ('Pending','Served','Rejected','Consumed','ReleaseDenied'):raise Invalid('RequestState')
            if r.status == 'Served' and r.failure is not None:raise Invalid('ResultFailureConfusion')
        return True

    def _atomic(self,fn):
        candidate=deepcopy(self.data)
        result=fn(candidate)
        self._normalize(candidate)
        for key in sorted(self.data.live ^ candidate.live):
            node=candidate.nodes[key]
            self._event(candidate,node.label,'BecameLive' if key in candidate.live else 'BecameInactive',key)
        for key,b in sorted(candidate.bindings.items()):
            old=self.data.bindings.get(key)
            if old is not None and b.lineage==old.lineage and b.cursor!=old.cursor:
                self._event(candidate,candidate.nodes[key].label,'SemanticFallback',key,b.lineage,old.cursor,b.cursor)
        self._wf(candidate)
        self.data=candidate;self._append()
        return result

    @staticmethod
    def _event(d,label,kind,*payload):d.events.append(Observation(label,kind,tuple(payload)))
    @staticmethod
    def _new_node(d,kind,name,label,support):
        if kind not in ('locus','member','module','state','relation','anchor','access','theory'):raise Invalid('UnknownNodeKind')
        if type(label)is not int or label<0:raise Invalid('InvalidLabel')
        k=(label,name);n=d.names.get(k,0)+1;d.names[k]=n
        key=f'{label}:{name}@{n}'
        d.nodes[key]=Node(key,kind,label,support)
        return key

    def admin_node(self,kind,name,label=0,support=None,principal=None):
        def go(d):
            k=self._new_node(d,kind,name,label,Top() if support is None else support)
            if principal is not None:
                if kind!='member':raise Invalid('NotMembership')
                old=d.memberships.get(principal)
                if old is not None:
                    prior=d.nodes[old];d.nodes[old]=replace(prior,enabled=False,revision=prior.revision+1)
                d.memberships[principal]=k
            self._event(d,label,'Declare',k,kind)
            return k
        return self._atomic(go)

    def admin_issuer(self,key):return self._atomic(lambda d:d.auth.add_issuer(key))
    def admin_policy(self,policy):
        def go(d):
            old=d.auth.policies.get(policy.key)
            if old and old.label!=policy.label:raise Invalid('NoSilentPolicyRelabel')
            d.auth.install_policy(policy)
            self._event(d,policy.label,'PolicyRevision',policy.key,policy.version)
        return self._atomic(go)
    def admin_claim(self,issuer,principal,predicate,actions,targets,label=0):
        def go(d):
            if principal not in d.memberships or d.memberships[principal] not in d.live:raise Invalid('PrincipalNotLive')
            return d.auth.issue(issuer,principal,d.memberships[principal],predicate,actions,targets,label)
        return self._atomic(go)
    def admin_revoke(self,claim):
        def go(d):
            if not exact(d.auth.claims.get(claim.key),claim):raise Invalid('UnknownClaim')
            d.auth.revoked.add(claim.key);self._event(d,claim.label,'Revoke',claim.key)
        return self._atomic(go)
    def admin_retire(self,key):
        def go(d):
            if key not in d.nodes:raise Invalid('UnknownNode')
            old=d.nodes[key];d.nodes[key]=replace(old,enabled=False,revision=old.revision+1)
            self._event(d,old.label,'Retire',key)
        return self._atomic(go)
    def admin_reparent(self,key,support):
        def go(d):
            n=d.nodes[key]
            if not n.enabled:raise Invalid('RetiredNodeNeedsFreshIncarnation')
            d.nodes[key]=replace(n,support=support,revision=n.revision+1)
            self._event(d,n.label,'Reparent',key)
        return self._atomic(go)

    def prepare_reparent(self,principal,target,support,policy,claims):
        """A checked source-level lifecycle intent. No raw node delta is exposed.
        The dependency scan includes all branches, not only the current OR witness.
        An authorization decision is retained, but grants no future-use reservation.
        """
        def go(d):
            n=d.nodes[target];member=d.memberships.get(principal)
            if member not in d.live or target not in d.live:raise Invalid('PreparationTargetNotLive')
            if d.nodes[member].label>n.label or d.auth.policies[policy].label>n.label:raise Invalid('ControlInformationFlow')
            # Exact immutable incarnation keys plus revision stamps. New links
            # into a visited chain must revise a node that has been read.
            tracked={target:n.revision};todo=list(deps(support));seen=set()
            for p in deps(support):
                if p not in d.nodes:raise Invalid('UnknownParent')
                if d.nodes[p].label>n.label:raise Invalid('HighSupportToLow')
            while todo:
                x=todo.pop()
                if x==target:raise Invalid('Cycle')
                if x in seen:continue
                seen.add(x);parent=d.nodes[x]
                if parent.kind!=n.kind:continue
                tracked[x]=parent.revision
                todo.extend(p for p in deps(parent.support) if d.nodes[p].kind==n.kind)
            ctr=(principal,n.label,'patch');j=d.counters.get(ctr,0)+1
            pid=f'{n.label}:{principal}:patch:{j}'
            ctx=Context(principal,member,'reparent',target,d.instance,pid,(),fingerprint(support),'reparent-same-kind-dag/v1')
            auth=d.auth.evaluate(policy,ctx,tuple(claims))
            patch=PreparedReparent(pid,principal,member,target,support,tuple(sorted(tracked.items())),auth,n.label)
            d.preparations[pid]=patch;d.counters[ctr]=j
            self._event(d,n.label,'PrepareReparent',pid,target)
            return patch
        return self._atomic(go)

    def commit_reparent(self,patch,principal):
        def go(d):
            registered=d.preparations.get(patch.identity)
            if registered is None or not exact(registered,patch):raise Invalid('UnboundPatch')
            if principal!=patch.principal or d.memberships.get(principal)!=patch.member or patch.member not in d.live:raise Invalid('PatchPrincipalNotCurrent')
            if patch.target not in d.live:raise Invalid('PatchTargetNotCurrent')
            d.auth.revalidate(patch.authorization,patch.authorization.context)
            if any(k not in d.nodes or d.nodes[k].revision!=v for k,v in patch.tracked):raise Invalid('StalePreparation')
            old=d.nodes[patch.target]
            d.nodes[patch.target]=replace(old,support=patch.support,revision=old.revision+1)
            del d.preparations[patch.identity]
            self._event(d,patch.label,'CommitReparent',patch.identity,patch.target)
        return self._atomic(go)

    def admin_install_program(self,source,name,module,loci,labels,policies,contracts=None):
        """Owner-authorized *addition*. A final source/library syntax is not selected.
        Manifest only adds meaning absent from F0.2 source: lifetime, IFC, policy.
        It supplies no value results or new communication routes.
        """
        contracts=contracts or {}
        def go(d):
            if name in d.programs:raise Invalid('ProgramAlreadyInstalled')
            if module not in d.live or d.nodes[module].kind!='module':raise Invalid('ModuleNotLive')
            interfaces={q:b.operation.sig for q,b in d.operations.items()}
            program=parse_check_compile(source,name,interfaces)
            mapping={}
            for c in program.cells.values():
                if c.owner not in loci or loci[c.owner] not in d.live:raise Invalid('LocusNotLive')
                lab=labels[c.name]
                key=self._new_node(d,'state',name+'.'+c.name,lab,All(Ref(module),Ref(loci[c.owner])))
                mapping[c.name]=key;d.cells[key]=CellState(key,name,c.name,c.ty,c.initial)
            # Protect at module granularity in this profile. Every writer is covered.
            invariant_sets=[contracts[q][0] for q in contracts if q in program.operations]
            protected={n:poly for invs in invariant_sets for n,poly in invs.items()}
            for q,op in program.operations.items():
                policy=policies[q]
                if policy not in d.auth.policies:raise Invalid('UnknownPolicy')
                locus=loci[op.sig.owner]
                lab=max([d.auth.policies[policy].label,d.nodes[module].label,d.nodes[locus].label]+[d.nodes[mapping[r]].label for r in op.reads|op.writes])
                invs,ev=contracts.get(q,({},None))
                invvars={v for poly in invs.values() for monomial,_ in poly.terms for v in monomial}
                if any(v not in program.cells or program.cells[v].owner!=op.sig.owner or labels[v]>lab for v in invvars):raise Invalid('InvariantOutsideOwnerFootprint')
                if protected and op.writes:
                    if invs!=protected or ev is None:raise Invalid('UncertifiedMutator')
                if ev is not None:
                    if isinstance(ev,TotalEvidence):verify_total(op,invs,None,(),ev)
                    else:verify_contract(op,invs,None,ev)
                d.operations[q]=BoundOperation(op,module,locus,dict(mapping),policy,lab,dict(invs),deepcopy(ev),program.identity)
            d.programs[name]=program;d.modules[name]=module;d.locus_bindings[name]=dict(loci)
            self._event(d,d.nodes[module].label,'Install',name,module)
            return mapping
        return self._atomic(go)

    def admin_replace(self,name,source,contracts):
        """Quiescent, same-schema/same-signature exchange. Strict current authorization
        of later uses, no preparation-time grant reservation is assumed here.
        """
        def go(d):
            old=d.programs[name]
            if any(r.context.code==b.code and r.status=='Pending' for b in d.operations.values() if b.module==d.modules[name] for r in d.requests.values()):raise Invalid('PendingOperation')
            if any(a.program==name and not a.done for a in d.activities.values()):raise Invalid('LiveActivity')
            fresh=parse_check_compile(source,name,{q:b.operation.sig for q,b in d.operations.items()})
            if set(fresh.cells)!=set(old.cells) or any((fresh.cells[k].ty,fresh.cells[k].owner)!=(old.cells[k].ty,old.cells[k].owner) for k in old.cells):raise Invalid('SchemaChangeNeedsMigration')
            if set(fresh.operations)!=set(old.operations):raise Invalid('OperationSetChanged')
            for q,op in fresh.operations.items():
                b=d.operations[q]
                if op.sig.identity!=b.operation.sig.identity:raise Invalid('ContractChanged')
                ev=contracts.get(q)
                if b.invariants and ev is None:raise Invalid('MissingReplacementProof')
                if b.assurance=='TotalBody' and not isinstance(ev,TotalEvidence):raise Invalid('CannotSilentlyWeakenTotality')
                if ev is not None:
                    if isinstance(ev,TotalEvidence):verify_total(op,b.invariants,None,(),ev)
                    else:verify_contract(op,b.invariants,None,ev)
                d.operations[q]=replace(b,operation=op,evidence=deepcopy(ev),source_identity=fresh.identity)
            d.programs[name]=fresh
            self._event(d,d.nodes[d.modules[name]].label,'Replace',name)
        return self._atomic(go)

    def access_anchor(self,principal,anchor,policy,claims):
        def go(d):
            member=d.memberships.get(principal)
            if member not in d.live or anchor not in d.live or d.nodes[anchor].kind!='anchor':raise Invalid('AnchorNotLive')
            p=d.auth.policies[policy]
            label=max(p.label,d.nodes[member].label,d.nodes[anchor].label)
            node=self._new_node(d,'access','access.'+anchor+'.'+principal,label,All(Ref(anchor),Ref(member)))
            ctx=Context(principal,member,'anchor_use',anchor,d.instance,node,(),str(d.nodes[anchor].revision),'anchor-int/v1')
            ev=d.auth.evaluate(policy,ctx,tuple(claims))
            d.accesses[node]=Access(node,anchor,ev)
            self._event(d,label,'AccessAdmitted',node,anchor)
            return node
        return self._atomic(go)

    def admin_binding(self,name,module,candidates,policy,label=0):
        def go(d):
            if module not in d.live:raise Invalid('ModuleNotLive')
            if policy not in d.auth.policies or d.auth.policies[policy].label>label:raise Invalid('BindingPolicyFlow')
            if not candidates or any(c not in d.nodes for c in candidates):raise Invalid('InvalidCandidates')
            key=self._new_node(d,'relation',name,label,Ref(module))
            d.bindings[key]=Binding(key,tuple(candidates),policy=policy)
            self._event(d,label,'Bind',key)
            return key
        return self._atomic(go)

    def admin_relation(self,name,module,expression,label=0):
        def go(d):
            if module not in d.live:raise Invalid('RelationModuleNotLive')
            key=self._new_node(d,'relation',name,label,Ref(module))
            d.relations[key]=expression
            self._event(d,label,'DerivedRelation',key)
            return key
        return self._atomic(go)

    def admin_relation_update(self,key,expression):
        def go(d):
            if key not in d.relations or key not in d.live:raise Invalid('UnknownDerivedRelation')
            n=d.nodes[key];d.nodes[key]=replace(n,revision=n.revision+1)
            d.relations[key]=expression
            self._event(d,n.label,'DerivedRelationUpdate',key)
        return self._atomic(go)

    def project(self,principal,key,samples,presentation_context):
        """Authorized, read-only evaluation of a retained relation DAG. Labels of
        definitions and all potential alternatives dominate, not just used values.
        Observation storage and pure presentation do not authorize mutations.
        """
        d=self.data;grant=d.observer_grants.get(principal)
        if grant is None or grant[0] not in d.live or d.memberships.get(principal)!=grant[0]:raise Invalid('ObserverDenied')
        if key not in d.nodes or d.nodes[key].label>grant[1]:raise Invalid('ObservationDenied')
        memo={}
        def node(r):
            if r not in d.live:raise Invalid('SemanticUnavailable')
            if r in memo:return memo[r]
            if r in d.bindings:
                b=d.bindings[r]
                if b.cursor==len(b.candidates):raise Invalid('SemanticUnavailable')
                selected=b.candidates[b.cursor];anchor=d.accesses[selected].anchor if selected in d.accesses else selected;sample=samples.get(anchor)
                if not isinstance(sample,Sample) or (sample.node,sample.lineage,sample.presentation_context)!=(anchor,b.lineage,presentation_context) or type(sample.value)is not int:raise Invalid('PresentationGap')
                value=sample.value
            elif r in d.relations:value=expr(d.relations[r])
            else:raise Invalid('NotProjectableRelation')
            memo[r]=value;return value
        def expr(e):
            if e.tag=='const':return e.args[0]
            if e.tag=='ref':return node(e.args[0])
            if e.tag=='add':return expr(e.args[0])+expr(e.args[1])
            if e.tag=='scale':return e.args[0]*expr(e.args[1])
            raise Invalid('UnsupportedRelationExpression')
        return node(key)

    @staticmethod
    def _binding_context(d,principal,b,request,args=()):
        member=d.memberships.get(principal)
        if member not in d.live or b.node not in d.live:raise Invalid('InactiveBindingUse')
        return Context(principal,member,'reacquire',b.node,d.instance,request,tuple(args),str(b.lineage),fingerprint(b.candidates))
    def reacquire(self,principal,binding,claims,new_candidates=None):
        def go(d):
            b=d.bindings[binding]
            candidates=tuple(b.candidates if new_candidates is None else new_candidates)
            if not candidates or any(c not in d.nodes or d.nodes[c].kind not in ('anchor','access') for c in candidates):raise Invalid('InvalidReacquireCandidates')
            ctx=self._binding_context(d,principal,b,'reacquire:'+str(b.lineage+1),candidates)
            d.auth.evaluate(b.policy,ctx,claims)
            if not any(c in d.live for c in candidates):raise Invalid('NoLiveCandidate')
            b.candidates=candidates;b.cursor=0;b.lineage+=1
            self._event(d,d.nodes[b.node].label,'Reacquire',binding,b.lineage)
        return self._atomic(go)

    def present(self,binding,samples):
        """Read-only presentation query. Does not confer release authority to a
        guest. The view adapter is trusted; user-facing export is observe().
        samples is map exact node id -> (lineage,value); mismatches are gaps.
        """
        d=self.data;b=d.bindings[binding]
        if binding not in d.live or b.cursor==len(b.candidates):return ('Unavailable',None)
        selected=b.candidates[b.cursor];anchor=d.accesses[selected].anchor if selected in d.accesses else selected;sample=samples.get(anchor)
        if sample is None or sample[0]!=b.lineage:return ('PresentationGap',None)
        return ('Value',sample[1])

    def start(self,program,task,principal,args,claims,label=0):
        def go(d):
            if d.memberships.get(principal) not in d.live or d.modules[program] not in d.live:raise Invalid('ParticipantNotLive')
            p=d.programs[program];f=p.functions[task]
            locus=d.locus_bindings[program].get(f.sig.owner)
            if locus not in d.live:raise Invalid('ActivityLocusNotLive')
            if max(d.nodes[d.modules[program]].label,d.nodes[d.memberships[principal]].label,d.nodes[locus].label)>label:raise Invalid('ActivityContextLabel')
            if f.sig.kind!='task' or len(args)!=len(f.sig.params) or any(tyval(v)!=t for v,(_,t) in zip(args,f.sig.params)):raise Invalid('TaskSignature')
            # Whole-activity label is conservative. No runtime secret branch may
            # select a lower-labeled operation; all static call sites are checked.
            for ins in f.code:
                if ins[0]=='request' and d.operations[ins[2]].label<label:raise Invalid('SecretControlOfLowOperation')
            k=(principal,label,'activity');n=d.counters.get(k,0)+1;d.counters[k]=n
            aid=f'{label}:{principal}:activity:{n}'
            d.activities[aid]=Activity(aid,principal,d.memberships[principal],program,task,dict(zip((k for k,_ in f.sig.params),args)),tuple(claims),label)
            return aid
        return self._atomic(go)

    @staticmethod
    def _active_dependencies(d,b,member):
        roots=(member,b.module,b.locus)+tuple(b.cells[n] for n in sorted(b.operation.reads|b.operation.writes))
        if any(n not in d.live for n in roots):raise Invalid('DependencyNotLive')
        return roots
    def local_step(self,aid):
        def go(d):
            a=d.activities[aid]
            if a.done or a.waiting:raise Invalid('ActivityNotReady')
            if a.member not in d.live or d.memberships.get(a.principal)!=a.member or d.modules[a.program] not in d.live:raise Invalid('InactiveActivity')
            p=d.programs[a.program];f=p.functions[a.function]
            if not 0<=a.pc<len(f.code):raise Invalid('BadPC')
            ins=f.code[a.pc];tag=ins[0]
            if tag=='const':a.env[ins[1]]=ins[2];a.pc+=1
            elif tag=='move':a.env[ins[1]]=a.env[ins[2]];a.pc+=1
            elif tag=='unary':a.env[ins[1]]=-a.env[ins[3]] if ins[2]=='USub' else not a.env[ins[3]];a.pc+=1
            elif tag=='binary':a.env[ins[1]]=binop(ins[2],a.env[ins[3]],a.env[ins[4]]);a.pc+=1
            elif tag=='jump':a.pc=ins[1]
            elif tag=='branch':a.pc=ins[2] if a.env[ins[1]] else ins[3]
            elif tag=='nop':a.pc+=1
            elif tag=='call':
                _,dst,q,rs=ins;prog,fn=q.rsplit('.',1)
                if prog!=a.program:raise Invalid('OnlyLocalTaskCalls')
                nf=p.functions[fn]
                for other in nf.code:
                    if other[0]=='request' and d.operations[other[2]].label<a.label:raise Invalid('SecretControlOfLowOperation')
                vals=tuple(a.env[r] for r in rs)
                a.stack.append((a.function,a.pc+1,dst,a.env));a.function=fn;a.pc=0;a.env=dict(zip((n for n,_ in nf.sig.params),vals))
            elif tag=='return':
                value=a.env[ins[1]]
                if a.stack:
                    fn,pc,dst,env=a.stack.pop();a.function=fn;a.pc=pc;a.env=env;a.env[dst]=value
                else:a.result=value;a.done=True;self._event(d,a.label,'TaskReturn',aid,value)
            elif tag=='request':
                _,dst,q,rs=ins;b=d.operations[q]
                if b.label<a.label:raise Invalid('SecretControlOfLowOperation')
                roots=self._active_dependencies(d,b,a.member)+(d.modules[a.program],d.locus_bindings[a.program][f.sig.owner])
                args=tuple(a.env[r] for r in rs)
                counter=(a.principal,b.label,'request');n=d.counters.get(counter,0)+1
                rid=f'{b.label}:{a.principal}:request:{n}'
                ctx=Context(a.principal,a.member,q,b.module,d.instance,rid,args,b.code,b.contract)
                ev=d.auth.evaluate(b.policy,ctx,a.supplied)
                d.counters[counter]=n
                d.requests[rid]=Request(ctx,q,ev,roots,b.label,(a.program,a.function,a.pc))
                a.waiting=(rid,dst);a.pc+=1
                # Reading a higher result taints the continuation before it can run.
                a.label=max(a.label,b.label)
                self._event(d,b.label,'Request',rid,q)
                return rid
            else:raise Invalid('F03ContinuationProfileUnsupported:'+tag)
            return None
        return self._atomic(go)

    def serve(self,rid):
        def go(d):
            r=d.requests[rid]
            if r.status!='Pending':raise Invalid('DuplicateRequest')
            b=d.operations[r.operation]
            try:
                if r.context.instance!=d.instance or r.context.code!=b.code or r.context.contract!=b.contract:raise Invalid('StaleCode')
                if d.memberships.get(r.context.principal)!=r.context.member:raise Invalid('StaleMember')
                self._active_dependencies(d,b,r.context.member)
                if not set(r.dependencies)<=d.live:raise Invalid('SourceOrDependencyRetired')
                d.auth.revalidate(r.evidence,r.context)
                state={n:d.cells[k].value for n,k in b.cells.items()}
                new,value=b.operation.evaluate(state,r.context.arguments)
                for n in b.operation.writes:
                    cell=d.cells[b.cells[n]];cell.value=new[n];cell.version+=1
                r.status='Served';r.result=value
                self._event(d,r.label,'Served',rid,r.operation)
            except (Invalid,DomainFailure) as err:
                r.status='Rejected';r.failure=str(err)
                self._event(d,r.label,'Rejected',rid,r.failure)
            return r.status
        return self._atomic(go)

    def consume(self,aid):
        def go(d):
            a=d.activities[aid]
            if a.waiting is None:raise Invalid('NoPendingContinuation')
            rid,dst=a.waiting;r=d.requests[rid]
            if r.status not in ('Served','Rejected'):raise Invalid('NoRetainedOutcome')
            b=d.operations[r.operation]
            try:
                if a.member not in d.live or d.memberships.get(a.principal)!=a.member:raise Invalid('RetiredConsumer')
                if d.modules[a.program] not in d.live or not set(r.dependencies)<=d.live:raise Invalid('RetiredLiveResultContext')
                if b.code!=r.context.code:raise Invalid('StaleResultCode')
                d.auth.revalidate(r.evidence,r.context)
                if r.status=='Rejected':a.failure=r.failure;a.done=True
                else:a.env[dst]=r.result
                r.status='Consumed';a.waiting=None
                self._event(d,r.label,'Consumed',rid,r.failure or 'Value')
            except Invalid as err:
                # Keep the actual serve record/result, never claim nonexecution.
                r.status='ReleaseDenied';a.failure=str(err);a.done=True;a.waiting=None
                self._event(d,r.label,'ReleaseDenied',rid,a.failure)
            return a.failure
        return self._atomic(go)

    def drive(self,aid,max_steps=1000):
        for _ in range(max_steps):
            a=self.data.activities[aid]
            if a.done:return a.result
            if a.waiting:
                rid,_=a.waiting
                if self.data.requests[rid].status=='Pending':self.serve(rid)
                self.consume(aid)
            else:self.local_step(aid)
        raise Invalid('ExplicitStepBudget')

    def admin_observer(self,principal,clearance):
        def go(d):
            m=d.memberships.get(principal)
            if m not in d.live:raise Invalid('ObserverNotLive')
            d.observer_grants[principal]=(m,clearance)
        return self._atomic(go)
    def observe(self,principal,capacity=16):
        if type(capacity)is not int or capacity<0:raise Invalid('ObserverBudget')
        d=self.data;grant=d.observer_grants.get(principal)
        if grant is None or grant[0] not in d.live or d.memberships.get(principal)!=grant[0]:raise Invalid('ObserverDenied')
        safe=[e for e in d.events if e.label<=grant[1]]
        # Filter BEFORE bounded presentation retention: high traffic cannot evict
        # visible records. Data state, record IDs, and scheduler remain unchanged.
        rows=tuple(safe[-capacity:]) if capacity else ()
        self.observers[principal]=rows
        return rows

    def checkpoint(self):
        self._wf(self.data)
        position=len(self.journal)-1;digest=self.journal[position][0]
        cp=Checkpoint(self.data.instance,position,digest,COMPONENTS,deepcopy(self.data))
        self.snapshots[digest]=deepcopy(cp)
        return cp
    def recover(self,cp,tail,expected_head):
        """Same-instance crash recovery against an externally retained committed
        head; an old snapshot alone is not a current authority oracle.
        The journal supplies complete canonical state records, NOT replayed calls.
        """
        if cp.components!=COMPONENTS or cp.instance!=self.data.instance:raise Invalid('IncompleteOrForeignCheckpoint')
        registered=self.snapshots.get(cp.digest)
        if registered is None or not exact(canonical(registered),canonical(cp)):raise Invalid('UntrustedCheckpoint')
        if expected_head!=self.journal[-1][0]:raise Invalid('UntrustedRecoveryHead')
        previous=cp.digest;image=deepcopy(cp.image)
        for digest,prev,state in tail:
            if prev!=previous or fingerprint([prev,state])!=digest:raise Invalid('BrokenDurablePrefix')
            if state.instance!=cp.instance:raise Invalid('ForeignJournal')
            self._wf(state);previous=digest;image=deepcopy(state)
        if previous!=expected_head:raise Invalid('MissingCommittedSuffix')
        self._wf(image);self.data=image
        # Crash reconstruction invalidates observation buffers but not their
        # historic knowledge outside this system. No retroactive secrecy claim.
        self.observers={}
        return True

    def audit(self):return self._wf(self.data)
