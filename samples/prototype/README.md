# prototype samples

この directory は、**機能 family 自体は比較検討中だが、current L2 parser / runner で修正 prototype を回して挙動比較したい** sample を置く。

- `samples/current-l2/` には入れない。
- ただし current L2 fixed subset の text lowerer で parse できる形には寄せる。
- 実行するときは `mir-current-l2 run-source-sample <path> --format pretty|json` を使う。
- prototype sample は explicit path で実行し、sample file の隣に `.host-plan.json` sidecar があれば `--host-plan` を省略できる。
- sample debugging では、sidecar が `debug_*` または `_debug_` を含み `_output` / `_pipe` で終わる target に append した record を CLI が `debug_outputs` として preview してよい。
  これは helper-local convenience であり、final host-I/O / shell contract ではない。
- theorem / model-check bridge の current floor では、CLI が `verification_preview` と helper-local `artifact_preview` を出してよい。
  これは final public verifier contract や final emitted artifact schema を意味しない。

## current subdirectories

- `current-l2-order-handoff/`
  - `atomic_cut` / publication / handoff / room-local serial transition / static-stop / guarded chain の corrected prototype
- `current-l2-dynamic-attach-detach/`
  - attach / detach の corrected prototype

## current limit

- `underdeclared` omission family (`e5` / `e12`) は `samples/current-l2/` authored corpus に actualize 済みである。
- prototype bucket は、それでもなお mixed-gate の corrected prototype を runnable に比較したい family を置く場所として保つ。
