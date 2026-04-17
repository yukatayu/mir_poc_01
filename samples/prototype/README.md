# prototype samples

この directory は、**機能 family 自体は比較検討中だが、current L2 parser / runner で修正 prototype を回して挙動比較したい** sample を置く。

- `samples/current-l2/` には入れない。
- ただし current L2 fixed subset の text lowerer で parse できる形には寄せる。
- 実行するときは `mir-current-l2 run-source-sample <path> --format pretty|json` を使う。
- prototype sample は explicit path で実行し、sample file の隣に `.host-plan.json` sidecar があれば `--host-plan` を省略できる。
- sample debugging では、sidecar が `debug_*` または `_debug_` を含み `_output` / `_pipe` で終わる target に append した record を CLI が `debug_outputs` として preview してよい。
  これは helper-local convenience であり、final host-I/O / shell contract ではない。

## current subdirectories

- `current-l2-order-handoff/`
  - `atomic_cut` / publication / handoff / room-local serial transition の corrected prototype
- `current-l2-dynamic-attach-detach/`
  - attach / detach の corrected prototype
