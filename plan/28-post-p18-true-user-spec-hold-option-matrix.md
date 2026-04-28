# plan/28 — post-`P18` true user-spec hold option matrix

## role

この文書は、`P18` repo-side first cut の後に残る
**true user-spec hold line** を decision-ready な option inventory に整える
repository memory である。

- actual product target をここで確定しない
- final public freeze をここで claim しない
- current helper-local preview / report-local inventory / crate-local carrier を final public ABI と読み替えない

## already fixed before `U1`

- `P18` で閉じたのは repo-side freeze checklist / public-boundary inventory / mixed-gate と true user-spec hold line の分離である
- typed external boundary は `helper_local_synthetic_preview` と `host_boundary_inventory` までであり、exact host schema や final public adapter ABI ではない
- projection / placement は committed generated bridge evidence までであり、final emitted executable family ではない
- viewer は `typed public prototype inventory over helper/runtime surfaces; not a final public viewer API` までである
- hot-plug は helper-local package-manager preview inventory までであり、runtime hot-plug engine / final ABI ではない
- shared-space current subset は `Place != participant`、`world` host-side sugar、`MembershipRegistry` source of truth、`PlaceCatalog` と thin runtime shell までである

## axis 1. packaging shape / installed binary target

### options

1. `CLI`
2. `library`
3. `engine-adapter`
4. `hybrid`

### current recommendation

`library-first` を provisional recommendation に置く。

- current shell actualization を installed binary fact と読み替えない packaging guard が既にある
- `CLI second gate` は library-first ordering を保つための later gate として source-backed に残っている
- final command hierarchy、final flag naming、distribution policy、engine binding target は still later である

### what this means

- current package では installed binary adoption を決めない
- library surface を先に public-boundary candidate として比較し、
  actual installed binary promotion は second gate として扱う
- `hybrid` は library-first + thin CLI companion が必要になった時にだけ reopen する

## axis 2. host integration target

### options

1. `browser`
2. `native process`
3. `engine`
4. `mixed`

### current recommendation

`native process` を provisional recommendation に置く。

- current helper / runtime / closeout evidence は same-process / process-boundary shapeが strongest anchor である
- `host-facing port` は working label に留め、FFI / engine adapter は later gate に残す source-backed line がある
- browser / engine / mixed は exact host schema、adapter ABI、visualization / runtime binding decision を早く要求しすぎる

### what this means

- current `host_boundary_inventory` と `browser_network_vr_host_family_split` は option label の inventory に留める
- `engine-abi` placeholder を actual target adoption と読まない

## axis 3. first shipped public surface scope

### options

1. `parser/checker/runtime/verifier first`
2. `adapter/viewer/projection/hot-plug first`
3. `two-step split`

### current recommendation

`two-step split` を provisional recommendation に置く。

- first step:
  core library surface candidate を parser/checker/runtime/verifier 側で narrow に整理する
- second step:
  adapter / viewer / projection / hot-plug / transport を host target つき integration surface として reopen する

### why

- `P18` inventory は mixed gate 全体を span しており、parser-only ship を source-backed に決めてはいない
- ただし integration surfaces は packaging shape と host target の影響を強く受ける
- core surface と integration surface を 1 bucket にすると overclaim と drift が起きやすい

## axis 4. final shared-space operational catalog breadth

### options

1. `minimal subset`
2. `portal / multi-world expansion`
3. `fairness / quorum / exhaustive catalog`

### current recommendation

`minimal subset` を長く維持する。

- current authoritative-room / logical multi-place subset は source-backed で強い
- replay / fairness / final witness-provider contract / exhaustive catalog は still later
- final catalog drafting を始めてもよいが、actual adoption は true user-spec gate に残す

## broader application target

この文書では principal axis にしない。

- broader application target は上の 4 軸に依存する
- current recommendation は、
  まず packaging shape / host target / shipped-surface scope / catalog breadth を narrow にしてから
  upper-layer application target を reopen することである

## validation floor

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

## stop line

- installed binary promotion を current shell actualization と混同しない
- `host_boundary_inventory` を final public adapter ABI や exact host schema と呼ばない
- `typed public prototype inventory` を final public viewer API と呼ばない
- committed generated bridge evidence を final emitted executable family と呼ばない
- package-manager preview inventory を runtime hot-plug engine / final ABI と呼ばない
- `U1` option matrix を actual product decision と呼ばない

## related memory

- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/25-typed-external-boundary-executable-roadmap.md`
- `plan/27-public-api-parser-gate-roadmap.md`
