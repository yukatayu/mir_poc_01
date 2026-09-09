import unittest,itertools
from model.ifc import *
from model.external_effect import *
class IFC(unittest.TestCase):
 def setUp(self):self.schema={'lo':CellSpec(INT,'O',0),'hi':CellSpec(INT,'O',1)}
 def test_direct_high_write_to_low_rejected(self):
  with self.assertRaisesRegex(StaticError,'WriteDown'):check(Q('assign','lo',E('cell','hi')),self.schema)
 def test_implicit_flow_rejected(self):
  c=Q('if',E('le',E('cell','hi'),E('lit',0)),Q('assign','lo',E('lit',0)),Q('assign','lo',E('lit',1)))
  with self.assertRaisesRegex(StaticError,'WriteDown'):check(c,self.schema)
 def test_secret_event_presence_rejected(self):
  c=Q('if',E('le',E('cell','hi'),E('lit',0)),Q('emit',0,E('lit',0)),Q('skip'))
  with self.assertRaisesRegex(StaticError,'DisclosureDown'):check(c,self.schema)
 def test_high_loop_does_not_change_low_trace_when_both_terminate(self):
  c=Q('seq',Q('while',E('le',E('lit',1),E('cell','hi')),Q('seq',Q('emit',1,E('cell','hi')),Q('assign','hi',E('sub',E('cell','hi'),E('lit',1))))),Q('emit',0,E('cell','lo')))
  for x in range(8):
   for y in range(8):
    a,ta=run(c,self.schema,{'lo':5,'hi':x});b,tb=run(c,self.schema,{'lo':5,'hi':y});self.assertEqual(a['lo'],b['lo']);self.assertEqual(low_view(ta),low_view(tb))
 def test_projected_identifiers_do_not_reveal_secret_count(self):
  self.assertEqual(low_view([(1,3),(1,5),(0,7)]),[(0,7)])
 def test_selected_program_family_two_run_checks(self):
  cs=[Q('emit',0,E('cell','lo')),Q('assign','hi',E('add',E('cell','hi'),E('cell','lo'))),Q('seq',Q('assign','lo',E('add',E('cell','lo'),E('lit',1))),Q('emit',0,E('cell','lo'))),Q('if',E('le',E('cell','hi'),E('lit',0)),Q('emit',1,E('lit',11)),Q('emit',1,E('lit',22)))]
  for c in cs:
   for lo,x,y in itertools.product(range(-2,3),repeat=3):
    a,ta=run(c,self.schema,{'lo':lo,'hi':x});b,tb=run(c,self.schema,{'lo':lo,'hi':y});self.assertEqual(a['lo'],b['lo']);self.assertEqual(low_view(ta),low_view(tb))
class Effects(unittest.TestCase):
 def test_indistinguishable_execution_and_nonexecution_after_crash(self):
  a=EffectState();a=act(act(act(a,'start'),'crash'),'recover')
  b=EffectState();b=act(act(act(act(b,'start'),'call'),'crash'),'recover')
  self.assertEqual(visible_knowledge(a),visible_knowledge(b));self.assertEqual((a.calls,b.calls),(0,1))
 def test_no_retry_after_ambiguous_start(self):
  s=act(act(act(act(EffectState(),'start'),'call'),'crash'),'recover')
  for _ in range(10):s=act(s,'retry')
  self.assertEqual(s.calls,1);self.assertIsNone(s.requester_result)
 def test_retained_result_replayed_without_new_call(self):
  s=EffectState()
  for a in ('start','call','return','retain','crash','recover','retry','retry'):s=act(s,a)
  self.assertEqual(s.calls,1);self.assertEqual(s.requester_result,42)
 def test_revoked_result_not_released(self):
  s=EffectState()
  for a in ('start','call','return','retain','revoke','retry'):s=act(s,a)
  self.assertIsNone(s.requester_result);self.assertEqual(s.calls,1)
 def test_prestart_revocation_prevents_call(self):
  s=act(EffectState(),'revoke')
  with self.assertRaises(ValueError):act(s,'start')
 def test_deliberate_recovery_mutant_duplicates_effect(self):
  s=act(act(act(act(EffectState(),'start'),'call'),'crash'),'recover')
  # Unsafe restart erases the durable Start: a concrete counterexample, not a valid rule.
  s=replace(s,durable='Absent');s=act(act(s,'start'),'call');self.assertEqual(s.calls,2)

class RelationalRefinementWarning(unittest.TestCase):
 def test_trace_inclusion_alone_does_not_preserve_noninterference(self):
  abstract={0:{(0,),(1,)},1:{(0,),(1,)}}
  concrete={0:{(0,)},1:{(1,)}}
  self.assertTrue(all(concrete[h]<=abstract[h] for h in (0,1)))
  self.assertEqual(abstract[0],abstract[1]);self.assertNotEqual(concrete[0],concrete[1])
