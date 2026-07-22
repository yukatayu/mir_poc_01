---
id: working/WRK-0019
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, arch/02-boundary-contracts, theory/11-metatheory-ledger]
summary: P-COMP-03 arrays-bounds negative の既存 helper input を、非 production の一つの Product Alpha world sidecar で既存 check/run-local 経路へ通したときの固定 MirCompute error 観測を検査する可逆 L3 record。helper、schema、runtime、CLI、public failure carrier は変更しない。
open_items: []
---

# WRK-0019 - P-COMP-03 bounds direct-carrier observation

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@4ea2d008f631d2f62e54645f06e79e0963348154:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, arch/02-boundary-contracts@4ea2d008f631d2f62e54645f06e79e0963348154:b9ae8932c10e25d4506f8395a1bbd30aaed8d22f6fd2a293f1a0bed022e39cd3, theory/11-metatheory-ledger@4ea2d008f631d2f62e54645f06e79e0963348154:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:plan/post-wrk0013-no-candidate-disposition.md@4ea2d008f631d2f62e54645f06e79e0963348154:2da7e0d571e1f4b382147feefe1de969bcc6c89f0d4381caeeac76845b37a9c7, LAB:plan/167-pcomp03-rejection-phase-cross-carrier-audit.md@4ea2d008f631d2f62e54645f06e79e0963348154:0d75bb6355d516da7b0ac2d06b7f63871ba4973703b5bd3032fb985205a2289a, LAB:samples/product-alpha1/computational/arrays-bounds/negative/package.mir.json@4ea2d008f631d2f62e54645f06e79e0963348154:80ac9129eed38a1c50dd8e338c4aae156f04932ca36c9b74c1db2e95d65a7ea0, LAB:samples/product-alpha1/computational/arrays-bounds/negative/arrays-bounds-negative.mir@4ea2d008f631d2f62e54645f06e79e0963348154:26475fa7876b56a93a272d78c3debacb0e96df0b0f56d43082dfbfd140331bab
Permitted LAB locations: plan, samples/product-alpha1/computational/arrays-bounds/negative
Reserved surfaces: excluded

## Pre-registered working question

Question: Given the pinned existing arrays-bounds helper input and the fixed
complete world manifest below, can exactly one new non-production world package
at `samples/product-alpha1/computational/arrays-bounds/negative/direct-world/package.mir.json`
carry `Computational.Arrays.NegativeOutOfBounds.second` with `Int(5)` through
the existing Product Alpha `check` / `run-local` route? Ignoring JSON
whitespace and member order, the sidecar must carry exactly this structured
value:

```json
{
  "schema_version": "mirrorea-product-alpha1-v0",
  "package_id": "computational-arrays-bounds-negative-direct-world",
  "package_version": "0.1.0-alpha.1",
  "package_kind": "world",
  "dependencies": [],
  "effects": ["typed_host_io.read_int", "typed_host_io.write_int"],
  "failures": ["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"],
  "capabilities": ["RunComputationalRow"],
  "witness_requirements": [],
  "membership_requirements": ["active_participant"],
  "auth_policy": {
    "policy_id": "computational-arrays-bounds-negative-direct-world-auth-policy",
    "required_bindings": ["participant_membership"]
  },
  "auth_stack": ["membership_auth", "capability_auth"],
  "contracts": [{
    "contract_id": "computational-arrays-bounds-negative-direct-world-contract",
    "variance": "invariant",
    "effect_row": ["typed_host_io.read_int", "typed_host_io.write_int"],
    "failure_row": ["AdapterUnavailable", "TypeMismatch", "MirComputeRejected"]
  }],
  "observation_policy": {
    "view_role": "observer_safe",
    "labels": ["observer_safe_compute_summary"]
  },
  "redaction_policy": {
    "level": "observer_safe",
    "redacted_fields": ["raw_auth_evidence"]
  },
  "retention_policy": {
    "scope": "computational_session",
    "retained_artifacts": ["checker_report", "runtime_plan", "compute_trace"]
  },
  "message_recovery_policy": {
    "handled_failures": ["reject"],
    "recovery": "reject"
  },
  "savepoint_policy": {
    "classes": ["R0", "R2"],
    "quiescent_required": true
  },
  "runtime_input": {
    "entry_place": "Place[ComputationalHostPlace]",
    "host_input": {
      "adapter_kind": "ReadInt",
      "effect_ref": "typed_host_io.read_int",
      "request_payload": {"kind": "int", "value": 5},
      "expected_response": {"kind": "int", "value": 5}
    },
    "mir_compute": {
      "module_id": "Computational.Arrays.NegativeOutOfBounds",
      "function_id": "second",
      "input_type": "Int64",
      "output_type": "Int64",
      "required_capabilities": ["RunComputationalRow"],
      "failure_tag": "MirComputeRejected",
      "expected_output": {"kind": "int", "value": 0}
    },
    "host_output": {
      "adapter_kind": "WriteInt",
      "effect_ref": "typed_host_io.write_int",
      "request_payload": {"kind": "int", "value": 0},
      "expected_response": {"kind": "int", "value": 0}
    }
  },
  "native_policy": {
    "execution_policy": "disabled",
    "provenance_required": true
  },
  "compatibility": {
    "min_cli_schema_version": "mirrorea-product-alpha1-v0",
    "migration_policy": "alpha_schema_migration_required"
  }
}
```

