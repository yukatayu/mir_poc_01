# Codex 向け handoff: mir_poc_01 current line と今後の研究・実装進行

配置先: `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`  
作成日: 2026-04-30  
対象: `yukatayu/mir_poc_01`

## 0. この handoff の位置づけ

この文書は Codex / agent が次の研究・実装・検証・docs 更新を自律的に進めるための handoff である。規範正本ではない。規範判断は常に `specs/` にあり、長期 repository memory は `plan/`、current snapshot は `progress.md` / `tasks.md` / `samples_progress.md`、作業証跡は `docs/reports/` に置く。

この handoff に書かれた内容を実作業で使う場合、必要に応じて次へ mirror すること。

- semantic / invariant / decision の変更: `specs/`
- roadmap / long-lived memory / comparison: `plan/`
- current status / task map / sample dashboard: `progress.md`、`tasks.md`、`samples_progress.md`
- reader-facing summary: `Documentation.md`、`docs/research_abstract/`、`docs/hands_on/`
- non-trivial task evidence: 新規 `docs/reports/NNNN-*.md`

`docs/reports/` は本 handoff 作成時の読解対象からは除外したが、通常の Codex 作業では新規 report 作成が義務である。既存 report を上書きしない。

---

## 1. 最初に読む順序

作業開始時は、保持コンテキストがない前提で読む。

1. `README.md`
2. `Documentation.md`
3. `progress.md`
4. `tasks.md`
5. `samples_progress.md`
6. `.docs/progress-task-axes.md`
7. `.docs/continuous-task-policy.md`
8. `.docs/current-l2-source-sample-authoring-policy.md`
9. `AGENTS.md`
10. `specs/00-document-map.md`
11. `specs/01-charter-and-decision-levels.md`
12. `specs/02-system-overview.md`
13. `specs/03-layer-model.md`
14. `specs/09-invariants-and-constraints.md`
15. `specs/04-mir-core.md`
16. `specs/05-mirrorea-fabric.md`
17. `specs/06-prismcascade-positioning.md`
18. `specs/07-typed-effects-wiring-platform.md`
19. `specs/08-cross-system-relations.md`
20. `specs/10-open-questions.md`
21. `specs/11-roadmap-and-workstreams.md`
22. `specs/12-decision-register.md`
23. `plan/00-index.md`
24. relevant `plan/` files, especially `plan/01..11`, `plan/18..38`
25. `samples/README.md`
26. `scripts/README.md`
27. relevant `docs/hands_on/` and `docs/research_abstract/`
28. relevant `crates/` and `scripts/` implementation
29. relevant `sub-agent-pro/*.md` handoff named by the user or task

Do not let `sub-agent-pro/` override `specs/`. It is handoff material, not normative source.

---

## 2. Current project identity

The repo is a specification-first research workspace for four intentionally separable systems.

1. **Mir**: semantic language core for causality, effect, contract, ownership, lifetime, rollback, cut, and safe evolution.
2. **Mirrorea**: distributed control / routing / audit fabric and runtime evolution substrate for executing and evolving Mir-described systems across logical Places.
3. **PrismCascade**: independent media-processing kernel for audiovisual / planning / scheduling work. It is not a Mir sub-runtime.
4. **Typed-Effect Wiring Platform**: operational layer for typed, inspectable, contract-aware, rewritable external effect boundaries.

The current operational axis is:

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

This axis does not collapse the subsystems. Preserve subsystem boundaries unless an explicit decision in `specs/12` changes them.

---

## 3. Current status snapshot

### 3.1 Repo-local alpha-ready current layer

The current reachable layer is a repo-local alpha-ready current layer. It has runnable samples, Rust crates, helper CLIs, generated artifacts, Lean fragments, and validation commands. It is not final public product.

Current active floor:

- `samples/clean-near-end/`: active canonical executable suite
- `samples/current-l2/`: base current-L2 source corpus
- `samples/lean/`: mechanization evidence
- Sugoroku world runtime attachment vertical slice
- avatar follow representative slice
- typed external helper preview
- network canary family
- projection / codegen bridge evidence
- visual debugger / viewer prototype inventory
- hot-plug P21 narrow runtime floor
- storage / LLVM guardrail

### 3.2 Closed current packages

The current snapshot states that `P0..P18`, `P19`, `P20`, `P21`, `R1..R7`, and the post-`P21` docs-first trilogy are closed for current scope.

