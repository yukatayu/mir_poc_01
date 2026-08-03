# Report 2580 — repository understanding: Canon / LAB / code audit

- Date: 2026-08-03 18:13 JST
- Author / agent: Codex
- Scope: Canon-first repository orientation, current-status reading, Rust
  workspace and representative runnable-evidence audit
- Decision levels touched: none. No L0/L1/L2/L3 decision, Canon statement,
  Core/Config/SCN/OBL/Gate/Phase, implementation contract, or public claim was
  changed.

## Objective

Establish a careful, source-backed understanding of what this repository is
trying to build, how its normative and experimental sources are separated,
which implementation surfaces actually exist, and what currently blocks the
next official and semantic steps. Preserve that understanding as additive LAB
evidence without turning an orientation task into a new requirement or status
claim.

## Scope and assumptions

- `mirrorea_canon/` is the sole normative source. Root documentation, legacy
  `specs/`, `plan/`, status snapshots, Rust code, samples, and this report are
  LAB evidence or repository memory.
- The task is an orientation and consistency audit. It does not authorize a
  Canon amendment, a new research lane, a runtime feature, or a lifecycle
  transition.
- The current project must be read on three independent coordinates:
  official Canon lifecycle, parallel LAB semantic integration, and bounded
  runnable LAB evidence.
- Existing runnable surfaces are evidence of implemented bounded cuts. They do
  not by themselves establish Canon conformance, proof discharge, a final
  grammar/API/ABI, or a public-product completion claim.
- Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform are related
  but intentionally separable. This audit preserves that boundary.

## Start state / dirty state

- Branch: `main`.
- Initial `HEAD`: `b6c73dc0a96134774aea5374ddd280c31dffc9ca`.
- Initial `HEAD == origin/main`; worktree and index were clean.
- Repository size was approximately 6.0 GiB. The repository filesystem had
  approximately 23 GiB free of 188 GiB; memory had approximately 9.2 GiB
  available. No heavy build or generated-artifact package was started.
- The initial source cut already contained Report 2579's reader-facing project
  overview. This task independently re-established the underlying Canon/LAB
  and code evidence rather than treating that overview as normative.

## Documents consulted

- Canon entry and hierarchy: `mirrorea_canon/README.md`,
  `mirrorea_canon/MAP.md`, `mirrorea_canon/NORTH-STAR.md`,
  `mirrorea_canon/GLOSSARY.md`, and root `CANON.md`.
- Canon process and decisions: every current `mirrorea_canon/adr/ADR-0001`
  through `ADR-0014`, the ADR index, Canon meta source-hierarchy/agent/style
  rules, and the Canon working-area rules.
- Canon semantics and architecture: every current file in
  `mirrorea_canon/theory/` (`00` through `12`), `spec/` (`01` through `07`),
  `architecture/` (`01` through `05`), and `mental-model/` (`01` through
  `03`), including their indexes.
- Canon conformance and lifecycle: SCN-01 through SCN-10, Canon plans 00
  through 03, Proposal 016, Proposal 017, WRK-0045, and WRK-0046.
