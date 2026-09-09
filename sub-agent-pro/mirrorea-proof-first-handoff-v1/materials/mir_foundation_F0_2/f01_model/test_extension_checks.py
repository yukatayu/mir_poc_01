"""Further non-fixed-source additions and graph-composition falsifiers.
These execute the independent research model, not the repository's runtime.
"""
import copy
import json
import unittest
from pathlib import Path
from core import CellSpec, Config, E, Grant, INT, Operation, StaticError, compile_operation, topological_rank


def base():
    schema = {'base': CellSpec(INT, 'B')}
    source = Operation('base_inc', 'A', 'base', E('add', E('cell', 'base'), E('lit', 1)))
    c = Config(schema, {'base': 0}, {'base_inc': compile_operation(source, schema)},
               membership={'alice': 0}, patch_admins={'operator'})
    return c


def extension():
    cells = {'addition': (CellSpec(INT, 'C'), 10)}
    ops = (Operation('extension_inc', 'A', 'addition', E('add', E('cell', 'addition'), E('lit', 2))),
           Operation('parent_inc', 'C', 'base', E('add', E('cell', 'base'), E('lit', 3))))
    return cells, ops, {('base', 'addition')}


class Extensions(unittest.TestCase):
    def test_one_module_generates_different_locus_operations(self):
        c = base()
        self.assertEqual(c.install_module('operator', *extension()), 'Installed')
        self.assertEqual(c.operations['extension_inc'].edges,
                         (('A', 'C', 'request'), ('C', 'A', 'outcome')))
        self.assertEqual(c.operations['parent_inc'].edges,
                         (('C', 'B', 'request'), ('B', 'C', 'outcome')))
        for op in ('extension_inc', 'parent_inc'):
            grant = c.issue_grant('operator', 'alice', op)
            self.assertEqual(c.receive(c.serve(c.request(grant))), 'Success')
        self.assertEqual(c.store, {'base': 3, 'addition': 12})

    def test_checked_module_alone_does_not_issue_grant(self):
        c = base()
        self.assertEqual(c.install_module('operator', *extension()), 'Installed')
        self.assertEqual(c.grants, set())
        with self.assertRaises(PermissionError):
            c.request(Grant('alice', 0, 'extension_inc', 0))

    def test_invalid_operation_leaves_no_partial_added_state(self):
        c = base()
        before = copy.deepcopy(c)
        cells, ops, edges = extension()
        bad = Operation('cross_owner', 'A', 'addition', E('cell', 'base'))
        self.assertEqual(c.install_module('operator', cells, ops + (bad,), edges),
                         'CrossOwnerReadRequiresSnapshotBoundary')
        self.assertEqual(c, before)

    def test_shadowing_is_rejected_without_mutation(self):
        c = base()
        before = copy.deepcopy(c)
        cells, ops, edges = extension()
        duplicate = Operation('base_inc', 'C', 'base', E('lit', 999))
        self.assertEqual(c.install_module('operator', cells, ops + (duplicate,), edges), 'OperationCollision')
        self.assertEqual(c, before)

    def test_unrelated_pending_request_need_not_stop_addition(self):
        c = base()
        g = c.issue_grant('operator', 'alice', 'base_inc')
        request = c.request(g)
        self.assertEqual(c.install_module('operator', *extension()), 'Installed')
        self.assertEqual(c.receive(c.serve(request)), 'Success')
        self.assertEqual(c.store['base'], 1)

    def test_detach_and_reinstall_does_not_revive_old_grant(self):
        c = base()
        c.install_module('operator', *extension())
        grant = c.issue_grant('operator', 'alice', 'extension_inc')
        old_inc = c.cell_incarnations['addition']
        self.assertEqual(c.detach('operator', {'addition'}), 'Detached')
        cells, ops, edges = extension()
        # parent_inc is still live and may not be shadowed; only add the removed operation.
        self.assertEqual(c.install_module('operator', cells, (ops[0],), edges), 'Installed')
        self.assertGreater(c.cell_incarnations['addition'], old_inc)
        with self.assertRaises(PermissionError):
            c.request(grant)
        fresh = c.issue_grant('operator', 'alice', 'extension_inc')
        self.assertEqual(c.receive(c.serve(c.request(fresh))), 'Success')

    def test_stale_patch_stamp_rejects_recreated_same_name_and_value_version(self):
        c = base()
        c.install_module('operator', *extension())
        from dataclasses import replace
        replacement = replace(c.operations['extension_inc'].source, revision=1,
                              body=E('add', E('cell', 'addition'), E('lit', 9)))
        stale = c.prepare_versions({'addition'})
        self.assertEqual(c.detach('operator', {'addition'}), 'Detached')
        cells, ops, edges = extension()
        self.assertEqual(c.install_module('operator', cells, (ops[0],), edges), 'Installed')
        before = copy.deepcopy(c)
        self.assertEqual(c.replace_operation('operator', replacement, stale), 'StalePreparation')
        self.assertEqual(c, before)

    def test_disjoint_added_edge_endpoints_do_not_ensure_acyclic_union(self):
        # Independently safe, endpoint-disjoint edits can close a global cycle.
        nodes = set('abcd')
        old = {('b', 'c'), ('d', 'a')}
        first, second = {('a', 'b')}, {('c', 'd')}
        topological_rank(nodes, old | first)
        topological_rank(nodes, old | second)
        with self.assertRaisesRegex(StaticError, 'Cycle'):
            topological_rank(nodes, old | first | second)

    def test_stale_full_graph_does_not_overwrite_unrelated_patch(self):
        schema = {x: CellSpec(INT, 'A') for x in 'abcd'}
        c = Config(schema, {x: 0 for x in schema}, {}, edges={('b', 'c'), ('d', 'a')},
                   patch_admins={'operator'})
        original = set(c.edges)
        stamp_a = c.prepare_versions({'a', 'b'})
        stamp_b = c.prepare_versions({'c', 'd'})
        self.assertEqual(c.reparent('operator', original | {('a', 'b')}, {'a', 'b'}, stamp_a), 'Activated')
        after_a = copy.deepcopy(c)
        self.assertEqual(c.reparent('operator', original | {('c', 'd')}, {'c', 'd'}, stamp_b),
                         'IncompleteFootprint')
        self.assertEqual(c, after_a)


if __name__ == '__main__':
    result = unittest.TextTestRunner(verbosity=2).run(
        unittest.defaultTestLoader.loadTestsFromModule(__import__(__name__)))
    Path(__file__).with_name('extension_results.json').write_text(json.dumps({
        'tests': result.testsRun, 'failures': len(result.failures),
        'errors': len(result.errors), 'successful': result.wasSuccessful(),
    }, indent=2) + '\n')
    raise SystemExit(not result.wasSuccessful())
