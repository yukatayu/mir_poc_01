# Report 2613 — Mirrorea proof-first W3 dynamic composition and graphs

- Started: 2026-09-12T19:52:15.364633+09:00
- Author: sole main Codex; no subagents
- State: active theory/proof/reference research, not formal acceptance

## Objective

Complete user-requested W3: ordinary source expresses a base on A/B/C and a new
composition on A/C, with supported addition, retirement, reparent and compatible
exchange. Distinguish definition/instance/package/live dependencies; preserve
individual DAGs, cross-kind support, references/fallback and current result use.
Stop after W3; W4+/alpha/Plan250-I3-4 are not automatically resumed.

## Scope and assumptions

PL1 S0/S1/S3 theory/proof/reference. The active semantic goal is W3 dynamic
composition; the first dependency is support under genuine finite-domain growth.
The owner explicitly requests W3 under the earlier single-agent/proof-first rules.
Reversible research/limited internal work is authorized; Canon, owner signatures,
L0/L1 privacy/authority and public/production boundaries remain unchanged.
Q18 is unresolved: prepared authorization reservation differs from commit-time
reauthorization. W2's finite pure/handle/resource/captured-frame result is reused
within its actual assumptions, not as a whole-language implementation theorem.

Mapped motivations (not blanket adopted requirements):
REQ SL-05, ID-01, ID-02, ID-03, ID-04, ID-05, ID-08, RL-01, RL-02, RL-03, RL-04, RL-05, RL-06, RL-07, AU-01, AU-02, AU-03, AU-04, AU-05, CT-01, CT-02, CT-03, CT-04, CT-05, EV-01, EV-02, EV-03, EV-04, EV-05, EV-06, EV-08, EV-10;
U U01, U03, U04, U05, U06, U07, U08, U09, U10, U11, U12, U14, U16, U17; PT PT-01, PT-02, PT-03, PT-04, PT-05, PT-06, PT-10, PT-11, PT-12, PT-13, PT-14, PT-15; SC SC-01, SC-03, SC-04, SC-05, SC-06, SC-07, SC-09, SC-11, SC-12, SC-13, SC-19, SC-22, SC-24;
Q Q-05, Q-06, Q-08, Q-09, Q-10, Q-12, Q-16, Q-17, Q-18, Q-21, Q-23, Q-24. All119 original rows remain intact.

Candidate A: extend the finite snapshot domain with an injective old-slot map,
retain old support formulas via structural mapping and preserve old eligibility.
New nodes may depend on old nodes. Prove old Grounded/computed membership exactly
preserved, using independent proof-tree semantics. Separate semantic identities
from enumeration slots. Smallest alternative B: unbounded incarnation keys in a
finite map with explicit complete-domain checks. Neither is a public carrier.
The no-impact theorem is for closed old dependencies; it cannot cover arbitrary
reparent/replacement/eligibility change. Those require later explicit checks.

## Start state / dirty state

HEAD161be3f9a46df7cf279a3da136d565b4c373a21a main, clean, origin GitHub unchanged.
No reset to19a6decf, no user edits overwritten. W2 proof2760e17d and final receipt
161be3f9 retained; Report2612 remains closed and unmodified.
Root filesystem188GiB/50GiB free, RAM12GiB available. /mnt/mirrorea-work is absent
(findmnt returned no target); only small temporary copies use /tmp. No heavy build,
cache deletion, host-share placement or original handoff changes.

## Documents consulted

Handoff start/AGENTS, owner/context/protocols and W3 workstream; Canon
README/MAP/North Star/Constitution/source hierarchy/phase/ADR0043/Plan05/Plan250;
W2 RESUME and relevant final report/companions; W1 support/graph definitions.
Prior full reads reused only on identical hashes:1004 current files match the
ledger,12 changed files were excluded from automatic reuse. REUSED_READS.json
lists them externally. Mandatory global corpus remains incomplete (next legacy
example283); no new whole-repository roadmap accepted. Index/grep/partial tool
output is not counted as full reading. Oracle manuals/help read before first use.
Skill instructions used: using-superpowers, discord-report, brainstorming,
writing-plans, TDD, verification-before-completion, systematic-debugging and
receiving-code-review; user-specific single-main/one-report/autonomous research
instructions override delegation, extra approval and extra plan-directory defaults.

