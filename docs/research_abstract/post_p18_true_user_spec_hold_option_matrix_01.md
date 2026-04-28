# post-`P18` true user-spec hold option matrix 01

## この文書の役割

この文書は、`P18` repo-side first cut の後に残る
**true user-spec hold line** を reader-facing に短く読む summary です。

- final public freeze を宣言する文書ではありません
- actual product target を決める文書でもありません
- option inventory と provisional recommendation を current reading として束ねます

## current reading

`P18` で閉じたのは repo-side framing です。
actual commitment は still later であり、`U1` では次の 4 軸を explicit にします。

| Axis | Options | Current recommendation |
|---|---|---|
| packaging shape | `CLI` / `library` / `engine-adapter` / `hybrid` | `library-first` |
| host integration target | `browser` / `native process` / `engine` / `mixed` | `native process` |
| first shipped public surface | core first / integration first / `two-step split` | `two-step split` |
| final shared-space operational catalog | `minimal subset` / portal-multi-world / fairness-quorum-exhaustive | `minimal subset` keep |

## why these recommendations

- installed binary promotion は current shell actualization と分ける guard が既にある
- current host-boundary evidence は process-boundary / same-process sideが strongest であり、
  FFI / engine adapter は later gate に残っている
- parser/checker/runtime/verifier と adapter/viewer/projection/hot-plug/transport を
  1 bucket にして first ship を決めると drift しやすい
- shared-space は authoritative-room minimal subset が source-backed で、
  portal / multi-world / fairness / quorum は still later の heavy line に残っている

## stop line

- `U1` option matrix を final decision と呼ばない
- `library-first` を installed binary adoption と読み替えない
- `native process` recommendation を final host schema と読み替えない
- `two-step split` を final public API freeze と読み替えない

## 関連文書

- `mirrorea_future_axis_01.md`
- `public_api_parser_gate_plan_01.md`
- `../hands_on/post_p18_true_user_spec_hold_01.md`
- `../../plan/28-post-p18-true-user-spec-hold-option-matrix.md`
