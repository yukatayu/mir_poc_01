# progress

最終更新: 2026-04-27 13:22 JST

## この文書について

- この文書は repo 全体の **rough progress snapshot** です。
- 規範判断の正本は `specs/`、長期参照は `plan/`、runnable sample dashboard は `samples_progress.md`、詳細証跡は `docs/reports/` にあります。
- 進捗率は **repo-local alpha-ready current layer**、**Mirrorea future-axis docs-first integration**、**phase/sample/progress/storage foundation** に scoped した rough estimate であり、final public completion を意味しません。

## project axis

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

## current snapshot

- execution lane:
  `Macro 4` は active clean near-end suite と Sugoroku world vertical slice を中心に runnable
- theory / proof lane:
  `Macro 5` は finite-index typing / order-handoff / model-check second line / Lean foundation の repo-local alpha floor に到達
- sample/progress/storage lane:
  `samples_progress.md` と storage guardrail script に加え、mounted external workdir と `target/` SSD cutover を確認した
- Mirrorea future lane:
  current reading は `Sugoroku sample progress alignment` を先に置き、その次の semantic carrier package を `TermSignature registry / debug output` と読む
- reserve integration lane:
  real transport、final public contract、packaging、final public auth / visualization / projection / hot-plug surface は still later

## 現在の一言での読み

2026-04-27 時点の repo は、
**clean near-end current layer と Sugoroku world vertical slice を runnable floor として維持しつつ、
phase/sample/progress/storage foundation を追加し、
Mirrorea future-axis を sample-first / docs-first queue として組み替えた**
段階です。

ただし、次はまだ終わっていません。

- final public parser grammar
- final public parser / checker / runtime / verifier API
- final public theorem / model-check / witness-provider contract
- final public auth / visualization / projection / hot-plug API
- real network / multi-server consensus / durable distributed commit
- packaging / installed binary / FFI / engine adapter

## 3 軸 progress

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 現在の読み |
|---|---:|---:|---:|---|
| Mir core / current-L2 | 93% | 90% | 81% | finite-index current layer は強いが、public parser / API は未完 |
| order / handoff / cut family | 92% | 95% | 80% | high-level relation line は動くが、public wording と contract は残る |
| theorem / model-check boundary | 95% | 94% | 89% | repo-local bridge は強いが、production binding は未完 |
| Lean foundations / proof spine | 89% | 92% | 69% | small proof fragment はあるが、full discharge ではない |
| Sugoroku world / current shared-space sample | 84% | 83% | 67% | logical multi-place sample は runnable だが、real transport / durable commit は未着手 |
| phase/sample dashboard | 80% | 86% | 63% | phase 0〜16 matrix は作成したが、future rows の active helper actualization はこれから |
| storage / external workdir guardrail | 66% | 74% | 61% | mount と `target/` cutover は入ったが、cargo registry cache / LLVM actual probe はまだ |
| Mirrorea future axis | 58% | 64% | 15% | typed external boundary / auth / visualization / projection / hot-plug の方向と queue は整理済みだが、実装はこれから |

## macro phase map

| Macro phase | 主眼 | 現在位置 | rough progress % | 自走可否 |
|---|---|---:|---:|---|
| `Macro 0` | repository memory / docs / traceability | samples dashboard + storage foundation active | 98% | 着手可能 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 87% | 着手可能 |
| `Macro 2` | parser-free validation substrate | late | 88% | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | late | 84% | public parser / API 以外は着手可能 |
| `Macro 4` | executable sample floor | active clean suite | 98% | 着手可能 |
| `Macro 5` | typed / theorem / model-check bridge | repo-local alpha-ready current layer | 97% | public seam 以外は自走可能 |
| `Macro 6` | shared-space / fabric boundary | Sugoroku sample + future-axis queue defined | 87% | production transport / final catalog 以外は自走可能 |
| `Macro 7` | toolchain / backend / host integration | mixed | 66% | mounted workdir と target cutover までは自走済み |
| `Macro 8` | application realization | early | 20% | 要仕様確認 |

