"""Executable RESEARCH MODEL, not the Mir runtime or a verified compiler.

Types: mathematical integers, booleans and finite products. No network bytes,
cryptography, crash recovery, pointers, hidden RPC, or domain-specific primitive.
Every transition is an atomic transition of the explicitly described model.
"""
from __future__ import annotations
from dataclasses import dataclass, field, replace
from collections import deque
from typing import Any, Mapping, Iterable
import hashlib
import json

@dataclass(frozen=True)
class Ty:
    tag: str
    left: 'Ty | None' = None
    right: 'Ty | None' = None

INT, BOOL = Ty('Int'), Ty('Bool')
def PROD(a: Ty, b: Ty) -> Ty:
    return Ty('Product', a, b)

def value_type(v: Any) -> Ty:
    if type(v) is bool:
        return BOOL
    if type(v) is int:
        return INT
    if type(v) is tuple and len(v) == 2:
        return PROD(value_type(v[0]), value_type(v[1]))
    raise TypeError('Not a model value')

@dataclass(frozen=True)
class Expr:
    tag: str
    args: tuple[Any, ...]

def E(tag: str, *args: Any) -> Expr:
    return Expr(tag, args)

@dataclass(frozen=True)
class CellSpec:
    ty: Ty
    owner: str
    label: int = 0  # 0 = low, 1 = high, NOT an authority level

@dataclass(frozen=True)
class Info:
    ty: Ty
    reads: frozenset[str]
    label: int

class StaticError(Exception):
    pass


def infer(expr: Expr, schema: Mapping[str, CellSpec],
          env: Mapping[str, tuple[Ty, int]] | None = None) -> Info:
    env = {} if env is None else env
    tag, a = expr.tag, expr.args
    def need(ok: bool, why: str) -> None:
        if not ok:
            raise StaticError(why)
    if tag == 'lit':
        try:
            return Info(value_type(a[0]), frozenset(), 0)
        except TypeError as exc:
            raise StaticError(str(exc)) from exc
    if tag == 'var':
        need(a[0] in env, 'UnboundVariable')
        ty, label = env[a[0]]
        return Info(ty, frozenset(), label)
    if tag == 'cell':
        need(a[0] in schema, 'UnknownCell')
        spec = schema[a[0]]
        return Info(spec.ty, frozenset({a[0]}), spec.label)
    if tag == 'let':
        name, source, body = a
        i = infer(source, schema, env)
        j = infer(body, schema, {**env, name: (i.ty, i.label)})
        return Info(j.ty, i.reads | j.reads, max(i.label, j.label))
    if tag in ('neg', 'not', 'fst', 'snd'):
        i = infer(a[0], schema, env)
        if tag == 'neg':
            need(i.ty == INT, 'ExpectedInt'); ty = INT
        elif tag == 'not':
            need(i.ty == BOOL, 'ExpectedBool'); ty = BOOL
        else:
            need(i.ty.tag == 'Product', 'ExpectedProduct')
            ty = i.ty.left if tag == 'fst' else i.ty.right
        return Info(ty, i.reads, i.label)
    if tag == 'if':
        c, t, f = (infer(x, schema, env) for x in a)
        need(c.ty == BOOL and t.ty == f.ty, 'ConditionalTypeMismatch')
        return Info(t.ty, c.reads | t.reads | f.reads,
                    max(c.label, t.label, f.label))
    if tag in ('add', 'sub', 'le', 'eq', 'and', 'pair'):
        i, j = (infer(x, schema, env) for x in a)
        if tag in ('add', 'sub', 'le'):
            need(i.ty == INT and j.ty == INT, 'ExpectedInt')
            ty = BOOL if tag == 'le' else INT
        elif tag == 'and':
            need(i.ty == BOOL and j.ty == BOOL, 'ExpectedBool'); ty = BOOL
        elif tag == 'eq':
            need(i.ty == j.ty, 'EqualityTypeMismatch'); ty = BOOL
        else:
            ty = PROD(i.ty, j.ty)
        return Info(ty, i.reads | j.reads, max(i.label, j.label))
    raise StaticError('UnsupportedExpression')


