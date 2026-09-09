"""Quiescent snapshot import into a FRESH instance, NOT rollback of a live world.
Existing grants, request decisions and endpoint authority are NOT imported.
Current policy re-admission is required. Ordinary crash replay uses journal.py.
"""
from __future__ import annotations
import copy
from dataclasses import dataclass
from .engine import Engine
from .language import DomainFailure,StaticError,tyval,parse_check_compile
@dataclass
class Snapshot:
    saved_instance:str
    programs:list
    values:dict
    protections:dict
    history:list
    code_updates:list
    exports:dict
class InstanceAuthority:
    def __init__(self):self.issued=set();self.sequence=0
    def fresh(self,prefix='branch'):
        self.sequence+=1;n=f'{prefix}:{self.sequence}'
        if n in self.issued:raise DomainFailure('InstanceReuse')
        self.issued.add(n);return n

def capture(e):
    if any(not f.done for f in e.fibers.values()):raise DomainFailure('SnapshotNotQuiescent')
    if any(not s.up or s.frozen for s in e.shards.values()):raise DomainFailure('SnapshotNotQuiescent')
    # old in-flight *copies* are harmless only because they cannot cross fresh instance admission
    return Snapshot(e.run_id,copy.deepcopy(e.programs),copy.deepcopy(e.state()),copy.deepcopy(e.protected),copy.deepcopy(e.trace),copy.deepcopy(e.code_updates),copy.deepcopy(e.exports))

def restore_as_fresh(s,authority):
    # Validate the complete candidate before issuing an active instance.
    e=Engine(s.programs,run_id='inactive-candidate')
    e.patch_principals.add('__restore_authority')
    for source,module,operation in s.code_updates:
        p=parse_check_compile(source,module,{op.sig.name:op.sig for op in e.ops.values()})
        e.replace_operation('__restore_authority',p,operation)
    e.patch_principals.clear()
    if set(e.state())!=set(s.values):raise StaticError('SnapshotSchemaMismatch')
    cells={n:c for p in e.programs for n,c in p.cells.items()}
    if any(tyval(v)!=cells[n].ty for n,v in s.values.items()):raise StaticError('SnapshotTypeMismatch')
    for shard in e.shards.values():
        for n in shard.state:shard.state[n]=s.values[n];shard.base[n]=s.values[n]
        shard.log.clear();shard.base_operations=dict(shard.operations);shard.op_versions={n:0 for n in shard.operations};shard.versions={n:0 for n in shard.state}
        e.binding_versions.update(shard.op_versions)
    for owner,(invariants,evidence) in s.protections.items():e.protect_owner(owner,invariants,evidence)
    for name,(proof,post) in s.exports.items():e.register_export(name,proof,post)
    fresh=authority.fresh()
    if fresh==s.saved_instance:raise DomainFailure('InstanceReuse')
    e.run_id=fresh
    e.trace.append({'kind':'snapshot-imported','from':s.saved_instance,'to':fresh,'old_history':'historical-only'})
    # No grants, runnable old fibers, old messages, patch permissions, or success receipts.
    return e
