# Report 2585 — Mir Theory v0 / I1+ Milestone 4: maintained relation / late projection

- Date: 2026-08-04 13:30 JST
- Author / agent: parent orchestrator with independent planner, theory/formalization, implementer, test-author, and reviewer roles
- Scope: ADR-0015 M4 only — finite maintained relation / fallback / consumer-local late projection calculus and executable/proof evidence
- Decision levels touched: owner-delegated M4 Canon refinement; no owner-reserved decision, public ABI, grammar, wire format, or deployment

## Objective

Close the finite relation-first vertical slice: an authoritative owner keeps a maintained relation and its semantic fallback lineage; a consumer receives the admitted relation DAG and evaluates it only in a coherent local presentation context. The pressure case is B-owned bird follows A.shoulder, with B's shoulder as semantic fallback and C as renderer/consumer.

## Scope and assumptions

M3 EvalPlan is accepted input. M4 adds no final Surface grammar, parser, wire format, external runtime, general save/load implementation, or renderer integration. The selected candidate is owner-held relation plus consumer-local evaluation; the only compared alternative is an absolute bird-pose stream, which violates late materialization and is rejected. Semantic fallback is monotone; a presentation gap is non-semantic and cannot mutate the relation.

## Start state / dirty state

Started from clean pushed M3 revision b15ce1efb7d660fa075294106d8e010a34dab8ac (HEAD == origin/main). No pre-existing user changes or untracked files were present. Resource/toolchain checks preceded generated Lean-inventory work; no heavy external artifact was created.

## Documents consulted

Canon reading followed mirrorea_canon/README.md and MAP.md, then the Constitution, ADR-0015/0018, theory 01/04--07/09/11/13, SCN-08/11, spec 04/05, and current Plan 247. The completed M3 report and the historical PoseGraph helper were LAB evidence only. An external theory consultation was advisory; only its conclusions independently supported by the Canon were adopted.

## Actions taken

1. Selected the owner-held relation candidate, rejecting absolute derived-pose publication. Added PROPOSAL-022, ADR-0019, theory-14, and SCN-12, plus Core/runtime/theory cross-references and the single proof ledger.
2. Defined a domain-neutral relation/binding record with owner-only semantic mutation, a distinct relation activation frontier, expected anchor epochs, guarded primary → fallback selection, and fresh witness/epoch reacquire. The M4 frontier is explicitly not M3's designated-result input_frontier.
3. Defined C's local PresentationContext: each required anchor is released specifically to C, must use one binding frontier and the expected epoch, and contributes to the finite derived-label join before any transform is evaluated. Split, stale, unadmitted, weak-release, overflow, and dependency cycle paths reject without a raw derived transform.
4. Added the finite Lean foundation and its generated active-sample inventory. The sync-test expectation was updated from the stale M3-only command to the exact lean --trust=0 M4 invocation.
5. Implemented the typed deterministic relation-projection reference and RED/GREEN test suites. Semantic owner authority is opaque, issued only at activation, and binds the exact relation, owner, binding epoch, and witness; fallback/reacquire invalidate stale authority.
6. Applied the independent-review corrections once per finding, then obtained a final re-review before status synchronization.

## Files changed

