# 2033 — P-A1-23 practical alpha1 integrated workflow carrier

## Objective

`P-A1-23` として、既存の practical alpha-1 first-floor evidence と bounded operational α-0.5 / α-0.8 / α-0.9 carrier を、product/public-ready と混同しない 1 つの bounded practical α-1 integrated workflow carrier に束ねる。

## Scope and assumptions

- 規範正本は `specs/`、repository memory は `plan/`、snapshot は `progress.md` / `tasks.md` / `samples_progress.md` として扱った。
- scope は source front-door / checker / same-session runtime / typed host-I/O / hot-plug / save-load / session devtools / product-preview evidence の integration まで。
- final public parser / checker / runtime API、final public viewer / telemetry ABI、distributed durable save/load、WAN/federation、native avatar execution、product/public-ready alpha-1 は scope 外。
- `PA1W-*` は bounded practical workflow ready として読み、product-ready alpha-1 とは読まない。

## Start state / dirty state

- start state:
  previous closeout `P-A1-22`; base commit `f644db5`。
- dirty state at start:
  none before the `P-A1-23` RED cycle. During the package, `scripts/practical_alpha1_integrated_workflow.py` and `scripts/tests/test_practical_alpha1_integrated_workflow.py` were added.
- resource check:
  `df -h .` showed `/dev/vda2` 99G total / 30G available / 70% used. `free -h` showed 960Mi RAM and 19Gi swap.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `specs/22-observability-devtools-semantics.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `plan/44-practical-alpha1-roadmap.md`
- `plan/47-operational-alpha09-devtools-roadmap.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`
- `docs/reports/2032-p-a1-22-alpha09-session-bound-devtools-export.md`

## Actions taken

1. Added a RED test for `scripts.tests.test_practical_alpha1_integrated_workflow`; it initially failed because `scripts/practical_alpha1_integrated_workflow.py` did not exist.
2. Added `scripts/practical_alpha1_integrated_workflow.py` with `PA1W-01..08` rows over exact practical first-floor evidence and bounded operational session carriers.
3. Composed the workflow from the existing checker report, local runtime report, α-0.9 session-bound devtools payload, exact product-preview bundles, α-0.5 closeout, and α-0.8 closeout.
4. Fixed stale exact expected evidence in `VIS-A1-01`: runtime-side accepted dispatch already carried `capability_sufficient` and `required_witnesses_present`, while the devtools expected bundle still omitted them.
5. Revalidated exact devtools and product-preview bundles after the expected sync.
6. Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, `samples/practical-alpha1/README.md`, `scripts/README.md`, `specs/18`, `specs/24`, and `plan/44` to record the bounded workflow reading and remaining product/public stop lines.

## Files changed

- `scripts/practical_alpha1_integrated_workflow.py`
- `scripts/tests/test_practical_alpha1_integrated_workflow.py`
- `samples/practical-alpha1/expected/vis-a1-01-event-dag-export.expected.json`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`
- `specs/18-practical-alpha1-scope.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `plan/44-practical-alpha1-roadmap.md`
- `docs/reports/2033-p-a1-23-practical-alpha1-integrated-workflow-carrier.md`

## Commands run

```bash
git status --short
pwd
find . -maxdepth 2 -type d | sort | sed -n '1,80p'
sed -n '1,260p' scripts/practical_alpha1_integrated_workflow.py
sed -n '1,240p' scripts/tests/test_practical_alpha1_integrated_workflow.py
python3 -m unittest scripts.tests.test_practical_alpha1_integrated_workflow
python3 - <<'PY'
import json, sys
sys.path.insert(0, 'scripts')
import practical_alpha1_export_devtools as e
actual=e.build_bundle('VIS-A1-01')
expected=json.loads(open('samples/practical-alpha1/expected/vis-a1-01-event-dag-export.expected.json').read())
print('actual reason_refs=', actual['export_sections']['dispatch_records'][0]['reason_refs'])
print('expected reason_refs=', expected['export_sections']['dispatch_records'][0]['reason_refs'])
print('actual == expected', actual == expected)
PY
python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-01 --format json
python3 scripts/practical_alpha1_product_preview.py run PE2E-01 --format json
python3 scripts/practical_alpha1_product_preview.py run PE2E-07 --format json
python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py check-all --format json
date '+%Y-%m-%d %H:%M %Z'
df -h .
free -h
git log --oneline -5
python3 scripts/practical_alpha1_integrated_workflow.py closeout --format json
python3 scripts/practical_alpha1_integrated_workflow.py run PA1W-08 --format json
python3 -m unittest scripts.tests.test_practical_alpha1_integrated_workflow
python3 scripts/practical_alpha09_devtools.py check-all --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- RED:
  `python3 -m unittest scripts.tests.test_practical_alpha1_integrated_workflow` initially failed with `ModuleNotFoundError` before the script existed.
- Initial integration blocker:
  `VIS-A1-01` exact expected drift. The actual accepted dispatch reason refs were `membership_frontier_current`, `destination_registered`, `capability_sufficient`, `required_witnesses_present`, `local_queue_dispatch_ready`; expected evidence lacked the two positive guard refs.
