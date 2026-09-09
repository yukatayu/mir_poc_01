"""Actor continuations and owner services over an IDEAL authenticated message bag.
No QUIC/sockets. State-only operations and decisions have an atomic durable model
record; this is not a filesystem proof. All exported source values are public.
"""
from __future__ import annotations
from dataclasses import dataclass,field
import copy
from .language import Program,DomainFailure,StaticError,tyval,binop,parse_check_compile
@dataclass(frozen=True)
class Permit:
    serial:int
    principal:str
    operation:str
    generation:int
class Authority:
    def __init__(self):self.current={};self.revoked=set();self.issued={};self.serial=0
    def grant(self,principal,op):
        self.serial+=1;k=(principal,op);g=self.current.get(k,0)+1;self.current[k]=g
        p=Permit(self.serial,principal,op,g);self.issued[p.serial]=p;return p
    def revoke(self,p):self.revoked.add(p.serial)
    def allowed(self,p,principal,op):return isinstance(p,Permit) and self.issued.get(p.serial)==p and p.principal==principal and p.operation==op and p.serial not in self.revoked and self.current.get((principal,op))==p.generation
@dataclass(frozen=True)
class Request:
    key:tuple[str,int,int]
    principal:str
    origin:str
    operation:str
    code_hash:str
    contract_hash:str
    args:tuple
    permit:Permit|None
    source_line:int
    binding_epoch:int=0
@dataclass(frozen=True)
class Outcome:
    request:Request
    value:object=None
    failure:str|None=None
    executed:bool=False
@dataclass
class Record:
    request:Request
    writes:dict
    outcome:Outcome
    touched:tuple=()
@dataclass
class PatchRecord:
    patch_id:str
    writes:dict
    touched:tuple
    replacements:dict
    binding_versions:dict
class Shard:
    def __init__(self,owner,initial,operations):
        self.owner=owner;self.base=dict(initial);self.state=dict(initial);self.operations=dict(operations)
        self.base_operations=dict(operations);self.op_versions={n:0 for n in operations};self.versions={n:0 for n in initial}
        self.log=[];self.decisions={};self.up=True;self.frozen=set();self.fence_owners={}
    def recover(self):
        self.state=dict(self.base);self.decisions={};self.versions={n:0 for n in self.base};self.operations=dict(self.base_operations);self.op_versions={n:0 for n in self.operations}
        for r in self.log:
            self.state.update(r.writes)
            for n in r.touched:self.versions[n]=self.versions.get(n,0)+1
            if isinstance(r,Record):self.decisions[r.request.key]=r.outcome
            else:self.operations.update(r.replacements);self.op_versions.update(r.binding_versions)
        self.up=True
    def service(self,r,auth,issued,crash_at=None):
        if not self.up:return None
        if issued.get(r.key)!=r:return Outcome(r,failure='UnboundRequest')
        op=self.operations.get(r.operation)
        if op is None or op.sig.owner!=self.owner:return Outcome(r,failure='WrongEndpoint')
        if op.identity!=r.code_hash or op.sig.identity!=r.contract_hash or self.op_versions[r.operation]!=r.binding_epoch:
            old=self.decisions.get(r.key);return Outcome(r,failure='StaleCode',executed=bool(old and old.executed))
        if not auth.allowed(r.permit,r.principal,r.operation):
            old=self.decisions.get(r.key);return Outcome(r,failure='AuthorityDenied',executed=bool(old and old.executed))
        if (op.reads|op.writes)&self.frozen:return None
        if r.key in self.decisions:
            old=self.decisions[r.key]
            return old if old.request==r else Outcome(r,failure='IdentityCollision')
        if crash_at=='before_commit':self.up=False;return None
        try:
            new,v=op.evaluate(self.state,r.args)
            if set(new)!=set(self.state) or any(k not in op.writes and new[k]!=self.state[k] for k in self.state):raise DomainFailure('UndeclaredWrite')
            out=Outcome(r,value=v,executed=True)
        except (DomainFailure,KeyError,StaticError) as e:
            out=Outcome(r,failure=e.reason if isinstance(e,DomainFailure) else str(e));new=dict(self.state)
        delta={k:v for k,v in new.items() if v!=self.state[k]}
        self.log.append(Record(r,delta,out,tuple(sorted(op.writes)) if out.executed else ()))
        for n in self.log[-1].touched:self.versions[n]+=1
        self.state=dict(new);self.decisions[r.key]=out
        if crash_at=='after_commit':self.up=False;return None
        return out
