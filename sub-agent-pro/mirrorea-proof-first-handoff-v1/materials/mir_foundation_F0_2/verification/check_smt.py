#!/usr/bin/env python3
"""Regenerate and check arithmetic/control VCs; not an independent proof kernel."""
from pathlib import Path
import sys,json,re,hashlib
ROOT=Path(__file__).resolve().parents[1]
sys.path.insert(0,str(ROOT/'f01_verification'))
from z3_runner import run_text
VERSIONS=set()
CASES=[
('square_nonnegative','unsat','forall-input arithmetic VC','(declare-const x Int)\n(assert (< (* x x) 0))'),
('positive_result','unsat','forall-input arithmetic VC','(declare-const x Int)\n(assert (< (+ (ite (< x 0) (- x) x) 1) 1))'),
('owner_resource_preserved','unsat','forall-input owner VC','(declare-const free Int)\n(declare-const n Int)\n(assert (and (>= free 0) (> n 0) (<= n free) (< (- free n) 0)))'),
('strengthened_assert_counterexample','sat','deliberate counterexample','(declare-const x Int)\n(assert (= x 0))\n(assert (not (< x 0)))'),
('result_capture_counterexample','sat','deliberate counterexample','(declare-const local_result Int)\n(declare-const returned Int)\n(assert (and (= local_result 10) (= returned (- 1)) (> local_result 0) (< returned 0)))'),
('certificate_product_rule','unsat','forall-input certificate-rule VC','(declare-const p Real)\n(declare-const q Real)\n(assert (and (>= p 0) (>= q 0) (< (* p q) 0)))'),
('certificate_equality_multiple','unsat','forall-input certificate-rule VC','(declare-const h Real)\n(declare-const r Real)\n(assert (and (= h 0) (< (* h r) 0)))'),
('sequential_rmw','unsat','forall-input operation VC','(declare-const x Int)\n(declare-const a Int)\n(declare-const b Int)\n(assert (not (= (- (- x a) b) (- x (+ a b)))))'),
('blind_write_counterexample','sat','deliberate counterexample','(declare-const x Int)\n(assert (= x 100))\n(assert (not (= (- x 10) (- (- x 10) 10))))'),
('prefix_retirement_fence','unsat','forall-input epoch/floor VC','(declare-const seq Int)\n(declare-const old Int)\n(declare-const new Int)\n(assert (and (<= seq old) (<= old new) (> seq new)))'),
('binding_stamp_reuse_counterexample','sat','deliberate counterexample','(declare-const oldver Int)\n(declare-const newver Int)\n(declare-const oldinc Int)\n(declare-const newinc Int)\n(assert (and (= oldver newver) (not (= oldinc newinc))))'),
('full_stamp_no_revival','unsat','forall-input epoch/floor VC','(declare-const oldinc Int)\n(declare-const newinc Int)\n(assert (distinct oldinc newinc))\n(assert (= oldinc newinc))'),
('migration_conservation','unsat','forall-input migration VC','(declare-const x Int)\n(declare-const y Int)\n(declare-const d Int)\n(assert (not (= (+ (+ x d) (- y d)) (+ x y))))'),
('local_patch_commuting','unsat','forall-input migration VC','(declare-const x Int)\n(declare-const a Int)\n(declare-const b Int)\n(assert (not (= (+ (+ x a) b) (+ (+ x b) a))))'),
('committed_prepared_or_new_not_old','unsat','finite-phase invariant VC','(declare-const p Int)\n(assert (or (= p 1) (= p 2)))\n(assert (= p 0))'),
('abort_does_not_have_new','unsat','finite-phase invariant VC','(declare-const p Int)\n(assert (or (= p 0) (= p 1) (= p 3)))\n(assert (= p 2))'),
('fence_owner_mismatch_denies_release','unsat','symbolic lock VC','(declare-const held Int)\n(declare-const acting Int)\n(assert (distinct held acting))\n(assert (= held acting))'),
('post_install_shadow_counterexample','sat','deliberate counterexample','(declare-const shadow Int)\n(declare-const now Int)\n(assert (= now (+ shadow 1)))\n(assert (distinct shadow now))'),
('high_pc_cannot_low_write','unsat','forall-input label VC','(declare-const pc Int)\n(declare-const target Int)\n(declare-const observer Int)\n(assert (and (> pc observer) (<= pc target) (<= target observer)))'),
('definition_label_cannot_erased','unsat','forall-input label VC','(declare-const definition Int)\n(declare-const result Int)\n(declare-const observer Int)\n(assert (and (> definition observer) (<= definition result) (<= result observer)))'),
('trace_inclusion_not_noninterference','sat','deliberate counterexample','(declare-const h Bool)\n(declare-const out Int)\n(assert (= out (ite h 1 0)))\n(assert (or (= out 0) (= out 1)))'),
('waiting_version_unchanged_under_frame','unsat','symbolic dependency VC','(declare-const version Int)\n(declare-const before Int)\n(declare-const after Int)\n(assert (= before version))\n(assert (= after version))\n(assert (distinct before after))')]
def one(path,expected,category,dest):
 text=path.read_text();out,solver_version=run_text(text);VERSIONS.add(solver_version);m=re.search(r'(?m)^\s*(sat|unsat|unknown)\s*$',out);status=m.group(1) if m else 'missing'
 dest.mkdir(parents=True,exist_ok=True);(dest/(path.stem+'.out')).write_text(out)
 return {'file':str(path.relative_to(ROOT)),'expected':expected,'actual':status,'matches':status==expected and '(error' not in out,'category':category,'input_sha256':hashlib.sha256(text.encode()).hexdigest(),'output_sha256':hashlib.sha256(out.encode()).hexdigest()}
def main():
 base=json.loads((ROOT/'f01_verification/manifest.json').read_text());b=[]
 for x in base:b.append(one(ROOT/'f01_verification/vcs'/x['file'],x['expected'],x['category'],ROOT/'evidence/f01_smt_rerun'))
 new=[];manifest=[]
 for index,(name,expected,category,body) in enumerate(CASES,1):
  filename=f'F2_{index:02d}_{name}.smt2';path=ROOT/'verification'/filename
  path.write_text('(set-option :produce-proofs true)\n(set-option :produce-models true)\n(set-option :timeout 15000)\n(set-logic ALL)\n'+body+'\n(check-sat)\n'+('(get-proof)\n' if expected=='unsat' else '(get-model)\n'))
  manifest.append({'file':filename,'expected':expected,'category':category});new.append(one(path,expected,category,ROOT/'evidence/f02_smt'))
 (ROOT/'verification/MANIFEST.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2))
 results={'solver':sorted(VERSIONS),'baseline':b,'f02':new,'all_expected':all(x['matches'] for x in b+new),'trust':'solver outputs not independently checked; VCs are not the complete metatheory'}
 (ROOT/'evidence/smt_results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2));print('solver',sorted(VERSIONS),'baseline',len(b),'new',len(new),'match',results['all_expected'])
 if not results['all_expected']:raise SystemExit(1)
if __name__=='__main__':main()
