# 279 — current L2 minimal-public-checker-entry-criteria-ready public-checker-command-surface comparison

## 目的

`specs/examples/278-current-l2-public-checker-entry-criteria-ready-minimal-public-checker-entry-criteria-threshold.md`
で public checker comparison 専用の entry criteria を
docs-only minimal API fixed + source-backed family-facade command-surface pressure
に留める判断を fixed した次段として、

- public checker command surface comparison をどの cut から始めるのが current first choice か
- existing family facade pattern を docs-only public checker command surface として切ってよいか
- shared generic entry / shared output contract / parser boundary をどこまで still later に残すべきか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-public-checker-entry-criteria-ready public-checker-command-surface comparison** であり、

- final generic public checker command
- shared output contract
- parser-front public checker boundary
- emitted verifier handoff surface

はまだ固定しない。

## scope

- current L2 checker-side docs-first bridge だけを扱う。
- public checker payload schema、public checker API minimal read relation、public checker entry criteria minimum までを前提にする。
- actual generic CLI / library API / parser-front public checker には進まない。

## current 前提

current repo では次が成立している。

1. public checker comparison を始めてよい minimum は、
   docs-only minimal API fixed + source-backed family-facade command-surface pressure
   に留めるのが current first choice である。
2. source-backed pressure の中身は、
   `scripts/current_l2_family_checker_support.py`
   を thin shared support helper に保ったうえで、
   `scripts/current_l2_same_lineage_checker.py`、
   `scripts/current_l2_missing_option_checker.py`、
   `scripts/current_l2_capability_checker.py`
   が family facade を保っており、
   `scripts/current_l2_detached_loop.py`
   に
   `smoke-same-lineage-checker`、
   `smoke-missing-option-checker`、
   `smoke-capability-checker`
   が repo-visible smoke command として存在する、ということである。
3. `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
   と
   `specs/examples/50-current-l2-generic-family-checker-entry-comparison.md`
   により、current helper-local line では shared support helper 導入後も family facade script を維持し、
   generic checker-side shared entry はまだ切らないのが current judgment である。
4. `specs/examples/72-current-l2-try-rollback-first-tranche-generic-public-recheck.md`
   により、generic/public pressure が薄い branch を無理に current public checker comparison へ混ぜないのが current ratchet である。
5. public checker API 自体は
   `checker_subject + public_checker_payload_schema_ref`
   docs-only read relation に留まっており、
   actual command surface ではない。

したがって current 問いは、
**public checker entry criteria minimum を満たした次段として、public checker command surface comparison をどの public-looking cut から始めるのが自然か**
である。

## 比較観点

1. source-backed family-facade pressure をそのまま first comparison target にできるか
2. `specs/examples/49` / `50` の family facade 維持 judgment と衝突しないか
3. shared generic entry、shared output contract、parser boundary を premature に混ぜないか
4. next 段の minimal public checker command surface threshold を narrow に保てるか

## 比較対象

### 案 1. existing family facade pattern を first docs-only public checker command surface として切る

#### 読み

current public checker command surface comparison は、
generic shared entry ではなく、
existing family facade pattern を
docs-only public-looking command surface として読むところから始める。

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

ここで detached loop の `smoke-*` command 名は、
repo-visible evidence として扱うが、
minimal shape 自体にはまだ混ぜない。

#### 利点

- entry criteria で拾った source-backed family-facade pressure をそのまま first comparison target にできる
- `specs/examples/49` / `50` の current judgment と揃う
- public checker API minimal relation と command surface comparison を別段に保てる
- shared generic entry / shared output contract / parser boundary を still later に残せる

#### 欠点

- execution entry は plural な family facade のままである
- detached loop smoke wrapper と facade command のどちらを later public surface に強く出すかはまだ未決になる

### 案 2. generic shared public checker command surface を先に切る

#### 読み

current source が family facade でも、
public checker command surface comparison では
generic shared entry を先に置く。

#### 利点

- single command surface を早く比較できる
- future family 追加時の naming pattern は統一しやすい

#### 欠点

- `specs/examples/50` の current judgment と衝突しやすい
- source-backed pressure は family facade であり、generic shared entry ではない
- try/rollback など generic/public pressure が薄い branch を premature に巻き込みやすい

### 案 3. shared output contract か parser boundary まで public checker command surface を deferred にする

#### 利点

- public-looking surface を強く凍結できる

#### 欠点

- entry criteria で明示した source-backed command-surface pressure を拾えない
- family facade pattern と public checker comparison の接点が again prose 依存になる
- shared output contract / parser boundary を unnecessary に直前 blocker 化しやすい

## current judgment

current L2 で最も自然なのは、
**案 1. existing family facade pattern を first docs-only public checker command surface として切る**
である。

理由は次の通り。

1. current public checker entry criteria が示す source-backed pressure 自体が family facade pattern であり、first comparison target もそこから始めるのが自然である
2. `specs/examples/49` / `50` により、shared support helper と generic shared entry を切り分け、family facade 維持で止める ratchet は already ある
3. generic shared entry を先に切ると、command surface comparison の段で current family taxonomy や pressure-thin branch まで premature に reopen しやすい
4. shared output contract / parser boundary を待つ必要はないが、それらを今の command surface comparison に混ぜる必要もない

## current guidance

current docs-only judgment では、
public checker command surface comparison は
**family facade pattern を public-looking command surface の first candidate として切る**
ところから始めてよい。

この line が示すのは、

- current public checker command surface の first candidate は `family_facade_checker_commands` である
- その下にある read relation は `public_checker_api_ready_sketch` である
- detached loop `smoke-*` wrapper は repo-visible evidence だが、minimal command-surface shape 自体にはまだ混ぜない
- shared generic entry / shared output contract / parser boundary は still later に残す

の 4 点である。

## next promoted line

next promoted line は、
**public-checker-command-surface-ready minimal-public-checker-command-surface threshold**
に置く。

## what is decided here

### decided

- public checker command surface comparison の first candidate は existing family facade pattern である
- command surface comparison は public checker API minimal relation の次段として docs-only に 1 段切ってよい
- shared generic entry / shared output contract / parser boundary は still later に残す

### not decided

- minimal public checker command surface の final field set
- detached loop `smoke-*` wrapper を later public surface にどこまで含めるか
- generic shared public checker entry を later で切る timing
- shared output contract / parser boundary / verifier handoff の final surface

## open questions

- minimal public checker command surface を何 field までに留めるべきか
- detached loop `smoke-*` wrapper を command surface minimum に含めるべきか
- shared output contract を command surface comparison の次段に置くべきか
