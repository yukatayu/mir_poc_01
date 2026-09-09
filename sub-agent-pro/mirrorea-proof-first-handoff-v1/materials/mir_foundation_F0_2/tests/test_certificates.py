import unittest,copy,itertools
from fractions import Fraction
from dataclasses import replace
from pathlib import Path
from model.language import *
from model.engine import *
from model.certificates import *
ROOT=Path(__file__).resolve().parents[1]
def example(n,name):return parse_check_compile((ROOT/'examples'/n).read_text(),name)
class Certificates(unittest.TestCase):
 def setUp(self):
  self.pool=example('pool.mirx','Pool');self.pos=example('positive.mirx','Positive');self.op=self.pool.operations['Pool.allocate_size'];self.inv={'free_nonnegative':Poly.var('free')};self.post=Poly.var('result')-1
 def test_ring_normalization(self):
  x,y=Poly.var('x'),Poly.var('y');self.assertEqual((x+y)*(x-y),x*x-y*y)
 def test_noncanonical_polynomial_rejected(self):
  with self.assertRaises(CertificateError):Poly(((('x',),Fraction(1)),(('x',),Fraction(-2))))
 def test_normalizer_merges_permuted_monomials(self):
  self.assertEqual(Poly.make({('x','y'):1,('y','x'):2}),Poly.var('x')*Poly.var('y')*3)
 def test_positive_square(self):
  x=Poly.var('x');self.assertTrue(verify(C('square',x-7),(x-7)*(x-7),[]))
 def test_negative_constant_rejected(self):
  with self.assertRaises(CertificateError):check(C('nonnegative_constant','-1'),[])
 def test_no_arbitrary_axiom(self):
  with self.assertRaises(CertificateError):check(C('axiom',Poly.c(-1)),[])
 def test_equality_hypothesis_not_unconditional(self):
  x=Poly.var('x');c=C('equality_multiple',0,Poly.c(-10));self.assertEqual(check(c,[],[x]),x*-10)
  with self.assertRaises(CertificateError):check(c,[],[])
 def test_wrong_conclusion(self):
  with self.assertRaises(CertificateError):verify(C('nonnegative_constant','1'),Poly.c(-1),[])
 def test_total_node_budget(self):
  c=C('nonnegative_constant','1')
  for _ in range(8):c=C('sum',c,c)
  with self.assertRaisesRegex(CertificateError,'Budget'):check(c,[],budget=100)
 def test_source_vcs_all_branches(self):
  op=self.pos.operations['Positive.positive'];ev=produce_evidence(op,{},self.post);self.assertEqual(len(ev.proofs),2);self.assertTrue(verify_export(op,{},self.post,ev))
 def test_missing_branch_rejected(self):
  op=self.pos.operations['Positive.positive'];ev=produce_evidence(op,{},self.post)
  with self.assertRaisesRegex(CertificateError,'MissingPath'):verify_export(op,{},self.post,replace(ev,proofs=ev.proofs[:1]))
 def test_pool_invariant_and_result(self):
  ev=produce_evidence(self.op,self.inv,self.post);self.assertTrue(verify_export(self.op,self.inv,self.post,ev))
  for n in range(1,17):
   r=ProtectedRegistry({'free':16},self.inv);r.admit(self.op,ev,self.post);self.assertEqual(r.invoke(self.op.sig.name,(n,),True),n);self.assertGreaterEqual(r.state['free'],0)
 def test_bad_mutator_cannot_get_proof(self):
  bad=parse_check_compile(self.pool.source.replace('free = free - n','free = free - n - 1'),'Pool').operations[self.op.sig.name]
  with self.assertRaises(CertificateError):produce_evidence(bad,self.inv,self.post)
 def test_code_hash_splicing_rejected(self):
  ev=produce_evidence(self.op,self.inv,self.post)
  bad=parse_check_compile(self.pool.source.replace('return n','return n + 1'),'Pool').operations[self.op.sig.name]
  with self.assertRaisesRegex(CertificateError,'Binding'):verify_export(bad,self.inv,self.post,ev)
 def test_invariant_substitution_rejected(self):
  ev=produce_evidence(self.op,self.inv,self.post)
  with self.assertRaisesRegex(CertificateError,'Binding'):verify_export(self.op,{'stronger':Poly.var('free')-100},self.post,ev)
 def test_evidence_does_not_grant(self):
  ev=produce_evidence(self.op,self.inv,self.post);r=ProtectedRegistry({'free':16},self.inv);r.admit(self.op,ev,self.post)
  with self.assertRaisesRegex(DomainFailure,'AuthorityDenied'):r.invoke(self.op.sig.name,(1,))
  self.assertEqual(r.state['free'],16)
 def test_invariant_requires_initial_model(self):
  with self.assertRaisesRegex(CertificateError,'InitialInvariant'):ProtectedRegistry({'free':-1},self.inv)
 def test_every_mutator_must_be_proved(self):
  e=Engine([self.pool])
  with self.assertRaisesRegex(CertificateError,'UnprovedMutator'):e.protect_owner('PoolOwner',self.inv,{})
 def test_proof_and_network_composition(self):
  iface={o.sig.name:o.sig for p in (self.pool,self.pos) for o in p.operations.values()}
  c=parse_check_compile((ROOT/'examples/client.mirx').read_text(),'Client',iface)
  e=Engine([self.pool,self.pos,c]);ev=produce_evidence(self.op,self.inv,self.post);e.protect_owner('PoolOwner',self.inv,{self.op.sig.name:(ev,self.post)})
  e.grant_all('alice');fid=e.spawn('Client.allocate_positive',(-3,));e.run();e.assert_invariants();self.assertEqual(e.fibers[fid].result,4);self.assertEqual(e.state()['free'],12)
 def test_source_bound_logical_call_bridge(self):
  iface={o.sig.name:o.sig for p in (self.pool,self.pos) for o in p.operations.values()};c=parse_check_compile((ROOT/'examples/client.mirx').read_text(),'Client',iface)
  op=self.pos.operations['Positive.positive'];ev=produce_evidence(op,{},self.post)
  b=prove_call_chain(c,'allocate_positive',op,ev,self.post,self.op,Poly.var('n')-1,C('premise',0));self.assertEqual(b.client_identity,c.identity)
 def test_reassigned_value_not_covered_by_old_fact(self):
  iface={o.sig.name:o.sig for p in (self.pool,self.pos) for o in p.operations.values()};s=(ROOT/'examples/client.mirx').read_text().replace('    return allocate_size(n)','    n = -1\n    return allocate_size(n)');c=parse_check_compile(s,'Client',iface)
  op=self.pos.operations['Positive.positive'];ev=produce_evidence(op,{},self.post)
  with self.assertRaises(CertificateError):prove_call_chain(c,'allocate_positive',op,ev,self.post,self.op,Poly.var('n')-1,C('premise',0))
 def test_local_post_does_not_prove_capacity(self):
  with self.assertRaises(CertificateError):verify(C('premise',0),Poly.var('free')-Poly.var('n'),[Poly.var('n')-1])
 def test_certificate_rules_by_exhaustive_evaluation(self):
  x,y=Poly.var('x'),Poly.var('y');prem=(x+3,y*y)
  cs=[C('premise',0),C('premise',1),C('square',x-y),C('sum',C('premise',0),C('square',y)),C('product',C('premise',0),C('premise',1))]
  for c in cs:
   p=check(c,prem)
   for a,b in itertools.product(range(-5,6),repeat=2):
    env={'x':a,'y':b}
    if all(h.value(env)>=0 for h in prem):self.assertGreaterEqual(p.value(env),0)

