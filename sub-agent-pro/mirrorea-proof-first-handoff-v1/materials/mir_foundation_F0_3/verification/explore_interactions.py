"""Enumerate all orders of six distinct model commands on one real F0.3 state.
This is exhaustive for that declared experiment, not an unbounded theorem.
"""
from pathlib import Path
import sys,json,itertools
from copy import deepcopy
ROOT=Path(__file__).resolve().parents[1];sys.path.insert(0,str(ROOT))
from examples.integrated import build,pending
from model.support import Invalid

def clone(k):
    c=object.__new__(type(k));c.data=deepcopy(k.data)
    # Carry the trusted prefix tip; previous states are not needed for this
    # no-recovery experiment. This is branching of the model checker only.
    c.journal=[k.journal[-1]];c.snapshots={};c.observers={}
    return c

def run():
    seed,h=build();aid,rid=pending(seed,h['use']);counts={};steps=0;examples={}
    actions=('serve','revoke-operation','revoke-access','retire-module','consume','observe')
    for order in itertools.permutations(actions):
        k=clone(seed);previous_cursor=0
        for action in order:
            try:
                if action=='serve':k.serve(rid)
                elif action=='revoke-operation':k.admin_revoke(h['use'][1])
                elif action=='revoke-access':k.admin_revoke(h['read'][1])
                elif action=='retire-module':k.admin_retire(h['y'])
                elif action=='consume':k.consume(aid)
                elif action=='observe':k.observe('viewer',2)
            except Invalid:pass # explicit rejection is a model outcome, not omission.
            k.audit();steps+=1
            current=k.data.bindings[h['binding']].cursor
            assert current>=previous_cursor;previous_cursor=current
            assert k.data.cells[h['cells']['stock']].value in (7,10)
            serves=sum(e.kind=='Served' and e.payload[0]==rid for e in k.data.events)
            assert serves<=1
        tag=(k.data.requests[rid].status,k.data.cells[h['cells']['stock']].value)
        key=str(tag);counts[key]=counts.get(key,0)+1;examples.setdefault(key,list(order))
    out={'actions':actions,'orders':720,'executed_or_rejected_steps':steps,'terminal_counts':counts,'representative_orders':examples,'scope':'one pending source-generated request; six commands each exactly once; no fairness or general schedule theorem'}
    (ROOT/'evidence/INTERLEAVINGS.json').write_text(json.dumps(out,indent=2)+'\n');return out
if __name__=='__main__':print(json.dumps(run(),indent=2))
