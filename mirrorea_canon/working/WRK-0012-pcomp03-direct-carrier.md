---
id: working/WRK-0012
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, arch/02-boundary-contracts, theory/11-metatheory-ledger]
summary: P-COMP-03 の固定一正例・一負例を、既存 Product Alpha world package の MirCompute carrier へ bounded sidecar manifest として載せ、既存 check/run-local で直接実行・拒否できるかを調べる可逆な L3 record。helper、schema、runtime、CLI、public carrier は変更しない。
open_items: []
---

# WRK-0012 - P-COMP-03 direct carrier

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@66229addaa1044c4759a2759b5ef41f355f25d11:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, arch/02-boundary-contracts@66229addaa1044c4759a2759b5ef41f355f25d11:b9ae8932c10e25d4506f8395a1bbd30aaed8d22f6fd2a293f1a0bed022e39cd3, theory/11-metatheory-ledger@66229addaa1044c4759a2759b5ef41f355f25d11:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:plan/170-post-wrk0011-candidate-selection.md@66229addaa1044c4759a2759b5ef41f355f25d11:6c8a85b9be3e3a7bca328068f26e97d9676768db05db9acd5458d4fa1f5c952d, LAB:samples/product-alpha1/computational/control-flow/positive/package.mir.json@66229addaa1044c4759a2759b5ef41f355f25d11:e9f97f2ca90ba3fbb2602bd9407d39eff30275034fdf30fe27a493b66abe35f2, LAB:samples/product-alpha1/computational/variables-scope/negative/package.mir.json@66229addaa1044c4759a2759b5ef41f355f25d11:d6e0fa12796d46d9bb1664eba7a55841ce4cddebdfb5a9997fa2242b424ad274
Permitted LAB locations: plan, samples/product-alpha1/computational/control-flow/positive, samples/product-alpha1/computational/variables-scope/negative
Reserved surfaces: excluded

## Pre-registered working question

Question: Given the pinned authority and LAB-input snapshot, can exactly two bounded non-production world package sidecars, samples/product-alpha1/computational/control-flow/positive/direct-world/package.mir.json and samples/product-alpha1/computational/variables-scope/negative/direct-world/package.mir.json, carry the existing P-COMP-03 MirCompute requests through the existing Product Alpha schema and mirrorea-cli check / run-local route? The positive sidecar must check and run with Computational.ControlFlow.Positive.sum_to(5) = Int(15). The negative sidecar must check, then make run-local exit 2 with the existing MirCompute error classification and an unbound variable detail for Computational.Scope.NegativeUseBeforeDeclare.clamp_zero(3). Direct textual .mir, all other P-COMP-03 rows, and helper execution are excluded.
Status quo: The pinned P-COMP-03 fixtures are computational_helper_row manifests dispatched by the Python matrix helper. Their closed registry has an accepted control-flow row with sum_to(5) summarized as Int(15) and a rejected variables/scope row with clamp_zero(3) classified as containing unbound variable. Existing Product Alpha runtime tests construct valid world packages that directly execute/reject the same registry, but no checked-in P-COMP-03 row has the complete direct package shape. This is an unmodified execution route, not evidence that either selected P-COMP-03 sidecar already works.
Alternative: Either sidecar may fail schema check; the accepted sidecar may fail direct execution or report a different function/output; the rejected sidecar may fail to check, may not return the existing MirCompute/unbound variable rejection, or may return a different exit/result; or an unchanged matrix/regression command may fail. These are bounded carrier outcomes only.
Expected falsifier: Any check/run-local result other than the registered positive and negative classifications, inability to express either fixed module/function with the existing schema, any change needed in a helper, schema, CI/Make target, script, Rust crate, runtime, CLI, or public interface, or any required retained source change outside the two registered direct-world leaves falsifies this experiment and stops it.
Rollback / reopen trigger: On an expected or operational falsifier, set Reliance status to frozen, retain only reproducible permitted LAB evidence, and reopen through a narrower successor or escalation rather than repairing the carrier. Escalate instead of proceeding if an interpretation of rejection phase, .mir/registry equivalence, generic direct execution, semantics, defect, coverage, public interface, contract, OBL, Gate, or Phase would be required.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: workdir="$(mktemp -d /tmp/mirrorea-wrk0012-pcomp03-direct-carrier.XXXXXX)" && positive=samples/product-alpha1/computational/control-flow/positive/direct-world && negative=samples/product-alpha1/computational/variables-scope/negative/direct-world && python3 scripts/mir_computational_samples.py matrix --format json > "$workdir/matrix.json" && python3 scripts/mir_computational_samples.py check-all --format json > "$workdir/check-all.json" && cargo test -p mir-runtime --test product_alpha1_session product_alpha1_run_local_executes_comp03_positive_modules -- --exact && cargo test -p mir-runtime --test product_alpha1_session product_alpha1_run_local_rejects_comp03_negative_modules -- --exact && cargo run -q -p mirrorea-cli -- check "$positive" --format json > "$workdir/positive-check.json" && MIRROREA_ALPHA_SESSION_DIR="$workdir/positive-session" cargo run -q -p mirrorea-cli -- run-local "$positive" --format json > "$workdir/positive-run.json" && cargo run -q -p mirrorea-cli -- check "$negative" --format json > "$workdir/negative-check.json" && ( set +e; MIRROREA_ALPHA_SESSION_DIR="$workdir/negative-session" cargo run -q -p mirrorea-cli -- run-local "$negative" --format json > "$workdir/negative-run.json"; status=$?; set -e; test "$status" -eq 2; ) && python3 -c 'import json,sys; root=sys.argv[1]; p=json.load(open(root+"/positive-check.json")); assert p["verdict"]=="accepted"; p=json.load(open(root+"/positive-run.json")); assert p["mir_computation_claimed"] is True and p["session"]["mir_compute_history"][0]["function_id"]=="sum_to" and p["session"]["mir_compute_history"][0]["output_summary"]=="Int(15)"; n=json.load(open(root+"/negative-check.json")); assert n["verdict"]=="accepted"; n=json.load(open(root+"/negative-run.json")); assert n["status"]=="error" and n["command"]=="run-local" and n["diagnostic_code"]=="MirCompute" and "unbound variable" in n["message"]' "$workdir"
Execution cut: 66229addaa1044c4759a2759b5ef41f355f25d11 is the authority/LAB-input snapshot, not a checkout target because the proposed sidecars do not exist there. Execute only after this registration commit. The subsequent evidence commit is limited by this record's declared locations and the working-annex path validator; it must add only the two exact direct-world leaves and plan/171-wrk0012-pcomp03-direct-carrier-evidence.md plus exact operational metadata.
Non-claims: This does not claim direct textual .mir execution, all P-COMP-03 coverage, helper/sidecar equivalence, the semantic phase of every rejection, language completeness, runtime correctness, a defect, a required repair, a public Product Alpha API, a Canon carrier, contract/conformance status, OBL evidence, SCN, Gate, or Phase movement. It does not change or select a helper, schema, CI/Make surface, script, Rust crate, runtime, CLI, adapter, API, contract, transport, or production behavior.

