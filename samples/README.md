# samples

この directory は、**active sample、base corpus、planned skeleton、prototype reference、archive、generated reserve** を分けて置く。

## current taxonomy

- `clean-near-end/`
  active canonical executable suite
- `current-l2/`
  base current-L2 source corpus
- `lean/`
  mechanization evidence
- `alpha/`
  Mirrorea Spaces alpha-0 phase-indexed sample matrix scaffold with `.expected.json` sidecars
  - current-scope evidence root であり、practical alpha-1 front-door root ではない
  - Stage A..F `100%` は evidence closeout として読む
- `practical-alpha1/`
  practical alpha-1 front-door sample root
  - current cuts は limited `package.mir.json` front-door fixture family、first checker-floor fixture family、first local-runtime fixture family、attach-time freshness/witness negatives、narrow object preview seam、explicit deferred detach minimal contract を含む practical hot-plug fixture family、exact hot-plug reports を source とする `AV-A1-01/02/03` practical avatar preview fixture family、distinct transport-plan/report carrier を通る `TR-A1-01..07` practical transport fixture family、exact practical reports を source とする `VIS-A1-01/02/03/04/05/06` practical devtools export bundle family、one exact practical local-runtime frontier と distinct save-load plan を前提にする `SL-A1-01/02` practical local save/load fixture family、preview manifest から exact practical reports / exact practical devtools bundles / exact avatar preview reports を束ねる `PE2E-01..09` practical product-preview family
  - active canonical runnable root や full toolchain root ではまだない
- `not_implemented/`
  residual planned skeleton family
- `prototype/`
  historical prototype / compatibility anchor
- `old/`
  archive
- `generated/`
  non-Lean generated sample artifact reserve + committed generated bridge evidence

## reading rules

- active executable sample として読むのは、まず `clean-near-end/`
- `current-l2/` は active clean suite を支える base source corpus
- `lean/clean-near-end/` は generated theorem stub だが、proof bridge evidence として committed されている
- `alpha/` は current alpha-local theory-freeze / checker/runtime roadmap 用の scaffold であり、expected-verdict sidecar を伴う
  - current task では mixed alpha-local scaffold root として扱い、active runnable sample root に silently promote しない
  - root 内には scaffold-only family と runner-backed non-public floor が併存するが、いずれも repo の active canonical root ではない
  - `lifetime-fallback/` は `LIF-01/05..08`、`contract-variance/` は `VAR-02/03/05/07/09/10/15` について `reason_codes_scope = alpha-static-floor` の non-public checker-floor seed rows を current repo state で持つ
  - `lifetime-fallback/` の `LIF-02/03/04` と `contract-variance/` の `VAR-01/04/06` は `acceptance_scope = alpha-acceptance-floor` の helper-local synthetic acceptance rows を current repo state で持つ
  - `lifetime-fallback/` の `LIF-13` は `snapshot_scope = alpha-snapshot-selected-floor` の helper-local synthetic snapshot-selected row を current repo state で持つ
  - `lifetime-fallback/` の `LIF-11` は `anchor_handoff_scope = alpha-anchor-handoff-floor` の helper-local synthetic anchor-handoff row を current repo state で持つ
  - `contract-variance/` の `VAR-08/11/13` は `runtime_mirror.scope = alpha-runtime-mirror-floor` の non-public runtime-mirror rows を current repo state で持ち、source authority は `layer-insertion/` の `LI-04/01/03` runtime-floor sidecars に残す
  - `reason_codes_scope` confinement、`acceptance_scope` confinement、`snapshot_scope` confinement、`anchor_handoff_scope` confinement、`runtime_mirror.scope` confinement は別であり、positive row は negative reason code 不在ではなく explicit acceptance rows、snapshot rows、anchor-handoff rows、または runtime-mirror rows で扱う
  - current package line では `local-runtime/`、`layer-insertion/`、`network-docker/`、`avatar-runtime/` が non-public Rust/runtime-private floor を持つが、sample file parsing front door ではなく、active root でもない。`local-runtime/` は `alpha_local_runtime_samples.py` により current-scope Stage B closeout surface を持つが、これは `CUT-04/17` local-only save/load subset を supporting evidence として再利用するだけで、distributed save/load や active runnable-root promotion は主張しない
  - `layer-insertion/` と `avatar-runtime/` / `hotplug-runtime/` は `alpha_hotplug_lifecycle_samples.py` により current-scope Stage D closeout surface を持つが、これは `LI-01/02/03/04/05` と `AV-01/02/06/08/09` / `HP-11/12/15` attach-time/package-admission subset を束ねるだけで、detach runtime、durable migration、distributed activation ordering、native execution realization、final public ABI は主張しない
  - `network-docker/` は `mirrorea_alpha_network_runtime` + `alpha_network_docker_e2e.py` により `NET-02/03/04/05/07/09` を narrow Stage-C transport / Docker cut として actualize し、`stage-c-closeout` command が current-scope Stage C closeout surface を与える。helper-local `clean-near-end/network-transport/` canary familyとは別物であり、`NET-01/06/08/10` は引き続き planned
  - `avatar-runtime/` は `mirrorea_alpha_avatar_runtime` + `alpha_avatar_runtime_samples.py` により `AV-01/02/06/08/09` を runtime-private package/avatar admission floor として actualize した。`HP-11/12/15` も同 runner で検証するが、`hotplug-runtime/` family 全体を runnable root へ昇格したわけではない
  - `cut-save-load/` は `alpha_cut_save_load_samples.py` により `CUT-04` local-only save/load bridge、`CUT-17` stale-membership rejection bridge、`CUT-11` checker-backed Z-cycle inadmissibility row を actualize し、selected negative rows の checker floor と併存する。`P-A0-23` 以降は `CUT-04/17` が Stage B closeout supporting subset として再利用されるが、family 全体の 100% 化や `CUT-10/12/16` completion は意味しない
  - `visualization/` は `alpha_visualization_samples.py` により `VIS-01/02/03/05/06/07/08/10/11` を dedicated Stage-E subset runner として actualize し、`stage-e-closeout` command が current-scope Stage E completion surface を与える。`VIS-04/09/12` は planned-only のままであり、family 全体や final viewer API を completion 扱いしない
  - `e2e/` は `alpha_e2e_samples.py` により `E2E-01/02/03/04/05/06/07/09/10` を thin integrated Stage-F bridge として actualize し、`stage-f-closeout` が current-scope Stage F closeout surface を与える。`E2E-08` upper-layer seed は planned-only のまま保ち、public alpha / `U1` completion や active runnable-root promotionには使わない
  - `hotplug-runtime/` と `contract-variance/` の overlapping rows は引き続き planned mirror / verdict authority であり、runtime-sensitive attach evidence authority は `layer-insertion/` 側に置く。`P-A0-18` では `VAR-08/11/13` だけを `LI-04/01/03` から mirror actualization したが、`contract-variance/` 自体を runnable root へ昇格していない。`hotplug-runtime/` では `HP-11/12/15` だけが avatar/package runner 経由の native-policy subset として actualize 済み
  - family ごとの status / blocker / next runner は `samples_progress.md` と `progress.md` / `tasks.md` を参照する
