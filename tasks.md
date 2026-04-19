# tasks

最終更新: 2026-04-19 20:16 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 規範判断の正本は `specs/`、detail-side の repository memory は `plan/11`、`plan/12`、`plan/13`、`plan/16`、`plan/17`、`plan/18` に寄せる。
- `tasks.md` は append-only ではなく、毎回 snapshot に合わせて全体を書き直す。

## current status at task level

- current mapped corpus では、
  - authored sixteen
  - corrected prototype set `p01...p14`
  - runner / CLI / regression / helper-local compare floor
  が already runnable である。
- representative Lean sample set `e5 / p06 / p10 / p11 / p12 / p07 / p08 / p09 / p13 / p14` は actual Lean execution reached であり、`samples/lean/current-l2/` に committed corpus として保存済みである。
- `samples/lean/foundations/` には
  - `CurrentL2LabelModel.lean`
  - `CurrentL2IfcSecretExamples.lean`
  - `CurrentL2ProofSkeleton.lean`
  の actual small proof fragment が入った。
- したがって、remaining work の主眼は次に移っている。
  - strong typing / IFC beyond first checker fragment の checker payload public-schema sketch ratchet
  - final public theorem/model-check/order-handoff/shared-space contract の mixed gate
  - order-handoff/shared-space residual public-seam maintenance
  - packaging / FFI / broader app target の user-spec residual
- exact rough stimulus は `samples/not_implemented/` preservation bucket に残し、corrected runnable version と混同しない。

## current executable floor

- `samples/current-l2/`
  - authored sixteen は inventory / runner / verification ladder / emitted artifact wiring / regression helper に乗っている
- `samples/prototype/`
  - corrected prototype set `p01...p14` は explicit path + adjacent host-plan sidecar で runnable
  - helper-local `debug_outputs` / `verification_preview` / `artifact_preview` を見せる current cut がある
- `samples/lean/`
  - `foundations/` は actual small proof fragment
  - `current-l2/` は representative Lean sample set generated stub corpus
  - `manifest.json` は Lean verification summary
  - generated current-L2 theorem stubs は `sorry` を含むため、artifact well-formedness / bridge alignment evidence として読む
- theorem / model-check / order-handoff / shared-space current floor
  - theorem-side:
    theorem-first pilot、binding preflight、Lean-stub pilot、artifact-conformance bridge、representative trace alignment、public-seam compression、toolchain probe/reopen manifest、representative Lean sample set actual Lean execution、reopen-threshold helper mirror
  - model-check side:
    row-local property route、checker-artifact route、final public-contract reopen threshold、public-seam compression、reopen-threshold helper mirror
  - order-handoff / shared-space side:
    authoritative-room vertical slice、surface actual adoption、source-wording route actual adoption、serial-scope reserve surface、witness/provider route/schema route actual adoption、emitted-contract trace alignment bridge、public-seam compression、public-seam helper mirror、CLI `surface_preview`、late-join negative static stop `p13 / p14`

## ordered self-driven packages

| package | question | package weight | macro phase | current recommendation | promotion criteria |
|---|---|---|---|---|---|
| `67` checker payload public-schema sketch ratchet | checker payload supported-kind summary threshold の次段として public checker payload schema sketch をどこまで helper-local summary に近づけるか | `M` | `Macro 5/7` | checker-adjacent principal と IFC first-fragment を維持したまま、5 ref wrapper の public schema sketch を helper-local threshold まで ratchet する | final public checker artifact / final public verifier contract を凍らせず、helper preview を public schema sketch まで整理できる |

## recently closed package note

### Package 57 — Lean formal skeleton / proof obligations

- current reading:
  first slice は close 済みである。
- close evidence:
  `specs/examples/521`
  `samples/lean/foundations/CurrentL2ProofSkeleton.lean`
  `samples/lean/current-l2/`
  `crates/mir-runtime/examples/current_l2_emit_theorem_lean_bundle.rs`
  `scripts/current_l2_lean_sample_sync.py`
- kept later:
  concrete production prover binding、final proof object public contract、final public verifier contract

### Package 56 — layered strong typing / IFC first-fragment

- current reading:
  first actual adoption package として close 済みである。
- close evidence:
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
  `samples/lean/foundations/CurrentL2LabelModel.lean`
  `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt`