@dataclass
class Frame:
    function:str
    pc:int
    locals:dict
    return_to:str|None=None
@dataclass
class Fiber:
    ident:int
    principal:str
    locus:str
    frames:list[Frame]
    counter:int=0
    waiting:tuple|None=None
    result:object=None
    failure:str|None=None
    done:bool=False
    children:set[int]=field(default_factory=set)
class Engine:
    def __init__(self,programs,run_id='run-1'):
        self.programs=[];self.run_id=run_id;self.ops={};self.tasks={};cells={}
        # Recheck complete untrusted source on admission; never trust a mutated Program.
        for supplied in programs:
            p=parse_check_compile(supplied.source,supplied.name,{s.name:s for s in supplied.imports.values()})
            if set(cells)&set(p.cells):raise StaticError('GlobalCellCollisionRequiresInstanceRenaming')
            cells.update(p.cells)
            if set(self.ops)&set(p.operations):raise StaticError('OperationCollision')
            self.ops.update(p.operations);incoming={f.sig.name:f for f in p.functions.values() if f.sig.kind=='task'}
            if set(self.tasks)&set(incoming):raise StaticError('TaskCollision')
            self.tasks.update(incoming);self.programs.append(p)
        for p in self.programs:
            for s in p.imports.values():
                actual=self.ops.get(s.name)
                if actual is None or actual.sig.identity!=s.identity:raise StaticError('InterfaceBindingMismatch')
        owners={c.owner for c in cells.values()}|{o.sig.owner for o in self.ops.values()}
        self.shards={o:Shard(o,{n:c.initial for n,c in cells.items() if c.owner==o},{n:op for n,op in self.ops.items() if op.sig.owner==o}) for o in owners}
        self.auth=Authority();self.permits={};self.fibers={};self.durable={};self.next_fiber=1
        self.issued={};self.requests=[];self.responses=[];self.outcomes={};self.trace=[];self.delivery_count=0
        self.patch_principals=set();self.protected={};self.exports={};self.binding_versions={n:0 for n in self.ops};self.code_updates=[];self.next_patch_serial=0
    def grant(self,principal,operation):
        if operation not in self.ops:raise KeyError(operation)
        p=self.auth.grant(principal,operation);self.permits[(principal,operation)]=p;return p
    def grant_all(self,principal):return [self.grant(principal,n) for n in self.ops]  # explicit trusted setup only
    def spawn(self,function,args,principal='alice',parent=None):
        f=self.tasks[function]
        if len(args)!=len(f.sig.params) or [tyval(a) for a in args]!=[t for _,t in f.sig.params]:raise DomainFailure('TaskArgumentType')
        fid=self.next_fiber;self.next_fiber+=1
        self.fibers[fid]=Fiber(fid,principal,f.sig.owner,[Frame(function,0,dict(zip((n for n,_ in f.sig.params),args)))])
        if parent is not None:self.fibers[parent].children.add(fid)
        self.durable[fid]=copy.deepcopy(self.fibers[fid]);return fid
    def step(self,fid):
        f=self.fibers[fid]
        if f.done:return False
        if f.failure:
            if any(not self.fibers[c].done for c in f.children):return False
            f.done=True;self.durable[fid]=copy.deepcopy(f);return True
        if f.waiting:
            kind,dst,key=f.waiting
            if kind!='join':return False
            child=self.fibers[key]
            if not child.done:return False
            if child.failure:f.failure=child.failure;f.waiting=None;f.done=all(self.fibers[c].done for c in f.children)
            else:f.frames[-1].locals[dst]=child.result;f.waiting=None
            self.durable[fid]=copy.deepcopy(f);return True
        fr=f.frames[-1];fun=self.tasks[fr.function];ins=fun.code[fr.pc];line=fun.source_map[fr.pc];fr.pc+=1;t=ins[0];e=fr.locals
        if t=='const':e[ins[1]]=ins[2]
        elif t=='move':e[ins[1]]=e[ins[2]]
        elif t=='unary':e[ins[1]]=-e[ins[3]] if ins[2]=='USub' else not e[ins[3]]
        elif t=='binary':e[ins[1]]=binop(ins[2],e[ins[3]],e[ins[4]])
        elif t=='branch':fr.pc=ins[2] if e[ins[1]] else ins[3]
        elif t=='jump':fr.pc=ins[1]
        elif t=='nop':pass
        elif t=='request':
            _,dst,n,argregs=ins;op=self.ops[n];f.counter+=1;k=(self.run_id,f.ident,f.counter)
            r=Request(k,f.principal,f.locus,n,op.identity,op.sig.identity,tuple(e[a] for a in argregs),self.permits.get((f.principal,n)),line,self.binding_versions[n])
            if k in self.issued:raise AssertionError('FreshKeyReused')
            f.waiting=('request',dst,k);self.issued[k]=r;self.durable[fid]=copy.deepcopy(f);self.requests.append(r)
            self.trace.append({'kind':'issue','key':k,'op':n,'source_line':line,'locus':f.locus})
        elif t=='call':
            _,dst,n,args=ins;callee=self.tasks[n]
            f.frames.append(Frame(n,0,dict(zip((n for n,_ in callee.sig.params),(e[a] for a in args))),dst))
        elif t=='spawn':
            _,dst,n,args=ins;e[dst]=self.spawn(n,tuple(e[a] for a in args),f.principal,fid)
        elif t=='join':
            _,dst,r=ins
            if e[r] not in f.children:raise DomainFailure('ForeignFuture')
            f.waiting=('join',dst,e[r])
        elif t=='return':
            if len(f.frames)==1 and any(not self.fibers[c].done for c in f.children):fr.pc-=1;return False
            if len(f.frames)==1:
                failures=[self.fibers[c].failure for c in sorted(f.children) if self.fibers[c].failure]
                if failures:
                    f.failure=failures[0];f.done=True;self.durable[fid]=copy.deepcopy(f);return True
            v=e[ins[1]];ret=fr.return_to;f.frames.pop()
            if f.frames:f.frames[-1].locals[ret]=v
            else:f.result=v;f.done=True
        else:raise DomainFailure('UnknownInstruction')
        self.durable[fid]=copy.deepcopy(f);return True
    def deliver_request(self,index=0,duplicate=False,crash_at=None):
        if not self.requests:return False
        r=self.requests.pop(index)
        if duplicate:self.requests.append(r)
        op=self.ops.get(r.operation)
        if op is None:return True
        shard=self.shards[op.sig.owner];before=len(shard.log);out=shard.service(r,self.auth,self.issued,crash_at)
        if len(shard.log)>before:self.trace.append({'kind':'decision','key':r.key,'op':r.operation,'source_line':r.source_line,'executed':shard.log[-1].outcome.executed,'failure':shard.log[-1].outcome.failure,'locus':shard.owner})
        if out is not None:self.responses.append(out);self.outcomes.setdefault(r.key,[]).append(out)
        self.delivery_count+=1;return True
    def deliver_response(self,index=0,duplicate=False):
        if not self.responses:return False
        out=self.responses.pop(index);r=out.request
        if duplicate:self.responses.append(out)
        f=self.fibers.get(r.key[1])
        if self.issued.get(r.key)!=r or out not in self.outcomes.get(r.key,[]) or f is None or f.done or f.waiting is None or f.waiting[0]!='request' or f.waiting[2]!=r.key:return True
        if out.failure is None and (not self.auth.allowed(r.permit,f.principal,r.operation) or self.binding_versions.get(r.operation)!=r.binding_epoch):self.trace.append({'kind':'release-rejected','key':r.key,'reason':'AuthorityDenied'});return True
        dst=f.waiting[1]
        if out.failure:f.failure=out.failure;f.waiting=None;f.done=all(self.fibers[c].done for c in f.children)
        else:f.frames[-1].locals[dst]=out.value;f.waiting=None
        self.durable[f.ident]=copy.deepcopy(f);self.trace.append({'kind':'consume','key':r.key,'failure':out.failure,'locus':f.locus});return True
    def resend(self,key):
        f=self.fibers[key[1]]
        if f.done or not f.waiting or f.waiting[0]!='request' or f.waiting[2]!=key:raise DomainFailure('NotPending')
        self.requests.append(self.issued[key])  # explicit retry, NEVER timeout-triggered
    def crash_owner(self,o):self.shards[o].up=False
    def recover_owner(self,o):
        self.shards[o].recover();self.ops.update(self.shards[o].operations);self.binding_versions.update(self.shards[o].op_versions)
    def crash_resume_fiber(self,fid):self.fibers[fid]=copy.deepcopy(self.durable[fid])
    def state(self):return {n:v for s in self.shards.values() for n,v in s.state.items()}
    def run(self,limit=100000):
        for _ in range(limit):
            if self.fibers and all(f.done for f in self.fibers.values()):return
            changed=False
            for f in list(self.fibers):changed=self.step(f) or changed
            changed=self.deliver_request() or changed;changed=self.deliver_response() or changed
            if not changed:raise DomainFailure('BlockedOrRemoteUnknown')
        raise DomainFailure('ExecutionBudget')
    def register_export(self,name,evidence,post):
        from .certificates import verify_contract, CertificateError
        if name not in self.ops:raise CertificateError('UnknownExport')
        op=self.ops[name];invariants=self.protected.get(op.sig.owner,({},{}))[0]
        verify_contract(op,invariants,post,evidence)
        if name in self.exports:
            old_ev,old_post=self.exports[name]
            if old_post!=post or getattr(old_ev,'preconditions',None)!=getattr(evidence,'preconditions',None):raise CertificateError('ExportContractUpdateRequired')
        self.exports[name]=(evidence,post)
    def protect_owner(self,owner,invariants,evidence_by_operation):
        from .certificates import verify_export, CertificateError
        s=self.shards[owner]
        variables={v for p in invariants.values() for m,_ in p.terms for v in m}
        if not variables<=set(s.state):raise CertificateError('InvariantOwnerScope')
        if any(n in self.exports for n in s.operations) and self.protected.get(owner,({},{}))[0]!=invariants:raise CertificateError('ExportInvariantContextChange')
        if any(p.value(s.state)<0 for p in invariants.values()):raise CertificateError('InitialInvariantViolation')
        for name,op in s.operations.items():
            if not op.writes:continue
            if name not in evidence_by_operation:raise CertificateError('UnprovedMutator:'+name)
            evidence,post=evidence_by_operation[name];verify_export(op,invariants,post,evidence)
        self.protected[owner]=(dict(invariants),dict(evidence_by_operation))
    def install_addition(self,principal,program):
        if principal not in self.patch_principals:raise DomainFailure('PatchAuthorityDenied')
        p=parse_check_compile(program.source,program.name,{s.name:s for s in program.imports.values()})
        if set(p.cells)&set(self.state()) or set(p.operations)&set(self.ops) or any(f.sig.name in self.tasks for f in p.functions.values() if f.sig.kind=='task'):raise StaticError('AdditionCollision')
        for sig in p.imports.values():
            if sig.name not in self.ops or self.ops[sig.name].sig.identity!=sig.identity:raise StaticError('InterfaceBindingMismatch')
        for op in p.operations.values():
            if op.writes and op.sig.owner in self.protected:raise StaticError('InvariantRequiresNewMutatorEvidence')
        for c in p.cells.values():
            if c.owner not in self.shards:self.shards[c.owner]=Shard(c.owner,{}, {})
            self.shards[c.owner].base[c.name]=c.initial;self.shards[c.owner].state[c.name]=c.initial;self.shards[c.owner].versions[c.name]=0
        for n,op in p.operations.items():
            self.ops[n]=op
            if op.sig.owner not in self.shards:self.shards[op.sig.owner]=Shard(op.sig.owner,{}, {})
            self.shards[op.sig.owner].operations[n]=op;self.shards[op.sig.owner].base_operations[n]=op;self.shards[op.sig.owner].op_versions[n]=0;self.binding_versions[n]=0
        for f in p.functions.values():
            if f.sig.kind=='task':self.tasks[f.sig.name]=f
        self.programs.append(p);self.trace.append({'kind':'module-added','module':p.name,'identity':p.identity});return p
    def replace_operation(self,principal,program,name,evidence=None,post=None,invariant_evidence=None):
        from .certificates import verify_export,verify_contract,CertificateError,TotalEvidence
        if principal not in self.patch_principals:raise DomainFailure('PatchAuthorityDenied')
        p=parse_check_compile(program.source,program.name,{s.name:s for s in program.imports.values()});old=self.ops[name];new=p.operations[name]
        if old.sig.identity!=new.sig.identity:raise StaticError('ContractUpdateRequired')
        actual={n:c for q in self.programs for n,c in q.cells.items()}
        for cell in new.reads|new.writes:
            if cell not in p.cells or cell not in actual or (p.cells[cell].owner,p.cells[cell].ty)!=(actual[cell].owner,actual[cell].ty):raise StaticError('ReplacementSchemaMismatch')
        if new.reads-old.reads or new.writes-old.writes:raise StaticError('FootprintWidening')
        affected=old.reads|old.writes|new.reads|new.writes
        if affected&self.shards[old.sig.owner].frozen:raise DomainFailure('PatchFenceBusy')
        for f in self.fibers.values():
            if f.waiting and f.waiting[0]=='request':
                r=self.issued[f.waiting[2]];op=self.ops[r.operation]
                if r.operation==name or (op.reads|op.writes)&affected:raise DomainFailure('AffectedRequestUnsettled')
        ev,newpost=evidence,post
        if isinstance(evidence,tuple) and len(evidence)==2:ev,newpost=evidence
        inv,proofs=self.protected.get(old.sig.owner,({},{}))
        if name in self.exports:
            oldev,oldpost=self.exports[name]
            if ev is None:raise CertificateError('ReplacementProofRequired')
            if newpost!=oldpost:raise CertificateError('PostconditionChanged')
            if isinstance(oldev,TotalEvidence) and (not isinstance(ev,TotalEvidence) or ev.preconditions!=oldev.preconditions):raise CertificateError('SuccessDomainChanged')
            verify_contract(new,inv,newpost,ev)
        invpacket=None
        if old.sig.owner in self.protected and old.writes:
            old_inv_post=proofs[name][1]
            invpacket=invariant_evidence if invariant_evidence is not None else (ev,newpost)
            if invpacket[1]!=old_inv_post:raise StaticError('StrongContractUpdateRequired')
            verify_export(new,inv,invpacket[1],invpacket[0])
        # All checks precede publication of any proof/code/epoch changes.
        shard=self.shards[new.sig.owner];epoch=self.binding_versions[name]+1
        shard.log.append(PatchRecord('body:'+str(len(self.code_updates)),{},(),{name:new},{name:epoch}))
        self.ops[name]=new;shard.operations[name]=new;shard.op_versions[name]=epoch;self.binding_versions[name]=epoch
        if name in self.exports:self.exports[name]=(ev,newpost)
        if invpacket is not None:proofs[name]=invpacket
        self.code_updates.append((p.source,p.name,name))
        self.trace.append({'kind':'operation-replaced','operation':name,'old':old.identity,'new':new.identity})
    def assert_invariants(self):
        for s in self.shards.values():
            keys=[r.request.key for r in s.log if isinstance(r,Record)];assert len(keys)==len(set(keys))
            for r in s.log:
                if isinstance(r,Record):assert self.issued[r.request.key]==r.request
        consumed=[tuple(t['key']) for t in self.trace if t['kind']=='consume'];assert len(consumed)==len(set(consumed))
        for f in self.fibers.values():
            if f.waiting and f.waiting[0]=='request':assert f.waiting[2] in self.issued
            if f.done:assert all(self.fibers[c].done for c in f.children)
        for o,(invs,_) in self.protected.items():assert all(p.value(self.shards[o].state)>=0 for p in invs.values())