class CertificateCaptureRegression(unittest.TestCase):
 def test_local_named_result_cannot_replace_logical_return(self):
  p=parse_check_compile('@owner("O")\ndef f()->int:\n    result:int = 10\n    return -1\n','Capture');op=p.operations['Capture.f']
  with self.assertRaises(CertificateError):produce_evidence(op,{},Poly.var('result')-1)

class CertificateSerialization(unittest.TestCase):
 def test_roundtrip_then_check_not_just_deserialize(self):
  p=example('pool.mirx','Pool');op=p.operations['Pool.allocate_size'];inv={'free_nonnegative':Poly.var('free')};post=Poly.var('result')-1;ev=produce_evidence(op,inv,post);decoded=decode_evidence(encode_evidence(ev));self.assertEqual(ev,decoded);self.assertTrue(verify_export(op,inv,post,decoded))
 def test_duplicate_fields_rejected(self):
  with self.assertRaises(CertificateError):decode_evidence(b'{"operation":"x","operation":"y"}')
 def test_arbitrary_rule_rejected_before_verification(self):
  with self.assertRaises(CertificateError):cert_from_data(['axiom',-1])
 def test_noncanonical_fraction_rejected(self):
  with self.assertRaises(CertificateError):cert_from_data(['nonnegative_constant','02'])
 def test_bad_identity_not_proof(self):
  with self.assertRaises(CertificateError):decode_evidence(b'{}')
 def test_oversize_before_parse(self):
  with self.assertRaisesRegex(CertificateError,'Size'):decode_evidence(b' '*(1024*1024+1))

