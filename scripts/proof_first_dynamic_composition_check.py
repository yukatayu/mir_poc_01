"""Fresh W3 LAB kernel/axiom and theorem-targeted mutation checks; not W3 acceptance."""
from pathlib import Path
import argparse, hashlib, json, os, re, subprocess, tempfile

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--work-root', type=Path, required=True,
                    help='existing directory outside repository for small disposable copies')
args = parser.parse_args()
repo = Path(__file__).resolve().parents[1]
work_root = args.work_root.resolve()
if not work_root.is_dir() or work_root.is_relative_to(repo):
    parser.error('--work-root must be an existing directory outside repository')
version = subprocess.check_output(['lean','--version'],text=True)
if 'version 4.29.1,' not in version:
    raise SystemExit('Lean4.29.1 required for this evidence cut')
root = Path(tempfile.mkdtemp(prefix='mir-w3-dynamic-',dir=work_root))
out = root / 'mutations'
out.mkdir()
print(root,flush=True)
env = dict(os.environ, LEAN_PATH=str(root))
modules = ['Support','CurrentUse','TrackedValidation','GraphValidation',
           'DynamicSupport','DynamicGraphs','DynamicIdentity','SupportImpact',
           'CurrentChoice','NamedCatalog','DynamicScopeControls']
results = dict(classification='LAB general-theorem checks and fixed controls, not production/source/lifecycle acceptance',
               lean_version=version,baselines=[],mutations=[])
for module in modules:
    name = 'MirroreaProofFirst'+module+'.lean'
    data = (repo/'samples/lean/foundations'/name).read_bytes()
    (root/name).write_bytes(data)
    if re.search(rb'(?m)^\s*(?:axiom\b|.*:=\s*(?:sorry|admit)\b)',data):
        raise AssertionError(('forbidden declaration',name))
    command = ['lean','--trust=0','-o',name[:-5]+'.olean',name]
    r = subprocess.run(command,cwd=root,env=env,capture_output=True,text=True)
    (root/(module+'.log')).write_text(r.stdout+r.stderr)
    results['baselines'].append(dict(module=module,command=command,exit=r.returncode,
        sha256=hashlib.sha256(data).hexdigest(),stdout=r.stdout,stderr=r.stderr))
    (root/'RESULT.json').write_text(json.dumps(results,indent=2)+'\n')
    if r.returncode or 'sorryAx' in r.stdout or 'sorryAx' in r.stderr:
        raise AssertionError(results['baselines'][-1])
    print(module,'kernel PASS',flush=True)

# Inspect every kernel declaration owned by the imported proof modules, including
# private/unused declarations. This is stronger than only grepping printed roots;
# evidence execution and imported-file provenance still belong to the harness TCB.
def audit_source(imports, allowed):
    return '\n'.join(['import Lean', *['import '+name for name in imports],
        'set_option maxRecDepth 10000', 'open Lean Elab Command',
        'run_cmd do',
        '  let env ← getEnv',
        '  let modules : List String := '+json.dumps(imports),
        '  let allowed : List Name := ['+', '.join('``'+name for name in allowed)+']',
        '  let mut counts : List (String × Nat) := modules.map (fun name => (name,0))',
        '  for (name, _) in env.constants.toList do',
        '    if let some index := env.getModuleIdxFor? name then',
        '      let owner := env.header.moduleNames[index.toNat]!.toString',
        '      if modules.contains owner then',
        '        counts := counts.map (fun (m,k) => (m,if m == owner then k+1 else k))',
        '        for dependency in (← Lean.collectAxioms name) do',
        '          unless allowed.contains dependency do',
        '            throwError "AXIOM_AUDIT_REJECT {name}: {dependency}"',
        '  for (name,count) in counts do',
        '    if count == 0 then throwError "AXIOM_AUDIT_EMPTY {name}"',
        '    logInfo m!"AXIOM_AUDIT_OK {name} {count}"', ''])