- kept later:
  stronger typed source principal、checker-hint / diagnostics widening beyond IFC trio、actual checker payload family、final typed calculus、final IFC syntax、final public verifier contract

## active package notes

### Package 58 — helper / CLI hardening and broader coverage

- current reading:
  actual Lean execution reached 後の helper/CLI hardening と broader theorem-side / diagnostics / order-handoff corpus widening は close 済みである。`p09-dice-delegated-rng-provider-placement` carry-over、order-handoff CLI `surface_preview`、`p13 / p14` late-join visibility static stop、negative pair theorem-side Lean carry-over、`p10 / p11 / p12` sample-local `typed_checker_hint_preview`、theorem result-object preview helper mirror、model-check public-checker preview helper mirror まで actualize 済みと読む。
- evidence:
  toolchain probe / reopen manifest、representative sample set actual Lean execution、`samples/lean/` committed corpus、source-side IFC trio `p10 / p11 / p12`、`specs/examples/525`、`specs/examples/526`、`specs/examples/527`、`specs/examples/528`、`specs/examples/529`、`specs/examples/530`、`docs/reports/0806`、`docs/reports/0807`、`docs/reports/0808`、`docs/reports/0809`、`docs/reports/0810`、`docs/reports/0811`、`p09-dice-delegated-rng-provider-placement`、`p13-dice-late-join-missing-publication-witness`、`p14-dice-late-join-handoff-before-publication`
- stop line:
  final public theorem contract / final parser grammar / packaging には上げない

### Package 59 — near-end closeout sync

- current reading:
  close 済み。Package 58 close 後の helper-local actualization / residual gate reading を snapshot / roadmap / traceability に同期し、queue を residual mixed-gate packages へ再構成した。
- evidence:
  `specs/examples/531`
  `docs/reports/0812`
  `progress.md`
  `plan/11`
  `plan/17`
  `plan/18`
  `plan/90`

### Package 60 — theorem/model-check residual mixed-gate compression

- current reading:
  close 済み。theorem/model-check final public-contract reopen threshold を `run-source-sample` helper summary に mirror し、`p08` theorem reached / model-check guarded と `p09` theorem guarded / model-check reached の非対称を helper-local operational summary に actualize した。
- evidence:
  `specs/examples/532`
  `docs/reports/0813`
  `crates/mir-runtime/src/current_l2_cli.rs`
  `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- stop line:
  final public theorem/model-check contract adoption、final public verifier contract adoption、concrete production prover/model-check binding には上げない

### Package 61 — order-handoff/shared-space residual mixed-gate compression

- current reading:
  close 済み。order-handoff source wording residual / emitted-artifact residual / witness-provider public-seam residual を `run-source-sample` helper summary に mirror し、`p07 / p08` reached・`p09` guard の current cut を helper-local operational summary に actualize した。
- evidence:
  `specs/examples/533`
  `docs/reports/0814`
  `crates/mir-runtime/src/current_l2_cli.rs`
  `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- stop line:
  final source wording adoption、final public witness/provider/artifact contract adoption、final parser/public API adoption には上げない

### Package 62 — typed/IFC helper-to-checker ratchet

- current reading:
  close 済み。`typed_checker_hint_preview` を final typed source principal や final public verifier contract に上げず、checker-adjacent payload threshold まで narrow に ratchet し、`actual_checker_payload_family_threshold` を `payload_family_kind + source_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
  `specs/examples/529`
  `specs/examples/534`
- stop line:
  final typed source principal、final IFC syntax、final public checker artifact、final public verifier contract には上げない

### Package 63 — checker payload row-family ratchet

- current reading:
  close 済み。`actual_checker_payload_family_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row family まで narrow に ratchet し、`actual_checker_payload_row_family_threshold` を `payload_family_ref + row_family_kind` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/265`
  `specs/examples/266`
  `specs/examples/534`
  `specs/examples/535`
- stop line:
  supported kind detail、actual checker row payload、final public checker artifact、final public verifier contract には上げない

### Package 64 — checker payload row-detail ratchet

- current reading:
  close 済み。`actual_checker_payload_row_family_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row detail まで narrow に ratchet し、`actual_checker_payload_row_detail_threshold` を `payload_row_family_ref + row_source_ref + row_reason_kind` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/267`
  `specs/examples/268`
  `specs/examples/535`
  `specs/examples/536`
- stop line:
  actual checker row body、final public checker artifact、final public verifier contract には上げない

### Package 65 — checker payload row-body ratchet

- current reading:
  close 済み。`actual_checker_payload_row_detail_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload row body まで narrow に ratchet し、`actual_checker_payload_row_body_threshold` を `payload_row_family_ref + row_source_ref + row_reason_kind + row_body` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/269`
  `specs/examples/270`
  `specs/examples/536`
  `specs/examples/537`
- stop line:
  supported kind detail
  final public checker artifact
  final public verifier contract

### Package 66 — checker payload supported-kind-summary ratchet

- current reading:
  close 済み。`actual_checker_payload_row_body_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload supported kind summary まで narrow に ratchet し、`actual_checker_payload_supported_kind_summary_threshold` を `payload_row_family_ref + supported_kind_scope + supported_kind_refs` current cut で helper-local operational summary に actualize 済みと読む。