## feature family snapshot

| feature family | 現在地 | できていること | まだ残ること |
|---|---|---|---|
| executable sample corpus | `S6` | active clean suite 16 本、family run、matrix、closeout | final public sample catalog |
| finite-index typing | `S6` | authority / label / capture / region / cost finite theory | final typed source principal と public checker contract |
| order / handoff | `S6` | witness / publication / handoff relation family、negative sample も整備 | final source wording と public artifact contract |
| model-check second line | `S5-S6` | Peterson SC pass、relaxed CE、broken mutex CE | concrete external tool binding |
| Lean foundations / proof spine | `S6` | small actual proof fragment、generated stub corpus | full domain discharge と public proof contract |
| Sugoroku world / current shared-space sample | `S5-S6` | runtime attach / membership / witness / handoff / leave / reset model-check が runnable | detach lifecycle、real transport、durable evidence |
| samples progress dashboard | `S3-S4` | phase 0〜16 matrix、blocker table、recent validation、storage row | finer per-sample row split、future row actualization |
| storage / detachable workdir guardrail | `S4` | audit、env script、detach prepare、cleanup script、mounted workdir、`target/` cutover | cargo registry cache relocation、LLVM probe、broader generated-artifact policy |
| typed external effect / adapter boundary | `S2-S3` | core-free I/O principle と typed adapter direction を docs に固定 | carrier / helper / sample actualization |
| layer composition / auth / visualization / projection / hot-plug | `S1-S2` | package queue と stop line を固定 | TermSignature、LayerSignature、envelope、viewer、projection、AttachPoint actualization |

## 着手可能 / 要仕様確認 / 後段依存

| 項目 | 状態 | 理由 |
|---|---|---|
| `Sugoroku sample progress alignment` | 着手可能 | existing helper / debug surface / reports を `samples_progress.md` に tighter に結び直せる |
| `Avatar fairy follow sample plan` | 着手可能 | prototype anchor はあるが active sample/helper がないため、先に plan を固める価値が高い |
| `TermSignature registry / debug output` | 着手可能 | current helper / sample / emitted artifact から signature carrier を docs-first に切り出せる |
| `LayerSignature system` | 着手可能 | law surface と layer composition は public API を凍らせず整理できる |
| `MessageEnvelope / Auth seam` | 着手可能 | `auth none` / local queue baseline から narrow に始められる |
| `Typed external boundary / adapter` sample plan | 着手可能 | stdio を core primitive にしない current line を sample-first に具体化できる |
| `Projection / placement plan` | 着手可能 | docs-first validity line を先に切れる |
| `HotPlug Patch / AttachPoint` | 後段依存 | signature / layer / projection line の下敷きを使った方が手戻りが少ない |
| `Network transport` | 後段依存 | envelope / auth seam / projection なしで進めると drift しやすい |
| final public auth / visualization / hot-plug API | 要仕様確認 | public contract と retention / ecosystem target が未定 |
| packaging / FFI / broader application target | 要仕様確認 | distribution target と acceptance criteria が未定 |

## 再現性アンカー

- docs / hierarchy:
  `python3 scripts/check_source_hierarchy.py`
  `python3 scripts/validate_docs.py`
