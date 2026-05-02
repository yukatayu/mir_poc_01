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
  - current package line では `local-runtime/`、`layer-insertion/`、`network-docker/`、`avatar-runtime/` が non-public Rust/runtime-private floor を持つが、sample file parsing front door ではなく、active root でもない
  - `network-docker/` は `mirrorea_alpha_network_runtime` + `alpha_network_docker_e2e.py` により `NET-02/03/04/05/07/09` を narrow Stage-C transport / Docker cut として actualize した。helper-local `clean-near-end/network-transport/` canary familyとは別物であり、`NET-01/06/08/10` は引き続き planned
  - `avatar-runtime/` は `mirrorea_alpha_avatar_runtime` + `alpha_avatar_runtime_samples.py` により `AV-01/02/06/08/09` を runtime-private package/avatar admission floor として actualize した。`HP-11/12/15` も同 runner で検証するが、`hotplug-runtime/` family 全体を runnable root へ昇格したわけではない
  - `cut-save-load/` は `alpha_cut_save_load_samples.py` により `CUT-04` local-only save/load bridge、`CUT-17` stale-membership rejection bridge、`CUT-11` checker-backed Z-cycle inadmissibility row を actualize し、selected negative rows の checker floor と併存する
  - `visualization/` は `alpha_visualization_samples.py` により `VIS-01/02/03/05/06/07/08/10/11` を dedicated Stage-E subset runner として actualize したが、`VIS-04/09/12` は planned-only のままであり、family 全体や final viewer API を completion 扱いしない
  - `e2e/` は `alpha_e2e_samples.py` により `E2E-01/02/03/04/05/06/07/09/10` を thin integrated Stage-F bridge として actualize したが、`E2E-08` upper-layer seed は planned-only のまま保つ
  - `hotplug-runtime/` と `contract-variance/` の overlapping rows は引き続き planned mirror / verdict authority であり、first runtime-sensitive attach evidence authority は `layer-insertion/` 側に置く。`hotplug-runtime/` では `HP-11/12/15` だけが avatar/package runner 経由の native-policy subset として actualize 済み
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
```

- `current_l2_guided_samples.py` は active current-L2 front-door compatibility wrapper であり、`list` / `smoke-all` / `closeout` を `clean_near_end_samples.py` へ forward する
- docs checks、storage guardrail、Cargo regression を含む broader validation floor は `samples_progress.md`、`progress.md`、`tasks.md` を参照する

## move policy

- active sample は silent delete せず、archive へ移す
- generated artifact と source sample を混ぜない
- heavy disposable generated artifact は repo root ではなく external workdir を優先する