`check` must accept it; `run-local` must exit 2 with `diagnostic_code`
`MirCompute` and message exactly containing `OutOfBounds: array index 1 is out
of bounds for length 1`. This observes only the existing Product Alpha package
behavior for this one fixed sidecar.
Status quo: The checked-in arrays-bounds negative fixture is a
`computational_helper_row` whose Python helper reports an out-of-bounds
classification. Separately, the closed Rust registry's matching module reaches
an `OutOfBounds` evaluator error after typechecking, while constructed Product
Alpha packages collapse all five P-COMP-03 negatives into `MirCompute`. No
checked-in arrays-bounds direct-world sidecar exists and no package-path
observation for it has been retained.
Alternative: The declared full world manifest may fail schema checking; the
existing package path may emit another exit/status/diagnostic/message; a pinned
input may differ; the command may require an unregistered path; or retention
may require a helper, schema, validator, runtime, CLI, public carrier, or
other reserved-surface change. These are bounded observation outcomes only.
Expected falsifier: Any input-digest mismatch; any candidate command before
this registration is committed and pushed; a `check` rejection; any
`run-local` result other than exit 2 / `MirCompute` with the registered literal
detail; an unregistered execution path; inability to retain only the declared
sidecar, plan memo/index, direct report, and permitted metadata; or any needed
change to a helper, schema, validator, CI/Make surface, Rust crate, runtime,
CLI, public interface, Canon file, Gate, Phase, contract, OBL, or ledger.
Rollback / reopen trigger: On any falsifier, immediately set Reliance status
to frozen, retain only permitted reproducible failure evidence, and do not
adapt the sidecar or command. Reopen only through a distinct narrower record
or escalate if a reserved boundary must be decided. Do not repair WRK-0012,
reinterpret the helper classification, or infer a general direct-carrier,
failure-phase, language, runtime, defect, or workflow conclusion.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: workdir="$(mktemp -d /tmp/mirrorea-wrk0019-pcomp03-bounds-direct-carrier.XXXXXX)" && sidecar=samples/product-alpha1/computational/arrays-bounds/negative/direct-world && test "$(sha256sum plan/post-wrk0013-no-candidate-disposition.md | awk '{print $1}')" = 2da7e0d571e1f4b382147feefe1de969bcc6c89f0d4381caeeac76845b37a9c7 && test "$(sha256sum plan/167-pcomp03-rejection-phase-cross-carrier-audit.md | awk '{print $1}')" = 0d75bb6355d516da7b0ac2d06b7f63871ba4973703b5bd3032fb985205a2289a && test "$(sha256sum samples/product-alpha1/computational/arrays-bounds/negative/package.mir.json | awk '{print $1}')" = 80ac9129eed38a1c50dd8e338c4aae156f04932ca36c9b74c1db2e95d65a7ea0 && test "$(sha256sum samples/product-alpha1/computational/arrays-bounds/negative/arrays-bounds-negative.mir | awk '{print $1}')" = 26475fa7876b56a93a272d78c3debacb0e96df0b0f56d43082dfbfd140331bab && python3 scripts/mir_computational_samples.py matrix --format json > "$workdir/matrix.json" && python3 scripts/mir_computational_samples.py check-all --format json > "$workdir/check-all.json" && cargo test -p mir-semantics --test mir_computational_core declared_comp03_negative_modules_reject_with_stable_reason -- --exact --nocapture && cargo test -p mir-runtime --test product_alpha1_session product_alpha1_run_local_rejects_comp03_negative_modules -- --exact --nocapture && cargo run -q -p mirrorea-cli -- check "$sidecar" --format json > "$workdir/check.json" && ( set +e; MIRROREA_ALPHA_SESSION_DIR="$workdir/session" cargo run -q -p mirrorea-cli -- run-local "$sidecar" --format json > "$workdir/run.json"; status=$?; set -e; test "$status" -eq 2; ) && python3 -c 'import json,sys; root=sys.argv[1]; check=json.load(open(root+"/check.json")); assert check["verdict"]=="accepted"; run=json.load(open(root+"/run.json")); assert run["status"]=="error" and run["command"]=="run-local" and run["diagnostic_code"]=="MirCompute" and "OutOfBounds: array index 1 is out of bounds for length 1" in run["message"]' "$workdir"
Execution cut: `4ea2d008f631d2f62e54645f06e79e0963348154` is the authority/input snapshot. Create only the exact declared sidecar and execute the first outcome command only after this registration commit is committed and pushed. The evidence commit may add only that sidecar, `plan/wrk-0019-pcomp03-bounds-direct-carrier.md`, its `plan/00-index.md` entry, a direct numbered report, allowed working-record metadata/control files, and disposable `/tmp` output must not be retained. A later metadata-only commit may append the exact evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: Observed `OutOfBounds` is retained as LAB evidence only. It is not a public rejection contract, rejection-phase carrier, Phase criterion, Gate criterion, conformance classification, Canon semantic decision, or evidence that the helper and Product Alpha routes are the same implementation path. This does not claim direct textual `.mir` execution, all P-COMP-03 coverage, helper/sidecar equivalence, general direct carrier support, language completeness, runtime correctness, a defect, a required repair, a public Product Alpha API, Canon carrier selection, contract status, OBL evidence, SCN, Gate, Phase, or workflow readiness. It does not change or select a helper, schema, validator, CI/Make surface, script, Rust crate, runtime, CLI, adapter, API, transport, production behavior, Canon theory, or proof ledger.

## Results and review

Reliance status: not-promoted
Positive evidence: none
Negative evidence: none
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: Registration adds no sidecar and executes no candidate command. Existing helper, registry, Product Alpha runtime, and CLI material remain pinned read-only execution machinery until the separately committed evidence package. No Canon theory, helper, schema, validator, runtime, CLI, public behavior, OBL, Gate, Phase, conformance, sample-dashboard workflow classification, or project-completion status changes at registration.
Independent review: not-required-for-L3

## Supersession

Supersession: forward candidate after the post-WRK-0013 reserve disposition; it neither repairs frozen WRK-0012 nor treats WRK-0013 retention evidence as this observation.
