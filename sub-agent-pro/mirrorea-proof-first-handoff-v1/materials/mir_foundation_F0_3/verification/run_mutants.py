"""Execute isolated deliberately weakened implementations against direct falsifiers.
These mutations are experiments, not undiscovered defects of the baseline.
"""
from pathlib import Path
import sys,subprocess,tempfile,shutil,json
ROOT=Path(__file__).resolve().parents[1]
MUTANTS=[
 ('mixed-subject-policy','model/policy.py',"                if (c.principal,c.member)!=(ctx.principal,ctx.member):continue",'                pass # intentionally drop subject binding','test_mixed_principal_cannot_compose'),
 ('skip-use-revalidation','model/kernel.py','                d.auth.revalidate(r.evidence,r.context)','                pass # intentionally skip use-time validation','test_revocation_before_serve'),
 ('hidden-rows-evict-visible','model/kernel.py',"        safe=[e for e in d.events if e.label<=grant[1]]", "        safe=[e for e in (d.events[-capacity:] if capacity else []) if e.label<=grant[1]]",'test_high_events_do_not_evict_low_observations'),
 ('skip-reference-currentness','model/kernel.py','                d.auth.revalidate(ev,current)','                pass # intentionally trust stale access evidence','test_revoked_capability_triggers_semantic_fallback'),
 ('accept-incomplete-support','model/support.py',"        if holds(forms[k],live):raise Invalid('OmittedDerivableSupport')",'        pass # intentionally omit least-closure completeness','test_reject_everything_is_not_complete'),
 ('ignore-absence-dependency','model/transactions.py',"        self.reads[key] = stamp","        if key in self._store.values: self.reads[key] = stamp",'test_absent_read_blocks_phantom'),
]

def run():
    rows=[];logdir=ROOT/'evidence/mutants';logdir.mkdir(exist_ok=True)
    for name,path,old,new,test in MUTANTS:
        with tempfile.TemporaryDirectory(prefix='mir-f03-mutant-') as t:
            root=Path(t)
            for d in ('model','base','tests'):shutil.copytree(ROOT/d,root/d,ignore=shutil.ignore_patterns('__pycache__'))
            target=root/path;text=target.read_text()
            if old not in text:raise AssertionError('MutantAnchorLost:'+name)
            target.write_text(text.replace(old,new,1))
            p=subprocess.run([sys.executable,'-m','unittest','discover','-s','tests','-p','test_foundation.py','-k',test,'-v'],cwd=root,capture_output=True,text=True,timeout=15)
            log=p.stdout+p.stderr;(logdir/(name+'.log')).write_text(log)
            detected=p.returncode!=0 and 'FAIL:' in log and 'Ran 1 test' in log
            rows.append({'name':name,'path':path,'test':test,'returncode':p.returncode,'detected_by_behavioral_assertion':detected,'replacement':[old,new]})
            if not detected:raise AssertionError(name+' did not reproduce target counterexample: '+log)
    (ROOT/'evidence/MUTANTS.json').write_text(json.dumps(rows,indent=2)+'\n')
    return rows
if __name__=='__main__':print(json.dumps(run(),indent=2))
