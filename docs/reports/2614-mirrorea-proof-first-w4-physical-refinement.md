# Report 2614 — Mirrorea proof-first W4 physical refinement

- Started: 2026-09-14T08:39:26.358953+09:00
- Author: sole main Codex; no subagents
- State: active research; W4 not complete, no Canon or alpha acceptance.

## Objective

Complete owner-requested W4 physical refinement: concrete/abstract correspondence, existing Rust/QUIC/queue/memory boundaries, actual source-to-runtime observation, real-process positives/falsifiers, prior I3-3 regression and alternate-entry closure. Stop after W4. W5/W6/W7 and old Plan250/I3-4 are not automatically resumed.

## Scope and assumptions

One active semantic goal: W4, PL1/PL2/PL0, S4/S6, theory/refinement before bounded implementation. Sole main; no subagents. Task-specific owner authorization permits this reversible LAB research and evidence-dependent limited internal changes. It does not adopt unpresented auth policy, 119 detailed proposals, Canon THM/OBL/lifecycle, public wire/API, production or signed acceptance.

First dependency: publication of a complete configuration/authority payload at a distinct protocol revision to a finite admitted group, preserving current-use checks across receiver queues and late results. REQ DS-01/02/03/04/08, AU-01/04/05/08, VF-04/05; PT-03/11/14; SC-04/07; U ordinary source, correct distribution, distinct authority and no stale resurrection. Exact U IDs will be retained from the requirement registry, not invented. Q18 and W3 H/H2/C/C2 policies remain distinct and conditional.

Candidate A: freeze all protected uses of the explicitly participating finite group, collect exact acknowledgements, publish the checked successor, then install/reopen under matching monotone fences. Compare B: owner-only update/ack while caller may keep a stale head. No whole-world coordinator, ordinary-read snapshot or multi-owner transaction is implied. Missing acknowledgement may remain closed, not success. Full payload/realm/auth provenance, locks/queues, bounded resources and actual Session refinement remain unestablished.

Acceptance requires actual definitions/checker correspondence, general kernel proofs, non-vacuous controls, targeted Oracle review and real-layer refinement. A small isolated theorem is not W4 completion.

## Start state / dirty state

HEAD5d7c13a821c912d14c347775e23493a77f790768, clean; origin git@github.com:yukatayu/mir_poc_01.git. W3 source/evidence81f82a0b and final docs5d7c13a8 preserved. No reset/clean/force, user edits, handoff-original changes, host-share or Chrome configuration change.

Resource audit: root188GiB/47GiB free; RAM15GiB/9.5GiB available; swap15GiB/1.5GiB used. lsblk/findmnt confirm no /mnt/mirrorea-work mount (path absent). Existing target8.3GiB and .git127MiB; .cargo/.lake absent. Only small bounded external /tmp work used; no new large root cache. Heavy runtime build still needs a measured existing-cache/external artifact strategy.

## Documents consulted

Current AGENTS/user override, immutable handoff start/context/protocols/workstreams, verifier source; Canon README/MAP/North Star/Constitution/source hierarchy/phase/ADR0043/architectures09/10/plan05 and Plan250 I3-4/5/6 contracts. Identical-hash full reads of supplied MASTER/119requirements and F0.3/F0.2/F0.1 dependencies are inherited from the previous read ledger; current ledger proof/code entries were checked before reuse. Partial current excerpts are not new full reads. W3 RESUME/CURRENT_GOAL, report2613 relevant current evidence, reference companion/Authority/Session, current tasks/progress and progress axes. Mandatory global corpus remains incomplete, next legacy example301; no new whole-repo roadmap adopted.

Oracle manuals and actual help/debug-help read. Skills: using-superpowers, discord-report, brainstorming, writing-plans, TDD and systematic-debugging. User single-main/autonomous/one-report instructions supersede skill defaults for delegation, extra approval, extra plan directories or sessions. No tool self-knowledge assumption substitutes for actual help.

## Actions taken

Created W4-only active goal; Discord beginfe6c4c recorded silently. Verified handoff byte/identity integrity2ec861. Created external workdir/tmp/mirrorea-w4-20260914-a3e0bpks. Prepared neutral frozen Oracle entry packet and sent once1dd16e (exec76019); actual sessionmir-w4-head-publicatio (requested slug was truncated by wrapper). Final exact-prompt-bound answer recovered73e017 at2026-09-13T23:46:35Z; wrapper terminalexit0 collected28ea87. No resend. No arbitrary job timeout/retry, checks>=180seconds.

W3 fresh-copy source/checker/proof baseline ran998f38 and exited0 at2026-09-13T23:33:56.260063Z. Began external nonproduction Publication generation-fence theory; initial reserved-word/parser and proof-script failures are retained, never counted as success. Corrected explicit proof passes8190ec. No Rust production edit.

## Files changed

Forward checkpoint:45 reviewed Lean/support sources in samples/lean/foundations, existing source-check runner --with-owner-boundary, publication explanation, samples/script READMEs and current status/evidence mirrors. No new sample root. Exact45file list/hashes in W4_CHECK external mirror manifest.

This milestone report; docs/proof-first/CURRENT_GOAL.md, RESUME.md, READ_LEDGER.json and W4_CHECK.json; plan/proof-first-foundation-correspondence.md, progress.md, tasks.md, Documentation.md and samples_progress.md.

- `docs/project-status.md`
- `samples/lean/foundations/MirroreaProofFirstPublication*.lean` and companion `.md`
- `samples/lean/foundations/MirroreaProofFirstReceivedResult.lean` and `MirroreaProofFirstReceivedResultControls.lean`
- `scripts/proof_first_reference_source_check.py`
- `scripts/tests/proof_first_publication_mutants.py`
- `scripts/README.md`, `samples/README.md` and `samples/lean/README.md`

Eleven proof/control modules mirrored69ccb5 with exact import renames; two standalone root mains removed to avoid the demonstrated import collision e98e6a. Namespaced mains remain and generated standalone drivers invoke them. External MIRROR_MANIFEST.json records byte transformations/hashes. No Rust production edit. The outcome/progress additions were reviewed by Oracle3; the newer closed execution response was reviewed by Oracle4; external placement/owner-input candidates were reviewed by Oracle5; qualified-source/Session successor is under Oracle6 review.

## Commands run

Full docs validation248bca passed after fixing the stale progress header and recovering a lost result. The retained receipt records that rerun; later status edits still require final validation.

Read-only git/resource/hash/Canon/code inventory commands as recorded above. python3 sub-agent-pro/mirrorea-proof-first-handoff-v1/tools/verify_bundle.py. python3 scripts/proof_first_reference_source_check.py --work-root /tmp/mirrorea-w4-20260914-a3e0bpks. lean --trust=0 MirroreaProofFirstPublication.lean in external workdir, 4GiB child address-space limit. Oracle browser-only dry-run659ff0 and structured once-only COMMAND.json invocation1dd16e; no paid/API fallback.

## Evidence / outputs / test results

Bundle:669manifest/614original expanded files,119requirements/30decisions/24scenarios, passed byte integrity only. New W3 baseline mir-w3-reference-ztqpxt8e: passed72foundation+4consumer modules,9055owned declarations,32source cases,13integrity controls,5proof mutants; same manifestbd7af23035a560f1f41f8ec30ef7c5af3589bb8102b2c8e708670613ac40b189. No stale historical result renamed as a rerun.

External Publication current source a03f12a4cb9d6a4935d6c8bc86d0e19e27e738766dffdf7c87d1d6492257aef6 passes PUBLICATION_CORE_FIXED.log (8c44e2): independent Allowed/check exactness, execute correspondence, all-action/finite-trace invariant, enabled generation=current published, stale installation rejection and owner-only publication gap. Current #print axioms output uses only propext/Quot.sound; no source holes or Mir axioms. All-owned audit24c280 covers136core+3control declarations. Thirteen fixed controls include two successful rounds, reordered/duplicate packets and stale activation; they are not general proofs. Five exact-anchor mutants fail their designated general theorem62f7ea; concrete countertraces were subsequently added at the material checkpoint below. Earlier failed compiles are retained as failures, not evidence of success.

Actual I3 process/QUIC regression: cargo test --locked --offline -j 1 -p mirrorea-i3-probe --test i3_process_localnet -- --test-threads=1. Current run exit0, 46passed/0failed/0ignored at2026-09-13T23:43:38Z (full log8f68c3, terminal2d8190). Reused existing target with4GiB child address-space cap and one worker; four local crates rebuilt38seconds, tests71seconds. This one integration target is not all workspace regressions or W4 implementation evidence.

W4 material proof checkpoint 2026-09-14 09:24 JST: external PublicationPayload connects full
published/cached values to numeric revisions and actual staged evaluator results;
PublicationSession proves exact coverage of the six existing W3 entries and rooted
Session preservation; PublicationUse adds held local intervals, proves publication
has no active interval and held payload remains current, and projects every trace
to the prior component. New compile failures (PAYLOAD_FIRST, SESSION_FIRST,
USE_FIRST/PROJECTION) are retained; corrected compilations pass without source
holes. Full all-owned audit5d8e3f:136+152+72+119+3+51+15+21=569declarations.
Only propext/Quot.sound plus inherited W3 Classical.choice occur.

Concrete weakening traces562aa7 expose stale-enabled states for forged ACK admission,
omitted participant, absent fence update and wrong published revision. Dropping
only installation's floor permits stale install but the use fence still rejects;
no stale-use claim is made for that control. Held-use controls pass ec1cd9. Actual
Rust-parser-generated17statement consumer is reused unchanged (source hash
8bc404a76afe79d257fafa3ba8e3633aa065ea70292804f27d71801c79b938ad); local model
35round/3participant run matches source values/writes/origins/events/pending/
partitions. New-head and retained old-pending controls pass4d956b. Model only,
not physical W4 execution, permanent replication design or implementation gate closure.

Fresh mirrored integration: command with --with-publication completed exit0 ef7676;
work mir-w3-reference-6ef1eymu, manifest48f1fcc03569f7fc38376a0cbbda478b003784cd0d9ae81fe4a2c6e5ebf02744.
All87modules/9887owned declarations (832publication),32source/13integrity controls,
5reference plus5publication proof mutants pass. Actual source/outcome logs13ae03
and audit counts e7b270. This rebuild validates the new runner/import integration,
not a baseline repeated because of compaction. Defaults retain the W3 scope.
Outcome return preservation c36355 and general nonempty-cohort payload progress
b633ec pass, with only standard logical axioms; historical image immutability
and current prepared/certificate availability are derived. Finite outcome
controls635946 distinguish consecutive IDs and rejected/authorized cancellations.

Explicit delivered-result/source continuation proof is under development externally;
reference waiting tick must not be exposed as a physical result-producing fallback.
Its first compile failures (namespace/binder/layout) are retained; current scheduler
proof489337 passes; first control failed on a wrong field name and corrected
controlf59162 passes. All-owned audit1d2d6f checks21+7declarations. ReceivedResult
remains external and unintegrated; no physical-origin claim is made.

New closed-execution checkpoint: PublicationExecution general proofs pass caaaab
(PUBLICATION_EXECUTION_REACHED.log). The same command language covers ordinary
source and publication; waiting source writes require explicit ReceivedResult.arrive.
Returned values/commands/ordinals survive publication; rooted source and unique
ordinals from empty history are derived; every independent Entry has a normal
finite publication under stated reachability/free/settled/nonempty premises.
Final controls and all-owned audit ff94ae pass:21ReceivedResult+184Execution+17control
declarations, only the allowed standard logical axioms. Full17statement source,
17writes/21replies, wrong id/value then valid input, pending head invalidation,
duplicate rejection and old waiting-publication bypass are executed locally.
Failure logs FIRST/OUTCOME/OUTCOME_SECOND are preserved as failures (no source holes).
Five new source/control modules mirrored26501f; EXECUTION_MIRROR_MANIFEST records
exact import renames/root-main removal. New fresh integration exec85093 exited0 (334773), with92modules/10119owned,
source32/integrity13/weakening5+8, manifest719a4051d6aec6b8fabdac658c0ba1c5786ae747a68f5ae1153a1fa8c4e4fc18.
The new source/result/projection driver logs and all8mutants were read dcf5b0. No Rust edit or W4 closure.

External owner-projection dependency bf4ed6/f3985e/56e0f4 passes: sufficient
configuration+authority input excludes caller values/continuation/history;
actual submitted WitnessMeaning (existing ReferenceAccess) is retained, unlike
existential authorization alone. General checker/serve exactness, admissible
positive execution and wrong-locus rejection, source-write projection frame;
36+9owned audit. No wire/disclosure permission or actual owner claim. First
compile failed on implicit name ticket resolving to the existing constructor;
retained FIRST.log is failure, no source proof holes.

Actual existing Rust checker capability probe9a578b/bffbea: cargo run --locked
--offline -j1 -p mir-semantics --example full_system_v1_check -- --format json
samples/clean-near-end/mirrorea-proof-first-composition/reference.mir exits2 after
successful6.70s build.50unknown-type,2unbound function-value and4undeclared alias-call
diagnostics. This is the current implementation gap, not a failed Lean proof or
accepted I3 regression. Raw JSON/command receipt retained externally. No successful
Rust/Core acceptance inferred from the separate W3 parser+Lean adapter.

2026-09-15 03:27 JST — W4のowner/codec/resource一般証明を既存sampleへ統合し、fresh133module・137module監査/14148所有宣言、通常source32・整合性13・弱化5+8を検査。Oracle20回収、source開始と所有者の全使用区間・実Rust/QUIC対応は継続中。

レビュー済みowner/codec/resourceの45ファイルを既存Lean sampleへ移し、`python3 scripts/proof_first_reference_source_check.py --work-root /tmp --with-owner-boundary` でfresh再構築しました。133moduleの構築、137module/14148所有宣言の公理監査、通常source32・整合性13・弱化5+8が通っています。公開componentの過去92module結果は履歴であり、現在の`--with-publication`は追加されたPublication依存も発見します。native entrypointと新しいsource/owner管理候補は外部workdirのままです。第20回Oracleは回収済みで、予約後の割込み・開始中の取消し・Python入力/再初期化・必須監査対応の指摘を検証しています。実processの通常C/A継続と全bytes照合は成功していますが、全経路の排他管理・物理namespace・既存Rust/Core/QUIC接続とW4全体の受理は未完了です。

