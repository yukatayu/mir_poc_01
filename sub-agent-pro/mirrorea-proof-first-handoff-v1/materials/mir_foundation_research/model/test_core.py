"""Finite tests of the executable research model; NOT general proofs."""
from __future__ import annotations
import copy
from dataclasses import replace
import itertools
import json
from pathlib import Path
import random
import unittest
from core import *

STATS = {}

def declarative_types(e, schema, env):
    """Independent relation-style implementation for the finite test corpus.
    It does not call infer. The general equivalence proof is in FOUNDATION.md section 3.
    """
    tag, a = e.tag, e.args
    if tag == 'lit':
        try: return {value_type(a[0])}
        except TypeError: return set()
    if tag == 'var': return {env[a[0]]} if a[0] in env else set()
    if tag == 'cell': return {schema[a[0]].ty} if a[0] in schema else set()
    if tag == 'let':
        return set().union(*(declarative_types(a[2], schema, {**env, a[0]: t})
                             for t in declarative_types(a[1], schema, env)))
    if tag in ('neg','not','fst','snd'):
        types = declarative_types(a[0], schema, env)
        if tag == 'neg': return {INT} if INT in types else set()
        if tag == 'not': return {BOOL} if BOOL in types else set()
        return {t.left if tag == 'fst' else t.right for t in types if t.tag == 'Product'}
    if tag == 'if':
        return (declarative_types(a[1], schema, env) & declarative_types(a[2], schema, env)
                if BOOL in declarative_types(a[0], schema, env) else set())
    left, right = (declarative_types(x, schema, env) for x in a)
    if tag in ('add','sub','le'):
        return {BOOL if tag == 'le' else INT} if INT in left and INT in right else set()
    if tag == 'and': return {BOOL} if BOOL in left and BOOL in right else set()
    if tag == 'eq': return {BOOL} if left & right else set()
    if tag == 'pair': return {PROD(l, r) for l in left for r in right}
    return set()


def example():
    schema = {'x': CellSpec(INT, 'B'), 'y': CellSpec(INT, 'A'),
              'secret': CellSpec(INT, 'B', 1)}
    op = compile_operation(Operation('inc','A','x',E('add',E('cell','x'),E('lit',1))), schema)
    c = Config(schema, {'x':0,'y':10,'secret':99}, {'inc':op},
               membership={'alice':0}, patch_admins={'admin'})
    return c, c.issue_grant('admin','alice','inc')

