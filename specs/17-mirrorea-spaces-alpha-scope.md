# 17 — Mirrorea Spaces Alpha Scope

## role

この文書は、Mirrorea Spaces alpha line の
**product scope / stage/phase reading / completion condition / non-goal**
を alpha-local 規範として置く。

- immediate target is Mirrorea Spaces alpha
- Reversed Library is future upper-layer application
- PrismCascade is separate media kernel candidate
- final public parser/API/ABI freeze is not the immediate goal

## decision level

- `L1`
  - immediate target は Mirrorea Spaces alpha である
  - Mirrorea != Reversed Library
  - PrismCascade is out of alpha implementation scope
- `L2`
  - browser-like interpretation
  - stage / phase mapping
  - alpha-local implementation shape

## Mirrorea Spaces alpha definition

Mirrorea Spaces alpha は次を満たす
minimal browser-like virtual-space substrate として読む。

- multiple `Place` execution
- typed `MessageEnvelope` exchange
- runtime package / layer / object hot-plug via request / verdict / activation cut
- membership / capability / witness / route / telemetry evidence
- typed debug / visualization surface

この alpha は final public product ではないが、
docs-only plan memo でもない。

## VRChat-class relationship

long-term functional lower bound として、
VRChat-class social virtual-space capability を eventually target に含めてよい。

examples:

- worlds / rooms / instances
- participants and avatar roles
- portals / world links
- object interaction
- synchronized state
- user-generated runtime package / object / avatar ecosystem
- moderation / safety / permission surfaces

しかし alpha target は VRChat clone ではない。
browser-like world substrate を principal にする。

## browser-like virtual-world interpretation

useful analogy:

| Web | Mirrorea Spaces |
|---|---|
| browser | Mirrorea client |
| page / app | world / space package |
| URL | logical world name / route |
| link | portal / world reference |
| extension | client-side layer / policy / visualization |
| devtools | event DAG / route / witness debugger |

important difference:

- Mirrorea starts from shared stateful virtual space
- presence / shared state / witness / ordering / hot-plug / visibility frontier are central

## alpha-local implementation shape

current alpha-local implementation shape は次を default に置く。

- `library-first`
- helper CLI
- HTML / devtools-style viewer
- Docker E2E

これは installed binary packaging, final host target, final public ABI の freeze ではない。

## completion condition

Mirrorea Spaces alpha を current-scope complete と呼ぶには、
少なくとも次の integrated evidence が必要である。

- local or controlled runtime with multiple Places
- network/container E2E at narrow cut
- runtime package hot-plug
- layer hot-plug
- typed debug / visualization output
- local save/load or explicit non-claim with checker-backed invalid distributed cut rejection
- negative tests for stale membership / missing capability / missing witness / incompatible patch / invalid cut / unsigned native package reject

これらがなければ final Spaces alpha complete と書かない。

## non-goals

alpha immediate scope では次を goal にしない。

- final public parser grammar freeze
- installed binary / packaging commitment
- production auth / transport / viewer / verifier
- production theorem prover binding
- production model checker binding
- durable distributed save/load completion
- WAN federation protocol
- full VRM / VRChat / Unity compatibility
- Reversed Library implementation
- PrismCascade implementation

## stage / phase map

current alpha-local stage reading は次を使う。

- `Stage A`
  - existing repo-local alpha-ready floor
  - imported baseline for the alpha line
- `Stage B`
  - integrated local Mirrorea runtime
  - current-scope closeout for this stage is:
    - one accepted local queue / `MessageEnvelope` dispatch / event DAG export path
    - one stale-membership rejection path before state mutation
    - one local-only save/load resume path
    - one local-only save/load stale-membership non-resurrection path
  - Stage B closeout does not imply distributed save/load completion, `CUT-10/12/16` completion, or active runnable-root promotion of `samples/alpha/`
- `Stage C`
  - real transport narrow cut
- `Stage D`
  - hot-plug minimal lifecycle
- `Stage E`
  - visualization / devtools
- `Stage F`
  - Mirrorea Spaces alpha demo

current concrete phase reading は次を使う。