Exact fresh result: mir-w3-reference-rkm583l3/RESULT.json SHA6718ddb13cc7b87b9d045410b652e48674bcb0dd261ae209eeb540651793e0c3; all149commands exit0, Lean4.29.1 trust0, only standard axioms. No new Rust/Canon or native executable entrypoint. SYS4 dispatch full17803lines now read; corpus still incomplete, next322. Resource c3592f/f65c4b:42GiBdiskfree/10GiBRAMavailable, serial4GiB limits retained.

## What changed in understanding

Existing I3-3 B-local successor plus parent acknowledgement/publication is not automatically a realization of W3 strict realm-wide currentness. This is a new consumer obligation, not a reproduced defect against accepted I3-3. Monotone endpoint fences and retained acknowledgement origin permit an explicit small arithmetic invariant without putting the desired result into State or transition premises. Matching a generation does not yet match a head payload or authenticate its issuer.

Main source-derived counterexample daafb8 (external PublicationProjectionControls):
from actual source after let-mut count=2, prepare controlInput leave(B), execute
ordinary count=count+1, then install the saved entire prepared Session. count=3
and its actual write disappear. This deliberately unsafe comparator is outside
admitted Publication histories, not an I3 regression. A split metadata/local
implementation therefore needs a proved projection/merge or protected preparation;
blind whole-Session replacement is not a valid implementation of local source progress.
Candidates remain technical research: serialize the proposing source actor during
preparation, or evaluate its control against the current actor state at the
publication/commit point. Neither authorizes a policy or production change.

Actual QUIC source review confirms original-reply inner currently holds a mutable
runtime borrow across its frame read; pure runtime admission follows afterward.
Existing retained-ingress APIs already separate receive/admission for one fixed
session-one→successor-session custody cut. They must not be repurposed as a general
W4 ingress without preserving its control/session/permit constraints. W4's pending
remote work cannot be confused with an active local critical section. This is a
new-consumer mapping obligation, not a claim that accepted I3's bounded contract fails.

Oracle3 identified an actual composition boundary: ReceivedResult.advanceReady
alone does not restrict the old PublicationSession/Outcome evaluator's waiting tick.
The original local model is valid, but its publication route cannot establish actual
owner provenance. The new PublicationExecution relation excludes that route and
retains positive source behavior. Physical code must still implement only these
admitted transitions, preserve existing I3 gates, and record actual owner ancestry.
Correct integer results and pure recomputation cannot establish that ancestry.

Current code inventory identifies separate implementation surfaces: I3 build_project
uses check_and_elaborate_surface_v0/project_checked_core; the actual W3 adapter uses
textual_alpha. full_system_v1 TypedExprKind.Call resolves direct declared functions,
not instance-valued callable expressions. Its older provider receipt preview and
simple computational_core sample module catalog cannot provide W4 owner evidence.
Both inspected Rust arithmetic evaluators use plain i64 operations; W3 Machine
ALREADY uses checked Int64 input/intermediate semantics (InstancePrograms1–17).
Any consumed backend needs correspondence for that profile, not adoption of the
older evaluator merely because both types say Int64. No overflow execution or
accepted-I3 arithmetic defect is claimed from this read-only inventory.

## Open questions

Entry Oracle recovered and disposition recorded below. Complete-head binding, group/realm identity and nonforking publication; source Session correspondence across distributed state; all runtime entries including bootstrap/alternate executor; queue/memory failure atomicity and actual negative network coverage. W5 restart and W6 confidentiality remain later. Need current snapshots synchronization after entry review and dependency reading; provisional remaining work estimate24–60hours, task-work estimate5–10% at this early checkpoint; neither is an acceptance metric.

## Suggested next prompt

Already authorized: continue W4 through final candidate evidence/integration. No new prompt or approval needed for independent research.

## Plan update status

plan/ updated: current W4 authority/scope plus forward proof checkpoint in the same correspondence file; historical records preserved, no new roadmap or Plan250 activation.

## Documentation.md update status

Documentation.md updated: W4 active/current evidence and distinct W3 closure; no runtime capability promotion.

## docs/project-status.md update status

更新済み: W4 current position now records the active proof/model gate while preserving accepted Canon/Plan250 state and W3 history. No new runtime capability is claimed.

## progress.md update status

progress.md updated: W4 three-axis readiness/current gate and dated log; prior closed logs retained.

## tasks.md update status

tasks.md rewritten as current W4 snapshot with dependency stages, provisional estimates, and separate research/owner gates.

## samples_progress.md update status

samples_progress.md updated: W4 active external model evidence and existing baseline distinguished from runnable network workflow. No new sample status or taxonomy promotion.

## Reviewer findings and follow-up

Fourth narrow review mir-w4-execution submitted onceb9e932 (exec53193), after
successful dry-rune4c7fd (36frozen files, browser GPT6Astra). Question
 a25e6df76320122be5d8d3b5ae27d3ee1662961a5e5d6999645f1b8ac32810c6;
manifest6a49a68a8d860525bedc595247e945de0c26ccd07440d02db30b741746c15272.
Started2026-09-14T01:47:54.795732Z. First exactcaptureaaa92a at01:52:44.969Z
matches full question, one user, no assistant, stop true. Keep same job and>=180s
interval. Reviews closed-entry response and source/Core/placement A/B question;
new external owner-projection proof was after freeze and is not in this packet.


Third-review findings independently checked: waiting publication bypass reproduced;
source/metadata lost-write comparator independently reproduced; mutable refusal must
preserve private saved continuation and cannot infer detailed reason from none.
New PublicationExecution responds with general closed-entry/publication and outcome
proofs plus discriminating controls. Physical provenance, projection, guard ownership,
non-reusing finite representation and committed normalization prefix remain open.
This new response has not yet received its own final narrow review.


Main self-check is not independent review. Read-only Oracle sessionmir-w4-head-publicatio completed. Question4456cf700ba8ceab173cc0e4d8887edf7f918ace097c4db2d215689bd449024b; manifest3cb803f865e1b2e361fb9a5f51b3c1c9cf7f925089a71c2e5f43062704085e02. Exact prompt verified before capture; final assistant count1, stop control absent, wrapperexit0. Newline-added answerfile SHA d8942a595e9428718221eaeee23a59275ab7d07793d2e42bfd92256cd7cd668f. External RECEIPT.json and DISPOSITION.md retain the collection and main assessment; no signed reviewer fabricated.

Main adopts the criticism that composition currentness is independent of authority generation: actual InvocationBoundary.Controls with unchanged view rejects a ticket after replacement; Session control and tick preserve the authority view. Full state/configuration binding, all semantic linearization points, maintained freeze provenance, strict-successor rather than +1 authority generations, pending/history retention and concrete mutant bad traces are proof obligations. The existing generation component uses numeric protocol rounds only; it does not discharge them. Candidate A remains research; B's stale-caller trace is an abstract falsifier, not an observed I3 defect. Owner policy, same-instance recovery, public contracts and lifecycle acceptance stay open. The Oracle lacked exact requirement texts, so its review does not certify the ID crosswalk.

Material proof-cut review mir-w4-payload-cut submitted once220858, exec83456;
questionf79f482156b1ffee25a7ac703fe1f6850a546ddf137ffd92e2b62e42da2881e9,
manifest657015b0b7a2d44b67a0e1a18992e481867020429900be08fe217b34a61b671d.
Dry-run0f7ed4 verified browser GPT6Astra and frozen21attachments. Exact prompt-bound
final read1e996c at2026-09-14T00:30:54.328Z matches the exact question, one user/one assistant and no stop control. Wrapper exit0 collected6ed467. No retry/deadline/browser configuration change. Full answer read984298/12a747/4cb24e; receipt/disposition recorded942618. Oracle found no reachable counterexample to numeric/payload/interval currentness, but state-only control-return erasure, possible guard/publication circular wait, missing general round progress and physical owner execution remained. Main verified the return erasure and interval behavior against actual definitions; no accepted I3 defect inferred. Authority timing/admission and Q18/H/H2/C/C2 remain conditional. The third review below subsequently examined the outcome/progress additions.

Docs validation initially failed on missing canon-wins wording in tasks.md (ef3fbf), corrected942618; next run found the project-status declaration format (c2102a), corrected in this checkpoint. Neither failed run counts as success. Discord progress5bf807 sent once at the earlier natural checkpoint; task continues, no complete notification.

New narrow review mir-w4-result-cut submitted once c26627 (exec31914),
packet oracle-w4-result; question5ecf2882a3c0086debc7739b962f97305500648cd5180cad51b60c51f80bb9a6,
manifest14aee761286ac3cf2490dabc45f39dbac9a8a9351d920bdf25f739cfd95f9ee0.
Dry-run c79ce8: browser GPT6Astra,38files; explicit old-source excerpts and lossless
ActualReference interning bound the review packet. Final recovered1fe354 at2026-09-14T01:27:53.577Z; wrapperexit0d7de99, full answer8e3673/9a3ed6/6edf15. Exact prompt, one user/assistant and no stop control. Answerfile SHA f331a7573dae74783fde5f23dda04d6b1a8b86da040cd04ec609f46e3922d7a3; receipt/dispositionc543dc. No retry or browser change.
Later docs failures: required tasks non-promoted sectionf85b05 and missing canonical
source pathf1ed7e, both corrected. Focused checks116473/ebe871 now pass headings,
source references and timestamps; report helper invocation had a wrong constant
name, so that focused script was not a complete pass.

Oracle20 c8710b/91b2a7 is fully recovered; wrapper72824e exit0. Local lemmas have no found source-level falsifier. Lease does not span reserve–compute; in-flight enter needs serial source ownership; None/reinit and canonical-but-untyped reply are Python boundary issues. Required-audit null binding accepted in actual copied result c5f60f; repaired mode/nonnull/path checks reject three mutants1705de after new full397input C/A replaysedfbe5. SourceCreditEntry general proof876676 and new native coordinator positive3751c0/bce4ff are external after the frozen review; physical all-entry/funding obligations remain. Oracle is advice and not a signed reviewer.

## Skipped validations and reasons

Actual W4 Rust/network/continuation integration, remaining I3-3 regression targets and final docs checks not yet run; the external native owner component now has16process controls: implementation gate is still under investigation. No W4 completion claim. Broad global corpus reading remains incomplete; indexes/grep and truncated output are not full reading.

## Commit / push status

Reviewed publication checkpoint ab316353 committed/pushed with exact remote parity (8b9562/ff82f2/d0fb35); later evidence/status changes are own dirty work. Baseline clean5d7c13a8 preserved. Future own commits use --no-gpg-sign and normal push; no force/reset/clean.

## Sub-agent session close status

No subagents started or used. Sole main owns code/proof/commands/Oracle/Git. Oracle is advisory, not a delegated writer or owner key.

### Fourth Oracle recovered: closed execution and source/Core bridge

Exact capture ea24ef / wrapper c4feb6, session mir-w4-execution; full answer 7854a6/23a6fb/d3e20b. No internal counterexample found within stated hypotheses; joined actual-owner execution/caller consumption remains open. A/B bridge not selected; no implicit caller-to-operation routing, retroactive provenance, terminal failure, history rollback, privacy permission or policy approval inferred. External OwnerProjection and actual Rust checker probe postdate the packet and were not reviewed. RECEIPT/DISPOSITION retained under workroot/oracle-w4-execution.

### Source placement discriminator and external candidate

Actual Rust parser -> existing W3 checked source: C-only instance called at A rejects; A/C placement succeeds at A (679830). New external SourcePlacement has independent compile/elaboration exactness, old local compatibility, general source provenance/rootedness and exact captured target/site/dependencies (97d296); all-owned audit108+10 standard logical dependencies only (32269e). First compile errors remain failures. Explicit RAW call target C produces saved argument3/result10 with unchanged caller input; no parser/network claim. Crucially call-only placement is insufficient: A-bound reference then C call fails normalization ownerDenied, and unchanged A cancellation cannot cancel C ticket. Coherent original C reference/call and independently C-authorized cancellation pass in a manual local comparator (7d4702). This is a falsifier/adoption gate, not permission to retarget saved bindings or weaken H/C policies. Fifth frozen Oracle packet oracle-w4-placement reviews this delta and OwnerProjection; no production edit.

Docs validation248bca passed with retained log/receipt after recovery of the lost prior result; later entries require final revalidation. Read ledger advanced through historical example299; mandatory global corpus remains incomplete, next300.

### Receiver input without exporting private saved binding

External ReceivedTicket (after fifth frozen Oracle packet, unreviewed): incoming ticket/value is compared with the caller-owned saved entry, then original protected completion uses that private entry. General exactness/relative completeness, original binding protection, duplicate refusal and invalid-private-binding refusal pass2955af; all-owned audit8+10passes936c6d. Finite controls65912b include malformed/wrong-value then valid input and a lower-machine release comparator where unary owner eligibility remains true but private reference protection rejects. That comparator is not a newly admitted SourceSession release path. No actual owner ancestry, wire/privacy permission or full Session integration is inferred.

### Fifth Oracle recovery — 2026-09-14T02:38:30Z

Exact bound final 0a47a5, wrapper336273 exit0, full a602d9/bd476e; one submission, no Chrome mutation or retry. Receipt/DISPOSITION retained in oracle-w4-placement. Caller-field substitution, lifecycle qualification, normalization allocation prefix and coincident output/code counterexamples accepted as refinement obligations; no false named theorem identified. Existing source/Core candidate is insufficient and remains unadopted. Next external candidate carries explicit placement with reference identity and preserves legacy local intent, with independent caller custody/admission and unchanged operation authority. ReceivedTicket was outside this frozen cut. No physical W4 or policy acceptance follows.

### Qualified source/Session checkpoint — 2026-09-14T03:13:34.839295+00:00

External QualifiedSource and QualifiedSession compile44c851. Independent annotation/selection/typing relations match actual checkers; general alias theorem preserves the entire old machine; waiting capture retains selected locus/site/read inputs. All seven admitted Session entries preserve source partition/archive, original source reachability, fixed logical Actor, allocation/pending/provenance. A waiting tick has no result branch. Same-caller source adoption preserves private state/archive; it is not authenticated custody.

