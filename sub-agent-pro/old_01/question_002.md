# Question 002 — 最終型理論の位置づけと段階的導入

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- 現在の repo には、ownership / monotone lifetime / effect / contract / capability / fallback / place-local rollback / `atomic_cut` など、型理論的に強い制約に接続しそうな要素があります。
- ただし full 型システムはまだ mainline に入れていません。
- current line では static gate、tool-neutral formal hook、proof/model-check bridge、sample-visible narrow evidence line を先に進めています。

## 相談したいこと

この言語の最終的な型理論は、単純に System F やラムダキューブ上の既知の 1 点として理解するのでは足りない気がしています。
特に次の軸があるからです。

- ownership / linearity
- monotone degradation / fallback admissibility
- effect / contract / capability
- `place`
- rollback / `atomic_cut`
- 将来の authority / shared-space / higher-level ordering family

ここで相談したいのは次です。

1. 最終理論を考えるとき、どの既存理論群の組み合わせとして考えるのが自然でしょうか。
   - linear / affine / capability / effect / modal / temporal / session / separation など
2. この言語を「ラムダキューブ上のどこか」として説明するのは有益でしょうか。それとも別の説明軸の方がよいでしょうか。
3. full theory に入る前に、repo の current line で段階的に固定してよい judgment や kind は何でしょうか。
4. 将来重くなる型理論 line を、今の narrow runnable subset へどう接続すると手戻りが少ないでしょうか。

## 期待する回答

- 最終理論の候補を 2〜3 案に整理してください。
- そのうえで、「今の repo が最初に切るべき最小 typed core」を提案してください。
- もし「ラムダキューブ以外の説明軸の方が適切」なら、その説明軸も示してください。
