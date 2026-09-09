from __future__ import annotations
import unittest
from itertools import product, combinations
from dataclasses import replace
from copy import deepcopy
from model.support import *
from model.policy import *
from model.transactions import Store, ABSENT, validate_new_edge
from model.kernel import Kernel, fingerprint, canonical
from base.language import parse_check_compile
from base.certificates import Poly, produce_evidence, produce_total_evidence, CertificateError

SOURCE='''stock: Cell["C", int] = 10
@owner("C")
def use(n: int) -> int:
    assert 0 <= n
    assert n <= stock
    stock = stock - n
    return stock
@task("A")
def run(n: int) -> int:
    return use(n)
'''


def setup(label=0):
    k=Kernel()
    a=k.admin_node('locus','A');c=k.admin_node('locus','C')
    base=k.admin_node('module','X',support=All(Ref(a),Ref(c)))
    m=k.admin_node('member','alice',support=Ref(a),principal='alice')
    y=k.admin_node('module','Y',label,support=All(Ref(base),Ref(c)))
    k.admin_issuer('identity');k.admin_issuer('owner')
    p=Policy('use',0,All(Ref('identity'),Ref('permission')),(('identity',Need('identity','member')),('permission',Need('owner','operate'))),label)
    k.admin_policy(p)
    parsed=parse_check_compile(SOURCE,'Y');inv={'nonnegative':Poly.var('stock')}
    ev=produce_evidence(parsed.operations['Y.use'],inv)
    mapping=k.admin_install_program(SOURCE,'Y',y,{'A':a,'C':c},{'stock':label},{'Y.use':'use'},{'Y.use':(inv,ev)})
    c1=k.admin_claim('identity','alice','member',{'Y.use'},{y},label)
    c2=k.admin_claim('owner','alice','operate',{'Y.use'},{y},label)
    return k, {'a':a,'c':c,'base':base,'member':m,'module':y,'stock':mapping['stock'],'claims':(c1,c2)}


def pending(k,x,n=3,label=0):
    aid=k.start('Y','run','alice',(n,),x['claims'],label)
    for _ in range(10):
        r=k.local_step(aid)
        if r:return aid,r
    raise AssertionError('NoGeneratedRequest')

class SupportTests(unittest.TestCase):
    def test_cross_graph_cycle_has_no_bootstrap(self):
        forms={'module':Ref('grant'),'grant':Ref('member'),'member':Ref('module')}
        self.assertEqual(derive(forms,set(forms)),(frozenset(),{}))
    def test_or_cycle_with_external_root(self):
        forms={'root':Top(),'a':Any(Ref('root'),Ref('b')),'b':Ref('a')}
        live,rank=derive(forms,set(forms));self.assertEqual(live,set(forms))
        edges=set()
        for k,f in forms.items():edges|={(p,k) for p in witness(f,live,rank,rank[k])}
        dag_order(set(forms),edges)
    def test_unknown_support_rejected(self):
        with self.assertRaises(Invalid):derive({'a':Ref('missing')},{'a'})
    def test_every_3node_positive_formula_model(self):
        names=('a','b','c');choices=[Top(),Bot()]+[Ref(n) for n in names]+[All(Ref('a'),Ref('b')),Any(Ref('b'),Ref('c'))]
        checked=0
        for fs in product(choices,repeat=3):
            forms=dict(zip(names,fs))
            for mask in range(8):
                eligible={names[i] for i in range(3) if mask>>i&1}
                live,rank=derive(forms,eligible)
                # Independent finite characterization as intersection of all
                # pre-fixed points of F(X)=X U enabled-supported(X).
                prefixes=[]
                for m in range(8):
                    x={names[i] for i in range(3) if m>>i&1}
                    if all(not holds(forms[k],x) or k in x for k in eligible):prefixes.append(x)
                least=set.intersection(*prefixes)
                self.assertEqual(live,least)
                for k in live:self.assertTrue(holds(forms[k],{n for n,r in rank.items() if r<rank[k]}))
                for disabled in eligible:
                    self.assertLessEqual(derive(forms,eligible-{disabled})[0],live)
                checked+=1
        self.assertEqual(checked,2744)

