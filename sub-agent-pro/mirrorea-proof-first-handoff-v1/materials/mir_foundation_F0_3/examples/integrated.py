from pathlib import Path
import sys,json
from dataclasses import asdict
from copy import deepcopy
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT))
from model.kernel import Kernel, RelationExpr as E, Sample, fingerprint
from model.support import Top,Ref,All,Invalid
from model.policy import Policy,Need
from base.language import parse_check_compile
from base.certificates import Poly,produce_evidence

def build():
    k=Kernel('integration-1');loci={n:k.admin_node('locus',n) for n in ('A','B','C')}
    x=k.admin_node('module','X',support=All(*(Ref(n) for n in loci.values())))
    alice=k.admin_node('member','alice',support=Ref(loci['A']),principal='alice')
    viewer=k.admin_node('member','viewer',support=Ref(loci['B']),principal='viewer')
    remote=k.admin_node('member','remote',support=Ref(loci['C']),principal='remote')
    y=k.admin_node('module','Y',support=All(Ref(x),Ref(loci['A']),Ref(loci['C'])))
    alt=k.admin_node('module','AlternativeBase',support=Ref(x))
    for issuer in ('identity','owner'):k.admin_issuer(issuer)
    for key,predicate in (('operate','operate'),('patch','edit'),('anchor','read'),('reacquire','relate')):
        k.admin_policy(Policy(key,0,All(Ref('id'),Ref('permission')),(('id',Need('identity','member')),('permission',Need('owner',predicate)))))
    source=(ROOT/'examples/service.mirx').read_text();p=parse_check_compile(source,'Service');inv={'nonnegative':Poly.var('stock')};ev=produce_evidence(p.operations['Service.use'],inv)
    cells=k.admin_install_program(source,'Service',y,loci,{'stock':0},{'Service.use':'operate'},{'Service.use':(inv,ev)})
    k.admin_install_program((ROOT/'examples/client.mirx').read_text(),'Client',y,loci,{}, {})
    base=(ROOT/'examples/base.mirx').read_text();bp=parse_check_compile(base,'Base');binv={'nonnegative':Poly.var('count')};bev=produce_evidence(bp.operations['Base.increment'],binv)
    basecells=k.admin_install_program(base,'Base',x,loci,{'count':0},{'Base.increment':'operate'},{'Base.increment':(binv,bev)})
    def claims(action,target,predicate):
        return (k.admin_claim('identity','alice','member',{action},{target}),k.admin_claim('owner','alice',predicate,{action},{target}))
    use=claims('Service.use',y,'operate');increment=claims('Base.increment',x,'operate');edit=claims('reparent',y,'edit')
    primary=k.admin_node('anchor','remote-anchor',support=Ref(remote));fallback=k.admin_node('anchor','local-anchor',support=Ref(x))
    read=claims('anchor_use',primary,'read');access=k.access_anchor('alice',primary,'anchor',read)
    binding=k.admin_binding('follow',y,(access,fallback),'reacquire')
    relation=k.admin_relation('offset',y,E('add',(E('ref',(binding,)),E('const',(5,)))))
    reacquire=claims('reacquire',binding,'relate')
    k.admin_observer('viewer',0)
    return k,locals()

def pending(k,claims,n=3):
    aid=k.start('Client','run','alice',(n,),claims)
    while k.data.activities[aid].waiting is None:k.local_step(aid)
    return aid,k.data.activities[aid].waiting[0]

