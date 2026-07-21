# progress

最終更新: 2026-07-21 20:06 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction, theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/` is LAB: evidence, history, implementation, and operational notes. If LAB text conflicts with canon, canon wins.

## document role

This is the concise LAB snapshot of workflow readiness and evidence. It is not
a canon decision record or a historical log. `docs/project-status.md` is the
human control view, `tasks.md` is the current work map, and `plan/` holds
detailed repository memory.

## project axis

```text
Correct theory -> safe hot-plug -> execution, communication, verification,
and visualization across Places in a virtual-space system.
```

Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform remain
separable. A domain `World` or `Game` is user-defined on Mir; it is not a Mir
core primitive.

## final ideal

```text
.mir -> Surface parse/elaboration -> Core typed IR and obligations
     -> checker/runtime -> projection/deployment -> host/provider/view evidence
```

The target carries source semantics through placement, explicit communication,
contracts, evolution, and observation without folding authentication or
visualization into an untyped transport/debug channel.

## current milestone position

| Axis | Status | Readiness / next boundary |
| --- | --- | --- |
| Logical specification | `T0/G0 rebaseline`; ADR-0014 enables autonomous bounded LAB theory research | G0 exit and T1 entry remain unrecorded under `mirrorea_canon/plan/01-phases.md`; L3 is limited to committed `working/WRK-####`, while L2 is fail-closed pending an owner-authenticated trust anchor |
| User-facing specification | source-first direction and examples have bounded LAB evidence | Surface grammar closure and public contract remain owner-reserved |
| Implementation / operation | Product Alpha, Full System V1, Surface, and operational roots are runnable bounded LAB evidence | no C-static/C-runtime/C-distributed conformance or final runtime/product claim |

`plan/156-t0-t2-research-autonomy-envelope.md` remains the evidence record for
T-RESEARCH-001..033. Its `research-complete` and `decision-ready` labels do
not describe the current authority route. New non-reserved theory work uses the
LAB candidate lifecycle in `plan/158-standing-bounded-autonomy.md`.
Its L3 branch is standing-delegated in `working/WRK-####`; existing canon text
remains read-only and L2 selection is fail-closed pending an owner-authenticated
trust anchor.

## milestone map

| Phase | Primary aim | Current position | Autonomy |
| --- | --- | --- | --- |
| T0 | vocabulary and G0 | current; G0-D3 deferred | bounded LAB research and WRK L3 records; reserved boundaries escalate |
| T1 | calculus, G1-G3 statements, final SCN expectations | no official entry | statement research; reserved boundaries escalate |
| T2 | proof skeletons and G5 statements | later research target | conditional Lean work; final proof status stays owner-controlled |
| I1 | reference implementation | later | blocked on theory exits |
| I2 | multi-locus | later | blocked on I1 |
| I3 | real transport | later; first real LAN point | blocked on I2 / transport ADR |
| I4-I6 | persistence, View, distributed federation | later | blocked on prior phases |

Exact exits are `mirrorea_canon/plan/00-gates.md` and
`mirrorea_canon/plan/01-phases.md`, not this table.

## line snapshots

### Product Alpha line

`samples/product-alpha1/` provides bounded runnable product-alpha evidence,
not a final product, public compatibility promise, or canon implementation
status. See `samples_progress.md`.

### Operational Suite line

The WorldCore through TwoShard/Gradient suite checks documented same-session
contracts and evidence. It is not real distributed durability or a final
shared-space catalog. See `samples_progress.md`.

### Mir Language line

Surface parser, indexed-state checker, elaboration, role admission, source
patch, and static devtools are runnable LAB evidence. Current theory candidate
selection is governed by ADR-0014 and `plan/158`, not by a runtime widening
claim.

### PoseGraph line

PoseGraph has bounded LAB sample evidence. Its performance-sensitive kernel
remains separable from Mir runtime semantics. See `samples_progress.md`.

### Projection/Backend line

Projection, deployment, and backend/provider artifacts remain bounded LAB
evidence or planned boundaries. BND-006 and later implementation phases govern
their realization. See `mirrorea_canon/plan/00-gates.md`.

### Engine/Provider line

Provider and engine adapters are typed external boundaries. They do not define
world semantics, authentication, authorization, or the Mir core. See
`samples_progress.md`.

## validation floor

