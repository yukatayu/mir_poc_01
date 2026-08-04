---
id: theory/01-mircore-v0
status: L1-fixed
maturity: draft
depends_on: [theory/00-overview, theory/02-types-effects-failures, adr/ADR-0003, adr/ADR-0004, adr/ADR-0005]
summary: MirCore v0 計算体系。抽象構文、実行構成、統合判定、小ステップ操作意味論、定理 statement の所在。
open_items: [OPEN-010, OPEN-012]
---

# 01 — MirCore v0

This chapter fixes the minimal calculus. It unifies the two judgments that were
separate in LAB (`LAB:specs/19` checking, `LAB:specs/39` elaboration) into one.
Everything here is syntax-independent. The displayed `Surface` alternatives
are semantic categories rather than a selected concrete grammar; spec/02 is
the retained pre-M6 candidate and M6 chooses the final bounded grammar.

## 1. Abstract syntax

```text
Loci        ℓ ∈ Locus                      declared locus names
Principals  π ∈ Prin
Keyspaces   K ::= Participant | Object | ...   (finite, declared)
Values      v ::= n | b | t | ⟨f̄ = v̄⟩ | key(k) | capref(ρ) | witref(ω)
Types       A, B  (theory/02)
Effects     ε  effect row;  Failures φ  failure row  (theory/02)

State decl  D ::= state x[k:K] : A at ℓ [init e] [visible vis [fields f̄]]
Chain decl  C ::= chain c : A = o₁ > o₂ @lin > ... > oₙ @lin      (theory/06)
Option      o ::= option(name, target, cap, lease)

Surface s ::= x[e].f = e′            assignment
            | x[e].f ⊕= e′           compound assignment (⊕ ∈ {+,-})
            | let y = e | if e {s̄} else {s̄} | e
            | ℓ { s̄ }                locus block
            | when h(p̄:K̄) fails φ { s̄ }
            | join ℓ as R via ℓₐ
            | grant g(π) | require e | publish e produces witness w
Expr e    ::= v | y | x[e].f | e ⊕ e | e == e | ¬e | e ∧ e | ⟨f̄ = ē⟩ | c

Core  c ::= read(ℓ, x[v].f)                       dependency, not occurrence
          | write(ℓ, x[v].f, v′)                  owner-local occurrence
          | request(ℓ_src → ℓ_own, op, v̄, ρ̄, ω̄, φ)  owner-directed occurrence
          | eval(k, body, EP)                         theory/13; pure body
          | relationdef(r, O, subject, primary, fallback, transform, policy, label)
          | bindrel(r, selected, lineage, epoch, witness, frontier)
          | projectrel(r, selected, anchor, epoch, transform, frontier, label)
          | remote-result(r, O → T, read, receipt, EP) | consume-result(k, F, version, C)
          | publish(ℓ, x[v].f, v′, vis) | observe(π, ℓ, x[v].f, vis)
          | grant(π, g, verdict) | use(ρ) | witness(ω) | usewit(ω)
          | admitreq(π, R, ℓ) | verdict(π, R, ℓ, out, epoch, inc, ρ̄, ω)
          | cut(ℓ)                                 atomic_cut
          | patchreq(P) | patchverdict(P, out) | activate(P, F)
          | seq(c, c) | cond(v, c, c) | pure(v)
```

Design notes (settled): `read` produces a dependency edge, never an occurrence
(ADR-0002). `request` carries its authorizing capability refs, required witness
refs, and its declared failure row. `activate` is bound to the admission-time
frontier `F` (theory/08).

## 2. Runtime configurations

```text
Config  Σᵣ = ⟨ H ; Q ; S ; M ; G ; W ; L ; P ; R ; D ; J ⟩
  H  occurrence DAG (E, ≺)                       theory/04
  Q  per-locus request queues  Locus → Queue(request)
  S  stores: ℓ ↦ (x ↦ Active(K, epoch) ⇀ value)   indexed state, owner-local
  M  membership: ℓ ↦ (epoch, π ⇀ incarnation × status)
  G  capability store: ρ ↦ grant-lineage record    theory/05
  W  witness store: ω ↦ witness record
  L  lease/chain store: c ↦ (position, per-option lease state)  theory/06
  P  patch lifecycle store                          theory/08
  R  explicit remote-result receipt store            theory/13
  D  designated-result/version store                 theory/13
  J  maintained relation / binding store              theory/14, finite shared profile theory/15
```