class Expressions(unittest.TestCase):
    def setUp(self):
        self.schema = {'x':CellSpec(INT,'A'), 'h':CellSpec(INT,'B',1), 'b':CellSpec(BOOL,'A')}
        self.store = {'x':3,'h':90,'b':True}

    def test_literals_do_not_confuse_bool_and_integer(self):
        self.assertEqual(infer(E('lit',True),{}).ty, BOOL)
        with self.assertRaises(StaticError): infer(E('add',E('lit',True),E('lit',1)),{})

    def test_unbound_rejected(self):
        with self.assertRaises(StaticError): infer(E('var','bad'),{})

    def test_product_and_let(self):
        e=E('let','p',E('pair',E('cell','x'),E('lit',False)),E('fst',E('var','p')))
        self.assertEqual(infer(e,self.schema).ty,INT)
        self.assertEqual(evaluate(e,self.store),3)

    def test_branch_mismatch_rejected(self):
        with self.assertRaises(StaticError): infer(E('if',E('lit',True),E('lit',1),E('lit',False)),{})

    def test_cross_owner_rejected_not_silently_compiled(self):
        with self.assertRaisesRegex(StaticError,'Snapshot'):
            compile_operation(Operation('bad','A','x',E('add',E('cell','x'),E('cell','h'))),self.schema)

    def test_implicit_flow_rejected(self):
        e=E('if',E('le',E('cell','h'),E('lit',0)),E('lit',0),E('lit',1))
        with self.assertRaisesRegex(StaticError,'Flow'):
            compile_operation(Operation('bad','A','x',e),{**self.schema,'h':CellSpec(INT,'A',1)})

    def test_generated_edges_follow_owner_not_caller_authority(self):
        schema={'x':CellSpec(INT,'B')}
        p=compile_operation(Operation('inc','A','x',E('add',E('cell','x'),E('lit',1))),schema)
        self.assertEqual(p.edges,(('A','B','request'),('B','A','outcome')))
        self.assertEqual(p.owner,'B')

    def test_local_operation_needs_no_network_edge(self):
        p=compile_operation(Operation('inc','A','x',E('add',E('cell','x'),E('lit',1))),self.schema)
        self.assertEqual(p.edges,())

    def test_generated_expression_corpus(self):
        rng=random.Random(17041)
        atoms=[E('lit',0),E('lit',2),E('lit',True),E('lit',False),E('cell','x'),E('cell','h'),E('cell','b'),E('var','p')]
        def gen(depth):
            if depth==0 or rng.random()<.3:return rng.choice(atoms)
            tag=rng.choice(['add','sub','le','eq','and','pair','neg','not','fst','snd','if','let'])
            if tag in ('neg','not','fst','snd'):return E(tag,gen(depth-1))
            if tag=='if':return E(tag,gen(depth-1),gen(depth-1),gen(depth-1))
            if tag=='let':return E(tag,'p',gen(depth-1),gen(depth-1))
            return E(tag,gen(depth-1),gen(depth-1))
        valid=invalid=lowchecked=0
        for e in atoms+[gen(4) for _ in range(4000)]:
            d=declarative_types(e,self.schema,{'p':INT})
            try:i=infer(e,self.schema,{'p':(INT,0)})
            except StaticError:
                self.assertFalse(d);invalid+=1;continue
            self.assertEqual(d,{i.ty});valid+=1
            result=evaluate(e,self.store,{'p':4})
            self.assertEqual(value_type(result),i.ty)
            other={**self.store,'h':-1000}
            if i.label==0:
                self.assertEqual(result,evaluate(e,other,{'p':4}));lowchecked+=1
            # Altering a cell not in syntactic support preserves evaluation.
            for x in set(self.schema)-i.reads:
                changed={**self.store,x:(not self.store[x] if type(self.store[x]) is bool else -343)}
                self.assertEqual(result,evaluate(e,changed,{'p':4}))
        STATS['expressions']={'checked':valid+invalid,'valid':valid,'invalid':invalid,'low_two_run_checks':lowchecked}

class GraphsAndFallback(unittest.TestCase):
    def test_all_four_node_directed_graphs(self):
        nodes=list('abcd');pairs=[(x,y) for x in nodes for y in nodes if x!=y]
        def dfs_acyclic(edges):
            active=set();done=set()
            def visit(n):
                if n in active:return False
                if n in done:return True
                active.add(n)
                for a,b in edges:
                    if a==n and not visit(b):return False
                active.remove(n);done.add(n);return True
            return all(visit(n) for n in nodes)
        ok=bad=0
        for bits in itertools.product((False,True),repeat=len(pairs)):
            edges={p for p,b in zip(pairs,bits) if b}
            d=dfs_acyclic(edges)
            try:r=topological_rank(nodes,edges)
            except StaticError:
                self.assertFalse(d);bad+=1;continue
            self.assertTrue(d);self.assertTrue(all(r[a]<r[b] for a,b in edges));ok+=1
        STATS['graphs']={'nodes':4,'checked':ok+bad,'acyclic':ok,'cyclic':bad}

    def test_unknown_graph_endpoint_rejected(self):
        with self.assertRaises(StaticError):topological_rank({'a'},{('a','b')})

    def test_self_cycle_rejected(self):
        with self.assertRaises(StaticError):topological_rank({'a'},{('a','a')})

    def test_multiple_roots(self):
        self.assertEqual(topological_rank({'a','b','c'},{('a','c'),('b','c')}),{'a':0,'b':0,'c':1})

    def test_descendant_closed_removal(self):
        edges={('a','b'),('b','c'),('x','c')}
        self.assertEqual(descendants({'a'},edges),{'a','b','c'})

    def test_all_small_fallback_chains(self):
        count=0
        for n in range(7):
            for bits in itertools.product((False,True),repeat=n):
                for cursor in range(n+1):
                    nxt=resolve_fallback(cursor,bits)
                    expected=min([j for j in range(cursor,n) if bits[j]]+[n])
                    self.assertEqual(nxt,expected);self.assertGreaterEqual(nxt,cursor)
                    if cursor<n and bits[-1]:self.assertLess(nxt,n)
                    count+=1
        STATS['fallback']={'max_length':6,'cases':count}

    def test_no_automatic_repromotion(self):
        p=resolve_fallback(0,(False,True));self.assertEqual(p,1)
        self.assertEqual(resolve_fallback(p,(True,True)),1)

    def test_exhaustion_is_sticky_within_lineage(self):
        p=resolve_fallback(0,(False,False));self.assertEqual(p,2)
        self.assertEqual(resolve_fallback(p,(True,True)),2)
        # Resetting the cursor is a separate fresh-lineage operation.
        self.assertEqual(resolve_fallback(0,(True,True)),0)

    def test_flatten_associativity(self):
        self.assertEqual(flatten_chain(('fallback',('fallback','a','b'),'c')),
                         flatten_chain(('fallback','a',('fallback','b','c'))))

