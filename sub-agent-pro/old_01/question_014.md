# Question 014 — public parser/checker/runtime API と CLI packaging

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- current repo では public surface inventory を済ませており、already-public parser-free stack、crate-public but non-production tranche、repo-local helper surface の 3 bucket split を持っています。
- later gate として、runtime-led thin facade、public operational CLI second gate、concrete shell naming / actualization まで narrow に actualize しています。
- ただし final public parser / checker / runtime API と installed binary / final host contract はまだ later です。

## 相談したいこと

今の thin facade / concrete shell の方向を踏まえたうえで、final public surface をどんな形に収束させるのが自然でしょうか。

知りたいのは次です。

1. final public parser/checker/runtime API は、最初から統一 surface を目指すべきでしょうか。それとも parser / checker / runtime を別 public crate / module として出すべきでしょうか。
2. public operational CLI は、いつ narrow helper shell から installed binary / final contract へ昇格させるべきでしょうか。
3. JSON / pretty / artifact output / host-plan input は public contract にどう入れるのが自然でしょうか。
4. current runtime-led thin facade path は良い中間段階でしょうか。

## 期待する回答

- final public surface の staged packaging strategy を提案してください。
- 今の repo がまだ public にしない方がよいものも明示してください。
