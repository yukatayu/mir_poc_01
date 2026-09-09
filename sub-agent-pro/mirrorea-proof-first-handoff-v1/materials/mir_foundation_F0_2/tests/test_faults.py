import unittest

class JournalUncertainError(unittest.TestCase):
 def test_append_error_after_possible_durable_write_requires_reopen(self):
  from model.journal import DurableCell
  from unittest.mock import patch
  import tempfile
  from pathlib import Path
  with tempfile.TemporaryDirectory() as d:
   path=Path(d)/'j';cell=DurableCell(path);append=cell.journal.append
   def committed_then_error(record):
    append(record)
    raise OSError('simulated outcome loss after append')
   with patch.object(cell.journal,'append',committed_then_error):
    with self.assertRaises(OSError):cell.add('i',1,5)
   with self.assertRaisesRegex(RuntimeError,'ReopenAfterCrash'):cell.add('i',1,5)
   revived=DurableCell(path);self.assertEqual(revived.add('i',1,5),5);self.assertEqual(revived.value,5)