def evaluate(expr: Expr, store: Mapping[str, Any],
             env: Mapping[str, Any] | None = None) -> Any:
    """Pure evaluation. Caller must supply a well-typed store/environment."""
    env = {} if env is None else env
    tag, a = expr.tag, expr.args
    if tag == 'lit': return a[0]
    if tag == 'var': return env[a[0]]
    if tag == 'cell': return store[a[0]]
    if tag == 'let':
        return evaluate(a[2], store, {**env, a[0]: evaluate(a[1], store, env)})
    if tag == 'if':
        return evaluate(a[1] if evaluate(a[0], store, env) else a[2], store, env)
    if tag in ('neg', 'not', 'fst', 'snd'):
        v = evaluate(a[0], store, env)
        if tag == 'neg': return -v
        if tag == 'not': return not v
        return v[0 if tag == 'fst' else 1]
    x, y = (evaluate(e, store, env) for e in a)
    if tag == 'add': return x + y
    if tag == 'sub': return x - y
    if tag == 'le': return x <= y
    if tag == 'eq': return x == y
    if tag == 'and': return x and y
    if tag == 'pair': return (x, y)
    raise ValueError('UnsupportedExpression')


def typed_store(schema: Mapping[str, CellSpec], store: Mapping[str, Any]) -> bool:
    return set(schema) == set(store) and all(value_type(store[x]) == s.ty for x, s in schema.items())


def expr_data(expr: Expr) -> Any:
    return [expr.tag, *[expr_data(x) if isinstance(x, Expr) else x for x in expr.args]]

@dataclass(frozen=True)
class Operation:
    name: str
    origin: str
    target: str
    body: Expr
    parameters: tuple[tuple[str, Ty, int], ...] = ()
    guard: Expr = E('lit', True)
    revision: int = 0
    pc: int = 0
    kind: str = 'write'  # write or sample; not an arbitrary host effect

@dataclass(frozen=True)
class CheckedOperation:
    source: Operation
    owner: str
    reads: frozenset[str]
    fingerprint: str
    edges: tuple[tuple[str, str, str], ...]


def compile_operation(op: Operation, schema: Mapping[str, CellSpec]) -> CheckedOperation:
    if op.kind not in ('write', 'sample'):
        raise StaticError('UnsupportedOperationKind')
    if op.target not in schema:
        raise StaticError('UnknownTarget')
    if len({n for n, _, _ in op.parameters}) != len(op.parameters):
        raise StaticError('DuplicateParameter')
    env = {n: (t, l) for n, t, l in op.parameters}
    body, guard = infer(op.body, schema, env), infer(op.guard, schema, env)
    target = schema[op.target]
    if body.ty != target.ty or guard.ty != BOOL:
        raise StaticError('OperationTypeMismatch')
    reads = body.reads | guard.reads
    if any(schema[x].owner != target.owner for x in reads):
        raise StaticError('CrossOwnerReadRequiresSnapshotBoundary')
    if max(op.pc, body.label, guard.label) > target.label:
        raise StaticError('ImplicitOrExplicitFlowToLowerLabel')
    payload = [op.name, op.origin, op.target, op.revision, op.pc, op.kind,
               expr_data(op.body), expr_data(op.guard),
               [(n, repr(t), l) for n, t, l in op.parameters],
               sorted((x, repr(schema[x])) for x in reads | {op.target})]
    fingerprint = hashlib.sha256(json.dumps(payload, sort_keys=True).encode()).hexdigest()
    # Fingerprints are bookkeeping in this model, NOT credentials/attestation.
    edges = () if op.origin == target.owner else ((op.origin, target.owner, 'request'),
                                                   (target.owner, op.origin, 'outcome'))
    return CheckedOperation(op, target.owner, reads, fingerprint, edges)