Key closed items:

- `P19`: engine-neutral `HotPlugRequest` / `HotPlugVerdict` carrier in `mirrorea-core`
- `P20`: thin runtime / report assembly in `mir-runtime`
- `P21`: runtime-side hot-plug engine-state progression narrow floor
- post-`P21` rollback / durable migration family: docs-first first recommendation
- post-`P21` distributed activation ordering family: docs-first second recommendation
- post-`P21` final public hot-plug ABI family: docs-first third recommendation with stop line `freeze prerequisite fixed; public ABI still unfrozen`

### 3.3 Current next open point

The next actual product-shaping reopen point is `U1` actual commitment. The repo currently has no promoted new implementation package beyond maintenance unless the user provides or authorizes public-shape decisions.

`U1` requires decisions on:

1. packaging / installed binary target
2. host integration target
3. first shipped public surface scope
4. final shared-space operational catalog breadth

Current provisional recommendations from `tasks.md`:

- packaging: `library-first`
- host target: `native process`
- first shipped surface: `two-step split`
- shared-space catalog: `minimal subset`

These are recommendations, not decisions. Do not freeze public API / ABI using them unless user explicitly commits or task policy authorizes a narrow provisional branch with explicit non-claim.

---

## 4. Strict non-claims

Never claim any of the following as complete unless fresh evidence and updated specs explicitly support it.

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public auth / adapter / visualization / projection / hot-plug / transport surface
- final public hot-plug ABI
- rollback protocol
- durable migration engine
- distributed activation ordering protocol
- production socket / durable replay transport
- production theorem prover binding
- production model checker binding
- full domain proof discharge for all samples
- low-level `memory_order_*` source syntax
- final witness / provider / emitted-artifact contract
- packaging / installed binary adoption
- FFI / engine adapter / browser host integration
- final viewer API / telemetry service / retention policy
- `auth none` as final auth design
- helper-local preview as public contract
- generated bridge evidence as source sample or final executable program

---

## 5. Source hierarchy and document responsibilities

| Source | Responsibility | Agent handling |
|---|---|---|
| `specs/` | Normative source | Edit only when semantic decision or invariant changes. Mark decision level. |
| `plan/` | Long-lived repository memory | Update when roadmap, package sequencing, current interpretation, or historical comparison changes. |
| `progress.md` | Rough current snapshot | Update after nontrivial checkpoint / validation / task close. Timestamp via `date`. |
| `tasks.md` | Current task map | Rewrite as snapshot; do not append stale chains. Separate self-driven / user decision / research discovery. |
| `samples_progress.md` | Runnable sample dashboard | Update with sample path, validation, progress %, blocker, docs/report. |
| `Documentation.md` | Concise current entry | Keep concise; avoid turning into scratchpad. |
| `README.md` | Short public-ish entry and commands | Keep active commands and non-claims current. |
| `.docs/` | Policy docs | Respect progress axes, continuous task policy, source sample authoring policy. |
| `docs/research_abstract/` | Reader-facing summaries | Mirror current interpretation, not normative changes. |
| `docs/hands_on/` | Command landing pages | Keep runnable commands aligned. |
| `docs/reports/` | Work evidence | Every nontrivial task creates one new report. Never overwrite. |
| `samples/` | Sample corpus | Maintain taxonomy. Do not promote planned skeleton silently. |
| `scripts/` | Repo-local helper / runner / validation | Not final installed CLI. Keep wrappers stable. |
| `crates/` | Rust implementation | Keep current/final boundary explicit. |
| `sub-agent-pro/` | Handoffs / directives | Not normative. Mirror important claims elsewhere. |

---

## 6. Normative theory digest

### 6.1 Decision levels

- `L0`: foundational. Changing it changes the whole stack.
- `L1`: strong architectural direction.
- `L2`: design proposal under refinement.
- `L3`: exploratory or unresolved.

Preserve these labels. Do not silently promote L2 or L3 content to L0/L1.

### 6.2 Layer model

- `L0`: existing OS / network substrate
- `L1`: Mir Core
- `L2`: Mirrorea Fabric
- `L3`: shared space / shared state
- `L4`: domain engines / frameworks, including PrismCascade
- `L5`: applications / communities / Reversed Library

### 6.3 Mir Core

