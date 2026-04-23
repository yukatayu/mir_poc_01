# clean near-end modal 01

## 要約

current clean near-end modal line は、raw `◯` / `□` を final syntax にせず、`stable`、`later`、`published(room)`、`witnessed(draw_pub)` のような **Mir 側の mode evidence** として扱う構成です。

## 何が built-in か

built-in として扱うのは、`transition`、`stage`、`publish`、`witness` などの最小構文語です。

## 何が user-defined か

この family では次が user-defined です。

- `GameConfig`
- `draw_pub`
- `published(room)`
- `witnessed(draw_pub)`

つまり、modal reading は current sample と mode constraint によって支えられており、raw modal symbol を public syntax として確定したわけではありません。

## active sample

- `01_stage_stable_later_minimal`
- `02_published_witnessed_mode_bridge`

## この family で確認できること

- `stable` value を stage をまたいで使う current reading
- `later` value を publish stage で要求する current reading
- `published(room)` から `witnessed(draw_pub)` への bridge

## 現在の評価

この family は repo-local alpha-ready current layer としては成立しています。
ただし、final public modal syntax、full multimodal dependent type theory、public artifact contract まではまだ未完です。

## 実行コマンド

```bash
python3 scripts/clean_near_end_samples.py run modal --format json
```

## detail

full sample code と actual JSON output は `clean_near_end_modal_01_detail.md` を参照してください。
