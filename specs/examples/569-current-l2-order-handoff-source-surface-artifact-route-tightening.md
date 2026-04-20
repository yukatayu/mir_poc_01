# 569. current L2 order-handoff source surface / artifact route tightening

## 目的

`specs/examples/490`、
`503`、
`526`、
`527`、
`533`
と
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`、
`p13-dice-late-join-missing-publication-witness`、
`p14-dice-late-join-handoff-before-publication`
を前提に、

- explicit edge-row principal
- stage-block secondary
- repo-local emitted artifact reading
- delegated RNG placement reserve
- negative static-stop pair
- `run-source-sample` helper summary

を 1 本の actual adoption package として揃える。

ここで actualize するのは、
**Problem 2 current first line のうち、source surface と emitted artifact の読み筋を helper-local operational summary へ narrow に mirror する current cut**
であり、

- final parser grammar
- final public parser / checker / runtime API
- final source-surface handoff wording
- final emitted-artifact schema
- final emitted-handoff contract
- final public witness / provider / combined contract
- low-level `memory_order` exact source surface
- authoritative-room `serial on ...` sugar の public principal promotion
- final modal foundation adoption

は still later に残す。

## current first line

current repo では、次を current first line に置く。

1. principal source surface
   - edge-row / vertical continuation を principal に置く
   - `publish` / `handoff` / `observe`
   - `rollback` / `refresh` / `invalidate`
   - `after ...`
   - `requires witness(...)`
   を head / continuation で見せる
2. secondary source surface
   - stage-block は readable secondary candidate として保つ
   - principal edge-row surface を置き換えない
3. authoritative-room reserve
   - `serial on ...` は authoritative-room-specific reserve surface に留める
   - `p09` delegated RNG placement はこの practical route に残す
4. negative pair
   - `p13` missing publication witness
   - `p14` handoff before publish
   は current late-join visibility line の helper-local negative static-stop pair として保つ
5. artifact reading
   - source surface reached pair `p07 / p08` では repo-local emitted artifact refs を source wording route と隣接させて読んでよい
   - ただし final emitted-artifact schema adoption とは読まない

## actualization cut

current package では、`run-source-sample` helper summary に
`order_handoff_source_surface_artifact_actual_adoption`
を追加してよい。

この summary は次の cut を持つ。

### reached pair

- `p07-dice-late-join-visible-history`
- `p08-dice-stale-reconnect-refresh`

では次を出してよい。

- `profile_axis_refs`
- `principal_surface_lines`
- `secondary_surface_lines`
- `repo_local_emitted_artifact_refs`
- `source_wording_route_refs`
- `emitted_artifact_candidate_keep_refs`

### reserve pair

- `p09-dice-delegated-rng-provider-placement`

は `guarded_not_reached` に保ち、
guard reason で
**serial-scope practical route に残す current cut**
を明示してよい。

これは delegated RNG placement を否定するものではなく、
first pair `p07 / p08` と source-surface principal package を collapse しないための guard である。

### negative pair

- `p13-dice-late-join-missing-publication-witness`
- `p14-dice-late-join-handoff-before-publication`

は `guarded_not_reached` に保ちつつ、
`negative_static_stop_refs` で

- publication witness required before handoff
- publish then handoff then observe order required

を sample-visible にしてよい。

## actual runnable evidence

| evidence | current reading |
|---|---|
| `cargo test -p mir-runtime --test current_l2_operational_cli` | `run-source-sample` JSON/pretty summary に source-surface/artifact route tightening が現れることを machine-check する |
| `p07` | edge-row principal + stage-block secondary + repo-local emitted artifact refs reached |
| `p08` | rollback/refresh/replay family の edge-row principal + stage-block secondary reached |
| `p09` | serial reserve practical route keep、source-surface principal pair には入れない |
| `p13 / p14` | late-join visibility negative pair を helper-local static stop として保つ |

## current recommendation

1. order/handoff source surface は low-level exact surface に戻さず、edge-row / vertical continuation を principal に保つ。
2. stage-block は current readable secondary candidate として保つ。
3. repo-local emitted artifact refs は reached pair `p07 / p08` に隣接させて読む。
4. delegated RNG placement `p09` は serial-scope practical route に残し、first pair に混ぜない。
5. `p13 / p14` negative pair は source-surface package の adequacy corpus として visible に保つ。

## retained alternatives

- stage-block surface を principal に昇格する cut
- `serial on ...` reserve surface を first public principal に昇格する cut
- low-level `memory_order` exact source surface を principal source wording に戻す cut
- `p09` を first source-surface pair に混ぜる cut
- `p13 / p14` negative pair を docs-only のままに戻す cut

## stop line

この package の先で still later に残すものは次である。

- final source-surface handoff wording
- final emitted-artifact schema
- final emitted-handoff contract
- final public witness / provider / combined contract
- low-level `memory_order` exact source surface
- authoritative-room `serial` sugar public promotion
- final modal foundation adoption

## next line

current package を close した後の active queue は、

1. Package 96 authoritative-room first scenario
2. Package 97 reserve strengthening
3. Package 98 documentation/report closeout

として読むのが自然である。
