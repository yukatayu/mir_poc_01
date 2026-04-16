# 430 — current L2 malformed duplicate-cluster later reopen comparison

## 目的

capability pair `e13/e20` の source-backed widening close 後に、

- duplicate cluster
- `TryFallback` / `AtomicCut` malformed-static broader family

をどの順序 / cut で reopen するかを比較する。

ここで fixed するのは
**duplicate cluster の kept-later family と next reopen sequencing**
であり、

- duplicate source-sample widening actualization
- typed / mirrored promotion
- broader try-rollback malformed-static promotion

は still later に残す。

## source-backed floor

- duplicate cluster `e14/e15` は static-only actual corpus として already 持つ。
- duplicate cluster は current helper cut では
  `checked_reasons` と detached `reason_codes`
  に昇格していない。
- authored source sample dozen には duplicate rows がまだ含まれていない。
- capability widening package は duplicate cluster と broader malformed-static family を later に残して close 済みである。

## 比較対象

### 案 1. duplicate-cluster reopen sequencing を comparison first に固定する

#### 利点

- `e14/e15` という existing actual corpus anchor を先に整理できる。
- typed / mirrored promotion を immediate に要求しない。
- broader `TryFallback` / `AtomicCut` malformed-static line と混線しない。

#### 欠点

- source sample widening 自体はまだ次 package へ残る。

### 案 2. duplicate source-sample widening を immediate next actualization にする

#### 利点

- source-backed execution lane をすぐ太らせられる。

#### 欠点

- checked-reason / detached-reason non-promotion cut を再点検する package を飛ばしやすい。
- broader malformed-static family との順序 drift を招きやすい。

### 案 3. broader `TryFallback` / `AtomicCut` malformed-static family を先に reopen する

#### 利点

- runtime-side broader malformed line を先に前へ出せる。

#### 欠点

- duplicate cluster には already actual static-only anchors がある一方、broader family は inventory line に留まる。
- duplicate cluster kept-later family の整理を飛ばしやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. duplicate-cluster reopen sequencing を comparison first に固定する**
である。

理由は次の通り。

1. duplicate cluster には already actual static-only anchors `e14/e15` がある。
2. それでも current helper cut では typed / mirrored promotion を避ける split が強く、これを先に明示する必要がある。
3. broader `TryFallback` / `AtomicCut` malformed-static family は duplicate cluster より late inventory 寄りであり、順序を逆にすると drift を招きやすい。

## guard

- duplicate cluster を immediate typed / mirrored promotion にしない。
- duplicate cluster を `checked_reasons` / detached `reason_codes` に昇格させない。
- broader try-rollback malformed-static family を先に mainline へ出さない。

## next promoted line

next promoted line は、
**malformed duplicate-cluster source-sample widening comparison**
に置く。

## what is not decided here

- duplicate source-sample widening の exact cut
- nested-scope / try-fallback duplicate variants
- duplicate cluster の typed / mirrored promotion timing
- broader `TryFallback` / `AtomicCut` malformed-static family の reopen timing
