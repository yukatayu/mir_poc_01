import unittest
from dataclasses import replace
from model.activation import *
class Activation(unittest.TestCase):
 def test_exhaustive_one_to_three_participants(self):
  for n in (1,2,3):self.assertTrue(explore(n)['safe'])
 def test_each_reachable_state_has_recovery_completion(self):
  for n in (1,2):
   _,states=explore(n,retain_states=True)
   for s in states:
    done=finish_after_recovery(s);self.assertEqual(violations(done),[]);self.assertTrue(all(p in (NEW,ABORTED) for p in done.phase))
 def test_mutant_unilateral_timeout(self):self.assertFalse(explore(2,'timeout-abort')['safe'])
 def test_mutant_volatile_preparation(self):self.assertFalse(explore(2,'volatile-prepare')['safe'])
 def test_mutant_volatile_decision(self):self.assertFalse(explore(2,'volatile-decision')['safe'])
 def test_committed_but_uninformed_participant_is_blocked(self):
  s=initial(2)
  for i in range(2):s=step(s,Action('prepare',i));s=step(s,Action('vote',i))
  s=step(s,Action('decide-commit'));s=step(s,Action('learn',0));s=step(s,Action('install',0))
  self.assertEqual(read(s,0),1);self.assertIsNone(read(s,1));self.assertEqual(abstract_epoch(s),1)
 def test_abort_leaves_original_visible_after_release(self):
  s=step(initial(2),Action('prepare',0));s=step(s,Action('decide-abort'));s=finish_after_recovery(s)
  self.assertEqual([read(s,i) for i in range(2)],[0,0])
 def test_coordinator_loss_does_not_choose_a_decision(self):
  s=step(initial(2),Action('prepare',0));s=step(s,Action('crash',2));self.assertEqual(s.decision,U);self.assertIsNone(read(s,0));self.assertEqual(read(s,1),0)
