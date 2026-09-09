import unittest
from pathlib import Path
from model.language import *
from model.engine import *
from model.live_patch import *
from model.snapshot import *
from model.certificates import *
ROOT=Path(__file__).resolve().parents[1]
BASE='x: Cell["A", int]=2\ny: Cell["B", int]=3\n@task("Client")\ndef read_all()->int:\n    a:int = x\n    b:int = y\n    return a+b\n@task("Client")\ndef pure()->int:\n    return 77\n'
def setup():
 p=parse_check_compile(BASE,'Base');e=Engine([p]);e.patch_principals.add('admin');e.grant_all('alice')
 ma=parse_check_compile('x: Cell["A", int]=0\n@owner("A")\ndef migrate()->int:\n    x=x+10\n    return 0\n','MA')
 mb=parse_check_compile('y: Cell["B", int]=0\n@owner("B")\ndef migrate()->int:\n    y=y+20\n    return 0\n','MB')
 patch=LivePatch(e,'admin',{'A':(ma,'MA.migrate',None),'B':(mb,'MB.migrate',None)});return e,patch
class LivePatchTests(unittest.TestCase):
 def test_checked_migration_then_ordinary_code(self):
  e,p=setup();p.complete();f=e.spawn('Base.read_all',());e.run();self.assertEqual(e.fibers[f].result,35);e.assert_invariants()
 def test_partial_install_cannot_expose_old_participant(self):
  e,p=setup()
  for i in range(2):p.advance('prepare',i);p.advance('vote',i)
  p.advance('decide-commit');p.advance('learn',0);p.advance('install',0)
  self.assertEqual(p.visible_object('A','x'),12)
  with self.assertRaisesRegex(DomainFailure,'PatchPending'):p.visible_object('B','y')
  self.assertEqual(p.semantic_values(),{'x':12,'y':23})
 def test_coordinator_crash_after_decision_preserves_fences(self):
  e,p=setup()
  for i in range(2):p.advance('prepare',i);p.advance('vote',i)
  p.advance('decide-commit');p.advance('crash',2)
  with self.assertRaisesRegex(DomainFailure,'PatchPending'):p.visible_object('A','x')
  p.advance('recover',2)
  for i in range(2):p.advance('learn',i);p.advance('install',i)
  e.recover_owner('A');e.recover_owner('B');self.assertEqual(e.state(),{'x':12,'y':23})
 def test_old_request_issued_while_frozen_is_not_new_generation(self):
  e,p=setup()
  for i in range(2):p.advance('prepare',i);p.advance('vote',i)
  f=e.spawn('Base.read_all',())
  for _ in range(20):
   e.step(f)
   if e.requests:break
  old=e.requests[0];self.assertEqual(old.binding_epoch,0)
  p.advance('decide-commit')
  for i in range(2):p.advance('learn',i);p.advance('install',i)
  e.run();self.assertEqual(e.fibers[f].failure,'StaleCode');self.assertEqual(e.state(),{'x':12,'y':23})
 def test_unrelated_activity_progresses_during_fence(self):
  e,p=setup();p.advance('prepare',0);f=e.spawn('Base.pure',());e.run();self.assertEqual(e.fibers[f].result,77)
 def test_invalid_migration_rejected_before_fence(self):
  e,p=setup();bad=parse_check_compile('z: Cell["A", int]=0\n@owner("A")\ndef migrate()->int:\n    z=z+1\n    return 0\n','Bad')
  with self.assertRaises(StaticError):LivePatch(e,'admin',{'A':(bad,'Bad.migrate',None)})
  self.assertFalse(e.shards['A'].frozen)
 def test_version_change_invalidates_prepared_candidate(self):
  e,p=setup();e.shards['A'].versions['x']+=1
  with self.assertRaisesRegex(DomainFailure,'StalePatch'):p.advance('prepare',0)
  self.assertFalse(e.shards['A'].frozen)
 def test_reserved_authority_not_silently_retroactively_revoked(self):
  e,p=setup();p.advance('prepare',0);p.request_revocation();self.assertIn('admin',e.patch_principals)
  p.advance('vote',0);p.advance('prepare',1);p.advance('vote',1);p.advance('decide-commit')
  for i in range(2):p.advance('learn',i);p.advance('install',i)
  self.assertNotIn('admin',e.patch_principals)
 def test_snapshot_preserves_replaced_code_not_only_data(self):
  base=parse_check_compile((ROOT/'examples/counter.mirx').read_text(),'Counter');e=Engine([base]);e.patch_principals.add('admin')
  new=parse_check_compile(base.source.replace('stock = stock - amount','stock = stock - amount - amount'),'Counter');e.replace_operation('admin',new,'Counter.take');s=capture(e);r=restore_as_fresh(s,InstanceAuthority());r.grant_all('alice');f=r.spawn('Counter.session',(1,));r.run();self.assertEqual(r.fibers[f].result,8)
 def test_same_owner_protected_migration_requires_certificate(self):
  pool=parse_check_compile((ROOT/'examples/pool.mirx').read_text(),'Pool');e=Engine([pool]);e.patch_principals.add('admin');op=e.ops['Pool.allocate_size'];inv={'nonnegative':Poly.var('free')};post=Poly.var('result')-1;e.protect_owner('PoolOwner',inv,{op.sig.name:(produce_evidence(op,inv,post),post)})
  mig=parse_check_compile('free: Cell["PoolOwner", int]=0\n@owner("PoolOwner")\ndef migrate()->int:\n    free=free+10\n    return 0\n','M');m=mig.operations['M.migrate']
  with self.assertRaises(CertificateError):LivePatch(e,'admin',{'PoolOwner':(mig,'M.migrate',None)})
  p=LivePatch(e,'admin',{'PoolOwner':(mig,'M.migrate',produce_evidence(m,inv))});p.complete();self.assertEqual(e.state()['free'],26);e.assert_invariants()

