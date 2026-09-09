# Report 2612 — Mirrorea proof-first W2 contract boundary

- Date: 2026-09-10 01:08 JST
- Author: sole main Codex; no sub-agents
- Status: ongoing nonproduction LAB research, no acceptance

## Objective

Connect reusable ordinary computation to a typed local-contract/resource boundary
without erasing assumptions, proof identity, current ownership or authority.
This report accumulates W2; a small resource theorem does not close W2 or alpha.

## Scope and assumptions

Active goal W2-local-contract-resource, PL1 S1/S2 theory/proof. Initial consumer:
arithmetic-produced positive length followed by exclusive allocation, split,
move and release. U05/07/09/14/17, TY01–08, ID07, SL08, PT01/02/04/05/11/12,
SC03/14/17/19/24 and Q03/Q19 are motivations, not blanket-adopted proposals.
Candidate A retains dynamically checked opaque handles; smallest viable B adds
affine caller typing while retaining currentness/auth checks. Neither lets a
copied handle create another resource. Mathematical regions are not malloc or a
Core primitive. Public syntax/ABI, production, restore and final alpha are outside
this candidate's claims.

## Start state / dirty state

HEAD3408c4cf8664d752699361d5d0a48d05781e19d3, main, already pushed. Own W1 reading,
status corrections and unreviewed AbortFlow/AddressFlow mirrors are dirty. No
preexisting user changes were overwritten. Handoff originals unchanged. Small
scratch work remains outside repo on the measured root filesystem; configured
external mount is absent, and no heavy build or cleanup was performed.

## Documents consulted

Exact full/range/hash ledger: docs/proof-first/READ_LEDGER.json. Relevant handoff
WORKPLAN/W2 exit, system requirements, F0.1 resource theory and tests, F0.2 local
certificates/source/engine and all tests; Canon type/effect/authority boundaries;
reviewed W1 companions. Mandatory whole-corpus reading is incomplete. No new
whole-project roadmap is adopted.

## Actions taken

Selected one direct nonproduction consumer and wrote independent Region separation,
State well-formedness, operational allocation/erase/move/split, declarative Allowed
and a separate Boolean checker. Proved checker equivalence, actual checked execution
preservation, finite-run preservation, monotone issuance and no future current use
of an already consumed handle. Raw mutators are deliberately distinguishable from
checked execution; a non-interior raw split actually violates the invariant.
W1's already reviewed required dependencies permit this independent research;
its later pending abort/alias/source cut is not a premise. The same Oracle job
continues and its eventual readiness advice will be checked, not presumed.

## Files changed

CURRENT_GOAL and current snapshots/correspondence; this report;
the W2 check records; ten candidate Lean sources and one companion under
samples/lean/foundations. Mirroring preserves their unreviewed LAB status.

- `docs/project-status.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/proof-first/CURRENT_GOAL.md`
- `docs/proof-first/W2_RESOURCE_CHECK.json`
- `docs/proof-first/W2_CONTRACT_CHECK.json`
- `docs/proof-first/W2_INTEGRATED_CHECK.json`
- `docs/proof-first/W2_MIRROR_CHECK.json`
- `docs/proof-first/W2_EXTENSION_CHECK.json`
- `samples/lean/foundations/MirroreaProofFirstResourceBoundary.lean`
- `samples/lean/foundations/MirroreaProofFirstLocalContract.lean`
- `samples/lean/foundations/MirroreaProofFirstContractExport.lean`
- `samples/lean/foundations/MirroreaProofFirstPureFunctions.lean`
- `samples/lean/foundations/MirroreaProofFirstFunctionContractBridge.lean`
- `samples/lean/foundations/MirroreaProofFirstOwnerAssignment.lean`
- `samples/lean/foundations/MirroreaProofFirstProfileGuarantees.lean`
- `samples/lean/foundations/MirroreaProofFirstHandleValues.lean`
- `samples/lean/foundations/MirroreaProofFirstPureHandleFunctions.lean`
- `samples/lean/foundations/MirroreaProofFirstContracts.md`
- `samples/README.md`, `samples/lean/README.md`, `scripts/README.md`
- `plan/proof-first-foundation-correspondence.md`

## Commands run

lean --trust=0 on the small scratch ResourceBoundary.lean; exact argv, toolchain,
source hash, stdout and exit code are in W2_RESOURCE_CHECK.json. Commands during
development exposed inference/tactic errors and sorryAx in failed elaborations;
those failed runs are not proof evidence and no sorry/admit source was introduced.

## Evidence / outputs / test results

Latest coherent five-source mirror check exits0 for every file, Lean4.29.1, with
only standard propext, Quot.sound and Classical.choice where printed; some
lemmas are axiom-free. W2_MIRROR_CHECK records repo hashes and exact commands. General theorems
use inductive/structural reasoning; fixed decide examples are only controls.
Positive 12-unit region splits at5 and transfers the right part to another holder.
Zero/endpoint split, unauthorized allocation, reused old handle and receiver use
without its separate policy grant reject. The raw endpoint split yields an empty
region and fails WF; selecting a historical state makes an old handle current,
showing why the live-head/restore obligation is not solved here.

## What changed in understanding

State invariant need not be stored as a postcondition field or asserted after
mutation. Independent transition proofs can cover the exact resource operations.
Finite live support follows from every live ID lying below the monotone counter;
physical representation and capacity remain separate. Ownership transfer can be
accepted without granting the receiver execution authority. This is still no
proof-export protocol or ordinary-source elaborator.

## Open questions

Actual authenticated current value/code/contract/theory/assumption binding;
arbitrary heterogeneous theories beyond the two checked evidence profiles;
first-class current module handles, ordinary parent writes and explicit other-owner
snapshots; existing-source typing/refinement and finite-word/resource bounds.
Pure higher-order functions, finite iteration, certificate-relative completeness
and arithmetic-to-function correspondence now have scoped candidate proofs.
Policy authenticity, restored head, all actual constructor/migration/decoder
entries, physical atomicity and failure/secret disclosure remain open.

## Suggested next prompt

Continue independent scoped proof/counterexample research and required reading.
Retain both running Oracle jobs: original W1 and revised W2 integrated review.
The first W2 attempt failed before submission on the shared profile lock; its
actual-error retry uses the strict dedicated-tab attach route recorded below.
Neither invocation is a collected review. Do not duplicate or cancel either job.
A browser-display clarification is pending after read-only CDP diagnostics; it
does not authorize resetting jobs and does not block independent local research.

## Plan update status

Updated task-local correspondence only; no new global roadmap, Plan250 resume or
closed-plan rewrite.

## Documentation.md update status

Updated active goal and unreviewed resource-candidate status.

## docs/project-status.md update status

更新済み: task-local W2 active goal and unreviewed kernel evidence synchronized; official lifecycle unchanged.

## progress.md update status

Updated current task snapshot and actual-time recent log.

## tasks.md update status

Rewritten as current snapshot with one W2 goal; W1 pending review remains explicit.

## samples_progress.md update status

Updated active task pointer and W2 mechanization row/command/remaining blocker.
Mirrored W2 is evidence only, not a runnable alpha root.

## Reviewer findings and follow-up

W2 candidate unreviewed; mirrorea-w2-resource-contract was launched once and
failed before submission (profile lock held by the W1 controller, exit1).
The separate mirrorea-source-boundary Oracle is retained. Main inspected
W1 metadata: promptSubmitted=false, so invocation is not verified submission.
Read-only CDP connected but did not return frame/DOM commands within a bounded
diagnostic window; that diagnostic timeout is not an Oracle terminal failure.
No job was cancelled/retried, and a display clarification was requested. All work is by main;
Oracle advice is not independent kernel execution or a signed reviewer.

## Skipped validations and reasons

No production source changed; no Rust/full-workspace/network/recovery rerun for
this mathematical draft. Source bridge and alpha requirements remain unverified.
Earlier docs validation38186 passed before this mirror; the new synchronized
cut now passed docs14161 (f8cefb),1762 reports; exact source hashes and
focused diff check also passed3d2166. The validator checks scaffold/wording,
not the missing source/network/alpha semantics.

## Commit / push status

Prepared as an unreviewed LAB research checkpoint based on HEAD3408c4c.
Commit/push results are recorded forward after the command completes. No
force/reset/clean or public publication action.

## Sub-agent session close status

No sub-agents used. User request continues; no final completion notification.

Docs validation initially failed because five required section headings had wrong spelling/case; corrected to the repository template, rerun pending. No validation success inferred from that failure.

Continuation — actual local-contract/export kernel: three coherent scratch modules
ResourceBoundary, LocalContract and ContractExport now pass Lean trust0; exact
source hashes/commands/axiom output are retained in W2_CONTRACT_CHECK.json.
LocalContract independently defines integer term Denotes, an evaluator, a
nonnegative derivation language and certificate inference. Soundness and relative
completeness concern that explicit derivation language, not all true arithmetic.
Inputs' hypotheses are separately checked at invocation. Square/add/product rules
are general; fixed examples are controls only.