- `python3 scripts/practical_alpha1_export_devtools.py run VIS-A1-01 --format json`
  passed after syncing the expected bundle.
- `python3 scripts/practical_alpha1_product_preview.py run PE2E-01 --format json`
  passed; a follow-up ad hoc Python postprocess attempted to read a non-existent `source_report_ids` key and failed, but the preview command itself had emitted JSON successfully.
- `python3 scripts/practical_alpha1_product_preview.py run PE2E-07 --format json`
  passed.
- `python3 -m unittest scripts.tests.test_practical_alpha1_integrated_workflow`
  passed; `4` tests passed.
- `python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json`
  passed; `PA1W-01..08` passed and `bounded_practical_alpha1_workflow_ready = true`, `product_public_ready = false`.
- `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
  passed; `VIS-A1-01..07` passed.
- `python3 scripts/practical_alpha1_product_preview.py check-all --format json`
  passed; `PE2E-01..09` passed.
- `python3 scripts/practical_alpha1_integrated_workflow.py closeout --format json`
  passed; closeout reports `product_public_ready = false`.
- `python3 scripts/practical_alpha1_integrated_workflow.py run PA1W-08 --format json`
  passed and preserved non-claims for final public parser grammar, final public checker/runtime API, final public viewer / telemetry ABI, distributed durable save/load, production WAN/federation, native avatar execution, and product/public-ready alpha-1.
- `python3 scripts/practical_alpha09_devtools.py check-all --format json`
  passed; `OA09-01..09` passed and `operational_alpha09_ready = true`.
- `python3 -m unittest scripts.tests.test_validate_docs`
  passed; `11` tests passed.
- `python3 scripts/check_source_hierarchy.py`
  passed; `84/84` required paths present.
- `python3 scripts/validate_docs.py`
  passed; documentation scaffold complete and latest numbered report accepted.
- `cargo fmt --check`
  passed.
- `git diff --check`
  passed.

## What changed in understanding

- The practical alpha-1 line now has a reproducible integrated developer workflow, but the honest category is bounded workflow readiness rather than final product readiness.
- Exact devtools evidence should carry positive capability/witness guard refs when the source runtime report already carries them; omitting them from `VIS-A1-01` was stale evidence, not a runtime regression.
- After `P-A1-23`, the next material gap is public/product boundary definition and `U1`, not another silent first-floor widening.

## Open questions

- What should `U1` mean concretely: repo-local package, installed binary, hosted service, or another shipped surface?
- Whether final public viewer / telemetry ABI should be designed before durable audit backend, or kept behind the same product/public boundary recut.
- Whether accepted detach execution should be reopened before product packaging, or stay later because `HP-A1-07` is currently only an explicit deferred boundary.

## Suggested next prompt

`P-A1-24` として、bounded practical α-1 workflow と product/public-ready α-1 の差を recut し、`U1` packaging / host target / shipped surface / final public parser / CLI / viewer / telemetry ABI の user decision inventory を整理してください。実装を広げる場合も、product-ready claim は user decision 後に限定してください。

## Plan update status

- updated:
  `plan/44-practical-alpha1-roadmap.md`
- reason:
  `P-A1-23`、`PA1W-01..08`、bounded workflow carrier、next product/public boundary recut を repository memory に追加した。

## Documentation.md update status

- updated:
  bounded practical α-1 integrated workflow carrier and remaining product/public stop lines were added to the current snapshot.

## progress.md update status

- updated:
  latest closeout, three-axis progress, line snapshot, validation floor, blockers, and recent log were synchronized to `P-A1-23`.

## tasks.md update status

- updated:
  current task-level status now marks `P-A1-23` closed and promotes `P-A1-24` product/public boundary recut or `U1` decision inventory as the next safe reopen point.

## samples_progress.md update status

- updated:
  added practical α-1 integrated workflow row, `PA1W-01..08` validation anchor, and latest validation log entry.

## Reviewer findings and follow-up

- sub-agent reviewers were not spawned in this turn because the current user request did not explicitly ask for sub-agents and the active tool policy only permits spawning when explicitly requested.
- local focused review:
  checked that the workflow composes real existing helper outputs rather than a thick fake E2E wrapper, verified `PA1W-08` preserves product/public non-claims, and confirmed exact devtools/product-preview bundle checks pass after the `VIS-A1-01` expected sync.
- follow-up:
  product/public boundary and `U1` need an explicit recut before any final public alpha-1 claim.

## Skipped validations and reasons

- Rust focused behavior tests:
  skipped; this package changed Python workflow glue, docs, and one expected JSON evidence bundle. Rust runtime behavior was not changed.
- Docker Compose / transport focused reruns:
  skipped; transport code and Docker fixtures were not changed, and `PE2E-02` product-preview evidence was revalidated through the product-preview script.
- full repo-wide test suite:
  skipped; scope was the practical alpha-1 integrated workflow and exact evidence freshness. Focused Python validations plus required docs/format checks were run.

## Commit / push status

- package commit:
  `3f192a0` `mirrorea: add practical alpha1 workflow carrier`
- push:
  pending at time of this report metadata update

## Sub-agent session close status

- no sub-agents were opened for this package.