- Canon: PROPOSAL-022, ADR-0019, theory-14, SCN-12, Core/runtime/theory references, indexes, and changelog.
- Formal evidence: samples/lean/foundations/MirTheoryV0M4MaintainedRelationProjection.lean and companion, the Lean inventory sync script/test, and generated manifest.
- Reference behavior: crates/mir-semantics/src/maintained_relation.rs, module export, and the three focused maintained-relation test files.
- Current snapshot files:
  - `plan/247-mir-theory-v0-i1plus-current-roadmap.md`
  - `Documentation.md`
  - `docs/project-status.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- This sole M4 report.

## Commands run

- Canon-first/start-state/resource/toolchain and focused code-map reviews.
- RED/GREEN targeted Rust test development, then cargo test -p mir-semantics --test maintained_relation_projection --test maintained_relation_freshness --test maintained_relation_fallback_privacy.
- cargo fmt --check
- cargo clippy -p mir-semantics --tests -- -D warnings
- cargo test -p mir-semantics
- lean --trust=0 samples/lean/foundations/MirTheoryV0M4MaintainedRelationProjection.lean
- targeted sorry / admit / axiom scan and five #print axioms ledger-lemma inspections
- python3 scripts/current_l2_lean_sample_sync.py
- python3 scripts/tests/test_current_l2_lean_sample_sync.py
- cd mirrorea_canon && python3 meta/build-index.py && python3 meta/build-index.py --check
- make docs, git diff --check, and the documentation validation's secret-scan coverage.

## Evidence / outputs / test results

- Rust focused suite: 13 passed. It covers relation-only publication/no absolute stream, M3-plan separation, coherent same-context relative offset, stale and split-frame rejection, checked transform overflow, finite cycle rejection, presentation-gap nonmutation, monotone fallback/fresh reacquire, exact owner-authority binding/invalidation, and C-specific release/redaction.
- The full mir-semantics suite, format check, and test-target Clippy with -D warnings passed.
- The M4 Lean foundation compiled under --trust=0; direct source scan found no sorry, admit, or user axiom. The five exact #print axioms inspections report no user-defined axiom: the two propositional equalities use standard propext, the monotonicity proof additionally uses standard Quot.sound, and two exact reject/label lemmas use none. OBL-035--039 are therefore lean-proved only for the explicitly stated finite fragments.
- The current Lean sample sync and its 21-test guard passed. Canon index was regenerated and checked at 150 files. Full documentation validation and the independent final review passed with no P0/P1 finding.

## What changed in understanding

Late materialization needs a complete presentation admission boundary, not merely a relation payload. Binding activation frontier, every anchor epoch and release, and the finite label join are semantic preconditions for C's local calculation. Conversely, C's temporary frame gap has no owner-side causal or authority effect. Exact opaque owner authority is necessary to keep a relation binding from becoming a forgeable mutable record.

## Open questions

- OPEN-033 defers arbitrary relation-DAG composition, approximation, and the general label lattice to M5 or later.
- M5 must unify M1--M4 in one non-opaque Core/Config/Step/WF/Trace/Diagnostic model. M6 owns grammar/source spans; M8 owns persistent runtime/save/load/patch behavior.

## Suggested next prompt

No prompt is required. Continue autonomously with M5 shared formal model / modular metatheory, using M1--M4 only as accepted inputs.

## Plan update status

更新済み: Plan 247 marks M4 completed, M5 as the sole active milestone, M6 next, and M5's non-opaque shared-model correspondence as the direct blocker.

## Documentation.md update status

更新済み: the reader-facing entry routes to M5 and records M4's finite relation/projection evidence without claiming grammar, wire/API, runtime, general persistence, conformance, or I1 completion.

## docs/project-status.md update status

更新済み: the derived control view records ADR-0019/theory-14/SCN-12, OBL-035--039's finite status, review path, M5 active boundary, and M4 non-claims.

## progress.md update status

更新済み: the three-axis snapshot, milestone/feature rows, macro phase, and dated recent log now identify M4 close evidence and M5 as the only frontier.

## tasks.md update status

更新済み: the current task map was recut to M5 active / M6 next, with no parallel semantic candidate family and no owner decision pending.

## samples_progress.md update status

更新済み: the M4 Lean foundation, trusted command, generated inventory, and finite-evidence/non-claim boundary are listed as evidence, not product completion.

## Reviewer findings and follow-up

- The pre-edit planner found no escalation trigger and required rejection of hidden absolute materialization, consumer authority, stale re-promotion, split-frame acceptance, privacy weakening, and semantic mutation from a presentation gap.
- The first independent review found five P1 issues plus a cycle-coverage P2: missing binding frontier/anchor-epoch checks, open owner-binding mutation, incomplete label joining, missing C-specific sample release, and saturating arithmetic. Tests and implementation corrected them; finite dependency-cycle rejection was added.
- A narrow second review found one remaining P1: owner authority could be forged or used for a different relation/epoch/witness. Opaque activation-only authority and successor invalidation corrected it. Final re-review found no P0/P1. The external Oracle critique remained advisory, not Canon authority.

## Skipped validations and reasons

- Parser/Surface grammar and source span evidence are M6; checker/elaborator diagnostics are M7; deterministic runtime, durable save/load, and patch are M8; auth/verification extension is M9; release conformance is M10.
- No claim is made for arbitrary relation DAG/naturality, general label noninterference, renderer temporal coherence, final public ABI/wire, or production deployment. The finite Lean and Rust evidence is not generalized.

## Commit / push status

M4 is integrated in one --no-gpg-sign closeout commit, pushed to origin/main, and remote parity was checked after the push.

## Sub-agent session close status

- Pre-edit planner and snapshot-maintenance planner: complete; the latter changed only current planning/status files.
- Theory/formalization: complete; Canon/Lean/index evidence delivered.
- Rust implementer and test author: complete; their production/test ownership remained separate.
- Independent reviewer: complete after final re-review with no P0/P1. The orchestrator remains responsible for this integration, commit, and push.
