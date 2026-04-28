# plan/17 — research phases and autonomy gates

## macro phase 一覧

| Macro phase | 主眼 | 現在位置 | 重さ | autonomy gate |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance + future-axis sync | 低 | self-driven |
| `Macro 1` | semantic kernel / invariant stabilization | late | 中 | self-driven |
| `Macro 2` | parser-free validation substrate | late | 中 | self-driven |
| `Macro 3` | compile-ready minimal actualization | late | 中 | self-driven up to public-API residual |
| `Macro 4` | executable sample floor | active on clean near-end floor | 重 | self-driven |
| `Macro 5` | typed / theorem / model-check bridge | repo-local alpha-ready current layer | 重 | self-driven up to public contract |
| `Macro 6` | fabric / shared-space / runtime evolution | minimal subset default + future-axis queue defined | 重 | self-driven up to final catalog / production transport |
| `Macro 7` | toolchain / backend / host-facing integration | mixed | 重 | mixed beyond repo-local helper |
| `Macro 8` | domain / application realization | early | とても重い | user-spec required |

## current lane split

- execution lane:
  `Macro 4` active on clean near-end suite + Sugoroku world vertical slice
- theory-lab lane:
  `Macro 5` current layer alpha-ready
- sample/progress/storage lane:
  phase-sample-progress-storage-foundation close 後、`samples_progress.md` と external-workdir guardrail を current repository discipline に昇格
- repository-structure lane:
  `plan/19-repository-map-and-taxonomy.md`、`samples/README.md`、`scripts/README.md`、`docs/research_abstract/repository_layer_structure_01.md` で layer-aware repo map を docs-first に固定
- Mirrorea future lane:
  `Macro 6` では `P0` current-state audit、`P1` repository layer map / `samples_progress.md` stabilization、foundation package、Sugoroku per-sample alignment、avatar fairy follow skeleton plan、`TermSignature registry / debug output`、`LayerSignature system`、`MessageEnvelope / AuthEvidence` seam、`VisualizationProtocol`、typed external boundary / adapter docs-first plan、projection / placement、hot-plug docs-first plan、network transport docs-first plan、hands-on closeout、avatar representative slice widening、hot-plug executable widening、transport helper-local canaries、cross-package sweep、`FAIRY-05` residual reacquire design review closeout、typed external synthetic preview helper widening、projection / placement executable widening、typed external residual reopen matrix closeout、projection residual emitted-program gate closeout、`mirrorea-core` first real implementation tranche を close 済みと読む
  - `P4` `TermSignature` registry hardening までは close 済み
  - `P5` `LayerSignature` system hardening も close 済み
  - `P6` `MessageEnvelope / AuthEvidence` seam hardening も close 済み
  - `P7` `VisualizationProtocol / VisualizationSecurity` hardening も close 済み
  - `P8` Sugoroku runtime attach hardening も close 済み
  - `P9` avatar fairy follow hardening も close 済み
  - next promoted package は `P11` logical multi-place runtime tranche
  - next reopen point は `P12` external adapter / host boundary tranche
  - `P11-P17` は first real implementation tranche、`P18` は final mixed gate
- toolchain/backend lane:
  `Macro 7` では mounted workdir、`CARGO_TARGET_DIR`、`CARGO_HOME`、LLVM path readiness、non-destructive cleanup probe を current guardrail として actualize 済み
- reserve / mixed lane:
  `Macro 6 / 7` final public seam、real transport、packaging residual

## self-driven でよい line

- semantic kernel の narrow refinement
- finite-index first-layer hardening
- Lean foundations の small-proof hardening
- order / handoff relation family の docs-first hardening
- model-check second-line corpus maintenance
- TermSignature / LayerSignature / MessageEnvelope / Visualization / Projection / HotPlug の docs-first / repo-local package
- Sugoroku world logical multi-place sample hardening
- sample matrix / progress dashboard / storage guardrail hardening
- snapshot / traceability maintenance

## mixed gate の line

- final typed source principal
- final theorem / model-check public contract
- final witness / provider / emitted-artifact public contract
- final public auth / visualization / projection / hot-plug surface
- real network / consensus / durable commit for Sugoroku-like runtime attachment
- final parser / public checker / runtime / verifier surface
- packaging / installed binary

## user specification が必要な line

- broader application target
- exhaustive shared-space final catalog
- FFI / engine adapter / host integration target

## historical note

pre-clean-near-end package chain は historical phase memory として残す。
current autonomy reading の正本は clean near-end current layer、Sugoroku vertical slice、Mirrorea future-axis queue、residual mixed / user-spec gate である。