class PolicyTests(unittest.TestCase):
    def setUp(self):
        self.a=Authority();self.a.add_issuer('I');self.a.add_issuer('P')
        self.p=Policy('p',0,All(Ref('id'),Ref('right')),(('id',Need('I','member')),('right',Need('P','operate'))))
        self.a.install_policy(self.p)
        self.ctx=Context('alice','m1','write','x','inst','r1',(2,),'c1','contract1')
        self.c1=self.a.issue('I','alice','m1','member',{'write'},{'x'})
        self.c2=self.a.issue('P','alice','m1','operate',{'write'},{'x'})
    def test_joint_context(self):
        ev=self.a.evaluate('p',self.ctx,(self.c1,self.c2));self.assertEqual(self.a.revalidate(ev,self.ctx),ev)
    def test_mixed_principal_cannot_compose(self):
        b=self.a.issue('P','bob','mb','operate',{'write'},{'x'})
        with self.assertRaises(Invalid):self.a.evaluate('p',self.ctx,(self.c1,b))
    def test_exact_context_all_fields(self):
        ev=self.a.evaluate('p',self.ctx,(self.c1,self.c2))
        for field in self.ctx.__dataclass_fields__:
            val=(99,) if field=='arguments' else 'foreign'
            with self.subTest(field=field),self.assertRaises(Invalid):self.a.revalidate(ev,replace(self.ctx,**{field:val}))
    def test_revocation_and_epoch(self):
        ev=self.a.evaluate('p',self.ctx,(self.c1,self.c2));self.a.revoked.add(self.c2.key)
        with self.assertRaises(Invalid):self.a.revalidate(ev,self.ctx)
        self.a.revoked.clear();self.a.retire_issuer_epoch('I')
        with self.assertRaises(Invalid):self.a.revalidate(ev,self.ctx)
    def test_policy_replacement_invalidates_evidence(self):
        ev=self.a.evaluate('p',self.ctx,(self.c1,self.c2));self.a.install_policy(replace(self.p,version=1))
        with self.assertRaises(Invalid):self.a.revalidate(ev,self.ctx)
    def test_unissued_claim(self):
        with self.assertRaises(Invalid):self.a.evaluate('p',self.ctx,(replace(self.c1,principal='bob'),self.c2))
    def test_any_is_explicit_audited_branch(self):
        self.a.install_policy(replace(self.p,version=1,expression=Any(Ref('right'),Ref('id'))))
        ev=self.a.evaluate('p',self.ctx,(self.c1,));self.assertEqual(ev.branch[:2],('any',1));self.a.revalidate(ev,self.ctx)
    def test_empty_policy_not_vacuous_authority(self):
        for f in (Top(),All(),Any(),All(Ref('id'),Top())):
            with self.subTest(f=f),self.assertRaises(Invalid):self.a.install_policy(replace(self.p,version=1,expression=f))
    def test_private_evidence_cannot_decide_public_policy(self):
        c=self.a.issue('I','alice','m1','member',{'write'},{'x'},1)
        with self.assertRaises(Invalid):self.a.evaluate('p',self.ctx,(c,self.c2))

class TrackedValidationTests(unittest.TestCase):
    def test_absent_read_blocks_phantom(self):
        s=Store();token,prep=s.prepare('alice',{'output':1},lambda v:v.get('permit'))
        s.write({'permit':'new'})
        with self.assertRaisesRegex(Invalid,'StalePreparation'):s.commit(token,prep,'alice',lambda *_:True)
    def test_aba_blocks_equal_value(self):
        s=Store({'x':0});token,p=s.prepare('a',{'x':1},lambda v:v.get('y'))
        s.write({'y':7});s.write({'y':ABSENT})
        with self.assertRaises(Invalid):s.commit(token,p,'a',lambda *_:True)
    def test_unrelated_update_preserves_preparation(self):
        s=Store({'a':1});t,p=s.prepare('u',{'b':2},lambda v:v.require('a'));s.write({'z':9})
        self.assertEqual(s.commit(t,p,'u',lambda *_:True),1)
    def test_registered_delta_cannot_be_spliced(self):
        s=Store();t,p=s.prepare('u',{'b':2},lambda v:True)
        with self.assertRaises(Invalid):s.commit(t,replace(p,writes=(('b',999),)),'u',lambda *_:True)
    def test_commit_requires_current_authority(self):
        s=Store();t,p=s.prepare('u',{'b':2},lambda v:True)
        with self.assertRaisesRegex(Invalid,'CurrentAuthorizationDenied'):s.commit(t,p,'u',lambda *_:False)
        self.assertEqual(s.values,{})
    def test_disjoint_endpoints_still_cyclic(self):
        # b->c,d->a initially. a->b and c->d individually safe, jointly cyclic.
        s=Store({'out/b':('c',),'out/d':('a',)})
        t1,p1=s.prepare('u',{'out/a':('b',)},lambda v:validate_new_edge(v,'a','b'))
        t2,p2=s.prepare('u',{'out/c':('d',)},lambda v:validate_new_edge(v,'c','d'))
        s.commit(t1,p1,'u',lambda *_:True)
        with self.assertRaisesRegex(Invalid,'StalePreparation'):s.commit(t2,p2,'u',lambda *_:True)
        with self.assertRaisesRegex(Invalid,'Cycle'):s.prepare('u',{'out/c':('d',)},lambda v:validate_new_edge(v,'c','d'))

