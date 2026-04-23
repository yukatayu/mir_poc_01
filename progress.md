# progress

最終更新: 2026-04-23 08:57 JST

## この文書について

- この文書は repo 全体の **rough progress snapshot** です。
- 規範判断の正本は `specs/`、長期参照は `plan/`、詳細証跡は `docs/reports/` にあります。
- 進捗率は **repo-local alpha-ready current layer** に scoped した rough estimate であり、final public completion を意味しません。

## current snapshot

- execution lane:
  `Macro 4` は active clean near-end suite を中心に runnable
- theory / proof lane:
  `Macro 5` は finite-index typing / order-handoff / model-check second line / Lean foundation の repo-local alpha floor に到達
- reserve integration lane:
  `Macro 6` 以降は minimal working subset を維持しつつ、public-seam residual と user-spec residual を分離中

## 現在の一言での読み

2026-04-23 時点の repo は、
**clean near-end active suite と finite-index first layer を中心に、repo-local alpha-ready current layer が動いている**
段階です。

ただし、次はまだ終わっていません。

- final public parser grammar
- final public parser / checker / runtime / verifier API
- full dependent type theory
- concrete theorem / model-check production binding
- final public witness / provider / emitted-artifact contract
- packaging / installed binary / FFI / engine adapter

## 3 軸 progress

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 現在の読み |
|---|---:|---:|---:|---|
| Mir core / current-L2 | 93% | 90% | 81% | finite-index current layer は強いが、public parser / API は未完 |
| order / handoff / cut family | 92% | 95% | 80% | high-level relation line は動くが、public wording と contract は残る |
| theorem / model-check boundary | 95% | 94% | 89% | repo-local bridge は強いが、production binding は未完 |
| Lean foundations / proof spine | 89% | 92% | 69% | small proof fragment はあるが、full discharge ではない |
| Mirrorea / shared-space boundary | 78% | 75% | 59% | minimal subset は読めるが、final catalog は未定 |

## macro phase map

| Macro phase | 主眼 | 現在位置 | rough progress % | 自走可否 |
|---|---|---:|---:|---|
| `Macro 0` | repository memory / docs / traceability | active maintenance | 97% | 着手可能 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 87% | 着手可能 |
| `Macro 2` | parser-free validation substrate | late | 88% | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | late | 84% | public parser/API 以外は着手可能 |
| `Macro 4` | executable sample floor | active clean suite | 98% | 着手可能 |
| `Macro 5` | typed / theorem / model-check bridge | repo-local alpha-ready current layer | 97% | public seam 以外は自走可能 |
| `Macro 6` | shared-space / fabric boundary | minimal working subset default | 79% | final catalog 以外は自走可能 |
| `Macro 7` | toolchain / backend / host integration | mixed | 57% | repo-local まで |
| `Macro 8` | application realization | early | 18% | 要仕様確認 |

## feature family snapshot

| feature family | 現在地 | できていること | まだ残ること |
|---|---|---|---|
| executable sample corpus | `S6` | active clean suite 16 本、family run、matrix、closeout | final public sample catalog |
| finite-index typing | `S6` | authority / label / capture / region / cost finite theory | final typed source principal と public checker contract |
| order / handoff | `S6` | witness / publication / handoff relation family、negative sample も整備 | final source wording と public artifact contract |
| model-check second line | `S5-S6` | Peterson SC pass、relaxed CE、broken mutex CE | concrete external tool binding |
| Lean foundations / proof spine | `S6` | small actual proof fragment、generated stub corpus | full domain discharge と public proof contract |
| shared-space / fabric | `S4-S5` | minimal authoritative-room 読みを維持 | exhaustive final catalog と operational realization |

## 着手可能 / 要仕様確認 / 後段依存

| 項目 | 状態 | 理由 |
|---|---|---|
| parser/public API residual の整理 | 着手可能 | repo-local helper surface と final public surface の境界をさらに狭められる |
| theorem / model-check public seam の narrowing | 着手可能 | helper-local bridge を public contract の手前まで詰められる |
| witness / provider public shape residual の整理 | 着手可能 | `memory_order` exact surface や final catalog に触れず進められる |
| packaging / installed binary / FFI | 要仕様確認 | distribution target と acceptance criteria が未定 |
| broader application target | 要仕様確認 | 上位アプリに何を要求するか user 側の固定が必要 |
| exhaustive shared-space final catalog | 後段依存 | minimal subset を越えるために higher-level product assumption が要る |

## 再現性アンカー

- active suite smoke:
  `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
- active suite closeout:
  `python3 scripts/current_l2_guided_samples.py closeout --format json`
- family run:
  `python3 scripts/clean_near_end_samples.py run typing --format json`
  `python3 scripts/clean_near_end_samples.py run order-handoff --format json`
  `python3 scripts/clean_near_end_samples.py run model-check --format json`
  `python3 scripts/clean_near_end_samples.py run modal --format json`
- Lean sync:
  `python3 scripts/current_l2_lean_sample_sync.py`
- docs validation:
  `python3 scripts/validate_docs.py`

## recent log

- 2026-04-22 23:53 JST — clean near-end active suite を `samples/clean-near-end/` に切り替え、finite-index typing・order/handoff・model-check second line・Lean sync・crate test・docs validation を再実行して repo-local alpha current layer を同期した。
- 2026-04-23 00:07 JST — stale reference を active lane から除去し、guided sample / clean suite / Lean sync / docs validation を fresh 実行して alpha-ready 判定の evidence を固定した。
- 2026-04-23 08:43 JST — active suite family output、closeout、Lean manifest を基準に、README / Documentation / progress / tasks / research_abstract を日本語で全面刷新する作業に着手した。
- 2026-04-23 08:57 JST — `research_abstract` を summary / `_detail` 構成に再編し、typing / order-model / modal / Lean の actual sample code、actual output、built-in / user-defined 境界を文書化したうえで、active docs に対する stale-reference scan と docs validation を通した。