| Evidence | Current command |
| --- | --- |
| documentation/source hierarchy | `python3 scripts/validate_docs.py` and `python3 scripts/check_source_hierarchy.py`; malformed or rewritten reachable WRK history, invalid registration, invalid manifested evidence, and unresolvable L2 frozen material are rejected; `--authoritative-working-annex` additionally requires a clean disposable worktree |
| canon metadata | `python3 meta/build-index.py --check` from `mirrorea_canon/`; stale `INDEX.json` is rejected |
| Surface static LAB anchor | `python3 scripts/surface_mir_samples.py check-all --format json` |
| OBL statement shapes | direct `lake env lean` checks under `samples/lean/lab-statements/` |
| runnable dashboards | commands recorded in `samples_progress.md` |

Run the anchor relevant to the changed layer plus the required documentation
checks; broad validation is evidence, not a phase transition.

## non-claims

No Gate/Phase exit, OBL discharge, final proof, conformance result, final
grammar/API/ABI, real transport, distributed durable save/load, or public
product claim is made by this snapshot, a reviewed working theory, or runnable
LAB evidence.

## user decision items vs research-discovery items

| Kind | Item | Current handling |
| --- | --- | --- |
| Owner / canon | G0-D3 | deferred and dormant until explicit owner reopen |
| Owner / canon | PROPOSAL-003 and PROPOSAL-004 | L1-reserved organization / grammar choices; owner records A/B/C |
| Owner / LAB route | OBL-001 concrete-evidence bridge | defer or authorize an artifact-free design comparison only |
| Research | non-reserved theory target | pin standing eligibility, pre-register alternatives/falsifier, and seek evidence in LAB. It may enter WRK L3; steward rebase/freeze and independent review precede L2 integration or escalation |
| Later dependency | runtime, conformance, final ABI, transport, federation | do not preempt theory phase |

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
| --- | --- | --- | --- | --- |
| 0 | repository memory and reporting discipline | delegated governance and cockpit are current | light | maintenance and drift audit |
| 1 | semantic kernel and invariant boundaries | canon direction fixed; LAB candidates can be selected | medium | bounded LAB research and WRK L3; L2 review-gated |
| 2 | parser-free validation substrate | existing runners are compatibility anchors | medium | reproduce / existing-lane research only |
| 3 | compile-ready actualization | Surface alpha evidence closed | heavy | maintenance only |
| 4 | sample expansion | bounded operational evidence exists | heavy | maintenance only |
| 5 | theorem / model-check bridge | historical countermodels and conditional kernels exist | medium | review-gated research without proof laundering |
| 6 | distributed fabric and runtime evolution | later | heavy | later dependency |
| 7 | toolchain/backend surface | bounded LAB evidence only | heavy | later dependency |
| 8 | domain/application realization | bounded samples exist; products are later | heavy | later dependency |

## feature maturity rows

| Feature | Evidence status | Remaining gate | Startability |
| --- | --- | --- |
| multi-node / fabric | local and bounded LAB evidence | I2/I3 and transport choice | later dependency |
| contracts / theorem / model-check boundary | statement drafts, countermodels, and static evidence | reviewed working premises, proof skeletons, `theory/11` final status | research eligible |
| dynamic attach/detach / DAG-safe evolution | bounded LAB sample evidence | canon G7 / implementation | later dependency |
| `atomic_cut` / ordering | canon theory exists | G5 statements / proof research | research eligible when target is scoped |
| executable sample corpus | runnable bounded LAB workflows | conformance and public operational workflow | maintenance only |

## recent log

- 2026-07-21 20:06 JST: pre-registered WRK-0003 as an L3 countermodel test of
  whether total/unique per-result projections and equality component
  comparisons still fail to force Result identity. The WRK-0002 source,
  temporary Oracle review, and independent Canon audit agree that Canon fixes
  the intended output tuple/function contract but does not define the LAB
  draft's projection witness or extensionality laws. No WRK-0003 outcome has
  run or been relied on; no theory/11, Gate/Phase, conformance, implementation,
  or public status changed.
- 2026-07-21 19:59 JST: manifested WRK-0002's L3 Lean countermodel evidence.
  Lean 4.29.1 checks that the existing OBL-021 LAB statement draft can hold
  with two distinct successful results for one well-scoped input when all nine
  result projections are empty. This narrows the statement-shape gap to result
  identity / projection non-vacuity; it does not choose a premise, equality,
  diagnostic ABI, proof status, Gate/Phase action, conformance, implementation,
  or public status. L2 remains fail-closed.
- 2026-07-21 19:42 JST: pre-registered WRK-0002 as an L3 countermodel test of
  whether OBL-021's existing LAB statement draft permits distinct successful
  results through empty result projections. No Lean outcome has run or been
  relied on. This does not change theory/11, Gate/Phase, conformance,
  implementation, or public status; L2 remains fail-closed.
