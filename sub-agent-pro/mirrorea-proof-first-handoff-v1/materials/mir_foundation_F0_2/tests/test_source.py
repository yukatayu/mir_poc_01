import ast,copy,random,unittest
from dataclasses import replace
from pathlib import Path
from model.language import *
from model.engine import *
ROOT=Path(__file__).resolve().parents[1]
def example(n,name):return parse_check_compile((ROOT/'examples'/n).read_text(),name)
def run(p,name,args=(),grants=True):
 e=Engine([p]);
 if grants:e.grant_all('alice')
 fid=e.spawn(p.name+'.'+name,args);e.run();e.assert_invariants();return e,e.fibers[fid]
class Source(unittest.TestCase):
 def test_loop_and_source_map(self):
  p=example('counter.mirx','Counter');e,f=run(p,'session',(4,));self.assertEqual((f.result,e.state()['stock']),(30,6))
  self.assertTrue(all(row['source_line']>0 for row in p.generated_edges));self.assertEqual(len(p.generated_edges),3)
 def test_recursive_stack(self):
  p=example('counter.mirx','Counter');e,f=run(p,'triangle',(50,));self.assertEqual(f.result,1275)
 def test_structured_parallel(self):
  p=example('counter.mirx','Counter');e,f=run(p,'parallel',(12,));self.assertEqual(f.result,169);self.assertTrue(all(c.done for c in e.fibers.values()))
 def test_direct_assignment_is_owner_rmw(self):
  p=example('counter.mirx','Counter');e,f=run(p,'direct');self.assertEqual(f.result,12)
  self.assertEqual(next(o.sig.owner for o in p.operations.values() if o.kind=='assign'),'B')
 def test_multiple_assignments_on_one_line(self):
  p=parse_check_compile('x: Cell["B", int] = 10\n@task("A")\ndef main() -> int:\n    x = x + 1; x = x + 2\n    return x\n')
  e,f=run(p,'main');self.assertEqual(f.result,13)
 def test_unreachable_suffix_is_not_emitted(self):
  p=parse_check_compile('@task("A")\ndef main()->int:\n    return 1\n    x = not_declared\n')
  e,f=run(p,'main');self.assertEqual(f.result,1)
 def test_early_branch_return(self):
  p=parse_check_compile('@task("A")\ndef main(n:int)->int:\n    if n<0:\n        return 1\n    else:\n        v:int = 2\n    return v\n')
  for n,v in [(-1,1),(0,2)]:self.assertEqual(run(p,'main',(n,))[1].result,v)
 def test_mutual_recursion(self):
  p=parse_check_compile('@task("A")\ndef even(n:int)->bool:\n    if n<=0:\n        return True\n    return odd(n-1)\n@task("A")\ndef odd(n:int)->bool:\n    if n<=0:\n        return False\n    return even(n-1)\n')
  for n in range(12):self.assertEqual(run(p,'even',(n,))[1].result,n%2==0)
 def test_cross_owner_operand_needs_snapshot(self):
  s='x: Cell["B", int]=10\ny: Cell["C", int]=2\n@task("A")\ndef main()->int:\n    x = x + y\n    return x\n'
  with self.assertRaisesRegex(StaticError,'Snapshot'):parse_check_compile(s)
  p=parse_check_compile(s.replace('    x = x + y','    v:int = y\n    x = x + v'));self.assertEqual(run(p,'main')[1].result,12)
 def test_bool_is_not_int(self):
  with self.assertRaises(StaticError):parse_check_compile('@task("A")\ndef main()->int:\n    return True\n')
 def test_possibly_unbound_branch(self):
  with self.assertRaises(StaticError):parse_check_compile('@task("A")\ndef main(b:bool)->int:\n    if b:\n        x:int=1\n    return x\n')
 def test_while_does_not_define_after_zero_iterations(self):
  with self.assertRaises(StaticError):parse_check_compile('@task("A")\ndef main(b:bool)->int:\n    while b:\n        x:int=1\n    return x\n')
 def test_no_arbitrary_host_code(self):
  for s in ['import os','x = open("/etc/passwd")','@task("A")\ndef main()->int:\n    return __import__("os")\n']:
   with self.assertRaises((StaticError,SyntaxError)):parse_check_compile(s)
 def test_owner_loop_rejected(self):
  with self.assertRaises(StaticError):parse_check_compile('@owner("A")\ndef main()->int:\n    while True:\n        pass\n    return 1\n')
 def test_argument_annotations(self):
  with self.assertRaises(StaticError):parse_check_compile('@task("A")\ndef main(n)->int:\n    return n\n')
 def test_deployment_does_not_grant(self):
  p=example('counter.mirx','Counter');e,f=run(p,'session',(1,),False);self.assertEqual(f.failure,'AuthorityDenied');self.assertEqual(e.state()['stock'],10)
 def test_forged_permit_rejected(self):
  a=Authority();p=a.grant('alice','x');fake=replace(p,serial=p.serial+10)
  self.assertFalse(a.allowed(fake,'alice','x'));self.assertFalse(a.allowed(p,'bob','x'))
 def test_source_artifact_tampering_not_used(self):
  p=example('counter.mirx','Counter');p.functions['triangle'].code=[('const','v',999),('return','v')]
  self.assertEqual(run(p,'triangle',(4,))[1].result,10)
 def test_signature_includes_effects(self):
  p=example('counter.mirx','Counter');sig=p.operations['Counter.take'].sig
  bad=replace(sig,effect_bound=())
  q=parse_check_compile('from Counter import take\n@task("A")\ndef main()->int:\n    return take(1)\n','Client',{'Counter.take':bad})
  with self.assertRaisesRegex(StaticError,'InterfaceBindingMismatch'):Engine([p,q])
 def test_independent_import_and_link(self):
  pool=example('pool.mirx','Pool');positive=example('positive.mirx','Positive')
  iface={o.sig.name:o.sig for p in (pool,positive) for o in p.operations.values()}
  c=parse_check_compile((ROOT/'examples/client.mirx').read_text(),'Client',iface)
  e=Engine([pool,positive,c]);e.grant_all('alice');fid=e.spawn('Client.allocate_positive',(-3,));e.run();self.assertEqual(e.fibers[fid].result,4);self.assertEqual(e.state()['free'],12)
 def test_all_owner_writes_staged_on_failure(self):
  s='x: Cell["B", int]=10\n@owner("B")\ndef op()->int:\n    x = 100\n    assert x < 0\n    return x\n@task("A")\ndef main()->int:\n    return op()\n'
  e,f=run(parse_check_compile(s),'main');self.assertEqual(e.state()['x'],10);self.assertEqual(f.failure,'PreconditionFailed')
 def test_unjoined_child_failure_is_not_silent_success(self):
  s='x: Cell["B", int]=0\n@owner("B")\ndef nope()->int:\n    assert x < 0\n    return 1\n@task("A")\ndef child()->int:\n    return nope()\n@task("A")\ndef main()->int:\n    f = spawn(child)\n    return 7\n'
  e,f=run(parse_check_compile(s),'main');self.assertEqual(f.failure,'PreconditionFailed')
 def test_no_shared_mutable_frame_between_spawn(self):
  p=example('counter.mirx','Counter');e,f=run(p,'parallel',(5,));self.assertEqual(f.result,36)
 def test_random_expression_cfg_differential(self):
  rng=random.Random(5121)
  def expr(d):
   if d==0:return rng.choice(['n','0','1','2','-3'])
   if rng.randrange(3)==0:return f'({expr(d-1)} if n < 2 else {expr(d-1)})'
   return f'({expr(d-1)} {rng.choice(["+","-","*"])} {expr(d-1)})'
  for _ in range(150):
   source='@task("A")\ndef main(n:int)->int:\n    return '+expr(3)+'\n';p=parse_check_compile(source)
   for n in (-3,0,5):self.assertEqual(run(p,'main',(n,))[1].result,interpret_activity(p,'main',(n,),p.operations,{}))
 def test_loop_differential(self):
  p=example('counter.mirx','Counter')
  for n in range(11):
   state={'stock':10};want=interpret_activity(p,'session',(n,),p.operations,state);e,f=run(p,'session',(n,));self.assertEqual((f.result,e.state()),(want,state))
