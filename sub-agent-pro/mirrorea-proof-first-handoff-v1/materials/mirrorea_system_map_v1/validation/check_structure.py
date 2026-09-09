#!/usr/bin/env python3
"""Check this requirements dossier's internal traceability, not Mir correctness."""
from pathlib import Path
import json, re, hashlib
from collections import Counter

ROOT=Path(__file__).resolve().parents[1]
d=json.loads((ROOT/'requirements.json').read_text(encoding='utf-8'))
errors=[]
def insist(cond,msg):
    if not cond: errors.append(msg)

def inventory(name):
    rows=d[name]; ids=[r['id'] for r in rows]
    insist(len(ids)==len(set(ids)),f'duplicate IDs: {name}')
    return set(ids)
R=inventory('requirements');Q=inventory('decisions');P=inventory('proof_targets');S=inventory('scenarios');G=inventory('goals')
U=set(d['user_intents']);refs=set(d['sources']);cats=set(d['groups'])
for r in d['requirements']:
    for f in ['title','statement','acceptance_positive','acceptance_negative','owner','baseline','usability']:
        insist(bool(r.get(f)),f"{r['id']}: missing {f}")
    insist(r['origin'] in {'U','D'},f"{r['id']}: bad origin")
    for field,allowed in [('dependencies',R),('decisions',Q),('proof_targets',P),('scenarios',S),('user_refs',U),('source_refs',refs)]:
        for ref in r[field]: insist(ref in allowed,f"{r['id']}: missing {field} ref {ref}")
    insist(r['group'] in cats,f"{r['id']}: missing group")
    insist(bool(r['scenarios']),f"{r['id']}: no acceptance scenario")
    insist(bool(r['proof_targets']),f"{r['id']}: no assurance target")
    insist(not r['acceptance_spec_approved'],f"{r['id']}: unexpected approval")
for s in d['scenarios']:
    insist(bool(s['steps']) and bool(s['expected']) and bool(s['falsifier']),f"{s['id']}: incomplete scenario")
    insist(set(s['requirements'])<=R,f"{s['id']}: bad requirement")
for a in d['alpha_conditions']:
    insist(set(a['requirements'])<=R,f"{a['id']}: bad alpha ref")
for a in d['risks']:
    insist(set(a['requirements'])<=R,f"{a['id']}: bad risk requirement")
    insist(set(a['scenarios'])<=S,f"{a['id']}: bad risk scenario")
for g in d['goals']: insist(set(g['depends'])<=G,f"{g['id']}: bad goal ref")
usedU={u for r in d['requirements'] for u in r['user_refs']}
usedR={u for s in d['scenarios'] for u in s['requirements']}
usedQ={u for r in d['requirements'] for u in r['decisions']}
usedP={u for r in d['requirements'] for u in r['proof_targets']}
insist(usedU==U,'unmapped explicit intent');insist(usedR==R,'unmapped requirement')
insist(usedQ==Q,'unmapped decision');insist(usedP==P,'unmapped proof target')
# Goal dependencies must be acyclic. Requirement references may be co-design cycles.
graph={g['id']:g['depends'] for g in d['goals']};vis=set();active=set()
def visit(g):
    if g in active: errors.append('cyclic goal dependency: '+g);return
    if g in vis:return
    active.add(g)
    for dep in graph[g]:visit(dep)
    active.remove(g);vis.add(g)
for g in G:visit(g)
# Tarjan records (rather than silently hiding) requirement co-design components.
graphR={r['id']:r['dependencies'] for r in d['requirements']}
idx={};low={};stack=[];on=set();comps=[]
def strong(v):
    idx[v]=low[v]=len(idx);stack.append(v);on.add(v)
    for w in graphR[v]:
        if w not in idx:strong(w);low[v]=min(low[v],low[w])
        elif w in on:low[v]=min(low[v],idx[w])
    if low[v]==idx[v]:
        c=[]
        while True:
            w=stack.pop();on.remove(w);c.append(w)
            if w==v:break
        if len(c)>1:comps.append(sorted(c))
for v in sorted(R):
    if v not in idx:strong(v)
text=(ROOT/'MASTER.md').read_text(encoding='utf-8')
for r in R:insist(text.count('#### '+r+' — ')==1,'missing/duplicate detailed heading '+r)
for collection in ['decisions','proof_targets','scenarios','goals']:
    for row in d[collection]:insist('### '+row['id']+' — ' in text,'missing heading '+row['id'])
html=(ROOT/'MASTER.html').read_text(encoding='utf-8')
html_ids=re.findall(r'\bid="([^"]+)"',html)
insist(len(html_ids)==len(set(html_ids)),'duplicate HTML ID')
for link in re.findall(r'href="#([^"]+)"',html):insist(link in set(html_ids),'missing HTML anchor '+link)
insist(not re.search(r'<script[^>]+src=',html),'external script')
insist(not re.search(r'<link[^>]+href=["\']https?://',html),'external stylesheet')
insist(not re.search(r'<img[^>]+src=["\']https?://',html),'external image')
insist(d['metadata']['repo_commit']=='aafde92229bb0ff18116f38d4750a0a8f61cb069','unexpected baseline')
counts={k:len(d[k]) for k in ['requirements','groups','user_intents','decisions','proof_targets','scenarios','goals','risks']}
result=dict(kind='document-internal-structure-check',passed=not errors,counts=counts,
 origin_counts=dict(Counter(r['origin'] for r in d['requirements'])),
 coverage={'explicit_intents':len(usedU),'requirements_with_scenarios':len(usedR),'referenced_decisions':len(usedQ),'referenced_assurance_targets':len(usedP)},
 co_design_components=comps,co_design_note='論点参照の相互依存であり、実行順や循環論法の受理ではない。本文第11節参照。',
 errors=errors,does_not_establish=['requirements completeness for all future uses','project theoretical soundness','F0.x proof verification','project test execution','Canon adoption'])
(ROOT/'VALIDATION.json').write_text(json.dumps(result,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps(result,ensure_ascii=False,indent=2))
raise SystemExit(0 if not errors else 1)