Actual parser experiment4811ef: provisional typed referenceAt appends one explicit Place argument. The real17-line Mir source at A generates qualified C reference operations,4explicit local-model receipts10/10/11/10 and exact source completion. Main-run all-owned audit16af03: ReceivedTicket8/QualifiedSource210/QualifiedSession167/OwnerProjection36/ParsedQualified11, only standard axioms. Actual-source controlsf6020a reject unknown/nonliteral place, wrong type, missing argument and wrong provider; renamed alias/UTF8 source passes. No Rust implementation or physical E2E is claimed. The adapter is an external experimental desugaring, not an adopted public library/provider contract.

Admitted Session control576753 preserves pending without C cancellation authority; C cancellation succeeds with invocation/access/holding revoked. It also reproduces a stronger-claim counterexample: after caller A leaves, the fixed Actor still initiates qualified C work. Current activation custody/admission remains a separate physical boundary, not derived from source typing or fixed fields. Oracle6 packetoracle-w4-qualified submitted once1fe8c6, sessionmir-w4-qualified, exec4671, questionafececcfd0303907e75ce15a6b92a084e70bb75fd5352cd4113f88dfcea98c1b, manifest097701797d51eb7265b97516bd17ef22846624474ee0ce6c0c4a889b1d549f8d; browser dryrun a722ed/16c60e. Normal job retained; first exact capture773231 still generating. No failure/resend/Chrome change.

Independent finite AuthorityImage work after this freeze proves exact claim/submitted-witness/producer preservation with a finite epoch row per issued claim972594. No codec/currentness/disclosure permission is inferred. Initial proof-script failures for new modules and reserved-name controls are retained as failures; no source holes are used. All new qualified/image work remains external, not fresh92-integrated or committed. Current user-facing rough remaining estimate stays24–60active hours;15–25percent is only a workflow estimate, not requirements coverage.

### Sixth Oracle and original declaration repair

Final exact capture9eae72 at2026-09-14T03:26:14.906Z, wrapperef3eba exit0, full59494c/d6cd43. No duplicate submission or Chrome change. Packet frozen before AuthorityImage/Continuing additions; receipt and disposition saved. Source-inspection review identifies an original declaration defect, not a false named Lean theorem: declaration-only instantiateAt mutation with unchanged instantiate calls was admitted. Actual parser reproduction32b79c confirms17accepteditems and2undeclaredcalls. Adapter now resolves all original Perform declarations/arity/provider boundaries before desugaring. Fourteen actual parser casesef4b0b pass, including both declaration orders, qualified-only declarations and independent byte/token position assertions for all17lines after Japanese/Greek text. Original rejection reasons remain explicit; no general parser proof or physical execution inferred.

Continuing additions083e43/107380 (after freeze) prove pending/stopped source-site agreement over all entries. Typed residual and tag/value ancestry remain open. Review preserves the caller-left counterexample and separates new initiation from already-prepared receipt/cancellation. Duplicate fresh activation and direct mathematical receive show that logical identity/value checking cannot replace custody, disjoint namespaces or real owner/receipt occurrences. CurrentOwnerRep remains the immediate physical bridge gate; A/B unselected. AuthorityImage54+5owned audit d18e2e passes but is unreviewed, and does not prove authorized/current images.

Docs validationc37dd6 failed solely on stale samples_progress header; header synchronized with its existing12:15JST timestamp. Failed receipt retained in DOCS_CHECK_QUALIFIED.json; no success claimed. No W4 commit or physical implementation yet.

### Finite owner representation and residual typing — 2026-09-14 12:53 JST

General QualifiedSource environment preservation3fce87 and QualifiedSession all-entry residual typing/fresh result binder ef3f35 pass; a malformed waiting/no-pending comparator satisfies partition metadata but violates continuation/residual typing. Failed first proof scripts are retained as failures. Exact main17line source, existing authority/cancellation controls and new comparator rebuild a3dd97; owned225/191/21 for source/Session/control. No tag-value ancestry or parsed-origin theorem is inferred.

OwnerImage general roundtrip e41837 and held-publication relation4191e5 preserve full structural support, finite represented policy rows and exact submitted witness checks. Source-derived code execution remains the original evaluator. Controls06a34d/3373f0 reject wrong endpoint, missing epoch, revocation, disabled support and evidence version; same-generation retirement distinguishes stale and current images. All-owned audit3373f0 finds only standard axioms; OwnerImage101/control6. The finite data includes potentially sensitive metadata, so disclosure remains independently authorized; no physical currentness/codec guarantee is claimed.

Seventh packet oracle-w4-owner-image frozen3b440d, dryrun4afa4e (browser GPT6Astra40files~144923tokens), submitted once4e5d49 at2026-09-14T03:50:54.645145Z, exec21081/sessionmir-w4-owner-image. Questionba5617df6b7db387f9f1cdc2f402c94a09a6db259afcb4201a758151e6e93c0c; manifestae7089694100627b1fc76a9c24dfcea6725df7f04b56c93a8cd0c692b312e383. One early capture invocationbef031 was refused by the>=180second guard before any browser read; it is not Oracle failure and does not cause resubmission. No Chrome change.

Main additionally retains a profile distinction: request namespace uniqueness per realm/principal alone cannot synchronize independent private composition states in a shared realm. The proposed physical cut needs one coherent authoritative composition domain and an explicit private-activation partition; multiple independent domains are not implicitly merged. No new policy or production contract selected. Mandatory full-reading ledger advances through historical example301 (413b96), next302.

### Reviewed publication checkpoint integration preparation

All16mirrored formal/control modules exactly match completed Oracle review packets2–4 (e6f9f2); runner, mutants and all16files exactly match the passing fresh92manifest (d2768e). Focused runner/mutant review d1dae6 preserves default W3 and rejects missing publication evidence; diffcheck19ff31 and source hierarchycf2940 pass. Docs validation5d2fa3 passes (DOCS_CHECK_OWNER_IMAGE.json/log,2026-09-14T03:58:12Z); earlier stale-header failure remains recorded. No new heavy proof baseline was rerun just for status maintenance.

After seventh packet freeze, external OwnerImageCodec compilationd600e5 and actual-source JSON controls9e0de7 pass:3144-byte image, exact ticket/source result,9shape/profile/range/unknown-field negatives, extra ticket rejection and decoded inactive image still denied. This is reference experimentation; no general inverse theorem, byte/depth/resource bound, authenticated install or production entry is established. Seventh exact captureb154cf at04:01:41.566Z remains healthy/generating; same job21081 retained.

Staged-diff check732059 caught extra trailing blank lines in three new control modules (untracked files were not covered by the earlier unstaged check). Only those EOF blank lines were removed; all three affected controls kernel-compile again and staged whitespace passes97d9fa. Before/after hashes are recorded in checkpoint-whitespace/RESULT.json. The reviewed/fresh92 equality claim refers to the pre-trim source cut; this explicit whitespace-only successor preserves the same proof/control statements. First W4 checkpoint commit is prepared with33own files; task and seventh review continue, no W4 closure.

### Seventh review and first checkpoint pushed

Oracle7 exact finalb2dfb0 at2026-09-14T04:05:30.841Z, wrapper5c8f71exit0, full499c9c/19c520/8ddc21; RECEIPT/DISPOSITION retained. No false literal new theorem found by inspection. Arbitrary image validity/currentness/disclosure, finite policy future growth, independent endpoint realm/locus, residual freshness versus ancestry, source-adoption admission, private preparation across all7entries and serviceable reserved control queues are explicit remaining gates. Actual queue deadlock can occur even without an outstanding guard. Image installation must frame occurrence history; unrelated growth must not blanket-reject old semantically valid tickets.

Main selects checked Lean evaluator route B solely for reversible nonproduction owner-worker/codec experimentation; final backend/production integration remains unselected. It receives structured finite image/ticket data, never arbitrary Lean source or a caller Session. General codec correspondence, all-entry physical simulation, concrete currentness/custody and actual occurrence join remain required before W4 closure. One coherent realm composition/private activation partition is required; per-principal request uniqueness alone is insufficient.

First W4 component checkpoint ab316353f6ac8bb64c15705b50cf1fcca763be7e committed8b9562 without GPG, normally pushedff82f2. Remote main exact parityd0fb35; working tree was clean at that check. This closes no W4/Canon milestone. Current report/evidence updates after the checkpoint form new own dirty work; active W4 continues.

### Native owner evaluator experiment — 2026-09-14 13:29 JST

Oracle7 invalid-image concern reproduced d81fdc: shape decoder plus unary use accepts constant1 with output[0,0]. This falsifies unchecked-install guarantees, not the conditional totality theorem. OwnerValidity general checkState_exact/check_exact, capture_accepted, protected_source_capture and checked_admitted_completes pass6406f8/3bc52d. Five well-shaped invalid configurations are rejected1bc312; valid inactive input remains unauthorized. OwnerEvaluator proves result_exact/admitted_completes/rejection_exact/no_execution_failure3fe1a2, only standard logic axioms in printed dependencies. No source holes; failed compiler/script attempts are retained in external logs (wrong constructor, decidability instance, layout and tactic names), not successes.

Fixed OwnerWorker accepts data only via bounded one-request framing and independent launcher realm/locus; no source text, caller Session, install, authority issuance or history endpoint exists. Native buildaabef8 from54frozen modules succeeds; binary SHA2567c3ad154390a4dd99963bc1ed6d0df5d677b7e0a3c99feccdc7ff4704f8fd11c, build/log/command receipt OWNER_NATIVE_BUILD.json. Native process harness60a379 passes16cases, exact process IDs/input hashes/results in OWNER_WORKER_PROCESS_CHECK.json. Actual parser-derived source prefix exports input/ticket18573a, and the independent worker computes10. Invalid current realm/locus/cohort, claims, parent support, arithmetic profile, evidence argument, extra fields, lying contracts and intermediate overflow are rejected; unrelated valid growth retains an old ticket. Oversize header/depth/UTF8/truncation are separate boundary cases. Four-GiB memory and10CPU-second harness limits are explicit; no general resource liveness is inferred.

The JSON codec remains unproved generally. Local Lean4.29.1 source inspection44a5a2/0ae22b shows recursive deriving and Json BEq use partial implementations; this is a real proof boundary, not a reason to equate16tests with a codec theorem. A total structural codec is the next bounded comparison. No new native/codec Oracle cut is submitted yet. Existing review7 does not cover this delta. Mandatory full reading advances through legacy302500682, next303; global corpus remains incomplete. Status snapshots and the existing plan are synchronized; no Canon/production/sample-root change. Discord progress90c095 sent; continue W4, no completion notification.

Native dependency owned-declaration audit e1dd67 covers54modules/8391declarations with only propext/Classical.choice/Quot.sound allowed. This audits assumptions, not correctness of partial JSON, code generation, runtime or IO. External total structural codec candidate now has general roundtrip/canonical proofs for primitive/container/isomorphic records eeb270 and recursive arithmetic terms, contracts and definitions26e4f7. Shape/error bytes and complete OwnerImage are not yet connected. No new Oracle review or production gate closure is claimed.

Full structural OwnerImage/Ticket codec and evaluator correspondence passa46346; source request/result10 and9malformed/cohort/index/vector/recursive-arity controls5b0305. Six-module owned audit47f341 covers499declarations with only propext/Quot.sound. New actual-UInt8 natural-number prefix/suffix roundtrip, canonical-prefix and strict-consumption laws pass1dcd7e. Full Tree/byte/native correspondence is still in progress; these sources remain external/unreviewed after Oracle7.

### Certified bytes and native successor — 2026-09-14T05:20:08.224506+00:00

Actual UInt8 Nat/list/text/tree general prefix, canonical, cost and complete decoder proofs pass194219/9e513e; full request/result packet laws62782f/9e513e connect actual byte decoding to the original evaluator. Unicode scalar validation and suffix boundaries are explicit. Source packet controls7ee401 pass; native candidate fuel256 admits actual source cost191, request3821bytes. Failure logs (layout, missing type argument and tactic scripts) remain failures; no source holes.

Native successor61frozen modules compilesbe8f41, binary1ba8a5b7e99a65f907d8a5d0d47b7a82cbea36d9720d48bf22bc0c2711f3b473. All-owned audit910aa1 permits only standard axioms; native/runtime/IO correctness is not thereby proved. Black-box22cases8a90d0 pass, including actual source10, wrong independently assigned realm/locus/cohort, revoked/inactive context, altered argument/profile, invalid contract/graphs/placement, malformed and resource-bound input. Actual Rust parser and checked prefix variants564e0c feed the same native worker: square+1 at2=5, linear3*x+1 at2=7, square+1 at1=2. No transport/source receipt consumption/E2E claim. Old JSON experiment is retained separately.

Oracle8 packet owner-bytes frozen2ec1c2, dryrunf31316, submitted once280e3e at05:14:47Z/sessionmir-w4-owner-bytes, exec24303. Questiondb9587391b03aa71ccd4b556bf85b6a34ee2acc2e25fe376e379f502fc1c8ab4; manifestd02ff941ff07ddb15493cc0fdd68ab4894f82babcc781206124ee7dc789f52b3. Exact capturef3d74d05:18:37Z still generating, no retry/Chrome mutation.

Independent post-freeze PublicationImage680626 defines executable image-only replicas and private coordinator state. General check/apply/execute projection, reachable-path lifting and held-image/current-image equality pass; it does not simply assume a current-image invariant. Actual physical transport, authenticated publication, custody and independently authorized disclosure remain obligations. Qualified7entry/private-preparation connection continues. No W4 completion or Canon/production change.

### W4 bounded native and image publication — 2026-09-14 15:27 JST

Oracle8 final4271f0 was fully read3a2980/0486bf and wrapper collectedba98b2. Its inspection found no displayed general-law counterexample; it did identify missing actual payload correspondence, inconsistent CPU launch bounds, post-frame trailing data, output catch and result-only correlation gaps. Receipt and dispositions remain external oracle-w4-owner-bytes. This is advice, not replay/signature/acceptance.

