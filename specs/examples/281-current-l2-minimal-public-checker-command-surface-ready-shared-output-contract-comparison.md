# 281 — current L2 minimal-public-checker-command-surface-ready shared-output-contract comparison

## 目的

`specs/examples/280-current-l2-public-checker-command-surface-ready-minimal-public-checker-command-surface-threshold.md`
で public checker command surface の minimal shape を
`command_surface_kind + family_facade_command_refs + public_checker_api_ref`
minimal family-facade bundle に留める判断を fixed した次段として、

- shared output contract comparison をどの cut から始めるのが current first choice か
- existing family checker shared helper が出している summary line を docs-only shared output contract として切ってよいか
- detached loop wrapper / parser boundary / verifier handoff をどこまで still later に残すべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-public-checker-command-surface-ready shared-output-contract comparison** であり、

- parser-front public checker boundary
- generic shared public checker entry
- emitted verifier handoff surface

はまだ固定しない。

## scope

- current L2 checker-side docs-first bridge だけを扱う。
- public checker payload schema、public checker API minimal read relation、public checker command surface minimum までを前提にする。
- actual generic CLI / library API / parser-front public checker には進まない。

## current 前提

current repo では次が成立している。

1. public checker command surface の minimum は、
   `command_surface_kind + family_facade_command_refs + public_checker_api_ref`
   minimal family-facade bundle に留めるのが current first choice である。
2. `scripts/current_l2_family_checker_support.py`
   は same-lineage / missing-option / capability の 3 family で shared に使われており、
   current shared output line として
   - `fixture: ...`
   - `artifact: ...`
   - `cluster: ...`
   - `status: ...`
   - `fixture rows:`
   - `actual rows:`
   を出す。
3. その shared helper の `status_for_rows()` は、
   shared status として
   `matched`、
   `mismatch`、
   `out_of_scope`
   を持ち、family-local missing case は helper 呼び出し側の
   `fixture_*_rows_missing`
   string で渡す。
4. `scripts/current_l2_detached_loop.py`
   の `smoke-same-lineage-checker`、
   `smoke-missing-option-checker`、
   `smoke-capability-checker`
   は wrapper 側で
   `static gate artifact: ...`
   を出してから family checker へ委譲する。
5. `specs/examples/72-current-l2-try-rollback-first-tranche-generic-public-recheck.md`
   により、try/rollback dedicated branch は generic/public pressure 不足のため pause と読むのが current first choice であり、
   current shared output contract comparison へはまだ混ぜない。

したがって current 問いは、
**public checker command surface の次段として、どの shared output line を current docs-only contract の first candidate とみなすのが自然か**
である。

## 比較観点

1. current actual source にある shared family checker output を source-backed に拾えるか
2. command surface line と emitted output line を docs-only に 1 段分けて読めるか
3. detached loop wrapper / parser boundary / verifier handoff を premature に混ぜないか
4. next 段の minimal shared output contract threshold を narrow に保てるか

## 比較対象

### 案 1. shared output contract は still prose に残す

#### 利点

- 最も軽い
- parser boundary や public-looking emitted contract の premature 固定を避けやすい

#### 欠点

- current source-backed shared output line が docs 上で prose 依存に残る
- command surface と emitted summary の接点が見えにくい
- next threshold comparison の field 候補が again 曖昧になりやすい

### 案 2. shared family checker summary line を first docs-only shared output contract として切る

#### 読み

current shared output contract comparison は、
detached loop wrapper 全体でも parser-front boundary でもなく、
family checker shared helper が出している summary line を
docs-only shared output contract の first candidate として切る。

```text
shared_output_contract_ready_sketch = {
  output_contract_kind = family_checker_row_compare_summary,
  checker_cluster_name,
  checker_status,
  public_checker_payload_schema_ref = public_checker_payload_schema_ready_sketch
}
```

ここで `fixture:` / `artifact:` path line と
`fixture rows:` / `actual rows:` の textual rendering は
source-backed evidence として読むが、
minimal shared output contract 自体にはまだ混ぜない。

#### 利点

- current shared helper が already 出している `cluster` / `status` summary を docs-only contract に anchored できる
- command surface line と emitted output line を別段に読める
- public checker payload schema との接点を narrow に保てる
- detached loop wrapper / parser boundary / verifier handoff を still later に残せる

#### 欠点

- actual console output の path line や row snippet 自体は contract 外に残る
- `checker_cluster_name` を later で ref family に寄せるかは未決のまま残る

### 案 3. detached loop wrapper line や full textual row payload まで shared output contract に入れる

#### 利点

- current console output には最も近い
- wrapper から emitted text まで一度に比較できる

#### 欠点

- `static gate artifact:` line と family checker summary を premature に同格化しやすい
- textual row rendering と payload schema / parser boundary / verifier handoff が一気に近づきすぎる
- current docs-first line としては still heavy である

## current judgment

current L2 で最も自然なのは、
**案 2. shared family checker summary line を first docs-only shared output contract として切る**
である。

理由は次の通り。

1. current repo に actual source-backed で存在する shared output line は detached loop wrapper 全体ではなく、`scripts/current_l2_family_checker_support.py` の `cluster` / `status` summary である
2. `public_checker_payload_schema_ready_sketch` は already 切れているため、emitted output line と payload schema の接点も docs-only に 1 段作れる
3. `static gate artifact:` path line、parser-front boundary、verifier handoff を同じ reopen で混ぜると current promoted line が重くなりすぎる

## current guidance

current docs-only judgment では、
shared output contract comparison は
**family checker shared helper の summary line を first candidate として切る**
ところから始めてよい。

この line が示すのは、

- current shared output contract の first candidate は `family_checker_row_compare_summary` である
- command surface の next 段として emitted summary line を docs-only に 1 段分けてよい
- detached loop wrapper の path line、parser boundary、verifier handoff は still later に残す

の 3 点である。

## next promoted line

next promoted line は、
**shared-output-contract-ready minimal-shared-output-contract threshold**
に置く。

## what is decided here

### decided

- shared output contract comparison の first candidate は family checker shared helper の summary line である
- current shared output contract comparison は command surface の次段として docs-only に 1 段切ってよい
- detached loop wrapper / parser boundary / verifier handoff は still later に残す

### not decided

- minimal shared output contract の final field set
- `fixture:` / `artifact:` path line を later public output contract に入れるか
- row snippet textual rendering を later public payload に含めるか
- parser-front public checker boundary と verifier handoff surface の final shape

## open questions

- minimal shared output contract を何 field までに留めるべきか
- `checker_cluster_name` を later で `*_ref` family へ寄せるべきか
- parser boundary の前に verifier handoff を切る必要があるか