## Actions taken

Created the W3-only active goal, recorded Discord begin388710 without notification,
verified the immutable bundle, audited resources/toolchain and reused prior reads
only under equal hashes. Main alone prepared and sent each Oracle packet once.
Entry mirrorea-w3-entry-growth completed14m44s; material dependency review
mirrorea-w3-growth-kernel completed18m04s. Answers/dispositions are pinned in
W3_ENTRY_CHECK/W3_KERNEL_REVIEW. Both full reviews are terminal. Narrow correction review mirrorea-w3-audit-correction
was subsequently sent once432ad4 (29714tokens/13files, exec59702), and completed
exit0 in8m28s, terminal7c9f2a. All Oracle jobs are terminal.
No Chrome configuration changes.

Mirrored seven new Lean candidates in the existing foundations root, with one
companion and fresh-copy11-dependency/19-mutation harness. Main reproduced Oracle
countermodels and repaired its identified non-discriminating mutation target.
The audit now traverses every imported proof-module kernel declaration, including
private/unused ones, and rejects dependencies outside the three logical axioms.

Started external nonproduction InstancePrograms/InstanceState over the actual
W2 checked arithmetic and W1/W3 support/graph dependencies. Its explicit finite
input/output contract and structural insertion/retirement/reparent/replacement
proofs pass; ordinary-source and authorization/result adapters remain next.

## Files changed

- docs/reports/2613-mirrorea-proof-first-w3-dynamic-composition.md
- docs/proof-first/CURRENT_GOAL.md, RESUME.md, READ_LEDGER.json,
  W3_ENTRY_CHECK.json, W3_DYNAMIC_CHECK.json, W3_KERNEL_REVIEW.json
- `docs/project-status.md`
- Documentation.md, progress.md, tasks.md
- plan/proof-first-foundation-correspondence.md
- samples/README.md, samples/lean/README.md, samples_progress.md, scripts/README.md
- samples/lean/foundations/MirroreaProofFirstDynamicSupport.lean,
  MirroreaProofFirstDynamicGraphs.lean, MirroreaProofFirstDynamicIdentity.lean,
  MirroreaProofFirstSupportImpact.lean, MirroreaProofFirstCurrentChoice.lean,
  MirroreaProofFirstNamedCatalog.lean, MirroreaProofFirstDynamicScopeControls.lean,
  MirroreaProofFirstDynamicComposition.md
- scripts/proof_first_dynamic_composition_check.py

These are candidate proof/reference and task records. No production, Canon,
closed W2 report or immutable handoff source changed.

## Commands run

Startup git root/status/HEAD/remote; bundle verify; df/free/lsblk/findmnt/du;
Lean/Rust/Python versions; Oracle manuals/help/dryruns and two single submissions.
Exact commands/hashes/logs live under /tmp/mirrorea-w3-20260912-1_kv7wfk.

python3 scripts/proof_first_dynamic_composition_check.py --work-root
/tmp/mirrorea-w3-20260912-1_kv7wfk: corrected fresh11/19 and declaration audit pass
95d8c4, result mir-w3-dynamic-yydemoq4/RESULT.json. Failed development runs remain
separate; source errors never count as proofs. lean --trust=0 is used throughout.
Existing Rust textual_mir_alpha_parse accepted external source-probe/authoring.mir
ab3ed0; this is syntax evidence only. External lifecycle proof commands and hashes
are PROGRAMS_DRAFT*.json / STATE_DRAFT*.json, with failure/success logs retained.