ContractExport has exact expected-binding comparison (profile/version, contract
version, instance/generation/principal/request, actual term/argument list/assumptions),
independent scope checks and Common judgment, actual result equality and separate
profile checks. Checked-value evidence covers this positive instance; symbolic
nonnegativity covers its theorem under explicit hypotheses. Both produce the same
positive-length contract and use the same resource allocator. General soundness,
respective completeness, exact Int-to-Nat value preservation, lawful allocation
and inability to grant policy authority passed the kernel. Actual controls allocate
10 from square(-3)+1 and4 from a positive dynamic input, and reject result swaps,
changed generations/requests/theory versions, missing arguments and missing authority.

These are two evidence profiles, not proof that arbitrary heterogeneous user theories
are already integrated. The symbolic path also evaluates the actual expression;
no optimization or resource bound is claimed. Expected context and policy are
trusted current inputs; structural equality does not authenticate them. No combined
CurrentUse caller/code/lifetime binding, finite-word refinement, parser, higher-order
or recursion/module-handle floor, restore head or production implementation is proved.
All three modules remain unreviewed candidates. The earlier source-boundary Oracle
is retained unchanged; this new cut has not been submitted or implicitly reviewed.

The second docs validation failed the exact project-status declaration convention; added the required 更新済み declaration and explicit Files changed entry. The substantive kernel checks remain separate. Resource schedule extension now proves preservation and nonresurrection with independently supplied policy at every step, with coherent three-module PASS; no policy authenticity or concurrency is inferred. Region size and positive-result exports have no information-flow/release theorem: a secret-dependent size cannot be treated as public because its positivity is proven.

Docs validator rerun38186 completed exit0: Documentation scaffold complete,1762 numbered reports. Prior report-format failures remain recorded.

Continuation — pure functions and actual contract translation: PureFunctions now
has an independent Typed/infer equivalence, explicit closure environment typing,
general successful evaluator type preservation, and a distinct Executes relation.
Fuel execution is sound and relatively complete for finite declarative executions;
no normalization theorem for all typed programs or global fixed fuel is claimed.
A general theorem covers every finite Nat iteration count and mathematical Int
initial/delta, yielding initial+delta*n. Higher-order twice(increment)(40)=42 and
iterate7(+3)(0)=21 are additional fixed controls, not substitutes for those proofs.
FunctionContractBridge structurally translates the scoped arithmetic contract
term into this same evaluator, preserving types and actual declarative results.
A reusable unary lambda works for arbitrary scoped unary arithmetic terms; accepted
exports with the same arguments yield their actual accepted result under sufficient
fuel. Square duplication is restricted to pure expressions, not effect continuation
or mutable reads.

Fresh coherent five-source check at w2-integrated-0a8cc8rf passed (exec9030aa),
recorded in W2_INTEGRATED_CHECK.json; standard propext/Quot.sound/Classical.choice
only where audited. All are unreviewed candidate sources, not production.
The prepared-only resource Oracle packet was retained; the selected successor
oracle-resource-export-integrated (question SHA0f3f05b6f13e4c07eda2f116985fa44cbe181763898034c6ba350e43f1444c15)
adds these actual proofs and remains unsent while the existing source-boundary
job runs. Do not send both preparations or treat preparation as review.

Existing computational_core and full_system_v1 typed IR have named Call forms,
not this closure/type-arrow carrier. Their finite-word arithmetic is not the
mathematical Int semantics above. This is source inventory, not a new execution
result. No production parser change or purported source higher-order support is
inferred. Ordinary-source current module handles, explicit other-owner snapshots,
IFC/failure semantics and conditional implementation gates remain open.

### W2 resource semantic coverage continuation

The main added general exact interval-cover/disjointness theorems for split,
geometry preservation for move, a frame theorem for every old nonconsumed resource,
and actual returned-handle currentness for successful execution. These supplement
WF/nonresurrection: preservation alone would not establish an operation's useful
result. The coherent five-file cut in `W2_INTEGRATED_CHECK.json` passed Lean
4.29.1 `--trust=0` (all five exit 0; standard logical axioms only). This newer
scratch cut is unreviewed; the prepared but unsent Oracle packet must be refreshed
before submission. No previous sent packet was changed.

Full historical Plan09/31/33–36/38/42–47/49 reading preserved the separation of
helper report assembly, same-session behavior, accepted detach, durable migration,
and actual Mir computation. Old scoped alpha closeouts do not satisfy this task's
ordinary-source/dynamic/private-observation integration target. Read hashes/ranges
are in READ_LEDGER; mandatory full corpus reading is still incomplete.


W2 Oracle invocation 2026-09-09T16:46:26.847587+00:00: sole main launched distinct read-only
`mirrorea-w2-resource-contract` once (exec session19233). Packet question SHA-256
`a12dea8199a65171b07052f79a872b8bb50d760b329aeef7a933440fdc8bf0cd`; frozen five-file
cut and CHECK/HASHES in `/tmp/mirrorea-proof-first-20260909-gn29zka5/oracle-w2-resource`. Main read installed browser
implementation: manual-profile runs allocate isolated tabs, with shared-profile
launch/submit locks and no fallback to an existing tab when reusing Chrome.
Therefore this independent W2 review need not wait for W1's terminal answer;
this supersedes the earlier conservative serialized-launch plan. W1 job54675 is
retained unchanged, without cancellation, resubmission or a new deadline. No
parallel writer/sub-agent was started. Earlier prepared resource packets were
never submitted. CLI invocation is not yet evidence of browser prompt submission
or model completion. Poll both jobs only after each LAST_CHECK_UTC +180 seconds,
print new log lines only, retain each original job until terminal outcome.


Mirror continuation: the five W2 candidate files are now stored under
samples/lean/foundations, with one companion and the existing fresh-copy command.
Only imports changed from the original frozen cut. Fresh W2_MIRROR_CHECK records
all5 exit0 in w2-mirror-oxnf2g59 (exec10458/c1c31e). No review/adoption inferred.
Two deliberately invalid reference mutations were rejected: returning the old
consumed handle after move fails returned-currentness (and useful controls), and
leaving a point gap between split regions fails exact cover and preservation.
The mutations remain outside repo in w2-resource-mutants-rcslwg4n; checks are
countervalidation, not proof of a production implementation. The original Oracle
packet and good source definitions were unchanged by these controls.


Oracle terminal outcome: W2 invocation19233 ended exit1 at
2026-09-09T16:51:27.344Z, browser-automation error: profile lock held by W1
controller1852666 after the installed tool's300s lock wait. This was not a model
answer or a caller-added deadline. No prompt submission or review occurred, no
retry was sent, and W1 job54675 remains intact. Browser isolation support alone
did not ensure launch-lock availability; the attempted concurrent read-only
review was a failed operation, not successful parallel review. Main retains the
packet, reports the error, and continues independent local research. Any consumer
requiring W2 source review remains unaccepted. W2_MIRROR_CHECK records the failure
separately from successful kernel checks and deliberate mutant rejections.

### Module/current-use composition extension

Sixth candidate `MirroreaProofFirstModuleContractBoundary.lean` composes existing
CurrentUse with exact module/operation stamps, actual descriptor code/contract,
arity, integer-tagged arguments and common export. General call soundness and
relative completeness for both profiles, no-authority rejection, actual execution
and changed-context/stamp rejection pass Lean4.29.1 trust0. Fresh eight-file
dependency cut passes; exact hashes/axioms are appended to W2_MIRROR_CHECK.
It is later than the failed five-file Oracle packet and remains UNREVIEWED.
Trusted registry/world, install/update/retire, current-head authentication, source
module handles and physical commit/recovery remain open. No production change.
Docs validator exec58490 passed (375d14), but concurrent later documentation edits
mean it is not validation of this final synchronized cut.

Documentation validation21817 failed only on progress header freshness after its
new timestamped log. Header corrected using the current Asia/Tokyo time; this
failed run is not a success. A synchronized-cut rerun follows.

Docs14161 passed after the freshness fix (f8cefb); six W2 source hashes
match successful kernel records. Main focused diff review preserves the
unreviewed/TCB/source-network boundaries; no Oracle result is substituted.
Browser-level CDP responds and reports the owned ChatGPT page attached, while
page-level reads remain unresolved. No job reset, cancellation or repeat submit.

Intermediate checkpoint0047e8f8a0fa527ceffa9bd21f77bf65b3f9e9d9 committed
with --no-gpg-sign (3f98d9), pushed normally (15075/81926b), and exact remote
parity plus clean tree confirmed (47723/9d0bd8). This saves unreviewed research;
W2, the alpha target and the user request remain open. Main continues the
owner-assignment/snapshot dependency in small scratch, using only reviewed
FallibleFlow/ProducerFlow/Passive premises.

