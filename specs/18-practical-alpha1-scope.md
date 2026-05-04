# 18 — Practical Alpha-1 Scope

## role

この文書は、Mirrorea Spaces の
**practical alpha-1 readiness / 100% 条件 / 非目標 / validation boundary**
を規範として置く。

- ここで定義するのは final public product ではない
- ここで定義するのは current-scope evidence closeout と別の readiness metric である
- `specs/13..17` と `samples/alpha/` の current-scope evidence は前提として保持するが、それだけで practical alpha-1 完了とは呼ばない

## decision level

- `L1`
  - future `100%` は practical alpha-1 readiness を指す
  - current-scope evidence closeout と practical readiness を混同しない
  - alpha-1 は library-first toolchain である
  - parser/front-door、checker、runtime、package/hot-plug、transport、devtools、local save/load、product prototype の 8 系統を揃える
- `L2`
  - alpha source / package surface
  - alpha IR / checker report shape
  - alpha-local CLI / viewer / JSON schema

## one-sentence definition

Mirrorea Spaces practical alpha-1 とは、
Mir / Mirrorea の current theory に基づき、
開発者が小さな shared virtual-space product prototype を
**source front-door から検査・実行・観測・保存再開できる**
library-first toolchain である。

## relationship to current alpha-0 evidence

`specs/13..17`、`plan/39..43`、`samples/alpha/`、および
`P-A0-01..28` の closeout は current-scope evidence closeout として保持する。

この evidence は useful であり、practical alpha-1 の前提として数える。
ただし、それだけで次を意味しない。

- source front-door execution
- reusable checker/runtime API completion
- practical package/toolchain completion
- public alpha / `U1`
- final public product

## required toolchain shape

practical alpha-1 では、少なくとも次の path が必要である。

```text
alpha source / package manifest
  -> parser / loader
  -> alpha IR
  -> checker
  -> runtime plan
  -> local runtime or Docker runtime
  -> devtools export
  -> product-like usage
```

runtime は sample-ID keyed bridge だけに依存してはならない。

## completion condition

practical alpha-1 を `100%` と呼ぶには、少なくとも次を全て満たす。

### 1. source front-door

- `samples/practical-alpha1/` または同等の practical root から source/manifest を読む
- alpha grammar は final public grammar でなくてよい
- initial admissible cut は limited `package.mir.json` loader でもよい
- ただし、その cut は practical root から実際に読まれ、positive/negative parse test を持ち、non-final front-door だと明記されること
- grammar / loader は documented され、実 sample で使われる
- sample-ID keyed helper ではなく source/manifest input が front-door になる

### 2. typed checking

checker は少なくとも次を検査する。

- raw dangling reference reject
- guarded fallback access path
- inherited chain and snapshot-selected distinction
- mutable/read-write invariance
- contract pre/post variance
- effect row containment
- failure row containment
- capability monotonicity
- observation/redaction/retention constraints
- invalid distributed cut rejection
- package manifest admission

positive proofは negative reason code 不在ではなく explicit accepted evidence で扱う。

## first checker floor boundary

`P-A1-02` の current actualization は、
typed checking 全体の completion ではなく、
**first practical checker floor** である。

- parsed package carrier と checker IR は分ける
- checker report は non-final alpha-local surface とする
- current actualized row は:
  - `CHK-LIF-01/02/03/04`
  - `CHK-VAR-01/02/03`
  - `CHK-CUT-01`
  - `CHK-PKG-01/02`
- accepted proof は `accepted_obligations` で示す
- rejected proof は `rejected_rows` と `diagnostics` で示す
- package admission rows は checker-only preview であり、
  hot-plug/runtime attach verdict completionを意味しない
- current floor は runtime plan を emit しない
- current floor は local run / Docker run / save-load / devtools export を行わない

したがって、`P-A1-02` closeout は useful practical floor だが、
effect row containment、failure row containment、
observation/redaction/retention constraints、
package positive admissionなどを含む
full typed-checking completion と同一視しない。

## first local runtime floor boundary

`P-A1-03` の current actualization は、
reusable runtime 全体の completion ではなく、
**first practical local-runtime floor** である。

- checked package carrier、runtime plan、local runtime report は分ける
- checker report を runtime plan や runtime report と同一視しない
- current actualized row は:
  - `RUN-01`
  - `RUN-02`
