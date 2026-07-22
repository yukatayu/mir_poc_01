---
id: working/WRK-0013
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, arch/02-boundary-contracts, theory/11-metatheory-ledger]
summary: 凍結した WRK-0012 の二つの固定 direct-world sidecar を入力としてのみ pin し、登録後の fresh execution を既存 unnumbered plan artifact 経路へ独立に保持できるかを調べる可逆な L3 reproduction record。sidecar、validator、helper、schema、runtime、CLI、public carrier は変更しない。
open_items: []
---

# WRK-0013 - P-COMP-03 retention reproduction

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@d8bfbc38bab7a20cfd0574b9f987319944998a12:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, arch/02-boundary-contracts@d8bfbc38bab7a20cfd0574b9f987319944998a12:b9ae8932c10e25d4506f8395a1bbd30aaed8d22f6fd2a293f1a0bed022e39cd3, theory/11-metatheory-ledger@d8bfbc38bab7a20cfd0574b9f987319944998a12:0b423aa49c984386233ae30e958d0b9eda8a36ef6b93a5717ef577ee0455fbd1
LAB inputs: LAB:plan/wrk-0013-retained-reproduction-selection.md@d8bfbc38bab7a20cfd0574b9f987319944998a12:dfa0199a8f5546c82da6a42ca81f83ca202a2cd835d3a0ee7a065149bd12ba7f, LAB:plan/00-index.md@d8bfbc38bab7a20cfd0574b9f987319944998a12:d9914ef53dacb930d6c369e26ede1305497c9f7ccf88126675551cedad872a3a, LAB:samples/product-alpha1/computational/control-flow/positive/direct-world/package.mir.json@2242901a44d3feb7708f82ff535d91bff4fbe143:af09bf1cf56c341b6f91e7572b0f20c67e8f1b9942730270bdf753fae0da1fa3, LAB:samples/product-alpha1/computational/variables-scope/negative/direct-world/package.mir.json@2242901a44d3feb7708f82ff535d91bff4fbe143:220452b11ea7410f889833e05ee9519b884036bd74b708cd4f401ef1bc5c41b1
Permitted LAB locations: plan, samples/product-alpha1/computational/control-flow/positive/direct-world, samples/product-alpha1/computational/variables-scope/negative/direct-world
Reserved surfaces: excluded

## Pre-registered working question

Question: Given exactly the two pinned direct-world/package.mir.json inputs, can a fresh execution performed only after this registration reproduce the fixed classifications and retain that fresh result through exactly plan/wrk-0013-pcomp03-retained-reproduction.md plus its plan/00-index.md entry, while the existing documentation and source-hierarchy validators and execution machinery remain unchanged? The positive input must check and run with sum_to(Int(5)) -> Int(15). The negative input must check, then make run-local exit 2 with MirCompute and an unbound-variable detail. WRK-0012 output, direct textual .mir, all other P-COMP-03 rows, helper execution, and a new carrier claim are excluded.
Status quo: WRK-0012 is frozen. Its two sidecars at 2242901a44d3feb7708f82ff535d91bff4fbe143 are retained artifacts, but its observed outcome in R-2347 is historical metadata only because the registered numbered result path required an excluded validator/source-hierarchy change. The already indexed unnumbered plan/wrk-... convention is a distinct possible retention route, but no WRK-0013 record, fresh output, result memo, or new plan index entry exists yet.
Alternative: Either pinned input may differ from its recorded digest; either fresh check/run classification may differ; the fresh output may not be retainable through the exact unnumbered memo/index delta under unchanged validators; or the old WRK-0012 output may be the only available result. These are provenance and retention outcomes only, not a second carrier result.
Expected falsifier: Any input-digest mismatch; any execution before this committed registration; use of WRK-0012/R-2347 output as successor evidence; a positive or negative result different from the registered classification; inability of exactly plan/wrk-0013-pcomp03-retained-reproduction.md plus its plan/00-index.md entry and direct report to pass unchanged validation; or any required change to a sidecar, validator, source-hierarchy list, helper, schema, script, CI/Make surface, Rust crate, runtime, CLI, public interface, or unrelated plan file falsifies this record and stops it.
Rollback / reopen trigger: On any reproducible falsifier, set Reliance status to frozen, retain no result beyond permitted reproducible evidence, and reopen only through a narrower successor or a separately scoped policy escalation. Do not repair WRK-0012 or this record in place. Escalate rather than proceed if the result would require a semantics, defect, generic direct-execution, rejection-phase, carrier, contract, OBL, Gate, Phase, conformance, or public-workflow interpretation.

## Method and evidence plan