- `not_implemented/` は residual planned skeleton であり、active sample ではない
  - `avatar-fairy-follow/` は phase 8 residual planned family
  - `typed-external-boundary/` は phase 9 residual planned family
    - `scripts/typed_external_boundary_samples.py` はこの root の `EXT-03` / `EXT-04` を helper-local synthetic preview anchor として読むが、active semantic sample root へ昇格させない
  - `network-transport/` は phase 13 planned family
- `clean-near-end/avatar-follow/` は phase 8 active representative slice
- `clean-near-end/network-transport/` は phase 13 active helper-local canary landing page
  - runnable helper-local canary ID は `NET-02` / `NET-03` / `NET-04` / `NET-05`
  - `NET-01` は standalone sample ID ではなく、Sugoroku loopback parity anchor として reported に扱う
- `prototype/` は historical anchor / compatibility reference であり、active canonical path ではない
- `old/` は archive
- `generated/` は reserve path であり、source sample を置かない
  - current committed generated bridge evidence は `samples/generated/projection-placement/manifest.json`
  - これは generated artifact であり、source sample でも final emitted executable program でもない
  - projection / placement の actual emitted executable family、optimizer、deployment planner は kept-later gate に残す
- `not_implemented/` と `alpha/` を混同しない
  - `alpha/` は current alpha-local promoted scaffold
  - `not_implemented/` は residual / historical planned family の preservation root
- `alpha/` と `practical-alpha1/` を混同しない
  - `alpha/` は evidence closeout root
  - `practical-alpha1/` は current repo state では `package.mir.json` loader fixtures、first checker-floor fixtures、first local-runtime fixtures、`HP-A1-01..05` / `HP-A1-04B1` / `HP-A1-04B2` / `HP-A1-06` / `HP-A1-07` hot-plug fixtures、`TR-A1-01..07` transport fixtures、`SL-A1-01/02` local save/load fixtures を持つ
  - `practical-alpha1/` もまだ active canonical runnable root ではなく、`previews/` root に `PE2E-01..09` の first practical product-preview floor を持ち、別 lane で `AV-A1-01/02/03` の first practical avatar preview floor を持つが、native avatar execution、same-session product runtime、final object package attach、detach runtime lifecycle、WAN/federation、distributed durable save/load、full product surfaces は later packages に残る
  - current practical devtools export floor は `VIS-A1-01/02/03/04/05/06` に限られ、`VIS-A1-03` は exact `SL-A1-02` save-load report から saved frontier / later live membership advance / restored frontier / stale-membership reject を export する membership timeline widening、`VIS-A1-04` は exact practical hotplug reports から attach accepted boundary / membership snapshot / deferred detach boundary を export する observability widening に留まり、`VIS-A1-05` は exact `AV-A1-03` avatar preview report から rejected source lane / degraded roles / missing host capability を export する fallback degradation widening に留まる。`P-A1-14` 以後、`VIS-A1-07` retention/on-demand trace は retained-artifact catalog / on-demand retrieval trace を持つ exact source carrier が出るまで blocker として保持する

## current commands

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/clean_near_end_samples.py smoke-all --format json
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/avatar_follow_samples.py check-all --format json
python3 scripts/typed_external_boundary_samples.py check-all --format json
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/projection_codegen_samples.py check-all --format json
python3 scripts/visual_debugger_viewer_samples.py check-all --format json
cargo test -p mir-ast practical_alpha1_front_door -- --nocapture
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_transport.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_save_load.py check-all --format json
```

- `current_l2_guided_samples.py` は active current-L2 front-door compatibility wrapper であり、`list` / `smoke-all` / `closeout` を `clean_near_end_samples.py` へ forward する
- docs checks、storage guardrail、Cargo regression を含む broader validation floor は `samples_progress.md`、`progress.md`、`tasks.md` を参照する

## move policy

- active sample は silent delete せず、archive へ移す
- generated artifact と source sample を混ぜない
- heavy disposable generated artifact は repo root ではなく external workdir を優先する
