import unittest,copy
from pathlib import Path
from fractions import Fraction
from model.relations import *
from model.snapshot import *
from model.language import parse_check_compile
from model.engine import Engine
from model.certificates import Poly,produce_evidence
ROOT=Path(__file__).resolve().parents[1]
class RelationsAndRestore(unittest.TestCase):
 def rel(self):
  a,b=Anchor('A',1),Anchor('B',1);r=RelationSystem({'root':Binding((a,b),1)},{'offset':Add(Ref('root'),K(3)),'double':Scale(2,Ref('offset'))});return a,b,r
 def test_shared_relation_late_evaluation(self):
  a,b,r=self.rel();v,lab,sel=r.project({a:Sample(a,5,Fraction(4))},{a,b},5);self.assertEqual(v['double'],14);self.assertEqual(r.bindings['root'].cursor,0)
 def test_gap_does_not_change_semantic_binding(self):
  a,b,r=self.rel()
  with self.assertRaisesRegex(DomainFailure,'PresentationGap'):r.project({}, {a,b},5)
  self.assertEqual(r.bindings['root'].cursor,0)
 def test_invalidation_advances_but_does_not_promote(self):
  a,b,r=self.rel();r.invalidate({b},True);r.invalidate({a,b},True);self.assertEqual(r.bindings['root'].cursor,1)
 def test_stale_anchor_rejected_without_silent_change(self):
  a,b,r=self.rel()
  with self.assertRaisesRegex(DomainFailure,'StaleSemanticBinding'):r.project({a:Sample(a,5,Fraction(4))},{b},5)
  self.assertEqual(r.bindings['root'].cursor,0)
 def test_fresh_reacquire_separate_authority(self):
  a,b,r=self.rel();r.invalidate({b},True)
  with self.assertRaises(DomainFailure):r.reacquire('root',(a,b),2)
  r.reacquire('root',(a,b),2,True);self.assertEqual(r.bindings['root'].cursor,0)
  with self.assertRaisesRegex(DomainFailure,'Reused'):r.reacquire('root',(a,b),2,True)
 def test_incoherent_frontier_rejected(self):
  a,b,r=self.rel()
  with self.assertRaisesRegex(DomainFailure,'Incoherent'):r.project({a:Sample(a,4,Fraction(4))},{a,b},5)
 def test_high_sample_not_released(self):
  a,b,r=self.rel()
  with self.assertRaisesRegex(DomainFailure,'Visibility'):r.project({a:Sample(a,5,Fraction(4),1)},{a,b},5)
 def test_cycle_rejected(self):
  with self.assertRaisesRegex(StaticError,'Cycle'):RelationSystem({}, {'x':Ref('y'),'y':Ref('x')})
 def make_engine(self):
  p=parse_check_compile((ROOT/'examples/counter.mirx').read_text(),'Counter');e=Engine([p]);e.grant_all('alice');fid=e.spawn('Counter.session',(2,));e.run();return e
 def test_snapshot_requires_quiescence(self):
  e=self.make_engine();e.spawn('Counter.session',(1,))
  with self.assertRaisesRegex(DomainFailure,'Quiescent'):capture(e)
 def test_fresh_import_data_no_old_authority(self):
  e=self.make_engine();s=capture(e);r=restore_as_fresh(s,InstanceAuthority());self.assertEqual(r.state(),e.state());self.assertNotEqual(r.run_id,e.run_id);self.assertFalse(r.auth.issued);self.assertFalse(r.issued);self.assertFalse(r.patch_principals)
  f=r.spawn('Counter.session',(1,));r.run();self.assertEqual(r.fibers[f].failure,'AuthorityDenied')
 def test_fresh_import_readmit_then_use(self):
  e=self.make_engine();r=restore_as_fresh(capture(e),InstanceAuthority());r.grant_all('alice');f=r.spawn('Counter.session',(1,));r.run();self.assertEqual(r.fibers[f].result,7)
 def test_snapshot_invalid_type_rejected_before_activation(self):
  e=self.make_engine();s=capture(e);s.values['stock']=False;a=InstanceAuthority()
  with self.assertRaisesRegex(StaticError,'TypeMismatch'):restore_as_fresh(s,a)
  self.assertFalse(a.issued)
 def test_snapshot_rechecks_strong_invariant(self):
  p=parse_check_compile((ROOT/'examples/pool.mirx').read_text(),'Pool');e=Engine([p]);op=e.ops['Pool.allocate_size'];inv={'nonnegative':Poly.var('free')};post=Poly.var('result')-1;ev=produce_evidence(op,inv,post);e.protect_owner('PoolOwner',inv,{op.sig.name:(ev,post)});s=capture(e);s.values['free']=-1
  with self.assertRaisesRegex(ValueError,'InitialInvariant'):restore_as_fresh(s,InstanceAuthority())

class RelationPolicyComposition(unittest.TestCase):
 def test_private_relation_definition_is_not_public_because_anchors_are(self):
  a=Anchor('A',1);r=RelationSystem({'root':Binding((a,),1)},{'offset':Add(Ref('root'),K(100))},{'offset':1})
  with self.assertRaisesRegex(DomainFailure,'VisibilityDenied'):r.project({a:Sample(a,1,Fraction(3),0)},{a},1,observer=0)
  values,labels,_=r.project({a:Sample(a,1,Fraction(3),0)},{a},1,observer=1);self.assertEqual(labels['offset'],1);self.assertEqual(values['offset'],103)
