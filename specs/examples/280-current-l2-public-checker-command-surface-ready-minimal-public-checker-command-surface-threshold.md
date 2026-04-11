# 280 — current L2 public-checker-command-surface-ready minimal-public-checker-command-surface threshold

## 目的

`specs/examples/279-current-l2-minimal-public-checker-entry-criteria-ready-public-checker-command-surface-comparison.md`
で public checker command surface comparison の first candidate を
existing family facade pattern に置く判断を fixed した次段として、

- public checker command surface の minimum をどこまでに留めるか
- family facade pattern と public checker API read relation の接点をどこまで explicit に持たせるか
- detached loop smoke wrapper / generic shared entry / shared output contract をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
public-checker-command-surface-ready minimal-public-checker-command-surface threshold** であり、

- generic shared public checker command
- shared output contract
- parser-front public checker boundary
- verifier handoff surface

はまだ固定しない。

## 比較観点

1. public checker API minimal read relation と distinguish できる command-surface minimum になっているか
2. source-backed family facade pattern を minimum に反映しつつ still minimal に保てるか
3. detached loop smoke wrapper / generic shared entry / shared output contract を premature に minimum へ混ぜないか
4. next promoted line を narrow に `shared-output-contract comparison` へ進められるか

## 比較対象

### 案 1. `public_checker_api_ref` だけを持つ

#### shape

```text
public_checker_command_surface_ready_sketch = {
  public_checker_api_ref = public_checker_api_ready_sketch
}
```

#### 利点

- 最も軽い
- public checker API read relation をそのまま使える

#### 欠点

- command surface と API read relation の区別が still 弱い
- family facade pattern が minimum に現れない

### 案 2. `command_surface_kind + family_facade_command_refs + public_checker_api_ref` を持つ

#### shape

```text
public_checker_command_surface_ready_sketch = {
  command_surface_kind = family_facade_checker_commands,
  family_facade_command_refs = [
    same_lineage_checker_command,
    missing_option_checker_command,
    capability_checker_command
  ],
  public_checker_api_ref = public_checker_api_ready_sketch
}
```

#### 利点

- command surface が family facade pattern に anchored していることを minimum に反映できる
- public checker API read relation と command surface line を分けて読める
- generic shared entry / shared output contract / parser boundary を still later に残せる

#### 欠点

- `public_checker_api_ref` only より 1 段重い
- family facade command ref naming discipline が要る

### 案 3. detached loop smoke wrapper / generic selector / shared output contract 近傍 field まで minimum に入れる

#### 利点

- repo-visible surface を広く一度に表せる

#### 欠点

- current threshold としては still heavy である
- detached loop smoke wrapper と public checker command surface を premature に同格化しやすい
- shared output contract / generic shared entry / parser boundary を一気に持ち込みやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `command_surface_kind + family_facade_command_refs + public_checker_api_ref` を持つ**
である。

理由は次の通り。

1. command surface として 1 段切る以上、family facade pattern が minimum に見える方が自然である
2. ただし detached loop smoke wrapper や generic shared entry まで minimum に含めるのは still early である
3. `public_checker_api_ref` を下位 relation として参照すれば、payload schema / checker subject 側の docs-only read relation を重複なく保てる

## current first choice shape

```text
public_checker_command_surface_ready_sketch = {
  command_surface_kind = family_facade_checker_commands,
  family_facade_command_refs = [
    same_lineage_checker_command,
    missing_option_checker_command,
    capability_checker_command
  ],
  public_checker_api_ref = public_checker_api_ready_sketch
}
```

### この shape でまだ入れないもの

- detached loop `smoke-same-lineage-checker`
- detached loop `smoke-missing-option-checker`
- detached loop `smoke-capability-checker`
- generic shared public checker entry
- shared output contract
- parser / checker public boundary
- verifier handoff surface

これらは still later である。

## practical reading

current minimal command surface が示すのは、

- public checker command surface の first cut は family facade bundle である
- individual facade command は same-lineage / missing-option / capability の 3 family に限る
- その下位の docs-only read relation は `public_checker_api_ready_sketch` である

の 3 点だけである。

detached loop smoke wrapper は current source-backed evidence として扱うが、
minimal command surface の first-class field にはまだ上げない。

## next promoted line

next promoted line は、
**minimal-public-checker-command-surface-ready shared-output-contract comparison**
に置く。

## open questions

- detached loop `smoke-*` wrapper を later public surface に含めるべきか
- generic shared public checker entry を later でどの trigger から切るべきか
- shared output contract の minimum を command surface の次段でどこまで narrow に切るべきか
