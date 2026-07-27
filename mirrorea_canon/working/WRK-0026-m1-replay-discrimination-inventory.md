---
id: working/WRK-0026
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority, spec/05-runtime-semantics, meta/proposal-012, meta/proposal-013]
summary: M1 request-local validation claims と既存 authority/history 文言が、同一 claims を持つ二つの request を replay と別個の正当要求に分類する semantic relation を既に供給するかを literal inventory で検査する。request identity、replay policy、Core、runtime は選ばない。
open_items: []
---

# WRK-0026 - M1 replay-discrimination inventory

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@884b20c4f381bdcf7e042bf6eef30d1eca49f700:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/01-mircore-v0@884b20c4f381bdcf7e042bf6eef30d1eca49f700:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/04-ordering-and-cuts@884b20c4f381bdcf7e042bf6eef30d1eca49f700:70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264, theory/05-authority@884b20c4f381bdcf7e042bf6eef30d1eca49f700:e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4, spec/05-runtime-semantics@884b20c4f381bdcf7e042bf6eef30d1eca49f700:25749e3b171659fa59e3de6ff49126e15331ef52cf3ba5337ece4c46e72ca06c, meta/proposal-012@884b20c4f381bdcf7e042bf6eef30d1eca49f700:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5, meta/proposal-013@884b20c4f381bdcf7e042bf6eef30d1eca49f700:4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@884b20c4f381bdcf7e042bf6eef30d1eca49f700:44c5e7e88bd68a6000abac7be9553f4362f0f276d5a8dfd52c46521c045cc7e0, LAB:plan/193-post-admission-validation-context-literature-and-counterexample-memo.md@884b20c4f381bdcf7e042bf6eef30d1eca49f700:82e36cee61cc92311dc93b373c80182d43de4524684b07d8e6f78fd6c6cb94da
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: Do M1's request-local claims plus the currently displayed request,
queue, authority, history, and runtime rules define a semantic relation that
classifies two requests with equal displayed validation claims as either the
same replayed request or distinct intended requests? If not, can a literal
inventory establish only that replay rejection needs an additional selected
identity/correlation/policy relation before a proof or runtime claim relies on
it?
Status quo: Theory/05 requires copied/replayed capability references to be
rejected. P013 M1 requires copied/replayed requests to be rejected without a
store mutation and explicitly does not select a request-instance identity,
occurrence identity, queue carrier, or hidden correlation. The displayed Core
request contains endpoint, operation, values, refs, and failure row; its queue
is per-locus. None of those statements yet says whether equal valid request
claims denote a retry of one action or two separately intended actions.
Alternative: the pinned texts already define a request-instance identity,
deduplication/replay predicate, or a policy that classifies equal claim tuples
without using transport identity or another unselected relation.
Expected falsifier: Any pinned digest differs; the literal inventory finds a
defined request identity, duplicate/replay predicate, or equal-claims policy;
the existing authority/history relation already distinguishes the two cases; or
retaining the result requires choosing a request field, queue/event/history
carrier, replay policy, Core/OBL/theory/11/SCN/Gate/Phase change, helper/schema/
CI, or public contract.
Rollback / reopen trigger: On any falsifier set `Reliance status` to `frozen`,
retain only reproducible procedure evidence, and do not repair or rerun this
record. Escalate rather than repair if a follow-up chooses a semantic request
identity, duplicate policy, receipt correlation, queue/history representation,
replay behavior, Core/authority primitive, SCN/Gate/Phase, implementation, or
public interface.

## Method and evidence plan

Result class: literal-transcription
Commands: rg -n -C 3 'request\(|request queues|Copied / replayed|Post-admission messages carry|request-instance identity|copied/replayed requests|history facts|Request lifecycle|send → receive|duplicate|retry' mirrorea_canon/theory/01-mircore-v0.md mirrorea_canon/theory/04-ordering-and-cuts.md mirrorea_canon/theory/05-authority.md mirrorea_canon/spec/05-runtime-semantics.md mirrorea_canon/meta/proposals/PROPOSAL-012-mircore-value-flow-and-occurrence-identity.md mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md; python3 -c "from pathlib import Path; sources = {p: Path(p).read_text() for p in ('mirrorea_canon/theory/01-mircore-v0.md', 'mirrorea_canon/theory/05-authority.md', 'mirrorea_canon/spec/05-runtime-semantics.md', 'mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md')}; required = {'mirrorea_canon/theory/01-mircore-v0.md': ('request(', 'per-locus request queues'), 'mirrorea_canon/theory/05-authority.md': ('Post-admission messages carry', 'Copied / replayed'), 'mirrorea_canon/spec/05-runtime-semantics.md': ('Request lifecycle', 'Owner seriality'), 'mirrorea_canon/meta/proposals/PROPOSAL-013-post-admission-request-validation-context.md': ('request-instance identity', 'copied/replayed requests')}; assert all(all(token in sources[p] for token in tokens) for p, tokens in required.items())"; git diff --check
Execution cut: `884b20c4f381bdcf7e042bf6eef30d1eca49f700` is the authority/input snapshot. Execute every outcome command only after this registration is committed and pushed. The evidence commit may add only `plan/wrk-0026-m1-replay-discrimination-inventory.md`, its `plan/00-index.md` entry, a direct numbered report, allowed working-record metadata/control files, and no helper, schema, CI/Make, runtime, parser, checker, theory, contract, or public artifact. A later metadata-only commit may append the exact evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not assert exactly-once delivery, require retry support,
define a request identity, choose whether identical requests are permitted,
change replay rejection, select a queue/history/event carrier, add a Core term,
modify authority/failure/receipt semantics, or alter OBL/theory/11, SCN,
Gate/Phase, conformance, runtime, transport, API, or public behavior.

## Results and review

Reliance status: not-promoted
Positive evidence: Pending registration and the registered commands.
Negative evidence: Pending registration and the registered commands.
Evidence artifacts: none
Evidence commits: none
Impact / non-effects: Until an outcome is retained, this record only fixes a
read-only replay-discrimination question. It does not equate claim equality
with request identity or treat transport delivery as authority.
Independent review: not-required-for-L3

## Supersession

Supersession: none