Mir Core fixes semantic event DAG discipline, explicit effects / contracts, ownership / lifetime discipline, and safe evolution principles. Mir-0 currently includes:

- event DAG
- `place`
- minimal effect request operation, currently written as `perform` in companion notation
- effect / contract
- minimal failure space: `Reject`, `Approximate`, `Compensate`
- primitive fallback
- local rollback `try`
- place-local `atomic_cut`
- linear resource discipline

`perform`, `try`, `fallback`, `atomic_cut`, `lease`, `lineage` spellings are current-L2 companion notation unless `specs/12` says otherwise. Do not call them final public grammar.

### 6.4 `atomic_cut`

`atomic_cut` is a place-local rollback frontier. It is not:

- process-wide synchronization
- global fence
- distributed consensus point
- durable commit
- persistence guarantee

### 6.5 fallback / preference chain / lease

Current-L2 reading:

- A fallback chain is a finite ordered list of options over one logical access path / semantic lineage.
- `lease` is an option lifetime guard.
- lease expiry is monotone degradation.
- Earlier options are not resurrected by `try`, rollback, or `atomic_cut`.
- Canonical normalization is left-to-right flattening only when same logical path / lineage and monotone degradation hold.
- Incompatible branches are static rejection when declared evidence suffices.
- Underdeclared branch is surface-level static error in current-L2.
- `lineage(A -> B)` is explanatory / companion notation, not final keyword.

### 6.6 current-L2 typing

Current first strong typing layer is finite and decidable where possible:

```text
Σ ; Ψ ; Γ ; Δ ⊢ e : A @ μ ! ε ⇝ C ; O
```

- `Σ`: user-defined index theory / policies / finite preorders / lattices / powersets / bounds
- `Ψ`: mode / stage / place / visibility / witness / durability environment
- `Γ`: unrestricted context
- `Δ`: linear / affine / capability context
- `A`: ordinary type
- `μ`: mode
- `ε`: effect row
- `C`: finite decidable constraints
- `O`: residual obligations outside first decidable fragment

Domain predicates are not magical builtins. Authority hierarchy, security labels, capture sets, lifetime regions, and cost bounds are user-defined finite theories in samples.

### 6.7 order / handoff

Source principal is not low-level `memory_order_release` etc. Current line uses high-level relations:

- `program_order`
- `dependency_order`
- `publication_order`
- `observation_order`
- `witness_order`
- `finalization_order`
- `scoped_happens_before`

Weak-memory / mutex issues belong to model-check second line rather than being forced entirely into static typing.

### 6.8 Lean and proof bridge

- `samples/lean/foundations/`: actual small proof fragments.
- `samples/lean/clean-near-end/`: generated theorem stubs.

Generated stubs are not proof completion. They are bridge / alignment / mechanization evidence.

### 6.9 Mirrorea

Mirrorea responsibilities:

- logical naming
- routing / route rebinding
- overlay insertion
- patch activation
- compatibility-preserving evolution
- audit / path proof
- dynamic scaling / reconfiguration
- non-Mir integration with explicit boundary semantics

Place is an execution locus, not participant / principal. Preserve transport, authentication, authorization, membership, capability, witness, visualization, and telemetry as separate typed lanes.

### 6.10 PrismCascade

PrismCascade remains an independent kernel. It is not folded into Mir runtime. Integration points should be narrow:

- meta-layer effect provider
- remote execution / resource delegation
- shared trace / audit identifiers
- collaborative graph editing / synchronization

### 6.11 Typed external boundary

Mir core does not have privileged standard I/O. External connection occurs through typed external effect adapter boundary. Do not make `stdin` / `stdout` / `stderr` core primitives without explicit normative change.

Visualization and telemetry are information-bearing effects with label / authority / redaction / retention.

---

## 7. Implementation map

### 7.1 `crates/mir-ast`

Role: parser / AST carrier for non-production current-L2 subsets.

Key file: `crates/mir-ast/src/current_l2.rs`

Current accepted surface includes:

- stage 1 option declarations
- stage 1 chain declarations
- fallback edge rows
- edge-local lineage assertions
- stage 2 `try { ... } fallback { ... }` structural subset
- stage 2 `atomic_cut` head recognition
- stage 3 option-local `admit` helper
- stage 3 predicate fragment / request clause suite helper
- stage 3 `perform on` / `perform via` head helper
- request head + clause bundle inspection

