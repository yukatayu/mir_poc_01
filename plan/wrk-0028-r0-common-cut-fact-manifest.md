# WRK-0028 R0 - Common-cut source-local fact manifest

## Role and scope

This is non-production LAB evidence for
`mirrorea_canon/working/WRK-0028-r0-common-cut-fact-manifest.md`. It records
only the pre-enumerated C0/C2 source spans at Canon cut
`4ee275507000b905e46c6b5389865f7c0985ab79`. It is a reader-oriented Markdown
layout, not a schema, API, validator input, or shared semantic model.

Each row records what the cited source literally says about its own present
role. A row never resolves conflicts, translates two sources into a new shared
proposition, or supplies semantics the cited source leaves open. Historical
WRK results are deliberately excluded; later work must pin each original
record and evidence cut separately if it needs one.

## C0 source-local rows

### C0-01 - Current lexical vocabulary

- Source: `spec/01-lexical-and-modules`, L1-fixed, ``Not reserved`` note.
- Literal: ``Not reserved: ... `Participant`(keyspace name, declared)``.
- Role: current Canon lexical fact at this cut.
- Bounded reading: the current lexical document does not reserve `Participant`.
- Non-claim: this does not choose a Surface keyspace grammar, resolution rule,
  Core representation, or authority meaning.

### C0-02 - Current Surface forms

- Source: `spec/02-surface-grammar`, L1-fixed, `StateDecl` and `Stmt`.
- Literal: `StateDecl ::= "state" Ident "[" Ident ":" Keyspace "]" ":" Type`
  and `Stmt ::= ... | Return | ...`.
- Role: current Canon grammar facts at this cut.
- Bounded reading: displayed state declarations are indexed, and `return` is
  still displayed in the current `Stmt` production.
- Non-claim: this does not assert that every displayed production has a
  complete Core counterpart, select its rejection behavior, or preserve it in
  a future exact fragment.

### C0-03 - Current static indexed-state account

- Source: `spec/03-static-semantics`, L1-fixed, obligation 2.
- Literal: `state x[k:K]: A at ell denotes an ell-owned partial map
  Active(K, epoch) -> A`.
- Role: current Canon static-semantics fact at this cut.
- Bounded reading: the stated static account is indexed and key authority is
  explicitly rejected.
- Non-claim: this does not supply a scalar-state correspondence or a general
  keyspace declaration mechanism.

### C0-04 - Current elaboration contract

- Source: `theory/03-elaboration`, L1-fixed, BND-001.
- Literal: a `well-scoped Surface item` either produces the displayed tuple
  `or a Diagnostic`; clause 5 says elaboration is a function of its inputs.
- Role: current Canon contract wording at this cut.
- Bounded reading: outcome wording and determinism wording are distinct spans.
- Non-claim: this does not choose the exact `WellScoped` domain, a result or
  Diagnostic equality, an obligation identity, or a proof status.

### C0-05 - Current Diagnostic working carrier

- Source: `theory/10-diagnostics`, L2-working, `Carrier`.
- Literal: `Diagnostic = { id, span, rule_instance, failed_premise,
  missing_evidence, suggested_repair, refs }`.
- Role: current Canon **working** statement, with its L2 status retained.
- Bounded reading: the current working text exposes a Diagnostic carrier and
  named-premise blame principle.
- Non-claim: this does not freeze the carrier, prove diagnostic completeness,
  allocate a new error family, or decide C0's totality domain.

### C0-06 - P004 direction and limitation

- Source: `meta/proposal-004`, L3-open, `Owner disposition` and non-effects.
- Literal: `A accepted — Participant-only closure`; it `does not itself amend
  spec/01, spec/02, spec/03, a scenario, a parser, or a checker`.
- Role: Canon-recorded bounded proposal direction, not a current grammar rule.
- Bounded reading: a later wording package may work on the stated closure.
- Non-claim: candidate EBNF/detail is not present `spec/02`, and no parser,
  Core, diagnostic, scenario, or implementation state is selected here.

### C0-07 - P008 direction and limitation