Result class: reproduction
Commands: workdir="$(mktemp -d /tmp/mirrorea-wrk0013-retained-reproduction.XXXXXX)" && positive=samples/product-alpha1/computational/control-flow/positive/direct-world && negative=samples/product-alpha1/computational/variables-scope/negative/direct-world && test "$(sha256sum "$positive/package.mir.json" | awk '{print $1}')" = af09bf1cf56c341b6f91e7572b0f20c67e8f1b9942730270bdf753fae0da1fa3 && test "$(sha256sum "$negative/package.mir.json" | awk '{print $1}')" = 220452b11ea7410f889833e05ee9519b884036bd74b708cd4f401ef1bc5c41b1 && cargo run -q -p mirrorea-cli -- check "$positive" --format json > "$workdir/positive-check.json" && MIRROREA_ALPHA_SESSION_DIR="$workdir/positive-session" cargo run -q -p mirrorea-cli -- run-local "$positive" --format json > "$workdir/positive-run.json" && cargo run -q -p mirrorea-cli -- check "$negative" --format json > "$workdir/negative-check.json" && ( set +e; MIRROREA_ALPHA_SESSION_DIR="$workdir/negative-session" cargo run -q -p mirrorea-cli -- run-local "$negative" --format json > "$workdir/negative-run.json"; status=$?; set -e; test "$status" -eq 2; ) && python3 -c 'import json,sys; root=sys.argv[1]; p=json.load(open(root+"/positive-check.json")); assert p["verdict"]=="accepted"; p=json.load(open(root+"/positive-run.json")); h=p["session"]["mir_compute_history"][0]; assert p["mir_computation_claimed"] is True and h["function_id"]=="sum_to" and h["input_summary"]=="Int(5)" and h["output_summary"]=="Int(15)"; n=json.load(open(root+"/negative-check.json")); assert n["verdict"]=="accepted"; n=json.load(open(root+"/negative-run.json")); assert n["status"]=="error" and n["command"]=="run-local" and n["diagnostic_code"]=="MirCompute" and "unbound variable" in n["message"]' "$workdir"
Retention validation: After only the declared result memo, plan index entry, and direct report are prepared, run `git diff --check`, `python3 scripts/validate_docs.py`, `python3 scripts/check_source_hierarchy.py`, and `(cd mirrorea_canon && python3 meta/build-index.py --check)` before the evidence commit.
Execution cut: d8bfbc38bab7a20cfd0574b9f987319944998a12 is the authority/selection snapshot. Execute only after this registration commit is committed and pushed. The later evidence commit may add only plan/wrk-0013-pcomp03-retained-reproduction.md, its plan/00-index.md entry, and a direct numbered report; a later manifest may append their exact evidence commit and artifact digest to this record without changing the first three sections.
Non-claims: This does not claim a new direct carrier, general P-COMP-03 coverage, direct textual .mir execution, helper/sidecar equivalence, rejection-phase equivalence, language completeness, runtime correctness, a defect, a required repair, a public Product Alpha API, Canon carrier selection, contract/conformance status, OBL evidence, SCN, Gate, Phase, or workflow readiness. It does not modify or select a sidecar, helper, schema, validator, source-hierarchy rule, script, CI/Make surface, Rust crate, runtime, CLI, adapter, API, transport, or production behavior.

## Results and review

Reliance status: not-promoted
Positive evidence: After the registration and its reader-guide-only successor were pushed, the exact registered command ran in a clean detached checkout at ac8e1f3b90e5d33baf025a66b415ce09fa103713. Both pinned input hashes matched. The positive package checked and ran with one sum_to history entry from Int(5) to Int(15); the negative package checked, then run-local exited 2 with MirCompute and the registered unbound-variable detail. The command and its JSON assertions returned 0. The fresh output is retained only through the declared plan memo owned by acf542feb9bb94f5d471054004065cb096517ea8.
Negative evidence: The registered provenance, classification, and retention falsifiers did not occur. The old WRK-0012/R-2347 output was not used; the two inputs matched their pins; the fresh positive/negative classifications matched; and the exact memo/index/direct-report evidence delta passed unchanged documentation, source-hierarchy, and Canon-index validation. This does not convert the reproduction into a general carrier or workflow conclusion.
Evidence artifacts: LAB:plan/wrk-0013-pcomp03-retained-reproduction.md@acf542feb9bb94f5d471054004065cb096517ea8:5e9f078f99570261d5c20469c2484eb5e45e3bd9c24a6cc8b866155fec3e9d75
Evidence commits: acf542feb9bb94f5d471054004065cb096517ea8
Impact / non-effects: The retained artifact is exactly the declared unnumbered plan memo and its index/report operational metadata at the listed evidence commit. The pre-existing sidecars remain pinned LAB inputs, not W13 evidence artifacts; temporary JSON/session/trace output is disposable. No Canon theory, helper, schema, validator, runtime, CLI, public behavior, OBL, Gate, Phase, conformance, or sample-dashboard workflow classification changes.
Independent review: not-required-for-L3

### Evidence addendum — 2026-07-22

The listed evidence commit owns only the declared `plan/` memo, its index entry,
and R-2353. Existing CLI/Rust machinery executed in the fresh checkout remains
unchanged non-production execution machinery. This result is a retained fresh
reproduction/provenance observation only; it neither repairs frozen WRK-0012
nor widens any runtime, language, carrier, diagnostic, workflow, or Canon claim.

## Supersession

Supersession: forward L3 successor to frozen WRK-0012; it does not repair or unfreeze that record.