def audit(name, imports, allowed):
    source = audit_source(imports, allowed)
    (root/(name+'.lean')).write_text(source)
    command = ['lean','--trust=0',name+'.lean']
    r = subprocess.run(command,cwd=root,env=env,capture_output=True,text=True)
    (root/(name+'.log')).write_text(r.stdout+r.stderr)
    return dict(command=command,exit=r.returncode,stdout=r.stdout,stderr=r.stderr,
                sha256=hashlib.sha256(source.encode()).hexdigest())

results['declaration_audit'] = audit('DeclarationAudit',
    ['MirroreaProofFirst'+module for module in modules],
    ['propext','Classical.choice','Quot.sound'])
if results['declaration_audit']['exit'] or 'AXIOM_AUDIT_OK' not in results['declaration_audit']['stdout']:
    raise AssertionError(results['declaration_audit'])

# A private, otherwise unused declaration genuinely depends on the standard
# choice axiom. Deliberately excluding that existing axiom must make audit fail.
# No new axiom, proof hole, or admitted result is introduced for this control.
control = 'MirroreaProofFirstAuditPrivateControl'
(root/(control+'.lean')).write_text(
    'private noncomputable def unusedChoice (h : Nonempty Nat) : Nat := Classical.choice h\n')
r = subprocess.run(['lean','--trust=0','-o',control+'.olean',control+'.lean'],
                   cwd=root,env=env,capture_output=True,text=True)
if r.returncode:
    raise AssertionError((r.stdout,r.stderr))
expected_control_error = 'AXIOM_AUDIT_REJECT _private.'+control+'.0.unusedChoice: Classical.choice'
def rejects_private_choice(result):
    errors = re.findall(r'^.*?error: (.+)$',result['stdout'],re.MULTILINE)
    return result['exit'] != 0 and errors == [expected_control_error]

control_diagnostic = 'PrivateAuditControl.lean:5:0: error: '+expected_control_error+'\n'
assert rejects_private_choice(dict(exit=1,stdout=control_diagnostic))
assert not rejects_private_choice(dict(exit=0,stdout=control_diagnostic))
assert not rejects_private_choice(dict(exit=1,stdout=control_diagnostic.replace('Classical.choice','propext')))
assert not rejects_private_choice(dict(exit=1,stdout=control_diagnostic.replace('unusedChoice','otherDeclaration')))
assert not rejects_private_choice(dict(exit=1,stdout='unrelated elaboration failure'))
results['audit_predicate_controls'] = 'exact diagnostic accepted; zero exit, wrong axiom, wrong declaration and unrelated error rejected'
results['audit_control_positive'] = audit('PrivateAuditPositive',[control],
    ['propext','Classical.choice','Quot.sound'])
if results['audit_control_positive']['exit'] or 'AXIOM_AUDIT_OK' not in results['audit_control_positive']['stdout']:
    raise AssertionError(results['audit_control_positive'])
results['audit_control'] = audit('PrivateAuditControl',[control],['propext','Quot.sound'])
if not rejects_private_choice(results['audit_control']):
    raise AssertionError(results['audit_control'])
print('declaration audit and private unused control PASS',flush=True)
(root/'RESULT.json').write_text(json.dumps(results,indent=2)+'\n')

