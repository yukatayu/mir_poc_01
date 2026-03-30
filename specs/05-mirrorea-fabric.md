# 05 — Mirrorea Fabric

## 目的

Mirrorea は、Mir で定義された system を実ノード上で実行し、その意味論の下で安全に進化させるための制御・経路付け fabric である。

## 主な責務

1. **論理名と routing**
   - client は logical name に対して話しかける。
   - logical name の背後にある physical route は、制御されたルールの下で変更されうる。

2. **Overlay insertion**
   - service は compatibility-preserving overlay で包まれうる。
   - Overlay は inspect、transform、log、rate-limit、authenticate、reject を行ってよいが、以前に定義された API を shadow してはならない。

3. **Patch application**
   - 推奨モデルは downstream addition である。
   - patch activation は明示的な cut で行われるべきである。

4. **Path proof と audit**
   - request が必要な overlay を通過したことを証明できるべきである。
   - audit trace は route change と execution history を説明できなければならない。

5. **Scaling と再構成**
   - dynamic scaling と route rebinding は、同じ意味論 discipline に適合しなければならない。

## すでに議論されている原則

### No shadowing

route または overlay が、以前は到達可能だった interface を、型のない "no service" error として単純に消し去ってはならない。
request が拒否される場合、その rejection は元の、あるいは明示的に version づけられた contract space の中で表現可能でなければならない。

### compatibility-preserving overlay

現時点の方向性:

- precondition を強めない
- guarantee を弱めない
- 明示的な contract change なしに time / resource budget を悪化させない
- failure behavior を合意された failure space の中に保つ

### 既存システムとの統合

Mirrorea は tunnel / proxy / adapter を用いて legacy system を包める（wrap できる）べきである。
ただし、それが理論上 Mir の「内部」にあるのか、単なる operational wrapping なのかは、各ケースで明確に述べなければならない。

## 重要な未解決事項

- route change が単一の consensus mechanism にどれほど強く結びつくべきか（そのように仕様化すべきではない）。
- path proof をどう表現し、どう検証するか。
- in-flight coroutine / task state が route change や patch activation をまたぐときにどう振る舞うか。