Actual lock-error recovery: browser-level CDP and a newly created disposable blank
page respond (48b9a9), while the existing W1 page does not. Installed CLI/source
inspection identified its strict dedicated-tab attach mode. Task-only private
DevTools attach metadata enables discovery without changing the original profile,
copying cookies/keys, resetting W1, or launching another Chrome. A revised W2
packet was frozen and all12 Lean dependencies passed14551/d73aae. One retry was
launched as mirrorea-w2-integrated-review (exec4661), question hash
5dd28908150125d782d8af3a68c711d9df76790848c9bf091b185d15d8f5d6ff.
This follows an actual terminal pre-submission error, not a latency deadline.
The original W1 remains retained. No submission/answer/review is inferred yet.
The later OwnerAssignment source compares owner-local live inputs and explicit
frozen values; it proves scoped checker/elaborator correspondence and exposes
value-preserving literalization as insufficient for two-run confidentiality.
Capture provenance/authority/labels, current references and actual source/queue
integration remain open. Scratch notes/checks are frozen into this review packet.

Later independent scratch ProfileGuarantees.lean: accepted symbolic evidence
implies UniformPositive for every valuation satisfying the explicit assumptions;
a checked positive invocation need not have that property. Both the general
implication and a checked-value counterexample pass Lean (ae2e10/6ae867), using
only standard propext/Quot.sound. This makes the guarantee quantification explicit,
not merely two proof-checking methods. It remains later than the frozen Oracle
packet and cannot inherit that packet's future review. It neither establishes an
arbitrary theory loader nor closes first-class source module/capture obligations.


Later HandleValues scratch adds a finite single-sort binding language for module
interface values. Scoped/check equivalence, independent Evaluates/evaluator
correspondence, termination for scoped bindings, and no introduction of references
outside supplied environment/source constants pass Lean4.29.1 trust0. Calling the
result retains its module/operation stamps and the caller's separate subject,
request and arguments, then uses the actual current contract checker. General
call soundness, no-issued-authority rejection and stale-module rejection pass.
The positive binding passes a handle and returns42; retired/stale handles and
empty authority reject. A fresh caller proof cannot silently replace a carried
stale module stamp. Two deliberate mutations (discard carried stamp / bypass
call boundary) fail both their general obligations and concrete rejection controls.
Exact source/hash/axioms and mutant logs are in module-contract/HANDLE_CHECK.json
under the retained task work root (valid a5a671; mutants36551/4a1e0e, inspected559a43).
An earlier termination-index elaboration failed5d2a74, then was corrected; it is
not counted as proof. No authored sorry/admit or Mir-specific axiom was added.
This is UNREVIEWED and later than the active W2 packet. It is a binding fragment,
not full higher-order module composition, parser support, secret-dependent handle
selection, source lifecycle construction or real communication. Registry/world
provenance and physical invocation remain separate unmet obligations. No W2 or
alpha closure, production change, new sample root or global roadmap is inferred.
Plans89–96 are now fully read and hash-ledgered; their indexes do not credit
referenced source/report files. Existing samples_progress.md commands remain
accurate for the six mirrored candidates; this later scratch is not promoted
into that active reproducible sample set.


The later PureHandleFunctions scratch extends the pure closure/iteration candidate
with a distinct interface-handle value/type at arbitrary finite universe size.
Independent Typed/infer correspondence, successful evaluator type preservation,
and finite declarative execution/evaluator soundness and relative completeness
pass again for this extended language. A higher-order twice(identity) function
carries every supplied interface unchanged (general theorem), and invocation of
that returned handle equals the current contract boundary call. General call
soundness, no-authority and stale-module rejection hold; concrete controls return42,
reject stale/retired modules and handle arithmetic, and iterate the reference value.
Exact source2ebd3ca2e2ec81e3ea02db283466fd774ea0b27582d391d0b40f6a8fcf25dcc5
and trust0/axioms are in module-contract/PURE_HANDLE_CHECK.json (70648/35a934).
Initial draft type-application/binder errors were corrected before successful
kernel checking; no failed draft is credited. This separate research extension
has not replaced the committed PureFunctions or its frozen review packet. It
makes no existing-source conservativity/refinement, closure shipping, handle IFC,
linear resource duplication, authority issuance or network/lifecycle claim.
The two binding models are exploratory source-boundary evidence, not two new
production interpreters or an adopted public grammar.
ProfileGuarantees additionally proves that accepted symbolic contracts have a
satisfying valuation supplied by their actual accepted invocation; their uniform
conditional guarantee is not vacuous through contradictory assumptions. Updated
PROFILE_CHECK retains the earlier cut and records new PASSbc49f2. All these later
results remain unreviewed and outside the active Oracle packet.


Additional readonly browser diagnosis: owned Chrome processes are present in
sleeping/event-wait states (6a32bf), not proof of failure. X11 owned-window lookup
found no window and the Chrome environment did not expose DISPLAY; no unrelated
window or desktop was captured. A dedicated diagnostic CDP session targeting only
the revised W2 page received actual Internal error on Page.captureScreenshot and
was detached (2f56ee). Browser-level target/window reads still return an attached
ChatGPT page and normal1300x743 window (97480b). These diagnostics do not show a
login state, do not classify the Oracle job as terminal, and do not authorize
reloading, cancellation or duplicate submission. No screenshot/private contents
were saved or sent. Main keeps both jobs and the pending user display clarification.


2026-09-10 03:19 JST extension mirror: OwnerAssignment, ProfileGuarantees, HandleValues and
PureHandleFunctions now join the existing foundation root as unreviewed research;
only the last source's HandleValues import module name changed. A fresh isolated
fifteen-module dependency build passed every --trust=0 command (97457/a38973),
with exact hashes/axioms in W2_EXTENSION_CHECK.json. The original active Oracle
packet remains frozen and excludes the three later modules. The reproduction
command and samples/README, scripts/README, samples_progress, Documentation,
project-status, progress and tasks snapshots are synchronized. No new sample
root, runtime wrapper, Canon promotion, production change or milestone closure.

Documentation validator13830 completed exit0 (907a64),1762 numbered reports.
Latest source-comment correction was rechecked65597/45a9a3; all15 source hashes
match the recorded kernel cut. Main full new-source/focused-diff inspection
preserves the separate typing/currentness/authority and unreviewed boundaries.
Companion table/wording fixes do not promote the candidate. No new Rust or network
run is claimed. A best-effort Discord progress notification sent14699/2d122d;
this user request continues, with both Oracle jobs retained.

Intermediate checkpoint b35744b73a110e031376f8439c9578f4c27f57e8 committed
--no-gpg-sign (e46fb3),19files1742+/156−. Normal push87346/d902d7 succeeded;
ls-remote69459/2f31e2 equals HEAD and tree was clean7f6fdd. This records only
unreviewed LAB candidates; the user task continues with full reading/source
boundary research and retained Oracle jobs. No acceptance or alpha completion.


2026-09-10 03:40 JST capture-boundary extension: direct consumer is OwnerAssignment's explicit
frozen input (U07/U17, TY/SL/observation obligations within the same W2 goal).
Main defined independent capture/sequence judgments and Boolean checkers, actual
per-slot evaluation, and one capture followed by ordinary assignment. The
per-slot valuations do not require a global shared snapshot. General captured
typing/low equivalence and failure-inclusive sequence frame/type/two-run projected
outcome preservation pass Lean4.29.1 trust0 (87692c; final9ec404, mirror e03909).
Standard propext/Quot.sound only for these printed theorems; no Mir axiom.
An initial proof draft left one hidden-result branch unsolved (e047cc); explicit
attempt_key resolved it (8ee30a). Failed elaboration is not credited.

The decisive negative case is secret inputs (i64::MAX-1)/i64::MAX under checked signed-64-bit
arithmetic: input+1 succeeds/fails, causing the same public constant-write body
to write/fail although it never reads the capture. Their public projections
differ. Requiring capture-label <= target-label keeps that completion dependency.
The independent sequence checker accepts a high-target positive case and rejects
the unsafe low-target case. Mutations dropping capture-PC or completion checks
fail both correspondence and concrete rejection controls (9ec404). Fixed decide
cases are counter/positive controls, not the general theorems.

Candidate A retains explicit capture metadata/control/completion flow; smallest
alternative B would delay or separate the operation so capture failure cannot
control its low result, requiring an explicit different source dependency. No
implicit snapshot or Q18 policy was selected. Actual M7 CheckedEvaluationParameter
(name/type/source_ref) and ParameterRead(name/span), and M8 string-to-i64 argument
evaluation were inspected (21934e/ca16ee); these fields do not carry the modeled
labels/provenance. No actual deployed leak or source refinement is inferred.
Capture authentication, authority, event-selection/presence/timing, physical
read valuations and source elaboration remain implementation/proof obligations.