mutations = [
 ('growth_ref_erasure','DynamicSupport','| .ref k => .ref (embed k)',
  '| .ref k => .top','eval_map'),
 ('growth_old_formula','DynamicSupport','then mapFormula (left n m) (old.forms ⟨k.val,h⟩)',
  'then .either (mapFormula (left n m) (old.forms ⟨k.val,h⟩)) .top','append_forms'),
 ('growth_old_eligibility','DynamicSupport','then old.eligible ⟨k.val,h⟩',
  'then true','append_eligible'),
 ('old_to_new_edge','DynamicGraphs','| .inl _, .inr _ => False',
  '| .inl _, .inr _ => True','path_from_old'),
 ('new_graph_erased','DynamicGraphs','| .inr a, .inr b => fresh a b',
  '| .inr a, .inr b => False','lift_new_path'),
 ('old_code_changed','DynamicIdentity','code := r.code,',
  'code := r.code + 1,','grow_context'),
 ('handle_identity_changed','DynamicIdentity','⟨h.instanceId,left n m h.key,h.identity⟩',
  '⟨h.instanceId,left n m h.key,{h.identity with incarnation := h.identity.incarnation + 1}⟩',
  'grow_checkHandle'),
 ('authority_changed','DynamicIdentity','authority := s.authority, policies :=',
  'authority := {s.authority with revoked := 999 :: s.authority.revoked}, policies :=',
  'grow_authority'),
 ('principal_substitution','DynamicIdentity','⟨u.principal,mapHandle m u.member',
  '⟨u.principal + 1,mapHandle m u.member','grow_context'),
 ('unused_branch_omitted','SupportImpact','| .both p q | .either p q => refs p ++ refs q',
  '| .both p q => refs p ++ refs q\n  | .either p q => refs p','only_refs'),
 ('impact_direction_reversed','SupportImpact','reachable (dependencyBool forms) k j',
  'reachable (dependencyBool forms) j k','affected_exact'),
 ('all_impact_omitted','SupportImpact','changed j && reachable',
  'false && reachable','affected_exact'),

 ('choice_unchecked','CurrentChoice','if checkUse s u e then some e else none',
  'if true then some e else none','offer_sound'),
 ('cursor_rewind','CurrentChoice','position := c.position+j',
  'position := 0','resolve_monotone'),
 ('choice_wrong_index','CurrentChoice','(j+1,v,e)',
  '(j,v,e)','pick_sound'),
 ('name_alias_accepted','NamedCatalog','uniqueNames c.names && (List.finRange n).all',
  'true && (List.finRange n).all','checked_unique'),
 ('missing_name_as_root','NamedCatalog','| .ref name => (lookup names name).map .ref',
  '| .ref name => some .top','compile_sound'),
 ('alternative_as_conjunction','NamedCatalog',
  '| .either p q => do return .either (← compile names p) (← compile names q)',
  '| .either p q => do return .both (← compile names p) (← compile names q)',
  'compile_sound'),
 ('absent_eligibility','NamedCatalog',
  'match lookup c.names name with | some k => c.eligible k | none => false',
  'match lookup c.names name with | some k => c.eligible k | none => true',
  'missing_eligibility_false'),
]

def run(name, text):
    p = out / (name + '.lean')
    p.write_text(text)
    command = ['lean','--trust=0',p.name]
    r = subprocess.run(command,cwd=out,env=env,capture_output=True,text=True)
    (out/(name+'.log')).write_text(r.stdout+r.stderr)
    return dict(command=command,exit=r.returncode,stdout=r.stdout,stderr=r.stderr,
                sha256=hashlib.sha256(p.read_bytes()).hexdigest())

for name,module,old,new,theorem in mutations:
    text=(root/('MirroreaProofFirst'+module+'.lean')).read_text()
    if text.count(old)!=1:
        raise AssertionError((name,'replacement not unique',text.count(old)))
    text=text.replace(old,new)
    lines=text.splitlines()
    begin=next(i+1 for i,l in enumerate(lines) if re.match(r'(?:@\[simp\] )?theorem '+theorem+r'\b',l))
    end=next((i+1 for i in range(begin,len(lines)) if re.match(r'(?:@\[simp\] )?(?:theorem|def|structure|namespace|#print)\b',lines[i])),len(lines)+1)
    r=run(name,text)
    errors=[int(n) for n in re.findall(re.escape(name)+r'\.lean:(\d+):\d+: error:',r['stdout'])]
    rejected=r['exit']!=0 and any(begin<=n<end for n in errors)
    results['mutations'].append(dict(name=name,module=module,theorem=theorem,range=[begin,end],rejected_at_theorem=rejected,old=old,new=new,**r))
    (root/'RESULT.json').write_text(json.dumps(results,indent=2)+'\n')
    print(name,'PASS' if rejected else 'FAIL',flush=True)
    if not rejected:
        raise AssertionError(results['mutations'][-1])
print('baselines',len(results['baselines']),'mutations',len(results['mutations']))
