---
id: working/WRK-0039
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/04-ordering-and-cuts, theory/05-authority, meta/proposal-012, meta/proposal-013, working/WRK-0038]
summary: C2-B/C3 の Canon carrier を選ばず、WRK-0037 の全十 supplied key ごとに independently enumerated relation graph と bundled lookup の fiberwise translation を検査する。key recovery、identity、authority、persistence、source rule、runtime は除外する。
open_items: []
---

# WRK-0039 - C2-B/C3 fiberwise relational presentation experiment

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@1041505a5979591414ef29e4f850e9d6ab52f28a:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, meta/proposal-012@fcf5ea613c2153667e1c4a887589fb939692c7a5:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5, meta/proposal-013@fcf5ea613c2153667e1c4a887589fb939692c7a5:4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213, theory/01-mircore-v0@a51ab57b2df121186029dfae09a8206cee1f6702:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/04-ordering-and-cuts@a51ab57b2df121186029dfae09a8206cee1f6702:70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264, theory/05-authority@a51ab57b2df121186029dfae09a8206cee1f6702:e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4, working/WRK-0038@5b876c523b83bcdb2774eb17f2e12ee264338719:ddcabc21d3be50c43ac651d5ce8cbdd4311d87f00c17da16ecd8d1492228d88c
LAB inputs: LAB:plan/213-c2b-c3-fiberwise-relational-comparison-selection.md@6e366925b185f49a7c214b86e3cfb4f564bbca69:45d328257f6841049b292d3d895ac87bcb37e57b457c6bc1c4f856caa00e7a0a, LAB:plan/wrk-0037-c2b-c3-b-primary-opaque-anchor-experiment.md@99f468d6d5e415ed05f90b77c2b37956102fdc36:839ffda0e4c01fb1dab476598b97f658a8f85e27d8ce2547ab6a8c49e8662739
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: Can five independently enumerated finite graph relations over exactly WRK-0037's existing types and all ten supplied `(Frontier, Request)` keys form a relation fiber that has total pointwise translations to and from the bundled lookup fiber at the same supplied key? The relations are `CellR`, `IncidentalR`, `ReceiptResultR`, `ResumeResultR`, and `RestoreR`; they must enumerate every cell/outcome/swap row directly, use no bundled lookup in a definition, and derive combined receipt-then-resume only from receipt and resume rows. The translations receive frontier and request as indices and preserve every view column, all accepted and rejected transition results, derived combined behavior, and local restore lookup. No function or theorem recovers a key from a bare view or incidental observation, or denotes a Mir carrier, identity/equality, authority, freshness, persistence, recovery, source rule, runtime behavior, or public contract.
Status quo: WRK-0037 is one two-atom finite lookup table with equal incidental data, explicit staged fields, exact receipt/resume tables, and local restore. WRK-0038 is unexecuted because bare views collide across supplied keys and the old table has no reachability closure. Plan 213 selects only a successor whose domain is all ten supplied fibers, not a global or key-reconstructing inverse.
Alternative: Retain no relation comparison because independent graphs cannot be enumerated, require a hidden premise, or are only a repackaging of a bundled view. In that event, do not materialize source; retain the scope correction and return to `no-candidate` for this finite comparison.
Expected falsifier: Any graph row is missing, extra, nonfunctional, or defined through `DirectView`, a lookup, a row/profile key, a copied view record, phase shortcut, or missing-proof-as-rejection; an all-ten-cell map is partial or either round trip fails; a view column, incidental row, receipt/resume result including `none`, derived combined result, or restore lookup differs; a translation recovers a key from a bare view or incidental data; the exact WRK-0037 baseline differs; or any selected carrier, identity, authority, freshness, persistence, recovery, source/elaboration, helper/schema/CI/Make, THM/OBL, SCN, Gate/Phase, runtime, or public premise is required.
Rollback / reopen trigger: On any typed finite falsifier, set `Reliance status: frozen`, retain the procedure and falsifier only in declared LAB locations, and do not repair this record into a positive result. On a repackaging finding, close as duplicate rather than enlarging the table. Syntax, extraction, or toolchain failure is inconclusive infrastructure evidence. A changed authority cut, actual carrier/identity selection, source/elaboration artifact, contract, theorem/OBL, helper/schema/CI surface, runtime, or public behavior requires a forward successor or ordinary Canon escalation.

## Method and evidence plan