- current floor は local queue / `MessageEnvelope` / membership frontier / event DAG export hook を扱う
- capability / auth / witness lane は carrier に残すが、この package で full runtime enforcement を claim しない
- current floor は package/hot-plug practical API completionを意味しない
- current floor は Docker/local TCP transport completionを意味しない
- current floor は local save/load command completionを意味しない
- current floor は final public runtime/devtools ABI を意味しない

したがって、`P-A1-03` closeout は useful practical runtime floor だが、
reusable runtime family 全体、package/hot-plug、transport、
save/load、devtools completion と同一視しない。

## first package / hot-plug floor boundary

`P-A1-04a` / `P-A1-04b` / `P-A1-04c` の current actualization は、
package / hot-plug stage 全体の completion ではなく、
**non-final practical hot-plug first floor** である。

- checked package carrier、hotplug plan、hot-plug report は分ける
- current front-door attach path は manifest-driven `layer` package attach/reject と、
  narrow `object` package attach preview seam を distinct hotplug-plan carrier 経由で扱う
- current actualized row は:
  - `HP-A1-01`
  - `HP-A1-02`
  - `HP-A1-03`
  - `HP-A1-04`
  - `HP-A1-05`
- `HP-A1-04B1`
- `HP-A1-04B2`
- `HP-A1-06`
- `HP-A1-07`
- accepted proof は exact hot-plug report と manifest checks で示す
- rejected / deferred proof は exact hot-plug report の `terminal_outcome`、`reason_family`、
  `rejection_reason_refs`、`witness_reason_refs` で示す
- current floor は debug/auth/rate-limit layer の manifest-driven attach path を actualize する
- current floor は incompatible patch reject と missing attach capability/admin reject を actualize する
- current floor は attach-time stale membership reject と missing witness reject を actualize する
- current floor は narrow object package attach preview seam を actualize する
- current floor は `HP-A1-07` で `operation_kind = detach` と `detach_boundary_ref` を持つ explicit deferred detach minimal contract を actualize する
- current floor は final object package attach completionを意味しない
- current floor は detach runtime lifecycle completionを意味しない
- current floor は Docker/local TCP transport completionを意味しない
- current floor は final public hot-plug/package ABI を意味しない

したがって、`P-A1-04c` closeout は useful practical package/hot-plug floor だが、
`specs/18` package / hot-plug stage 全体、
final object package attach、detach runtime lifecycle、
detach execution / rollback / migration completion と同一視しない。

## first practical avatar-preview companion floor boundary

- current repo state で actualize 済みなのは final avatar runtime completion ではなく、
  **first practical avatar-preview companion floor** である
- current admissible rows は次に限る
  - `AV-A1-01`
    - placeholder avatar object package について、
      exact hot-plug accepted preview seam を distinct avatar preview report へ下ろす
  - `AV-A1-02`
    - custom Mir avatar object package について、
      selected representation を `mir_humanoid_runtime_preview` として示す
      non-final custom avatar preview report を actualize する
  - `AV-A1-03`
    - unsupported custom runtime request について、
      source hot-plug report は missing host capability で rejected のまま保ちつつ、
      visible monotone placeholder fallback を示す companion avatar preview report を actualize する
- current carrier split は次である
  - `checked practical package -> distinct hotplug plan -> exact hot-plug report -> distinct avatar preview report`
- `AV-A1-03` は unsupported runtime execution success を示さない
- `AV-A1-03` の fallback は rejected source hot-plug report に対する visible companion preview であり、
  hidden successful native execution ではない
- current floor は `native_execution_performed = false` を保つ
- current floor は final object package attach completion、
  final avatar package ABI、same-session product runtime completion、
  VRM / VRChat / Unity compatibility、active runnable-root promotionを意味しない

### 3. reusable runtime

- multiple `Place` を持つ reusable runtime API がある
- local queue / `MessageEnvelope` / membership / capability / witness / event DAG を扱う
- world / layer / object / runtime package を attach できる
- local sample world を library/CLI から起動できる

### 4. package / hot-plug

- request / verdict / activation cut が reusable API で使える
- debug layer / auth layer / rate-limit layer / object package の attach path がある
- incompatible patch / missing capability / missing witness / stale membership は typed reject になる
- detach minimal contract は reject / defer / fallback の少なくとも 1 つで explicit に扱う
- current actualization は defer branch を採る。detach request は `detach_boundary_ref` を持つ explicit deferred boundary として扱い、accepted detach execution はまだ claim しない

### 5. transport

- Docker/local TCP E2E が product-like command で動く
- transport / auth / membership / capability / witness lane は分離される
- observer-safe route trace がある
- production WAN / federation は要求しない

