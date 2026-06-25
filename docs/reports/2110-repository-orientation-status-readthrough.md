# Report 2110 — repository orientation status readthrough

- Date: 2026-06-25T07:18:03.138378Z
- Author / agent: Codex
- Scope: Repository orientation / current status read-through / management-rule consolidation
- Decision levels touched: Read-only. No L0 / L1 / L2 / L3 decisions changed.

## Objective

この repository の内容、現状、理想、目指すところ、管理方法を、次の作業へ入る前の
orientation として読み取る。

この report は新しい仕様判断を作らない。規範判断の正本は引き続き `specs/`、
長期 repository memory は `plan/`、current snapshot は `progress.md` / `tasks.md` /
`samples_progress.md` に置く。

## Scope and assumptions

- ユーザ依頼は「まず読む」ことであり、仕様・コード・sample behavior の変更は scope 外とした。
- 読取対象は、AGENTS.md の指定順に従い、root docs、current status docs、normative specs、
  relevant `plan/`、sample/script taxonomy docs、reader-facing docs、report template とした。
- historical `docs/reports/` 全件の逐語読了、全 crate の code audit、全 sample execution は scope 外。
  ただし最新 closeout chain、source hierarchy、validation surface、sample taxonomy は snapshot docs と
  roadmap docs から確認した。
- `docs/research_abstract/README.md` には一部古い deferred wording が残っているため、
  current status 判定では `progress.md` / `tasks.md` / `samples_progress.md` と latest reports を優先した。
- 作業時点の current date は 2026-06-25、timezone は Asia/Tokyo。

## Start state / dirty state

- 開始時 `git status --short` は空で、worktree は clean。
- `discord-report` skill の task baseline を `begin` で記録した。
- report 作成後の dirty state は、この report file の追加のみ。

## Documents consulted

- Required root / snapshot docs:
  `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`.
- Required normative docs:
  `specs/00-document-map.md`, `specs/01-charter-and-decision-levels.md`,
  `specs/02-system-overview.md`, `specs/03-layer-model.md`,
  `specs/09-invariants-and-constraints.md`.
- Normative subsystem specs:
  `specs/04-mir-core.md` through `specs/08-cross-system-relations.md`,
  `specs/10-open-questions.md` through `specs/43-surface-mir-v1-alpha-scope.md`.
- Decision register:
  `specs/12-decision-register.md`; dense table summary was checked for `D-001..D-203`.
- Progress / task policy docs:
  `.docs/progress-task-axes.md`, `.docs/continuous-task-policy.md`,
  `.docs/current-l2-source-sample-authoring-policy.md`.
- Repository memory:
  `plan/00-index.md`, `plan/01-status-at-a-glance.md`,
  `plan/02-system-overview-and-positioning.md`,
  `plan/03-decision-strengths-and-boundaries.md`,
  `plan/04-core-semantics-current-l2.md`,
  `plan/05-fallback-lease-and-chain-semantics.md`,
  `plan/06-surface-notation-status.md`,
  `plan/07-parser-free-poc-stack.md`,
  `plan/08-representative-programs-and-fixtures.md`,
  `plan/09-helper-stack-and-responsibility-map.md`,
  `plan/10-roadmap-overall.md`,
  `plan/11-roadmap-near-term.md`,
  `plan/12-open-problems-and-risks.md`,
  `plan/14-glossary-and-boundary-rules.md`,
  `plan/17-research-phases-and-autonomy-gates.md`,
  `plan/19-repository-map-and-taxonomy.md`,
  `plan/50-product-alpha1-public-boundary-roadmap.md`,
  `plan/51-operational-product-sample-roadmap.md`,
  `plan/57-autonomous-computational-core-master-plan.md`,
  `plan/58-full-system-v1-roadmap.md`,
  `plan/64-surface-mir-placement-roadmap.md`,
  `plan/65-indexed-state-roadmap.md`,
  `plan/66-role-admission-roadmap.md`,
  `plan/67-source-patch-hotplug-roadmap.md`,
  `plan/68-surface-full-system-v1-roadmap.md`,
  `plan/90-source-traceability.md`, `plan/91-maintenance-rules.md`.
- `plan/` overview:
  headings for all `plan/*.md` were scanned to confirm coverage and routing.
- Sample / script taxonomy:
  `samples/README.md`, `scripts/README.md`.
- Reader-facing docs:
  `docs/hands_on/README.md`, `docs/research_abstract/README.md`.
- Report policy:
  `docs/reports/TEMPLATE.md`, `scripts/new_report.py`.
- Skill docs:
  `superpowers:using-superpowers`, repo-local `discord-report`.

## Actions taken

