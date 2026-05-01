# Report 1086 — stale validation-command reference audit

- Date: 2026-05-01 12:59 JST
- Author / agent: Codex
- Scope: active validation-command and reader-facing reference freshness
- Decision levels touched: none; wording / command-anchor mirror only

## Objective

Audit active docs and helper/test surfaces for stale validation references after the current-L2 regression floor repair, then correct any active reader-facing command drift without changing sample semantics or public surface commitments.

## Scope and assumptions

- Scope is narrow: active policy docs, reader-facing hands-on / research abstract pages, `plan/27`, snapshot docs, and this report.
- Historical reports, old samples, and normative example history were not rewritten unless an active current reference used the stale command as live guidance.
- Stop line: this package does not claim final public parser/API/ABI, production transport, production theorem/model-check binding, rollback/durable migration, distributed activation ordering, or final viewer/projection surface.
- `projection_codegen_samples.py check-all` is treated as live anchor / manifest alignment validation; `closeout` is treated as manifest inventory evidence.
- `network_transport_samples.py check-all` is treated as executable canary validation; `closeout` is treated as inventory evidence.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `AGENTS.md`
- `scripts/README.md`
- `samples/README.md`
- `samples/current-l2/README.md`
- `samples/lean/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/network_transport_canaries_01.md`
- `docs/hands_on/projection_placement_views_01.md`
- `docs/hands_on/public_api_parser_gate_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `plan/00-index.md`
- `plan/10-roadmap-overall.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/22-network-transport-roadmap.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/current_l2_model_check_carrier_pipeline.py`
- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_model_check_carrier_pipeline.py`

## Actions taken

- Spawned a docs researcher to audit active reader-facing docs and repository memory for stale validation-command anchors.
- Spawned a code mapper to audit active scripts/tests/sample docs for deleted current-L2 regression targets and old command counts.
- Found and corrected `.docs/current-l2-source-sample-authoring-policy.md`, which still listed deleted Cargo target `current_l2_source_sample_emitted_artifact_wiring` as an active regression bundle step.
- Updated `plan/27-public-api-parser-gate-roadmap.md` so the validation floor uses network `check-all`, projection `check-all`, and labels `closeout` as inventory evidence.
- Updated `docs/research_abstract/mirrorea_future_axis_01.md`, `docs/hands_on/public_api_parser_gate_01.md`, and `docs/research_abstract/projection_placement_plan_01.md` to split projection live alignment validation from manifest inventory evidence.
- Updated `docs/hands_on/projection_placement_views_01.md` after review so its prose also separates `check-all` live alignment validation from `closeout` inventory evidence.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` to record the audit and focused validation.
- Added this report.

## Files changed

- `.docs/current-l2-source-sample-authoring-policy.md`
- `docs/hands_on/public_api_parser_gate_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/projection_placement_plan_01.md`
- `docs/hands_on/projection_placement_views_01.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1086-stale-validation-command-reference-audit.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
git branch --show-current
git log -1 --oneline
date '+%Y-%m-%d %H:%M %Z'
sed -n '1,220p' README.md
sed -n '1,240p' Documentation.md
sed -n '1,220p' scripts/README.md
rg -n "current_l2_source_sample_emitted_artifact_wiring|emitted artifact wiring|21/21|22/22|21-step|22-step|21 commands|22 commands|23/23|23-step|model-check carrier" README.md Documentation.md progress.md tasks.md samples_progress.md samples scripts docs/hands_on docs/research_abstract plan .docs specs --glob '!docs/reports/**' --glob '!docs/research_abstract/old/**' --glob '!plan/old/**' --glob '!target/**'
rg -n "network_transport_samples\\.py closeout|network_transport_samples\\.py check-all|projection_codegen_samples\\.py closeout|projection_codegen_samples\\.py check-all|equivalence|inventory evidence|canary" README.md Documentation.md progress.md tasks.md samples_progress.md samples scripts docs/hands_on docs/research_abstract plan .docs specs --glob '!docs/reports/**' --glob '!docs/research_abstract/old/**' --glob '!plan/old/**' --glob '!target/**'
python3 scripts/current_l2_source_sample_regression.py --help
python3 scripts/current_l2_model_check_carrier_pipeline.py --help
python3 scripts/current_l2_guided_samples.py --help
sed -n '1,120p' .docs/current-l2-source-sample-authoring-policy.md
python3 - <<'PY'
import sys
from pathlib import Path
sys.path.insert(0, 'scripts')
import current_l2_source_sample_regression as r
for i, c in enumerate(r.plan_regression_commands(Path('target/audit'), 'audit', python_executable='python3'), 1):
    print(f'{i}. {c.name}: {" ".join(c.argv)}')