def run():
    k,h=build();out={'profile':'integrated semantic-control reference; no actual network/host isolation claim','repository_ref':'19a6decfbea0815b7b4265e8fadfb399270aaed2'}
    out['source_generated_edges']=[e for p in k.data.programs.values() for e in p.generated_edges]
    a,r=pending(k,h['use']);k.drive(a);out['first_result']=k.data.activities[a].result
    original=fingerprint(k.data)
    try:k.project('viewer',h['relation'],{},'frame')
    except Invalid as e:out['gap']=str(e)
    assert fingerprint(k.data)==original and k.data.bindings[h['binding']].cursor==0
    out['initial_relation']=k.project('viewer',h['relation'],{h['primary']:Sample(h['primary'],0,'frame',4)},'frame')
    a2,r2=pending(k,h['use'],2)
    prep=k.prepare_reparent('alice',h['y'],Ref(h['alt']),'patch',h['edit'])
    cp=k.checkpoint()
    # Current authority and cross-graph retirement change while pending work exists.
    k.admin_revoke(h['use'][1]);k.admin_retire(h['remote']);k.admin_revoke(h['edit'][1])
    stock_before=k.data.cells[h['cells']['stock']].value
    current_head=k.journal[-1][0]
    try:k.recover(cp,[],current_head)
    except Invalid as e:out['old_snapshot_without_suffix']=str(e)
    k.recover(cp,deepcopy(k.journal[cp.position+1:]),current_head)
    out['restored_components']=list(cp.components)
    assert r2 in k.data.requests and prep.identity in k.data.preparations
    out['pending_after_restore']=k.data.requests[r2].status
    k.serve(r2);k.consume(a2)
    out['revoked_request']=k.data.requests[r2].status
    out['no_second_mutation']=k.data.cells[h['cells']['stock']].value==stock_before
    before=fingerprint(k.data)
    try:k.commit_reparent(prep,'alice')
    except Invalid as e:out['revoked_patch_commit']=str(e)
    assert fingerprint(k.data)==before
    out['fallback_cursor']=k.data.bindings[h['binding']].cursor
    out['fallback_projection']=k.project('viewer',h['relation'],{h['fallback']:Sample(h['fallback'],0,'frame2',8)},'frame2')
    # A logically unrelated owner remains usable under the declared scope.
    independent=k.start('Base','run','alice',(7,),h['increment']);k.drive(independent)
    out['unrelated_owner_result']=k.data.activities[independent].result
    # Fresh incarnations require new authority; a new access alone does not promote.
    newremote=k.admin_node('member','remote',support=Ref(h['loci']['C']),principal='remote')
    newanchor=k.admin_node('anchor','remote-anchor',support=Ref(newremote))
    c1=k.admin_claim('identity','alice','member',{'anchor_use'},{newanchor});c2=k.admin_claim('owner','alice','read',{'anchor_use'},{newanchor})
    newaccess=k.access_anchor('alice',newanchor,'anchor',(c1,c2))
    out['before_explicit_reacquire']=k.data.bindings[h['binding']].cursor
    k.reacquire('alice',h['binding'],h['reacquire'],(newaccess,h['fallback']))
    out['after_explicit_reacquire']={'cursor':k.data.bindings[h['binding']].cursor,'lineage':k.data.bindings[h['binding']].lineage}
    # Observer activity never edits Data. High metadata cannot evict low buffers.
    observed=k.observe('viewer',8);state_before=fingerprint(k.data)
    for _ in range(50):k.observe('viewer',8)
    assert state_before==fingerprint(k.data)
    for j in range(7):k.admin_node('anchor','private-'+str(j),1)
    assert k.observe('viewer',8)==observed
    out['passive_erasure']=True;out['filter_before_retention']=True
    out['live_count']=len(k.data.live)
    out['events']=[asdict(e) for e in k.observe('viewer',10000)]
    out['research_assumptions']=['atomic semantic control transitions','trusted issued claims and roots, no cryptographic identity proof','static label lattice, no silent relabel/declassification','trusted current durable tip with complete in-memory records','polynomial checker TCB retained from F0.2','passive observation has independent scheduling/storage; no physical zero-cost claim']
    k.audit()
    (ROOT/'evidence/INTEGRATED_DEMO.json').write_text(json.dumps(out,ensure_ascii=False,indent=2)+'\n')
    return out
if __name__=='__main__':
    out=run();print(json.dumps({k:v for k,v in out.items() if k!='events'},ensure_ascii=False,indent=2))