class IntegrationTests(unittest.TestCase):
    def test_source_generated_two_layer_call(self):
        k,x=setup();a,r=pending(k,x);self.assertEqual(k.data.requests[r].site,('Y','run',1))
        k.serve(r);k.consume(a);k.drive(a)
        self.assertEqual(k.data.activities[a].result,7);self.assertEqual(k.data.cells[x['stock']].value,7);k.audit()
    def test_proof_does_not_grant(self):
        k,x=setup();a=k.start('Y','run','alice',(3,),());k.local_step(a)
        before=fingerprint(k.data)
        with self.assertRaisesRegex(Invalid,'PolicyDenied'):k.local_step(a)
        self.assertEqual(fingerprint(k.data),before)
    def test_retired_member_rejects_pending_without_mutation(self):
        k,x=setup();a,r=pending(k,x);k.admin_retire(x['member']);k.serve(r)
        self.assertEqual(k.data.requests[r].status,'Rejected');self.assertEqual(k.data.cells[x['stock']].value,10)
    def test_retired_module_rejects_pending(self):
        k,x=setup();a,r=pending(k,x);k.admin_retire(x['base']);k.serve(r)
        self.assertNotIn(x['module'],k.data.live);self.assertNotIn(x['stock'],k.data.live)
        self.assertEqual(k.data.requests[r].status,'Rejected')
    def test_fresh_member_does_not_reuse_old_claims(self):
        k,x=setup();k.admin_retire(x['member']);fresh=k.admin_node('member','alice',support=Ref(x['a']),principal='alice')
        self.assertNotEqual(fresh,x['member']);a=k.start('Y','run','alice',(1,),x['claims']);k.local_step(a)
        with self.assertRaisesRegex(Invalid,'PolicyDenied'):k.local_step(a)
    def test_revocation_before_serve(self):
        k,x=setup();a,r=pending(k,x);k.admin_revoke(x['claims'][1]);k.serve(r)
        self.assertEqual(k.data.cells[x['stock']].value,10)
    def test_revocation_after_serve_prevents_release_not_rollback(self):
        k,x=setup();a,r=pending(k,x);k.serve(r);k.admin_revoke(x['claims'][1]);k.consume(a)
        self.assertEqual(k.data.cells[x['stock']].value,7);self.assertEqual(k.data.requests[r].status,'ReleaseDenied')
        self.assertTrue(any(e.kind=='Served' for e in k.data.events))
    def test_duplicate_no_second_mutation(self):
        k,x=setup();a,r=pending(k,x);k.serve(r)
        with self.assertRaisesRegex(Invalid,'DuplicateRequest'):k.serve(r)
        self.assertEqual(k.data.cells[x['stock']].value,7)
    def test_domain_rejection_can_be_consumed(self):
        k,x=setup();a,r=pending(k,x,30);k.serve(r);k.consume(a)
        self.assertEqual(k.data.cells[x['stock']].value,10);self.assertEqual(k.data.activities[a].failure,'PreconditionFailed')
    def test_checked_replacement_changes_code(self):
        k,x=setup();new=SOURCE.replace('stock = stock - n','stock = stock - n + 1');p=parse_check_compile(new,'Y');inv={'nonnegative':Poly.var('stock')}
        ev=produce_evidence(p.operations['Y.use'],inv);old=k.data.operations['Y.use'].code
        k.admin_replace('Y',new,{'Y.use':ev});self.assertNotEqual(k.data.operations['Y.use'].code,old)
        a,r=pending(k,x);k.drive(a);self.assertEqual(k.data.activities[a].result,8)
    def test_pending_replacement_rejected(self):
        k,x=setup();a,r=pending(k,x)
        with self.assertRaisesRegex(Invalid,'PendingOperation'):k.admin_replace('Y',SOURCE,{})
    def test_unproven_replacement_rejected(self):
        k,x=setup()
        with self.assertRaisesRegex(Invalid,'MissingReplacementProof'):k.admin_replace('Y',SOURCE,{})
    def test_schema_changes_not_silently_accepted(self):
        k,x=setup()
        with self.assertRaises(Exception):k.admin_replace('Y',SOURCE.replace('int] = 10','bool] = True'),{})
    def test_same_kind_cycle_rejected_atomically(self):
        k,x=setup();before=fingerprint(k.data)
        with self.assertRaisesRegex(Invalid,'Cycle'):k.admin_reparent(x['base'],Ref(x['module']))
        self.assertEqual(fingerprint(k.data),before)
    def test_high_parent_cannot_gate_low_existence(self):
        k,x=setup();secret=k.admin_node('anchor','private',1);before=fingerprint(k.data)
        with self.assertRaisesRegex(Invalid,'HighSupportToLow'):k.admin_node('state','low',0,Ref(secret))
        self.assertEqual(fingerprint(k.data),before)