A configuration is **well-formed** iff H is acyclic, every `use(ρ)` in H has a
`grant` ancestor with matching lineage, every `observe` has a `publish`
ancestor, every store entry's key is Active at its recorded epoch or explicitly
tombstoned, every chain position is ≤ its history maximum (monotone), and every
J relation dependency graph is acyclic with an owner-held binding whose selected
anchor/epoch/frontier is live. `projectrel` is an admitted relation publication;
it is not a derived-value stream. Well-formedness is preserved by every step
rule below (OBL-020).

## 3. The unified judgment

```text
Σ ; Ψ ; Γ ; Δ ; L ⊢ s ⇝ c : A @ μ ! ε ‖ φ ▷ C ; O ; G_e
```

- `Σ`  module/type/place/role/effect environment (declared, finite)
- `Ψ`  place / phase / frontier / freshness context
- `Γ`  unrestricted context;  `Δ`  linear / capability context
- `L`  current locus;  `s` Surface item;  `c` Core term
- `A` type; `μ` mode (`local` | `remote(ℓ)`); `ε` effect row; `φ` failure row
- `C` constraints discharged decidably now; `O` residual obligations
  (`ResidualObligation` carriers, ADR-0010); `G_e` generated edges
  (request / publish / observe / witness rows with source spans)

This single judgment is simultaneously the checker judgment (drop `⇝ c` and
`G_e`) and the elaboration judgment (keep them). Line-1 checking is decidable
on the declared finite fragment (OBL-003).

## 4. Key rules (rule sketches; full premises in 03–06)

**[READ-LOCAL]** L owns x. `x[v].f ⇝ read(L, x[v].f)`; adds a dependency edge
to G_e only when Ψ marks the read audited/cross-cut-relevant; no occurrence;
requires key Active in current epoch (else E-IDX-003).

**[READ-CROSS]** owner(x) = O ≠ L. Elaborates to `request(L→O, read, ...)` or
an observe edge, requires declared visibility/observe authority in Δ,
and extends φ with the generated failure set; rejected if the enclosing
`fails` row does not contain it (E-ROW-001).

**[WRITE-LOCAL]** owner(x) = L. `x[v].f = e ⇝ write(...)`; occurrence; key must
be Active; visible fields additionally elaborate a `publish` edge per D's
`visible` clause.

**[WRITE-CROSS]** owner(x) = O ≠ L. Elaborates to
`request(L→O, write(x[v].f), v′, ρ̄, ω̄, φ_gen)` with
`φ_gen ⊆ {StaleMembership, MissingCapability, MissingWitness,
RouteUnavailable, VisibilityDenied, TypeMismatch}`. Premises: Δ contains a
write-capability for (O, x) or the op is owner-mediated; φ_gen ⊆ declared
`fails`; source span preserved. The capability authorizes the request; it never
becomes a direct remote store (ADR-0003, ADR-0005).

**[LOCUS-BLOCK]** `O { s̄ }` under current locus L: if L = O, elaborate body with
current_locus := O (save/restore). If L ≠ O, the block is **not** an ambient
authority switch: body is checked under O for owner invariants, but elaborates
to owner-directed requests authorized from L, carrying L's principal, epoch,
incarnation, capability refs, witness refs, and spans.

For an owner-directed body whose mutable operands and write are all O-owned,
O is also the evaluation site: on service it evaluates the reads and write as
one bounded owner transition. L remains the authority origin. This is not a
requester-side private read or a blind write; a genuinely other-owner operand
uses an explicit result/receipt path or is outside the ordinary v0 fragment
(ADR-0016, ADR-0018). The exact `EP` carrier and service rules are theory/13.