## Results and review

Reliance status: frozen
Positive evidence: At 2242901a44d3feb7708f82ff535d91bff4fbe143, the registered sequence completed: the existing 15-row matrix/check-all retained its expected classifications, both selected Rust tests passed, the fixed positive sidecar checked and ran with Int(5) -> Int(15), and the fixed negative sidecar checked then exited 2 with MirCompute and message "UnboundVariable: unbound variable `y`". R-2347 preserves this as historical metadata; it is not a substitute evidence artifact.
Negative evidence: The registered positive/negative outcome falsifier did not occur. A separate reproducible operational falsifier did occur: the required plan/171-wrk0012-pcomp03-direct-carrier-evidence.md cannot be admitted by the current numbered-plan validator without changing scripts/validate_docs.py and source-hierarchy registration outside this record's declared locations and stop line. The plan draft was not retained.
Evidence artifacts: LAB:samples/product-alpha1/computational/control-flow/positive/direct-world/package.mir.json@2242901a44d3feb7708f82ff535d91bff4fbe143:af09bf1cf56c341b6f91e7572b0f20c67e8f1b9942730270bdf753fae0da1fa3, LAB:samples/product-alpha1/computational/variables-scope/negative/direct-world/package.mir.json@2242901a44d3feb7708f82ff535d91bff4fbe143:220452b11ea7410f889833e05ee9519b884036bd74b708cd4f401ef1bc5c41b1
Evidence commits: 2242901a44d3feb7708f82ff535d91bff4fbe143
Impact / non-effects: The only retained evidence artifacts are the two exact non-production direct-world/package.mir.json leaves under the declared locations; /tmp outputs and MIRROREA_ALPHA_SESSION_DIR sessions are disposable. The uncommitted numbered-plan draft is not retained. Existing scripts and Rust tests are pinned unmodified execution machinery, not retained LAB inputs or artifacts. No Canon theory, helper, schema, runtime, CLI, public behavior, OBL, Gate, Phase, conformance, or sample-dashboard state changes at registration.
Independent review: not-required-for-L3

## Supersession

Supersession: escalation needed for a separately scoped numbered-plan retention policy or a forward successor with an admissible artifact path; no repair is selected in this frozen record.
