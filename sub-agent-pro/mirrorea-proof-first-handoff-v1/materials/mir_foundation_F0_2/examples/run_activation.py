from pathlib import Path
import sys,json
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from model.activation import explore
ROOT=Path(__file__).resolve().parents[1]
data={'scope':'exhaustive finite patch phase model; no packet queue enumeration, no general proof by counting','normal':{},'mutants':{}}
for n in (1,2,3):data['normal'][str(n)]=explore(n)
for m in ('timeout-abort','volatile-prepare','volatile-decision'):data['mutants'][m]=explore(2,m)
assert all(v['safe'] for v in data['normal'].values());assert all(not v['safe'] for v in data['mutants'].values())
(ROOT/'evidence/ACTIVATION_FINAL.json').write_text(json.dumps(data,ensure_ascii=False,indent=2));print(json.dumps(data,ensure_ascii=False,indent=2))
