# 証明から実装へ進む条件

## 証拠を区別する

少なくとも `design-only`, `hand-proof`, `kernel-checked-general`, `bounded-model`, `runtime-test`, `oracle-review`, `implementation-correspondence`, `unresolved` を別の属性として記録する。列挙はこの作業用であり、Canonの公式分類を勝手に置換しない。

proof assistantを使ったことと一般定理の成立は別である。固定例を`decide`した結果を任意のsource・構成に対する保存へ拡大しない。SMTのUNSATと、独立kernelで検査された一般証明も区別する。

## theory gate

production実装が依存する前に、その増分について以下を満たす。

1. **表現**：具体的な目的の正例を通常の構成規則で表せる。普通の代入、handle、局所理論、動的追加を専用native helperへ追い出していない。
2. **定義**：型・状態・遷移・前提・環境干渉・失敗・観測・更新の該当範囲を明示する。
3. **アルゴリズム**：checkerを「正しいものを返す関数」とだけ定義しない。実際の受理・拒否条件を与える。
4. **定理**：健全性、必要な相対完全性、保存、合成、source/implementation対応の量化範囲を明示する。安全性・全成功・進行性・情報流を混ぜない。
5. **機械検証**：基礎の依存coneにある主要補題をLean等で検査し、未証明前提を監査する。toolchainを固定し、再現コマンドとlogを保存する。
6. **否定的検査**：過剰拒否、仮定の不足、証拠のすり替え、型消去、失効、干渉について反例を試す。statementの妥当性もoracleに確認する。
7. **実装への責任**：抽象的な原子性・公平性・信頼head等の前提を、どの実機構が担うかを記録する。未実現の前提を完了扱いしない。

### Lean等について

現在のローカルtoolchainとrepoのlean-toolchain/lake設定を確認し、既存環境を優先する。入っていなければ、許可された場所へofficial toolchainを取得し、容量とネットワーク権限を確認する。以前のChatGPT sandboxで導入できなかったことを、今回も不可能と推定しない。

`sorry`, `admit`, 目標命題を新公理で仮定すること、結論と同値の型class前提を置いて「証明」と呼ぶことは禁止。`#print axioms`等で依存を確認し、標準の論理公理・商・choice等と、Mir固有の未証明前提を区別する。`native_decide`等が追加する信頼を使う場合も明示する。schemaやDataへglobal invariantを丸ごとproof fieldとして押し込み、どんな遷移も自明に安全に見せない。

有限の入力profileにも一般定理は書ける。任意有限集合・任意適合source・任意有限遷移列という量化を、三つの固定record等へ縮小して一般解消扱いしない。

### proof toolが使えない場合

導入失敗の実エラーを確認し、現実的な代替を試す。Rocq/Isabelle等へ変更する場合は対象の証拠を実際に検査する。SMT/unit testへ自動降格してtheory gateを通さない。独立した定義・反例研究は続けるが、その一般証明に依存するproduction作業を解禁しない。

## implementation gate

対応するtheory gate後に、既存Rust/QUIC等の最小incrementを実装する。型、source/contract identity、現在の認可、linearization、pending/result、failure、observerの各実境界を明示して対応表を作る。

試験は正例と反例、static拒否とruntime拒否、実networkとモデル、終了とtimeout、成功とunknownを区別する。差分の依存coneに合わせて回帰を実行する。別々のサブシステムのJSONを後から結合して統合受入れにしない。

理論の変更が必要なら、必要な命題と反例へ戻る。既存実装の都合に定義を合わせて、その影響を隠さない。
