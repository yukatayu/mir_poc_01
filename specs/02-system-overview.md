# 02 — システム概要

## スタック全体の 1 ページ要約

このプロジェクトは現在、層状スタックとして構成された四つの概念システムから成る。

1. **Mir** — 意味論的な言語コア
2. **Mirrorea** — 分散 control / routing / audit fabric
3. **PrismCascade** — offline / live の audiovisual pipeline 向けに分離された media-processing kernel
4. **Typed-Effect Wiring Platform** — software と container のための routable・inspectable・contract-aware な effect boundary

これらは目的の上では独立ではないが、意図的に**同じソフトウェアではない**。
共有すべきなのは、必要最小限の interface、identifier、contract に限られる。

## なぜこのスタックは大きいのか

このプロジェクトは単なる「新しいサービス」ではない。
変えようとしているのは次である。

- **実行と言語進化のための言語**（Mir）
- **分散合成と安全な挿入のための fabric**（Mirrorea）
- **重要かつ performance-sensitive なドメインの kernel**（PrismCascade）
- **運用境界を観測し、rewire する方法**（Typed-Effect Wiring Platform）
- そしてこれらを使って virtual reality social system や Reversed Library のような上位空間を支えること

## 既存システムとの中核的な違い

現在の多くのシステムは、これらの責務を次の層に分散している。

- プログラミング言語意味論
- service mesh / デプロイツール群
- application code
- observability stack
- ad-hoc な運用手順

このプロジェクトは、それらの層を単に運用上両立させるのではなく、**理論的に整列させる**ことを目指す。

## 分離可能であるべきもの

交換可能性と概念的明晰さを保つために、次を守る。

- Mir は media-specific runtime になってはならない。
- PrismCascade は汎用 distributed-language runtime になってはならない。
- Typed-Effect Wiring Platform は言語そのものになってはならない。
- Mirrorea はすべての application logic を吸収してはならない。

## 高水準の関係

- Mir は、計算の意味と進化の制約を**何として定義するか**を定める。
- Mirrorea は、その計算を**どこで、どの route を通して**実行・進化させるかを定める。
- PrismCascade は、**高性能 media domain をどう表現し、計画し、実行するか**を定める。
- Typed-Effect Wiring Platform は、**effect boundary をどう inspectable・rewritable にできるか**、特に統合と運用の文脈でそれをどう扱うかを定める。
