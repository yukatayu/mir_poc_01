import unittest
from model.language import parse_check_compile, StaticError

class DeclarationBoundary(unittest.TestCase):
 def test_duplicate_parameters_rejected(self):
  with self.assertRaises(StaticError):parse_check_compile('@task("A")\ndef f(x:int,x:int)->int:\n    return x\n','M')
 def test_default_parameter_not_silently_erased(self):
  with self.assertRaises(StaticError):parse_check_compile('@task("A")\ndef f(x:int=3)->int:\n    return x\n','M')
 def test_extra_decorator_not_silently_erased(self):
  with self.assertRaises(StaticError):parse_check_compile('@uninterpreted_policy\n@task("A")\ndef f(x:int)->int:\n    return x\n','M')
 def test_nonstring_cell_owner_rejected(self):
  with self.assertRaises(StaticError):parse_check_compile('x:Cell[3,int]=0\n@task("A")\ndef f()->int:\n    return 0\n','M')