Old worker maximum natural/text inputs abort with child stack overflow64fef4/5032aa; these remain failed evidence. OwnerPayload general digit/text checks and conditional payload theoremf4cec2 precede bounded decode. New one-frame-plus-EOF input and output-failure propagation, shared4GiB/10CPU-second/15wall-second worker limits (never Oracle), pass25process/3actual-source/4IO controls. Initial IO test substring-capitalization failure819636 is preserved separately; corrected harness5ed12d passes.

Support duplicate-disjunction cost is concrete: paired4fdd4f source-sized valid images give old0.467072s versus table0.013414s with equal denial; single-reference0.029492s versus0.016055s. SupportTable/OwnerTableEvaluator general rounds/check/evaluator/payload equalityf36e1a/ef07a3 preserves full least fixed point. It is not worst-case resource/timing confidentiality evidence. Native64module build4475a7 passes; hash22634dc05779462755c64a0affd69e567dc8a31ad52f3bda25436077d75ed6d8. Controls f73422/26760d/4e6c4b pass.

PublicationImage holds only owner image in replica/cache/history/held fields; private coordinator retains source/prepared state. General executable path lift, held-current image and constructive normal publication pass680626/54f4de. QualifiedPublication maps all7entries, retains real command/results, proves checker exactness/rooted preservation/normal publicationb199e1/6026a6; controls a6a974 use17source statements/21private model results and4explicit local receipts. No physical source or occurrence ancestry follows yet.

These successor files are external, unreviewed after Oracle8 and not mirrored. No Rust production/Canon/public contract/key change. Status/plan/samples updated; mandatory reading through historical303f89d3f, next304. W4 remains active at roughly2–3/10 workflow,24–60active hours low-confidence remaining. Goal tool reported blocked at e01fbe/9fad04 boundary despite locally runnable work; this user continuation is being executed without manufacturing completion or recreating the goal.

### Post-review-freeze occurrence/receipt connection

Oracle9 submitted once649d84, actual wrapper sessionmir-w4-bound-publicatio (requested slug truncated), packet42files/363698bytes; question25bcde90077939c93eb8762a8497d9aae307c6ba69986a692d169f44d3b11c20, manifestac1ecefdb5262b1518d1217fab0ffb6ea330bd671ab4429a2b197bf9afbe1fa5. Wrong requested-name metadata lookup a46ad3 failed without browser read; actual Session3c2ef6 corrected in START, no resend. Exact captures187488/d280e1/f38d07 show matched prompt and ongoing generation.

After this freeze, OwnerOccurrence proves current admission before duplicate classification, fresh accepted progress, actual original execution in retained decisions, old record preservation and no second production across arbitrary later submission/installation steps611482. Installs preserve namespace/history; duplicate returns no stored result. This retains the existing I3 distinction read in sys5_i3_process_runtime.rs87c912/04a83b/fa994d; no accepted Rust behavior is changed. Controls1fd040 cover real parsed-source pending, lost reply, reinstall, authorized conflicting binding, equal-valued distinct identity and finite non-eviction. Clearing history on install demonstrably permits second production.

OwnerReceipt adds a concrete scope/ordinal/revision/exact-ticket/result envelope and byte law, with original private pending/protection, no-double acceptance, distinct-ticket refusal and positive issued completionecfd6e. Local joined control9ba386 passes through actual pure owner decision,831bytecodec and source result write. Crucially, a forged matching tuple is accepted despite an empty owner history; this is an explicit counterexample to inferring authenticated owner provenance from typed bytes. Native worker still emits the earlier bare Result. Auditcd830d covers115+46+9owned declarations with only standard axioms. No stateful native/network refinement is inferred.

Failed proof scripts74f295/53ee0f/1655f2 and control8d7724/39ae58/29e1a9 are retained. The conflict control initially asked for4 outside source contract[0,1,2,3], then used separately authorized2; no false admission claim or source holes. Source-generated type validity remains distinct from oracle/tests.

Existing cohort ordinal fresh_occurrence_ref uses AtomicU64.fetch_add (source8768c7). Native boundary litmuse2c4d1/6ab030 shows max->0->1; checked fetch_update stops at max without reuse. This is not an actual2^64-cohort run or a claim against the accepted bounded I3 profile. W4 cannot reuse this function as an unconditional nonreuse premise. W2 bounded identifier proofs already require checking before overflow (ffd4f5); a concrete allocator mapping remains before the physical namespace claim.

### Oracle9 findings and resource/history repair — 2026-09-14T07:06:53.628682+00:00

Oracle9 completed06:43:46.108Z, wrapper013c25exit0 and exact prompt metadata7baa1f/57b47a; full answer3e9f6f/75119e, SHA f66e2259fd9bc780ec9fab0776ec08f8458d9d920363970fc02d9b7b37043a9f. Closed CDP targetbfcadd occurred after successful completion, no resubmission. All9jobs collected. This is inspection with incomplete replay dependencies, not independent machine verification, signature or acceptance.

Main reproduced launcher /proc-inspection failure leaving a child live c4dc2d. Enclosing all post-spawn work in cleanup and nonblocking deadline-aware open-pipe writing repairs inspection0511e8, injected write failure and actual blocked pipefea003. The exact child is killed/reaped; the fixed worker spawns no descendants. Worker25/source3/IO4 regressions538bb5/b4a2e9/0c7171 pass. OS scheduling and whole parser/lowering resource liveness remain unproved.

Actual operational inputs95a822 separate ID2^128-1(value10) from mathematically admitted ID2^128(boundary3), and reject two huge declared dimensions before Vector construction. Source inspection checks actual parsed list length before allocation; no global memory theorem is inferred.

Oracle9 graph-cost concern is concrete: source-derived valid chains all fit the proposed byte/fuel/record profile. Old64native takes2.6417s for16nodes and exits-9 at10.0073s with zero reply for24nodes16486c. This remains failed evidence. GraphTable general reachability/check equality82317d and both separate DAG checks in OwnerTableEvaluator02810b pass; occurrence/receipt dependencies recompile with only standard axioms. New65module native build started1aa7a6; no runtime success claimed yet.

QualifiedPublication now proves empty-root retained ordinal order and uniqueness05fa8c. Updated controls4db553 exercise all7constructors via actual model publication, and show arbitrary initial history produces[1,1]. OwnerOccurrence/Receipt controls rerun after evaluator change4db553; forged typed receipt without owner provenance remains the required counterexample. No physical custody/network integration follows. Docs validator30298d passed before this update; later status synchronization needs checking.

### Reservation consumer and reviewed delta preparation — 2026-09-14T07:22:27.106407+00:00

Graph native65module builddfad23 passes, binary378864a7be809b66123670db8eefe63a6453998b452e67f44be6fd9523569f2a. Auditdab0a4 covers8992owned declarations, standard logic only. Same chain inputs0c7c21 all return10;24nodes0.095s versus old10secondCPU kill. Boundary controls7479ef/4d3e24:64nodes3.341s accepted,65nodes refused for record bound, grounded support cycle accepted/rootless denied. Initial handwritten expected65node reply bytes were wrong; FIRST receipt is retained and fixed expectation is generated by certified codec. Input-generator layout error75bbe1 also retained; fixed738235.25process/3actualsource/4IO regressions7e8ff3/c59163 pass. No worst-case availability claim.

OwnerReceipt.current_production_completesfa707d constructs private source consumption from actual owner production, exact current image, pending membership/protection and machine invariant. The earlier theorem simply transported already accepted receive; it remains under that narrower scope. Initial tactic-constant error7d2c84 is failed evidence, including compiler-generated sorryAx, never source holes or accepted audit.

New OwnerReservation is the direct nonproduction owner-driver consumer: reserve stores a key before compute, excludes competing dispatch/install while active, abandons without erasing reservation, and retains actual result before output. General rooted active-reservation ownership, no-second-reservation and no-second-computation over arbitrary permitted paths passcf05d1. Fresh initial namespace is distinct from same-instance recovery. Controls77edcf pass actual source reserve→compute→receipt→private write, lost reply/reinstall duplicate, abandoned-computation tombstone; same-namespace erasure demonstrably reopens execution.4module post9auditc024b3 covers505owned declarations with only standard axioms. Native custody, crash behavior, authenticated namespace and exact source/QUIC integration remain unproved.

Oracle10 frozen packet44files287326bytes plus manifest; dependency inventory lists88modules but full transitive imports exceed context, so exact excerpts and review limits are explicit. Dryrun c20baf~103206tokens, submitted once9cb99607:20:21.435Z, sessionmir-w4-reservation/exec45514. Question671332688f0aec51a4118e5d080270d42c388f59d3437d1bef3eff3236a16160; manifest53358aa1468199dbef85f379ce135e9b25e8fea04974a8a4a87c8784066c88c1. Review covers post9graph/launcher/history/receipt/reservation delta, not product acceptance. No Chrome settings changes.

Operational exception: Oracle CLI dry-run unexpectedly reported pruning8stored sessions older than168hours through its default retention policy. Main did not request cleanup; this is not hidden or described as no deletion. Live invocation explicitly uses --retain-hours0 to prevent further automatic pruning. W4 packet/answer evidence is retained externally and mirrored by hashes; no task-side cleanup was run.

### Actual stateful owner/source component — 2026-09-14T07:35:35.799531+00:00

After Oracle10 freeze, external OwnerReservationWorker introduces a bounded256command transcript, one fresh namespace, no populated reinitialization, retained reservation/decision state, monotone install revision and no install during active evaluation. General command codec roundtrip and transition_step show every existing-state command frames it or maps to the exact reservation model e4f398. The driver commits its next value before output; malformed framing, evaluation/output failure or transcript exhaustion ends that process. Native custody/authenticated fresh namespace/OS failure refinement remain outside those kernel statements.

Fresh serial79module native build ea87b2 succeeds; binary421b375715cb82b5f61068726fb099236d871c5b427f8bec2286c10a22fe1adb. All-owned auditd21f4e covers10186declarations, standard axioms only. Actual Rust parser output regenerates the source prefix3a9747, rather than importing two executable main declarations. Five persistent-process transcripts97bbeb exercise real reserve/compute/install/abandon, repeated/changed/unauthorized request, pending reservation before EOF, uninitialized commands and old revision. Actual836byte native output is decoded and consumed by the original source binder first=10, once only66b584; an equal-valued real reply from a different request is rejected even after scope substitution. Expected owner envelopes are never supplied by the test harness. This is an actual component source/worker/receipt composition; no QUIC/authentic source/image custody or distributed E2E is claimed.

Five IO controls1022d4 cover partial header, stalled input after reservation, oversized frame, partial trailing frame and closing output after observing initialization/reservation acknowledgments, then allowing compute. Every failed process is reaped; no output-failed namespace is resumed or called same-instance recovery. Wrapper/codegen proof-script failures7f38e0/eeb40d and duplicate-main import013e2a are preserved. Oracle10 remains healthy00565207:32:59.907Z; actual wrapper sessionmir-w4-reservatio truncates requestedmir-w4-reservation. Wrong-name metadata lookup4b9963 did not read Chrome; START corrected5521b6/9a2964, no resend.

### Oracle10 collected and post-review verification — 2026-09-14T07:58:08.504056+00:00

Oracle10 actual sessionmir-w4-reservatio completed07:37:19.422Z; exact prompt receipt77dc83, wrapperc8ee9fexit0, answerSHA316dd5f6f75db6cdaaf812cd2f41ad6c4e72580626b94c74f6f0ccb0de508d5c, fully read8e4362/aa6aa7. DISPOSITION records source-inspection limits, findings and repairs. All10jobs collected, no Oracle polling/retry pending. No independent replay/signature/authority is inferred.

Fresh-root Inventory/rooted_inventory/rooted_reserved_computes1065ab connect every successful reservation to constructive production when computation is selected. OwnerOccurrence.Ordinals and Reservation.rooted_ordinals add correct initial-history assumptions; four compiler logs recoveredebfae7 contain only standard axioms. Pure completed transitions do not count interrupted native dispatch starts. Physical exclusive custody and quiescence mapping remain obligations. Receiver result validation executes arithmetic too; owner production and receiver verification must be counted separately. Minimal private invariant does not protect arbitrary mutated binding/name/site; rooted private custody remains open.

Oracle identified a remaining post-Popen clock exception leak, reproduced6ac05c, fixed45c8c8. Cleanup preserves primary exceptions and separately reports cleanup failure with exact child handle; injected kill failure is NOT successful reaping, and the test harness performs final recovery. Unsafe historical-PID signaling was removed from live fault harnessfe5ab7; frozen packets remain unchanged. Post-repair five protocol and five IO controls a9b626 pass. Actual native bytes again consume the original source binder4d4f5eexit0; prior extra --run commanddf2465 invoked imported worker main without arguments after successful #eval and exited64, retained as failed command. OWNER_RESERVATION_POST10_REGRESSION.json binds current launcher/input/response/receiver identities.

Correction to earlier CPU wording: old exit-9 near10elapsed seconds is consistent with configured CPU10 but does not establish cause independently. CPU10 is entire persistent process lifetime, not per command. Stateful repeated checks are not covered by old one-request timings. OWNER_GRAPH_LIMIT_IDENTITY_LINK.json734bd6 supplies current binary/input/generator identities against the historical FIRST receipt; it does not retroactively invent the omitted FIXED pin.

Existing private QUIC read1061–2772 maps reserved-before-await ingress, exact retained pending, checked transport ordinals, current runtime authorization and result consume; no raw-frame bypass/new production source. Status/plan/task/sample snapshots synchronized; new code remains external and unreviewed after Oracle10 freeze. W4 continues, no Canon/THM/OBL/policy/phase update.

### Private activation and saved continuation — 2026-09-14T08:14:26.961689+00:00

Main reproduced Oracle10 private-erasure attack12e01d: removing reference binding and substituting name preserves minimal machine invariant and matching receipt writes invented binder. This is unrooted state mutation, not an accepted source transition. New external QualifiedCustody retains initial realm/member/locus incarnation and guards tick/replace/continue; receive/cancel/head/control keep independent existing entry checks. Declarative PrivateEntry and actual evaluator have general exactness/relative completeness; rooted private source+ordinal preservation and pending_retained/live_pending_exact passb0815b. Existing waiting data is unchanged or cleared, never edited by the seven permitted entries. Proof script errors0f978b/79ae25 remain failures; no source holes.