**[HANDLER]** `when h(p̄) fails φ_d { s̄ }`: body elaborated with parameters
bound; every generated failure of the body must be ⊆ φ_d (containment is a
Line-1 check); h becomes a transition entry point at L.

**[JOIN]** `join ℓ as R via ℓₐ ⇝ admitreq(π_self, R, ℓ)` targeted at ℓₐ. A role
claim confers nothing; subsequent statements requiring grants must appear
causally after the corresponding `verdict` (theory/05).

**[GRANT/REQUIRE/PUBLISH]** admissible only in loci whose declarations permit
them; `grant` elaborates to `grant(π, g, verdict_ref)`; `publish ... produces
witness w` elaborates to `publish` + `witness(ω)`.

**[CUT]** `atomic_cut ⇝ cut(L)`: fixes the rollback frontier of L only; not a
distributed commit, not a fence (ADR-0007, theory/04).

**[CHAIN-ACCESS]** access through chain c resolves the leftmost admissible
option at the recorded position; static evidence floor and monotone advance in
theory/06.

## 5. Small-step operational semantics (shape)

Steps are labelled `Σᵣ ─a→ Σᵣ′` where `a` appends zero or one occurrence to H
(reads append none). Selected rules:

```text
[E-WRITE]   owner-local write:  S′ = S[ℓ][x][k ↦ v′];  H′ = H + write-occ
[E-REQ]     request emission:   Q′[O] = enqueue(req);   H′ = H + request-occ
[E-SERVE]   owner O dequeues req; validates (epoch, incarnation, ρ̄ lineage, ω̄,
            visibility). On pass: perform op as [E-WRITE]/read+reply, append
            served-occ with hb edge request ≺ serve. On fail: append explicit
            failure occurrence F ∈ φ(req); no store change (fail-closed).
[E-PUB]     publish appends publication-occ; [E-OBS] observe requires a
            publish ancestor and observer authority; redaction per theory/07.
[E-ADMIT]   admission verdict updates M (epoch+1 on membership change),
            issues grants into G, and witnesses into W.
[E-CUT]     cut(ℓ) appends cut-occ; later rollback at ℓ cannot cross it.
[E-DEGRADE] lease expiry / explicit failure advances chain position
            monotonically (never decreases);  [E-REACQ] explicit reacquire
            starts a new lineage with new witness/epoch (ADR-0004).
[E-REL-DEGRADE] semantic invalidation of a J binding is owner-recorded and
            advances its selected relation option primary → fallback; a
            consumer presentation gap has no J mutation.
[E-REL-PROJECT] owner emits an admitted `projectrel` with `publish-relation`;
            a consumer evaluates it only with a coherent presentation context.
[E-PATCH]   admitted patch activates via activate(P, F) only if the live
            frontier still matches F; otherwise verdict flips to deferred.
```

Scheduling: the calculus is nondeterministic over enabled steps; the
**conformance profile** (spec/05) fixes a deterministic schedule for testing.
Owner queues are served serially per owner — this *is* the concurrency model
(ADR-0003): interleaving exists between loci, never inside one owner's store.
`eval`, `remote-result`, and designated-result rows refine this shape in
theory/13; no receipt, evaluator version, or evaluation coordinate is implicit.

## 6. Theorems anchored on this calculus

Statements live with their chapters; statuses live in theory/11.
THM-001 (assignment elaboration soundness, 03) — THM-002 (fallback
monotonicity, 06) — THM-003 (cut prefix closure / no stale resurrection, 04) —
THM-004 (authority soundness, 05) — THM-005 (observation noninterference, 07)
— THM-006 (patch rejection no-mutation, 08).

## 7. Deliberately outside v0

Coroutines/continuations (LAB D-008: only restricted models are candidates),
`barrier`, full `durable_cut` algebra beyond the all_of profile (04 §7),
arbitrary dependent types (ADR-0010), route rebinding algebra, arrays as
indexed-state owners, dynamic locus expressions (`at e { }` needs a separate
decision), distributed transactions.

OPEN-010: whether `serve` failure replies are occurrences at the requester too
(currently: yes, as receive-occ). OPEN-012: whether `cond` needs join-point
normalization for the checker's finite fragment.