PY
rg -n "current_l2_source_sample_emitted_artifact_wiring|emitted artifact wiring test|21/21|22/22|21-step|22-step|21 commands|22 commands" README.md Documentation.md progress.md tasks.md samples_progress.md samples scripts docs/hands_on docs/research_abstract plan .docs --glob '!docs/reports/**' --glob '!docs/research_abstract/old/**' --glob '!plan/old/**' --glob '!samples/old/**' --glob '!target/**'
python3 -m unittest scripts.tests.test_current_l2_source_sample_regression scripts.tests.test_current_l2_model_check_carrier_pipeline
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/network_transport_samples.py closeout --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py closeout --format json
rg -n "current_l2_source_sample_emitted_artifact_wiring|emitted artifact wiring test|21/21|22/22|21-step|22-step|network_transport_samples\\.py closeout --format json|projection_codegen_samples\\.py closeout --format json" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md scripts/README.md docs/hands_on docs/research_abstract plan/00-index.md plan/10-roadmap-overall.md plan/19-repository-map-and-taxonomy.md plan/20-projection-and-placement-roadmap.md plan/22-network-transport-roadmap.md plan/27-public-api-parser-gate-roadmap.md .docs/current-l2-source-sample-authoring-policy.md --glob '!docs/research_abstract/old/**' --glob '!plan/old/**'
rg -n "current_l2_source_sample_emitted_artifact_wiring|emitted artifact wiring test|21/21|22/22|21-step|22-step" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md scripts/README.md docs/hands_on docs/research_abstract plan/00-index.md plan/10-roadmap-overall.md plan/19-repository-map-and-taxonomy.md plan/20-projection-and-placement-roadmap.md plan/22-network-transport-roadmap.md plan/27-public-api-parser-gate-roadmap.md .docs/current-l2-source-sample-authoring-policy.md --glob '!docs/research_abstract/old/**' --glob '!plan/old/**'
rg -n "closeout.*live-anchor|closeout.*alignment|alignment.*closeout|helper/runtime anchor.*closeout|check-all.*live alignment|manifest inventory evidence" docs/hands_on docs/research_abstract plan/27-public-api-parser-gate-roadmap.md .docs/current-l2-source-sample-authoring-policy.md tasks.md samples_progress.md progress.md
```

## Evidence / outputs / test results

Initial state:

```text
$ git status --short
<clean>

$ git branch --show-current
main

$ git log -1 --oneline
a06437d Record repository validation freshness checkpoint
```

Sub-agent evidence:

```text
docs researcher: found active drift in plan/27, docs/research_abstract/mirrorea_future_axis_01.md, docs/hands_on/public_api_parser_gate_01.md, and docs/research_abstract/projection_placement_plan_01.md. No active stale hits for deleted current-L2 target or old 21/22 command counts.

code mapper: found no active stale code/test invocation. current_l2_source_sample_regression.py plans 23 commands; the deleted Cargo target survives only as an intentional negative guard in script unit tests.
```

Current-L2 command reality:

```text
current regression command plan:
1-3: mir-runtime source lowering / runner / verification ladder tests
4: mir-semantics formal hook support test
5-19: detached formal-hook smoke commands
20-21: theorem Lean-stub conformance for e2 and e5
22-23: model-check carrier conformance for e2 and e5
```

Stale-reference search after policy patch:

```text
$ rg ... "current_l2_source_sample_emitted_artifact_wiring|emitted artifact wiring test|21/21|22/22|21-step|22-step|21 commands|22 commands" ...
scripts/tests/test_current_l2_source_sample_regression.py:235: negative guard assertion only

$ rg ... active docs / policy / landing pages for deleted target and old command counts
<no output>

$ rg ... "closeout.*live-anchor|closeout.*alignment|alignment.*closeout|helper/runtime anchor.*closeout|check-all.*live alignment|manifest inventory evidence" ...
only expected split wording remains: `check-all` is live alignment validation and `closeout` is manifest inventory evidence
```

Focused validation:

```text
$ python3 -m unittest scripts.tests.test_current_l2_source_sample_regression scripts.tests.test_current_l2_model_check_carrier_pipeline
Ran 18 tests
OK

