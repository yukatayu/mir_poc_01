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

- **current tranche の closeout 完了。current checkpoint では self-driven portion は一旦尽き、reserve path に戻した**

#### 重さ

- 中程度からやや重い

#### autonomy gate

- **docs-first inventory の再確認までは可能だが、current promoted subline としては後段依存**

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

- **前半の authoritative baseline closeout 済み。catalog comparison の first cut として working subset row を切り、`auditable_authority_witness` の minimal witness core、authoritative delegated-provider practical cut、control-plane separated carrier threshold comparison まで整理済み。current package は checkpoint close で、next promoted line は Phase 5 inventory へ寄せる**

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

- **current theorem-line later package close。`specs/examples/126...` で 4-way split、`specs/examples/127...` で proof-obligation matrix と mixed handoff sketch、`specs/examples/128...` で mixed row default / boundary-specific split / actual emitter の reopen threshold、`specs/examples/129...` で first concrete consumer pressure を theorem line に置き、`specs/examples/130...` で theorem-side projection bundle、`specs/examples/131...` で typed symbolic `evidence_refs` family、`specs/examples/132...` で public checker migration defer threshold、`specs/examples/133...` で minimum contract row core を `obligation_kind + evidence_refs` に留める current first choiceまで固定し、次は concrete theorem consumer class と attachment family の comparison を later reopen 候補として選ぶ段階**

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

いま repo の主線は、次の 3 本である。

1. **checkpoint / maintenance tail**
   - Phase 0 / 1 / 2 の drift suppression を継続する
2. **Phase 4 前半の side line**
   - shared-space / membership の baseline closeout を保ちつつ、catalog comparison を進める
3. **Phase 5 入口の inventory line**
   - small decidable core と proof boundary の inventory は `specs/examples/126...` と `specs/examples/127...` までで current package close に入った
   - `specs/examples/128...` により、mixed row bundle を current default に維持し、boundary-specific handoff artifact や actual emitter は concrete pressure が出たときだけ reopen する threshold まで current package に含めてよい
   - `specs/examples/129...` により、first concrete consumer pressure の current first practical candidate は `theorem_prover_boundary` に置いてよい

shared-space line は、これらを壊さない範囲で進める **Phase 4 の side line** である。
Phase 3 は current checkpoint では **reserve path** として残し、later pressure が出たときだけ reopen 候補にする。

## immediate execution order

user 指示を反映した current immediate sequence は次である。

1. **detached validation loop の運用摩擦低減を先に進める**
   - export / compare / triage の current baseline は checkpoint close とみなし、`reference update / bless` だけを later candidate に残す
2. **consistency / fairness / causal metadata catalog の current package は checkpoint close として維持する**
   - fixed final catalog は作らず、room profile の stop line を増やす
   - current first cut は `specs/examples/122-shared-space-catalog-working-subset-comparison.md` の row set とし、`specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` までで minimal witness core を切る
   - `delegated_rng_service` を authoritative room 側でも provider-placement candidate としてどこまで practical に読めるかは `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md` までで current first choice を切った
   - control-plane separated causal carrier を authoritative room side line に reopen する threshold も `specs/examples/125-shared-space-control-plane-carrier-threshold.md` までで current first choice を切った
   - stronger control-plane split は later pressure が出たときだけ reopen 候補に残す
3. **Phase 5 current package は checkpoint close として維持する**
   - small decidable core / proof boundary / async-control boundary の 4-way split は current first choice に固定した
   - proof-obligation matrix は docs 正本、external handoff artifact は mixed row sketch に留めた
   - `specs/examples/128...` により、mixed row default を current first choice とし、boundary-specific handoff artifact と actual emitter は concrete pressure が出たときだけ reopen 候補にする
   - `specs/examples/129...` により、first concrete consumer pressure の current first practical candidate は theorem line に寄せた
   - `specs/examples/130...` により、theorem line の current first cut は docs-only projection bundle に留めた
   - `specs/examples/131...` により、theorem-side `evidence_refs` は typed symbolic ref family を current first choice に置いた
   - `specs/examples/132...` により、public checker migration は concrete theorem consumer pressure が出たときだけ reopen する threshold に留め、`specs/examples/133...` により minimum contract row core を `obligation_kind + evidence_refs` に留めたうえで、next practical reopen は concrete theorem consumer class と attachment family の comparison に寄せる
4. **detached validation loop は maintenance mode に戻し、authoritative room baseline は checkpoint close として維持する**
5. **Phase 3 は current では reopen せず、later pressure が出たときだけ reserve path として見直す**

現在は detached validation loop の current self-driven friction reduction と authoritative room baseline closeout が終わり、Phase 3 staged reconnect line も freeze threshold まで整理できたので、phase 読みとしては **Phase 3 current tranche は closeout 済みで、repo は current self-driven 主線を Phase 2 maintenance tail / Phase 4 side line / Phase 5 inventory line に移してよい状態にある** と読む。

## 現在の「止めるべき線」

次は self-driven に比較を続けてよい。

- detached validation loop の運用摩擦低減
- shared-space docs-first boundary comparison
- small decidable core inventory の docs-first comparison

次は勝手に finalization しない。

- Phase 3 reserve path の premature reopen
- final parser grammar
- production exporter API
- richer host interface の全面 actualization
- shared-space の final activation / authority / auth / consistency / fairness catalog
- higher-layer application contract

## いまの判断

- **現在地は「Phase 3 current tranche closeout 後の checkpoint」で、Phase 2 終盤が maintenance tail、Phase 4 前半が side line」** と読むのが最も自然である。
- `progress.md` では、この phase 読みを rough snapshot として mirror する。
- ただし規範判断の正本は引き続き `specs/` と relevant `plan/` 個別文書に置く。
