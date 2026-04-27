# not_implemented samples

この directory は、**rough stimulus としては重要だが、current parser / runner ではそのまま扱えない** sample を置く。

- exact token / macro / syntax を preservation 目的で残す。
- current repo では parse / run しない。
- current runnable analogue がある場合は `samples/clean-near-end/` または `samples/current-l2/` から辿る。
- historical prototype anchor が必要な場合だけ `samples/prototype/current-l2-dynamic-attach-detach/` を参照する。

## current subdirectories

- `avatar-fairy-follow/`
  - phase 8 planned skeleton family
  - current parser / runner では扱わない
  - historical prototype anchor と future active helper surface を切り分けるための planning path
- `typed-external-boundary/`
  - phase 9 planned skeleton family
  - current parser / runner では扱わない
  - provider boundary / local queue / typed failure / debug label restriction の docs-first path
- `network-transport/`
  - phase 13 planned skeleton family
  - current parser / runner では扱わない
  - local queue / provider boundary current anchor から loopback / reconnect / failure matrix へ widening する docs-first path