- evidence anchor:
  `specs/examples/271`
  `specs/examples/272`
  `specs/examples/537`
  `specs/examples/538`
- stop line:
  public checker payload schema
  final public checker artifact
  final public verifier contract

### Package 67 — checker payload public-schema sketch ratchet

- current reading:
  next active line。`actual_checker_payload_supported_kind_summary_threshold` を final public checker artifact や final public verifier contract に上げず、checker payload public schema sketch まで narrow に ratchet する。
- evidence anchor:
  `specs/examples/273`
  `specs/examples/274`
  `specs/examples/538`
- stop line:
  public checker API
  final public checker artifact
  final public verifier contract

## research-discovery items

| item | 何に影響するか | current recommendation |
|---|---|---|
| stronger typed-surface actual adoption | typed source principal | checker-adjacent principal + structural marker family first を維持し、evidence pressure が出るまで experimental adoption を待つ |
| final modal foundation / final source marker | syntax / modality / proof spine | partial basis + stronger family keep を維持し、final adoption は mixed gate に残す |
| authoritative-room `serial` sugar admissibility | order-handoff source-facing reserve surface | room-specific reserve に留め、principal surface には上げないまま helper-local evidence を集める |
| formal skeleton artifact shape beyond first slice | proof plan / Rust-Lean alignment | public proof artifact contract へ上げず、mechanization-ready internal floor に留める |

## remaining mixed gates

| topic | impact | current recommendation |
|---|---|---|
| final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract | theorem-first pilot | review-unit transport first、notebook-consumer object first、Lean-stub bridge current floor と representative Lean sample set actual Lean execution floor を維持し、final public theorem contract 群には上げない |
| first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker-runtime-policy contract / final public verifier contract | model-check line | row-local property route first、checker-artifact route first、reopen-threshold helper mirror keep、public-seam compression keep を維持する |
| final source-surface handoff wording / final emitted-artifact schema | order-handoff public surface | edge-row principal、stage-block secondary keep、thread/node same causal language keep、serial sugar reserve を維持する |
| final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract | shared-space stronger public shape | claim/payload split first、route/schema route actual adoption、trace-alignment reserve を維持し、final public contract 群には上げない |
| stronger typed-surface actual adoption | typed source principal | checker-adjacent principal + layered stack + IFC first-fragment docs を先に actualize する |
| final modal foundation / final source marker | syntax / modality | partial basis + stronger family keep を維持する |
| final parser grammar / final public parser-checker-runtime API | public surface | this line では凍らせない |

## true user-spec residuals

| item | impact | current recommendation |
|---|---|---|
| shared-space exhaustive final catalog beyond minimal working subset | shared-space / room-profile final shape | minimal working subset default を保持し、exhaustive catalog は user-spec residual に残す |
| installed-binary / packaging / FFI / engine adapter / host integration target | backend / distribution / external embedding | repo-local runnable CLI + tests + emitted artifacts + reproducible compare floor を near-end success とし、distribution / embedding target は later に残す |
| upper-layer application target beyond authoritative-room first scenario | broader app realization | authoritative turn-based room first を保持し、それ beyond は user goal に応じて reopen する |

## next reopen order

1. Package 67 で checker payload public-schema sketch ratchet を進める。
