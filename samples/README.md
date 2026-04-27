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
  - `avatar-fairy-follow/` は phase 8 planned family
  - `typed-external-boundary/` は phase 9 planned family
- `prototype/` は historical anchor / compatibility reference であり、active canonical path ではない
- `old/` は archive
- `generated/` は future reserve path であり、source sample を置かない

## current commands

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/clean_near_end_samples.py smoke-all --format json
python3 scripts/sugoroku_world_samples.py check-all
```

## move policy

- active sample は silent delete せず、archive へ移す
- generated artifact と source sample を混ぜない
- heavy disposable generated artifact は repo root ではなく external workdir を優先する
