# 278 — current L2 public-checker-entry-criteria-ready minimal-public-checker-entry-criteria threshold

## 目的

`specs/examples/277-current-l2-minimal-public-checker-api-ready-public-checker-entry-criteria-comparison.md`
で public checker comparison 専用の entry criteria を
docs-only で 1 段切り、
first reopen target を command surface に限る判断を fixed した次段として、

- public checker entry criteria の minimum をどこまでに留めるか
- command surface comparison に進める条件と、still later に残す条件をどう切り分けるか
- shared output contract / parser boundary を entry criteria 自体へ混ぜるべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
public-checker-entry-criteria-ready minimal-public-checker-entry-criteria threshold** であり、

- actual public checker command surface
- shared output contract
- parser-front public checker boundary
- emitted verifier handoff contract

はまだ固定しない。

## 比較観点

1. public checker comparison を始める minimum として過不足がないか
2. command-surface pressure だけを拾い、shared output contract / parser boundary を premature に含めないか
3. current family facade / shared support helper pattern と `specs/examples/72` の pause judgment に整合するか
4. next promoted line を narrow に `public-checker-command-surface comparison` へ進められるか

## 比較対象

### 案 1. minimal public checker API が fixed していれば十分

#### minimum

1. `public_checker_api_ready_sketch = { checker_subject, public_checker_payload_schema_ref }`

#### 利点

- 最も軽い
- threshold が分かりやすい

#### 欠点

- docs-only relation と public-looking reopen condition の区別が弱い
- source-backed command-surface pressure を見ていない

### 案 2. docs-only minimal API + source-backed family-facade command-surface pressure を minimum にする

#### minimum

1. docs-only minimal public checker API が fixed している
2. repo-visible な family facade command surface が stable reason-family line で source-backed に存在する
3. next reopen target は command surface comparison に限ると明示できる
4. shared output contract / parser boundary は still later と明示できる

#### 利点

- current source-backed pressure を minimum に反映できる
- actual command surface の comparison を始める理由が明確になる
- shared output contract / parser boundary を threshold 自体へ混ぜない

#### 欠点

- 案 1 より 1 段重い
- deferred line の明示が必要である

### 案 3. shared output contract か parser boundary まで minimum に含める

#### 利点

- public-looking line をより強く揃えてから reopen できる

#### 欠点

- current promoted line としては still heavy である
- Phase 3 reserve path と later verifier/output contract line を unnecessary に blocker 化しやすい
- command-surface comparison の narrow reopen を遅らせる

## current judgment

current L2 で最も自然なのは、
**案 2. docs-only minimal API + source-backed family-facade command-surface pressure を minimum にする**
である。

理由は次の通り。

1. public checker comparison を始める minimum には、docs-only API relation だけでなく、repo-visible command-surface pressure が必要である
2. その pressure は current source では
   `smoke-same-lineage-checker`、
   `smoke-missing-option-checker`、
   `smoke-capability-checker`
   と thin facade / shared support helper の組に already 現れている
3. shared output contract / parser boundary まで minimum に含めると current promoted line が重くなりすぎる

## current minimum public-checker entry criteria

current first choice として、
public checker command-surface comparison へ進めてよい minimum は次である。

1. **minimal API fixed**
   - `public_checker_api_ready_sketch`
   - shape は `checker_subject + public_checker_payload_schema_ref`
2. **command-surface pressure present**
   - `scripts/current_l2_family_checker_support.py` が thin shared support helper として存在する
   - `scripts/current_l2_same_lineage_checker.py`
   - `scripts/current_l2_missing_option_checker.py`
   - `scripts/current_l2_capability_checker.py`
     が family facade を保っている
   - `scripts/current_l2_detached_loop.py` に
     `smoke-same-lineage-checker`、
     `smoke-missing-option-checker`、
     `smoke-capability-checker`
     が repo-visible smoke command として存在する
3. **comparison target narrowed**
   - 次段で reopen するのは command surface comparison だけである
4. **heavier boundary deferred**
   - shared output contract / parser boundary / verifier handoff は still later とする
   - `specs/examples/72` の pause judgment を崩さず、generic/public pressure が薄い family はまだ無理に持ち上げない

## practical reading

この minimum が意味するのは、

- public checker API minimal relation の docs-only line は閉じた
- current repo には family facade pattern という command-surface-adjacent evidence がある
- したがって next では shared output contract や parser boundary ではなく、
  public checker command surface の比較を narrow に始めてよい

ということである。

### まだ minimum に入れないもの

- shared output contract の row / payload core
- parser-front public checker entry
- generic emitted verifier handoff surface
- try/rollback dedicated family の generic/public reopen

## next promoted line

next promoted line は、
**minimal-public-checker-entry-criteria-ready public-checker-command-surface comparison**
に置く。

## open questions

- public checker command surface comparison の first candidates を family facade preservation / shared generic entry / query-token refinement のどれに置くべきか
- shared output contract を command surface comparison の次段に置くべきか
- `checker_subject` を later で dedicated query token / `*_ref` family に寄せるべきか
