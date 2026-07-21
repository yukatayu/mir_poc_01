---
id: working/readme
status: L1-fixed
maturity: draft
depends_on: [adr/ADR-0014, meta/proposal-006]
summary: 可逆な L2/L3 research proposition の canonical working annex。WRK-#### の必須記録、review、supersession、非宣言を定める。
open_items: []
---

# working - Reversible research annex

`working/` is the only canon location for an agent-maintained current research
proposition under ADR-0014. It records a bounded L3 hypothesis or a reviewed L2
working position. It is not a second theory ledger, a place for settled
semantics, or authority to alter another canon file.

## Record identity and lifecycle

- Records are named `WRK-####-short-topic.md`; `WRK-####` is registered in
  `MAP.md` and this directory is its ledger.
- A record begins `L3-open`. Its pre-registration is committed before outcome
  evidence is relied on. It can start without independent review. A record can
  become `L2-working` only after the exact frozen-cut independent review in
  ADR-0014 and an owner-authenticated trust anchor exists. Until then, every
  L2 promotion is intentionally fail-closed.
- Every record contains exactly one `Reliance status:` marker in **Results and
  review**. New L3 records use `not-promoted`; reviewed L2 records use `active`.
  A reproducible falsifier changes the marker to `frozen` immediately, without
  waiting for review. No consumer may rely on a frozen record as an L2 position;
  retain it and record the durable follow-up as a forward L3 successor or
  escalation bundle rather than an in-place demotion. A record's `WRK-####`
  identity and path are immutable through reachable Git history.
- Record a superseding or falsified successor rather than deleting the prior
  evidence. Never convert LAB success into a proof, Gate, Phase, SCN,
  implementation, or public claim.

## Required WRK sections

Every record contains these sections in this order:

1. **Classification and authority cut**: `Standing eligibility: pass`,
   `Author: <id>`, `Author fingerprint: <40-hex> | not-required-for-L3`,
   `Canon anchors: <id>@<40-hex-commit>:<64-hex-SHA-256>`, `LAB inputs:
   LAB:<path>@<40-hex-commit>:<64-hex-SHA-256>`, `Permitted LAB locations:
   ...`, and `Reserved surfaces: excluded`.
2. **Pre-registered working question**: `Question:`, `Status quo:`,
   `Alternative:`, `Expected falsifier:`, and `Rollback / reopen trigger:`.
3. **Method and evidence plan**: `Result class:` (`reproduction`,
   `literal-transcription`, `countermodel`, `conditional-lemma`, or
   `existing-lane-experiment`), `Commands:`, and `Non-claims:`.
4. **Results and review**: the exact `Reliance status: not-promoted | active |
   frozen` marker, plus `Positive evidence:`, `Negative evidence:`, `Evidence
   artifacts: LAB:<path>@<40-hex-commit>:<64-hex-SHA-256>`, `Evidence commits:
   none | <40-hex-commit>(, <40-hex-commit>)*`, `Impact / non-effects:`, and
   `Independent review:`. The evidence-commit list starts as `none`; after an
   L3 record retains an artifact, it appends the unique full commit IDs that
   own those artifacts and never removes or rewrites an earlier entry. An
   artifact snapshot names one listed evidence commit. An L2 record has
   `maturity: reviewed`, completed positive/negative evidence and artifact
   snapshots, and an exact `Independent review: reviewer-fingerprint=<40-hex>;
   frozen-base=<40-hex>; record-sha256=<64-hex>; decision=approved` binding.
   The recorded base must be signed by `Author fingerprint`; the normalized
   record SHA-256 must match; and the admission commit directly atop that base
   must verify with the distinct reviewer fingerprint. An L3 record uses
   `Independent review: not-required-for-L3`; it is a successor, not an
   in-place rollback of an existing L2 record.
5. **Supersession**: `Supersession:` with a forward reference, `none`, or an
   escalation bundle.

The front matter is exactly `id: working/WRK-####`, `status: L3-open` with
`maturity: draft` or `status: L2-working` with `maturity: reviewed`. The
pre-registration is not rewritten to fit later results. Clarifications are
recorded as dated addenda or successor records. A record must remain concise;
full source, command output, alternatives, and generated artifacts remain LAB.

Every current WRK record must be committed at `HEAD` before it is valid. The L2
admission is intentionally a post-commit validation: the author first signs
the frozen L3 base, then a distinct reviewer signs the exact L2 admission
directly atop it. Run `make docs` immediately after that admission. The
validator locates that signed admission in history, so a later unrelated commit
does not invalidate it. A later content edit invalidates the normalized digest;
start a new review rather than editing the reviewed position in place.

## Review trust root

`meta/review-keys.json` is a non-authoritative placeholder for a future
owner-controlled operational trust registry. Agents cannot edit it through this
route. Its
`activation` is intentionally `UNRESOLVED`, so the validator rejects every L2
promotion until an owner-authenticated trust anchor is established by a
separate canon action. L3 research is available immediately. Git commit
signatures are a candidate replaceable verifier adapter, not a Mir primitive or
a transport/authentication semantic.

## Eligible and reserved work

An agent may create or revise an L3 record only when all of the following are
true: it reads existing canon anchors without changing them; it uses an existing
documented LAB lane; it declares non-effects, alternative, and falsifier first;
and it needs no new helper family, schema, CI/Make surface, evidence lane, or
public interface. Existing-lane code and tests are allowed as research evidence
when they stay explicitly non-production and pass their documented validation.
A registration commit contains only its new WRK and exact operational metadata.
Retained source or test evidence is attributable only through the append-only
`Evidence commits:` list; each listed commit stays inside the declared
`Permitted LAB locations` plus exact operational metadata. An unlisted
independent commit is not evidence for the WRK. Operational metadata is only
the exact status/index/MAP/changelog files and direct numbered Markdown reports.
A new helper, schema, CI/Make surface, another WRK, `working/README.md`, or
other source outside the declared lane is rejected when it appears in a
registration or listed evidence commit. Authoritative validation uses a clean
disposable worktree; normal local validation does not make dirty files evidence.

L0/L1 changes, core or authority primitives, contracts, SCN/Gate/Phase or
lifecycle actions, every `theory/11` movement, final proof / OBL discharge,
and public claims are reserved. A record that touches, conflicts with, or cannot
clearly exclude a reserved surface is `escalated`; do not use `working/` to
smuggle the decision through a smaller wording change. Keep all `LAB inputs` and
`Evidence artifacts` under the declared `Permitted LAB locations`; a broader
location needs a successor pre-registration, not an unrecorded input.
