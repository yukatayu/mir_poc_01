# typed external boundary canaries 01

## この文書の役割

この文書は、phase 9 `Typed external boundary / adapter` の
**current synthetic preview helper cut** を最短コマンドで追うための hands-on landing page です。

- final public adapter API ではありません
- browser / network / VR host schema ではありません
- helper-local preview を public contract と混同しません

## まず実行するコマンド

```bash
python3 scripts/typed_external_boundary_samples.py list
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug envelopes --format json
python3 scripts/typed_external_boundary_samples.py run EXT-03 --debug visualization --format json
python3 scripts/typed_external_boundary_samples.py run EXT-04 --debug failures --format json
python3 scripts/typed_external_boundary_samples.py check-all --format json
python3 scripts/typed_external_boundary_samples.py closeout --format json
python3 -m unittest scripts.tests.test_typed_external_boundary_samples
```

## current synthetic preview subset

- `EXT-03`
  local queue room-message synthetic preview lane。
  effect boundary、transport envelope、auth evidence、witness refs を collapse しない。
- `EXT-04`
  typed adapter failure synthetic preview lane。
  adapter failure を hidden retry や domain rejection に潰さない。

## residual planned family

- `EXT-01`
  local console scenario。
  stdio builtin 誤読を避けるため residual planned に保つ。
- `EXT-02`
  world overlay scenario。
  projection / visualization split と結びつくため residual planned に保つ。
- `EXT-05`
  standalone visualization restriction scenario。
  current helper cut では `EXT-03` の visualization view に吸収する。

## これで確認できること

- standard I/O を Mir core primitive にせず、typed effect / adapter boundary に残す current reading
- local queue lane で、effect boundary、transport envelope、auth evidence、witness refs を separate に保つ synthetic preview helper evidence
- typed adapter failure を explicit result として返し、hidden retry や hidden repair をしない synthetic preview helper evidence
- visualization も effect であり、label / authority / redaction を持つ synthetic preview helper evidence

## これではまだ確認できないこと

- final public adapter API / FFI
- exact host schema
- browser / network / VR host family split
- real transport widening
- final public visualization service contract
- phase 9 `.mir` files の direct semantic execution

## current anchor

- `../../plan/25-typed-external-boundary-executable-roadmap.md`
- `../../samples/not_implemented/typed-external-boundary/README.md`
