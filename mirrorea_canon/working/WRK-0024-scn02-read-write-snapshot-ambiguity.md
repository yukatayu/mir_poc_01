---
id: working/WRK-0024
status: L3-open
maturity: draft
depends_on: [adr/ADR-0014, theory/01-mircore-v0, theory/03-elaboration, spec/05-runtime-semantics, scenarios/SCN-02, meta/proposal-012]
summary: SCN-02 の cross-locus read-dependent write について、owner-serial mutation だけでは二つの read reply 後の stale write を排除できないことを最小非production countermodel で検査する。snapshot、pending carrier、Core rule、SCN、OBL は選ばない。
open_items: []
---

# WRK-0024 - SCN-02 read/write snapshot ambiguity

## Classification and authority cut

Standing eligibility: pass
Author: codex
Author fingerprint: not-required-for-L3
Canon anchors: adr/ADR-0014@fcf5ea613c2153667e1c4a887589fb939692c7a5:b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323, theory/01-mircore-v0@fcf5ea613c2153667e1c4a887589fb939692c7a5:35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12, theory/03-elaboration@fcf5ea613c2153667e1c4a887589fb939692c7a5:2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641, spec/05-runtime-semantics@fcf5ea613c2153667e1c4a887589fb939692c7a5:25749e3b171659fa59e3de6ff49126e15331ef52cf3ba5337ece4c46e72ca06c, scenarios/SCN-02@fcf5ea613c2153667e1c4a887589fb939692c7a5:ec01c1552779cfd8107df60fe7a59520f0e1344bae655626821383292db0bc5b, meta/proposal-012@fcf5ea613c2153667e1c4a887589fb939692c7a5:09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5
LAB inputs: LAB:plan/199-selected-semantic-composition-and-inference-boundary.md@fcf5ea613c2153667e1c4a887589fb939692c7a5:8e0266267cf933b2c320932afb0925246cac08346cbf86747550c4a93caeca0e
Permitted LAB locations: plan, docs/reports
Reserved surfaces: excluded

## Pre-registered working question

Question: Under only the displayed premises that cross-locus reads may yield a
read result, owner queues serialize store mutations, and no selected snapshot,
read-modify-write, or pending-control relation is present, can two SCN-02-like
attacks both read HP=10 and then be serially served as writes of 7 and 6?
Does that finite trace show that owner seriality alone cannot establish the
intended combined 10 -> 3 update?
Status quo: SCN-02 requires dependency rows for both target HP and attacker
ATK, while theory/03's worked shape only exposes the ATK cross-locus read and
emits a write request with computed `vprime`. Theory/01 says `[E-SERVE]` may
perform `read+reply` and serializes owner mutation, but OPEN-011 leaves the
reply/receipt carrier unresolved. P012 records V1/R1 but does not amend a rule
or choose a snapshot/evaluation locus. No current text identifies an atomic
read-modify-write semantics for the assignment.
Alternative: the pinned Canon text already forces the target read and dependent
write to share one owner-serialized snapshot/atomic evaluation, or it otherwise
forbids both replies before either write. In that case the proposed finite
countermodel does not apply.
Expected falsifier: Any pinned digest differs; the pre-source marker already
exists; the minimal countermodel fails to typecheck or proves a different final
state; the pinned text itself supplies the missing snapshot/atomicity relation;
or retaining the result requires a Core constructor, grammar, pending/request
identity, queue carrier, SCN change, OBL/theory/11 change, helper/schema/CI,
or public contract.
Rollback / reopen trigger: On any falsifier set `Reliance status` to `frozen`,
retain only reproducible procedure evidence, and do not repair or rerun this
record. Escalate rather than repair if the next step chooses snapshot timing,
evaluation locus, read-modify-write fusion, reply/pending/request identity,
queue behavior, SCN expectation, theorem/OBL status, Gate/Phase, or an
implementation/public contract.

## Method and evidence plan

