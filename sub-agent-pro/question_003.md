# Question 003 — full 型システム前の最小 sample-visible 型/契約 surface

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- repo は full 型システムをまだ mainline に入れていません。
- しかし fixed-subset sample は実行され始めており、sample code 上に「型や契約や検証が見える」段階へ進めたいと考えています。
- current line では static gate、formal hook、proof-notebook pilot、model-check carrier、sample-facing evidence summary までは narrow に actualize されています。

## 相談したいこと

full の強い型システムへ入る前に、sample code と user-facing docs に現れてもよい **最小 typed / contract surface** は何でしょうか。

例えば候補としては、

- effect kind / capability kind の明示
- contract row の明示
- `place` や authority に関する annotation
- fallback / lineage の最小 declared surface
- proof obligation の user-facing label

などがありますが、どこから先に actualize するのが良いでしょうか。

知りたいのは次です。

1. 「full type theory ではないが、sample-visible に出してよい minimal surface」は何ですか。
2. その surface は parser / checker / runtime / formal hook のどの層に最初に現すべきですか。
3. 逆に、まだ sample code に現さない方がよい typed notion は何ですか。
4. 近接 3〜5 package の実装・検証・doc 更新まで含めると、どう刻むのが自然ですか。

## 期待する回答

- minimal surface の候補を優先順で挙げてください。
- 各候補について、
  - user-facing syntax に出すべきか
  - internal carrier だけに留めるべきか
  - formal artifact にだけ出すべきか
  を分けてください。