The extension is appended to the existing OwnerAssignment source, keeping ten
W2 files and one companion. W2_EXTENSION_CHECK preserves the original Owner cut
and current isolated rebuild plus scratch mutation records. The original frozen
Oracle packet is unchanged; this later delta has no review. Existing two Oracle
jobs still report running/submission false (ca16ee); original exec polling
586125/f94d58 returned no output and retained the sessions. No resend/cancellation.
Plan correspondence, progress/current goal and sample evidence row updated;
Documentation.md/docs/project-status.md/tasks.md 更新不要 for this sub-boundary:
the same W2 active goal, blockers and ten-source unreviewed status remain accurate.
No new reproduction command/root, production edit or Rust/network/recovery run.
Required Plans111–122 reading is full/hash-ledgered; corpus remains incomplete.
This is ongoing work; no W2/alpha completion, commit or push yet for this delta.

Validation58092/4f0922 rejected a stale progress header after the new log line.
The header was updated from actual local time before retry; the failed command
is not credited as passing. Capture controls now use i64::MAX-1 / i64::MAX,
with all literals in range and checked signed63 arithmetic. Current mirror hash
93e6549ece5ac234bdfe6852561a4c590ab3a9eafa6f301f5e15954d234e5f74 and
two concrete checker mutation rejections pass8704a6; general claims unchanged.


2026-09-10 03:48 JST descriptor/catalog sub-boundary: general descriptor uniqueness under one
fixed use/proof and old-proof rejection under descriptor replacement pass Lean
(0b5e9e/194f6a). A concrete same-value changed arithmetic tree rejects the old
envelope but accepts a rebuilt envelope under the same nominal-ID authority
context and supplied substituted registry. This is a countermodel to assuming
registry integrity from proof/current-use validity; no deployed exploit is inferred.

Candidate A: finite-capacity immutable descriptor catalog, no deletion/overwrite
in its insertion transition language. Independent InsertAllowed/execution-existence,
WF and old-binding preservation hold for arbitrary finite insertion sequences.
A selected catalogCall retains Successful and exact current operation-code catalog
lookup, agrees with raw call for registered descriptors, and rejects a different
descriptor even with a fresh proof. Capacity acceptance/rejection, actual42,
replacement rejection and two invalid mutants (overwrite/catalog-check bypass)
were checked (e0efe4). The overwrite mutant also reduces its concrete rejection
control to False; the bypass mutant's rejection control is false. Invalid
elaborator sorryAx outputs are never credited. Standard propext/Quot.sound/
Classical.choice where printed; no Mir-specific axiom.

Smallest alternative B would bind actual descriptor contents into the authority
context. That affects the authority boundary and is not adopted. Candidate A
still requires authenticated catalog/head acquisition, installer authorization,
World record/code/revision transitions, storage and recovery. Raw call remains
explicitly weaker; HandleValues/PureHandleFunctions still invoke raw call. No
all-entry protection, production change or global catalog semantics is claimed.

Main mirrored the extension into the existing ModuleContractBoundary source and
rechecked it plus both dependent handle modules in an isolated copy (c4c959),
all pass. Fifteen current records in W2_EXTENSION_CHECK preserve prior cuts and
new dependency results. The frozen Oracle packet is unchanged and this delta is
unreviewed. Full extension self-review00c25b checked the selected-entry limitation.
Correspondence, current goal, companion, progress and sample evidence row updated;
Documentation wording now distinguishes original fresh dependencies from later
cone rebuilds. docs/project-status.md/tasks.md 更新不要: same active goal/blockers,
no milestone close or roadmap change. No Rust/network/recovery run; no new root.
This sub-boundary continues the user task and does not close W2 or alpha.

Docs retry38408 passed (badbba),1762 numbered reports. Final current fifteen
source hashes/axiom logs and both pairs of rejected mutations were checked
(f154d2); diff check passed. Main focused doc diff review866a03 retained all
source/current-head/selected-entry limitations. No independent Oracle result is
available; latest7c7f15 retains both running/submission-false jobs. Plans129/130
are fully read/hash-ledgered; mandatory corpus remains incomplete. Own twelve-file
checkpoint is ready to commit/push; this is saved research, not task acceptance.

Checkpoint 53f9754109c6f07e14ac620427a5e78e18accff1 committed779572 (--no-gpg-sign),
12files1012+/135−. Normal push534257/93531d succeeded and exact remote parity
46a07a matched HEAD, with clean status. The user task continues; no acceptance
or completion. Next bounded research connects the catalog entry to handle
carriage without silently treating raw call as catalog-protected.


2026-09-10 03:59 JST registered handle connection: direct consumer remains W2 current
module/function invocation, under the same U/TY/SL/PT trace. Main added explicit
invokeRegistered entries to the existing HandleValues/PureHandleFunctions
research sources. They reuse actual evaluators and carried stamps, then call the
catalog/current-use/contract boundary. General success includes independent
handle execution and exact current-operation-code catalog lookup; stale and
empty-authority rejection remain proved. Original invoke remains the weaker
explicit candidate and its signature is preserved.

A separate issue was exposed: an ill-typed application whose lambda ignores its
argument can still evaluate to a valid handle and pass operational invocation.
That is not source typing admission. invokeClosedRegistered therefore requires
infer [] e = handle before using the existing evaluator. General soundness
returns Typed plus actual execution/catalog/current-use/contract evidence.
Relative completeness uses independent finite Executes derivation to obtain
sufficient fuel (7cf589), not just successful-evaluator assumptions. General
carried-handle equality and actual42/ill-typed rejection/substitution rejection
controls pass. Open external closure environments are not silently admitted by
this closed profile; there is no Mir parser/byte-source refinement claim.

Three invalid variants—catalog bypass, closed-entry typing bypass, and replacing
carried stamps with caller stamps—fail both obligations and concrete controls
(094506, decisive false propositions inspected61ac4c). Fixed controls are not
general proofs; failed mutant sorryAx logs are rejected evidence only. Final
scratch HandleCatalog.lean/CHECK.json lives under handle-catalog-jxagx7z2.
Mirroring split the additions into the existing two sources; isolated rebuild
of both passed b5194d, no new interpreter/source root. Main full extension
self-review d47e3f checked the low-level versus checked-entry distinction.
W2_EXTENSION_CHECK retains old cuts and current hashes/axioms. All later work
is outside the active Oracle packet and remains unreviewed. Latest a801d7 keeps
both Oracle jobs running/submissionfalse, no answer and no duplicate.

Current goal, plan correspondence, companion, progress and sample evidence row
synchronized. Documentation.md/docs/project-status.md/tasks.md 更新不要 for this
sub-boundary: same ten candidates/goal/blockers, no milestone close. No new
Rust/network/persistence run or production change. Plans133/134 fully read and
hash-ledgered. This is continued proof-first work, not W2/alpha completion.

The first diff check for this mirror reported an extra EOF blank line1574bf.
It was removed, both dependent sources rechecked84784a, and current15source
hashes/axioms plus all3 concrete mutant failures verified819e23. This is not
a hidden failed validation. Required Plans135–140 are now full/hash-ledgered.

The same catalog consumer now has general downstream-addition consequences:
Extends and finite insertion sequences preserve existing successful calls for a
fixed current World/use/registry; no catalog revives a noncurrent module; missing
current-code entries reject. Scratch5e9319 and current Module+Handle+PureHandle
rebuildbf3bd4 pass trust0 with standard axiom audit. No new implementation or
mutator was added for these four consequences; existing checked catalog
operations and countermodels remain the evidence basis. This is not rollback
protection for World, authenticated old-image handling, distributed atomicity,
fresh import or same-instance recovery. Exact source/commands are retained in
W2_EXTENSION_CHECK catalog_consequences. Companion/plan memory updated;
progress.md/tasks.md/samples_progress.md 更新不要 for these same-entry consequences:
no new command/root/readiness/goal or blocker change. Docs39803 passedefd5bd,
1762reports; no new Rust/network/recovery execution.

Final current15hash/axiom check5afe34 passed; retained capture2/catalog2/entry3
invalid variants remain rejected and current downstream-consequence proof passes.
Focused doc diff c12a94 preserves raw-versus-registered-versus-checked-entry scope.
The handoff F0.3 WORKPLAN was reread in full c9d746: its W2 floor permits final-
syntax-independent semantic examples but still requires type/resource/currentness/
effect distinctions and scoped acceptance. No W2 close follows from the current
component results. M10 designated-consumption and M9 private-continuation wording
were reread ba41b0; private continuation-image state is not automatically a
first-class effect continuation. No new lane is opened on that word match.
This own checkpoint is ready to save; the user task continues.

Checkpoint2c2ba24e6f8fb8ea31ed40b1b4dcbe4b09ddf798 committed9c0d7d,
pushed32340/2d5d4c; exact remote parity and clean status verified9a74d6.
Subsequent work continues the same W2 goal.