def topological_rank(nodes: Iterable[str], edges: Iterable[tuple[str, str]]) -> dict[str, int]:
    """Parent -> dependent. No fixed root. Exact finite cycle detection."""
    nodes, edges = set(nodes), set(edges)
    if any(a not in nodes or b not in nodes for a, b in edges):
        raise StaticError('UnknownDependencyEndpoint')
    incoming = {n: 0 for n in nodes}
    successors = {n: set() for n in nodes}
    for a, b in edges:
        successors[a].add(b); incoming[b] += 1
    queue = deque(sorted(n for n in nodes if incoming[n] == 0))
    rank = {n: 0 for n in queue}
    visited = 0
    while queue:
        n = queue.popleft(); visited += 1
        for nxt in sorted(successors[n]):
            rank[nxt] = max(rank.get(nxt, 0), rank[n] + 1)
            incoming[nxt] -= 1
            if incoming[nxt] == 0:
                queue.append(nxt)
    if visited != len(nodes):
        raise StaticError('DependencyCycle')
    return rank


def descendants(seeds: Iterable[str], edges: Iterable[tuple[str, str]]) -> set[str]:
    gone, edges = set(seeds), set(edges)
    while True:
        more = gone | {b for a, b in edges if a in gone}
        if more == gone: return gone
        gone = more


def resolve_fallback(cursor: int, available: tuple[bool, ...]) -> int:
    """n is a sticky exhausted state. Recovery needs a separate new lineage."""
    if not 0 <= cursor <= len(available): raise ValueError('BadCursor')
    for j in range(cursor, len(available)):
        if available[j]: return j
    return len(available)


def flatten_chain(tree: Any) -> list[Any]:
    if isinstance(tree, tuple) and len(tree) == 3 and tree[0] == 'fallback':
        return flatten_chain(tree[1]) + flatten_chain(tree[2])
    return [tree]

@dataclass(frozen=True)
class Grant:
    principal: str
    incarnation: int
    operation: str
    auth_epoch: int

@dataclass(frozen=True)
class Request:
    key: tuple[str, int, int]
    operation: str
    fingerprint: str
    args: tuple[Any, ...]
    grant: Grant
    cell_incarnations: tuple[tuple[str, int], ...]

@dataclass(frozen=True)
class Outcome:
    kind: str
    request: Request
    value: Any = None
    mutated: bool = False