Result class: countermodel
Commands: lean --version; test ! -e /tmp/mirrorea-wrk0024-scn02-snapshot/Scn02SnapshotAmbiguity.lean; lean --trust=0 /tmp/mirrorea-wrk0024-scn02-snapshot/Scn02SnapshotAmbiguity.lean; python3 -c "from pathlib import Path; text = Path('/tmp/mirrorea-wrk0024-scn02-snapshot/Scn02SnapshotAmbiguity.lean').read_text(); required = ('stale_final_is_six', 'atomic_final_is_three', 'owner_seriality_alone_does_not_imply_atomic_result'); forbidden = ('sorry', 'admit', 'axiom', 'unsafe', 'partial', 'implemented_by', 'Classical', 'Choice'); assert all(name in text for name in required); assert not any(token in text for token in forbidden)"; rg -n -C 3 '\[READ-CROSS\]|\[E-SERVE\]|Owner queues are served serially|read\+reply|OPEN-011' mirrorea_canon/theory/01-mircore-v0.md mirrorea_canon/theory/03-elaboration.md mirrorea_canon/spec/05-runtime-semantics.md mirrorea_canon/scenarios/SCN-02-attack.md; git diff --check
Execution cut: `fcf5ea613c2153667e1c4a887589fb939692c7a5` is the authority/input snapshot. Execute the pre-source marker check and every outcome command only after this registration is committed and pushed. The scratch source stays outside the repository. The evidence commit may add only `plan/wrk-0024-scn02-read-write-snapshot-ambiguity.md`, its `plan/00-index.md` entry, a direct numbered report, allowed working-record metadata/control files, and no helper, schema, CI/Make, runtime, parser, checker, theory, contract, or public artifact. A later metadata-only commit may append the exact evidence commit and artifact digest without rewriting this pre-registration.
Non-claims: This does not claim that the illustrative stale trace is a complete
Canon execution, select a snapshot point/evaluation locus/read-modify-write
primitive, change SCN-02's expected result, create a request/reply/pending
carrier, change owner seriality, alter a diagnostic/failure row, discharge an
OBL, change `theory/11`, Gate/Phase, conformance, runtime, queue, persistence,
transport, API, or public behavior.

## Results and review

Reliance status: not-promoted
Positive evidence: After registration `2a08b2f2dba9060138d99152dfdd22d29ca3674f`
was pushed, the registered marker confirmed that the scratch source did not
exist. Lean 4.29.1 compiled the finite model with `--trust=0`; the registered
required-name and forbidden-token audit also passed. It proves
`ownerSerialFinal(10, [7, 6]) = 6`, `atomicDamageFinal(10, [3, 4]) = 3`, and
their inequality. The scratch source SHA-256 is
`9c02e90a8accaf156dffd4ee14c9fc10052a8d6f16b2ec6e82fca85b99b15cac`.
Negative evidence: No registered falsifier occurred. The pinned source audit
does not state a snapshot, owner-side evaluation, read-modify-write fusion, or
pending relation that would make the finite model a complete Canon trace or
rule it out. This is a displayed-boundary observation, not an absence claim
about future Canon design.
Evidence artifacts: LAB:plan/wrk-0024-scn02-read-write-snapshot-ambiguity.md@e7fe81004759b02adf84661d13e09690ded156f8:dbd5bdf895ac34640c27a09dd051b2cebdc90db14ebd7836d1f8d9d1dc6d55f1
Evidence commits: e7fe81004759b02adf84661d13e09690ded156f8
Impact / non-effects: The retained Plan artifact records only the finite
non-implication from owner-serial submitted writes to atomic read-dependent
updates. It neither selects nor interprets the Canon as choosing an
asynchronous reply, stale-read schedule, fused atomic operation, snapshot,
pending carrier, or repair; it changes no SCN, Core, OBL, lifecycle,
implementation, or public behavior.
Independent review: not-required-for-L3

## Supersession

Supersession: none