Retained later includes final grammar, span-rich diagnostics, final public parser/checker/runtime surface, full program lowering, theorem/model-check concrete binding.

### 7.2 `crates/mir-semantics`

Role: semantics / proof / model-check bridge.

Known families:

- detached aggregate
- detached bundle
- formal hook
- Lean theorem stub
- model-check carrier
- proof notebook review unit
- static gate
- harness support

Treat emitted artifacts as current bridge evidence, not final public theorem / model-check contract.

### 7.3 `crates/mir-runtime`

Role: current runner / CLI / report-local evidence carrier.

Key components:

- `current_l2.rs`
- `current_l2_cli.rs`
- `clean_near_end.rs`
- `hotplug_runtime.rs`
- bin targets `mir-clean-near-end`, `mir-current-l2`
- runtime tests for current-L2, clean near-end, hot-plug runtime skeleton, source lowering, sample runner, verification ladder

`hotplug_runtime.rs` consumes admitted `mirrorea-core` carriers and logical runtime substrate. It builds:

- `HotPlugRuntimeSkeletonReport`
- `HotPlugRuntimeEngineState`
- `HotPlugRuntimeEngineReport`

Current engine-state progression is narrow and non-public. It covers operation/verdict combinations such as attach accepted/rejected/deferred and detach accepted/rejected/deferred. It does not actualize rollback, durable migration, distributed activation ordering, or final public ABI.

### 7.4 `crates/mirrorea-core`

Role: minimal carrier and logical runtime substrate.

Key files:

- `fabric.rs`: `PrincipalClaim`, `AuthEvidence`, `MessageEnvelope`, `HotPlugRequest`, `HotPlugVerdict`, lane functions.
- `layer.rs`: `LayerSignature`, lane functions, duplicate/conflict handling.
- `runtime.rs`: `MembershipRegistry`, `PlaceCatalog`, `LogicalPlaceRuntimeShell`, snapshots, participant-place helpers.
- `error.rs`, `lib.rs`.

Invariants:

- non-empty validation for important fields
- `MessageEnvelope.transport` must equal `transport_seam` because `transport` is compatibility-only alias
- `HotPlugVerdict.verdict_kind` must be accepted/rejected/deferred
- participant place must be registered with kind `ParticipantPlace` before membership mutation
- rejoin semantics remain unresolved
- place kind replacement is rejected

### 7.5 `scripts/`

Front-door active runners:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/clean_near_end_samples.py closeout
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/network_transport_samples.py closeout --format json
python3 scripts/projection_codegen_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
```

Current-L2 source sample regression flow:

```bash
python3 scripts/current_l2_source_sample_regression.py inventory
python3 scripts/current_l2_source_sample_regression.py regression --run-label <label> --artifact-root <root>
```

Storage guardrail:

```bash
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
```

Do not run destructive cleanup without explicit confirmation. Do not create large directories in repo root when external workdir is not mounted.

---

## 8. Sample taxonomy and execution goals

### 8.1 Active roots

- `samples/clean-near-end/`: active canonical executable suite
- `samples/current-l2/`: base current-L2 source corpus
- `samples/lean/`: mechanization evidence

### 8.2 Planned / historical / generated roots

- `samples/not_implemented/`: planned skeletons only
- `samples/prototype/`: historical prototype / compatibility anchor
- `samples/old/`: archive
- `samples/generated/`: generated bridge evidence / reserve, not source samples

Do not silently promote `not_implemented` samples. Do not silently delete active samples; archive instead.

### 8.3 Important active families

- `PH1`: current-L2 base corpus
- `PH6`: clean near-end suite
- `SUG-01`: runtime attach game
- `SUG-03`: roll publish handoff
- `SUG-05`: late join history visible
- `SUG-08`: reset interleaving model-check
- `SUG-09`: detach TODO boundary, not migration/rollback completion
- `FAIRY-01/02/03/04/06`: avatar follow representative slice
- `FAIRY-05`: planned only, carrier unresolved
- `EXT-03/04`: helper synthetic preview
- `EXT-01/02/05`: planned residual
- `NET-01..05`: helper-local canary family
- `PRJ-01/02`: projection preview
- `P15-GEN-01..04`: generated bridge evidence
- `P16-VIEW-01..05`: viewer inventory
- `P19-HOTPLUG-CARRIER`: hot-plug carrier
- `P20-HOTPLUG-SKELETON`: runtime skeleton
- `P21-HOTPLUG-ENGINE-STATE`: narrow runtime floor
- `STORAGE-01`: storage/backend guardrail

### 8.4 Sample progress rule

`samples_progress.md` uses evidence-backed progress. `100%` is allowed only when current scope has:

- implementation
- positive / negative samples
- debug / visualization
- docs
- report
- tests
- progress update
- git commit / push

Conceptual-only rows must stay at or below `25%`.

---

## 9. Validation floor

### 9.1 Full current validation floor

Run full floor after substantial work, before final commit/push if feasible.

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/clean_near_end_samples.py closeout
python3 scripts/sugoroku_world_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 scripts/network_transport_samples.py closeout --format json
python3 scripts/projection_codegen_samples.py closeout --format json
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
cargo test -p mir-ast
cargo test -p mirrorea-core
cargo test -p mir-runtime
cargo test -p mir-semantics
cargo fmt --check
git diff --check
```