@dataclass
class Config:
    schema: dict[str, CellSpec]
    store: dict[str, Any]
    operations: dict[str, CheckedOperation]
    edges: set[tuple[str, str]] = field(default_factory=set)
    cell_incarnations: dict[str, int] = field(default_factory=dict)
    versions: dict[str, int] = field(default_factory=dict)
    membership: dict[str, int] = field(default_factory=dict)
    auth_epochs: dict[tuple[str, str], int] = field(default_factory=dict)
    grants: set[Grant] = field(default_factory=set)
    next_counter: dict[tuple[str, int], int] = field(default_factory=dict)
    pending: dict[tuple[str, int, int], Request] = field(default_factory=dict)
    ledger: dict[tuple[str, int, int], Outcome] = field(default_factory=dict)
    received: dict[tuple[str, int, int], Outcome] = field(default_factory=dict)
    retired_cells: set[tuple[str, int]] = field(default_factory=set)
    patch_admins: set[str] = field(default_factory=set)
    clearances: dict[str, int] = field(default_factory=dict)
    writes: dict[str, int] = field(default_factory=dict)

    def __post_init__(self) -> None:
        for x in self.schema:
            self.cell_incarnations.setdefault(x, 0)
            self.versions.setdefault(x, 0)
            self.writes.setdefault(x, 0)
        self.assert_wf()

    def assert_wf(self) -> None:
        if not typed_store(self.schema, self.store):
            raise AssertionError('IllTypedStore')
        topological_rank(self.schema, self.edges)
        for x in self.schema:
            if (x, self.cell_incarnations[x]) in self.retired_cells:
                raise AssertionError('ResurrectedCell')
        for name, op in self.operations.items():
            if compile_operation(op.source, self.schema) != op:
                raise AssertionError(f'UncheckedOperation:{name}')

    def issue_grant(self, admin: str, principal: str, operation: str) -> Grant:
        """Explicit model authority transition, never called by a verifier."""
        if admin not in self.patch_admins or principal not in self.membership:
            raise PermissionError('AuthorityRequired')
        if operation not in self.operations: raise KeyError(operation)
        epoch = self.auth_epochs.setdefault((principal, operation), 0)
        g = Grant(principal, self.membership[principal], operation, epoch)
        self.grants.add(g)
        return g

    def authorized(self, g: Grant) -> bool:
        return (g in self.grants and self.membership.get(g.principal) == g.incarnation
                and self.auth_epochs.get((g.principal, g.operation)) == g.auth_epoch)

    def revoke(self, admin: str, principal: str, operation: str) -> None:
        if admin not in self.patch_admins: raise PermissionError('AuthorityRequired')
        k = (principal, operation)
        self.auth_epochs[k] = self.auth_epochs.get(k, 0) + 1
        # Old grants stay as historical data. Validity uses the current epoch.

    def leave(self, admin: str, principal: str) -> None:
        if admin not in self.patch_admins: raise PermissionError('AuthorityRequired')
        # Even numbers are not special: retirement is represented by absence.
        old = self.membership.pop(principal)
        self._retired_membership = getattr(self, '_retired_membership', {})
        self._retired_membership[principal] = max(old, self._retired_membership.get(principal, -1))

    def rejoin(self, admin: str, principal: str) -> None:
        if admin not in self.patch_admins: raise PermissionError('AuthorityRequired')
        if principal in self.membership: raise ValueError('AlreadyActive')
        retired = getattr(self, '_retired_membership', {})
        self.membership[principal] = retired.get(principal, -1) + 1

    def request(self, grant: Grant, args: tuple[Any, ...] = ()) -> Request:
        if not self.authorized(grant): raise PermissionError('StaleOrMissingGrant')
        op = self.operations[grant.operation]
        if tuple(value_type(v) for v in args) != tuple(t for _, t, _ in op.source.parameters):
            raise StaticError('ArgumentTypeMismatch')
        pi = (grant.principal, grant.incarnation)
        n = self.next_counter.get(pi, 0)
        self.next_counter[pi] = n + 1
        key = (*pi, n)
        req = Request(key, grant.operation, op.fingerprint, args, grant,
                      tuple(sorted((x, self.cell_incarnations[x])
                                   for x in op.reads | {op.source.target})))
        self.pending[key] = req
        return req

    def serve(self, req: Request) -> Outcome:
        # This is the semantic model's serialized service transition. It is
        # not evidence that a Rust/QUIC implementation linearizes here.
        old = self.ledger.get(req.key)
        if old is not None:
            return Outcome('Duplicate' if old.request == req else 'KeyConflict', req)
        if self.pending.get(req.key) != req:
            return Outcome('UnboundRequest', req)
        op = self.operations.get(req.operation)
        if op is None or op.fingerprint != req.fingerprint:
            result = Outcome('StaleProgram', req)
        elif req.grant.operation != req.operation or not self.authorized(req.grant):
            result = Outcome('AuthorityRejected', req)
        elif any(self.cell_incarnations.get(x) != e or x not in self.schema
                 for x, e in req.cell_incarnations):
            result = Outcome('StaleLifetime', req)
        else:
            env = dict(zip((n for n, _, _ in op.source.parameters), req.args))
            if not evaluate(op.source.guard, self.store, env):
                result = Outcome('PreconditionFailed', req)
            elif op.source.kind == 'sample' and self.schema[op.source.target].label > self.clearances.get(req.grant.principal, 0):
                result = Outcome('VisibilityDenied', req)
            else:
                val = evaluate(op.source.body, self.store, env)
                target = op.source.target
                if op.source.kind == 'write':
                    self.store[target] = val
                    self.versions[target] += 1
                    self.writes[target] += 1
                    # Reply for write is unit, not a copy of private owner data.
                    result = Outcome('Success', req, None, True)
                else:
                    result = Outcome('Success', req, val, False)
        self.ledger[req.key] = result
        self.assert_wf()
        return result

    def receive(self, outcome: Outcome) -> str:
        # A retained actual outcome, not arbitrary input metadata, is needed.
        if self.ledger.get(outcome.request.key) != outcome:
            return 'UnboundOutcome'
        if not self.authorized(outcome.request.grant):
            return 'AuthorityRejected'
        key = outcome.request.key
        if key in self.received: return 'DuplicateResult'
        if self.pending.get(key) != outcome.request: return 'NoPendingRequest'
        self.received[key] = outcome
        del self.pending[key]
        return outcome.kind

    def relevant_inflight(self, scope: set[str], operation_names: set[str] | None = None) -> bool:
        names = operation_names or set()
        for req in self.pending.values():
            if req.operation in names or scope.intersection(x for x, _ in req.cell_incarnations):
                return True
        return False

    def prepare_versions(self, scope: set[str]) -> dict[str, tuple[int, int]]:
        """Bind both the object incarnation and its local value/structure version.

        A local version may reset on a freshly created object; incarnation
        prevents an ABA-shaped stale patch from silently crossing recreation.
        This method name is retained for this research model's tests only.
        """
        return {x: (self.cell_incarnations[x], self.versions[x]) for x in scope}

    def replace_operation(self, admin: str, replacement: Operation,
                          stamps: Mapping[str, tuple[int, int]]) -> str:
        if admin not in self.patch_admins: return 'AuthorityRejected'
        old = self.operations.get(replacement.name)
        if old is None: return 'UnknownOperation'
        try: new = compile_operation(replacement, self.schema)
        except StaticError as exc: return str(exc)
        scope = set(old.reads | new.reads | {old.source.target, new.source.target})
        if set(stamps) != scope: return 'IncompleteFootprint'
        if any((self.cell_incarnations.get(x), self.versions.get(x)) != stamps[x] for x in scope): return 'StalePreparation'
        if self.relevant_inflight(scope, {replacement.name}): return 'AffectedRequestPending'
        # This concrete model admits only interface-identical body/guard
        # replacement. Stronger semantic contracts are treated by the report,
        # not silently decided by this representation/type check.
        if (replacement.origin, replacement.target, replacement.parameters, replacement.pc, replacement.guard, replacement.kind) != (
                old.source.origin, old.source.target, old.source.parameters, old.source.pc, old.source.guard, old.source.kind):
            return 'InterfaceChangeUnsupported'
        if replacement.revision != old.source.revision + 1: return 'BadRevision'
        self.operations[replacement.name] = new
        self.assert_wf()
        return 'Activated'

    def add_cells(self, admin: str, declarations: Mapping[str, tuple[CellSpec, Any]],
                  new_edges: set[tuple[str, str]]) -> str:
        if admin not in self.patch_admins: return 'AuthorityRejected'
        if set(declarations) & set(self.schema): return 'LiveIdentifierCollision'
        if any(b not in declarations for _, b in new_edges): return 'NotDownstreamAddition'
        schema = {**self.schema, **{x: s for x, (s, _) in declarations.items()}}
        store = {**self.store, **{x: v for x, (_, v) in declarations.items()}}
        if not typed_store(schema, store): return 'InitializerTypeMismatch'
        try: topological_rank(schema, self.edges | new_edges)
        except StaticError as exc: return str(exc)
        for x in {a for a, _ in new_edges if a in self.schema}:
            self.versions[x] += 1
        for x in declarations:
            self.cell_incarnations[x] = self.cell_incarnations.get(x, -1) + 1
            self.versions[x] = 0; self.writes[x] = 0
        self.schema, self.store, self.edges = schema, store, self.edges | new_edges
        self.assert_wf()
        return 'Activated'

    def install_module(self, admin: str,
                       declarations: Mapping[str, tuple[CellSpec, Any]],
                       sources: tuple[Operation, ...],
                       new_edges: set[tuple[str, str]]) -> str:
        """Atomic checked ADDITION of state and source-derived operations.

        The entire candidate is checked before any mutation. This model has
        explicit trusted patch authority; importing checked code mints no grant.
        Existing names/interfaces cannot be shadowed. No real distributed
        commit, loader security or arbitrary module calculus is claimed.
        """
        if admin not in self.patch_admins:
            return 'AuthorityRejected'
        if set(declarations) & set(self.schema):
            return 'LiveIdentifierCollision'
        names = [op.name for op in sources]
        if len(names) != len(set(names)) or set(names) & set(self.operations):
            return 'OperationCollision'
        if any(b not in declarations for _, b in new_edges):
            return 'NotDownstreamAddition'
        schema = {**self.schema, **{x: spec for x, (spec, _) in declarations.items()}}
        store = {**self.store, **{x: value for x, (_, value) in declarations.items()}}
        try:
            if not typed_store(schema, store):
                return 'InitializerTypeMismatch'
            topological_rank(schema, self.edges | new_edges)
            compiled = {op.name: compile_operation(op, schema) for op in sources}
        except StaticError as exc:
            return str(exc)
        except TypeError:
            return 'InitializerTypeMismatch'
        # No partial acceptance: all source/schema checks have completed.
        for x in {a for a, _ in new_edges if a in self.schema}:
            self.versions[x] += 1
        for x in declarations:
            self.cell_incarnations[x] = self.cell_incarnations.get(x, -1) + 1
            self.versions[x] = 0
            self.writes[x] = 0
        self.schema, self.store = schema, store
        self.edges |= new_edges
        self.operations.update(compiled)
        self.assert_wf()
        return 'Installed'

    def detach(self, admin: str, seeds: set[str]) -> str:
        if admin not in self.patch_admins: return 'AuthorityRejected'
        if not seeds <= set(self.schema): return 'UnknownCell'
        gone = descendants(seeds, self.edges)
        affected = {n for n, op in self.operations.items()
                    if gone.intersection(op.reads | {op.source.target})}
        if self.relevant_inflight(gone, affected): return 'AffectedRequestPending'
        for x in gone:
            self.retired_cells.add((x, self.cell_incarnations[x]))
            del self.schema[x]; del self.store[x]
        for n in affected: del self.operations[n]
        # Retire effect-use authority as well; a future operation with the
        # same spelling must not reactivate the old grant.
        for pi, op in list(self.auth_epochs):
            if op in affected: self.auth_epochs[(pi, op)] += 1
        self.edges = {(a, b) for a, b in self.edges if a not in gone and b not in gone}
        self.assert_wf()
        return 'Detached'

    def reparent(self, admin: str, new_edges: set[tuple[str, str]],
                 scope: set[str], stamps: Mapping[str, tuple[int, int]]) -> str:
        if admin not in self.patch_admins: return 'AuthorityRejected'
        changed_endpoints = {x for e in self.edges ^ new_edges for x in e}
        if not changed_endpoints <= scope or set(stamps) != scope: return 'IncompleteFootprint'
        if any((self.cell_incarnations.get(x), self.versions.get(x)) != stamps[x] for x in scope): return 'StalePreparation'
        if self.relevant_inflight(scope): return 'AffectedRequestPending'
        try: topological_rank(self.schema, new_edges)
        except StaticError as exc: return str(exc)
        self.edges = set(new_edges)
        for x in changed_endpoints: self.versions[x] += 1
        self.assert_wf()
        return 'Activated'


def is_closed_cut(events: set[int], edges: set[tuple[int, int]], cut: set[int]) -> bool:
    return cut <= events and all(not (b in cut and a not in cut) for a, b in edges)


def in_flight_at_cut(message_pairs: Mapping[str, tuple[int, int | None]], cut: set[int]) -> set[str]:
    return {m for m, (send, receive) in message_pairs.items()
            if send in cut and (receive is None or receive not in cut)}


def stable_under_updates(reads: frozenset[str], updates: set[str]) -> bool:
    """A sufficient structural rule, not complete semantic stability checking."""
    return not reads.intersection(updates)
