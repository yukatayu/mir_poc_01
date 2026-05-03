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

### 5. transport

- Docker/local TCP E2E が product-like command で動く
- transport / auth / membership / capability / witness lane は分離される
- observer-safe route trace がある
- production WAN / federation は要求しない

### 6. save/load

- local save/load が CLI/library から使える
- stale membership / stale lease / stale witness を hidden resurrect しない
- distributed durable save/load は未実装でもよい
- invalid distributed cut は explicit reject のまま残す

### 7. devtools / visualization

- event DAG
- route trace
- membership timeline
- publication / witness / handoff relation
- hot-plug lifecycle
- fallback degradation
- redacted observer view

を JSON/export + viewer で確認できる。

### 8. product prototype

少なくとも 1 つの small product prototype を
local + Docker で実行できる。

推奨 current target:

- Sugoroku-derived shared world
- debug/auth/rate-limit layer attach
- avatar placeholder/custom package
- local save/load
- devtools export

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
