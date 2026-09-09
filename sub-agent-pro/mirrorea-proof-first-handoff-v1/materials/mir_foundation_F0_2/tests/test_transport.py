import unittest,copy,random
from dataclasses import replace
from pathlib import Path
from model.language import *
from model.engine import *
ROOT=Path(__file__).resolve().parents[1]
class Transport(unittest.TestCase):
 def make(self,n=1):
  p=parse_check_compile((ROOT/'examples/counter.mirx').read_text(),'Counter');e=Engine([p]);e.grant_all('alice');fid=e.spawn('Counter.session',(n,));return p,e,fid
 def issue(self,e,fid):
  for _ in range(100):
   e.step(fid)
   if e.requests:return e.requests[0]
  self.fail('NoRequest')
 def test_duplicate_no_second_mutation(self):
  p,e,f=self.make();r=self.issue(e,f);e.deliver_request(duplicate=True);e.deliver_request();e.deliver_response(duplicate=True);e.deliver_response();e.run();e.assert_invariants();self.assertEqual(e.state()['stock'],9);self.assertEqual(len(e.shards['B'].log),1)
 def test_loss_is_unknown_not_failure(self):
  p,e,f=self.make();r=self.issue(e,f);e.deliver_request();e.responses.clear()
  with self.assertRaisesRegex(DomainFailure,'Blocked'):e.run()
  self.assertIsNone(e.fibers[f].failure);self.assertIsNotNone(e.fibers[f].waiting);self.assertEqual(e.state()['stock'],9)
 def test_explicit_replay_after_loss(self):
  p,e,f=self.make();r=self.issue(e,f);e.deliver_request();e.responses.clear();e.resend(r.key);e.run();self.assertEqual(e.state()['stock'],9);self.assertEqual(e.fibers[f].result,9)
 def test_crash_before_commit(self):
  p,e,f=self.make();r=self.issue(e,f);e.deliver_request(crash_at='before_commit');e.recover_owner('B');self.assertEqual(e.state()['stock'],10);e.resend(r.key);e.run();self.assertEqual(e.state()['stock'],9)
 def test_crash_after_commit(self):
  p,e,f=self.make();r=self.issue(e,f);e.deliver_request(crash_at='after_commit');e.recover_owner('B');self.assertEqual(e.state()['stock'],9);e.resend(r.key);e.run();self.assertEqual(e.state()['stock'],9);e.assert_invariants()
 def test_client_resume_uses_same_continuation(self):
  p,e,f=self.make();r=self.issue(e,f);e.deliver_request();e.crash_resume_fiber(f);e.deliver_response();e.run();self.assertEqual(e.fibers[f].result,9);e.assert_invariants()
 def test_revoke_before_service(self):
  p,e,f=self.make();r=self.issue(e,f);e.auth.revoke(r.permit);e.run();self.assertEqual(e.state()['stock'],10);self.assertEqual(e.fibers[f].failure,'AuthorityDenied')
 def test_revoke_after_service_does_not_undo(self):
  p,e,f=self.make();r=self.issue(e,f);e.deliver_request();e.auth.revoke(r.permit);e.deliver_response();self.assertEqual(e.state()['stock'],9);self.assertIsNotNone(e.fibers[f].waiting);self.assertIsNone(e.fibers[f].failure)
 def test_changed_request_rejected_before_write(self):
  p,e,f=self.make();r=self.issue(e,f);e.requests[0]=replace(r,args=(5,));e.deliver_request();self.assertEqual(e.state()['stock'],10);self.assertEqual(e.responses[0].failure,'UnboundRequest')
 def test_fabricated_response_rejected(self):
  p,e,f=self.make();r=self.issue(e,f);e.responses.append(Outcome(r,value=999,executed=True));e.deliver_response();self.assertIsNotNone(e.fibers[f].waiting)
 def test_requester_cannot_supply_code(self):
  p,e,f=self.make();r=self.issue(e,f);e.requests[0]=replace(r,code_hash='evil');e.deliver_request();self.assertEqual(e.state()['stock'],10)
 def test_unsettled_request_blocks_replacement(self):
  p,e,f=self.make();self.issue(e,f);e.patch_principals.add('admin')
  q=parse_check_compile(p.source.replace('stock = stock - amount','stock = stock - amount + 0'),'Counter')
  with self.assertRaisesRegex(DomainFailure,'Unsettled'):e.replace_operation('admin',q,'Counter.take')
 def test_settled_replacement_continues(self):
  p,e,f=self.make();e.run();e.patch_principals.add('admin');q=parse_check_compile(p.source.replace('stock = stock - amount','stock = stock - amount + 0'),'Counter');e.replace_operation('admin',q,'Counter.take');f2=e.spawn('Counter.session',(1,));e.run();self.assertEqual(e.fibers[f2].result,8)
 def test_addition_does_not_grant(self):
  p,e,f=self.make();e.run();q=parse_check_compile((ROOT/'examples/extension.mirx').read_text(),'Extension',{'Counter.take':p.operations['Counter.take'].sig});e.patch_principals.add('admin');e.install_addition('admin',q);g=e.spawn('Extension.run_bonus',(1,));e.run();self.assertEqual(e.fibers[g].failure,'AuthorityDenied');self.assertEqual(e.state()['bonus'],0)
 def test_addition_then_authorized_use_and_recovery(self):
  p,e,f=self.make();e.run();q=parse_check_compile((ROOT/'examples/extension.mirx').read_text(),'Extension',{'Counter.take':p.operations['Counter.take'].sig});e.patch_principals.add('admin');e.install_addition('admin',q);e.grant_all('alice');g=e.spawn('Extension.run_bonus',(1,));e.run();self.assertEqual(e.fibers[g].result,108);before=e.state();e.recover_owner('B');e.recover_owner('C');self.assertEqual(e.state(),before)
 def test_unauthorized_patch_no_mutation(self):
  p,e,f=self.make();before=copy.deepcopy(e.state())
  with self.assertRaisesRegex(DomainFailure,'PatchAuthority'):e.install_addition('alice',p)
  self.assertEqual(e.state(),before)
 def test_outside_activity_runs_while_owner_frozen(self):
  p,e,f=self.make();r=self.issue(e,f);e.shards['B'].frozen.add('stock');e.deliver_request();pure=e.spawn('Counter.triangle',(8,))
  for _ in range(1000):
   e.step(pure)
   if e.fibers[pure].done:break
  self.assertEqual(e.fibers[pure].result,36);self.assertIsNotNone(e.fibers[f].waiting)
 def test_two_clients_scheduling_and_duplicates(self):
  p,e,f=self.make(3);g=e.spawn('Counter.session',(3,));rng=random.Random(431)
  for _ in range(3000):
   for fid in rng.sample(list(e.fibers),len(e.fibers)):e.step(fid)
   if e.requests:e.deliver_request(rng.randrange(len(e.requests)),duplicate=rng.randrange(5)==0)
   if e.responses:e.deliver_response(rng.randrange(len(e.responses)),duplicate=rng.randrange(5)==0)
   e.assert_invariants()
   if all(x.done for x in e.fibers.values()):break
  self.assertTrue(all(x.done for x in e.fibers.values()));self.assertEqual(e.state()['stock'],4);self.assertEqual(len(e.shards['B'].log),6)
