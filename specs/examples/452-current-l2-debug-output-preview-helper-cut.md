# 452 — current L2 debug output preview helper cut

## 目的

current runnable sample と corrected prototype を実行するときに、
sample debugging 用の可視化を **host-I/O finalization や stdio adoption と混同せずに** 扱うための helper cut を与える。

## current judgment

1. current repo は `stdin/stdout` を Mir の built-in surface として採用しない。
2. current sample debugging では、host-plan sidecar の commit mutation が書き込む target のうち
   - `debug_*`
   - `_debug_` を含み `_output` で終わる target
   - `_debug_` を含み `_pipe` で終わる target
   に当たるものを **debug output preview target** として扱ってよい。
3. current operational CLI は、上の target に入った record を `debug_outputs` として表示してよい。
   表示対象は final place-store に残った record に限り、rollback で消える record を debug transcript として保持するとは読まない。
4. これは helper-local preview cut であり、
   - final host-I/O model
   - final adapter / FFI model
   - final public shell contract
   - final external transcript format
   を意味しない。
5. current sample では、sample family に応じて
   - `dice_debug_text_output`
   - `dice_recovery_debug_output`
   - `avatar_controller_debug_output`
   のような name を使ってよい。

## current non-goals

- `stdout` 相当の built-in channel を導入すること
- debug output preview target を final language keyword に昇格すること
- debug output record format を final external contract として固定すること
- FFI / engine adapter / host runtime binding をここで決めること