$ python3 scripts/network_transport_samples.py check-all --format json
sample_count: 4
passed: NET-02, NET-03, NET-04, NET-05
failed: []

$ python3 scripts/network_transport_samples.py closeout --format json
sample_count: 4
transport_scope: helper_local_process_boundary
transport_seam: loopback_socket
kept_later_gates: real_socket_or_broker, crypto_session_protocol, durable_replay_commit, continuous_shared_runtime_state, final_public_transport_abi

$ python3 scripts/projection_codegen_samples.py check-all --format json
artifact_count: 4
passed: P15-GEN-01, P15-GEN-02, P15-GEN-03, P15-GEN-04
failed: []
artifact_boundary: committed manifest bridge evidence only; not a final emitted executable program

$ python3 scripts/projection_codegen_samples.py closeout --format json
artifact_count: 4
projection_scope: generated_reserve_bridge_evidence
kept_later_gates: final_projection_ir, generated_place_program_emitter, placement_optimizer, cross_place_equivalence_checker, deployment_planner, final_public_emitted_program_abi
```

Docs scaffold and diff:

```text
$ python3 scripts/check_source_hierarchy.py
required: 35
present: 35
missing: 0
all required paths present

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 1084 numbered report(s).

$ git diff --check
<no output>
```

## What changed in understanding

The only active current-L2 deleted-target drift was in the authoring policy; active scripts/tests were already guarded against the deleted target. The remaining reader-facing drift was projection/public-gate wording that treated `closeout` as if it were the live alignment route. The current command hierarchy is now mirrored consistently: network `check-all` is executable canary validation, projection `check-all` is live alignment validation, and `closeout` is inventory evidence.

## Reviewer findings and follow-up

- Finding: `docs/hands_on/projection_placement_views_01.md` still described `projection_codegen_samples.py closeout --format json` as the place where helper/runtime-anchor and manifest alignment is seen.
- Follow-up: updated the hands-on page so `check-all` is the live alignment validation route and `closeout` is manifest inventory evidence.
- Finding: `.docs/current-l2-source-sample-authoring-policy.md` over-scoped theorem Lean-stub and model-check carrier conformance to the `e14/e15` duplicate-cluster pair.
- Follow-up: narrowed that sentence to say `e14/e15` are covered through formal-hook smoke/regression helper, while theorem/model-check conformance remains the representative `e2/e5` pilot in the regression bundle.
- Finding: no other overclaiming about final public parser/API/ABI, production transport, production prover/model-check binding, rollback/durable migration/distributed ordering, or final viewer/projection API was found.

## Open questions

- Actual `U1` commitment remains open.
- Projection final emitted program / optimizer / deployment planner / equivalence checker / public ABI remain deferred.
- Network real socket / session / durable replay / public ABI remain deferred.

## Suggested next prompt

Continue autonomous maintenance by selecting the next low-risk stale-doc or guardrail audit, keeping historical reports and normative examples separate from active command guidance.

## Plan update status

`plan/` 更新済み: `plan/27-public-api-parser-gate-roadmap.md` validation floor now mirrors network `check-all` and projection `check-all` / `closeout` split.

## progress.md update status

`progress.md` 更新済み: added the 2026-05-01 12:59 JST stale validation-command/reference audit log.

## tasks.md update status

`tasks.md` 更新済み: current task-level status now records the stale validation-command/reference audit and current command-anchor split.

## samples_progress.md update status

`samples_progress.md` 更新済み: PH0 / network / projection rows and recent validation row now include report 1086 and the 12:59 JST focused validation.

## Skipped validations and reasons

- Full repository validation floor was skipped because 1085 already ran it immediately before this package and this package only changed docs/policy command anchors.
- Focused command-anchor validation was run for current-L2 regression planning, network canary/inventory commands, projection alignment/inventory commands, docs scaffold, source hierarchy, and diff whitespace.

## Commit / push status

Pending at report write. Intended close command: `git commit --no-gpg-sign` followed by `git push`.

## Sub-agent session close status

- `Beauvoir` (`docs_researcher`): completed and closed; found projection/public-gate active doc drift and no active deleted-target / old-count current-L2 drift.
- `Aristotle` (`code_mapper`): completed and closed; found active scripts/tests consistent with the 23-command regression and deleted target present only as a negative guard.

All package sub-agent sessions are closed.
