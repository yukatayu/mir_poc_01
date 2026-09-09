# Question 010 — shared-space identity / admission / authority / fairness の整理

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- shared-space は current repo では docs-first boundary までは進めていますが、operational realization はまだ後段です。
- すでに docs-first で切っているものには、identity/auth layering、admission/compile-time visibility、authority/resource ownership などがあります。
- ただし final catalog、fairness、consistency mode、activation policy、control-plane protocol は未決です。

## 相談したいこと

shared-space line を本格化するなら、どの catalog から先に formalize / docs-first actualize するのが自然でしょうか。

特に悩んでいるのは次です。

1. identity / principal / membership / incarnation / activation_state の整理
2. admission policy と compile-time visibility の境界
3. authority placement、delegation、resource ownership の扱い
4. fairness / consistency / replay / recovery obligation をどの層へ置くか

また、VR social / collaborative / Reversed Library のような上位 application に寄せる前に、shared-space 自体で固定すべき最小カタログは何でしょうか。

## 期待する回答

- shared-space final catalog を、今後の検討順で分割してください。
- 「研究だけで self-driven に詰められる部分」と「user goal が必要な部分」を分けてください。
