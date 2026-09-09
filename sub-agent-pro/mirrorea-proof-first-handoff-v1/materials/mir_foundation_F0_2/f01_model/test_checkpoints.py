import json,itertools,unittest
from pathlib import Path
from checkpoints import complete,satisfies,exhaustive,find_z_path

STATS={}
class Checkpoints(unittest.TestCase):
    def test_Z_cycle_with_acyclic_event_order(self):
        # B sends m2; A receives m2; A sends m1; B receives m1.
        # A has an intermediate checkpoint between its receive and send.
        # B has only initial and after-both checkpoints.
        upper=(2,1)
        deps=((1,1,0,1),(0,2,1,1))
        r=complete(upper,deps,{0:1})
        self.assertIsNone(r.selected)
        self.assertEqual(r.conflict,(0,2,1))
        self.assertEqual(exhaustive(upper,deps,{0:1}),[])
        # The actual event graph has the rank Bsend=0,Arecv=1,Asend=2,Brecv=3.
        self.assertTrue(all(a<b for a,b in [(0,1),(1,2),(2,3),(0,3)]))

    def test_initial_and_later_checkpoints_are_usable(self):
        upper=(2,1);deps=((1,1,0,1),(0,2,1,1))
        self.assertEqual(complete(upper,deps,{0:0}).selected,(0,0))
        self.assertEqual(complete(upper,deps,{0:2}).selected,(2,1))

    def test_missing_sender_checkpoint_rejected(self):
        self.assertIsNone(complete((0,1),((0,1,1,1),),{1:1}).selected)

    def test_all_small_completion_systems(self):
        upper=(2,2)
        choices=[(0,s,1,r) for s in range(1,4) for r in range(1,4)]
        choices += [(1,s,0,r) for s in range(1,4) for r in range(1,4)]
        required=[{},*({p:k} for p in range(2) for k in range(3)),
                  *({0:a,1:b} for a in range(3) for b in range(3))]
        count=unsat=0
        for deps in itertools.product(choices,repeat=2):
            for req in required:
                actual=complete(upper,deps,req)
                expected=exhaustive(upper,deps,req)
                self.assertEqual(actual.selected is None,not expected)
                if expected:
                    self.assertTrue(satisfies(actual.selected,upper,deps,req))
                    self.assertTrue(all(all(a<=b for a,b in zip(actual.selected,c)) for c in expected))
                else:unsat+=1
                count+=1
        STATS.update(systems=count,no_completion=unsat,least_completion=count-unsat,
                     candidate_vectors_per_system=9)

    def test_Z_path_equivalence_independent_graph_search(self):
        upper=(2,2)
        choices=tuple((sp,a,1-sp,b) for sp in range(2) for a in (1,2) for b in (1,2))
        fixed=[{},*({p:k} for p in range(2) for k in range(3)),
               *({0:a,1:b} for a in range(3) for b in range(3))]
        count=paths=0
        for deps in itertools.product(choices,repeat=3):
            for req in fixed:
                witness=find_z_path(upper,deps,req)
                failed=complete(upper,deps,req).selected is None
                self.assertEqual(witness is not None,failed)
                if witness is not None:
                    first,last=deps[witness[0]],deps[witness[-1]]
                    self.assertGreater(first[1],req[first[0]])
                    self.assertLessEqual(last[3],req[last[2]])
                    for left,right in zip(witness,witness[1:]):
                        self.assertEqual(deps[left][2],deps[right][0])
                        self.assertLessEqual(deps[left][3],deps[right][1])
                    paths+=1
                count+=1
        STATS['Z_path_systems']=count
        STATS['Z_path_conflicts']=paths
        with self.assertRaisesRegex(ValueError,'IncompleteHorizon'):
            find_z_path((0,1),((0,1,1,1),),{1:1})

if __name__=='__main__':
    r=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromModule(__import__(__name__)))
    STATS.update(tests=r.testsRun,failures=len(r.failures),errors=len(r.errors),successful=r.wasSuccessful())
    Path(__file__).with_name('checkpoint_results.json').write_text(json.dumps(STATS,indent=2)+'\n')
    raise SystemExit(not r.wasSuccessful())
