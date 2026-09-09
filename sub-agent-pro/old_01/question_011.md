# Question 011 — Mirrorea の route proof / overlay / patch activation / suspended task

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- Mirrorea は logical naming、routing、overlay、patch application、path proof、audit、reconfiguration を担う fabric として位置づけています。
- ただし route proof representation、patch activation の protocol detail、suspended task/coroutine と route change の相互作用は未決です。
- repo の core invariants には no shadowing、compatibility-preserving overlay、safe downstream addition、explicit cut activation があります。

## 相談したいこと

Mirrorea の docs-first boundary から operational semantics / protocol line へ進むとき、どの最小 artifact と rule を先に固定すべきでしょうか。

知りたいのは次です。

1. path proof / route proof はどんな shape が自然でしょうか。
2. overlay compatibility をどこまで static / docs / audit で表し、どこから runtime policy に任せるべきでしょうか。
3. patch activation を explicit cut に結びつける場合、in-flight task / suspended coroutine / partially observed route の扱いをどう設計すると良いでしょうか。
4. 単一 consensus algorithm に縛られないまま、Mirrorea の最小 protocol surface をどう切るのが自然でしょうか。

## 期待する回答

- docs-first で先に切るべき artifact と、後段 protocol work を分けてください。
- no-shadowing と audit trace を壊さない最小設計を提案してください。