- Followed the repository read order: root docs, current status, normative specs, then subsystem / memory docs.
- Cataloged markdown, specs, plans, reports, sample roots, script roots, and top-level directories.
- Read the current state snapshot and compared it with long-term repository memory.
- Identified current active evidence roots, planned-only roots, archive/generated roots, and reader-facing docs roots.
- Created this report with `scripts/new_report.py`.
- No code, specs, snapshot docs, sample files, or plan files were modified.

## Files changed

- Added `docs/reports/2110-repository-orientation-status-readthrough.md`.

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short`
- `git branch --show-current`
- `git log -1 --oneline`
- `rg --files -g '*.md' | sort`
- `find specs -maxdepth 2 -type f | sort`
- `find plan docs .docs samples scripts -maxdepth 2 -type f | sort`
- `find . -maxdepth 2 -type d | sort | sed -n '1,220p'`
- `wc -l README.md Documentation.md progress.md specs/00-document-map.md specs/01-charter-and-decision-levels.md specs/02-system-overview.md specs/03-layer-model.md specs/09-invariants-and-constraints.md`
- `wc -l tasks.md samples_progress.md .docs/progress-task-axes.md .docs/continuous-task-policy.md .docs/current-l2-source-sample-authoring-policy.md docs/reports/TEMPLATE.md`
- `wc -l plan/*.md`
- `wc -l specs/[0-9][0-9]-*.md`
- `wc -l specs/00-document-map.md specs/10-open-questions.md specs/11-roadmap-and-workstreams.md specs/12-decision-register.md`
- `sed -n ...` reads over the documents listed above.
- `rg -n '^#{1,3} ' plan/*.md`
- `awk -F'|' '/^\\| D-[0-9]+ / { ... }' specs/12-decision-register.md`
- `ls docs/reports | sort | tail -30`
- `python3 scripts/new_report.py --slug repository-orientation-status-readthrough`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `git diff --check`

## Evidence / outputs / test results

- Initial `git status --short` was empty.
- Latest numbered report before this task was `2109-p-surf-99-final-surface-alpha-audit.md`.
- Current branch is `main`.
- Latest commit at read time: `94a901c0 add: sub-agent-pro`.
- Directory audit confirmed the major top-level roots:
  `specs/`, `plan/`, `docs/`, `crates/`, `samples/`, `scripts/`, `.docs/`, `sub-agent-pro/`.
- Documentation scaffold validation passed:
  `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and
  `Found 1262 numbered report(s).`
- Source hierarchy validation passed:
  `python3 scripts/check_source_hierarchy.py` reported `required: 546`, `present: 546`, `missing: 0`.
- Report/docs validator unit tests passed:
  `python3 -m unittest scripts.tests.test_validate_docs` ran 18 tests with `OK`.
- Whitespace / patch sanity passed:
  `git diff --check` exited 0 with no output.
- Product / operational / Surface / Full System V1 runnable validation suites were not executed;
  this task was an orientation read-through, not evidence refresh.

## What changed in understanding

Repository identity:

- The project is specification-first. It is not a single runtime or one product line.
- The project axis is:
  `正しい理論に基づき、正しく hot-plug でき、Place をまたいで実行・通信・検証・可視化できる仮想空間システム`.
- The intended architecture keeps Mir, Mirrorea, PrismCascade, Typed-Effect Wiring Platform,
  and upper shared-space / application layers separable.

Source hierarchy:

- `specs/` is the normative source.
- `plan/` is long-lived repository memory and must not become scratchpad.
- `docs/reports/` is chronological task evidence.
- `progress.md`, `tasks.md`, and `samples_progress.md` are current snapshots / dashboards.
- `samples/README.md` and `scripts/README.md` are taxonomy controls.
- `docs/hands_on/` and `docs/research_abstract/` are reader-facing entry points.
- `sub-agent-pro/` is working directive / handoff material, not normative source.

Core invariants and boundaries:

- Patch evolution is disciplined as a DAG; hidden backward edges and silent API shadowing are prohibited.
- Default evolution is safe downstream addition and compatibility-preserving overlay.
- Contracts, explicit failure behavior, ownership, lifetime, witness, and capability boundaries must stay visible.
- `atomic_cut` is a place-local rollback/finalization frontier, not global synchronization or distributed durability.
- Standard I/O is not a Mir core primitive; external world connection belongs at typed effect / adapter boundary.
- Authentication, authorization, membership, capability, and witness must not be collapsed into transport.
- Visualization / telemetry are typed information-bearing effects, not untyped debug leaks.
- PrismCascade remains a separable media kernel; it should not be folded into Mir runtime semantics.

Current status:

- There is no current promoted package after `P-SURF-99`.
- Product Alpha-1 is bounded alpha evidence with local/Docker workflow, release check, installed-binary probe,
  native host launch bundle, non-final viewer, R0/R2 save/load, and `mirrorea-alpha` command family.
- The operational product suite is workflow-ready in bounded local/Docker alpha scope:
  `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation`.
- Full System V1 source-first line is closed through `P-FSV1-99` as bounded release-check / final audit evidence.
- Surface Mir alpha source-authority line is closed through `P-SURF-99` as bounded evidence:
  canonical `S { ... }`, no `S[ ... ]` sugar, indexed state, elaboration, generated communication,
  role admission, source patch hot-plug, source operational rows, static devtools diagnostics, and final audit.
- Active current-L2 evidence remains rooted in `samples/clean-near-end/`, `samples/current-l2/`, and `samples/lean/`.

Non-claims / still-later gates:

- No final public grammar, final parser/checker/runtime/verifier API, final SDK, final ABI, or final package artifact authority.
- No production WAN/federation, distributed durable save/load R3/R4, or multi-server consensus completion.
- No arbitrary native/WASM execution, direct Mir-to-machine-code, final engine/provider ABI, or production backend.
- No final public viewer / telemetry ABI, durable retained-artifact service, or production auth/identity provider.
- No exhaustive shared-space catalog or final public product completion.
- Surface Mir evidence does not yet claim runtime MessageEnvelope dispatch / local queue delivery for generated communication.

Management method:

- Every non-trivial task writes a new numbered report under `docs/reports/`.
- Report sections and update-status checks are mandatory.
- Snapshot docs are updated only when current status / roadmap / validation / sample dashboard actually changes;
  otherwise reports must explicitly say update unnecessary.
- `progress.md` is a rough current snapshot, not normative source.
- `tasks.md` is a rewritten current task map, not append-only history.
- `samples_progress.md` is a dashboard, not a work log.
- Heavy build artifacts and disposable generated artifacts should use external workdir policy when relevant.

Reading risk found:

- `docs/research_abstract/README.md` contains wording that appears older than the latest `progress.md`
  for Full System V1 / Surface Mir status. This is a reader-facing freshness risk, not a normative conflict.

## Open questions

- No new open questions were introduced by this orientation task.
- Existing documented user-spec-required gates remain:
  final public grammar / API / ABI / SDK, broader distribution shape, final shared-space catalog breadth,
  WAN/federation, distributed durable save/load, final engine/backend target, and final viewer/telemetry/service surface.
- Existing research-discovery items remain around indexed-state runtime carrier, Surface-to-Core obligation carrier,
  role admission grant carrier, tombstone/compaction semantics, witness metadata, and source patch compatibility/activation carrier.

## Suggested next prompt

「現在 promoted package が無い状態から、次に自走で進める候補を `tasks.md` / `progress.md` / `samples_progress.md` に照らして提案し、必要なら fresh validation を先に回してください。」

## Plan update status

`plan/` 更新不要:

- 今回は orientation report の追加だけで、semantics、examples、fixtures、helper stack、roadmap、
  open questions、workstream sequencing、current status を変更していない。

## Documentation.md update status

`Documentation.md` 更新不要:

- Current reader entry の内容は今回変更していない。

## progress.md update status

`progress.md` 更新不要:

- Current status / roadmap / validation loop の到達見込みは変更していない。
- この task は status-changing closeout ではなく、read-through と report addition である。

## tasks.md update status

`tasks.md` 更新不要:

- Current task map、blocker、next promoted line は変更していない。

## samples_progress.md update status

`samples_progress.md` 更新不要:

- Runnable sample status、validation command、debug surface、blocker は変更していない。

## Reviewer findings and follow-up

- No sub-agent reviewer was invoked; this was a read-through / documentation report task.
- Local finding:
  `docs/research_abstract/README.md` has reader-facing stale-risk wording around Full System V1 / Surface Mir.
  A future docs freshness package should reconcile it against `progress.md`, `tasks.md`, `samples_progress.md`,
  `plan/58`, and `plan/68` if reader-facing docs are updated.

## Skipped validations and reasons

- Full Rust / product / operational / Surface / Full System V1 suites were not run.
  Reason: task scope was repository orientation, not behavior change or evidence refresh.
- Historical `docs/reports/` were not all line-by-line re-read.
  Reason: latest snapshot / roadmap / report chain is enough for orientation; historical reports remain evidence source
  when a specific package is reopened.
- Full code audit was not performed.
  Reason: user requested repository read-through and current status understanding, not implementation review.

## Commit / push status

- Not committed.
- Not pushed.
- Reason: user requested read-through/orientation only; this report records the work, but no implementation package was closed.

## Sub-agent session close status

- No sub-agent sessions were opened.
