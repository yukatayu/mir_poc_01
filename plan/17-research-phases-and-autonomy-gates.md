# plan/17 — research phases and autonomy gates

## 目的

この文書は、repo 全体の長期研究を **phase** 単位で見直すための補助線である。
`plan/10` の全体ロードマップを置き換えるものではなく、

- いまどの phase にいるか
- 各 phase がどれくらい重いか
- どこまで self-driven に進めてよいか
- どこで user 仕様確認が要るか

を、作業順序と autonomy gate の観点で見やすく整理する。

## この文書の読み方

- ここでいう phase は厳密なウォーターフォールではない。
- 複数 phase が並走することはある。
- ただし、**主線** と **side line** は区別する。
- `self-driven` と書いてあるものは、agent が narrow comparison / PoC / docs / regression を進めてよい範囲を指す。
- `user spec required` と書いてあるものは、勝手に finalization すると手戻りが大きい範囲を指す。

## phase 一覧

### Phase 0 — repository memory / decision boundary

#### 主眼

- `specs/` / `plan/` / `docs/reports/` / `progress.md` の役割分離
- L0/L1/L2/L3 の decision level
- invariant / open question / traceability の運用安定化

#### 現在地

- **ほぼ maintenance phase**

#### 重さ

- 低い

#### autonomy gate

- **self-driven で維持してよい**

### Phase 1 — current L2 semantics stabilization

#### 主眼

- fallback / `lease` / `TryFallback` / `AtomicCut` / guarded option chain の安定化
- representative example と prose drift の抑制
- parser に先行する semantics core の固定

#### 現在地

- **終盤**

#### 重さ

- 中程度

#### autonomy gate

- **self-driven で進めてよい**

### Phase 2 — parser-free PoC and detached validation loop

#### 主眼

- fixture / interpreter / host harness / bundle / batch / selection / profile の machine-check loop
- detached artifact の emit / save / compare / smoke
- fixture authoring / elaboration 実務の摩擦低減

#### 現在地

- **終盤。入口は成立済み、いまは運用摩擦の低減フェーズ**

#### 重さ

- 中程度

#### autonomy gate

- **self-driven で進めてよい**

### Phase 3 — parser boundary and first checker cut

#### 主眼

- final parser grammar を決めずに staged parser spike を actualize する
- helper-local compare family と static structural floor を narrow に切る
- parser / checker / runtime を一気に混ぜずに接点を inventory 化する

#### 現在地

- **前半から中盤。stage 1 / stage 3 の first tranche は actualize 済み**

#### 重さ

- 中程度からやや重い

#### autonomy gate

- **self-driven で進めてよい**

### Phase 4 — shared-space / membership / practical example boundary

#### 主眼

- participant carrier
- activation rule
- authority placement
- consistency mode
- RNG / fairness source
- reconnect / leave / rejoin / causal metadata
- practical example での room profile

#### 現在地

- **前半。docs-first boundary はかなり進んだが、final profile は未決**

#### 重さ

- 重い

#### autonomy gate

- **比較・PoC 例示・docs-first boundary までは self-driven**
- **final activation / authority / auth / consistency / fairness catalog は user spec required**

### Phase 5 — static analysis / type / theorem prover / model checker boundary

#### 主眼

- local / decidable judgment を core に入れる候補の整理
- external verifier / theorem prover / model checker に送る候補の整理
- hybrid staged approach の entry criteria 明文化

#### 現在地

- **入口整理段階**

#### 重さ

- とても重い

#### autonomy gate

- **docs-first inventory と small proof obligation 整理までは self-driven**
- **本格 actualization は後段依存**

### Phase 6 — actual parser / checker / runtime commitment

#### 主眼

- parser subset の actual introduction
- checker family の external / internal 境界確定
- runtime / host / artifact API の actual narrow cut

#### 現在地

- **未着手**

#### 重さ

- 重い

#### autonomy gate

- **後段依存**

### Phase 7 — higher-layer integration and domain realization

#### 主眼

- Mirrorea Fabric
- Typed-Effect Wiring Platform
- PrismCascade
- shared-space upper layer
- domain/application design（例: Reversed Library）

#### 現在地

- **未着手**

#### 重さ

- とても重い

#### autonomy gate

- **user spec required**

## 現在の主線

いま repo の主線は、次の 2 本である。

1. **Phase 3 前半〜中盤**
   - parser boundary / first checker cut を staged に actualize する
2. **Phase 2 終盤の maintenance tail**
   - detached validation loop を継続運用しやすい形へ整え、mirror drift を抑える

shared-space line は、これらを壊さない範囲で進める **Phase 4 の side line** である。

## immediate execution order

user 指示を反映した current immediate sequence は次である。

1. **Phase 0 / 1 / 2 の closeout baseline を固定する**
   - repository memory / docs maintenance
   - current L2 semantics drift suppression
   - detached validation loop helper surface の smoke evidence
2. **docs / specs / README / `progress.md` / `plan/` の top-level consistency sweep を入れる**
3. **その後で Phase 3 を主線として進める**
4. **Phase 3 の一区切りで現状整理を行い、一旦止まる**

現在は 1 と 2 の first pass が閉じたので、phase 読みとしては **Phase 3 を主線に戻しつつ、Phase 0 / 1 / 2 は maintenance tail として継続する** と読む。

## 現在の「止めるべき線」

次は self-driven に比較を続けてよい。

- detached validation loop の運用摩擦低減
- parser boundary staged spike
- first checker cut の narrow compare family
- shared-space docs-first boundary comparison

次は勝手に finalization しない。

- final parser grammar
- production exporter API
- richer host interface の全面 actualization
- shared-space の final activation / authority / auth / consistency / fairness catalog
- higher-layer application contract

## いまの判断

- **現在地は「Phase 3 前半〜中盤」が主線で、Phase 2 終盤は maintenance tail、Phase 4 前半が side line」** と読むのが最も自然である。
- `progress.md` では、この phase 読みを rough snapshot として mirror する。
- ただし規範判断の正本は引き続き `specs/` と relevant `plan/` 個別文書に置く。
