# 07 — Typed-Effects Wiring Platform

## 目的

Typed-Effects Wiring Platform は、分離されているが密接に関連した project idea である。
その役割は、外部 effect を operational layer で可視化し、型づけし、contract-aware にし、rewritable にすることにある。

## 中核アイデア

program または container は、次を露出する形で宣言されるか、または包まれるべきである。

- どの effect を実行しうるか
- それらの effect がどの data shape を持つか
- それらの effect がどの contract に従うか
- それらの effect がどこへ route されるか

## current direction for external effect boundary

- Mir core は `stdin` / `stdout` / `stderr` のような privileged standard I/O primitive を持たない。
- external world との接続は typed external effect adapter で扱う。
- adapter boundary には transport、authentication、authorization、membership、capability、witness を通せるが、それらを 1 つの untyped pipe に潰してはならない。
- visualization と telemetry も information-bearing effect として扱い、label / authority / redaction を要求する。
- exact contract language、message schema、serialization はなお open だが、境界を typed / inspectable / rewritable に保つ方向自体は current line として扱う。

## それが何ではないか

これは Mir の言語意味論そのものではない。
これは単なる logger ではない。
これは単なる service mesh ではない。
これは typed effect routing / inspection / rewiring のための概念層である。

## Mir との関係

Mir は、effect と contract を定義する意味論言語として捉えられる。
Typed-Effects Wiring Platform は、より低位、または隣接する operational layer として次を担う。

- effect boundary の観測
- それらの routing または rewiring
- state と event history との関係の記録
- 非 Mir system の統合支援

## PrismCascade との関係

PrismCascade はこの platform を次のために利用できる。

- Meta-layer service
- remote execution request
- trace linkage
- 外部 system との制御された統合

## 開かれた問い

- これを Mirrorea の一部とみなすべきか、それともその下の別 subsystem とみなすべきか。
- その contract language のどこまでを Mir syntax と文字どおり再利用すべきか。
- unknown / opaque effect を、安全性を完全には捨てずにどう表現するか。
