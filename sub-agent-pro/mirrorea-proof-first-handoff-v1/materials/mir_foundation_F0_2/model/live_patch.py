"""Connect checked source migrations, actor gates and the patch publication model.
An explicit control protocol, never automatic multi-owner data atomicity.
Metadata/log writes are model-atomic. No real process or storage implementation.
"""
from __future__ import annotations
from dataclasses import dataclass
from .activation import initial,actions,step,Action,OLD,PREPARED,NEW,ABORTED,U,C,A
from .language import parse_check_compile,DomainFailure,StaticError,digest
from .engine import PatchRecord
from .certificates import verify_export

@dataclass
class LocalPlan:
    operation:object
    dependencies:frozenset
    versions:dict
    bindings:dict
    values:dict
    shadow:dict|None=None

class LivePatch:
    def __init__(self,engine,principal,migrations):
        """migrations: owner -> (source Program, op name, evidence or None).
        Source declarations must match existing cell owner/types; initial literals
        do not become state or authority. All candidate checks precede any fence.
        """
        if principal not in engine.patch_principals:raise DomainFailure('PatchAuthorityDenied')
        self.e=engine;self.principal=principal;self.plans={};self.pending_revocation=False
        actualcells={n:c for p in engine.programs for n,c in p.cells.items()}
        for owner,(supplied,opname,evidence) in sorted(migrations.items()):
            p=parse_check_compile(supplied.source,supplied.name)
            if owner not in engine.shards or opname not in p.operations:raise StaticError('UnknownMigrationOwner')
            op=p.operations[opname]
            if op.sig.owner!=owner or op.sig.params or not op.writes:raise StaticError('MigrationProfile')
            for n in op.reads|op.writes:
                if n not in actualcells or n not in p.cells or (p.cells[n].owner,p.cells[n].ty)!=(actualcells[n].owner,actualcells[n].ty):raise StaticError('MigrationSchemaMismatch')
            inv=engine.protected.get(owner,({},{}))[0]
            if inv:verify_export(op,inv,None,evidence)
            deps=frozenset(op.reads|op.writes|{v for q in inv.values() for m,_ in q.terms for v in m})
            shard=engine.shards[owner]
            bindings={n:engine.binding_versions[n] for n,o in shard.operations.items() if (o.reads|o.writes)&deps}
            self.plans[owner]=LocalPlan(op,deps,{n:shard.versions[n] for n in deps},bindings,{n:shard.state[n] for n in deps})
        self.owners=tuple(self.plans);self.protocol=initial(len(self.owners))
        engine.next_patch_serial+=1
        self.identity=digest([engine.run_id,engine.next_patch_serial,principal,[(o,p.operation.identity,sorted(p.versions.items()),sorted(p.bindings.items())) for o,p in self.plans.items()]])
        self.log=[]
    def pending_conflict(self,plan):
        for f in self.e.fibers.values():
            if f.waiting and f.waiting[0]=='request':
                r=self.e.issued[f.waiting[2]];op=self.e.ops[r.operation]
                if (op.reads|op.writes)&plan.dependencies:return True
        return False
    def advance(self,name,participant=-1):
        a=Action(name,participant)
        if a not in tuple(actions(self.protocol)):raise DomainFailure('DisabledPatchTransition')
        if name=='prepare':
            owner=self.owners[participant];p=self.plans[owner];s=self.e.shards[owner]
            if self.principal not in self.e.patch_principals:raise DomainFailure('PatchAuthorityDenied')
            if p.dependencies&s.frozen:raise DomainFailure('PatchFenceBusy')
            if self.pending_conflict(p):raise DomainFailure('AffectedRequestUnsettled')
            if any(s.versions[n]!=v for n,v in p.versions.items()) or any(self.e.binding_versions[n]!=v for n,v in p.bindings.items()):raise DomainFailure('StalePatchPreparation')
            new,_=p.operation.evaluate(s.state,())
            p.shadow={n:new[n] for n in p.operation.writes}
            # Durable shadow + closed dependency fence precede a yes vote.
            s.frozen.update(p.dependencies)
            s.fence_owners.update({n:self.identity for n in p.dependencies})
        if name=='decide-commit' and self.principal not in self.e.patch_principals:raise DomainFailure('PatchAuthorityDenied')
        if name=='install':
            owner=self.owners[participant];p=self.plans[owner];s=self.e.shards[owner]
            if any(s.fence_owners.get(n)!=self.identity for n in p.dependencies):raise DomainFailure('PatchFenceOwnership')
            versions={n:self.e.binding_versions[n]+1 for n,op in s.operations.items() if (op.reads|op.writes)&p.operation.writes}
            rec=PatchRecord(self.identity,dict(p.shadow),tuple(sorted(p.operation.writes)),{},versions)
            s.log.append(rec);s.state.update(rec.writes)
            for n in rec.touched:s.versions[n]+=1
            s.op_versions.update(versions);self.e.binding_versions.update(versions)
            s.frozen.difference_update(p.dependencies)
            for n in p.dependencies:s.fence_owners.pop(n,None)
        if name=='abort-local' and self.protocol.phase[participant]==PREPARED:
            s=self.e.shards[self.owners[participant]];plan=self.plans[self.owners[participant]]
            if any(s.fence_owners.get(n)!=self.identity for n in plan.dependencies):raise DomainFailure('PatchFenceOwnership')
            s.frozen.difference_update(plan.dependencies)
            for n in plan.dependencies:s.fence_owners.pop(n,None)
        if name=='crash' and participant<len(self.owners):self.e.crash_owner(self.owners[participant])
        if name=='recover' and participant<len(self.owners):self.e.recover_owner(self.owners[participant])
        self.protocol=step(self.protocol,a);self.log.append({'transition':a.text(),'decision':self.protocol.decision,'identity':self.identity})
        if all(x in (NEW,ABORTED) for x in self.protocol.phase) and self.pending_revocation:self.e.patch_principals.discard(self.principal)
    def request_revocation(self):
        # Reservation lifetime is explicit: revocation becomes effective only after
        # this already-prepared control operation resolves. This is NOT an instant
        # cross-network revocation promise. The real adapter must reserve issuer epochs.
        if PREPARED in self.protocol.phase:self.pending_revocation=True
        else:self.e.patch_principals.discard(self.principal)
    def semantic_values(self):
        out=self.e.state()
        if self.protocol.decision==C:
            for i,owner in enumerate(self.owners):
                if self.protocol.phase[i]==PREPARED:out.update(self.plans[owner].shadow)
        return out
    def visible_object(self,owner,name):
        s=self.e.shards[owner]
        if not s.up or name in s.frozen:raise DomainFailure('PatchPending')
        return s.state[name]
    def complete(self):
        n=len(self.owners)
        for i in range(n):self.advance('prepare',i);self.advance('vote',i)
        self.advance('decide-commit')
        for i in range(n):self.advance('learn',i);self.advance('install',i)