- Source: `meta/proposal-008`, L3-open, `Owner disposition`.
- Literal: `A accepted — separate totality obligation`; the later package must
  select the domain, `WellScoped` predicate, result/Diagnostic abstraction,
  and explicit obligation placement before any Canon amendment.
- Role: Canon-recorded bounded proposal direction, not a current OBL change.
- Bounded reading: outcome existence is directed to remain separate from the
  OBL-021 determinism boundary in a later package.
- Non-claim: this does not choose totality's domain, equality, Diagnostic ABI,
  theorem wording, ledger identity, or proof status.

### C0-08 - P015 closure direction and limitation

- Source: `meta/proposal-015`, L3-open, `Owner disposition` and required
  follow-up boundary.
- Literal: scalar/terminal correspondence is explicit, while `return` is
  excluded from the future v0 exact fragment; scalar representation remains
  `UNRESOLVED`.
- Role: Canon-recorded bounded proposal direction, not a current grammar edit.
- Bounded reading: a later package must make the named correspondence and
  rejection policy explicit.
- Non-claim: this does not alter current `StateDecl`/`Return`, choose a scalar
  cell or finite-domain elaboration, infer a default, or change SCN-08.

## C2 source-local rows

### C2-01 - Current Core request and serve wording

- Source: `theory/01-mircore-v0`, L1-fixed, Core grammar and `[E-SERVE]`.
- Literal: `request(ell_src -> ell_own, op, values, caprefs, witrefs,
  failures)`; `[E-SERVE]` validates epoch, incarnation, lineage, witnesses,
  and visibility.
- Role: current Canon Core/rule wording at this cut.
- Bounded reading: request parameters and owner-side validation are displayed.
- Non-claim: this does not choose semantic request identity, payload equality,
  replay/retry policy, binding carrier, queue encoding, or persistence model.

### C2-02 - Current authority distinction

- Source: `theory/05-authority`, L1-fixed, `Claims vs grants` and
  post-admission messages.
- Literal: `role claim != authority`, `capability grant == authority`; messages
  carry principal, epoch, incarnation, capability refs, and required witness
  refs, while stale messages are rejected.
- Role: current Canon authority fact at this cut.
- Bounded reading: claims, grant lineage, and listed validation facts are not
  interchangeable with transport/locus/key identity.
- Non-claim: this does not define request-instance identity, duplicate
  classification, a receipt correlation, or save/load replay semantics.

### C2-03 - P012 direction and limitation

- Source: `meta/proposal-012`, L3-open, `Owner disposition`.
- Literal: `V1`, `R1`, `SW1`, and conditional `A2` are recorded; the text says
  they do not add a Core constructor, result/reply carrier, transition rule,
  occurrence schema, OBL, runtime, wire protocol, or public contract.
- Role: Canon-recorded bounded proposal direction, not an integrated Core or
  event model.
- Bounded reading: later comparison must make binding, pending, correlation,
  facet, failure, cut/save-load, DAG, and linearity relations explicit.
- Non-claim: this does not select any carrier, projection rule, identity, or
  implementation, and conditional A2 is not unconditional.

### C2-04 - P013 M1 direction and limitation

- Source: `meta/proposal-013`, L3-open, `Owner disposition` and non-effects.
- Literal: `M1 accepted — request-local validation context`; claims remain
  non-authoritative and the disposition selects neither request-instance
  identity, queue carrier, occurrence identity, transport session, wire
  envelope, nor persistence encoding.
- Role: Canon-recorded bounded proposal direction, not an existing request
  representation or replay policy.
- Bounded reading: later work may compare validation-claim presentations only
  against authoritative membership, lineage, witness, admission, visibility,
  and history facts.
- Non-claim: this does not make claims authority, supply hidden correlation,
  decide equality/replay, or modify `[E-SERVE]` semantics.

## Result

All pre-enumerated rows can retain their source-local role and limitation at
the pinned cut. No row needed a precedence rule, semantic reconciliation,
stable schema, helper, validator, or reserved decision. This is a retained L3
provenance result only. It does not establish C0/C2 compatibility or select a
shared model. The next candidates, if separately eligible, are C0-A source
authority and C2-A equality vocabulary.