- active suite smoke:
  `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  `python3 scripts/clean_near_end_samples.py smoke-all --format json`
- active suite closeout:
  `python3 scripts/current_l2_guided_samples.py closeout --format json`
- family run:
  `python3 scripts/clean_near_end_samples.py run typing --format json`
  `python3 scripts/clean_near_end_samples.py run order-handoff --format json`
  `python3 scripts/clean_near_end_samples.py run model-check --format json`
  `python3 scripts/clean_near_end_samples.py run modal --format json`
- Lean sync:
  `python3 scripts/current_l2_lean_sample_sync.py`
- Sugoroku world vertical slice:
  `python3 scripts/sugoroku_world_samples.py check-all`
  `python3 scripts/sugoroku_world_samples.py model-check`
  `python3 scripts/sugoroku_world_samples.py closeout --format json`
- storage:
  `bash scripts/storage/setup_mirrorea_workdisk_root.sh --plan`
  `bash scripts/env/mirrorea_storage_env.sh`
  `bash scripts/storage/detach_prepare.sh`
  `bash scripts/storage/cleanup_disposable_artifacts.sh --list`

## recent log

- 2026-04-22 23:53 JST — clean near-end active suite を `samples/clean-near-end/` に切り替え、finite-index typing・order/handoff・model-check second line・Lean sync・crate test・docs validation を再実行して repo-local alpha current layer を同期した。
- 2026-04-23 00:07 JST — stale reference を active lane から除去し、guided sample / clean suite / Lean sync / docs validation を fresh 実行して alpha-ready 判定の evidence を固定した。
- 2026-04-23 08:43 JST — active suite family output、closeout、Lean manifest を基準に、README / Documentation / progress / tasks / research_abstract を日本語で全面刷新する作業に着手した。
- 2026-04-23 08:57 JST — `research_abstract` を summary / `_detail` 構成に再編し、typing / order-model / modal / Lean の actual sample code、actual output、built-in / user-defined 境界を文書化したうえで、active docs に対する stale-reference scan と docs validation を通した。
- 2026-04-23 14:32 JST — Sugoroku world runtime attachment vertical slice を追加し、single-process logical multi-place emulator、10 sample、CLI、model-check、hands-on docs を実装・検証した。
- 2026-04-23 22:54 JST — `mir_hilight.html` を repo root に追加し、active `.mir` sample 全体を single-file browser viewer で Solarized Dark 標準 highlight、theme 切替、行番号、スマホ対応として読めるようにした。
- 2026-04-23 23:04 JST — `mir_hilight.html` に browser-local custom source input を追加し、任意の Mir 風コードを同じ highlighter で preview できるようにした。
- 2026-04-27 09:13 JST — Mirrorea future-axis handoff を docs / specs / plan / AGENTS / progress / tasks に統合し、package 1 audit と package 2 reporting discipline を close したうえで、docs validation、clean near-end smoke / closeout、Sugoroku closeout、`cargo test -p mir-ast`、`cargo test -p mir-runtime`、`cargo test -p mir-semantics` を通し、next queue を `TermSignature registry` に昇格させた。
- 2026-04-27 10:14 JST — phase 0〜16 runnable sample dashboard `samples_progress.md`、source hierarchy check、storage env / detach-safe cleanup script、storage audit、current-L2 guided smoke / closeout、Sugoroku `check-all` / closeout を同期し、current near-term reading を `Sugoroku sample progress alignment` -> `Avatar fairy follow sample plan` -> `TermSignature registry / debug output` へ組み替えた。
- 2026-04-27 10:19 JST — docs / report closeout を `cf5134c` として push し、`python3 scripts/validate_docs.py` で 911 reports、`python3 scripts/check_source_hierarchy.py` で required 23/23 present を確認して phase-sample-progress-storage foundation の repository snapshot を固定した。
- 2026-04-27 13:12 JST — extra SSD `/dev/vdb` が 200G raw disk として認識されていること、ただし current session では `sudo` password requirement により format / mount / `fstab` 編集は未実行であることを確認し、root setup helper `scripts/storage/setup_mirrorea_workdisk_root.sh` と report `0914` を追加した。
- 2026-04-27 13:20 JST — user 実行の root setup 後、`/dev/vdb1` ext4 `mirrorea-work` が `/mnt/mirrorea-work` に mounted され、UUID-based `fstab` 永続化が入っていることを確認し、`target/` 5.2G を SSD 側へ移して symlink 化し、`cargo test -p mir-ast --no-run` で externalized target の軽量確認を通した。