- 2026-07-21 17:49 JST: after explicit approval, `cargo clean` removed 18,248
  local build files (reported 8.5 GiB) and 460 Mirrorea temporary directories.
  Root free space rose from about 2.4 GiB to 12 GiB; source, Git history, and
  retained evidence were not removed. External workdir remains unmounted, so
  heavy-work capacity checks remain required.
- 2026-07-21 17:37 JST: storage audit found the root filesystem at 99% use with
  about 2.4 GiB free and no mounted `/mnt/mirrorea-work`. Existing `target/`
  (7.0 GiB) and `/tmp` (2.5 GiB) are not removed without explicit confirmation;
  future heavy work is paused pending cleanup approval or an external workdir.
  This does not change research or implementation claims.
- 2026-07-21 17:32 JST: closed the WRK-0001 pilot checkpoint. Clean detached
  authoritative validation, `make check`, full Python tests, and independent
  review passed; wording now distinguishes finite lifetime/capture carriers
  from the unbounded `Nat` budget parameter. The Oracle OBL-021 idea remains a
  future candidate requiring its own pre-registration. No Gate, Phase, SCN,
  OBL discharge, conformance, production implementation, or public status
  changed.
- 2026-07-21 17:22 JST: manifested evidence commit `887a0f6c` in WRK-0001 with
  its exact plan artifact hash. The Lean reproduction remains L3
  `not-promoted`; clean-worktree validation and the cross-cut checkpoint remain
  next. No Gate, Phase, SCN, OBL discharge, conformance, production
  implementation, or public status changed.
- 2026-07-21 17:17 JST: WRK-0001's registered Lean reproduction passed: the
  existing finite-index fragment compiled with Lean 4.29.1, and its four named
  local positive/rejection lemmas passed the placeholder/escape-token audit.
  The retained LAB evidence is limited to `plan/wrk-0001-finite-index-reproduction.md`;
  it has not yet been manifested in the WRK record, and L2 remains fail-closed.
  No Gate, Phase, SCN, OBL discharge, conformance, production implementation,
  or public status changed.
- 2026-07-21 17:09 JST: opened WRK-0001 as a committed L3 pre-registration for
  a bounded reproduction of theory/02's finite-index allowance in the existing
  helper-local Lean fragment. No outcome evidence has been run or relied on;
  L2 remains fail-closed. No Gate, Phase, SCN, OBL discharge, conformance,
  production implementation, or public status changed.
- 2026-07-21 17:00 JST: committed and pushed the standing bounded-autonomy
  governance package. Its authoritative WRK validation passed in a clean
  disposable detached worktree; the ordinary worktree's ignored local
  configuration was correctly rejected as non-evidence. No Gate, Phase, SCN,
  OBL discharge, conformance, production implementation, or public status
  changed.
- 2026-07-21 12:24 JST: reviewer findings added explicit L3-without-review
  wording, immediate `Reliance status: frozen` reliance stop, WRK structural
  validation, and stale-index rejection. No Gate, Phase, SCN, OBL discharge,
  conformance, production implementation, or public status changed.
- 2026-07-21 13:37 JST: strengthened the L2 working-record review evidence to
  resolve an author-signed Git base, exact canon/LAB SHA-256 snapshots,
  normalized record SHA-256, and a distinct reviewer signature on the direct
  admission commit. The missing owner-authenticated trust anchor then left L2
  intentionally fail-closed; L3 remains committed pre-registration without review. No
  Gate, Phase, SCN, OBL discharge,
  conformance, production implementation, or public status changed.
- 2026-07-21 11:12 JST: amended ADR-0014 to standing bounded autonomy. An agent
  may pre-register a non-reserved L3 candidate in `working/WRK-####` and run
  existing-lane theory/implementation evidence without routine target approval;
  L2 selection is currently fail-closed pending an owner-authenticated trust
  anchor. No Gate, Phase, SCN, OBL
  discharge, conformance, production implementation, or public status changed.
- 2026-07-21 16:31 JST: recut WRK provenance after independent planner and
  Oracle review. Reachable-DAG identity/pre-registration checks, append-only
  explicit evidence-commit ownership, artifact-to-commit binding, and optional
  clean-worktree validation replace the unsound descendant-wide attribution
  rule. L3 remains operational research governance only; L2 remains
  fail-closed. No Gate, Phase, SCN, OBL discharge, conformance, production
  implementation, or public status changed.
- 2026-07-18 16:45 JST: Completed the OPEN-025 literature anchoring scan.
  Four evidence-backed comparison rows were added without a novelty claim,
  semantic decision, proof status, Gate, Phase, or implementation change.
- 2026-07-17 20:55 JST: Completed the cross-boundary theory claim-integrity
  audit. Existence DAG, patch DAG, and stream fallback remain later
  formalization directions, not completed proofs.