Controls638f4e exercise actual parsed source prefix and all7entry forms: A departure rejects C initiation, authorized B joins A but old activation stays rejected, disabled/re-enabled member incarnation similarly rejected, unrelated authority generation does not retire activation, invocation/access/holding revocation leaves independent cancellation possible, replacement/continuation and normal receipt retain positives. Initial control layout9a0dc3 and Session BEq f1e8d3 failures are preserved. This is a reversible candidate; authenticated current view, private runtime custody and permission to rebind/resume are separate. Six-module changed/consumer all-owned audit a6cf07 passes702declarations with only standard logic axioms.

Oracle11packetoracle-w4-custody submitted oncef2cfc8 at08:13:00.258913Z, requested sessionmir-w4-custody, exec53793. Questiona839cde1a28c591465770a65e0f93d2da665c633b8e242202f7e93294ba50cdd, manifestbb11bf98a92c0683d1db890a3bcdc310ff07dba778937a2a72dd7c278ac8a449.38files275KB/90modulehashinventory, dryrunf929c3~88728tokens, --retain-hours0 also on dryrun. Initial inventory scriptb8772a incorrectly treated Std as project import; repaired before submission, no Oracle retry. First status check no earlier than08:16:00Z. No arbitrary deadline/Chrome edit. Discordprogressb0ec81 sent17:13JST; no completion.

Docs check00eb72 failed missing tasks canonical notice; restored. Successorcc6230 failed stale progress/sample updated headers after new timestamped evidence; fixing current snapshot headers before another check. Neither failed validation is called green. No production Rust/Canon/new sample root or W4 closure.

### Source input and native failure cuts — 2026-09-14T08:35:17.004283+00:00

Actual frozen owner binary421b3757 was stopped/killed at two native points: after evaluator return before decision allocation, and at third response with the resulting state/record retained before output. Both leave only initialization/reservation acknowledgments, no result (2259e7/747414, OWNER_NATIVE_PHASE_FAULTS_FIXED.json). The earlier GDB run reset command arguments and exited64 before either cut; 35dbc9 remains FAIL. Generated C inspection da5eb0 sharpens previous wording: the next state is held in an owned temporary before respond, while the loop-carried state assignment follows successful output. This is not a durable commit. Any failure terminates this synchronous namespace; no resume/liveness/authenticated origin theorem is inferred.

SourceCodec supplies complete ordinary/reference syntax, caller, source document/spans and placement codecs with general structural inverse/canonical and separate checker soundness/completeness (f30002). Actual parsed17statement program encoded345360bytes, exceeding the candidate65536byte frame. Compact document interning retains arbitrary multiple documents and checks canonical tables; expand/pack and general exact byte/source-typing laws pass087599, standard axioms only. Actual source now24187bytes/treeFuel60; actual/multiple-document, trailing byte, duplicate/unused table, missing document index and immutable-assignment controls pass60352d. First compact proof30742e failed string Boolean simplification/record eta; retained, never accepted despite compiler-generated sorryAx. No source holes. Source text needs a separate bound from short owner names. This remains a nonproduction input codec, not parsed-source equivalence, execution permission, observation disclosure or a populated Session decoder.

Oracle11 exact capturea3d2cd08:31:47.692Z remains matched/generating; no resubmission. One premature capture8eb6d0 was rejected by the180second guard BEFORE any browser read. Current source codec/native phase cuts are after that frozen packet. Required docs check3e3d69 fails tasks current-position missing exact Canon source; corrected to ADR0043 and fast targeted checkd533f7 passes. Earlier distinct notice/header/heading failures remain recorded. A new full docs check is still required; no current-green claim.

Oracle11 capture failure / retry — 2026-09-14T08:43:53.050339+00:00: wrapper0e62b8 exited0, but nested metadata028da0 has incomplete-capture and assistant timeout. Original exact question matched through08:31:47Z; df0cf1 at08:35:49Z reports mismatch, and wrapper-saved response answers an unrelated question. It is rejected and will not be sent onward, quoted as review or committed. Stored FAILED_CAPTURE.json preserves sanitized cause/identity. Wrapper log2997ce shows auto-reattach failing to locate the temporary conversation before falsely completing on another response. Local helpcf7c49 and source6614a2 confirm per-invocation flags; no global wrapper/config/Chrome edit. Identical38file frozen manifest8f217a was resubmitted once2ea38208:38:05.622775Z as mir-w4-custody-retry/exec41208, with --retain-hours0, --browser-auto-reattach-interval0, --browser-recheck-delay0 and --browser-keep-browser. Main exact-target capturee64838 matches and generates at08:41:53.526Z. A helper timeout alone will not end the model job; no duplicate on slowness.

Source input bounded soundness/relative completeness53a2c6 passes with byte65536/treeFuel256/digit128/source-text4096, independent of typing. Actual source fits7a854a; old owner-name text256 demonstrably refuses it. SourceInput structural finite rule/head/Core-control/all7command codecs and entry correspondence37be6f pass. New launcher/private driver proof is in progress; invalid Actor field818589 and Boolean-branch-premise d09aa2 are failed scripts, retained rather than accepted with generated holes. Mandatory historical example306 read in fullb6ef04; no adoption of its obsolete reopen queue.

### Actual source/owner processes and nonempty addition — 2026-09-14T09:06:19.431272+00:00

SourceInput launch/rooted preservation/explicit assigned realm/caller/member and no-reinitialize laws pass32713f; earlier818589/d09aa2 failures retained. Actual source/finite policy/all7entry controls4e8bc5 pass. First test wrongly expected an old-generation receipt to survive a new authority head; diagnostic77ffab locates generation1 receive rejection. Corrected control preserves refusal, then receives on the original matching head. Failed tactic/control attempts4a7850/77ffab/bc45f6 remain failures; no source holes. SourceWorker general private reply codecs and exchange preservation50b8fb pass. Private replies contain values/authority-bearing owner images for the trusted local launcher; they are not public/passive observation or disclosure authority.

Fresh86module native build c10b54/e8254a passes, binary3931b751aaf51acccbf87460a0ddc0f4ddcd05702393c64c99a6d22b193f1db9. All-owned audit a4c08f checks11427declarations, only standard axioms. Actual source and owner native processes44ad29 run17statements with real owner results10/10/11/10 and exact sourcecount3/first10/afterExchange10/afterReacquire11/afterRetire10. Waiting tick and duplicate reception refuse; no expected owner result is inserted by Python. Actual Rust-parsed continuation3statements c6a485 then201dba adds instance2 atC, reparents it to existingbase and invokes it using retainedcount3 in the SAME source/owner processes, preserving20writes and5productions. Exact captured source38/owner16 byte command/reply traces replay1928f0 against the proved transition/codec models; source retainsonearchive/threeinstances. Both actual processes exit0/reaped under4GiB/10CPU entire-lifetime limits. This is privileged-pipe component composition, not QUIC/authenticated publication/namespace allocation/secret observation/recovery closure.

Oracle11 retry completed08:54:43.175Z, wrapper31dc63exit0, exact final matched promptc97cb7. Answer83aca27f019253c00b8731bed679331d28029e87b60eb68fce58cb443458a2ab full0556e5/c49116/df86a7, RECEIPT/DISPOSITION recorded43dc29. All11consults now have a usable response; original failed capture remains rejected. Review is source inspection, not independent rebuild/signature. It confirms literal rooted inventory/ordinal/pending premises but identifies stronger custody/provenance gaps. Actual native a6651e reproduces older authority image under higher wrapperrevision leading to production, and a second fresh process under same scope901 producing again. Both are preserved counterexamples, not successes for currentness/nonreuse. Next consumer is guarded exact-private publication, owner-envelope binding and nonforking/nonreused physical namespace through existing Rust/QUIC.

Docs aafe0e passes after recorded notice/header/heading/current-Canon-link repairs,1764reports. Diff869100 clean. Later entries are forward evidence, no milestone close or production/Canon update. Latest user-facing progress is roughly3/10 workflow,24–60active hours low-confidence remaining.


Forward evidence 2026-09-14T09:30:24.556390+00:00: QualifiedReceipt full private arrival entry/checker correspondence, metadata/history preservation, immediate duplicate refusal and constructive production-to-receipt pass143bf2 with standard logical axioms only. Four prior rejected proof scripts remain external failed evidence; no source hole was accepted. CustodyPublication instantiates image-only publication with the guarded QualifiedCustody evaluator and proves rooted state, admitted-entry/receipt normal publication, held-current evaluation and retained-image installation57584b. Controlsfe649a replay actual prior five native receipts through17+3actual source statements,20writes/26publications, all7entries; wrong scope/revision/ticket/duplicate, active freeze/publish, stale image revision and old unguarded incarnation bypass discriminated. This is a local publication replay, not a new physical publisher or origin proof. PublicationInputc0c824 creates dispatch correlation only at enter, rejects raw receive, and proves source root/arrival correlation while retaining the full private state. Codec/fresh driver and native consumer remain in progress. Mandatory historical307 fully read af9315/1574ea, next308. Discord progress86b05d sent; no completion or Canon/production change.


Forward native publication 2026-09-14T09:43:39.056939+00:00: PublicationInput codec/fresh-driver invariant/no-reinitialization2f40bf and SourcePublicationWorker b5b394 pass. SourceWorkerSupport factors the prior external codec/IO without importing its old executable entrypoint; OwnerReservationWorkerSupport likewise factors the prior owner transition. This is pending final integration of the existing experiment, not a new framework/public API. Publisher native95module buildf7524e SHA9cefa05608269c16594e56771d167e0d43332f5d9235edeb993b6f1ad9ed5262,95module11978owned audit007be0 passes standard-only. OwnerEndpoint2553c4 proves local freeze ack only while idle, blocked reserve/compute under raised fence, monotone fence/retained histories and independent positive reservation. First proof script error from unreduced match was repaired; failed log remains. Endpoint native95module buildc00e53 SHAe0f45dbe29e4683ff5cc9a816e40aaf0e97a606679dfb9cef6a13516d2398861 passes.

Actual four processes45e53f passed: one publisher and three owner endpoints; every freeze ack is a response of the actual endpoint process before coordinator acknowledgement.26source-generated publications,20ordinary source writes,5owner computations; original17statements plus3nonempty continuation statements. Wrong scope/raw receive/duplicate/old revision/active owner freeze controls pass. Source397input frames, endpoint53/53/68commands retained under publication-process-x5uww_k3. No Python source arithmetic or expected owner result is forwarded. The finite profile uses512nativecommands because26three-endpoint publications already require286administrative commands;4GiBAS/10CPUwholelife/15wall perchild remains. Allfourreapedexit0. This is trusted private-pipe composition; a malicious supervisor can still replace individually valid images, and a fresh process may reuse an assigned scope. Source-generated output/actual private protocol is progress, not authenticated QUIC, cryptographic origin, physical exclusivity or recovery.95endpoint audit and actual full byte replay running25634. Full docs01cff1/f4ed9a PASS1764reports. Mandatory308/309 fullyread22a4b2/ecd556, next310. No Canon/production/THM/OBL update or W4 closure.


Forward generic source dispatch 2026-09-14T09:52:42.062594+00:00: full native endpoint audit28b232 passes95modules11962owned standard-only; actual first native publication/endpoint traces replay28b232. Inspection found the test supervisor still selected C literally; native source/owner executors had no example branch. Prior fixed-C harness and receipt were preserved, then supervisor routing was changed to the actual ticket.place and per-owner ordinals. Actual Rust-parsed additional source atA f555ba changes only the explicit allocation locus, and final generic C and alternateA four-process runs36329f pass with identical binaries. C run owner counts0/0/5; A variant1/0/4. Full final source/endpoint byte replays179429 pass for both. The source programs still generate all tickets/results; Python forwards actual values only. This does not fix physical scope reuse or malicious supervisor image substitution.

Oracle12 submitted once ac9795 at09:50:42.693953Z, sessionmir-w4-live-publication, exec32717.53frozenfiles438463bytes, QUESTION464b1998c54f35548cd25fd16b7fad041b6d59ac6d77e8d80b77fa388196ff39 / MANIFESTf9dcbbbab87339fe70b867ba8db637276212612374e332372591a24bbf637191. Dryrun84eb55~142666tokens; actual additionally attaches the frozen manifest. --retain-hours0, --browser-auto-reattach-interval0, --browser-recheck-delay0, --browser-keep-browser, no Oracle deadline or paid fallback. First exact-target capture not before09:53:43Z. Requested counterexamples to all-entry/receipt/publication/native scope and minimal real authentication/nonforking consumer; no approval/signature requested. No other running compiler/nativejob.


Forward audit 2026-09-14T09:55:26.283007+00:00: Oracle12 actual session ismir-w4-live-publicatio (CLI truncated requestedslug). Initial exact-capturebf8060 found no metadata under requested name and never read Chrome; ownwrapper41c02a supplied actual name. START keeps both;9acfdf09:54:13.069Z confirms exact question and generating state. Same exec32717/session retained; next capture>=09:57:14Z, no resubmission. Native recorded-cohort comparison508ba4 shows two separate full four-process cohorts with scope811 have byte-identical first4owner production envelopes (independent source/owner PIDs; original inputs and actual response hashes retained in PUBLICATION_COHORT_REUSE_COUNTEREXAMPLE.json). This strengthens the previously disclosed physical nonforking gap; it does not falsify the fresh-namespace premise of per-state theorems. Existing Rust private local-cut module1203lines fully read; noncloneable bootstrap objects eventually yield clonable Vec frames to the trusted launcher, and runtime-bound local-cut tokens are not a global cross-process uniqueness mechanism. No accepted I3 defect or unreviewed production fix is claimed. Runtime file20k remains only partially read.

### W4 guarded native publication and retry discriminator — 2026-09-14T10:11:30.176814+00:00