class RelationIntegrationTests(unittest.TestCase):
    def setUp(self):
        self.k,self.x=setup();k,x=self.k,self.x
        self.primary=k.admin_node('anchor','shoulder',support=Ref(x['member']))
        self.fallback=k.admin_node('anchor','stable',support=Ref(x['base']))
        k.admin_policy(Policy('rel',0,All(Ref('id'),Ref('right')),(('id',Need('identity','member')),('right',Need('owner','relate')))))
        self.binding=k.admin_binding('attachment',x['module'],(self.primary,self.fallback),'rel')
        self.relclaim=k.admin_claim('owner','alice','relate',{'reacquire'},{self.binding})
        self.idclaim=k.admin_claim('identity','alice','member',{'reacquire'},{self.binding})
    def test_gap_does_not_change_semantics(self):
        before=fingerprint(self.k.data)
        self.assertEqual(self.k.present(self.binding,{}),('PresentationGap',None));self.assertEqual(fingerprint(self.k.data),before)
    def test_member_retirement_cascades_into_binding(self):
        self.k.admin_retire(self.x['member']);b=self.k.data.bindings[self.binding]
        self.assertEqual(b.cursor,1);self.assertIn(self.fallback,self.k.data.live)
    def test_module_retirement_makes_binding_unavailable(self):
        self.k.admin_retire(self.x['base']);self.assertEqual(self.k.present(self.binding,{}),('Unavailable',None))
    def test_reacquire_requires_actual_two_layer_permission(self):
        with self.assertRaises(Invalid):self.k.reacquire('alice',self.binding,(self.idclaim,))
        self.k.reacquire('alice',self.binding,(self.idclaim,self.relclaim));self.assertEqual(self.k.data.bindings[self.binding].lineage,1)
    def test_stale_sample_rejected_after_reacquire(self):
        self.k.reacquire('alice',self.binding,(self.idclaim,self.relclaim))
        self.assertEqual(self.k.present(self.binding,{self.primary:(0,12)}),('PresentationGap',None))
        self.assertEqual(self.k.present(self.binding,{self.primary:(1,12)}),('Value',12))
    def test_fresh_membership_no_implicit_repromotion(self):
        self.k.admin_retire(self.x['member']);self.k.admin_node('member','alice',support=Ref(self.x['a']),principal='alice')
        self.assertEqual(self.k.data.bindings[self.binding].cursor,1)
    def test_hidden_fallback_cannot_leak_via_public_binding(self):
        high=self.k.admin_node('anchor','secret',1)
        with self.assertRaisesRegex(Invalid,'BindingInformationFlow'):self.k.admin_binding('bad',self.x['module'],(self.primary,high),'rel')

class SnapshotObserverTests(unittest.TestCase):
    def test_snapshot_pending_and_revocation_recovered_together(self):
        k,x=setup();a,r=pending(k,x);cp=k.checkpoint();k.admin_revoke(x['claims'][1]);head=k.journal[-1][0];tail=deepcopy(k.journal[cp.position+1:]);k.recover(cp,tail,head);k.serve(r)
        self.assertEqual(k.data.cells[x['stock']].value,10);self.assertEqual(k.data.requests[r].status,'Rejected')
    def test_old_snapshot_cannot_remove_current_suffix(self):
        k,x=setup();cp=k.checkpoint();k.admin_revoke(x['claims'][1]);before=fingerprint(k.data)
        with self.assertRaisesRegex(Invalid,'MissingCommittedSuffix'):k.recover(cp,[],k.journal[-1][0])
        self.assertEqual(fingerprint(k.data),before)
    def test_incomplete_manifest_rejected(self):
        k,x=setup();cp=k.checkpoint()
        with self.assertRaisesRegex(Invalid,'IncompleteOrForeignCheckpoint'):k.recover(replace(cp,components=cp.components[:-1]),[],cp.digest)
    def test_snapshot_tamper_rejected(self):
        k,x=setup();cp=k.checkpoint();bad=deepcopy(cp);bad.image.cells[x['stock']].value=-999
        with self.assertRaises(Invalid):k.recover(bad,[],cp.digest)
    def test_suffix_tamper_rejected(self):
        k,x=setup();cp=k.checkpoint();k.admin_revoke(x['claims'][1]);tail=deepcopy(k.journal[cp.position+1:]);tail[-1][2].auth.revoked.clear()
        with self.assertRaisesRegex(Invalid,'BrokenDurablePrefix'):k.recover(cp,tail,k.journal[-1][0])
    def test_retained_serve_not_reexecuted_after_recovery(self):
        k,x=setup();a,r=pending(k,x);cp=k.checkpoint();k.serve(r);k.recover(cp,deepcopy(k.journal[cp.position+1:]),k.journal[-1][0])
        with self.assertRaises(Invalid):k.serve(r)
        k.consume(a);k.drive(a);self.assertEqual(k.data.activities[a].result,7)
    def test_passive_observer_exact_state_erasure(self):
        k,x=setup();k.admin_observer('alice',0);other=deepcopy(k)
        a,r=pending(k,x);a2,r2=pending(other,x)
        for _ in range(12):k.observe('alice',3)
        k.serve(r);other.serve(r2);k.observe('alice',0)
        k.consume(a);other.consume(a2);k.drive(a);other.drive(a2)
        self.assertEqual(canonical(k.data),canonical(other.data))
    def test_high_events_do_not_evict_low_observations(self):
        k,x=setup();k.admin_observer('alice',0);before=k.observe('alice',2)
        for i in range(30):k.admin_node('anchor','hidden'+str(i),1)
        self.assertEqual(k.observe('alice',2),before)
    def test_revoked_membership_cannot_observe(self):
        k,x=setup();k.admin_observer('alice',0);k.admin_retire(x['member'])
        with self.assertRaises(Invalid):k.observe('alice')
    def test_paired_private_values_same_public_trace(self):
        k,x=setup();k.admin_observer('alice',0);h=k.admin_node('module','Private',1,Ref(x['base']));other=deepcopy(k)
        for engine,value in ((k,1),(other,9001)):
            src=f'''secret: Cell["C", int] = {value}\n@owner("C")\ndef read_secret() -> int:\n    return secret\n@task("A")\ndef high() -> int:\n    return read_secret()\n'''
            engine.admin_policy(Policy('private',0,Ref('right'),(('right',Need('owner','private')),),1))
            engine.admin_install_program(src,'Secret',h,{'A':x['a'],'C':x['c']},{'secret':1},{'Secret.read_secret':'private'})
            claim=engine.admin_claim('owner','alice','private',{'Secret.read_secret'},{h},1)
            aid=engine.start('Secret','high','alice',(),(claim,),1);engine.drive(aid)
            low,r=pending(engine,x);engine.drive(low)
        self.assertEqual(k.observe('alice',999),other.observe('alice',999))
        self.assertEqual(k.data.cells[x['stock']].value,7)


