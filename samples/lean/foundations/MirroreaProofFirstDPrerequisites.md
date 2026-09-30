# W4-D prerequisite models (LAB)

The selected bounded process design uses one trusted coordinator and one scoped
local-action grant at a time. `MirroreaProofFirstCoordinatorUse.lean` maps every
model transition to `PublicationUse` or a state-preserving step. For every finite
endpoint cohort, an active grant retains the published revision and payload.
Preparing an endpoint does not change its effective installed value; publication
requires all exact prepared acknowledgments, and reopening requires all installs.
Finish matches endpoint, action kind and monotone grant serial. Close prevents
new grants; an existing grant must finish before staging or publishing. There is
no expiry, disconnect, destructor release or network-wait action.

`OwnerStatementHeldAuthority.lean` adds a successor Path that permits a genuine
monotone authority head change while a source request/result is held. The old
idle-only Path remains unchanged. The new path preserves original held state,
waiting, designation, cursor, program, store/history, queues and result collection;
it refines the existing admitted owner/source path. Historical committed results
remain available, while existing current-generation acknowledgment may refuse.
The broader transition does not authenticate an issuer or renew old evidence.

The exact model checks and physical premises remain distinct. Registered control
channel identity, sole requester custody, actual M9 issuer/backend correspondence,
all runtime entrances, numeric capacity/allocation and genuine QUIC result origin
are still required before dependent execution. A matching grant/hash/TLS peer is
not semantic authority. The new models supply no process-death recovery or broad
public observation guarantee, and do not complete D or activate E/Plan250/I3-4.

The 2026-09-30 audit24745 checked197 modules/20278 owned declarations with only
standard Lean axioms and168 qualified false controls (156 prior+12 new). Seven
new/existing publication modules were compiled over190 hash-pinned unchanged C
modules, for176 commands. This is not a fresh197 rebuild. Standalone held proof
95852 and controls successor passed; coordinator proof29931 and3-endpoint controls
passed. Earlier missing-import/type-inference/namespace failures are retained in
the external workdir and were not counted as successes. The new source/audit is
preserved by `docs/proof-first/W4_D_PREREQUISITE_SOURCE_MANIFEST.json`; B and C
manifests remain unchanged. The fresh-copy recipe below subsequently passed as tool5304: all197 modules
rebuilt from source,20278 owned declarations,168 qualified false guards,366
commands. Exact source/log hashes and repository pins were independently checked.
W4_CHECK d_prerequisite_fresh_20260930 and Report2614 retain the receipt.

## Reproduction

Run from the repository root with Lean4.29.1. Set `MIR_PROOF_WORK_ROOT` to an
existing external evidence directory after checking free disk/RAM. This creates
a unique directory, compiles all197 source modules with an empty local import
cache, audits every owned declaration, and checks168 deliberately false guards.
Successful false controls exit1 solely because the guard evaluates false. No
expected output or successful proof is injected into the executable transition.