- `Phase 0` theory freeze preparation
- `Phase 1` typed IR / checker skeleton
- `Phase 2` executable semantics / event DAG
- `Phase 3` local runtime
- `Phase 4` layer insertion runtime
- `Phase 5` network / Docker E2E
- `Phase 6` save/load and consistent cut
- `Phase 7` runtime package / avatar adapter
- `Phase 8` integrated alpha demo

## Stage A imported-baseline boundary

`Stage A` is an imported baseline for the alpha line, not a new `samples/alpha/` runnable-root promotion.

- admissible evidence:
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`
  - `python3 scripts/current_l2_lean_sample_sync.py`
  - `python3 scripts/clean_near_end_samples.py closeout`
  - `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - `python3 scripts/avatar_follow_samples.py closeout --format json`
  - `python3 scripts/typed_external_boundary_samples.py closeout --format json`
  - `python3 scripts/network_transport_samples.py check-all --format json`
  - `python3 scripts/projection_codegen_samples.py check-all --format json`
  - `python3 scripts/visual_debugger_viewer_samples.py closeout --format json`
  - `cargo test -p mir-runtime --test hotplug_runtime_skeleton`
- this combination is sufficient to count Stage A as 100% imported baseline for the current alpha line
- this does not imply:
  - parser/runtime front-door execution of `samples/alpha/*.mir`
  - active runnable-root promotion of `samples/alpha/`
  - final public parser/runtime/viewer/hot-plug/transport ABI
  - durable distributed save/load completion
  - final host / packaging / product commitment

## required E2E and negative tests

current alpha-local demo line は少なくとも次の family を必要とする。

- local integrated Sugoroku-style world
- two-node Docker network sample
- hot-plug debug layer runtime sample
- hot-plug rate-limit / auth layer sample
- avatar runtime package sample
- local save/load continue sample
- invalid distributed snapshot negative sample
- optional/non-completion upper-layer seed demo may exist, but it does not count toward Alpha completion
- package-missing-runtime fallback sample

negative test floor:

- stale membership reject
- missing capability reject
- missing witness reject
- incompatible patch reject
- invalid cut reject
- unsigned native package reject

## Stage B current-scope closeout boundary

`Stage B` current-scope closeout is narrower than full `specs/15` family completion.

- admissible evidence:
  - `LR-01` local accepted dispatch path
  - `LR-02` stale-membership rejection path
  - `CUT-04` local-only save/load resume path
  - `CUT-17` local-only save/load stale-membership non-resurrection path
- this combination is sufficient to call alpha-0.5 local runtime complete for current scope
- this does not imply:
  - distributed/durable save/load completion
  - `CUT-10/12/16` completion
  - parser/runtime front-door execution of `samples/alpha/*.mir`
  - Stage C/D/E/F completion
  - final public runtime ABI

## Stage C current-scope closeout boundary

`Stage C` current-scope closeout is narrower than full transport-family completion.

- admissible evidence:
  - `NET-02` accepted Docker/local-subprocess envelope exchange
  - `NET-03` stale-membership rejection over the transport seam
  - `NET-04` missing-capability rejection over the transport seam
  - `NET-05` missing-witness rejection over the transport seam
  - `NET-07` observer-safe redacted route trace
  - `NET-09` auth-evidence lane preserved separately from transport delivery
- this combination is sufficient to call alpha-0.7 transport complete for current scope
- this does not imply:
  - `NET-06` route-rebinding / no-shadow completion
  - `NET-08` network-partition completion
  - `NET-10` transport-medium substitution completion
  - production WAN / session / replay runtime
  - final public transport ABI
  - Stage D/E/F completion

## Stage D current-scope closeout boundary

`Stage D` current-scope closeout is narrower than full hot-plug lifecycle completion.

