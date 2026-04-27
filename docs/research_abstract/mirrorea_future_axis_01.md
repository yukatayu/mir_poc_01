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
future queue では `MessageEnvelope / Auth seam` として transport insertion seam を整理します。

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

### hot-plug

Mirrorea の中心は runtime hot-plug です。

- Patch
- AttachPoint
- compatibility check
- activation cut
- migration contract

を今後の principal package として扱います。

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

次の promoted queue は次です。

1. current-state audit
2. AGENTS.md and reporting discipline
3. TermSignature registry
4. LayerSignature system
5. MessageEnvelope / Auth seam
6. VisualizationProtocol
7. Sugoroku vertical slice hardening
8. Avatar fairy follow slice
9. Projection / placement plan
10. Hot-plug Patch / AttachPoint
11. hands-on docs
12. closeout

このうち 1 と 2 は documentation / policy package です。
3 以降は順に repo-local actualization へ進めます。

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
