# plan/17 — research phases and autonomy gates

## macro phase 一覧

| Macro phase | 主眼 | 現在位置 | 重さ | autonomy gate |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | 低 | self-driven |
| `Macro 1` | semantic kernel / invariant stabilization | late | 中 | self-driven |
| `Macro 2` | parser-free validation substrate | late | 中 | self-driven |
| `Macro 3` | compile-ready minimal actualization | late | 中 | self-driven up to public-API residual |
| `Macro 4` | executable sample floor | active on clean near-end floor | 重 | self-driven |
| `Macro 5` | typed / theorem / model-check bridge | repo-local alpha-ready current layer | 重 | self-driven up to public contract |
| `Macro 6` | fabric / shared-space / runtime evolution | minimal working subset default | 重 | self-driven up to final catalog |
| `Macro 7` | toolchain / backend / host-facing integration | mixed | 重 | mixed beyond repo-local helper |
| `Macro 8` | domain / application realization | early | とても重い | user-spec required |

## current lane split

- execution lane:
  `Macro 4` active on base corpus + clean near-end suite
- theory-lab lane:
  `Macro 5` current layer alpha-ready
- reserve / mixed lane:
  `Macro 6 / 7` public-seam residual and packaging residual

## self-driven でよい line

- semantic kernel の narrow refinement
- finite-index first-layer hardening
- Lean foundations の small-proof hardening
- order/handoff relation family の docs-first hardening
- model-check second-line corpus maintenance
- snapshot / traceability maintenance
- Sugoroku world logical multi-place sample hardening

## mixed gate の line

- final typed source principal
- final theorem/model-check public contract
- final witness/provider/emitted-artifact public contract
- real network / consensus / durable commit for Sugoroku-like runtime attachment
- final parser/public checker/runtime/verifier surface
- packaging / installed binary

## user specification が必要な line

- broader application target
- exhaustive shared-space final catalog
- FFI / engine adapter / host integration target

## historical note

pre-clean-near-end package chain は historical phase memory として残す。
current autonomy reading の正本は clean near-end current layer と residual mixed/user-spec gate である。
