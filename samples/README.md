# samples

この directory は、**active sample、base corpus、planned skeleton、prototype reference、archive、generated reserve** を分けて置く。

## current taxonomy

- `clean-near-end/`
  active canonical executable suite
- `current-l2/`
  base current-L2 source corpus
- `lean/`
  mechanization evidence
- `not_implemented/`
  planned skeleton family
- `prototype/`
  historical prototype / compatibility anchor
- `old/`
  archive
- `generated/`
  future non-Lean generated sample artifact reserve

## reading rules

- active executable sample として読むのは、まず `clean-near-end/`
- `current-l2/` は active clean suite を支える base source corpus
- `lean/clean-near-end/` は generated theorem stub だが、proof bridge evidence として committed されている
- `not_implemented/` は planned skeleton であり、active sample ではない
  - `avatar-fairy-follow/` は phase 8 residual planned family
  - `typed-external-boundary/` は phase 9 residual planned family
  - `network-transport/` は phase 13 planned family
- `clean-near-end/avatar-follow/` は phase 8 active representative slice
- `clean-near-end/network-transport/` は phase 13 active helper-local canary landing page
- `prototype/` は historical anchor / compatibility reference であり、active canonical path ではない
- `old/` は archive
- `generated/` は future reserve path であり、source sample を置かない
  - projection / placement の emitted place-specific program family も later package (`P15`) までは reserve 扱いに保つ

## current commands

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/clean_near_end_samples.py smoke-all --format json
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/avatar_follow_samples.py check-all --format json
python3 scripts/typed_external_boundary_samples.py check-all --format json
python3 scripts/network_transport_samples.py check-all --format json
```

## move policy

- active sample は silent delete せず、archive へ移す
- generated artifact と source sample を混ぜない
- heavy disposable generated artifact は repo root ではなく external workdir を優先する
