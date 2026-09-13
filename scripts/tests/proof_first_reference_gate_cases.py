"""Metadata-gate negatives only; these synthetic records are not execution evidence."""
from pathlib import Path
import copy
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from proof_first_reference_source_check import FULL_CASES, SESSION_CASES, validate_source_cases

record = dict(status='passed', filter=None, cases=[
    dict(name=name, scope='admitted-session' if name in SESSION_CASES else 'source-component-control',
         parser_exit=0, lean_exit=0) for name in sorted(FULL_CASES)])
validate_source_cases(record)
for kind in ['empty', 'filtered', 'missing', 'duplicate', 'renamed', 'wrong-scope', 'failed']:
    bad = copy.deepcopy(record)
    if kind == 'empty':
        bad['cases'] = []
    elif kind == 'filtered':
        bad['filter'] = 'source-'
    elif kind == 'missing':
        bad['cases'].pop()
    elif kind == 'duplicate':
        bad['cases'][-1] = bad['cases'][0]
    elif kind == 'renamed':
        bad['cases'][0]['name'] = 'not-the-same-case'
    elif kind == 'wrong-scope':
        bad['cases'][0]['scope'] = 'unclassified'
    else:
        bad['cases'][0]['lean_exit'] = 1
    try:
        validate_source_cases(bad)
    except ValueError:
        print(kind + ' REJECT')
    else:
        raise RuntimeError(kind + ' unexpectedly accepted')