class PreparedControlIntegrationTests(unittest.TestCase):
    def setUp(self):
        self.k,self.x=setup();k,x=self.k,self.x
        self.parent=k.admin_node('module','Another',support=Ref(x['a']))
        k.admin_policy(Policy('patch',0,All(Ref('id'),Ref('owner')),(('id',Need('identity','member')),('owner',Need('owner','edit')))))
        self.claims=(k.admin_claim('identity','alice','member',{'reparent'},{x['module']}),k.admin_claim('owner','alice','edit',{'reparent'},{x['module']}))
    def prepare(self):return self.k.prepare_reparent('alice',self.x['module'],Ref(self.parent),'patch',self.claims)
    def test_honest_preparation_and_current_auth_commit(self):
        p=self.prepare();self.k.commit_reparent(p,'alice');self.assertEqual(self.k.data.nodes[self.x['module']].support,Ref(self.parent));self.k.audit()
    def test_unrelated_progress_does_not_invalidate(self):
        p=self.prepare();a,r=pending(self.k,self.x);self.k.drive(a);self.k.commit_reparent(p,'alice');self.assertEqual(self.k.data.cells[self.x['stock']].value,7)
    def test_auth_revocation_after_prepare_blocks_commit(self):
        p=self.prepare();self.k.admin_revoke(self.claims[1]);before=fingerprint(self.k.data)
        with self.assertRaisesRegex(Invalid,'PolicyDenied'):self.k.commit_reparent(p,'alice')
        self.assertEqual(fingerprint(self.k.data),before)
    def test_reachable_dependency_change_blocks_commit(self):
        p=self.prepare();self.k.admin_reparent(self.parent,Ref(self.x['base']))
        with self.assertRaisesRegex(Invalid,'StalePreparation'):self.k.commit_reparent(p,'alice')
    def test_spliced_support_not_permitted(self):
        p=self.prepare()
        with self.assertRaisesRegex(Invalid,'UnboundPatch'):self.k.commit_reparent(replace(p,support=Top()),'alice')
    def test_retire_parent_does_not_silently_adopt_new_incarnation(self):
        p=self.prepare();self.k.admin_retire(self.parent);self.k.admin_node('module','Another',support=Ref(self.x['a']))
        with self.assertRaisesRegex(Invalid,'StalePreparation'):self.k.commit_reparent(p,'alice')
    def test_full_checkpoint_retains_preparation_and_latest_denial(self):
        p=self.prepare();cp=self.k.checkpoint();self.k.admin_revoke(self.claims[1]);self.k.recover(cp,deepcopy(self.k.journal[cp.position+1:]),self.k.journal[-1][0])
        self.assertIn(p.identity,self.k.data.preparations)
        with self.assertRaisesRegex(Invalid,'PolicyDenied'):self.k.commit_reparent(p,'alice')
    def test_policy_replacement_invalidates_prepared_edit(self):
        p=self.prepare();self.k.admin_policy(replace(self.k.data.auth.policies['patch'],version=1))
        with self.assertRaisesRegex(Invalid,'StalePolicy'):self.k.commit_reparent(p,'alice')
    def test_foreign_principal(self):
        p=self.prepare()
        with self.assertRaisesRegex(Invalid,'PatchPrincipalNotCurrent'):self.k.commit_reparent(p,'bob')

class BoundaryRegressionTests(unittest.TestCase):
    def test_old_expected_head_not_accepted_from_caller(self):
        k,x=setup();cp=k.checkpoint();k.admin_revoke(x['claims'][1])
        with self.assertRaisesRegex(Invalid,'UntrustedRecoveryHead'):k.recover(cp,[],cp.digest)
    def test_cross_kind_cycle_is_inactive_not_self_authorizing(self):
        k=Kernel();a=k.admin_node('member','a',principal='a');b=k.admin_node('module','b',support=Ref(a));k.admin_reparent(a,Ref(b))
        self.assertNotIn(a,k.data.live);self.assertNotIn(b,k.data.live)
    def test_initial_high_context_cannot_start_public_activity(self):
        k,x=setup();highmember=k.admin_node('member','alice',1,Ref(x['a']),principal='alice')
        with self.assertRaisesRegex(Invalid,'ActivityContextLabel'):k.start('Y','run','alice',(1,),x['claims'],0)
    def test_total_certificate_preconditions_cannot_be_smuggled(self):
        k,x=setup();bad=SOURCE.replace('    assert 0 <= n\n','').replace('    assert n <= stock\n','')
        p=parse_check_compile(bad,'Y');inv={'nonnegative':Poly.var('stock')}
        cert=produce_total_evidence(p.operations['Y.use'],inv,None,(Poly.var('stock')-Poly.var('n'),))
        with self.assertRaises(CertificateError):k.admin_replace('Y',bad,{'Y.use':cert})
    def test_proof_for_other_code_rejected(self):
        k,x=setup();other=SOURCE.replace('stock = stock - n','stock = stock - n + 1');p=parse_check_compile(other,'Y');inv={'nonnegative':Poly.var('stock')};wrong=produce_evidence(p.operations['Y.use'],inv)
        with self.assertRaises(CertificateError):k.admin_replace('Y',SOURCE,{'Y.use':wrong})