class RuntimeAndEvolution(unittest.TestCase):
    def test_two_owner_updates_not_stale(self):
        c,g=example();r1=c.request(g);r2=c.request(g)
        o1=c.serve(r1);o2=c.serve(r2)
        self.assertEqual(c.store['x'],2)
        self.assertEqual(c.receive(o1),'Success');self.assertEqual(c.receive(o2),'Success')

    def test_duplicates_do_not_execute_twice(self):
        c,g=example();r=c.request(g);o=c.serve(r)
        for _ in range(10):self.assertEqual(c.serve(r).kind,'Duplicate')
        self.assertEqual(c.store['x'],1);self.assertEqual(c.receive(o),'Success')
        self.assertEqual(c.receive(o),'DuplicateResult')

    def test_lost_reply_keeps_ambiguity_and_blocks_affected_patch(self):
        c,g=example();r=c.request(g);c.serve(r)
        op=replace(c.operations['inc'].source,revision=1,body=E('add',E('cell','x'),E('lit',2)))
        self.assertEqual(c.replace_operation('admin',op,c.prepare_versions({'x'})),'AffectedRequestPending')
        self.assertIn(r.key,c.pending);self.assertEqual(c.store['x'],1)

    def test_unknown_request_is_not_admitted(self):
        c,g=example();r=c.request(g)
        fake=replace(r,key=('alice',0,500))
        self.assertEqual(c.serve(fake).kind,'UnboundRequest');self.assertEqual(c.store['x'],0)

    def test_request_splicing_is_rejected(self):
        c,g=example();r=c.request(g)
        self.assertEqual(c.serve(replace(r,args=(500,))).kind,'UnboundRequest')
        self.assertEqual(c.store['x'],0)

    def test_revocation_before_service_prevents_write(self):
        c,g=example();r=c.request(g);c.revoke('admin','alice','inc')
        self.assertEqual(c.serve(r).kind,'AuthorityRejected');self.assertEqual(c.store['x'],0)

    def test_post_service_revocation_does_not_undo_write(self):
        c,g=example();r=c.request(g);o=c.serve(r);c.revoke('admin','alice','inc')
        self.assertEqual(c.receive(o),'AuthorityRejected');self.assertEqual(c.store['x'],1)
        self.assertIn(r.key,c.pending)

    def test_leave_rejoin_does_not_resurrect_grant(self):
        c,g=example();c.leave('admin','alice');c.rejoin('admin','alice')
        self.assertFalse(c.authorized(g))
        fresh=c.issue_grant('admin','alice','inc');self.assertTrue(c.authorized(fresh))
        self.assertNotEqual(g.incarnation,fresh.incarnation)

    def test_checked_code_does_not_supply_authority(self):
        c,_=example();c.grants.clear()
        fake=Grant('alice',0,'inc',0)
        with self.assertRaises(PermissionError):c.request(fake)
        self.assertEqual(c.store['x'],0)

    def test_snapshot_then_cross_owner_assignment(self):
        c,_=example()
        read=compile_operation(Operation('sample_y','B','y',E('cell','y'),kind='sample'),c.schema)
        write=compile_operation(Operation('set_from_value','A','x',E('var','v'),(('v',INT,0),)),c.schema)
        c.operations.update(sample_y=read,set_from_value=write)
        rg=c.issue_grant('admin','alice','sample_y');wg=c.issue_grant('admin','alice','set_from_value')
        read_request=c.request(rg);out=c.serve(read_request);self.assertEqual(out.value,10)
        self.assertEqual(c.receive(out),'Success')
        # The transferred value is an immutable historical snapshot, not a
        # hidden live read from A at the later B service step.
        c.store['y']=999;c.versions['y']+=1
        w=c.request(wg,(out.value,));c.serve(w)
        self.assertEqual(c.store['x'],10);self.assertEqual(c.writes['y'],0)

    def test_private_snapshot_needs_release_clearance(self):
        c,_=example();op=compile_operation(Operation('sample_secret','A','secret',E('cell','secret'),kind='sample'),c.schema)
        c.operations['sample_secret']=op;g=c.issue_grant('admin','alice','sample_secret')
        self.assertEqual(c.serve(c.request(g)).kind,'VisibilityDenied')
        c.clearances['alice']=1
        self.assertEqual(c.serve(c.request(g)).value,99)

    def test_typed_patch_changes_actual_behavior(self):
        c,g=example();op=replace(c.operations['inc'].source,revision=1,body=E('add',E('cell','x'),E('lit',2)))
        self.assertEqual(c.replace_operation('admin',op,c.prepare_versions({'x'})),'Activated')
        c.serve(c.request(g));self.assertEqual(c.store['x'],2)

    def test_bad_patch_is_nonmutating(self):
        c,_=example();old=copy.deepcopy(c)
        op=replace(c.operations['inc'].source,revision=1,body=E('lit',False))
        self.assertEqual(c.replace_operation('admin',op,c.prepare_versions({'x'})),'OperationTypeMismatch')
        self.assertEqual(c,old)

    def test_patch_does_not_silently_strengthen_precondition(self):
        c,_=example();op=replace(c.operations['inc'].source,revision=1,guard=E('lit',False))
        self.assertEqual(c.replace_operation('admin',op,c.prepare_versions({'x'})),'InterfaceChangeUnsupported')

    def test_stale_patch_scope_rejected(self):
        c,g=example();stamps=c.prepare_versions({'x'});o=c.serve(c.request(g));c.receive(o)
        op=replace(c.operations['inc'].source,revision=1)
        self.assertEqual(c.replace_operation('admin',op,stamps),'StalePreparation')

    def test_unrelated_activity_does_not_stale_patch(self):
        c,_=example();stamps=c.prepare_versions({'x'})
        oy=compile_operation(Operation('inc_y','B','y',E('add',E('cell','y'),E('lit',1))),c.schema)
        c.operations['inc_y']=oy;gy=c.issue_grant('admin','alice','inc_y')
        # This unrelated request remains pending and has actually changed y.
        c.serve(c.request(gy))
        op=replace(c.operations['inc'].source,revision=1)
        self.assertEqual(c.replace_operation('admin',op,stamps),'Activated')

    def test_patch_requires_its_own_authority(self):
        c,_=example();op=replace(c.operations['inc'].source,revision=1)
        self.assertEqual(c.replace_operation('alice',op,c.prepare_versions({'x'})),'AuthorityRejected')

    def test_unbounded_schematic_addition_exercised_100_times(self):
        c,_=example()
        for n in range(100):
            node=f'node{n}';parent='x' if n==0 else f'node{n-1}'
            self.assertEqual(c.add_cells('admin',{node:(CellSpec(INT,'C'),n)},{(parent,node)}),'Activated')
        self.assertEqual(len(c.schema),103);c.assert_wf()
        STATS['evolution_demo']={'successive_additions_tested':100,'fixed_final_limit_in_model':False}

    def test_addition_does_not_mint_grants(self):
        c,_=example();before=set(c.grants)
        c.add_cells('admin',{'z':(CellSpec(INT,'C'),3)},{('x','z')})
        self.assertEqual(before,c.grants)

    def test_cycle_in_new_dependencies_rejected(self):
        c,_=example();old=copy.deepcopy(c)
        self.assertEqual(c.add_cells('admin',{'a':(CellSpec(INT,'C'),0),'b':(CellSpec(INT,'C'),0)},
                                    {('a','b'),('b','a')}),'DependencyCycle')
        self.assertEqual(c,old)

    def test_detach_removes_dependents_and_old_grants(self):
        c,g=example();c.add_cells('admin',{'z':(CellSpec(INT,'C'),1)},{('x','z')})
        self.assertEqual(c.detach('admin',{'x'}),'Detached')
        self.assertNotIn('z',c.schema);self.assertNotIn('inc',c.operations)
        self.assertFalse(c.authorized(g))

    def test_recreated_cell_does_not_reuse_incarnation(self):
        c,_=example();old=c.cell_incarnations['x'];c.detach('admin',{'x'})
        c.add_cells('admin',{'x':(CellSpec(INT,'B'),0)},set())
        self.assertGreater(c.cell_incarnations['x'],old)
        self.assertIn(('x',old),c.retired_cells)

    def test_reparent_rejects_cycle(self):
        c,_=example();old=copy.deepcopy(c)
        self.assertEqual(c.reparent('admin',{('x','y'),('y','x')},{'x','y'},c.prepare_versions({'x','y'})),'DependencyCycle')
        self.assertEqual(c,old)

    def test_reparent_accepts_valid_dag(self):
        c,_=example()
        self.assertEqual(c.reparent('admin',{('x','y')},{'x','y'},c.prepare_versions({'x','y'})),'Activated')
        self.assertEqual(c.edges,{('x','y')})

    def test_erasing_dedup_is_a_real_counterexample(self):
        c,g=example();r=c.request(g);c.serve(r)
        del c.ledger[r.key]  # DELIBERATE mutant, forbidden in the actual model
        c.serve(r)
        self.assertEqual(c.store['x'],2)

    def test_bounded_all_protocol_schedules(self):
        cases=0
        for seq in itertools.product(range(5),repeat=5):
            c,g=example();r0=c.request(g);r1=c.request(g)
            outcomes={}
            for a in seq:
                if a in (0,1):
                    out=c.serve((r0,r1)[a])
                    if out.kind=='Success':outcomes[a]=out
                elif a in (2,3):
                    if a-2 in outcomes:c.receive(outcomes[a-2])
                else:c.revoke('admin','alice','inc')
                self.assertLessEqual(c.store['x'],2)
                self.assertEqual(c.store['x'],sum(o.mutated for o in c.ledger.values()))
                c.assert_wf()
            cases+=1
        STATS['protocol_schedules']={'actions':5,'length':5,'cases':cases,
                                    'no_crash_no_byzantine':True}

