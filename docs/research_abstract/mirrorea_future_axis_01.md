# Mirrorea future axis 01

## この文書の役割

この文書は、Mirrorea の **次の docs-first / repo-local integration line** を日本語で短く読むための summary です。

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- 実行証跡は `docs/reports/`
- ここは current reading と next package queue を reader-friendly にまとめる入口です

## 現在地

2026-04-27 時点で repo が実装済みの current line は次です。

- clean near-end suite
- finite-index first strong typing layer
- order / handoff relation family
- model-check second line
- Lean foundations / generated stub
- Sugoroku world runtime attachment vertical slice

これらは **repo-local alpha-ready current layer** です。
Mirrorea full runtime、final public API、real network、final auth stack、final visualization API まで完了したことを意味しません。

同時に、Mirrorea future-axis 側では `TermSignature registry / debug output` の first cut を close しました。

- Sugoroku helper に `--debug signatures`
- clean near-end report / closeout に `term_signatures` と `signature_kinds`

を追加し、helper-local / report-local の evidence carrier を先に actualize しています。

さらに `LayerSignature system` の first cut も close しました。

- Sugoroku helper に `--debug layers`
- clean near-end report / closeout に `layer_signatures`

を追加し、helper-local current layer と runtime report-local lane を
どちらも `requires / provides / transforms / checks / emits / laws` で読めるようにしました。
ただし helper と runtime で current layer 名はまだ揃えていません。これは first cut であり、
final public layer law schema を意味しません。

さらに `MessageEnvelope / Auth seam` の first cut も close しました。
さらに `VisualizationProtocol` の first cut も close しました。
さらに `Typed external boundary / adapter` の docs-first sample plan も close しました。
さらに `Projection / placement` の docs-first plan も close しました。
さらに `HotPlug Patch / AttachPoint` の docs-first plan も close しました。
さらに `Network transport` の docs-first plan も close しました。

- Sugoroku helper に `message_envelopes` と `--debug envelopes`
- clean near-end report / closeout に `MessageEnvelope` inventory

を追加し、current none-auth baseline のまま transport / auth / membership / capability / witness を
separate lane で読めるようにしています。
ここでの `auth none` は temporary repo-local baseline であり、session token / signature /
federation protocol を fixed した意味ではありません。

## 主軸

守るべき project axis は次です。

```text
正しい理論に基づき、
正しく hot-plug でき、
Place をまたいで実行・通信・検証・可視化できる
仮想空間システムを作る。
```

局所最適化でこの軸を崩してはいけません。

## 高位の読み

### Place

`Place` は participant そのものではありません。

- physical process
- machine / node
- virtual thread
- authority / permission compartment
- state / queue / visibility / observation frontier を持つ execution locus

したがって `Alice is a Place` ではなく、
`Alice` は participant / principal、
`ParticipantPlace[Alice]` は Place と読みます。

### external world boundary

- standard I/O は Mir core primitive ではありません。
- `stdin` / `stdout` / `stderr` を core built-in として固定しません。
- external world とは typed external effect adapter で接続します。

ここでいう adapter は、unsafe FFI をそのまま privileged 化するものではなく、
contract と effect を保った boundary です。

### transport と auth の分離

次の層は潰さずに分けます。

- transport
- authentication
- authorization
- membership
- capability
- witness / audit

current repo-local line では Sugoroku sample が membership epoch / incarnation / witness を先に示しており、
helper-local / report-local first cut では `MessageEnvelope` carrier で transport insertion seam を visible にしました。
未決なのは final public `AuthEvidence` kind、real transport、session / signature schema です。

### visualization / telemetry

可視化は optional debug toy ではありません。
ただし visualization も telemetry も、外へ情報を出す effect です。

そのため次を要求します。

- label
- authority
- redaction
- evidence-oriented rendering

`untyped debug leak` として出してよいものではありません。

### projection / placement

高位の source から、後で place-specific program へ projection できる性質を保ちます。

- server-side authoritative path
- participant-side view / local debug path
- adapter path
- visualizer path

のどれかに source principal を早期固定しないことが重要です。

2026-04-27 時点では、`plan/20-projection-and-placement-roadmap.md` に
system-wide source / place-specific program distinction、place split、validity checklist、stop line を置いてあります。

### hot-plug

Mirrorea の中心は runtime hot-plug です。

- Patch
- AttachPoint
- compatibility check
- activation cut
- migration contract

を今後の principal package として扱います。

2026-04-27 時点では、`plan/21-hotplug-attachpoint-roadmap.md` に
compatibility checklist、activation cut、migration stop line、`SUG-01` attach と `SUG-09` detach TODO boundary を置いてあります。
storage detach script は operational cleanup concern であり、この runtime hot-plug lifecycle そのものではありません。

### network transport

transport widening は `local_queue` baseline をそのまま final public transport ABI にしないための docs-first line です。

- loopback
- reconnect
- membership epoch / member incarnation guard
- typed transport failure
- redacted route trace

2026-04-27 時点では、`plan/22-network-transport-roadmap.md` に
current anchor、`NET-01..05` planned family、transport widening invariant、stop line を置いてあります。
ここでも transport と auth / membership / capability / witness / visualization を collapse しません。

## representative slices

### current

- Sugoroku world runtime attachment vertical slice

### next candidate

- avatar fairy follow vertical slice

avatar fairy follow では、`FollowAnchor`、visibility guard、local fallback、stale anchor rejection、
object attach/follow lifecycle を representative に検証する予定です。

## 文書の読み分け

混同しやすいものは次のように分けて読みます。

- 規範 spec:
  `specs/`
- repository memory:
  `plan/`
- historical report:
  `docs/reports/`
- current sample:
  `samples/clean-near-end/`
- historical old sample:
  `samples/old/`
- helper-local debug output:
  script の `--debug` や detached artifact
- final public API:
  **まだ deferred**
- deferred mixed gate:
  parser/public API、auth/public contract、visualization/public contract、projection/public API、hot-plug/public API

## next package queue

初期の audit / reporting discipline と、future-axis の first-cut package は次まで close 済みです。

- `TermSignature registry / debug output`
- `LayerSignature system`
- `MessageEnvelope / Auth seam`
- `VisualizationProtocol`
- `Typed external boundary / adapter` docs-first sample plan
- `Projection / placement` docs-first plan
- `HotPlug Patch / AttachPoint` docs-first plan
- `Network transport` docs-first plan

現在の promoted queue は次です。

1. `Compiler/backend/LLVM preparation`
   - external workdir、cache placement、detach-safe cleanup、small-VPS guardrail を整理する package
2. `hands-on docs / closeout`
   - 日本語 docs、`samples_progress.md`、`progress.md`、`tasks.md`、reports を再同期する package
3. `Avatar fairy follow` representative slice
   - `samples/not_implemented/avatar-fairy-follow/` の skeleton family を active helper / validation line へ昇格できるかを検討する後段 package
4. `Network transport` executable widening
   - `plan/22` を helper / runtime / loopback proof-of-concept へ widen する後段 package

この queue は repo-local current reading であり、final public package structure や public API freeze を意味しません。

## stop line

この summary を読んでも、次を「すでに実装済み」とは読まないでください。

- real network transport
- multi-server consensus
- durable distributed commit
- final public auth protocol
- final public visualization API
- final public projection API
- final public hot-plug API
- final parser / checker / runtime / verifier API
- final product completion