2026-09-10 04:23 JST higher-order reference preservation: main proved general preservation
for independent Executes over closure bodies, captured environments, applications
and iteration; actual evaluator and registered invocation inherit the bound.
P is an arbitrary reference-set predicate, not authority or confidentiality.
Typed hidden literal/captured-value examples return a stale reference; omitting
the relevant input bound is insufficient. All references equal one original
implies exact output equality, including its stamps. General arbitrary-handle
application+iteration is a positive witness. No source literal authenticity or
handle-selection IFC follows.

Scratch ReferencePreservation.lean/REFERENCE_CHECK.json is retained under
handle-catalog-jxagx7z2. Initial control elaboration failuref60afd was fixed648d22;
one ill-formed parameter-changing mutant was discarded, not counted as semantic
evidence. Final code-bound omission fails the general law and concrete false
control; captured-environment replacement by True loses application preservation
and admits the hidden captured reference141b5a. The main proof and concrete
capture_bound_necessary pass d9dcef. Mirrored current source plus registered-entry
consequence pass76b74e with trust0; reference proof uses propext, registered
consequence also inherits standard Classical.choice/Quot.sound. No Mir axiom or
accepted sorryAx. W2_EXTENSION_CHECK preserves old cuts and exact current record.

Plan correspondence/current goal/companion/progress/sample row synchronized.
Documentation.md/docs/project-status.md/tasks.md 更新不要: same goal, ten files,
readiness and blockers, no milestone close. No Rust/network/restore or production
change. Mandatory Plans141–146 full-read; no whole-project roadmap adopted.
Oracle3e3263 both running/no answer, unchanged jobs and no new submission.
This is self-reviewed candidate evidence, not independent review or W2 close.

Docs60279 finished PASS89d7f7,1762reports; current15hash/axiom and diff08c5ee pass.
Focused diffca37fe found the need to distinguish a singleton exact-stamp bound
from selection among several existing references; companion/plan wording corrected.
Plans141–156, including all1618lines of156 and historical155JSON, fully read and
hash-ledgered. Historical queue/bridge restrictions are not silently promoted or
used to replace this user's explicit task-local proof-first authority. No linked
report was bulk-read. Oracle b8f9ff remains running/promptSubmittedfalse; exec
polls5128de/50b799 return no new output. Existing jobs preserved. Progress Discord
113df5 sent around04:26JST; no complete notification or task stop. Save checkpoint
and continue with requiredPlan158 (157 was already fully read).

Reference checkpoint89177592bc9293e309598de74657998f0d457657 committed3a90aa,
10files475+/121−, pushed76454/2fd138; exact remote parity and clean tree7c2a20.
The user task continues. New finite IterationSelection uses one typed expression
and the same World/caller/proof/catalog, varying only its ordinary Nat input.
Count0 returns current token and actual42; count1 returns noncurrent stamp and
actual rejection. Both retain the same reference bound. This pressures a future
information-flow checker to retain count dependence if input is secret and
success/failure visible. It does not establish an existing observer disclosure.
The earlier commentary's word "revoked" was corrected: this token's revision
is mismatched, and no actual revocation transition/history is supplied.
Scratch IterationSelection.lean/ITERATION_SELECTION_CHECK.json and final mirrored
source pass ec08a6/b2d2d4 trust0, propext only for the named controls. Fixed controls
are not general proofs. W2_EXTENSION_CHECK preserves previous source cut.
Companion/plan memory updated. Documentation.md/docs/project-status.md/progress.md/
tasks.md/samples_progress.md 更新不要 for this same-command counterexample: no new
root, goal, readiness or previously unknown IFC blocker. No runtime/network/restore
change or test rerun. Oracle58b64b still running/no answer, original jobs retained.

The separate IterationBudget namespace proves identity iteration returns the same
token for every Nat count in independent finite Executes, while actual fuel4
registered calls complete at0 and exhaust at3. General value existence/equality
therefore does not imply fixed-budget completion privacy. Scratch bbe117 and final
mirror0b239b pass trust0/propext; exact record in W2_EXTENSION_CHECK.iteration_budget.
A preliminary scratch command2f818b imported the already-mirrored names and failed
duplicate declarations; it is not evidence of invalid semantics. The original
IterationSelection scratch bytes were restored, and the new delta was checked in
its own scratch namespace. Reproduction of current source uses the documented
fresh-copy whole-module command; old scratch checks bind their earlier cuts.
No new implementation behavior, label policy or observer interface was selected.
The same snapshot non-update reasons apply. Required Plans178–198 gaps fully read
and hash-ledgered (earlier181/182/189/199 already full). New docs validation56787
is still running; do not count it as a pass until collected.

The budget discriminator now explicitly includes general execution_result_unique
and quiet_result_unique. Two finite derivations are compared at a common
sufficient fuel from existing completeness, so the same-token claim concerns
all actual declarative outcomes, not just one chosen derivation. Final mirror
e5b429 passes; these consequences inherit standard propext/Classical.choice/
Quot.sound, separately from the propext-only identity-execution construction.
No global normalization or budget sufficiency is claimed. Docs56787 finished
PASS e4f821,1762reports. Current15hash/diff ba01d5 passed before this final
uniqueness delta; final source record was refreshed by e5b429. Plans200–202 full
and ledgered. Latest Oracleb489f9 both running/no answer; preserve jobs.

2026-09-10 04:51 JST continuation reading: checkpoint6d5a7c74670b604a261d38ebb0cee6a682a3658a
verified HEAD/remote parity and clean tree821548/469254/a4ef7d. Old push process
64455 was already unavailable; remote parity was independently checked. Mandatory
numbered Plan gaps203–246/248 and Plan15 are fully read/hash-ledgered; earlier
220/235/247/249/250 retain prior full records. Six unnumbered post-WRK dispositions
and specs/examples00–09 are now full. Reading a reference did not credit its
target, and truncated output was recovered before full credit. No new global plan.

The historical comparison distinguishes typed result, validation-consulted
grounds, production grounds, receipt/pending correlation and restored consumption.
Current W2 reference-set preservation supplies none of those missing provenance
or restore relations. The old parser-free harness explicitly supplies predicate/
effect outcomes through sidecars; it is not actual ordinary-source computation
or network evidence. Its documented whole-store rollback remains a bounded LAB
implementation explanation, not a general place-local or cross-place guarantee.
No frozen record or historical source was changed.

Only reading/continuation/report metadata changed after the checkpoint. plan/,
Documentation.md, docs/project-status.md, progress.md, tasks.md and
samples_progress.md 更新不要: no semantic/readiness/goal/command/sample change or
milestone close. Lean/Rust/network/restore validators are not rerun for this
metadata-only reading block; prior passes remain bound to their recorded cuts.
Oracle3902ce both running/no answer/error; original jobs retained, no duplicate.
No independent review, W2 acceptance, alpha claim or task stop follows.

### 2026-09-10 05:01 JST — required LAB reading continuation

- All302 current files under plan/ now have a current-hash full reading record
  (9aaceb inventory, not a claim that every required corpus is read). WRK0001–0046
  retained artifacts were read with their registered scope, failures and historical
  source cuts. WRK0033/0034 truncation was recovered65efb8/55d99d.
- WRK0040–0043 classify supplied adverse fixtures; WRK0044/0045 expose conditional
  premises rather than proving operational authority/restore enforcement. WRK0046
  supplies a general finite-line consequence of local spent preservation plus a
  designated restore bridge, with a non-preserving countermodel. None substitutes
  for all-mutator implementation preservation in this task; none was rerun here.
- Current samples/README full read recovered truncated middle fdb1b0/c26c74.
  specs/examples10–16 full; selection/catalog helper behavior and detached expected
  artifacts remain distinct from ordinary-source execution and observation policy.
- Read ModuleContractBoundary and ContractExport current source again: module
  invocation authority and the resource allocator policy are separate inputs.
  No production bridge or new authorization semantics adopted.
- Oracle bb2c87 both running, no answer/error; 1cb0be check not due. Existing jobs
  retained. No extra consultation, baseline, Lean or Rust rerun for this reading
  metadata delta. Prior kernel/check evidence remains bound to its recorded cut.
- plan/ 更新不要; Documentation.md 更新不要; docs/project-status.md 更新不要;
  progress.md 更新不要; tasks.md 更新不要; samples_progress.md 更新不要:
  same active W2 goal, readiness, commands, sample taxonomy and unresolved gates.
  READ_LEDGER/RESUME updated only. Not milestone/user-request close.

### 2026-09-10 05:10 JST — closed typed pure-handle totality candidate

Same W2 goal/direct consumer and trace IDs. Candidate A constructs a logical
relation from typing, with induction over finite Nat iteration; smallest B retains
only the earlier finite-derivation-relative completeness and requires a separate
termination premise at each consumer. A now has kernel evidence and remains an
unreviewed LAB candidate. No production gate is closed.