make docs failedc3c2ea (Canon notice), c3813a (update declaration), ea0bf8/a77ce2
(exact standalone backticked project-status file bullet). Validator source was
inspected and each report/notice formatting issue corrected. Fifth run55304 failede92729 on the required tasks non-promoted-references
section, now restored after checking the full required section list. Final
current-doc validation is still required. Sixth run69121 failedab8b9b on the
missing explicit existing Canon path in tasks current promoted package. Restored
ADR-0043 there and inspected the remaining validation conditions before rerun.
Seventh make docs14970 passes493dec: documentation scaffold complete,1763 reports.
The targeted snapshot/source/heading/date/update-declaration checks also pass1d8013. git diff --check passes
bca85b. No unrelated full Rust/network build was run.

Guessed Canon filenames08-patch-evolution/14-maintained-relations did not exist;
the actual08-patch-hotplug/14-maintained-relation-projection sources were read.
No missing filename or truncated read is recorded as full reading.

## Evidence / outputs / test results

Bundle PASS611796:669 manifest files/614 expanded originals/119requirements/
30decisions/24scenarios. This is integrity, not mathematical acceptance.

Current mirrored general results: structural support embedding and finite-domain
append; separate old/new per-kind acyclicity; exact old Record/Handle/UseRequest/
Context/current-check preservation; conservative impact over every syntactic
all/any branch; independent least-current choice; named lookup/elaboration/catalog
checker exactness and old checked-catalog growth. Eleven fresh baselines pass.
Corrected19 mutations fail unchanged proof scripts within designated general
statements' ranges. This alone does not prove every mutated statement false.
All1716 declarations in the imported target modules pass the standard logical
axiom allowlist. The private unused choice-dependent audit control rejects with
choice excluded. No custom axiom, sorry or admit was introduced as proof evidence.
All printed successful dependencies are propext/Classical.choice/Quot.sound where
needed; graph-growth proofs have no axioms. Exact logs in W3_DYNAMIC_CHECK.

Oracle controls reproduce dead-rank changes, checked dead nodes, forgotten slot
transport, old-domain self-loop omission, stale inflationary retirement, unused
alternative impact, duplicate names, newly resolved old references, pre-granted
future targets, fresh witness search after revocation, and omitted module revision
with newly reconstructed handles. These are finite controls, not general proofs.

Historical development: GrowthDRAFT1, GraphGrowthDRAFT1, Identity/CurrentChoice/
NamedCatalog drafts and new scope controls2dd775/1929e1 failed elaboration and
were corrected. Old external4/12 run08506c and first mirrored11/19 e0f562 were
successful executions at those hashes. Oracle exposed a weak mutation interpretation;
the corrected95d8c4 cut supersedes that interpretation without rewriting history.

External lifecycle candidate: InstanceProgramsDRAFT3 passes5ce098 after a reserved
field name/library simplification fix. Independent bounded arithmetic Satisfies
and Refines checks are exact; accepted replacement preserves all old contract
inputs and output bounds. Explicit nonempty finite domain, changed x*x+1 -> x*x+2,
shrinking domain, wrong output and intermediate overflow controls discriminate it.
InstanceStateDRAFT7 passes0a5ca3 after recorded elaboration/rewriting fixes. The
actual state has distinct definition slots, instance slots, placement lists,
parent links, support formulas and per-instance revision/live bits. Structural
Valid is preserved by append, retire, checked reparent and compatible replacement;
retirement retains historical records and cannot resurrect support. A/B/C and A/C
instances share a definition, yet retiring/replacing one leaves the sibling record
intact; dependencies can become unusable; cycles reject. This is not yet a complete
source/authority/lineage/pending/result theorem or a physical deployment. Later
InstanceStateDRAFT10 passesb792fd with separate definition predecessor DAG, actual
compatible registration, old definition/instance locator preservation, unchanged
support under registration and derived snapshot preservation under insertion.

## What changed in understanding

The old surface_source_patch_hotplug helper sets its mutation flag from activation
report existence and manufactures its capability-reference list from requirements.
It is report evidence, not actual lifecycle state or authorization. No accepted
I3 regression claim is inferred from this separate helper observation.

Support growth does not require injection mathematically; actual stable identity
transport does. Insertion issues no authority, while an already issued claim can
name a future numeric target. Checked support is not checked naming. Current
existential authorization search is not revalidation of a saved witness; W1
context omits nonoperation revisions, so consumers must retain full handles/stamps.
Full static fallback and authorized fresh reacquire remain separate from a
monotone current-choice cursor. Domain/locus/instance/world identity fields must
not be identified just because all currently use natural numbers.

