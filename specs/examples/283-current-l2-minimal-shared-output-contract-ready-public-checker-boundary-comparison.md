# 283 — current L2 minimal-shared-output-contract-ready public-checker-boundary comparison

## 目的

`specs/examples/282-current-l2-shared-output-contract-ready-minimal-shared-output-contract-threshold.md`
で shared output contract の minimal shape を
`output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref`
に留める判断を fixed した次段として、

- public checker boundary comparison をどの cut から始めるのが current first choice か
- final parser grammar を固定せずに docs-only public checker boundary を 1 段切ってよいか
- generic shared entry / parser grammar / verifier handoff をどこまで still later に残すべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-shared-output-contract-ready public-checker-boundary comparison** であり、

- final parser grammar
- actual public parser / checker API
- emitted verifier handoff surface

はまだ固定しない。

## scope

- current L2 checker-side docs-first bridge だけを扱う。
- public checker command surface minimum と minimal shared output contract までを前提にする。
- actual parser-front public checker 実装、generic shared public checker entry、verifier handoff actualization には進まない。

## current 前提

current repo では次が成立している。

1. public checker command surface の minimum は
   `command_surface_kind + family_facade_command_refs + public_checker_api_ref`
   である。
2. shared output contract の minimum は
   `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref`
   である。
3. `specs/examples/112-current-l2-phase3-resume-side-selection.md`
   と
   `specs/examples/113-current-l2-first-checker-reconnect-family-selection.md`
   により、Phase 3 reserve path では syntax pressure を上げず、
   final parser grammar と public checker API を still later に残す current ratchet が already ある。
4. `specs/examples/275` から `282` までの current checker-side line は、
   payload schema、public checker API minimal read relation、entry criteria、command surface、shared output contract
   を docs-first に段階分割しており、
   final parser / checker boundary を same task へ混ぜない方向で進んでいる。

したがって current 問いは、
**shared output contract の次段として public checker boundary を 1 段切るなら、どの public-looking cut が最小か**
である。

## 比較観点

1. command surface と shared output contract の接点を docs-only に見える化できるか
2. final parser grammar を固定せずに public checker boundary を narrow に切れるか
3. generic shared entry / parser grammar / verifier handoff を premature に混ぜないか
4. next 段の minimal public checker boundary threshold を narrow に保てるか

## 比較対象

### 案 1. final parser grammar / public parser API が固まるまで public checker boundary を deferred にする

#### 利点

- parser-front public checker の premature 固定を避けやすい
- Phase 3 reserve path と衝突しにくい

#### 欠点

- command surface と shared output contract の接点が prose 依存に残る
- current checker-side line の next narrow step が unnecessary に later blocker 化しやすい

### 案 2. command surface と shared output contract をつなぐ docs-only public checker boundary を first candidate にする

#### 読み

current public checker boundary comparison は、
final parser grammar や query token を決めることではなく、
command surface と shared output contract を
docs-only parser-front boundary として読むところから始める。

```text
public_checker_boundary_ready_sketch = {
  boundary_kind = docs_only_parser_front_checker_boundary,
  public_checker_command_surface_ref = public_checker_command_surface_ready_sketch,
  shared_output_contract_ref = shared_output_contract_ready_sketch
}
```

#### 利点

- command side と output side の接点を docs-only に 1 段切れる
- final parser grammar を still later に残せる
- Phase 3 reserve path の syntax-pressure suppression judgment と衝突しにくい
- verifier handoff を次段へ narrow に送れる

#### 欠点

- parser-front input surface の concrete shape は still open に残る
- generic shared public checker entry と boundary の関係は later で整理が要る

### 案 3. parser input / query token / verifier handoff まで public checker boundary に同時に入れる

#### 利点

- public-looking boundary を一気に concrete にできる

#### 欠点

- final parser grammar、input token surface、verifier handoff を同じ reopen で混ぜやすい
- current docs-first ratchet に反する
- Phase 3 reserve path の later blocker を unnecessary に current package へ持ち込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. command surface と shared output contract をつなぐ docs-only public checker boundary を first candidate にする**
である。

理由は次の通り。

1. current checker-side line では、command surface と shared output contract の双方が already docs-only に切れており、その接点を 1 段作るのが next narrow step として自然である
2. ただし final parser grammar や query token surface まで同時に扱うのは still early である
3. Phase 3 reserve path は parser boundary staging を later blocker として保持しており、その freeze judgment を壊さずに進めるには docs-only boundary relation に留めるのがよい

## current guidance

current docs-only judgment では、
public checker boundary comparison は
**`boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref`**
から始めてよい。

この line が示すのは、

- current public checker boundary の first candidate は docs-only parser-front boundary である
- command surface と shared output contract の接点を docs-only に 1 段切ってよい
- final parser grammar / generic shared entry / verifier handoff は still later に残す

の 3 点である。

## next promoted line

next promoted line は、
**public-checker-boundary-ready minimal-public-checker-boundary threshold**
に置く。

## what is decided here

### decided

- public checker boundary comparison の first candidate は docs-only parser-front boundary である
- shared output contract の次段として public checker boundary を docs-only に 1 段切ってよい
- final parser grammar / generic shared entry / verifier handoff は still later に残す

### not decided

- final parser grammar / token surface
- query token や `checker_subject` の final public naming
- generic shared public checker entry の actual導入タイミング
- emitted verifier handoff surface の final shape

## open questions

- minimal public checker boundary を何 field までに留めるべきか
- query token / subject ref family を boundary minimum に入れるべきか
- verifier handoff surface を public checker boundary の次段でどこまで narrow に切るべきか