class DerivedRelationTests(unittest.TestCase):
    def build(self):
        from model.kernel import RelationExpr, Sample
        k,x=setup();k.admin_observer('alice',0)
        left=k.admin_node('anchor','left',support=Ref(x['member']));right=k.admin_node('anchor','right',support=Ref(x['base']))
        k.admin_policy(Policy('relation',0,Ref('right'),(('right',Need('owner','relate')),)))
        b=k.admin_binding('bound',x['module'],(left,right),'relation')
        double=k.admin_relation('double',x['module'],RelationExpr('scale',(2,RelationExpr('ref',(b,)))))
        result=k.admin_relation('result',x['module'],RelationExpr('add',(RelationExpr('ref',(double,)),RelationExpr('const',(3,)))))
        return k,x,left,right,b,double,result
    def test_relation_dag_project_not_absolute_stream(self):
        from model.kernel import Sample
        k,x,l,r,b,d,out=self.build();before=fingerprint(k.data)
        self.assertEqual(k.project('alice',out,{l:Sample(l,0,'frame',4)},'frame'),11)
        self.assertEqual(fingerprint(k.data),before)
    def test_authorized_observer_remains_when_other_member_retires(self):
        from model.kernel import Sample
        k,x,l,r,b,d,out=self.build()
        observer=k.admin_node('member','viewer',support=Ref(x['c']),principal='viewer');k.admin_observer('viewer',0)
        k.admin_retire(x['member'])
        self.assertEqual(k.data.bindings[b].cursor,1)
        self.assertEqual(k.project('viewer',out,{r:Sample(r,0,'frame',8)},'frame'),19)
    def test_cycle_rejected(self):
        from model.kernel import RelationExpr
        k,x,l,r,b,d,out=self.build();before=fingerprint(k.data)
        with self.assertRaisesRegex(Invalid,'Cycle'):k.admin_relation_update(d,RelationExpr('ref',(out,)))
        self.assertEqual(fingerprint(k.data),before)
    def test_context_mismatch_is_gap_not_fallback(self):
        from model.kernel import Sample
        k,x,l,r,b,d,out=self.build();before=fingerprint(k.data)
        with self.assertRaisesRegex(Invalid,'PresentationGap'):k.project('alice',out,{l:Sample(l,0,'old',4)},'now')
        self.assertEqual(fingerprint(k.data),before)
    def test_whole_component_checkpoint_includes_expression_graph(self):
        from model.kernel import Sample
        k,x,l,r,b,d,out=self.build();cp=k.checkpoint();k.recover(cp,[],cp.digest)
        self.assertEqual(k.project('alice',out,{l:Sample(l,0,'f',6)},'f'),15)
    def test_lower_label_cannot_reference_private_relation(self):
        from model.kernel import RelationExpr
        k,x,l,r,b,d,out=self.build();high=k.admin_relation('hidden',x['module'],RelationExpr('const',(93,)),1)
        with self.assertRaisesRegex(Invalid,'RelationInformationFlow'):k.admin_relation('leak',x['module'],RelationExpr('ref',(high,)),0)
    def test_no_observer_grant_no_view(self):
        from model.kernel import Sample
        k,x,l,r,b,d,out=self.build()
        with self.assertRaisesRegex(Invalid,'ObserverDenied'):k.project('outsider',out,{l:Sample(l,0,'f',6)},'f')
    def test_observer_clearance_blocks_private_relation(self):
        from model.kernel import RelationExpr
        k,x,l,r,b,d,out=self.build();h=k.admin_relation('secret',x['module'],RelationExpr('const',(99,)),1)
        with self.assertRaisesRegex(Invalid,'ObservationDenied'):k.project('alice',h,{},'f')

