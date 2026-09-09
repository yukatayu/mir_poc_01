import unittest,tempfile,struct
from pathlib import Path
from model.journal import *
class JournalTests(unittest.TestCase):
 def test_each_torn_suffix_is_not_a_commit(self):
  a=encode({'a':1});b=encode({'b':2})
  for n in range(len(b)):
   r,end=decode_prefix(a+b[:n]);self.assertEqual(r,[{'a':1}]);self.assertEqual(end,len(a))
  self.assertEqual(decode_prefix(a+b)[0],[{'a':1},{'b':2}])
 def test_complete_corruption_rejected(self):
  b=bytearray(encode({'b':2}));b[5]^=1
  with self.assertRaises(CorruptJournal):decode_prefix(bytes(b))
 def test_oversized_before_allocation(self):
  with self.assertRaises(CorruptJournal):decode_prefix(struct.pack('>I',MAX_FRAME+1))
 def test_real_file_reopen_and_replay(self):
  with tempfile.TemporaryDirectory() as d:
   p=Path(d)/'journal';c=DurableCell(p);self.assertEqual(c.add('a',0,3),3);self.assertEqual(DurableCell(p).add('a',0,3),3);self.assertEqual(DurableCell(p).value,3)
 def test_before_append_crash_does_not_commit(self):
  with tempfile.TemporaryDirectory() as d:
   p=Path(d)/'j';c=DurableCell(p);c.add('a',0,3,'before_append');self.assertEqual(DurableCell(p).value,0)
   with self.assertRaises(RuntimeError):c.add('a',0,3)
 def test_after_append_crash_no_double_execution(self):
  with tempfile.TemporaryDirectory() as d:
   p=Path(d)/'j';c=DurableCell(p);c.add('a',0,3,'after_append');r=DurableCell(p);self.assertEqual(r.value,3);self.assertEqual(r.add('a',0,3),3);self.assertEqual(r.value,3)
 def test_floor_survives_restart(self):
  with tempfile.TemporaryDirectory() as d:
   p=Path(d)/'j';c=DurableCell(p)
   for i in range(10):c.add('a',i,1)
   c.forget_prefix('a',9,9);self.assertEqual(len(c.decisions),0);r=DurableCell(p);self.assertEqual(r.add('a',0,1),{'failure':'ForgottenButFenced'});self.assertEqual(r.value,10);self.assertEqual(r.add('a',10,1),11)
 def test_unsettled_prefix_cannot_be_forgotten(self):
  with tempfile.TemporaryDirectory() as d:
   c=DurableCell(Path(d)/'j')
   with self.assertRaisesRegex(ValueError,'Unsettled'):c.forget_prefix('a',5,3)
 def test_truncated_tail_repaired_before_append(self):
  with tempfile.TemporaryDirectory() as d:
   p=Path(d)/'j';j=Journal(p);j.append({'a':1});p.write_bytes(p.read_bytes()+encode({'b':2})[:7]);j=Journal(p);j.append({'c':3});self.assertEqual(Journal(p).records,[{'a':1},{'c':3}])
 def test_request_identity_collision(self):
  with tempfile.TemporaryDirectory() as d:
   c=DurableCell(Path(d)/'j');c.add('a',0,1);self.assertEqual(c.add('a',0,100),{'failure':'IdentityCollision'});self.assertEqual(c.value,1)
