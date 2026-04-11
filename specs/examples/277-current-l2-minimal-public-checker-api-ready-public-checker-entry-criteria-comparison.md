# 277 — current L2 minimal-public-checker-api-ready public-checker-entry-criteria comparison

## 目的

`specs/examples/276-current-l2-public-checker-api-ready-minimal-public-checker-api-threshold.md`
で public checker API を
`checker_subject + public_checker_payload_schema_ref`
minimal docs-only read relation に留める判断を fixed した次段として、

- actual command surface / shared output contract / parser boundary を reopen してよい public checker entry criteria をどこまで narrow に切るか
- public checker API minimal shape と public-looking reopen condition を分けるべきか
- next promoted line をどの pressure から始めるのが current first choice か

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
minimal-public-checker-api-ready public-checker-entry-criteria comparison** であり、

- actual public checker command surface
- shared output contract
- final parser / checker boundary
- emitted public checker artifact

はまだ固定しない。

## scope

- current L2 checker-side docs-first bridge だけを扱う。
- checker-cluster matrix、payload family / row family / row detail / row body / supported kind summary、
  public checker payload schema、public checker API minimal read relationまでを前提にする。
- actual generic checker CLI / library entry / parser-front public checker には進まない。

## current 前提

current repo では次が成立している。

1. public checker API の minimal docs-only relation は
   `checker_subject + public_checker_payload_schema_ref`
   まで切れている。
2. checker-side actual source では、
   `scripts/current_l2_family_checker_support.py`
   が thin shared support helper として存在し、
   `scripts/current_l2_same_lineage_checker.py`、
   `scripts/current_l2_missing_option_checker.py`、
   `scripts/current_l2_capability_checker.py`
   は family facade を保っている。
3. `scripts/current_l2_detached_loop.py` には
   `smoke-same-lineage-checker`、
   `smoke-missing-option-checker`、
   `smoke-capability-checker`
   の repo-visible smoke command surface があり、
   current command-surface-adjacent pressure はすでに source-backed に見えている。
4. 一方で `specs/examples/72-current-l2-try-rollback-first-tranche-generic-public-recheck.md`
   により、dedicated try/rollback branch は generic/public comparison pressure 不足のため
   pause と読むのが current first choice である。
5. `specs/examples/63-current-l2-try-rollback-ast-helper-public-checker-entry-criteria.md`
   と theorem-side の `specs/examples/132`、`198`、`199`
   により、docs-only bridge と public-looking reopen condition を同じ task で混ぜない方が自然だという current ratchet が already ある。

したがって current 問いは、
**public checker API minimal shape が切れた後、actual command surface / shared output contract / parser boundary を reopen してよい条件をどう narrow に整理するか**
である。

## 比較観点

1. docs-only API shape と public-looking reopen condition を分けて読めるか
2. existing family facade / detached loop smoke surface を premature に public checker API 本体へ昇格させないか
3. source-backed pressure がある command surface と、まだ later に残す shared output contract / parser boundary を切り分けられるか
4. next promoted line を narrow に `public-checker-command-surface comparison` へ進められるか

## 比較対象

### 案 1. minimal public checker API が切れた時点で、そのまま public-looking comparison を始める

#### 読み

`checker_subject + public_checker_payload_schema_ref`
まで切れたので、それ自体を entry criteria とみなし、
actual command surface / shared output contract / parser boundary の comparison を
同列に reopen する。

#### 利点

- threshold の段数が少ない
- public-facing line を早く前へ進められる

#### 欠点

- docs-only API relation と public-looking reopen condition を混ぜやすい
- existing family facade smoke command をそのまま public checker API と誤読しやすい
- command surface、shared output contract、parser boundary の重さの違いを潰しやすい

### 案 2. public checker comparison 専用の entry criteria を別に置き、first reopen target を command surface に限る

#### additional entry criteria

少なくとも次を要求する。

1. docs-only minimal public checker API が fixed している
2. repo-visible な family facade command surface pressure が current source で成立している
3. shared output contract / parser boundary は still later と明示できる
4. generic/public pressure がまだ薄い family を無理に同時昇格させない

#### 利点

- docs-only API line と public-looking reopen condition を別段にできる
- command surface だけを first reopen target として narrow に扱える
- same-lineage / missing-option / capability の existing family facade pattern を source-backed に拾える
- try/rollback pause judgment とも衝突しにくい

#### 欠点

- threshold が 1 段増える
- shared output contract / parser boundary の deferred discipline を docs に明記する必要がある

### 案 3. shared output contract か parser boundary が固まるまで public checker comparison を deferred にする

#### 利点

- public-looking surface を強く凍結できる

#### 欠点

- current repo で見えている command-surface-adjacent pressure を拾えない
- family facade pattern と docs-only API relation の接点が again prose 依存になる
- parser boundary reserve path を unnecessary に current promoted line の blocker へしてしまう

## current judgment

current L2 で最も自然なのは、
**案 2. public checker comparison 専用の entry criteria を別に置き、first reopen target を command surface に限る**
である。

理由は次の通り。

1. `checker_subject + public_checker_payload_schema_ref` は public checker API minimal relation として十分だが、それだけで shared output contract / parser boundary まで reopen するのは still early である
2. current source で concrete に見えている public-looking pressure は、shared output contract や parser boundary ではなく、family facade smoke command が並んでいる command surface 側である
3. `specs/examples/72` の pause judgment が示すように、generic/public pressure が薄い branch を一緒に持ち上げるのは current ratchet に反する

## current guidance

current docs-only judgment では、
public checker API minimal relation の次段として
**public checker comparison 専用の entry criteria**
を 1 段切ってよい。

この line が示すのは、

- docs-only API relation が fixed しただけでは public-looking reopen を始めたことにはならない
- current source-backed pressure の first target は command surface organization である
- shared output contract / parser boundary は still later に残す

の 3 点である。

## next promoted line

next promoted line は、
**public-checker-entry-criteria-ready minimal-public-checker-entry-criteria threshold**
に置く。

## what is decided here

### decided

- public checker API minimal shape と public-looking reopen condition は別段に扱う
- current first reopen target は shared output contract / parser boundary ではなく command surface である
- generic/public pressure がまだ薄い family を current package に無理に混ぜない

### not decided

- actual public checker command naming
- shared output contract の minimal field set
- parser-front public checker boundary
- verifier handoff surface

## open questions

- minimum public-checker entry criteria を何項目までに留めるべきか
- command surface comparison の first candidate を family facade preservation と shared generic entry のどちらから始めるべきか
- shared output contract を command surface comparison の次段に置くべきか
