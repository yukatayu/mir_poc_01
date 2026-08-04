---
id: root/north-star
status: L0-frozen
maturity: draft
depends_on: [root/glossary]
summary: 軸・非軸の 1 枚。プロジェクトの起点、五動詞の軸、地平、世界観の要約。
open_items: []
---

# North Star — 軸と非軸

## 起点

> 複数の計算機がネットワーク越しに通信し合うシステムを組むとき、通信用インターフェースを先に取り決めてから実装するのが通例だが、それは(開かれた WebAPI 等を除けば)実装の結果生まれるものであることが多く、**早すぎる具体化**ではないか。

したがって: 意味のある一塊のコード(実行場所によって「色付け」される)を書けば、各ノード向けのコードと通信が**導出**されるべきである。通信境界・API 境界は設計の出発点ではなく、検証済みの意味からの**射影の産物**である。

## 軸(五動詞)

**source-level に普通に書かれた一枚のシステムの意味を、正しい理論に基づいて、**

1. 正しく**配置**し(placement / elaboration)
2. 正しく**通信**させ(generated, visible communication)
3. 正しく**検証**し(static / model-check / proof の三線)
4. 正しく**観測**でき(typed, redacted observation)
5. 正しく**進化**できる(patch / hot-plug、capstone)

**仮想空間システムを作る。** hot-plug は幹ではなく capstone である(ADR-0006、plan の Gate 順)。

## 世界観の要約(理論への写像)

- 読みは**依存**、書きは**出来事**(Elm 的直感の定式化。theory/01)。
- 状態はシステム全体で親子・依存・寿命を持ち、fallback により「最悪でも先祖の値へ**単調に劣化**する」参照が書ける。復帰は暗黙でなく明示的再取得(ADR-0004、theory/06)。
- 世界は 4 本のグラフで読める: 出来事 DAG / 存在 DAG / locus-admission グラフ / パッチ DAG(theory/00)。
- イベント駆動の代償として検証を強く入れる。ただし Mir は定理証明器そのものにはならない(三線分離、theory/00・spec/03)。
- 認証・runtime policy は付け外し可能な**型付き Contract 変換層**である。一方、
  verifier は Judgment / Obligation から Evidence / Diagnostic /
  ResidualObligation へ出る別の線であり、観測は型付き information effect である
  (theory/02, 05, 07)。
- プログラムは「ブラウザ的なもの」で参加し、View は FFI へ切り出し、ロジックは Mir 内に置く(architecture/05, BND-007)。

## 非軸(やらないこと)

- OS / 物理ネットワークスタックの置き換え。単一 consensus・単一ゲームエンジンへのコミット。
- `World` / `Room` / `Avatar` を core primitive にすること(ADR-0001)。
- Event を表層プログラミングモデルにすること(ADR-0002)。
- 通信 IF を設計の起点にすること。provider / transport / role 名を権限にすること(ADR-0005)。
- eval による hot-plug(ADR-0006)。全 subsystem の早期な単一 runtime 統合。

## 地平(canon v1 の外周)

VR ソーシャル空間、同期 Web ビュー、協調エディタ、知識空間、Reversed Library(裏返った図書館)。これらは S5 のアプリケーションであり、下位層を application-specific な仮定に潰さずに支えられることが下位層の成功条件である。PrismCascade / Typed-Effect Wiring Platform は独立衛星(architecture/05)。