## Open questions

Actual checked source -> definitions/instances -> complete fresh and old metadata;
definition registration and patch history; all selected mutator/entry/leave/revoke
paths; exact current auth/captured handles/result release; static fallback floor,
owner-bound semantic degradation and fresh reacquire. Q18 reservation versus
commit reauthorization remains a policy distinction. Finite contract candidate A
versus sound symbolic plugin B is reversible LAB research. Physical distribution,
durable restore, secrecy and alpha integration remain later scopes.

## Suggested next prompt

Continue the existing W3-only goal autonomously. Both full Oracle jobs are terminal;
do not resend any of the three terminal reviews. Finish the coherent dependency checkpoint and continue external
lifecycle/source/current-reference work, then final material review and appropriate
regressions/Git integration. W3 completion is the stop boundary, not this checkpoint.

## Plan update status

Updated current goal and appended a forward W3 entry in
plan/proof-first-foundation-correspondence.md; old W1/W2 entries remain history.
No Plan250 or Canon change. Entry Oracle supplied separate-view dependency review.

## Documentation.md update status

Current task-local introduction now says W3 active and W2 retained; source/
lifecycle/alpha gaps explicit. Other subsystem boundaries remain preserved.

## docs/project-status.md update status

更新済み: owner-requested W3 activation and the actual first-dependency evidence.

W3 activation and actual first-dependency evidence mirrored. Official lifecycle
and Plan250 owner pause remain unchanged.

## progress.md update status

W3 current position and actual timestamped recent log updated. Existing macro/
feature/subsystem axes retained; source path and all-mutator gate remain open.

## tasks.md update status

Whole snapshot rewritten for one W3 goal, dependencies within it, owner-policy
versus research questions, evidence class, stop/reopen and future scopes.
Historical estimate explicitly uncalibrated; no Oracle deadline introduced.

## samples_progress.md update status

Updated W3 candidate row, exact fresh-copy reproduction command and remaining
source/lifecycle/review gate; stale W2-active introduction corrected. Sample/Lean/
script indexes updated together. No operational alpha promotion.

## Reviewer findings and follow-up

Entry and material dependency reviews are read-only main-operated Oracle advice,
not kernel execution, an owner decision or a signature. Main fully read and
checked both answers, with question/answer/disposition hashes preserved.
A1 mutation-target weakness repaired; A2 declaration audit strengthened and tested;
A3 no-issuance claim clarified with pre-grant control. B1 checked-catalog/fresh-row
and namespace provenance, B2 saved witness versus fresh search, B3 full revision
binding, B4 support versus auth/query/destination footprints remain explicit
consumer obligations. No literal theorem countermodel was found by Oracle.
Narrow review completed without a blocking theory repair. Its remaining low-severity
diagnostic-specificity finding was locally verified and fixed; fresh11/19 and
1716-declaration audit plus exact-root/axiom and positive controls passdf6040.
Full actual source/lifecycle consumer review remains required. No subagent or independent signed reviewer invented.

## Skipped validations and reasons

No full W2/Rust/network rerun: corresponding sources are unchanged; actual newly
used Lean dependencies were compiled. No W3 physical deployment or source/runtime
refinement claim. All current11/19 kernel controls and declaration audit ran.
Mandatory global reading remains incomplete; no whole-project plan is adopted.
Document validation14970 passed493dec (1763 numbered reports), followed by minor
review-receipt synchronization; final checkpoint metadata checks remain. External lifecycle
proofs have no final material review or source/auth/result integration yet.

## Commit / push status

No W3 commit yet. Own coherent candidate checkpoint will be committed with
--no-gpg-sign, normally pushed and parity checked under the user's authorization.
No force/reset/clean. External lifecycle drafts remain external research until
coherent mirroring; the dependency checkpoint is not W3 acceptance or completion.

## Sub-agent session close status

None created. W3 goal active, task continues; no final completion notification.