class InvariantScopeRegression(unittest.TestCase):
 def test_owner_certificate_cannot_claim_another_owners_state(self):
  p=parse_check_compile('a: Cell["A", int]=0\nb: Cell["B", int]=0\n@owner("A")\ndef touch()->int:\n    a=a+1\n    return a\n@owner("B")\ndef decrement()->int:\n    b=b-1\n    return b\n','CrossInvariant');e=Engine([p]);inv={'foreign_nonnegative':Poly.var('b')};op=e.ops['CrossInvariant.touch'];ev=produce_evidence(op,inv)
  with self.assertRaisesRegex(CertificateError,'InvariantOwnerScope'):e.protect_owner('A',inv,{op.sig.name:(ev,None)})

class BridgeSourceBindingRegression(unittest.TestCase):
 def test_mutated_ast_is_not_source_authority(self):
  pool=example('pool.mirx','Pool');pos=example('positive.mirx','Positive');iface={o.sig.name:o.sig for p in (pool,pos) for o in p.operations.values()}
  text=(ROOT/'examples/client.mirx').read_text();good=parse_check_compile(text,'Client',iface);bad=parse_check_compile(text.replace('    return allocate_size(n)','    n=-1\n    return allocate_size(n)'),'Client',iface)
  bad.functions['allocate_positive'].node=good.functions['allocate_positive'].node
  op=pos.operations['Positive.positive'];post=Poly.var('result')-1;ev=produce_evidence(op,{},post)
  with self.assertRaises(CertificateError):prove_call_chain(bad,'allocate_positive',op,ev,post,pool.operations['Pool.allocate_size'],Poly.var('n')-1,C('premise',0))

class ExportEvolution(unittest.TestCase):
 def setUp(self):
  self.p=example('positive.mirx','Positive');self.e=Engine([self.p]);self.e.patch_principals.add('admin');self.op=self.e.ops['Positive.positive'];self.post=Poly.var('result')-1;self.ev=produce_evidence(self.op,{},self.post);self.e.register_export(self.op.sig.name,self.ev,self.post)
 def test_export_cannot_silently_lose_guarantee(self):
  bad=parse_check_compile('@owner("Compute")\ndef positive(x:int)->int:\n    return -1\n','Positive')
  with self.assertRaisesRegex(CertificateError,'ReplacementProof'):self.e.replace_operation('admin',bad,'Positive.positive')
 def test_same_contract_new_proof_allows_new_body(self):
  new=parse_check_compile(self.p.source.replace('+ 1','+ 2'),'Positive');op=new.operations['Positive.positive'];ev=produce_evidence(op,{},self.post);self.e.replace_operation('admin',new,'Positive.positive',(ev,self.post));self.assertEqual(self.e.ops[op.sig.name].identity,op.identity)
 def test_wrong_conclusion_cannot_replace_export(self):
  new=parse_check_compile(self.p.source.replace('+ 1','+ 2'),'Positive');op=new.operations['Positive.positive'];weaker=Poly.var('result')+100;ev=produce_evidence(op,{},weaker)
  with self.assertRaisesRegex(CertificateError,'PostconditionChanged'):self.e.replace_operation('admin',new,'Positive.positive',(ev,weaker))
 def test_export_survives_fresh_restore_via_recheck(self):
  from model.snapshot import capture,restore_as_fresh,InstanceAuthority
  s=capture(self.e);r=restore_as_fresh(s,InstanceAuthority());self.assertIn('Positive.positive',r.exports);self.assertFalse(r.auth.issued)
 def test_untrusted_link_rechecks_source_but_never_issues_grant(self):
  pool=example('pool.mirx','Pool');iface={o.sig.name:o.sig for p in (pool,self.p) for o in p.operations.values()};client=parse_check_compile((ROOT/'examples/client.mirx').read_text(),'Client',iface)
  prove_call_chain(client,'allocate_positive',self.op,self.ev,self.post,pool.operations['Pool.allocate_size'],Poly.var('n')-1,C('premise',0));self.assertFalse(self.e.auth.issued)