class AuthorizationFallbackTests(unittest.TestCase):
    def setUp(self):
        self.k,self.x=setup();k,x=self.k,self.x
        self.anchor=k.admin_node('anchor','external',support=Ref(x['base']));self.fallback=k.admin_node('anchor','local',support=Ref(x['base']))
        k.admin_policy(Policy('anchor',0,All(Ref('id'),Ref('use')),(('id',Need('identity','member')),('use',Need('owner','anchor')))))
        self.c1=k.admin_claim('identity','alice','member',{'anchor_use'},{self.anchor});self.c2=k.admin_claim('owner','alice','anchor',{'anchor_use'},{self.anchor})
        self.access=k.access_anchor('alice',self.anchor,'anchor',(self.c1,self.c2))
        k.admin_policy(Policy('reacquire',0,Ref('edit'),(('edit',Need('owner','edit')),)))
        self.b=k.admin_binding('link',x['module'],(self.access,self.fallback),'reacquire')
        self.edit=k.admin_claim('owner','alice','edit',{'reacquire'},{self.b})
        k.admin_observer('alice',0)
    def test_revoked_capability_triggers_semantic_fallback(self):
        k=self.k;k.admin_revoke(self.c2)
        self.assertNotIn(self.access,k.data.live);self.assertIn(self.anchor,k.data.live)
        self.assertEqual(k.data.bindings[self.b].cursor,1)
    def test_policy_change_invalidates_access_without_retiring_anchor(self):
        k=self.k;k.admin_policy(replace(k.data.auth.policies['anchor'],version=1))
        self.assertIn(self.anchor,k.data.live);self.assertNotIn(self.access,k.data.live);self.assertEqual(k.data.bindings[self.b].cursor,1)
    def test_fresh_access_does_not_repromote_until_explicit_reacquire(self):
        k=self.k;k.admin_revoke(self.c2);newclaim=k.admin_claim('owner','alice','anchor',{'anchor_use'},{self.anchor})
        fresh=k.access_anchor('alice',self.anchor,'anchor',(self.c1,newclaim))
        self.assertEqual(k.data.bindings[self.b].cursor,1)
        k.reacquire('alice',self.b,(self.edit,),(fresh,self.fallback))
        self.assertEqual(k.data.bindings[self.b].cursor,0);self.assertEqual(k.data.bindings[self.b].lineage,1)
    def test_snapshot_tail_cannot_resurrect_revoked_reference(self):
        k=self.k;cp=k.checkpoint();k.admin_revoke(self.c2);k.recover(cp,deepcopy(k.journal[cp.position+1:]),k.journal[-1][0])
        self.assertEqual(k.data.bindings[self.b].cursor,1);self.assertNotIn(self.access,k.data.live)
    def test_projection_consumes_checked_access_not_permission_boolean(self):
        from model.kernel import Sample
        k=self.k;self.assertEqual(k.project('alice',self.b,{self.anchor:Sample(self.anchor,0,'f',8)},'f'),8)
        k.admin_revoke(self.c2)
        self.assertEqual(k.project('alice',self.b,{self.fallback:Sample(self.fallback,0,'f',3)},'f'),3)
    def test_new_current_membership_retires_old_generation(self):
        k=self.k;old=self.x['member'];fresh=k.admin_node('member','alice',support=Ref(self.x['a']),principal='alice')
        self.assertNotIn(old,k.data.live);self.assertIn(fresh,k.data.live);self.assertNotIn(self.access,k.data.live)

class AdditionalScopeTests(unittest.TestCase):
    def test_fallback_event_from_actual_normalization(self):
        t=AuthorizationFallbackTests();t.setUp();t.k.admin_revoke(t.c2)
        self.assertTrue(any(e.kind=='SemanticFallback' and e.payload[0]==t.b and e.payload[-2:]==(0,1) for e in t.k.data.events))
    def test_private_claims_do_not_change_public_claim_ids(self):
        a=Authority();a.add_issuer('root');b=deepcopy(a)
        for _ in range(10):a.issue('root','high','mh','p',{'act'},{'x'},1)
        c1=a.issue('root','low','ml','p',{'act'},{'x'},0);c2=b.issue('root','low','ml','p',{'act'},{'x'},0)
        self.assertEqual(c1,c2)
    def test_totality_contract_cannot_be_downgraded_to_partial(self):
        source='''x: Cell["B", int] = 1\n@owner("B")\ndef f() -> int:\n    x=x+1\n    return x\n'''
        k=Kernel();l=k.admin_node('locus','B');m=k.admin_node('module','T',support=Ref(l));k.admin_issuer('r');k.admin_policy(Policy('p',0,Ref('r'),(('r',Need('r','use')),)))
        p=parse_check_compile(source,'T');inv={'n':Poly.var('x')};ev=produce_total_evidence(p.operations['T.f'],inv,None)
        k.admin_install_program(source,'T',m,{'B':l},{'x':0},{'T.f':'p'},{'T.f':(inv,ev)})
        partial=produce_evidence(p.operations['T.f'],inv)
        with self.assertRaisesRegex(Invalid,'CannotSilentlyWeakenTotality'):k.admin_replace('T',source,{'T.f':partial})

class ReviewFindings(unittest.TestCase):
    def test_retired_source_module_must_not_consume_live_result(self):
        k,x=setup();aid,rid=pending(k,x);k.serve(rid);k.admin_retire(x['module']);k.consume(aid)
        self.assertEqual(k.data.requests[rid].status,'ReleaseDenied')
        self.assertEqual(k.data.cells[x['stock']].value,7)

