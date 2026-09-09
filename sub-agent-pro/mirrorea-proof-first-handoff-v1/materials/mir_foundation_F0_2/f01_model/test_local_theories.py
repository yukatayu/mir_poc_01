import json
from pathlib import Path
import unittest
from resource_theory import ExclusiveRegions
from core import E,evaluate,infer,INT,stable_under_updates

class LocalTheories(unittest.TestCase):
    def test_interval_export_consumed_by_resource_theory(self):
        # These are two different assertion interpretations, connected by
        # the boundary contract n>0. SMT VCs 11/12 certify the arithmetic.
        expr=E('add',E('if',E('le',E('var','x'),E('lit',-1)),
                       E('neg',E('var','x')),E('var','x')),E('lit',1))
        self.assertEqual(infer(expr,{}, {'x':(INT,0)}).ty,INT)
        heap=ExclusiveRegions();heap.authorize('alice')
        for x in range(-100,101):
            n=evaluate(expr,{}, {'x':x})
            t=heap.allocate('alice',n)
            self.assertEqual(t.length,abs(x)+1)
        heap.assert_separated()

    def test_positive_proof_does_not_grant_allocation(self):
        heap=ExclusiveRegions()
        with self.assertRaises(PermissionError):heap.allocate('alice',abs(-9)+1)

    def test_split_is_disjoint_and_consumes_old_token(self):
        heap=ExclusiveRegions();heap.authorize('alice')
        t=heap.allocate('alice',10);a,b=heap.split('alice',t,4)
        self.assertEqual(a.start+a.length,b.start)
        with self.assertRaises(PermissionError):heap.split('alice',t,4)
        heap.assert_separated()

    def test_move_is_not_new_holder_policy_grant(self):
        heap=ExclusiveRegions();heap.authorize('alice')
        t=heap.allocate('alice',2);u=heap.move('alice',t,'bob')
        with self.assertRaises(PermissionError):heap.release('bob',u)
        heap.authorize('bob');heap.release('bob',u)
        with self.assertRaises(PermissionError):heap.release('alice',t)

    def test_double_release_rejected(self):
        heap=ExclusiveRegions();heap.authorize('alice')
        t=heap.allocate('alice',2);heap.release('alice',t)
        with self.assertRaises(PermissionError):heap.release('alice',t)

    def test_modal_contract_tracks_the_rely_boundary(self):
        state={'config':5,'counter':0}
        e=E('cell','config')
        reads=frozenset({'config'})
        self.assertTrue(stable_under_updates(reads,{'counter'}))
        proof_value=evaluate(e,state)
        for _ in range(100):
            state['counter']+=1
            self.assertEqual(evaluate(e,state),proof_value)
        self.assertFalse(stable_under_updates(reads,{'config'}))
        state['config']=6
        self.assertNotEqual(evaluate(e,state),proof_value)

if __name__=='__main__':
    r=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromModule(__import__(__name__)))
    Path(__file__).with_name('local_theory_results.json').write_text(json.dumps({'tests':r.testsRun,'failures':len(r.failures),'errors':len(r.errors),'successful':r.wasSuccessful()},indent=2)+'\n')
    raise SystemExit(not r.wasSuccessful())