- Current LAB entry points: root `README.md`, `Documentation.md`,
  `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, and `.docs/progress-task-axes.md`.
- Relevant LAB repository memory: `plan/00-index.md`, Plans 196, 197, 199,
  and 246; legacy `specs/00`, `01`, `02`, `03`, and `09`; Reports 2576
  through 2579.
- Implementation and runnable-surface entry points: root `Cargo.toml`, all 13
  crate manifests, representative source/module boundaries under `crates/`,
  crate integration tests, `Makefile`, `rust-toolchain.toml`,
  `lean-toolchain`, `scripts/README.md`, and `samples/README.md`.

## Actions taken

1. Read the repository in the mandated Canon-first order and separated
   normative statements from LAB recommendations and runnable evidence.
2. Reconstructed the target system from the North Star, the five derived
   concerns (placement, communication, verification, observation, evolution),
   and the subsystem separation rules.
3. Audited the current official T0/G0 state, the parallel S2-A semantic line,
   the current owner-controlled decisions, and the I1 stop boundary.
4. Mapped the Rust workspace, its dependency direction, implemented modules,
   placeholder boundary crates, test surfaces, CLI commands, and sample roots.
5. Ran representative current-L2, Surface Mir, Mirrorea core-carrier, Cargo,
   and Product Alpha checks to compare current documentation with executable
   behavior.
6. Received a Canon-first read-only planner review before writing this broad
   orientation report. The review confirmed that no new plan or status
   snapshot rewrite is justified by the source cut.

## Files changed

- `docs/reports/2580-repository-understanding-canon-lab-code-audit.md`

No Canon, plan, status snapshot, implementation, sample, or generated artifact
was changed.

## Commands run

- Source discovery and focused reads with `rg`, `sed`, `awk`, `wc`, and Git
  read-only commands.
- Resource audit with `df -h .`, `free -h`, `findmnt -T .`, and scoped `du`.
- `cargo metadata --no-deps --format-version 1`.
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`.
- `python3 scripts/surface_mir_samples.py check-all --format json`.
- `cargo check`.
- `cargo test -p mirrorea-core`.
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json`.
- `make docs`.
- `git diff --check` and focused status/report-heading inspection.

## Evidence / outputs / test results

- The Rust workspace contains 13 crates. The current source files under
  `crates/*/src` total 54,076 lines.
- The substantial implementation/evidence center is `mir-ast`,
  `mir-semantics`, `mir-runtime`, `mirrorea-cli`, and `mirrorea-core`.
  `shared-ids`, `shared-contracts`, `mir-lsp`, `mirrorea-control`, the three
  Prism crates, and `engine-abi` are explicit 13-line placeholder skeletons.
  This is evidence that architectural separability exists in the filesystem,
  not evidence that every subsystem is implemented.
- `cargo check` exited 0.
- `cargo test -p mirrorea-core` exited 0: 28 integration tests passed (12
  carrier/layer tests and 16 membership/place/runtime-substrate tests), with
  zero failures.
- The current-L2 guided smoke suite exited 0 and classified 16 samples across
  typing, ordering/handoff, model-checking, and modal families. Expected valid,
  malformed, pass, and counterexample outcomes were reproduced.
- The Surface Mir suite exited 0 with 53/53 expected rows and no validation
  errors. Its own result remains `workflow_ready: false` and its stop lines
  explicitly deny a final grammar/API, completed distributed runtime, and
  production identity/admission claim.
- Product Alpha package check exited 0 with schema verdict `accepted`, while
  also returning `product_alpha1_ready: false`, six explicit residual
  obligations, and `final_public_api_frozen: false`.
- `make docs` exited 0: the Canon index check found 134 indexed files, source
  hierarchy found all 796 required paths, and documentation validation reported
  a complete scaffold with 1,734 numbered reports.
- The final staged-diff whitespace check produced no findings and included the
  new report.
- These executions support the claim that the repository has broad bounded
  LAB evidence. They do not move official T0, G0-D3, any OBL row, Canon
  conformance, or product/public status.

## What changed in understanding

The most important result is a three-coordinate reading:

1. **Official Canon lifecycle:** the repository is at T0. The adopted v2
   profile has one valid artifact whose verdict is `fail` because fixed
   controls drifted. G0-D3 is deferred, G0 has not exited, and all 28 OBL rows
   in `theory/11` remain `open`.
2. **Parallel LAB semantic integration:** S2-A comparison is complete, but it
   did not choose semantics. SCN-02's two frozen cross-locus dependencies do
   not align cleanly with the one-read worked elaboration shape and the stated
   visibility/read-authority rules. N1/N2/N3 (SCN-02 authority
   reconciliation / C1 choice / C2 choice) therefore need an ordinary
   owner/Canon selection before S2-B model/prototype work.
3. **Runnable LAB evidence:** parser/checker/elaboration/runtime/CLI/sample and
   Lean-oriented evidence is extensive and often executable, but each lane
   carries explicit non-claims. It is neither an official phase transition nor
   a final public product.

The target itself is a specification-first virtual-space system in which
ordinary source-level declarations determine placement and allow communication,
verification obligations, typed observation, and safe evolution to be derived.
Communication is a projection result rather than the starting API. Ownership
is single-authority with explicit requests and handoff; authentication,
authorization, membership, capability, and witness remain distinct. External
I/O stays at typed effect/adapter boundaries, and visualization is a typed,
label/authority/redaction-aware outward effect.

At code level, the repository is not a uniformly implemented product tree. It
is a deliberately uneven evidence stack: working parser/semantic/runtime/CLI
cuts coexist with placeholder crates that reserve subsystem seams. This makes
the filesystem consistent with the architecture, while also making broad
"implemented" or "alpha-ready" readings unsafe unless qualified as LAB.

## Open questions

- Official CP-1: should the owner preserve/defer the fixed-control pins, or
  authorize a normal Canon rebase proposal? Any future artifact/evaluation
  remains a separate CP-2 owner/Canon authorization.
- Official CP-2: after separate authorization of a fresh evaluation and an
  accepted valid `pass` route, what exact G0-D3 evidence and owner acceptance
  opens G0 exit/T1 entry?
- N1: how should SCN-02's two cross-locus reads declare visibility/observe
  authority and failure containment?
- N2: select C1-A-r (target-owner, same-owner sampled update), C1-B
  (requester-determined value), or defer. The current LAB recommendation is
  conditional C1-A-r, not Canon semantics.
- N3: select C2-A-r relation state for request/result/receipt/one-shot use, or
  defer. The current LAB recommendation is directional C2-A-r, not Canon
  semantics.
- How should the future T1/T2 profile and separate I1-readiness/bootstrap
  record reconcile the current C-static entry/exit wording and bind all ten
  scenarios?

## Suggested next prompt

「Report 2580 の三座標を前提に、owner 判断が必要な N1・N2・N3 を一つずつ、現行
Canon、利点、反例、非効果、推奨案に分けて説明してください。まだ Canon や実装は変更
しないでください。」

## Plan update status

`plan/` 更新不要。No semantics, roadmap, workstream sequencing, open-question
set, or current status changed. Plans 196/197/246 already preserve the relevant
long-term repository memory.

## Documentation.md update status

`Documentation.md` 更新不要。Reader entry points and current meanings remain
accurate; this task adds an audit report rather than a new entry point.

## docs/project-status.md update status

更新不要: official lifecycle, semantic checkpoint, blockers, and owner/agent
boundary did not move.

## progress.md update status

`progress.md` 更新不要。This orientation revalidated the existing snapshot but
did not change workflow readiness, evidence classification, remaining Gate,
blocker, macro phase, or feature maturity. Report 2580 is the task-close
evidence.

## tasks.md update status

`tasks.md` 更新不要。The current autonomous package map and owner-decision map
remain unchanged; no new package was opened or promoted.

## samples_progress.md update status

`samples_progress.md` 更新不要。Representative suites were rerun, but no sample
path, command, debug surface, blocker, workflow classification, or dashboard
row changed.

## Reviewer findings and follow-up

- The pre-edit Canon-first planner confirmed the three-coordinate framing and
  the ordered dependency chain: fixed-control disposition -> future valid
  pass/G0-D3 -> T1, plus N1/N2/N3 (SCN-02 authority reconciliation / C1
  choice / C2 choice) -> S2-B -> T1 statements -> narrow T2 -> separate I1
  readiness/authorization.
- It identified two stale-reading hazards: older Plan 196/197 tail text is
  historical when it conflicts with their current top notes/current snapshots,
  and root README `alpha-ready` wording is a LAB workflow classification.
- It confirmed that the current source cut does not justify a new plan or
  snapshot rewrite.
- The final read-only review found four documentary closeout issues: CP-1 and
  CP-2 authority had been slightly conflated, N1/N2/N3 terminology had drifted,
  completed validation had not yet been recorded, and final-review state was
  still marked pending. All four were corrected. It found no other Canon,
  status, code, numeric-evidence, subsystem-separation, or runnable-evidence
  overclaim. The reviewer changed no files.

## Skipped validations and reasons

- Full Product Alpha, Full System V1, operational-product, Docker, native
  bundle, browser/viewer, WAN/federation, and clean-clone release suites were
  not run. They are much broader than a read-only orientation task, and this
  report makes no new claim about those surfaces.
- The full Cargo test workspace was not run. `cargo check`, the focused
  Mirrorea core tests, and representative current-L2/Surface/Product checks
  were selected to verify the architectural reading without reclassifying the
  repository.
- No Lean proof suite was rerun. This report preserves Canon's all-open OBL
  ledger and cites WRK-0046 only as bounded, non-promoted LAB evidence.
- No Oracle consultation was used. The Canon-first local source set and
  mandatory planner review were sufficient for this orientation audit.

## Commit / push status

At report-write time the task change is uncommitted and unpushed. After final
documentation/source-hierarchy validation and close review, the report will be
committed with `--no-gpg-sign`, pushed to `origin/main`, and exact parity will
be reported to the user. This sentence records the pre-commit state and is not
an unresolved success claim.

## Sub-agent session close status

The read-only planner completed both the required pre-edit review and the final
diff review, changed no files, and is closed. No other sub-agent session was
opened.