class TotalityAndEvolution(unittest.TestCase):
 def test_pool_succeeds_on_declared_body_domain(self):
  p=example('pool.mirx','Pool');op=p.operations['Pool.allocate_size'];inv={'free_nonnegative':Poly.var('free')};post=Poly.var('result')-1;pre=(Poly.var('n')-1,Poly.var('free')-Poly.var('n'));ev=produce_total_evidence(op,inv,post,pre);self.assertTrue(verify_total(op,inv,post,pre,ev))
 def test_guard_not_assumed_when_proving_success(self):
  p=example('positive.mirx','Positive');bad=parse_check_compile(p.source.replace('    if x < 0:','    assert x < 0\n    if x < 0:'),'Positive');op=bad.operations['Positive.positive'];post=Poly.var('result')-1
  # Partial postcondition is true, but total success on all integer inputs is false.
  self.assertTrue(verify_export(op,{},post,produce_evidence(op,{},post)))
  with self.assertRaises(CertificateError):produce_total_evidence(op,{},post,())
 def test_silent_precondition_strengthening_rejected(self):
  p=example('positive.mirx','Positive');e=Engine([p]);e.patch_principals.add('admin');op=e.ops['Positive.positive'];post=Poly.var('result')-1;ev=produce_total_evidence(op,{},post);e.register_export(op.sig.name,ev,post)
  bad=parse_check_compile(p.source.replace('    if x < 0:','    assert x < 0\n    if x < 0:'),'Positive');b=bad.operations[op.sig.name];partial=produce_evidence(b,{},post)
  with self.assertRaisesRegex(CertificateError,'SuccessDomainChanged'):e.replace_operation('admin',bad,op.sig.name,(partial,post))
 def test_new_body_preserves_total_contract(self):
  p=example('positive.mirx','Positive');e=Engine([p]);e.patch_principals.add('admin');op=e.ops['Positive.positive'];post=Poly.var('result')-1;e.register_export(op.sig.name,produce_total_evidence(op,{},post),post)
  q=parse_check_compile(p.source.replace('+ 1','+ 2'),'Positive');n=q.operations[op.sig.name];ev=produce_total_evidence(n,{},post);e.replace_operation('admin',q,op.sig.name,(ev,post));self.assertEqual(e.ops[op.sig.name].identity,n.identity)
 def test_missing_guard_certificate_rejected(self):
  p=example('pool.mirx','Pool');op=p.operations['Pool.allocate_size'];post=Poly.var('result')-1;pre=(Poly.var('n')-1,Poly.var('free')-Poly.var('n'));ev=produce_total_evidence(op,{},post,pre)
  with self.assertRaisesRegex(CertificateError,'MissingTotality'):verify_total(op,{},post,pre,replace(ev,guard_proofs=()))

class StatefulTotalReplacement(unittest.TestCase):
 def test_preservation_and_total_success_evidence_are_separate(self):
  p=example('pool.mirx','Pool');e=Engine([p]);e.patch_principals.add('admin');op=e.ops['Pool.allocate_size']
  inv={'free_nonnegative':Poly.var('free')};post=Poly.var('result')-1;pre=(Poly.var('n')-1,Poly.var('free')-Poly.var('n'))
  partial=produce_evidence(op,inv,post);e.protect_owner('PoolOwner',inv,{op.sig.name:(partial,post)})
  e.register_export(op.sig.name,produce_total_evidence(op,inv,post,pre),post)
  new=parse_check_compile(p.source.replace('free = free - n','free = free - n + 0'),'Pool');nop=new.operations[op.sig.name]
  np=produce_evidence(nop,inv,post);nt=produce_total_evidence(nop,inv,post,pre)
  e.replace_operation('admin',new,op.sig.name,(nt,post),invariant_evidence=(np,post))
  self.assertEqual(e.ops[op.sig.name].identity,nop.identity);e.assert_invariants()
 def test_failed_invariant_proof_does_not_publish_new_export(self):
  p=example('pool.mirx','Pool');e=Engine([p]);e.patch_principals.add('admin');op=e.ops['Pool.allocate_size']
  inv={'free_nonnegative':Poly.var('free')};post=Poly.var('result')-1;pre=(Poly.var('n')-1,Poly.var('free')-Poly.var('n'))
  ev=produce_evidence(op,inv,post);e.protect_owner('PoolOwner',inv,{op.sig.name:(ev,post)});te=produce_total_evidence(op,inv,post,pre);e.register_export(op.sig.name,te,post)
  new=parse_check_compile(p.source.replace('free = free - n','free = free - n + 0'),'Pool');nop=new.operations[op.sig.name];nt=produce_total_evidence(nop,inv,post,pre)
  with self.assertRaises(CertificateError):e.replace_operation('admin',new,op.sig.name,(nt,post),invariant_evidence=(ev,post))
  self.assertEqual(e.ops[op.sig.name].identity,op.identity);self.assertEqual(e.exports[op.sig.name],(te,post))
 def test_protected_export_context_cannot_be_changed_silently(self):
  p=example('positive.mirx','Positive');e=Engine([p]);op=e.ops['Positive.positive'];post=Poly.var('result')-1;e.register_export(op.sig.name,produce_total_evidence(op,{},post),post)
  with self.assertRaisesRegex(CertificateError,'ContextChange'):e.protect_owner('Compute',{'false':Poly.c(-1)},{})