- admissible evidence:
  - `LI-01` authorized debug layer attach with trace only after activation
  - `LI-02` non-admin debug layer rejection before activation
  - `LI-03` explicit contract-update auth path
  - `LI-04` declared-failure rate-limit preview path
  - `LI-05` incompatible patch rejection before activation
  - `AV-01` placeholder avatar runtime accepted
  - `AV-02` custom Mir avatar runtime accepted
  - `AV-06` untrusted native avatar rejected
  - `AV-08` runtime-unavailable placeholder fallback
  - `AV-09` undeclared effect widening rejected
  - `HP-11` unsigned native package rejected
  - `HP-12` signed over-capability package rejected
  - `HP-15` revoked/stale-signed native package rejected
- this combination is sufficient to call alpha-0.8 hot-plug lifecycle complete for current scope
- this does not imply:
  - detach runtime completion
  - durable migration
  - distributed activation ordering
  - native execution realization
  - `HP-08/09/13/14`
  - `AV-03/04/05/07/10`
  - final public layer attachment ABI
  - final public runtime package / avatar ABI
  - Stage E/F completion

## Stage E current-scope closeout boundary

`Stage E` current-scope closeout is narrower than full visualization/devtools-family completion.

- admissible evidence:
  - `VIS-01` event DAG export
  - `VIS-02` place-catalog projection view
  - `VIS-03` route trace export
  - `VIS-05` membership epoch/incarnation timeline
  - `VIS-06` hot-plug lifecycle bundle
  - `VIS-07` fallback degradation view
  - `VIS-08` observer-redacted route trace
  - `VIS-10` on-demand trace-only gating
  - `VIS-11` retention-policy enforcement
- this combination is sufficient to call alpha-0.9 devtools complete for current scope
- this does not imply:
  - `VIS-04/09/12` completion
  - witness timeline completion
  - privileged admin-full visualization completion
  - detach-stops-trace runtime completion
  - final public viewer API
  - final public telemetry service
  - Stage F completion

## Stage F current-scope closeout boundary

`Stage F` current-scope closeout is narrower than full public-alpha or upper-layer completion.

- admissible evidence:
  - `E2E-01` local integrated Sugoroku path
  - `E2E-02` Docker two-node transport path
  - `E2E-03` debug-layer attach path
  - `E2E-04` declared-failure rate-limit preview path
  - `E2E-05` placeholder/custom avatar runtime path
  - `E2E-06` local save/load continue path
  - `E2E-07` distributed inconsistent save negative path
  - `E2E-09` auth-layer contract-update path
  - `E2E-10` package-missing-runtime fallback path
  - current-scope `Stage E` closeout surface
- this combination is sufficient to call alpha-1 Spaces alpha complete for current scope
- this does not imply:
  - `E2E-08` completion
  - distributed/durable save/load completion
  - `CUT-10/12/16` completion
  - `LIF-15` or `VAR-14` completion
  - detach runtime completion
  - durable migration
  - distributed activation ordering
  - native execution realization
  - active runnable-root promotion of `samples/alpha/`
  - public alpha / `U1` completion
  - final public parser / checker / runtime / viewer / telemetry / transport / hot-plug ABI
  - full VRChat / Reversed Library completion

## relationship to Reversed Library

Reversed Library / 裏返した図書館 is upper-layer flagship application.
It is not the runtime/fabric itself.

alpha may prepare substrate that makes it possible later,
but alpha completion is not Reversed Library implementation.

## relationship to PrismCascade

PrismCascade is separate media-processing kernel candidate.

- it may later connect through typed external effect / adapter / trace linkage
- it is not in Mirrorea alpha implementation scope
- do not import its scheduling semantics into Mir core during alpha

## user-decision blockers that remain later

alpha-local self-driven packages may proceed without fixing final public product adoption.
still-later user-facing blockers remain:

- first public surface shape
- first host target
- final packaging / installed binary adoption
- final shared-space operational catalog breadth
- native binary execution policy beyond alpha skeleton

これらは alpha-local theory freeze / sample skeleton / checker first cut と distinct に扱う。

## stop line

- Mirrorea Spaces alpha を Reversed Library implementation と書かない
- Mirrorea Spaces alpha を full VRChat compatibility と書かない
- helper-local runtime or Docker E2E を production runtime と書かない
- alpha-local implementation shape を final product adoption decision と書かない
- PrismCascade を alpha implementation scope に含めない