Scratch base-calculus check c86ddb and handle-calculus check8d77e5 passed. Final
handle/checker/registered-call scope3c1d27 passed, followed by current whole
PureHandleFunctions mirror675bdb PASS. First drafts8b11fd/f0585a failed on explicit
Nat/size binder inference; a403c8 had generic-decide/typed-input control errors.
Those failed commands are not proofs; inferred sorryAx was never accepted.
CHECK/HANDLE_CHECK/RED records remain under pure-normalization-7cstqpj9.
W2_EXTENSION_CHECK retains the earlier source row and current hash/axiom output.
Original PureFunctions and frozen Oracle packets are unchanged.


The later unreviewed Normalization namespace supplies a general totality result
for closed, typed expressions of the pure higher-order handle calculus, including
arbitrary finite natural iteration. A type-indexed logical relation is proved
by induction on the independent typing derivation; it is not assumed for every
expression. The fundamental/finite-iteration construction uses propext only.
Existing execution completeness yields one result and a sufficient minimum fuel,
with propext/Classical.choice/Quot.sound. This is big-step termination for this
calculus, not strong normalization of an unspecified reduction relation.
For a checked closed handle expression, a finite selection derivation now exists
without a separate termination premise. At all sufficiently large fuel values,
the registered entry equals the catalog/current-authority/contract call on that
selected handle, including rejection. This creates no authority or successful
call guarantee. An unbound variable never returns; closed typing cannot be
omitted. Existing fixed-budget and secret-count counterexamples remain valid.
No runtime cost bound, fixed-budget completion, timing/resource noninterference,
source refinement, effectful/general recursion or module lifecycle is proved.
Original PureFunctions is unchanged; its analogous scratch proof is supporting
research only. All these additions are outside the pending Oracle packet.


Only the existing PureHandleFunctions source was extended; companion, plan memory,
sample evidence row, progress log, current check record and continuation/read ledger
are synchronized. Documentation.md/docs/project-status.md 更新不要: existing ten
unreviewed-candidate summary remains accurate. tasks.md 更新不要: same goal,
blockers/order/reopen triggers and no milestone close; no snapshot rewrite needed.
No new taxonomy or command, so samples/README and scripts/README 更新不要.
No Rust/network/restore rerun: proof-only extension and no production change.
Focused diff/current-hash and docs validation remain to be run after this update.
Oracle a8333e existing2jobs running/no answer/error. No review/acceptance inferred.
No sub-agents used. Prior metadata checkpoint4dde92c8 was pushed a44a35; this
new proof delta is not yet committed/pushed. Continue after validation/checkpoint.

Typed-environment refinement: f3c8d7 scratch and af22c3 whole-source mirror pass.
Draft9077e6 used unsupported mutual-inductive induction and was rejected; the
corrected proof uses the explicit HasType recursor and list induction.

The same extension also proves every independently HasType-typed value computable
using the existing mutual value/environment recursor. EnvTyped then entails the
logical environment relation, so any Typed expression in an EnvTyped environment
has an actual result of the declared type at every sufficiently large fuel.
This discharges the semantic environment premise from structural typing; it does
not add an external closure decoder, environment-admission checker, authority,
serialization rule or information-flow policy. The selected registered entry
remains closed. These general proofs have the same standard axiom boundary.

Docs39061 predates this last refinement and remains running; final docs validation
must cover the updated documents. No new production/source/milestone acceptance.

Docs39061 exited1 (1ee34a): progress last-updated header remained04:23 while
its new log was05:10. Header corrected72bd26; sample header refreshed from actual
clock. This was a documentation freshness failure, not a theorem failure.
Final documentation check follows on the corrected cut.

Final totality-cut validation: docs rerun33154 passed (fbc5b8) after the retained stale-header failure; full172-line proof delta reviewed (e5e909), current15 source hashes and successful axiom records matched. No new Rust/network/restore execution or Oracle acceptance is claimed.

Forward continuation after totality checkpoint494571ea (normal push202165):
request-bound allocation candidate remains in external scratch
allocation-request-binding-fo86o09w, pointer ALLOCATION_REQUEST_WORKDIR.
General exactness/soundness/WF/current-denial and unary policy size-indistinguishability
checked108312; standard propext/Quot.sound only. Positive10, accepted17-but-denied,
changed-request and deny controls; ignored authority and wrong length mutants
rejected15cca9/108312, including decisive finite wrong-length rejection control.
Two failed proof drafts retained as RED/SIMPLIFIER_RED, no accepted sorryAx.
Authorization callback consumes current State/full existing Binding/exact Action;
it is a trusted input, not a proven issuer or authenticated registry/head. Only
allocation is composed here; this does not close all-mutator authorization,
physical atomicity, source admission, replay, recovery or information-flow gates.
A request-bound current callback is compared only with the original unary policy;
no production contract or canon choice is adopted. Pending Oracle packets exclude it.
`plan/ 更新不要`, `progress.md 更新不要`, `tasks.md 更新不要`,
`samples_progress.md 更新不要` for this scratch comparison: existing documented
request-binding obligation, same readiness/current goal and no source/sample delta.
Documentation.md/docs/project-status.md remain unchanged for the same reason.
No new report/framework, no Rust/network execution and no sub-agent.

### 2026-09-10 05:37 JST — allocation request and checked-arithmetic consumer candidates


Later unreviewed ContractExport extensions address two existing W2 consumer
obligations. RequestAllocation passes the current reference State, the whole
existing Binding and the exact allocation Action to an independently supplied
authorization callback. General exactness exposes accepted positive actual length,
the callback decision and the same raw allocation; WF is inherited. The old unary
principal policy cannot distinguish any two positive lengths for that principal.
10-unit acceptance, valid17-unit rejection, changed-request rejection and current
deny controls separate arithmetic evidence from authorization. This callback is
trusted input, not an issuer, authenticated current head, full CurrentUse context,
all-mutator authorization, physical atomicity or a selected production contract.
The other resource mutators retain their original explicit reference boundary.

CheckedArithmetic adds a finite-interval evaluator for existing local-contract
Term inputs/literals/add/mul/square and independent declarative rules. General
exactness and soundness preserve the mathematical value, input scope and range.
Successful evaluation is equivalent to scoped inputs and mathematical bounds at
every syntax node. Accepted contract length plus these bounds yields the actual
checked result Int.ofNat(length). A positive in-range final value alone is
insufficient: an intermediate product may overflow before cancellation. Missing
inputs reject instead of using the mathematical evaluator's default. General
proofs use only propext/Classical.choice/Quot.sound (some subsets); no Mir axioms.
Signed64 controls motivate one finite candidate; no final numeric/failure/privacy
policy or existing-Rust implementation refinement is adopted. Current source
checker accepts square-plus-one and cancellation programs, but the current debug
interpreter panics on their overflowing products. This is retained counterevidence,
not a fixed runtime or an alpha acceptance claim. Both extensions are outside the
pending Oracle packets, and request-bound callback authenticity/actual source
admission and machine failure information flow remain open.

Same W2 goal/trace IDs/direct local-contract→exclusive allocation consumer.
Candidates: exact independently supplied request decision vs unary principal policy;
finite checked operations vs prior static intermediate-bound requirement. The
static bounds characterize when checked execution succeeds; neither is adopted as
final numeric policy. Existing arithmetic/authority obligations narrowed, not closed.

Scratch RequestAllocation final108312 (11ea1def...) and CheckedArithmetic final
e70a96 (exact hash in retained CHECK.json) passed trust=0. Arithmetic earlier
560332 failed on range-proof and monadic simplification; correctedf9a9e2 then
701cfb/e70a96 passed. Failed drafts with inferred sorryAx are rejected records.
Arithmetic unchecked-intermediate and default-missing-input mutants both fail
general proofs and concrete controls (e70a96/4a10aa). Request ignored-authority
and wrong-size mutants fail (15cca9/108312). Finite controls are not general proof.

Actual source: first29b683 invoked a preexisting test-harness artifact, which
rejected --entry; no semantic result from that command. Correct offline/locked
example builds393a60 and e5566b succeeded using existing target (runtime31 warnings,
no Clippy claim). Current source checker accepted both programs with zero diagnostics.
Actual session01ccde returns Int64(10) for x=-3 square-plus-one; x=Int64MAX and
cancellation x=4611686018427387904 both exit101 at interpreter.rs:1418 multiply
overflow. Cancellation's mathematical result9 is also Lean checked. The actual
source subtraction and reference addition of a negative literal are a finite
comparison, not a proved parser/IR refinement. Exact source/binary hashes, commands,
stdout/stderr and rejected harness invocation are retained in
checked-contract-arithmetic-npt5atdt. Capacity817528:59GiB free,12GiB available RAM;
existing target reused, no new large workdir/cleanup. No network/restore execution.

