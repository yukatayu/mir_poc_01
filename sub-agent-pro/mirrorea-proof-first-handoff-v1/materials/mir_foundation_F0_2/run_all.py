#!/usr/bin/env python3
"""Run the research package without network access or modifying any repository."""
from pathlib import Path
import argparse,json,subprocess,sys,platform,re,hashlib,datetime
ROOT=Path(__file__).resolve().parent
OUT=ROOT/'evidence';OUT.mkdir(exist_ok=True)
def run(name,args,cwd=ROOT):
 r=subprocess.run(args,cwd=cwd,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,timeout=120)
 (OUT/(name+'.txt')).write_text(r.stdout)
 match=re.search(r'Ran (\d+) tests?',r.stdout)
 result={'command':args,'cwd':str(cwd.relative_to(ROOT)),'exit_code':r.returncode,'test_count':int(match.group(1)) if match else None,'log':'evidence/'+name+'.txt'}
 print(name,r.returncode,result['test_count'],flush=True)
 return result

def main():
 ap=argparse.ArgumentParser();ap.add_argument('--skip-smt',action='store_true',help='Explicitly skip SMT; this is recorded, not counted as pass.');args=ap.parse_args()
 checks=[run('f02_final_tests',[sys.executable,'-m','unittest','discover','-s','tests','-v']),run('f01_final_tests',[sys.executable,'-m','unittest','discover','-v'],ROOT/'f01_model'),run('compileall',[sys.executable,'-m','compileall','-q','model','tests','examples'])]
 checks.append(run('integrated_demo',[sys.executable,'examples/run_integrated.py']))
 checks.append(run('activation_final',[sys.executable,'examples/run_activation.py']))
 if not args.skip_smt:checks.append(run('smt_final',[sys.executable,'verification/check_smt.py']))
 meta={'schema':'mir-foundation-research-f02-validation-1','time_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'python':sys.version,'platform':platform.platform(),'checks':checks,'smt':'explicitly-skipped' if args.skip_smt else 'executed','all_executed_commands_pass':all(c['exit_code']==0 for c in checks),'proof_status':'general claims have hand proofs; no Lean/Rocq execution or independent kernel validation of Z3 proofs'}
 (OUT/'RUN_SUMMARY.json').write_text(json.dumps(meta,ensure_ascii=False,indent=2))
 if not meta['all_executed_commands_pass']:raise SystemExit(1)
if __name__=='__main__':main()