### 9.2 Focused validation examples

Current-L2 sample / source work:

```bash
python3 scripts/current_l2_source_sample_regression.py inventory
python3 scripts/current_l2_source_sample_regression.py regression --run-label <label> --artifact-root <root>
cargo test -p mir-runtime --test current_l2_source_lowering
cargo test -p mir-runtime --test current_l2_source_sample_runner
cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder
```

Hot-plug runtime work:

```bash
cargo test -p mirrorea-core
cargo test -p mir-runtime --test hotplug_runtime_skeleton
```

Docs / dashboard work:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

Storage guardrail:

```bash
df -h
free -h
lsblk -f
findmnt
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
```

If a validation is not run, state it in the report and final answer. Do not claim success.

---

## 10. Sub-agent usage plan

Use sub-agents actively, but keep final responsibility in the main agent. Close unused or unknown sub-agent sessions when work no longer needs them. Do not delete sub-agent config files casually; close sessions / record unused status instead.

### 10.1 Suggested roles

- `docs_researcher`: read `specs/`, `plan/`, `.docs/`, `docs/research_abstract/`, `docs/hands_on/`; identify stale claims and missing mirrors.
- `code_mapper`: map relevant crates, scripts, tests, fixtures, and sample paths before changing implementation.
- `implementer`: make narrow patches only after current theory / plan / tests are clear.
- `test_author`: add focused tests, fixtures, negative cases, regression coverage.
- `eval_runner`: run validation floor, capture command output, detect warnings / failures.
- `reviewer`: inspect diff for stale claims, overclaims, architecture collapse, missing docs/report, missing validation.
- `status_reporter`: update `progress.md`, `tasks.md`, `samples_progress.md`, and summarize package close.
- `discord-report` skill: use only if repo policy / task scope requires it and config is present. Begin before edits if using it. Notification failure is not main task failure.

### 10.2 Sub-agent close rule

At the end of each package:

1. list active sub-agent sessions / roles used;
2. close those no longer needed;
3. keep only sessions with explicit context value;
4. record in report that unused / unknown sub-agents were closed or not used;
5. do not leave hung sub-agents unreported.

---

## 11. Git / report discipline

### 11.1 Start of task

```bash
git status --short
git branch --show-current
git log -1 --oneline
```

Do not overwrite uncommitted user changes. If there are uncommitted changes:

- identify whether they are part of current task, forgotten prior work, or unrelated user work;
- include clearly related forgotten changes in the relevant commit if safe;
- split unrelated changes into separate commit or leave untouched, documenting decision;
- never silently discard.

### 11.2 Report requirement

Every non-trivial task must create a new report under `docs/reports/NNNN-*.md`.

Report should include:

1. title and identifier
2. objective
3. scope and assumptions
4. documents consulted
5. actions taken
6. files changed
7. evidence / outputs / tests
8. what changed in understanding
9. open questions
10. suggested next prompt
11. plan/progress/tasks/samples_progress update status
12. skipped validations and reasons
13. commit / push status

Use `scripts/new_report.py` if available / appropriate.

### 11.3 Commit / push

Use non-GPG commit by policy:

```bash
git add <files>
git commit --no-gpg-sign -m "<concise message>"
git push
```

Commit and push after each closed package unless user explicitly says not to. If push fails due credentials/network, report honestly and keep commit local.