Mirrored into the existing ContractExport module only (cc3b83,472lines), preserving
old entries and all frozen packets. Fresh15 dependency rebuild29153 is pending;
no success claim before collection. No new framework/module/production source.
Plan/companion/sample evidence/progress and this report synchronized. Existing ten
unreviewed-source summary in Documentation.md/docs/project-status.md remains valid;
those files 更新不要. tasks.md 更新不要: same goal/readiness/order/blockers and no
milestone close. No sample taxonomy/new command; samples/README/scripts/README
更新不要. No Canon/THM/OBL/phase changes or sub-agents. New delta uncommitted/unreviewed;
continue after validation, not task close.

Fresh15-source rebuild29153 completed19852a PASS. Current hashes and axiom records match; W2_EXTENSION_CHECK preserves the preceding dependency cut and exact new scratch/mutant/actual-source evidence. Final documentation/diff validation follows.

Forward local flow closure for the checked arithmetic candidate:38863f general
flow-check exactness and two-run equality of the entire Option result passed;
b16089 True-constructor draft rejected. Inputs must agree as Option values on
every input visible to the same observer under the same supplied labels/range.
Thus both value and failure agree under those premises. Secret square counts0/4
produce some()/none even after payload erasure; ignoring square dependency mutant
fails both theorem and concrete control63de19. This does not cover runtime cost,
source-label authenticity, allocation geometry, authorization callback outputs or
a public failure policy. Source appended to the same ContractExport (536lines);
fresh15 rebuild95412 pending. Original scratch/Oracle cuts preserved.

Flow-inclusive fresh15 rebuild95412 completed e432a9 PASS; current15 hashes and
axiom audits match. Original request/arithmetic cut retained in the check record.
Finite ordinary-source texts are now included with their hashes beside their
actual execution evidence for reproduction without the disposable workdir.
Docs10674 passed22fc04 for its preceding cut; final flow-document check95362 remains
pending. Main-only focused source review e3db2d; later64-line flow delta reviewed
as authored, final diff check follows. Oracle79237f both still running/no answer.

Final flow-document check95362 passed e86e3e; focused flow diff3efc16 and
whitespace8fe215 passed. Current15 checked source hashes/axioms are verified.
JSON evidence/ledger validation and own nine-file checkpoint follow; no production
change or Oracle/alpha acceptance. Last Oracle a30fad remains running/no answer.

### 2026-09-10 05:58 JST — finite identifier history and checked allocation composition


The later unreviewed BoundedIdentifiers extension retains the same resource
operations and fresh-ID history with finite handle/block identifier ceilings.
Independent per-operation capacity rules and both addition-based and
remaining-capacity checks are exact; the latter checks current bounds before
subtracting. Successful operations preserve spatial WF and finite bounds, and
arbitrary finite changing-policy schedules preserve old-handle absence. Allocation,
move and split reject when fresh identifiers are exhausted; release can still
succeed without resetting history. Given representable ceilings, successful next
counters remain representable. This is not a native-arithmetic/refinement proof,
physical memory budget, chosen ABI, availability or authenticated recovery policy.
Resetting counters after release can restore spatial WF yet resurrect an old
handle; the history restriction cannot be replaced by spatial WF alone.

CheckedAllocation composes actual finite-interval Term evaluation, equality to the
claimed contract result, contract checking, the independent exact-allocation
callback and bounded resource admission. Its general exactness/soundness and
relative completeness expose each obligation; a valid positive mathematical
contract with overflowing intermediates does not allocate. This remains a
nonproduction reference composition, not ordinary-source/parser/IR or network E2E
refinement. Callback authenticity/currentness, allocation cost/geometry information
flow, actual memory failure, physical atomicity and recovery remain open. Neither
unbounded IDs nor recycled finite slots are silently adopted for production.

Same W2 trace IDs/goal/consumer, not a new milestone or accepted profile. Candidate
A finite exhaustion rejection is compared with B recycled slots plus independently
authenticated generations; B requires further lifecycle/restore/wire evidence.
Ceilings are parameters, not a selected public machine word or lifetime promise.

Scratch BoundedIdentifiers initial43c508, controlsaae172, mutants2ae568 and final
remaining-guardc35f3d passed (standard axioms only). Failed State.ext and function
simplification drafts34019c/2d442d/57b3c2 retained, not accepted. IgnoreCapacity and
OneIdentifierForSplit fail general proofs and concrete controls. Wrapped-counter
control demonstrates spatial WF plus resurrected old handle after an excluded
reset transition. The remaining-capacity predicate avoids addition in admission;
a host must preserve the initial guard before subtraction and implement atomic
checked updates. This does not prove a current Rust counter implementation safe.

Fresh15 scratch dependency cut27042/c15931 passed with the candidate Resource
source. CheckedAllocation final4b6a4c passed: actual interval execution→contract
actual value→independent request gate→finite allocator. SkipMachineEvaluation and
SkipIdentifierCapacity mutations fail proofs and concrete rejection controls,
including accepted positive9 mathematical cancellation with overflowing product.
Failed indentation/monadic simplification/control draftsac32d8/42346b/18ce23 remain
rejected. No auto-sorry is accepted. Exact CHECK/ALLOCATION_CHECK/MUTANT records
remain in bounded-resource-identifiers-3bldiy43; pointers are in RESUME.

Mirrored only existing ResourceBoundary (651lines) and ContractExport (624lines),
c13eec. Fresh15 actual-source mirror37970 running; collect before success claim.
No production or Canon files edited, no runtime/network/restore rerun for this
reference composition. Earlier current source multiplication panic remains open.
No new report/framework/module; same ten unreviewed W2 sources. Plan, companion,
sample evidence and progress updated. Documentation.md/docs/project-status.md
更新不要: existing unreviewed summary still accurate. tasks.md 更新不要: same goal,
readiness/order/blockers, no milestone close. No taxonomy/command changes, so
samples/README/scripts/README 更新不要. No sub-agents. Old Oracle packets exclude
these additions; no acceptance is inferred. Prior checkpoint869965f3 normal push
6f1759 and remote parity86739d succeeded. Current delta uncommitted; continue.

Actual-source fresh15 mirror37970 completed4fa406 PASS; exact current hashes and
axiom audits matched. W2_EXTENSION_CHECK retains preceding two source cuts and
new identifier/allocation proof and mutant records. Final docs60012 running;
whitespaceb1a75e passed. No Oracle or production/alpha acceptance inferred.

Final docs60012 completed PASS7a62a7. Full source delta7f1035 reviewed; no forbidden
proof declarations91bd86; whitespaceb1a75e passed. Specs/examples70–76 fully read
and hash-ledgered as historical LAB. Same W2 goal continues after checkpoint.

### 2026-09-10 06:21 JST — current module/resource authorization composition


The later unreviewed CurrentAllocation reference entry keeps current module use
and resource authorization independent. Both see the same supplied World/request,
complete carried stamps, actual descriptor/arguments and exact allocation action;
catalog integrity, checked arithmetic, contract and finite capacity remain checked.
General exactness, soundness and completeness relative to accepted local contracts
and intermediate bounds are kernel-checked; denial at either authorization layer
prevents allocation. This does not issue authority or authenticate the supplied
joint state. Controls accept42, reject revoked/retired use, absent catalog, exhausted
IDs and resource denial; refreshing module evidence for request10 does not reuse
the resource grant for request9. Four skipped/substituted checks fail general
proofs and concrete controls. A repeat of the same admitted request can allocate
fresh handles0 and1: request deduplication is NOT provided by current authorization
or fresh IDs. Atomic coupling of current checks, allocation and durable decision
history is still an implementation obligation, not a resolved Q-18 protocol.
No ordinary-source/E2E, network, save/restore, all-mutator, resource/authorization
information-flow or alpha acceptance follows from this reference composition.

Same W2 goal, PL1 S1/S2 theory/proof/reference and direct consumer. Current candidate
checks both layers against one supplied state; smallest alternative reuses a
module authorization as resource authority and is rejected by the denial and
borrowed-request counterexamples. No production protocol selection or Q-18
prepare/commit equivalence is adopted. Initial unknown lemmaaf27fa and existential
shape990783, then control layout8defa6 failed and remain recorded; finaldbaca8 passed
without sorryAx. Four mutations254043 fail concrete controls and general proofs.
Existing ModuleContractBoundary extended437→605lines; full15 fresh actual-source
mirror25760/d86cdb PASS with exact hashes and standard propext/Classical.choice/
Quot.sound only. No new source module/framework/report. Full specs/examples77–80
read94416e/8a68b4 as historical LAB, not a current parser contract. Prior checkpoint
c1470a3f committed6d2e86/pushed1831d3; remote parity586bad. Current delta uncommitted.