Forward LAB evidence only. Generic source-generated ticket routing composes actual publisher plus three owner endpoints, with26publications/20writes/5productions and nonempty source continuation atC orA using the same binaries (36329f/179429). Native publisher11978 and endpoint11962 owned declarations in95modules audit against only the three standard logical axioms. All actual source and endpoint byte traces replay; this is finite conformance evidence, not physical-origin/nonforking proof. Two actual complete cohorts using scope811 reproduce identical first4production envelopes (508ba4), so namespace reuse remains a concrete direct-consumer obligation.

Actual lost-install acknowledgement bc4904 leaves source waiting: old endpoint rejects the identical installed image/revision retry. A forward candidate uses complete canonical image equality with general codec injectivity; exact stored image/current revision/current fence returns confirmation without changing any state. General sameImage/confirmation/owner-step/changed-image refusal550cbb pass. New two-module native rebuild1525daab81850a59161ed2a20c0b3c13dcf6c4af4761826ee4f445a5a3ac4aa9 passed;6b9101 actual lost-ack path resumes to genuine owner result10 and rejects changed image/older fence. Confirmation during active reservation does not acknowledge quiescence. C/A regression578c1e passes. One test-construction failure593958 assumed adjacent publications must change owner image; preserved FAIL, replaced by demonstrably distinct initial image. New audit/full replay pending at this update.

Oracle12 actualsession mir-w4-live-publicatio, requested mir-w4-live-publication, single submitac9795, question464b1998c54f35548cd25fd16b7fad041b6d59ac6d77e8d80b77fa388196ff39/manifestf9dcbbbab87339fe70b867ba8db637276212612374e332372591a24bbf637191. Exact-question capture774fa4 at10:10:21.979Z remains generating; no retry or time limit. Earlier3422c4 attempted capture too early and interval guard refused BEFORE browser read. Later lost-ack repair is outside this frozen packet and needs narrow review. Existing Rust M8 admission gate fullread59d163 uses checked fetch_update and exact live-instance binding, explicitly local-process-only; no global namespace guarantee follows. No production/Canon/key/public-wire/policy adoption, no W4 close.

### Oracle12 collection and executable output closure — 2026-09-14T10:34:25.548532+00:00

Oracle12 exact-question finala0f845, wrapper208417exit0, fullb79157/a62844, RECEIPT/DISPOSITION retained. No Oracle job remains. Review is advice, not proof/acceptance; F1 matching actual lifetime/round/image, F2 cross-coordinator namespace exclusion, F5 entry-specific receive/cancel/rejoin and ignored ordinal, F6 terminal owner evidence for finish, F7 composite capture joins, F8 future policy/parser provenance/disclosure remain explicitly tracked. Codec preserves document plus byte offset, not a complete independently verified source span. Private rejection output remains private.

F4 combinedprimary/kill/logerror528b9f reproduced failure to retain original error/exact handle; forward harness repair7ccd4f/1e5eb7 passes. Native stream EOF/extra output25c496 and optimized mode0a831a discriminate missing capture closure. Actual C/A runs224024 with unconditional checks under-O pass and capture full frames; prior payload-only captures are not retroactively relabeled. These are source/runtime experiment controls, not authenticated network or same-instance recovery.

F3 actual Rust-parsed8programs f2c009 each encode17506/17541bytes with1024character distinct local names. Actual4process8ef53e/413eb2 atsource-growth-process-kttt6wyo completes7writes, then freezes all3owners atrevision15; publisher exits1 with explicit private response frame bound error, before returning publication15. This is a real predictable representation-exhaustion counterexample, not an inferred OOM. Cleanup reaps remaining owners by its explicit kill path.

PublicationCapacity general finite-path preflight6b29b2/735fe2 derives an encoded source completion sequence, command budget and preserved rooted source from its actual executable checker; relative completeness is restricted to that finite declared profile. Publisher-only positive/negative controls c597ad accept14growth stages and26C/26A stages, reject the oversized eighth write before announcing, and distinguish10versus11source-command budget. Endpoint install preflight/image limits and prospective held output added in c933e7; extract proofs/controls pending. Native publisher has NOT adopted this guard yet. CPU/deadline, owner command quota, actual event/namespace/admission/disclosure and arbitrary interleavings do not follow. Failed proof scripts e9631d/ccbbc4 and control field failures remain failures; all passing general statements use only standard logical axioms.

Full docs09b085 passes1764reports; subsequent ledger/status update remains later. Discord progressdab742 sent; no complete. Same11own docsdirty, all new codeexternal; no Canon/production/public contract/owner key or backend acceptance. W4 continues.

### Oracle13の反例と修正候補 — 2026-09-14T12:16:11.223767+00:00

Oracle13を回収し、一般定理そのものと consumer 義務を分離した。有限 Driver で拒否 launch による最終 install 欠落を2b7b43で再現。旧 native は511件正常EOF、512件全応答後exit1を381afbで実測。capture checker の alias/enter→stage/source-only prefix 誤受理もea2170で再現。既存成功証拠をこれらの強い主張に読み替えない。

現案（全受信を512回に含める）は拒否・再送で予約手順を失う。最小代替を、成功した内部状態遷移の有限 quota と、実状態に束縛した残り手順へ変更する候補とした。拒否は source/quota/suffix を保持し、別の受理操作は変更後の残り手順を全検査する。受信回数・CPU・DOS可用性の上限ではない。実 worker はEOFまで応答する必要があり、旧512回 loopを使えない。予測を実 ack とも認可とも数えない。immutable assigned realm は各予測状態で検査する。まだ native へ採用せず、機械証明と通常source正例/反例を先に行う。

### 固定source容量consumerの実機対応 — 2026-09-14T13:14:11.943215+00:00

一般命題: BoundSequence/checkBoundPath exactness とassignedrealm、有限local-prefixのsoundness、拒否時のsource/quota/suffix保存、reserved_head_admitted、fastCandidate_correct/transitionFast_exact、任意payloadのBoolean false replyがtrue以下の長さになる命題をLean4.29.1 trust0で検査した。初期の同長仮説は351315で反証、修正は73ab4b。sourceのready residualを勝手に置換する案は採用せず、追加sourceを採用する前に局所prefixを検査する。

実機: 旧naiveguardの3writes時終了209ce1、fast/double-encodingの6writes時終了6eb171を保持。後者はsource zombie9.98CPU秒、他3owner各0.02CPU秒でcleanupによりreapした。OOMとは断定しない。新sourceバイナリd8edb79fe4b5d04d195b0cd78ea68b46a6095608e4276a6df96b96951ffa60caは同じ4GiBAS/10CPU全寿命/15wallで、容量拒否後の通常source追加、厳密12受理遷移の完了、513追加拒否と正常EOF、通常C/A配置を通した。finalgrowth/quota86eb97、normalC/A c2830d。7値を保持しrecovered=0を新たに計算した。これはprivate pipeの実4processであり、QUIC、認証、永続復旧、public観測ではない。owner wrapperの512受信回数制限とowner予算は別の未保証。

実build102module12586owned全宣言監査6b7209は標準propext/Classical.choice/Quot.soundのみ。4source/12ownerの全入力について実preflight/decode/transitionと全replybytesをLeanで照合40e5ceし、最終ownerimageとpublisherimageも一致した。finalgrowth/quota再採録は全frame/payloadのbyte直接同値、同じargv/binary/limits/正常EOFから先の検査へ接続03a39b（再実行と区別）。C/Aの実571event×2は8種類のedge/framing反例を拒否3048b8。採録前driver/hashはfinalreceiptsに記録。

Oracle14は45file381505bytesの凍結packet oracle-w4-suffix、question760e5f8f7c7bc6bd00f06ab9ad8e9d6717bfee9e1131b871e4314581fa1a9bd4、manifestfec584d82406dc92872ab059117c95e309492645940468a4417cc39290f1d835。28e3b3で13:05:37.886299Zに一度送信、sessionmir-w4-suffix/job66167。165c95の13:08:57.527Z exact question trueで生成中。任意締切や再送は行わず180秒以上間隔を守る。レビューは受理・証明・署名ではない。

Documentation/project-status/CURRENT_GOAL/progress/tasks/samples_progress/plan/W4_CHECKを同期。READ_LEDGERではM8admission2574、authority1061、ownerqueue1572、SYS5runtime12168が既に同hash全文読了と判明し、無意味な再通読は行わない。SYS4dispatchは12170–12355を追加しても全体は部分読了。次のconsumerは真正なnamespace・実行lifetime admissionと既存Rust/QUIC。Canon/THM/OBL/Plan250/I3-4・public契約は更新せず、W4未完了。変更は11own docs等と外部codeで、新commit/pushはまだ行っていない。sub-agentは一切使用していない。


Forward Oracle14 / lifecycle counterevidence 2026-09-14T13:46:35.279777+00:00

Oracle14 mir-w4-suffix exact finalfc5e9a13:28:03Z, full da1979/65fc1a, wrapperc87001exit0; RECEIPT/DISPOSITION9f9ffb. Answerfile0640248c4398b6c601996d91d51e1b1f1a05d3ca5a71e75c480a9ca584a5a670. No live Oracle remains; no theorem refutation found by read-only reviewer, no independent Lean/native/raw-capture check. Findings concern lifecycle goal coverage, input/reply profile, composite outcome/assignment/failure framing, repeated evidence attachment and transitive cache context. They do not grant owner authority.

Main actual four-process101-credit counter6bc6dd reaches first owner computation10, then cannot finish held source interval.113-credit counter973067 receives actual result and completes publication but leaves a ready residual at quota0. Both close actual pipes normally; they are reproduced failures of completion coverage, not successful source completion. New source guard controls from typed bootstrap7274d6 additionally reproduce profile refusal of semantically admitted leave, member disable and unchanged-incarnation head refresh after8ordinary ticks (e8e6ec/c76126/a46a1b). These assertion failures deliberately kill/reap exact children; no OOM inference. The scheduler tries old predicted tick/completion against a newly staged administrative round. New native positive guard cases have NOT passed.

Local Python falsifiersbe3469 reproduce failure-code erasure, missing argv skip, the ACTUAL113 ready-residual cohort passing the generic terminal predicate, repeated evidence overwrite,128-vs129digit reader boundary, and three-module cache selection Leaf1/Mid1/Top0 using the helper's AST. Repairs retain raw source status, require typed startup assignments, distinguish normal source completion from installed prefix, and nest repeated evidence failures. Focused checks66d741 and stronger nested failure/handle testabf016 pass; the latter injected tests launch no child. Later helper hashes differ from old recorded traces and are not retroattributed. Read-only99-reused-module origin source-closure audit83ffc1 finds no mismatch in the current specific binary; generic cache selection is still defective and must be repaired before further native reuse.

New external PublicationLifecycle is the bounded successor experiment: checked completion must end installed/settled/no-held and source waiting, failed, finished, or inactive original incarnation. It recomputes from the actual candidate for off-schedule entries including arrival and administrative controls, while same scheduled heads retain a fast path. It does not generate physical finish authority or owner results. General transition/refusal/refinement and exhausted no-held/active-ready-finished proofs initially pass1cd316; later stronger planner/fast-path proof file is still compiling after retained syntax/type failurescc1c00/115c42/a1ceda/7c2f61. Failed logs are not accepted proof evidence; no source holes or failed olean consumers. One dependent control invocationf7e265 failed because no compiled module existed, and is not a positive. Full successor build/audit/native/review remain pending.

Docs header fixc457a7 follows real clock; validatorb9eb82 passes1764reports. Mandatory historical312 full445fa8 registered381228; next313. SYS4dispatch12355–13670 fully read sectionwise4cbd89/787647/dde962/17a517/14978e/a14a85, wholefile still partial. Patch clone and restore do not propagate a live owner gate. Same11own repo files dirty; no new commit/push/Canon/THM/OBL/Plan250 change. W4 remains active, W5+ inactive, no subagents.

Forward 2026-09-14T13:50:00.247637+00:00: full successor PublicationLifecycle-SIXTH18d3c0 PASS, standard axioms only including plan_certified/fast equivalence/general exhausted quiet/no-held/active-ready-finished. Prior four normal native source streams396/396/167/526inputs replay identically against stronger model91ffdb; not new native evidence. Revised composite C/A58b308 passes positives/eight mutants each. Boundary controls76661 still running; no new native build or materialdelta review.


### Forward evidence — lifecycle native consumer and pending review (2026-09-14)

Fresh103-module build629e7nyr completed e2e896; binaryaad0db8d3a6e265dacefa21ae674ed38c19bf9aa1bcb3c76aa91959c7640df99. All12682owned declarations audited657684 with only propext/Classical.choice/Quot.sound. Old context-less caches skipped; transitive/toolchain/artifact context selector repairc5ec2d passed its inert countercheck. This is local provenance, not cryptographic acceptance.

Nine actual four-process cohorts now pass: normalC/A (20writes/26publications/5actual owner results), growth plus smaller ordinary continuation, quota12 with513later refusals, locus/member rejoin guards, actual result rejection after revocation plus independent cancel/ordinary replacement, and101/113quota discriminators. All36children exit0 with complete stdoutEOF. Composite858154 preserves actual full-image/ack/envelope/terminal joins. Exact frozen-build Lean replayf85b02 passes9source+27owner streams and names each final outcome: complete source, retired activation, waiting, or cancelled. The waiting/cancelled cases are not original-program completion. Evidence: external LIFECYCLE_CAPTURE_REPLAY_CHECK.json and LIFECYCLE_COMPOSITE_CHECK.json.

Preserved failures: late-result harness double-wrapped an already encoded PublicationInput (native invalid-input exit1,2.5CPU seconds; notOOM). Its second expected count confused source step replies with owner results. Both corrected in test-only code, with FIRST/SECOND_FAILURE receipts retained; third8ba03c passes. Replay first missing Bool annotatione299fb failed elaboration; no eval! or downstream use, SECONDf85b02 passes. No semantics weakening to rescue tests.

New actual reader counter0002e6/58b413: arbitrary startup scope2^128 reaches enter, producing8491bytes within frame bound; Python128digit reader refuses that actual source response. Cleanup then kills exact cohort; not native crash/OOM. Literal ReplyFits remains byte-length only. Full finite recipient-profile guard versus startup-only restriction is a reversible unresolved technical comparison; neither numeric/auth/public policy adoption nor reader-totality proof is claimed.