---

## 12. Maintenance lane that can proceed without U1

The current repo says no new self-driven post-P21 docs-first family remains promoted. Still, the following are safe self-driven maintenance tasks.

1. Docs freshness / stale-current audit
   - `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `docs/research_abstract/`, `docs/hands_on/`, relevant `plan/`.
   - Validation: `check_source_hierarchy.py`, `validate_docs.py`, `git diff --check`.
   - Stop line: do not create normative decisions in snapshot docs.

2. Runnable dashboard refresh
   - Re-run closeout helpers.
   - Update `samples_progress.md` only with evidence-backed status.
   - Do not mark conceptual rows above `25%`.

3. Rust formatting / regression repair
   - `cargo fmt --check`, affected `cargo test`.
   - Do not mix feature work with formatting-only commit unless clearly necessary.

4. Storage guardrail check
   - Audit disk/mount/workdir/LLVM staging.
   - No destructive cleanup / mount / format without explicit confirmation.

5. Stale active-reference cleanup
   - Search for wording that claims final public completion or old active paths.
   - Replace with current alpha / helper-local / planned / generated / archive classification.

---

## 13. Research directions after current snapshot

The next major work depends on whether `U1` actual commitment is available. Use the following branching discipline.

### 13.1 If U1 is not committed

Do not freeze public API / ABI. Continue with safe research-discovery and maintenance.

Possible packages:

- `VerificationLayer` law inventory and boundary matrix refresh.
- `FAIRY-05` reopen criteria package without implementation promotion.
- projection equivalence evidence inventory beyond generated bridge manifest.
- rollback / durable migration pre-state-machine comparison, explicitly docs-first.
- distributed activation ordering evidence inventory, without protocol completion claim.
- production transport / durable replay risk matrix, preserving transport/auth/membership/capability/witness separation.
- parser grammar freeze readiness checklist, not final grammar freeze.
- stale docs and sample dashboard maintenance.

Each package must have stop lines and validation.

### 13.2 If U1 is committed with provisional recommendations

Assume user accepts:

- library-first packaging
- native process host integration first
- two-step public surface split
- minimal shared-space operational catalog

Then first implementation tranche can target a narrow public-core library surface, not a full product. Candidate package sequence:

1. Public-core internal API boundary draft
   - Create or update `plan/` and `specs/10/11/12` with exact non-final / candidate status.
   - Identify Rust crate export boundaries.
   - Do not freeze final parser grammar.

2. Parser/checker/runtime/verifier narrow public-core prelude
   - Expose a minimal library-facing API for current-L2 fixed subset if scope permits.
   - Keep helper-local CLI separate.
   - Add tests and docs.

3. Native process host boundary first cut
   - Use typed external boundary helper evidence.
   - Define exact host-boundary candidate schema in docs-first package before code.
   - Preserve transport/auth/membership/capability/witness separation.

4. Hot-plug public ABI preparation
   - Do not export runtime-private names directly as public ABI.
   - Create mapping table: helper-local / runtime-private / candidate-public names.
   - Use `plan/38` stop line.

5. Minimal shared-space catalog
   - Promote only a minimal representative subset from Sugoroku / avatar evidence.
   - Keep fairness / quorum / exhaustive catalog later.

### 13.3 If user chooses different U1 options

Do not override user choice. Recompute plan based on chosen packaging / host / surface / catalog. Update:

- `tasks.md` blockers
- `progress.md` macro phase and self-drive status
- `samples_progress.md` if validation surface changes
- relevant `plan/` files
- relevant `docs/hands_on/` landing pages
- new report

---

## 14. Concrete package templates

### 14.1 Documentation freshness package

Objective: remove stale current claims and sync status.

Steps:

1. Read required docs.
2. Run `git status --short`.
3. Search for stale terms: `final public complete`, `completed engine`, `active sample` old paths, old phase catch-all, helper preview as API, etc.
4. Update snapshot docs only when source-backed.
5. Create report.
6. Validate:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

7. Review diff.
8. Commit / push.

### 14.2 Current-L2 source sample package

Objective: add or update a current-L2 source sample row.

Preconditions:

- Read `.docs/current-l2-source-sample-authoring-policy.md`.
- Decide authored row vs deferred target-only row.
- Do not promote prototypes or not_implemented silently.

Steps:

1. Edit `samples/current-l2/<stem>.txt` if authored.
2. Align fixture / expectation.
3. Update `samples/current-l2/README.md` mapping matrix / ladder row.
4. Update docs / plan / dashboard if current status changes.
5. Run inventory/regression.
6. Add report.
7. Commit / push.

### 14.3 Hot-plug narrow runtime package

Objective: extend narrow runtime-side evidence without claiming rollback/durable/distributed/public ABI completion.

Preconditions:

- Read `plan/33..38`.
- Read `crates/mirrorea-core/src/fabric.rs`, `runtime.rs`, `crates/mir-runtime/src/hotplug_runtime.rs`.

Allowed work:

- Add narrow state / report fields if tied to admitted request/verdict and runtime snapshot.
- Add tests for accepted / rejected / deferred combinations.
- Add docs describing retained-later refs.

Disallowed without user/U1:

- final public ABI naming
- rollback protocol completion
- durable migration engine
- distributed activation ordering protocol
- production transport assumptions

Validation:

```bash
cargo test -p mirrorea-core
cargo test -p mir-runtime --test hotplug_runtime_skeleton
cargo fmt --check
git diff --check
```

### 14.4 Viewer / telemetry package

Objective: improve typed viewer inventory without final API claim.

Must preserve:

- label
- authority
- redaction
- retention_scope
- source_refs
- focus_refs / channel / value_summary as applicable

Disallow:

- raw trace fallback for observer-safe route trace
- untyped debug leak claim
- final viewer API wording

Validation:

```bash
python3 scripts/visual_debugger_viewer_samples.py closeout --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
git diff --check
```

---

## 15. Common failure modes to avoid

1. Claiming final public completion from helper-local evidence.
2. Treating generated theorem stubs as completed proofs.
3. Treating generated projection manifest as source sample or final executable.
4. Collapsing auth / membership / capability / witness into transport.
5. Treating visualization / telemetry as debug-only untyped leakage.
6. Promoting `not_implemented/` sample into active path without docs / dashboard / validation.
7. Updating `progress.md` without `tasks.md` when task status changes.
8. Updating sample runner without `samples_progress.md` when sample status changes.
9. Adding normative decisions in `Documentation.md` or `progress.md` instead of `specs/`.
10. Forgetting new report for nontrivial task.
11. Forgetting commit / push.
12. Running destructive cleanup or heavy build in repo root.
13. Moving active scripts without wrapper / alias.
14. Deleting historical samples instead of archiving.
15. Leaving stale “current active” references after path changes.

---

## 16. Desired autonomous loop

Use this loop repeatedly.

1. **Read / refresh**  
   Read required source hierarchy. Check current git status.

2. **Classify task**  
   Determine whether task is maintenance, docs-first research, narrow implementation, sample authoring, or user-decision dependent.

3. **Set stop lines**  
   Write down what this package will not claim.

4. **Use sub-agents**  
   Delegate docs, code map, implementation, tests, eval, review as appropriate.

5. **Patch narrowly**  
   Avoid broad restructuring unless task explicitly requires it.

6. **Validate**  
   Run focused tests first, full floor when needed.

7. **Mirror docs**  
   Update `specs/`, `plan/`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `docs/hands_on/`, `docs/research_abstract/` as appropriate.

8. **Write report**  
   New `docs/reports/NNNN-*.md` with evidence and skipped validations.

9. **Review**  
   Use reviewer or local diff audit for overclaims / stale docs / missing validation.

10. **Commit / push**  
    Include forgotten related changes if safe. Use `git commit --no-gpg-sign`. Push.

11. **Reflect**  
    Every package close: update current task map and decide next package. Every few packages: rerun broad validation and re-read macro plan.

Do not stop simply because one package closed if user requested continuous autonomous work and next package is safe. Stop only when requested scope is complete or a real user decision is required.

---

## 17. Current recommended next actions

Given the current snapshot, the safest next actions are:

1. Run a docs freshness audit and validation rerun.
2. Confirm no stale final-public or completed-engine claims remain after post-`P21` trilogy.
3. Keep `U1` actual commitment open and explicit.
4. Prepare a user-facing `U1` decision matrix if asked, but do not freeze without confirmation.
5. If implementation is requested without U1, choose a narrow maintenance / verification package rather than public API freeze.
6. Maintain commit / push discipline.