plan/ companion updated; progress.md recent log and sample evidence row updated.
Documentation.md/docs/project-status.md/tasks.md 更新不要: same ten unreviewed W2
sources and same active goal/blockers; no milestone close or readiness promotion.
samples/README.md/scripts/README.md 更新不要: no taxonomy or command change.
No runtime/network/restore rerun: only nonproduction proof extension; earlier real
source overflow remains unresolved. Oracle jobs running659d17, no final answer,
these additions excluded from frozen cuts. No independent signed review/subagents.
Final docs and diff checks pending; continue current consumer research afterward.

### 2026-09-10 06:26 JST — coupled allocation/history reference


The further unreviewed Once entry couples successful CurrentAllocation effects to
an append-only request-key history in one abstract atomic step. For arbitrary
finite invocation sequences (including changing supplied Worlds, registries and
grants), general proofs preserve resource WF and history uniqueness and exclude
a second effect for an already committed key. Failed attempts leave both parts
unchanged; fresh successful requests can allocate. Keys reuse the existing
CurrentUse (instance, principal, request) candidate; they are not regenerated from
changing argument/code/generation fields. A conflicting reuse is rejected, not
served from a cached result. No public retry/error or request-ID policy is frozen.
The raw CurrentAllocation entry still admits repeats; only this added reference
step includes effect/history coupling. It does not acquire an authentic World or
make two independent physical writes atomic. Resource-only crash survival permits
repetition; history-only survival blocks the missing effect. These two concrete
prefix counterexamples require an actual durable mechanism. History compaction,
finite request namespace/storage exhaustion, concurrent ownership of the decision,
recovery/fresh-import identity and history/failure information flow remain open.
Neither Q-18 reservation versus reauthorization nor physical exactly-once delivery
is selected or proved. Duplicate rejection returns no cached handle or authority.

Same W2 goal and direct resource consumer. Candidate atomic effect/history pair
versus smallest alternative separate writes: both crash prefixes are decisive
counterexamples to substituting the latter. This is a bounded reference research
result, not a new production contract or storage protocol. Scratch8bacc0 kernel
PASS; first map-simplification5faf90 failed, correctedf55f24; no accepted sorryAx.
IgnoreHistory/ForgetCommit/ForgetEffect mutantsd23c4d inspectedb04c4b: general
proofs and concrete controls fail. Controls include successful42 allocation,
repeat rejection, distinct-request success, denied-then-retry success and the two
excluded crash-prefix states. ModuleContractBoundary now752lines, no new module.
Fresh15 actual-source mirror11886 running; do not infer completion until collected.
Preceding current-allocation docs82438 completed PASSd2cc93; final new-cut docs pending.
Same documentation/sample/plan update scope and skipped production checks as above.
Oracle remains pending3de657, no acceptance; no sub-agents or new consultation.

Final current-source mirror11886 completed PASS2bc8aa; current15 hashes/axioms
match. No current Lean job pending. Final documentation check follows.

Read full sys5_i3_process_snapshot763 (7efdda/6b4106/44dfa6),
sys5_i3_process_local_cut1203 (917009/a4fb77/b9c2ec/0c628e), and
sys3_i3_private_snapshot2805 (b0a038 through f3ff95), hash-ledgered. The projection
DTO carries persistence responsibility categories, not a post-execution committed
resource/history image; private child seed restoration still needs separate trusted
start binding. Custody guards reject active asynchronous obligations/retained ingress/
owner reservations/pending reply but are not durable allocation/history transactions.
No claim about uninspected downstream runtime paths. Focused existing six snapshot
tests launched87275 under PROJECTION_SNAPSHOT_REGRESSION; collect actual result.
Resources ff6727/5f50c3:59GiB free,12GiB RAM available; cached build, no heavy new
workdir or cleanup. Docs98910 still pending. Discord progress4fc23c06:28JST sent;
next roughly07:28, no completion. Oracle3aaa05 both running/no final answer.

Existing snapshot regression87275 completed0ea8ca:6 PASS/392 filtered/0 ignored,
27.36s cached lib-test build,30 warnings; exact command/stdout/stderr retained in
W2_EXTENSION_CHECK. No actual network or effect/history persistence claim.
Nested local-cut tests386lines fully readb2c7dc; feature-specific suite not rerun.
Current proof delta source review78c3b4/c1e024 and whitespace8f2108 pass.

Final documentation validator98910 completed PASSf9fdda. Remaining changes are
evidence/read/continuation metadata. Final JSON/hash/diff checked before checkpoint.
No validation/Lean/Git process pending; keep the two Oracle jobs.


## Forward reservation/effect evidence — 2026-09-10 06:47 JST


Reservation is a later unreviewed reference boundary around the same actual
CurrentAllocation allocator. It separates retained identity, actual effect history
and response visibility. A pre-effect stop retains identity with no effect; a
lost response after checked allocation retains both allocation and effect history.
Arbitrary finite attempt sequences preserve resource WF, distinct reserved/effect
keys and effect-to-reservation inclusion. A repeated reserved key rejects without
another allocation, even after other invocations with changed supplied context.
A newly recorded effect entails actual checked allocation. Useful success returns
the real length42 handle; response loss retains that same effect. Three mutations
(ignore reservation, invent a pre-effect fact, erase effect on response loss) fail
general proofs and concrete controls. General proofs use only propext, Quot.sound
and inherited Classical.choice; no new Mir axiom or accepted sorry is introduced.

This is a conservative nonproduction state machine, not a refinement proof for
all SYS5/SYS4/native transitions. Its pre-effect stop is an explicit model input,
not a conclusion available from an arbitrary network error. Smallest viable
alternative: clear only an independently established no-effect/no-outstanding-action
reservation; the model accepts a retry after such a pre-effect stop. Clearing on
mere missing response instead repeats the actual allocation. No public outcome
or Q18 authorization-reservation semantics is adopted. Reservation exhaustion,
compaction, authenticated heads, crash persistence, concurrent ownership,
availability and outcome/history information flow remain open. In particular,
the atomic Once.failure_unchanged theorem does not describe every runtime error.

Full reads of current sys5_i3_process_runtime.rs and sys5_i3_private_quic.rs
confirm distinct real mechanisms: owner reservation before SYS4 handoff; retained
Ambiguous/ServeReserved state after uncertainty; current authority revalidation;
requester attempt reservation before the first awaited frame write; completed
write versus accepted receipt; retained ingress reservation before a cancellable
read. Provider occurrence references avoid raw-value-derived frame hashes, unlike
the ordinary restricted carrier profile. These are code facts, not general privacy
or durable-recovery proofs. The ordinary process start initializes pending,
tombstone and effect-occurrence maps empty: its image bootstrap must not be counted
as same-instance post-effect recovery. Runtime-only retry2 and ledger1 tests pass
at this cut; no new actual QUIC or persistence test was executed in this increment.

Start HEAD bbc25960968e93c3ffecfa1681f6a9a873de14c6, main already pushed;
only own read-ledger/RESUME metadata dirty at increment start. Main alone.
Scratch RESERVATION_EFFECT_WORKDIR: kernel b0361b, actual positive/negative controls;
three final mutants e2ec0f, each concrete false. Initial failed elaboration ef7985
retained in ELABORATION_RED.json (simp expansion, dependent equality, Handle numeral);
corrected source has no sorry/admit. W2_EXTENSION_CHECK retains final source/cut
and command output. Runtime regression2a65ae/413acc (2+1 pass, 30 existing warnings)
uses cached target, not a full workspace or physical-network validation.

plan/ and samples_progress.md updated for this evidence; progress.md recent log
updated. CURRENT_GOAL preserves the same W2 consumer. Documentation.md,
docs/project-status.md and tasks.md 更新不要: same active research, readiness,
critical path and owner/Canon gates; no milestone close or roadmap adoption.
Oracle269d86 both running with no answer/error; new extension excluded from frozen
review packets. Self-review only, no independent review claimed. Production and
required physical/durable/IFC validations remain unexecuted because the foundation
and review gates are open. Commit/push follows focused diff and docs validation;
no new report or subagent session, no task-complete notification.

Follow-on full read of sys5_i3_owner_admission_tests.rs3098 lines completed. Actual
component regression f56a8a:27 passed,0failed,0ignored,371filtered;30existing warnings.
It confirms the existing distinction between G2 revalidation before permit issuance
(preserved unrelated owner request can proceed) and generation-exact final handoff
(an already G1-issued permit cannot cross G2 even when another owner was revoked).
This is an existing restricted runtime policy, not adoption of a universal Q18
answer. No new production source or test code, no physical-network claim. Exact
command/logs are in W2_EXTENSION_CHECK.owner_admission_regression.

Final fresh15 mirror6f3f61 PASS; exact source/axiom audit4f7146 PASS and focused
source/docs diffsdac5ef/b4ed43 inspected. Documentation validator94276 exited0
aeaa7d (1762 reports; --help is ignored by its membership-only argument handling,
so this was the normal validation, not a help response). Whitespacef9368c PASS.
Only continuation/read/evidence metadata changed after validation; no pending
Lean/Rust/docs job. Two Oracle jobs retained, no review acceptance inferred.