## first practical transport floor boundary

`P-A1-05` の current actualization は、
transport stage 全体の completion ではなく、
**non-final practical transport first floor** である。

- checked package carrier、transport plan、transport report は分ける
- current actualized row は:
  - `TR-A1-01`
  - `TR-A1-02`
  - `TR-A1-03`
  - `TR-A1-04`
  - `TR-A1-05`
  - `TR-A1-06`
  - `TR-A1-07`
- current floor は same practical package input を `local_tcp` と `docker_compose_tcp` surface へ下ろす
- current floor は stale-membership / missing-capability / missing-witness negatives を transport-specific reject として actualize する
- current floor は observer-safe route trace を exact transport report に残す
- current floor は auth evidence を transport delivery lane と分けて扱う
- current floor は WAN/federation completionを意味しない
- current floor は local save/load command completionを意味しない
- current floor は devtools / viewer completionを意味しない
- current floor は product prototype completionを意味しない
- current floor は final public transport ABI を意味しない

したがって、`P-A1-05` closeout は useful practical transport floor だが、
transport stage 全体、production WAN/federation、
save/load、devtools、product prototype、
final public transport API と同一視しない。

### 6. save/load

- local save/load が CLI/library から使える
- stale membership / stale lease / stale witness を hidden resurrect しない
- distributed durable save/load は未実装でもよい
- invalid distributed cut は explicit reject のまま残す

#### first practical local save/load floor boundary

- current repo state で actualize 済みなのは full save/load completion ではなく、
  **first practical local save/load floor** である
- current admissible rows は次に限る
  - `SL-A1-01`
    - one exact practical local-runtime frontier を local-only に save/load し、
      resumed dispatch を 1 回 accepted させる
  - `SL-A1-02`
    - one exact practical local-runtime frontier を local-only に save/load したあと、
      later live membership frontier advance を explicit に入れて resumed dispatch を reject させる
- current carrier boundary は次のように分ける
  - checked practical package は distinct runtime-plan carrier を通る
  - save/load path は、one exact practical local-runtime execution から得た saved local frontier と、
    distinct save-load plan carrier の両方を要求する
  - saved local frontier と non-final save-load report は local-runtime report と別 carrier である
- current floor が save/load report へ persist するのは `runtime_snapshot` だけではなく、
  `current_owner`、`visible_history`、`pending_envelopes = []` を含む
  **saved local frontier** である
- current floor は queue/channel/transport live state persistence を claim しない
- current floor は `CHK-CUT-01` を existing orphan-receive checker guard としてだけ reuse する
- `CHK-CUT-01` reuse は full consistent-cut completion、`Z-cycle` completion、
  distributed save/load runtime completionを意味しない
- current floor は stale membership non-resurrection だけを actualize する
- current floor は stale witness non-resurrection、stale lease non-resurrection、
  distributed durable save/load、Docker/local TCP save/load、
  hot-plug lifecycle persistence、final public save-load ABI を意味しない

### 7. devtools / visualization

- event DAG
- route trace
- membership timeline
- publication / witness / handoff relation
- hot-plug lifecycle
- fallback degradation
- redacted observer view

を JSON/export + viewer で確認できる。

#### first practical devtools export floor boundary

- current repo state で actualize 済みなのは full devtools completion ではなく、
  exact practical reports を source とする distinct devtools export bundle +
  non-final viewer の first floor である
- current admissible rows は次に限る
  - `VIS-A1-01`
    - practical local-runtime report から event DAG と publication / witness / handoff relation を export する
  - `VIS-A1-02`
    - practical transport report から observer-safe route trace を export する
  - `VIS-A1-04`
    - exact practical hotplug reports から attach accepted boundary、membership snapshot、deferred detach boundary を export する
  - `VIS-A1-06`
    - practical transport report から auth lane を transport metadata から分離した redacted observer view を export する
- current carrier split は次である
  - `exact practical reports -> distinct devtools export bundle -> non-final viewer`
- この floor は new runtime semantics を追加しない
- この floor は full `PA1-6` completion を意味しない
- この floor は `VIS-A1-03` membership timeline、
  `VIS-A1-05` fallback degradation、`VIS-A1-07` retention/on-demand trace の completion を意味しない
- `VIS-A1-04` は export-side hot-plug lifecycle observability widening に限り、
  detach runtime lifecycle execution、rollback/migration completion、final object package attach completion を意味しない
