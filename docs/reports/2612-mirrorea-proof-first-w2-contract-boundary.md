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