```python
from pathlib import Path
import ast,datetime,hashlib,json,os,re,resource,subprocess,tempfile
repo = Path.cwd()
assert (repo / "mirrorea_canon/README.md").is_file()
root = Path(os.environ["MIR_PROOF_WORK_ROOT"])
assert root.is_dir()
work = Path(tempfile.mkdtemp(prefix="mir-w4-d-prerequisites-",dir=root))
manifest = json.loads((repo/"docs/proof-first/W4_D_PREREQUISITE_SOURCE_MANIFEST.json").read_text())
sha = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
sources = {}
for row in manifest["rows"]:
    src = repo/row["path"]
    assert sha(src) == row["sha256"]
    dst = work/(row["module"]+".lean")
    dst.write_bytes(src.read_bytes())
    sources[row["module"]] = dst.read_text()
audit = manifest["audit"]
assert sha(repo/audit["path"]) == audit["sha256"]
(work/"DPrerequisiteIntegratedAudit.lean").write_bytes((repo/audit["path"]).read_bytes())
prior = manifest["c_negative_recipe"]
assert sha(repo/prior["path"]) == prior["sha256"]
text = (repo/prior["path"]).read_text()
start = text.index("negative_sources = ")+len("negative_sources = ")
end = text.index("\nfor filename, content",start)
negative_sources = ast.literal_eval(text[start:end])
assert len(negative_sources) == 156
new_negative_sources = {'FalseDActivateBeforePublish.lean': 'import MirroreaProofFirstCoordinatorUseControls\n'
                                     'open MirroreaProofFirst.CoordinatorUse '
                                     'MirroreaProofFirst.CoordinatorUseControls\n'
                                     '#guard (allPrepared.bind fun s => execute evaluate s '
                                     '(.activate 0 41)).isSome\n',
 'FalseDChangedAckAccepted.lean': 'import OwnerStatementHeldAuthorityControls\n'
                                  'open MirroreaProofFirst '
                                  'MirroreaProofFirst.OwnerStatementHeldAuthorityControls\n'
                                  '#guard (delivered.bind fun (s,t) => '
                                  'OwnerStatementSourceCustodian.consume s t 0 7).isSome\n',
 'FalseDHeldHistoryErased.lean': 'import OwnerStatementHeldAuthorityControls\n'
                                 'open MirroreaProofFirst '
                                 'MirroreaProofFirst.OwnerStatementHeldAuthorityControls\n'
                                 '#guard (done.bind fun (s,t) => '
                                 'OwnerStatementResultCollection.collect s t).isNone\n',
 'FalseDHeldNewInvocation.lean': 'import OwnerStatementHeldAuthorityControls\n'
                                 'open MirroreaProofFirst '
                                 'MirroreaProofFirst.OwnerStatementHeldAuthorityControls\n'
                                 '#guard (delivered.bind fun (s,_) => '
                                 'OwnerStatementSourceCustodian.invokeAgain s).isSome\n',
 'FalseDHeldRefreshBanned.lean': 'import OwnerStatementHeldAuthorityControls\n'
                                 'open MirroreaProofFirst '
                                 'MirroreaProofFirst.OwnerStatementHeldAuthorityControls\n'
                                 '#guard queued.isNone\n',
 'FalseDMissingThirdAck.lean': 'import MirroreaProofFirstCoordinatorUseControls\n'
                               'open MirroreaProofFirst.CoordinatorUse '
                               'MirroreaProofFirst.CoordinatorUseControls\n'
                               '#guard (twoPrepared.bind fun s => execute evaluate s '
                               '.publish).isSome\n',
 'FalseDMissingThirdActivation.lean': 'import MirroreaProofFirstCoordinatorUseControls\n'
                                      'open MirroreaProofFirst.CoordinatorUse '
                                      'MirroreaProofFirst.CoordinatorUseControls\n'
                                      '#guard (twoActive.bind fun s => execute evaluate s '
                                      '.reopen).isSome\n',
 'FalseDPreparedActivated.lean': 'import MirroreaProofFirstCoordinatorUseControls\n'
                                 'open MirroreaProofFirst.CoordinatorUse '
                                 'MirroreaProofFirst.CoordinatorUseControls\n'
                                 '#guard (allPrepared.map fun s => '
                                 's.useState.base.barrier.installed 0) = some 41\n',
 'FalseDPublishWhileActive.lean': 'import MirroreaProofFirstCoordinatorUseControls\n'
                                  'open MirroreaProofFirst.CoordinatorUse '
                                  'MirroreaProofFirst.CoordinatorUseControls\n'
                                  '#guard (closing.bind fun s => execute evaluate s '
                                  '.publish).isSome\n',
 'FalseDStaleFinish.lean': 'import MirroreaProofFirstCoordinatorUseControls\n'
                           'open MirroreaProofFirst.CoordinatorUse '
                           'MirroreaProofFirst.CoordinatorUseControls\n'
                           '#guard (heldAgain.bind fun s => execute evaluate s (.finish '
                           'g0)).isSome\n',
 'FalseDWrongEndpointFinish.lean': 'import MirroreaProofFirstCoordinatorUseControls\n'
                                   'open MirroreaProofFirst.CoordinatorUse '
                                   'MirroreaProofFirst.CoordinatorUseControls\n'
                                   '#guard (closing.bind fun s => execute evaluate s (.finish {g0 '
                                   'with endpoint := 1})).isSome\n',
 'FalseDWrongPreparedValue.lean': 'import MirroreaProofFirstCoordinatorUseControls\n'
                                  'open MirroreaProofFirst.CoordinatorUse '
                                  'MirroreaProofFirst.CoordinatorUseControls\n'
                                  '#guard (preparing.bind fun s => execute evaluate s '
                                  '(.preparedAck 0 41 (105,7))).isSome\n'}
assert not negative_sources.keys() & new_negative_sources.keys()
negative_sources.update(new_negative_sources)
for filename, content in negative_sources.items():
    (work/filename).write_text(content)
order = []
visiting = set()
def visit(name):
    if name in order: return
    assert name not in visiting
    visiting.add(name)
    for line in sources[name].splitlines():
        if line.startswith("import "):
            for imported in line[7:].split():
                if imported in sources: visit(imported)
                else: assert imported in {"Std","Lean"}, imported
    visiting.remove(name)
    order.append(name)
for name in sorted(sources): visit(name)
assert len(order) == 197
def limits():
    resource.setrlimit(resource.RLIMIT_AS,(6*1024**3,6*1024**3))
    resource.setrlimit(resource.RLIMIT_CORE,(0,0))
runs = []
for name in order+["DPrerequisiteIntegratedAudit"]+[Path(n).stem for n in sorted(negative_sources)]:
    command = ["lean","--trust=0","-j1","-o",name+".olean",name+".lean"]
    log = work/(name+".log")
    with log.open("xb") as stream:
        process = subprocess.run(command,cwd=work,env=dict(os.environ,LEAN_PATH=str(work)),stdout=stream,stderr=subprocess.STDOUT,preexec_fn=limits)
    text = log.read_text()
    errors = [line for line in text.splitlines() if ": error:" in line or ": error(" in line]
    expected_false = name+".lean" in negative_sources
    runs.append(dict(module=name,exit=process.returncode,command=command,source_sha256=sha(work/(name+".lean")),log_sha256=sha(log)))
    (work/"RUNS.json").write_text(json.dumps(runs,indent=2)+"\n")
    if expected_false:
        assert process.returncode == 1 and len(errors) == 1 and "did not evaluate to `true`" in text,(name,text)
    else:
        assert process.returncode == 0 and not errors and "sorryAx" not in text,(name,text)
counts = re.findall(r"AXIOM_AUDIT_OK (\S+) (\d+)",(work/"DPrerequisiteIntegratedAudit.log").read_text())
assert len(counts) == 197 and len(negative_sources) == 168 and len(runs) == 366
result = dict(at=datetime.datetime.now(datetime.timezone.utc).isoformat(),workdir=str(work),modules=197,owned=sum(int(count) for _,count in counts),qualified_false=168,commands=len(runs),runs=runs)
(work/"RESULT.json").write_text(json.dumps(result,indent=2)+"\n")
print(work,sha(work/"RESULT.json"),result["owned"])
```