class SupportCertificateTests(unittest.TestCase):
    def test_independent_certificate_accepts_generated_closures(self):
        names=('a','b','c');choices=[Top(),Bot()]+[Ref(n) for n in names]+[All(Ref('a'),Ref('b')),Any(Ref('b'),Ref('c'))]
        for fs in product(choices,repeat=3):
            forms=dict(zip(names,fs))
            for mask in range(8):
                eligible={names[i] for i in range(3) if mask>>i&1}
                live,rank=derive(forms,eligible);self.assertTrue(check_closure(forms,eligible,live,rank))
    def test_circular_fake_proof_rejected(self):
        forms={'a':Ref('b'),'b':Ref('a')}
        with self.assertRaisesRegex(Invalid,'CircularSupportCertificate'):check_closure(forms,set(forms),set(forms),{'a':0,'b':1})
    def test_reject_everything_is_not_complete(self):
        with self.assertRaisesRegex(Invalid,'OmittedDerivableSupport'):check_closure({'root':Top()},{'root'},set(),{})
    def test_certificate_from_old_eligibility_rejected(self):
        forms={'a':Top(),'b':Ref('a')};live,ranks=derive(forms,set(forms))
        with self.assertRaises(Invalid):check_closure(forms,{'b'},live,ranks)
    def test_inflated_derivation_ranks_rejected(self):
        with self.assertRaises(Invalid):check_closure({'a':Top()},{'a'},{'a'},{'a':7})
    def test_low_closure_independent_of_high_formula(self):
        low={'public-root':Top(),'public-child':Ref('public-root')}
        for highf in (Top(),Bot(),Ref('private'),Ref('public-root'),Any(Ref('public-child'),Ref('private'))):
            forms={**low,'private':highf}
            for enabled in (set(forms),set(low)):
                full=derive(forms,enabled)[0]
                self.assertEqual(full & set(low),derive(low,set(low))[0])

class ControlDependenceReview(unittest.TestCase):
    def test_write_target_existence_labels_acknowledgment(self):
        source='''private: Cell["B", int] = 1\n@owner("B")\ndef put() -> int:\n    private=5\n    return 0\n'''
        k=Kernel();l=k.admin_node('locus','B');m=k.admin_node('module','P',support=Ref(l));k.admin_issuer('r');k.admin_policy(Policy('p',0,Ref('r'),(('r',Need('r','use')),)))
        k.admin_install_program(source,'P',m,{'B':l},{'private':1},{'P.put':'p'})
        self.assertEqual(k.data.operations['P.put'].label,1)

class ComponentCompletenessReview(unittest.TestCase):
    def test_undeclared_component_blocks_checkpoint(self):
        k,x=setup();k.data.unregistered_external_effect={'started':True}
        with self.assertRaisesRegex(Invalid,'UndeclaredStateComponent'):k.checkpoint()
    def test_same_name_fresh_anchor_does_not_revive_old_reference(self):
        t=AuthorizationFallbackTests();t.setUp();t.k.admin_retire(t.anchor)
        new=t.k.admin_node('anchor','external',support=Ref(t.x['base']))
        self.assertNotEqual(t.anchor,new);self.assertNotIn(t.access,t.k.data.live);self.assertEqual(t.k.data.bindings[t.b].cursor,1)

class TypedContextReview(unittest.TestCase):
    def test_boolean_and_integer_arguments_are_not_same_context(self):
        t=PolicyTests();t.setUp();original=replace(t.ctx,arguments=(1,));e=t.a.evaluate('p',original,(t.c1,t.c2))
        with self.assertRaisesRegex(Invalid,'AuthorizationContextMismatch'):t.a.revalidate(e,replace(original,arguments=(True,)))

class RepeatedReadReview(unittest.TestCase):
    def test_conflicting_repeat_read_does_not_overwrite_first_stamp(self):
        from model.transactions import View
        store=Store({'x':1});view=View(store,{})
        self.assertEqual(view.get('x'),1)
        store.write({'x':2})
        with self.assertRaisesRegex(Invalid,'UnstablePreparationRead'):
            view.get('x')
    def test_repeat_aba_requires_same_version_not_same_value(self):
        from model.transactions import View
        store=Store({'x':1});view=View(store,{})
        self.assertEqual(view.get('x'),1)
        store.write({'x':2});store.write({'x':1})
        with self.assertRaisesRegex(Invalid,'UnstablePreparationRead'):
            view.get('x')

class AccessRevisionReview(unittest.TestCase):
    def test_anchor_revision_change_invalidates_bound_access(self):
        case=AuthorizationFallbackTests();case.setUp()
        k=case.k
        # The access's κ records the anchor structural revision. Reparenting
        # retains the anchor identity, but must not reuse the old κ silently.
        k.admin_reparent(case.anchor,Ref(case.x['module']))
        self.assertIn(case.anchor,k.data.live)
        self.assertNotIn(case.access,k.data.live)
        self.assertEqual(k.data.bindings[case.b].cursor,1)

class SnapshotTypedEqualityReview(unittest.TestCase):
    def test_checkpoint_bool_int_substitution_rejected(self):
        k,x=setup();aid=k.start('Y','run','alice',(1,),x['claims'])
        cp=k.checkpoint();forged=deepcopy(cp)
        forged.image.activities[aid].env['n']=True
        with self.assertRaisesRegex(Invalid,'UntrustedCheckpoint'):
            k.recover(forged,[],k.journal[-1][0])

class PreparedTypedEqualityReview(unittest.TestCase):
    def test_prepared_int_write_cannot_become_bool(self):
        s=Store();token,prep=s.prepare('alice',{'x':1},lambda v:True)
        forged=replace(prep,writes=(('x',True),))
        with self.assertRaisesRegex(Invalid,'UnboundPreparation'):
            s.commit(token,forged,'alice',lambda *_:True)

if __name__ == '__main__':
    unittest.main()