Result class: existing-lane-experiment
Commands: Registration check, run before this record is created: `test ! -e mirrorea_canon/working/WRK-0039-c2b-c3-fiberwise-relational-presentation.md`. Outcome commands, run only after this registration is committed and pushed: exact source/digest checks for every Canon and LAB input above; extract the WRK-0037 Lean block and verify its known source SHA-256 `f80ece6b9b74985120e9016567a5543914c55006f5cae1ec01ade4d5c416bd5a`; extract the successor block, compare its marked pinned baseline byte-for-byte with that source, and reject any relation-definition reference to `DirectView`, `directView`, `phaseAt`, `validationAt`, `replyAt`, `receiptAt`, `failureAt`, `heldAt`, `resultAt`, `provenanceAt`, `resumeAt`, `dependencyAt`, `mutationAt`, `receiptExtension`, `resumeExtension`, `receiptThenResume`, `restore`, or `loadedView`; run duplicate/reserved-surface scans; extract and compile the sole successor block with `lean --trust=0`; inspect every retained theorem with `#print axioms`; scan extracted source for `sorry`, `admit`, `unsafe`, `partial`, `implemented_by`, `Classical`, `Choice`, `Quotient`, `Quot.sound`, `native_decide`, and `axiom`; enforce the declared evidence-commit allowlist; and run `git diff --check`.
Execution cut: `7f245eca1c2c40422adf806dd2bce65fed98dcc3` is the authority/input snapshot. Run every outcome command only after this registration is committed and pushed. The evidence commit may add only `plan/wrk-0039-c2b-c3-fiberwise-relational-presentation-experiment.md`, a direct numbered report, declared control files, and no helper, schema, validator, CI/Make surface, parser, checker, theory, contract, runtime, sample, or public artifact. The fenced Lean source is materialized only to disposable temporary files. It is not a stable module, schema, data model, validator input, or downstream interface. A later metadata-only commit may append the exact evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not select Family A, B, or C; define any Mir request/attempt/occurrence identity, equality, correlation, authority, freshness, persistence, recovery, source grammar/elaboration/omission rule, delivery/retry/fairness/timeout/cancellation behavior, implementation, API, contract, THM/OBL, SCN, Gate, Phase, conformance, lifecycle, or public claim. A passed finite Lean proposition would be machine-checked only for the exact artifact-local fibers, not a Canon theorem, general semantic equivalence, family adequacy proof, source-inference authorization, or implementation readiness.

## Results and review

Reliance status: not-promoted
Positive evidence: Every registered outcome command completed after the registration push. At the pinned authority/input cut, the marked WRK-0037 baseline matched byte-for-byte, and the artifact-local successor source with SHA-256 `468563ff31258b1010e4f22c73b3751a0427c6ce40f8548d09afa18dde049208` passed Lean 4.29.1 at `--trust=0`; all 35 declared theorem reports have no axioms. The independent direct graphs enumerate all ten cell rows, two incidental rows, twenty receipt results, ten resume results, two restore rows, and the receipt/resume-derived combined relation. Cell/view fibers have pointwise round trips preserving every `DirectView` column; separate finite soundness/completeness/commutation propositions cover incidental, receipt, resume, combined, and restore observations.
Negative evidence: No registered typed falsifier occurred. Registered input digests, relation-isolation, placeholder, duplicate/reserved-surface, evidence-commit allowlist, and diff checks passed. Two advisory Oracle reviews found evidence-retention, scan-boundary, wording, and coverage-reporting defects that were corrected before this metadata link; a third acceptance-review response timed out and supplied no semantic finding. The retained result is one finite supplied-key table, not a general recovery, restore, carrier, identity, or source-inference result.
Evidence artifacts: LAB:plan/wrk-0039-c2b-c3-fiberwise-relational-presentation-experiment.md@f250e117ffd4c7f1b81a1d604900ff63973cd582:bfbc66cf7fea87bdebb42e0412dd9e6c9279fd8dcfd24c672ebac5150c9aa229
Evidence commits: f250e117ffd4c7f1b81a1d604900ff63973cd582
Impact / non-effects: This record is normative only about a reversible L3 research boundary and procedure. The finite pass establishes only that this explicitly bounded supplied-key candidate did not trigger its registered falsifiers; it cannot make either presentation a Canon carrier, define request identity, authorize source omission/inference, establish a persistence/recovery rule, or establish an implementation result.
Independent review: not-required-for-L3

## Supersession

Supersession: none