Oracle15 single submissionc71280 at14:15:15.271104UTC, sessionmir-w4-lifecycle, wrapper23445. Frozen42files400565bytes; question4b2022354859c8930e4a2d0efb9ae73a817d3118360e76bca87b255838cb9d61; manifest09c773d4dd99b1e3780a892a2fcb54e6a10082c576565ccd60205ffea8a31f9f. Dry-run107c7a estimated129431tokens. Review pending, not acceptance; first exact capture after14:18:15UTC, then>=180s. No Chrome config/subagent/public/Canon change.

Owner quota steering: use fresh Codex weekly metadata about hourly or less often, natural checkpoints; stop around30%remaining after collecting evidence. Last observed14:00UTC remaining50%, no new check before15:00UTC needed. This is a pause condition, not W4 completion. Next mandatory historical reading314; status/docs still need final synchronization for this increment.


### Forward evidence — recipient profile and residual scheduling (2026-09-14T14:54:23.741327+00:00)

Generic PublicationReadability strengthens optional output PayloadFits with digit128/text4096/treeCost+status wrapper budget256. Exact checker/declarative agreement and all three status wrappers, actual bounded decode equality and preservation through Capacity/Driver/Lifecycle pass193bc3/51fa07. These are source-reply claims; Python decoder correspondence, owner terminal outputs and arrival wrappers still require their own obligations. Nine prior native source/27owner captures replay against the stronger reader model800f9b, not new native runs.

First104-module native b848ee64 passed seven boundary/authority controls but growth stopped after7writes before shorter repair. Actual pre-reap CPU9.97s, exit-9 and empty stderr6be9c6 are consistent with the unchanged10CPU whole-lifetime limit, not an OOM claim. The duplicate newly certified plan check was removed only after general checkedSchedule_exact/transitionPlanned_exact proofs, which require no invariant premise. New binary6bfed502ec9a017bfa52fe04fbcb987312d5ab647302802d273066c420e0f310 passes growth7557ce/54ecb6, the remaining11 native casesbd5a4d/884883 (including seven malformed/EOF subcases), and full104module/12753owned audit08b1d8 with standard axioms only. Historical b848 failure and unexecuted tail are retained separately.

Oracle15 exact finald0d3b5 at14:41:42Z, wrapperedfd2aexit0, answer aabc66c259aa7033cb1d66664129ffa21824e8d5cb691437a10345ffe21b7d8c. Full64c20d/a1237a. No new Oracle live. Review found residual-round overcharge, Quiet versus parked dispatch distinction, receipt startup/finalization/exception defects, and compiler-selection omissions. It did not run Lean/native and did not review the later Readability source; advice is not proof, owner authority or acceptance.

Main reproduced F1-first at quota12: original semantic transition admits freeze1 and an independently checked nine-command path ends Quiet, but lifecycle refuses (ab6aac). Actual four-process F1-first79623d and interleaved freeze/ack0158a7 reproduce the refusal then complete by canonical order. Candidate correction removes one actual command occurrence from the old suffix and rechecks every residual step against the real successor, falling back to fresh planning. Alternative current-round mask-specific planner would add another specialized reconstruction; general residual rechecking is the smaller change. No commutativity or actual owner acknowledgement is inferred from list removal. General residualPlan_certified/schedule_of_residual/residual_admitted and preserved all-entry/refusal/fast/readability obligations passd996cf. FIRST elaboration failuree40444 remains FAILED, with no dependent native use.

Compiler context3 now invokes absolute lean/leanc from the inventoried installation and supplies an allowlisted environment. Imports come from selected lean --deps. Copied source/artifact digests are compared against their expected values. A real Lean/C artifact test10c15d with conflicting PATH leanc and LEAN_CC confirms those overrides are not invoked; compiler/runtime/OS/unchanged local custody remain TCB. Fresh104-module buildcead0b, binaryd4e798d38e547155792c072b28cfd0de6505eda4fedfed369fdddc0ffe330776, is undergoing all-owned audit before native permutation positives.

Helper counter52f0ab reproduces failed receipt acceptance, dropped startup parameters, passed JSON with post-write diagnostic exception, and replacement of a pre-existing explicit cause. Corrections carry startup parameters with events, reject failed top-level records, define final receipt write as finalization without failure-bearing diagnostic output, and preserve explicit causes while retaining cleanup groups. Four targeted repairsb1bc90 and nested cleanup regressiond79fb4 pass; these injected faults are not native evidence. Explicit startup-to-generated-replay binding and parked-after-terminal outcome remain consumers.

Mandatory SYS4 read353e5e extends to16055; wholefile and corpus remain partial, next historical316. Same11own repo docs dirty, later implementation external, no new commit/push or Canon/THM/OBL/public contract change. Owner quota last14:00UTC50%remaining; hourly-or-longer checks continue. W4 remains active, W5+/Plan250I3-4 inactive; no subagents.

Forward 2026-09-14T15:10:39.215627+00:00 — actual residual worker d4e798d3 passes104module/12774owned audit628591, both notification permutations06b410 and all13native case invocationsed41b3. BoundCapturedReplay13source/39owner streams PASSace523/d36c24 (generated from exact captured argv and receipts, fullimages and EOF checked). Actualquota102 is parked_after_terminal, explicitly not source completion or durable recovery. Source-result predicate now requires actual original pending/dispatch and one terminal matching production. Oracle16 single31d77c15:03:27Z; Qd7678d5f8bc0abb141bd4ac3949f8323689d4fbe7256209ca84a03337bef472f,manifestd79c639774eae2762cf6568f5eaccc9375b142be4469d8b0b6eb5db1f807cf21; dry8872da155175tokens; review pending. Docs validator8df4f5/d14029 PASS1764 after snapshot timestamp correction; initial update script b64dc6 expected Japanese header but samples uses Last updated, corrected586797 (no validation skip).

Existing W3 InstancePrograms.Machine already uses checked Int64 arithmetic at every node (full1aa704, CheckedArithmetic25bc3e). An unbounded mathematical computation yielding2^128 is therefore not an established reachable owner-result counterexample in this profile. Scope2^128 remains an actual counterexample on the prior reader cut. The next proof should consume the already accepted Int64 result bound and actual owner ordinal/capacity, then establish whole response/arrival shape bounds; no duplicated predicted computation is needed for numeric range. New external OwnerResponseProfile is proof work only; failedFIRST/SECOND logs df051d/29d347 are not dependent build evidence. Mandatory historical316 fullae1ea3, next317. Weekly quota fresh15:01:13 telemetry checked71d3fc15:01:32 gives48%remaining; next no earlier16:01:32UTC.


Forward evidence 2026-09-14T15:29:48.943061+00:00: OwnerResponseProfile THIRTEENTH0bf972 (external only) checks general Int64 range from actual W3 semantics, rooted owner ordinal<64, compositional byte/digit/text bounds, exact responseCheck equivalence, owner_reply_readable and arrival_readable, and constructive rooted_reservation_readable. The latter assumes profile on the pre-reservation scope/revision/ticket and a rooted reservation, not produced output/readability. Conservative profile reserves373 bytes and24 tree fuel beyond ticket, plus scope/revision128-bit and ticket text256/digit128. Fixed positive/boundary controls are running; no native consumes this module yet, no Oracle16 coverage (post-freeze). Failed SEVENTH/NINTH/ELEVENTH proof elaborations retained as FAILED; no source holes or Mir axioms, final audit only standard axioms. Mandatory historical317 read full7d7094; next318. One premature Oracle capture159a36 was rejected by >=180s guard before browser read; same job continued, last exact6b00df15:26:34Z generating.


Forward Oracle16 findings and local verification 2026-09-14T15:47:18.374919+00:00: exact final8db28d/121ae5, answer f0f3b8a0f4b7ec62cbfc2263218c0deb3c97eeb66e2bb1367b8f81d103ecbf91 fully read aaf74d/34cc23. F1 post-save output reproduced from actual finalizer statements with a supplied result (f2003a, exit120/passed file); first extraction753407 failed before receipt and is not evidence. Both replay/build post-save prints removed. Full generator with real Lean success0/failure1 under closed stdout passes0feeb7/e14c73; build-path execution pending. F2 full real model counter e2b5f1 confirms original cancel/enter and actual BoundFits; actual4process counter1d3958/2af51e (source-partial-round-s2g9yk28) rejects enter after only owner1 freezes. No owner computation follows refusal. New releasedPlan retains completed notifications and certifies release+residual before full fallback. General release-path relative admission, all preservation and both optimizations PASSb9144d, no schedule-success premise on released_path_admitted; model positive36f9f8 completes exact10 remaining credits to zero. This does not supply physical terminal evidence. F3 actual completed freeze/ack and old notification during later round reproduced726480; generalized historical_no_new_credit/frozen/publication and unchanged generation proofs PASSf6dcf8. Composite checker distinguishes old evidence from outstanding closure; same actual3captures + all prior13cohorts and2 old-as-new mutations PASS2ead87. No new source binary from these changes yet.

OwnerResponseProfile FOURTEENTH82bc94 and finite controls80de15 pass. Reserve wrapper and text-bound monotonicity added to whole owner reply/arrival and constructive pre-reservation result readability. Proposed Capacity consumer now checks current/prospective dispatch and actual initialize as well as install recipient profiles; base general proofs86c32a pass, dependent driver/lifecycle proof check running. All changes remain external, after Oracle16 freeze, without production/Canon promotion.


Forward evidence 2026-09-14T16:09:40.011500+00:00: current105source f29a12626635050f0be21a60ef54931dc87a261f5823c2bdd777828c13f98f2b passes all12913owned declaration audit85c8a3 (standard axioms only) and same4GiBAS/10CPU-lifetime/15wall native controls. Actual partial-round enter and growth pass85c8a3/182bca; sixteen remaining invocations pass514f8a before first normalA fails due the main harness's wrong continuation path, not native semantics/OOM. Original failure retained; corrected actual path reruns only A and passes1f0fa6. Complete17source/51owner byte/argv/final-image conjunction cb16a3/1f784d passes; BoundCapturedReplay.log SHA2c3b07f769b8a9922eca118ebb83c702688101a607b39a09a69178fba4429408. These68children include waiting/cancelled/retired/parked outcomes, not68completed programs. Old104 evidence is preserved under pre-owner-profile-evidence, not attributed to105. Source-only scope and seven framing faults are separate. PublicationNotifications general proof and historical-checker regressions now pass; build/audit closed-stdout checks pass.

Direct-owner falsifier: 2b68a0 deliberately fails the original model's output-readability assertion after ordinary-source image initialize, reserve, actual-model produce10 with startup scope2^128. Every command is itself in the bounded recipient profile. The old95 native owner reproduces all3commands, normal exit0/EOF, and actual bad-number recipient rejection fa5280 (OWNER_DIRECT_PROFILE_COUNTER_CHECK.json). Existing conditional source-dispatch protection does not close this standalone entry. Current proposal: independently check scope/revision/ticket/capacity before reserving/dispatching, prove preservation over initialize/reserve/compute/install/abandon/freeze/reconfirm, then connect the private worker. Smallest alternative is retaining only the source guard, which leaves this reproduced direct-entry obligation open. A defensive compute precheck must precede any computation and cannot erase reservation history. Progress requires independent admitted/fresh/room/profile premises; no all-refusal or assumed-readable-output theorem. New private refusal code is provisional internal LAB, no public wire/authority change. This remains W4's same semantic goal, direct consumer owner boundary; no new lane/report.

Oracle17 mir-w4-owner-profile submitted once d9e65b at16:07:54.207672UTC with frozen46files552875bytes, question2c117d07c747463787831b1b6d36674b13f6a409c2781256376b76ef693fa379, manifest0f25274834eab3302efbd2214bfbd51419b2db11bce62ea5cb09f8e78c3bf1bb. Drybb1798 estimated203304tokens. Review asks minimal counterexamples in105response-profile consumer and Oracle16 F1/F2/F3 repairs; standalone owner extension is explicitly unimplemented. No deadline/retry, >=180s exact question checks; no subagent/Chrome settings/paid fallback. Current owner remains95binary; Rust/QUIC/authenticated namespace/command budget are open. Weekly metadata379af5 at16:03:06Z fresh16:02:44 gives46%remaining; next no earlier17:03:06UTC. Main focus remains W4, no premature completion.


Forward direct-owner profile 2026-09-14T16:30:23.815794+00:00: external OwnerEndpointProfile general checker equivalence, all-mutator active-profile preservation/rootedness, refusal framing and every_reply_readable pass FOURTHbfaf05 with only standard logical axioms. The latter handles arbitrary supplied state by PRE-compute checking, but grants no populated-state import. enabled_roundtrip constructs reservation and production from independent admitted/current/scope/fresh/room/profile and rooted inventory premises, not successful schedule/readable output. FIRST31cd32 and THIRDebad07 type inference failures remain FAILED, never dependent native evidence. Finite controls SECOND17bd6e pass; FIRSTa89e76 expected install2 after refusedfreeze, but the unchanged old fence correctly returns14. The test expectation was corrected without changing semantics.

Private native consumer439558/db25bd uses101modules and binary8e53a6e0dfe343d325f4ea1d78b7a7bcd772af40425bc3179dd82631522bf8de; all12450owned declarations audit543728/966e4e passes standard-only. Four direct actual processes/34commands966e4e cover ordinary128scope,129scope pre-reservation refusal, active freeze/install/reconfirm, duplicate requests/compute and retained abandoned key. Full34input/output framed capture replay f77f63/81159a passes; log151b2cda659e93011c8dfbde5d905185ff3a06c5f56c40a23232a486d4675709. This direct fixture originates from the ordinary Lean source-control state, not a fresh Rust source E2E claim. Actual C/A source105 plus guarded owner101 four-process continuations d62b3b pass20writes/26publications/5actualresults each. Combined2source/6owner exact-byte/argv/final-image replay361a00/81159a passes, log0cfd256371d8526be6601a746e06ed06a4adc236fb8da7c57c0b0d9985ea1c9f. All remain within unchanged native resource limits.

