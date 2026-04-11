# 291 — current L2 parser-to-checker-reconnect-freeze-ready phase1-semantics-closeout comparison

## 目的

`specs/examples/289-current-l2-minimal-parser-subset-freeze-ready-parser-to-checker-reconnect-freeze-comparison.md`
と
`specs/examples/290-current-l2-parser-to-checker-reconnect-freeze-ready-minimal-parser-to-checker-reconnect-freeze-threshold.md`
で Phase 3 reopen line の self-driven freeze を fixed した次段として、

- current L2 semantics stabilization をどの closeout cut で phase-complete snapshot にできるか
- invariants / proof-obligation wording / companion notation boundary をどこまで同じ package で閉じるべきか
- final parser grammar、final type system、actual external schema をどこまで still later に残すべきか

を比較する。

ここで固定するのは
**current L2 parser-to-checker-reconnect-freeze-ready phase1-semantics-closeout comparison**
であり、

- final parser grammar
- final type lattice
- actual theorem / model-check consumer schema
- actual emitted verifier artifact

はまだ固定しない。

## scope

- current L2 semantics closeout だけを扱う。
- fallback / `lease` / guarded option chain / `try` / `atomic_cut` の settled reading は維持する。
- `specs/09-invariants-and-constraints.md` と
  `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
  の橋を明示する。
- companion notation は explicit edge-row family を維持し、
  A2 polished first choice と A1 shorthand の関係だけを closeout する。
- parser grammar finalization、type-system finalization、actual tool binding には進まない。

## current 前提

current repo では次が成立している。

1. fallback は guarded option chain であり、same-lineage chain は left-to-right monotone degradation を取る。
2. rollback / `atomic_cut` は degradation order 自体を巻き戻さない。
3. explicit edge-row family は current companion notation の正本であり、
   A2 hanging continuation は polished first choice、A1 inline row は shorthand である。
4. proof-obligation matrix では `canonical normalization law / no re-promotion` と
   `rollback-cut non-interference / hidden rollback absence` を external boundary row として持っている。

したがって current 問いは、
**semantic redesign に入らずに、Phase 1 closeout として何を narrow に固定すればよいか**
である。

## 比較観点

1. semantics の settled reading と proof-side obligation 名を混同しないか
2. explicit edge-row family を保ったまま lexical finalization を still later に残せるか
3. parser grammar / type system / external schema を premature に phase closeout へ混ぜないか
4. next promoted line を Phase 2 parser-free PoC closeout へ移せるか

## 比較対象

### 案 1. mirror-only audit で済ませ、closeout bundle 自体は切らない

#### 利点

- 最小の更新で済む
- parser grammar / type system を accidental に固定しにくい

#### 欠点

- invariants と proof-obligation wording の橋が snapshot 上で暗黙のまま残る
- notation drift correction が decision-register level で見えにくい
- Phase 1 closeout を source-backed package として閉じたことが読み取りにくい

### 案 2. `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs + retained_later_refs` を持つ narrow closeout bundle を切る

#### shape

```text
phase1_semantics_closeout_ready_sketch = {
  closeout_kind = current_l2_semantics_closeout,
  core_semantics_refs = [
    fallback_lease_chain_semantics,
    try_atomic_cut_semantics
  ],
  invariant_bridge_refs = [
    invariants_8_9_to_canonical_normalization_law,
    invariant_11_to_rollback_cut_non_interference
  ],
  notation_status_refs = [
    explicit_edge_row_family,
    a2_polished_first_choice,
    a1_companion_shorthand
  ],
  retained_later_refs = [
    final_parser_grammar,
    final_type_system,
    actual_external_schema
  ]
}
```

#### 利点

- semantic core と proof-side naming bridge を同じ closeout cut に置ける
- decision-register の notation drift を narrow に修正できる
- explicit edge-row family を保ったまま lexical finalization を still later に残せる
- next promoted line を Phase 2 closeout へ移しやすい

#### 欠点

- mirror-only audit よりは field が増える
- actual schema や tool binding はまだ持たないため、docs-first discipline が要る

### 案 3. parser grammar / type system / external handoff schema の初回 finalization まで一気に含める

#### 利点

- 一見すると closeout 感は強い

#### 欠点

- Phase 1 closeout の範囲を超えて parser / type / verifier contract を premature に固定しやすい
- current repo の staged plan と autonomy gate を壊す
- narrow closeout ではなく broad finalization になる

## current judgment

current L2 で最も自然なのは、
**案 2. `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs + retained_later_refs` を持つ narrow closeout bundle を切る**
である。

理由は次の通り。

1. semantic redesign は要らず、必要なのは invariant bridge と notation boundary の closeout である
2. proof-obligation row 名を invariant の residual discharge 名として接続できる
3. explicit edge-row family の settled reading を保ちながら lexical finalization を still later に残せる
4. current mainline を Phase 2 parser-free PoC closeout へ移せる

## current first choice shape

```text
phase1_semantics_closeout_ready_sketch = {
  closeout_kind = current_l2_semantics_closeout,
  core_semantics_refs = [
    fallback_lease_chain_semantics,
    try_atomic_cut_semantics
  ],
  invariant_bridge_refs = [
    invariants_8_9_to_canonical_normalization_law,
    invariant_11_to_rollback_cut_non_interference
  ],
  notation_status_refs = [
    explicit_edge_row_family,
    a2_polished_first_choice,
    a1_companion_shorthand
  ],
  retained_later_refs = [
    final_parser_grammar,
    final_type_system,
    actual_external_schema
  ]
}
```

## この shape でまだ入れないもの

- final parser grammar
- final reserved keyword / punctuation
- full type lattice / compatibility completion
- actual theorem / model-check schema
- actual emitted verifier artifact

これらは still later である。

## practical reading

current Phase 1 closeout が示すのは、次の 3 点だけである。

1. fallback / `lease` / `try` / `atomic_cut` の core semantics は current L2 で安定している
2. `canonical normalization law / no re-promotion` と
   `rollback-cut non-interference / hidden rollback absence` は
   invariant の residual proof-side discharge 名として読む
3. companion notation は explicit edge-row family を維持し、
   A2 を polished first choice、A1 を shorthand としつつ lexical finalization は残す

## next promoted line

next promoted line は、
**phase1-semantics-closeout-ready phase2-parser-free-poc-closeout comparison**
に置く。

## open questions

- final parser grammar で A2 / A1 lexical choice をどこまで固定するか
- full type system / compatibility lattice をどの phase で reopen するか
- actual theorem / model-check contract をどの threshold で concrete tool に結ぶか