- この floor は local save/load、product prototype、WAN/federation、
  final public viewer / telemetry / runtime-devtools ABI を意味しない

### 8. product prototype

少なくとも 1 つの small product prototype を
local + Docker で実行できる。

推奨 current target:

- Sugoroku-derived shared world
- debug/auth/rate-limit layer attach
- avatar placeholder/custom package
- local save/load
- devtools export

#### first practical product-preview boundary

current repo state で actualize 済みなのは、
full product prototype completion ではなく、
**first practical product-preview floor** である。

- current carrier split は次に限る
  - preview manifest
  - exact practical reports / exact practical devtools bundles
  - non-final product-preview bundle
- current preview root は `samples/practical-alpha1/previews/`
- current actualized preview rows は次に限る
  - `PE2E-01`
  - `PE2E-02`
  - `PE2E-03`
  - `PE2E-04`
  - `PE2E-05`
  - `PE2E-06`
  - `PE2E-07`
  - `PE2E-08`
  - `PE2E-09`
- `PE2E-03` は debug layer の **companion preview bundle** であり、
  same-session runtime attach execution を証明しない
- `PE2E-04` は `HP-A1-06` placeholder object preview companion evidence に narrow される
- `PE2E-04` は custom Mir avatar runtime を証明しない
- `PE2E-04` は unsupported runtime fallback を証明しない
- `PE2E-08` は `AV-A1-02` exact avatar preview report を consume する
  custom-avatar companion preview bundle であり、
  native execution や same-session runtime attach execution を証明しない
- `PE2E-09` は `AV-A1-03` exact avatar preview report を consume する
  unsupported-runtime visible fallback companion preview bundle であり、
  unsupported-runtime execution success を証明しない
- avatar semantics の source authority は引き続き
  `AV-A1-01/02/03` distinct avatar-preview companion floor に置く
- `PE2E-06` は invalid distributed save/load runtime execution ではなく、
  explicit checker reject preview に留まる
- `PE2E-07` は exact devtools bundles の viewer-openability preview であり、
  final public viewer API ではない
- current floor は monolithic same-session product runtime execution を意味しない
- current floor は final public CLI / viewer / package-avatar / save-load / transport ABI を意味しない
- current floor は practical alpha-1 `100%` completion を意味しない

## non-goals

practical alpha-1 immediate scope では次を goal にしない。

- final public parser grammar
- final public ABI
- production WAN federation
- durable distributed save/load completion
- final public packaging
- marketplace / registry
- Reversed Library implementation
- PrismCascade integration
- full VRM / VRChat / Unity compatibility
- direct native code generation from Mir source

## sample-root boundary

- `samples/alpha/` は current-scope evidence root であり続ける
- `samples/alpha/` を active runnable root に silently promote しない
- practical alpha-1 の front-door root は `samples/practical-alpha1/` または同等の separate root に置く
- current initial cut は `samples/practical-alpha1/` の limited `package.mir.json` family であり、full practical runnable root とはまだ分けて読む

## native boundary

alpha-1 の native requirement は、
Mir source が machine code へ direct compile されることではない。

required:

- Rust native runtime / CLI / library が build できる
- alpha source / IR bundle を native runtime が実行できる
- package manifest admission / reject / fallback が native process で扱える

not required:

- direct native code generation
- signed native binary = semantic safety という claim

signature は provenance を示すだけであり、semantic safety proof ではない。

## practical alpha-1 validation boundary

package closeout の basic floor は少なくとも次である。

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

加えて、changed lane に応じた focused validation を実行する。
Docker が使えない場合は success claim をせず、skip reason を report に残す。

## stop line

- current-scope evidence closeout `100%` を practical alpha-1 `100%` と書かない
- helper-local / sidecar / runtime-mirror / non-public floor を product-ready runtime と書かない
- parser/front-door が無い sample を source-executed と書かない
- first checker floor を full `specs/18` typed-checking completion と書かない
- Docker/local TCP を production WAN と書かない
- local save/load を durable distributed save/load と書かない
- avatar / VRM / VRChat / Unity を Mir core primitive にしない
- PrismCascade を Mirrorea practical alpha-1 implementation scope に入れない

## success statement

practical alpha-1 完了時に言えるのは次である。

> Mirrorea Spaces practical alpha-1 は、
> final public product ではないが、
> source front-door から小さな shared virtual-space product prototype を
> 型検査し、local/Docker runtime で実行し、layer/package hot-plug し、
> devtools で観測し、local save/load できる toolchain である。