Existing build/audit/replay helpers gained explicit owner mode, leaving default source105 receipts immutable. Both source/owner audited frozen roots and shared module context/recipe equality are checked before combined replay. The executed direct-native helper was preserved byte-identically by its recorded SHA after adding the separate replay entry (4ada18); this recovery is documented, not an invented historical source claim. These post-Oracle17-freeze changes still require material consumer review. No new production, Canon, THM/OBL, key or public-wire decision. Owner lifetime command-budget closure, physical namespace admission/exclusive custody and Rust/Core/QUIC integration remain open. The old mirrored92module runner is not silently widened; during eventual mirroring, carry finalizer discipline into scripts/proof_first_reference_source_check.py, whose current post-final-receipt print has the same structural concern as Oracle16F1 (not yet reproduced/fixed here).


Forward 2026-09-14T16:48:18.340550+00:00: Oracle17 exact FINAL465b0a16:39:38Z, wrapper98866cexit0 (finished16:28:55); fullf242b7/019b8b. Captured contentd93e0617a25391fdd74a64ce8606da26fd97f7bc1d35dab8510d6bb8f3f3eab5, fileSHA73a9fbdb4857e93aeff560f426d072229f3e0e181189db776061cc91d1175df2. No Oracle remains live. F1 standalone composite tail reproduced with FULL actualC/A composite/mutant checker2204dd, committed results but producer120; removal repair running56074. F2 same unchanged audit helper isolated missing-compiler fixture8b450c exits1, leaves old passed JSON and truncates old log; the old downstream predicate still true. This is exception accounting, not a false Lean theorem or actual failed historical audit. Plan immutable attempt/result selection; no old successful evidence replaces a newly required failed attempt. F3 Oracle's synthetic historicalack/fence mutation is not a full-conjunction counterexample; actual captured mutation/full replay discriminator pending. New owner101 consumer is after review freeze. Profile refusal before reservation is not actual terminal completion or abandonment of an active request.

Owner resource counters ed6434/a48fff: native101 with512loop slots accepts reserve atinput512 and exits1; alternative reserve511 then denied initialize512 also leaves no compute slot. Both retain actual captures and explicit exhaustion stderr, no OOM inference. A last-reserve-only guard cannot close the second trace. New external OwnerEndpointBudget structurally reserves two credits for reservation or retained-active nonterminal work, one for releasing compute/abandon/idle operations, and rejects insufficient cost before any owner computation. Budget-refused input retains state/credits; a funded underlying profile/semantic refusal may spend a credit. FOURTH4a3210 general initial/preservation/rooted/exhausted-idle proof PASS with standard axioms; earlier type-inference/equality-goal failures remain FAILED. Normal constructive progress, response16 readability, finite positives, native connection and coupled source/owner budget remain pending; no production adoption. Planned semantic credit loop is distinct from host CPU/memory/malformed-IO failure, which cannot promise unconditional availability or recovery.

Docs validator4e7def/465b0a PASS1764 after current snapshot timestamps and concise CURRENT_GOAL rewrite. SYS4 suffix12170-end and prefix1-500 are now actually read; prefix501-12169 remains incomplete, not full-file reading. Mandatory historical320 read93f2f6, next321. No new Git commit/push/Canon/THM/OBL or subagents.


### Forward evidence — completion credit and required audit attempts (2026-09-14T17:07:25.381617+00:00)

Sole main; same W4 goal and11own dirty docs, HEADab316353. No Canon/THM/OBL/phase/public or production change. Native source105/owner101 remain previous binaries; new budget worker is only elaborated external reference code.

OwnerEndpointBudget general module FINAL802157 exit0 (`355c8457e73e27d05c2e971aaff5df02f857bbdbd8e998bf116e336f73133681`) includes all-mutator completion-credit preservation, exhausted-idle, every-reply readability and independent rooted/admitted/current/fresh/room/profile/credits>=2 reserve+compute progress. Only standard logical axioms. Finite controls6e8b25 pass ordinary10, two final-credit counters, interleaved refusal and explicit abandon retaining key; FIRST2afaf9 dotted identifier elaboration failed and was never executed. OwnerBudgetWorker FIRST failed codec namespace; qualified consumer SECOND71b34d elaborates with unchanged framing/preflight and an input-until-EOF loop using512semantic credits. No native budget process or source/owner coupled-budget theorem yet; OS/IO failures still terminate, no liveness claim.

Oracle17F1 actual complete standalone C/A checker with closed stdout now exits0 with identical resultSHA bebe6f... (32d2ed). F2 failed-audit reproduction8b450c repaired by new explicitly selected per-attempt artifacts, failed/missing attempts cannot fallback to history; isolated real-helper control d0b3fd preserves both old result and old log. REAL105source/101owner new audits d977ee/ec534b exit0 still12913/12450owned. Full actual C/A captured replay with explicit selections9103c0 exits0; audit helper is local custody/selection TCB, not authenticated or durable evidence. Old successful audits are retained, not relabeled failures.

Related F2 aggregate counter311347 is ACTUAL4process native success plus deliberately wrong final expected-counter discriminator: old savedstate passed despite producer1. Repaired515f43 stores failed aggregate while retaining execution_status passed; positive52d83d stores both passed. Composite result is embedded in final receipt. Repaired negative was before removal of the now-unneeded separate composite success artifact; no semantic path change to negative branch.

F3 main464838 copied actual historical ack bytes and changed only source reply14 fence[0]1->2 plus frame length; partial Python join accepts, full Lean replay afb13f rejects exactly source14. This is offline altered evidence, not another native execution. Keep full-byte replay conjunction rather than duplicate complete transition semantics in the join checker. No whole historical-state identity proof is claimed.

Oracle18 single submission ac1f99 at17:05:27.717014Z, session mir-w4-credit-attempt / wrapper27409. Frozen50files382350bytes, question e4e1fa986aa1e199a56cb3377d51d11a0a46298ef934e97ad514519b33a1de16; manifest0636782a49ac5b55c48c44f8e9a0f6fc784f476df1dd953433ef0c0f2f0b8dfc. Full direct profile/new completion-credit and17accounting repair review; not proof/acceptance. First state check no earlier17:08:27Z; no arbitrary timeout/retry. Narrowed pre-submission redundant build metadata retained separately; packet explicitly omits full artifacts/dependencies.

Mandatory321full483610; SYS4 prefix1-1020 and suffix12170-17803 read, remaining1021-12169 UNREAD, no Rust edit. Quota c81b1c at17:04:38Z, fresh17:04:18 event55%used/45%remaining; next check>=18:04:38Z, approximate safe30% stop. Status mirrors/plan updated as snapshot maintenance, no new roadmap. Latest docs1764validation predates this metadata update; fresh check pending. Source/Rust/Core/QUIC/namespace/custody integration and W4 completion still open. No new commit/push, no subagents.

Forward 2026-09-14T17:17:45.428094+00:00: private budgetnative102 buildac06d0 uses frozen reviewed-pending OwnerBudgetWorker/proof; binary60ffb79261cbf5d7b62906325c1ac2e4071b5485eaa4835064361613c51cd112, actual audit9b2d87 closes12554owned with standardaxioms. Two actual processes each514inputs f9737b/2c1960 close0EOF: no last-slot grant, prior granted work survives unrelated refused initialize and produces10. Full1028input/reply framedbytes and exactargv/model/final exhausted-idle replay72b828 passes (logSHAa7a55cb0d81420865b888e4eb0a4d7aeabe556f1504d364f72d3d71be155b9e7). Not source/network/E2E or coupled source-owner budget evidence. Build/audit owner-budget selectors and shared direct replay explicit-attempt consumer were added AFTER18freeze; keep narrow review pending, old binaries and receipts unchanged. Audit helper version binding consequently requires a new source/owner attempt under this helper before new full C/A replay; previous explicitly checked9103c0 result remains historical success. No current green is inferred from a new unexecuted command. SYS4 prefix1-2880 now read/registered, remainder2881-12169 unread; no Rust change. Oracle18d3bb54 normalpending, no retransmission.


### Forward evidence — Oracle18 findings and native close (2026-09-14T17:37:36.340908+00:00)

Oracle18 completed32dc64 exactquestion, contentSHA5e13ec0e9354b40a9ab152f9432c9304b3ae26184e5615593e2dbc029892155a, wrapperf0f7060. Full4015ac/3cab5e; in-memory Python probes only, no independent Lean/native execution. No general profile/credit/progress counterexample; findings concern remaining consumers/EOF/current-run accounting. Direct replay explicit audit consumer already migrated postfreeze;56f112 fullentry rejects omitted2/missing1/failed1, positive3d21c2 passes actual direct capture.

ActiveEOF ACTUAL984dda initializes/reserves thencloses0 with outstanding reservation. This is outside the semantic credit transition. Worker now checks held before returning onEOF, emits nonzero failure without implicit abandon/compute/reset. New102native9cebd2 binaryb6ccced29d262ec8da76df28592028f4d4c0ba356ebefad966c551cdc620ecef; audit55e409. b014c5 explicitly expectedincomplete childexit1. Positive final-slot/refusedinitialize/exactreconfirm/explicitabandon3cases1542inputs close0 and all bytes/argv/terminalstates replay3d21c2. Combined3f0e09 failed because its replay command used an invalid placeholder path; native receipt records successful3children, correct path was subsequently selected from BUILD.json without rerunning native. Old60ffb7 evidence frozenpre-active-eof-budget-evidence, not overwritten as newbehavior.

Main nativeold95cap65startup exits64 (6f7076), but offline copying old historical capture and changing onlyowner0argvcapacity8->65 passed priorwholebyte replay7974f7. Newreplay explicitlymodels actual startup p/a/cap limits, rejects5f45c0. No general Lean theorem failed. Legacybound mode silentlyignored explicit missing --owner-audit in fullcounterdf8b9e; unsupportedexplicitselection nowfails274e0e. Samefullproducer missingnewsourceaudit preservesoldconventionalresult df8b9e: not a current success or demonstrated downstreambypass. New exclusive replay-attempt directory created before inputvalidation; requiredrun/cases/audit/mode/stem selected and checked, failednew/missingnew/wronghistory f4b4e1 refuse, oldhistory preserved. Fullreallegacypositive3b386a and requiredselection771abf pass; different required audit rejected. Stable local custody and serialized use remain TCB, noatomic/durable/signature claim. Direct/native conventional receipt helpers have not all acquired this newaggregate interface; don't claim universal current-run API closure.

Same source105 + EOFguarded owner102 ACTUAL C/A2four-process continuations c74d3a pass; exactboth397source inputs plus allownerinputs/output/finalimages 0c67e8, selectedrequiredrun102c43 pass. Resultroot replay-attempts/budget_normal_1/RESULT.json; logSHA0cfd256371d8526be6601a746e06ed06a4adc236fb8da7c57c0b0d9985ea1c9f. Newsourceaudit5a9d2d under currenthelper pinned. These are actual privatepipes, not generalized ownerquota admission or authenticnetworkproof.

RoutedOwner general conditionaljoin1c2abe derives a generated dispatch's ownerprofileguard and fundedreserve/computation from sourceStateFits+actualenter, exact STORED responsecontext and independent owneradmission/currentness/freshness/room/credits. It does not manufacture that physical binding or resource premise. Source-enter before a depletedowner remains OPEN; idle abandonment remains insufficient terminalevidence. No newauthkey, cancellationpolicy or Q18decision. SYS4 prefix1-4230 andsuffix12170-end read; middle4231-12169stillunread, noRustedits. Read-only integrationinventoryc562f6 finds62existingmodulesidentical/45new, zeroexistingmodulechanges; noblindcopy yet.

Docs19cb44 validates1764reports beforethisforwardappend. A --help call actually ran the samevalidator20ce45; recorded, not counted as newsemantic evidence. Same11own dirtydocs, no newcommit/push, noformalphase/THM/OBL/W4completion. Review18repairs/IOdelta and conditionaljoin need bounded follow-upreview before promotedintegration. No liveOracle after18; next19notyetsubmitted.


Forward 2026-09-14T18:02:15.637249+00:00: Oracle19 COMPLETE4930ce, wrapperexit0; exact answer fulla58a94/f3aa9f. No theorem counter found, no Oracle Lean/native execution. Direct lexical normalizer still accepted+8/fullwidth/float metadata; actualnative b491aa rejects+8/fullwidth64, accepts0_8, so new ASCII private-profile gate is narrower than native parser. Full current entry6f405f rejects4mutants before generation; correct8 reaches immutable historical artifact guard, no new complete replay claim. Unsupported native--audit rejects2 CLI cases before work; originalOracle finding was sentinel dispatch, not main native reproduction. Framing reader full943fc9 and actual clean/truncated-header/truncated-payload6f405f confirm0/1/1 exits, no invented owner terminal event. Bound unique-run reader still needs saved reusable outer workflow integration; main's prior actual selected-run calls remain evidence.

New same-goal resource consumer:6049b2 ACTUAL ordinarysource9publications then owner1credit lets sourceenter0 but reserve16; no ticket-specific terminal association, normal processEOF is not sourcecompletion. OwnerCreditCustody a14df4/70a1fe prove exact all-command response credit tracking, declarative claim/check equivalence, phase-local restrictions and all local mutator invariants (standardaxioms only). FIRSTac64a5/SECOND228f3f failed proofs retained and never used downstream. Original native proxy allowed cancel after enter (8d8a01) and duplicate fresh constructor (8406bd); phase/copy/serialization/fresh-registry guards now checked in actual processes042794. Native1credit guard leaves original waiting source unentered,2credit produces actual10 and sourcefinish parks exact result. This does not include sourcearrival/subsequentpublication or fullsource completion. The local entered notification still requires real source-event binding; no arbitrary API caller/raw-pipe custody theorem. No new auth/priority/cancel/Q18 policy is adopted. Oracle19 independently identifies drive()'s extra activefreeze probe and laterallownerfreeze/install costs, which the bare2credit theorem does not pay.

Same11own dirtydocs, no newcommit/push or Canon/THM/OBL/phase status. SYS4 fullreadprefix now1-6120, suffix12170-end; middle still unread. Heavyprocesses serial4GiB, priorfree42GiB/RAM10GiB. Quota45% last17:04Z; next>=18:04:38Z. All newproof/consumer code remains external pending material review and reproducible integration.
