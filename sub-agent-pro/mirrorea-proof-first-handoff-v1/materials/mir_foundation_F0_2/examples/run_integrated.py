from pathlib import Path
import sys,json,tempfile
from dataclasses import asdict
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
from model.language import parse_check_compile
from model.engine import Engine
from model.certificates import C,Poly,produce_evidence,produce_total_evidence,prove_call_chain,encode_evidence,decode_evidence,verify_export
from model.live_patch import LivePatch
from model.snapshot import capture,restore_as_fresh,InstanceAuthority
from model.journal import DurableCell
ROOT=Path(__file__).resolve().parents[1]
def load(name,mod,imports=None):return parse_check_compile((ROOT/'examples'/name).read_text(),mod,imports)
def main():
 positive=load('positive.mirx','Positive');pool=load('pool.mirx','Pool');sigs={o.sig.name:o.sig for p in [positive,pool] for o in p.operations.values()};client=load('client.mirx','Client',sigs)
 e=Engine([positive,pool,client]);e.patch_principals.add('maintainer')
 pop=e.ops['Positive.positive'];aop=e.ops['Pool.allocate_size'];post=Poly.var('result')-1;invs={'free_nonnegative':Poly.var('free')}
 pe=produce_evidence(pop,{},post);te=produce_total_evidence(pop,{},post);ae=produce_evidence(aop,invs,Poly.var('result')-1)
 packet=encode_evidence(pe);verify_export(pop,{},post,decode_evidence(packet));(ROOT/'evidence/positive_certificate.json').write_bytes(packet)
 e.register_export(pop.sig.name,te,post);e.protect_owner('PoolOwner',invs,{aop.sig.name:(ae,Poly.var('result')-1)})
 bridge=prove_call_chain(client,'allocate_positive',pop,pe,post,aop,Poly.var('n')-1,C('premise',0))
 no_right=e.spawn('Client.allocate_positive',(-3,));e.run();assert e.fibers[no_right].failure=='AuthorityDenied'
 e.grant_all('alice');fid=e.spawn('Client.allocate_positive',(-3,));e.run();before=e.state();assert e.fibers[fid].result==4
 revised=parse_check_compile(positive.source.replace('+ 1','+ 2'),'Positive');rop=revised.operations[pop.sig.name];rte=produce_total_evidence(rop,{},post);e.replace_operation('maintainer',revised,pop.sig.name,(rte,post))
 migration=parse_check_compile('free: Cell["PoolOwner", int] = 0\n@owner("PoolOwner")\ndef grow() -> int:\n    free = free + 8\n    return free\n','CapacityPatch');mop=migration.operations['CapacityPatch.grow'];me=produce_evidence(mop,invs)
 patch=LivePatch(e,'maintainer',{'PoolOwner':(migration,mop.sig.name,me)})
 pure=parse_check_compile('@task("Client")\ndef unrelated(x: int) -> int:\n    return x*x\n','Pure');e.install_addition('maintainer',pure)
 patch.advance('prepare',0);unrelated=e.spawn('Pure.unrelated',(7,));e.run();assert e.fibers[unrelated].result==49
 fenced=sorted(e.shards['PoolOwner'].frozen);patch.advance('vote',0);patch.advance('decide-commit');patch.advance('learn',0);patch.advance('install',0);after=e.state();assert after['free']==before['free']+8
 snapshot=capture(e);fresh=restore_as_fresh(snapshot,InstanceAuthority());assert not fresh.auth.issued
 denied=fresh.spawn('Client.allocate_positive',(1,));fresh.run();assert fresh.fibers[denied].failure=='AuthorityDenied'
 fresh.grant_all('alice');res=fresh.spawn('Client.allocate_positive',(1,));fresh.run();assert fresh.fibers[res].result==3
 with tempfile.TemporaryDirectory() as d:
  path=Path(d)/'journal';j=DurableCell(path)
  try:j.add('requester',1,5,crash='after_append')
  except RuntimeError:pass
  j=DurableCell(path);assert j.add('requester',1,5)==5
  physical={'recovered_value':j.value,'replayed_result':j.decisions['requester',1][1],'journal_bytes':path.stat().st_size}
 data={'profile':'source/certificate/actor/patch/fresh-import integrated; journal tested as separate actual-file adapter','source_inputs':['positive.mirx','pool.mirx','client.mirx'],'generated_edges':[edge for program in e.programs for edge in program.generated_edges],'proof_does_not_grant':e.fibers[no_right].failure,'bridge_partial_precondition':asdict(bridge),'first_result':e.fibers[fid].result,'state_before_patch':before,'prepared_fence':fenced,'unrelated_result':49,'state_after_patch':after,'patch_history':patch.log,'fresh_grants_initially_empty':True,'restored_code_result':fresh.fibers[res].result,'actual_file_journal':physical,'trace':e.trace}
 (ROOT/'evidence/INTEGRATED_DEMO.json').write_text(json.dumps(data,ensure_ascii=False,indent=2,default=str));print(json.dumps({k:v for k,v in data.items() if k not in ('trace','patch_history','generated_edges')},ensure_ascii=False,indent=2,default=str))
 e.assert_invariants();fresh.assert_invariants()
if __name__=='__main__':main()
