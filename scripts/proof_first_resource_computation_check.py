#!/usr/bin/env python3
"""W2 nonproduction Lean mutation controls; use an external fresh proof workdir.

Requires dependencies compiled by samples/lean/README.md. Copies the candidate
from the repository; does not edit that source or reuse old test results.
No runtime/source/network refinement or W2 acceptance is inferred.
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--lean-path", type=Path, required=True,
                    help="existing directory containing the compiled dependency cone")
parser.add_argument("--work-root", type=Path, required=True,
                    help="existing external directory for a new disposable run")
parser.add_argument("--lean", default="lean")
args = parser.parse_args()
repo = Path(__file__).resolve().parents[1]
parent = args.work_root.resolve(strict=True)
if not parent.is_dir() or parent == repo or repo in parent.parents:
    parser.error("work-root must be an existing directory outside the repository")
dependency = args.lean_path.resolve(strict=True)
if not (dependency / "MirroreaProofFirstPureHandleFunctions.olean").is_file():
    parser.error("compiled PureHandleFunctions dependency is required")
version = subprocess.check_output([args.lean, "--version"], text=True)
if "version 4.29.1," not in version:
    parser.error("this evidence requires Lean4.29.1")
root = Path(tempfile.mkdtemp(prefix="mir-resource-mutants-", dir=parent))
print(root, flush=True)
source_path = repo / "samples/lean/foundations/MirroreaProofFirstResourceComputations.lean"
source = source_path.read_text()
(root / "ResourceComputations.lean").write_text(source)
env = dict(os.environ, LEAN_PATH=str(dependency))
lean = args.lean

def check(path):
    started = time.monotonic()
    command = [lean, "--trust=0", path.name]
    run = subprocess.run(command, cwd=root, env=env, capture_output=True, text=True)
    lines = path.read_text().splitlines()
    failures = []
    for match in re.finditer(r":(\d+):\d+: error[^\n]*", run.stdout):
        line = int(match.group(1))
        declaration = next((s.strip() for s in reversed(lines[:line])
                            if s.startswith(("theorem ", "def ", "example ", "#guard "))), None)
        failures.append(dict(line=line, declaration=declaration, diagnostic=match.group(0)))
    return dict(command=command, exit_code=run.returncode, stdout=run.stdout,
                stderr=run.stderr, failures=failures,
                elapsed_seconds=time.monotonic()-started,
                sha256=hashlib.sha256(path.read_bytes()).hexdigest())

baseline = check(root / "ResourceComputations.lean")
(root / "BASELINE.json").write_text(json.dumps(baseline, indent=2) + "\n")
if baseline["exit_code"] or "sorryAx" in baseline["stdout"]:
    sys.exit("baseline failed; inspect BASELINE.json")
mutants=[
 ('substitute_allowing_context', '| .tick fuel ctx => tick fuel ctx c', '| .tick fuel ctx => tick fuel {ctx with current := {ctx.current with grant := fun _ _ _ _ _ => true}} c', ['act_sound']),
 ('substitute_denying_context', '| .tick fuel ctx => tick fuel ctx c', '| .tick fuel ctx => tick fuel {ctx with current := {ctx.current with grant := fun _ _ _ _ _ => false}} c', ['act_sound']),
 ('shared_update_discard', 'match consumed action with | none => delta | some handle => erase delta handle.id', 'fun _ => none', ['update_untouched','execute_untouched']),
 ('drop_failed_residual', 'pending := frame.pending,position := frame.position', 'pending := rest,position := frame.position', ['tick_realizes','from_start_prefix']),
 ('skip_capture_completion', 'decide (d.captureLabel ≤ d.consumerLabel)', 'true', ['check_exact']),
 ('erase_capture_metadata', '⟨d,if check d then', '⟨{d with captureLabel := 0},if check d then', ['metadata_retained']),
 ('fabricate_capture_value', 'values := .integer x :: s.frame.values', 'values := .integer 0 :: s.frame.values', ['value_retained']),
 ('wrong_argument','arguments := [.integer x]}','arguments := [.integer (x+1)]}',['request_fields']),
 ('ignored_interface','moduleHandle := h.moduleHandle,operation := h.operation','moduleHandle := caller.moduleHandle,operation := caller.operation',['request_fields']),
 ('rights_not_consumed','| some handle => erase delta handle.id','| some handle => delta',['consume_frame','raw_realized']),
 ('discard_unused_rights','⟨s,some failure,[]⟩','⟨{s with frame := {s.frame with rights := fun _ => none}},some failure,[]⟩',['step_failure','resourceStep_rights','run_rights']),
 ('lost_effect_prefix','let later := run fuel contexts (position+1) first.store rest\n     {later with events := first.events ++ later.events}','let later := run fuel contexts (position+1) first.store rest\n     {later with events := later.events}',['run_effect_prefix','run_rights','run_append']),
 ('resume_without_consuming','some {c with phase := .active frame} else none','some c else none',['resume_once','capture_resume','resumed_retired']),
 ('drop_captured_environment','some ({c with phase := .suspended c.nextIdentity frame,nextIdentity :=','some ({c with phase := .suspended c.nextIdentity {frame with lexical := {frame.lexical with values := []}},nextIdentity :=',['capture_frame','capture_resume']),
 ('empty_failure_row','| _ => [.boundaryDenied]','| _ => []',['command_failure_row']),
 ('untyped_resource_name','| .release index => if index < s.references then some s else none','| .release index => some s',['command_exact']),
 ('erase_request_history','shared := {s.shared with resources := out.1},','shared := ⟨out.1,[]⟩,',['resource_transition_exact','resource_evolves','run_history']),
 ('discard_resource_grant','evidence := auth,proof := proof,grant := current.grant}','evidence := auth,proof := proof,grant := fun _ _ _ _ _ => true}',['invocation_exact','allocateOnce_preserves'])
]
results = []
for name, old, new, expected in mutants:
    if source.count(old) != 1:
        raise RuntimeError((name, "mutation needle count", source.count(old)))
    path = root / ("ResourceMutant_" + name + ".lean")
    path.write_text(source.replace(old, new))
    result = check(path)
    result["name"] = name
    result["expected_declarations"] = expected
    result["detected_at_expected_declaration"] = any(
        any(("theorem " + target + " ") in (f["declaration"] or "") or
            ("theorem " + target + "{") in (f["declaration"] or "")
            for target in expected) for f in result["failures"])
    result["passed_control"] = result["exit_code"] != 0 and result["detected_at_expected_declaration"]
    (root / ("MUTANT_" + name + ".json")).write_text(json.dumps(result, indent=2) + "\n")
    results.append({key: result[key] for key in
                    ["name", "exit_code", "sha256", "passed_control", "failures"]})
    print(name, "EXPECTED FAILURE" if result["passed_control"] else "INSPECT", flush=True)
summary = dict(version=version, source_sha256=baseline["sha256"],
               baseline_exit_code=baseline["exit_code"], results=results,
               all_controls=all(r["passed_control"] for r in results))
(root / "RESULTS.json").write_text(json.dumps(summary, indent=2) + "\n")
sys.exit(0 if summary["all_controls"] else 1)
