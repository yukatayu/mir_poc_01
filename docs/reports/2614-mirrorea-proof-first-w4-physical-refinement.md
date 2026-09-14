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

## Skipped validations and reasons

Actual W4 Rust/process/network tests, remaining I3-3 regression targets and final docs/Git checks not yet run: implementation gate is still under investigation. No W4 completion claim. Broad global corpus reading remains incomplete; indexes/grep and truncated output are not full reading.

## Commit / push status

No W4 commit yet. Baseline clean5d7c13a8; only own edits will be staged; ordinary commit --no-gpg-sign and normal push/parity at reviewable integration checkpoints. No force/reset/clean.

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