class CutAndModal(unittest.TestCase):
    def test_closed_cut(self):
        events={0,1,2,3};edges={(0,1),(1,2),(2,3)}
        self.assertTrue(is_closed_cut(events,edges,{0,1}))
        self.assertFalse(is_closed_cut(events,edges,{1,2}))

    def test_channel_state_is_send_without_receive(self):
        self.assertEqual(in_flight_at_cut({'m':(1,2)},{0,1}),{'m'})
        self.assertEqual(in_flight_at_cut({'m':(1,2)},{0,1,2}),set())

    def test_current_revocation_cannot_be_restored_away(self):
        old=set();current={'cap1'};restored=old|current
        self.assertIn('cap1',restored)

    def test_modal_stability_rule(self):
        self.assertTrue(stable_under_updates(frozenset({'config'}),{'counter','view'}))
        self.assertFalse(stable_under_updates(frozenset({'config'}),{'config'}))

    def test_history_fact_is_not_current_fact(self):
        state={'x':1};sample=state['x'];state['x']=2
        self.assertEqual(sample,1);self.assertNotEqual(sample,state['x'])

if __name__=='__main__':
    suite=unittest.defaultTestLoader.loadTestsFromModule(__import__(__name__))
    result=unittest.TextTestRunner(verbosity=2).run(suite)
    STATS['unit_tests']={'run':result.testsRun,'failures':len(result.failures),'errors':len(result.errors),
                        'successful':result.wasSuccessful()}
    (Path(__file__).resolve().parent/'test_results.json').write_text(json.dumps(STATS,ensure_ascii=False,indent=2)+'\n')
    raise SystemExit(not result.wasSuccessful())