class ReplacementSchemaRegression(unittest.TestCase):
 def test_same_signature_does_not_authorize_state_type_change(self):
  old=parse_check_compile('x: Cell["A", int]=0\n@owner("A")\ndef f()->int:\n    x=1\n    return 0\n','M');new=parse_check_compile('x: Cell["A", bool]=False\n@owner("A")\ndef f()->int:\n    x=False\n    return 0\n','M');e=Engine([old]);e.patch_principals.add('admin')
  with self.assertRaisesRegex(StaticError,'ReplacementSchemaMismatch'):e.replace_operation('admin',new,'M.f')

class CompositionalPatchRegressions(unittest.TestCase):
 def test_abort_of_unprepared_patch_does_not_release_another_patch_fence(self):
  e,a=setup();a.advance('prepare',0)
  ma=parse_check_compile('x: Cell["A", int]=0\n@owner("A")\ndef migrate()->int:\n    x=x+1\n    return 0\n','Other')
  b=LivePatch(e,'admin',{'A':(ma,'Other.migrate',None)})
  b.advance('decide-abort');b.advance('learn',0);b.advance('abort-local',0)
  self.assertIn('x',e.shards['A'].frozen)
  with self.assertRaisesRegex(DomainFailure,'PatchPending'):a.visible_object('A','x')
 def test_completed_patch_abstraction_does_not_override_later_owner_execution(self):
  text=BASE+'\n@task("Client")\ndef increment()->int:\n    x=x+1\n    return x\n'
  e=Engine([parse_check_compile(text,'Base')]);e.patch_principals.add('admin');e.grant_all('alice')
  m=parse_check_compile('x: Cell["A", int]=0\n@owner("A")\ndef migrate()->int:\n    x=x+10\n    return 0\n','M')
  p=LivePatch(e,'admin',{'A':(m,'M.migrate',None)});p.complete()
  f=e.spawn('Base.increment',());e.run();self.assertEqual(e.fibers[f].result,13)
  self.assertEqual(p.semantic_values()['x'],13)

class MultiOwnerPatch(unittest.TestCase):
    def setup_pair(self):
        original='''a: Cell["A", int] = 10
b: Cell["B", int] = 20
@owner("A")
def read_a() -> int:
    return a
@owner("B")
def read_b() -> int:
    return b
'''
        migrations='''a: Cell["A", int] = 10
b: Cell["B", int] = 20
@owner("A")
def change_a() -> int:
    a = a + 1
    return a
@owner("B")
def change_b() -> int:
    b = b + 2
    return b
'''
        p=parse_check_compile(original,'Pair');m=parse_check_compile(migrations,'MigratePair')
        e=Engine([p]);e.patch_principals.add('admin')
        patch=LivePatch(e,'admin',{'A':(m,'MigratePair.change_a',None),'B':(m,'MigratePair.change_b',None)})
        return e,patch
    def test_partial_install_uses_shadow_only_for_remaining_prepared_owner(self):
        e,p=self.setup_pair()
        for i in (0,1):p.advance('prepare',i);p.advance('vote',i)
        p.advance('decide-commit');p.advance('learn',0);p.advance('install',0)
        self.assertEqual(e.state(),{'a':11,'b':20})
        self.assertEqual(e.shards['B'].frozen,{'b'})
        self.assertEqual(p.semantic_values(),{'a':11,'b':22})
        # The other participant remains durably prepared through its model crash.
        p.advance('crash',1);p.advance('recover',1)
        self.assertEqual(e.shards['B'].frozen,{'b'})
        p.advance('learn',1);p.advance('install',1)
        self.assertEqual(e.state(),{'a':11,'b':22})
        e.recover_owner('A');e.recover_owner('B')
        self.assertEqual(e.state(),{'a':11,'b':22});self.assertEqual(p.semantic_values(),e.state())
    def test_abort_after_partial_prepare_changes_neither_owner(self):
        e,p=self.setup_pair();p.advance('prepare',0);p.advance('vote',0)
        p.advance('decide-abort')
        for i in (0,1):p.advance('learn',i);p.advance('abort-local',i)
        self.assertEqual(e.state(),{'a':10,'b':20})
        self.assertEqual(e.shards['A'].frozen|e.shards['B'].frozen,set())
        self.assertEqual(p.semantic_values(),e.state())
