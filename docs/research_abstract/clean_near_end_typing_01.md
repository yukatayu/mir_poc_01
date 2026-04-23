# clean near-end typing 01

## 要約

current clean near-end typing line は、**finite decidable index fragment による first strong typing layer** を示します。
ここでは、authority / label / capture / region / cost を user-defined finite theory として宣言し、magical builtin predicate に依存しません。

## 何が built-in か

current helper / sample line で built-in として扱うのは、`module`、`index`、`policy`、`principal`、`resource`、`effect`、`perform`、`require`、`ensure`、`transition`、`stage` などの最小構文語です。

## 何が user-defined か

この family では次が user-defined です。

- `SecurityLabel`
- `FingerprintAuthority`
- `CaptureScope`
- `Region`
- `CostBudget`
- `FingerprintReleasePolicy`
- `Public`
- `UserSecret`
- `KeyMaterial`
- `Observer`
- `Holder`
- `Releaser`
- `Admin`

つまり、旧来の権限専用 predicate 名を built-in predicate として持つ設計ではありません。

## active sample

- `01_authorized_declassification`
- `02_unauthorized_declassification_rejected`
- `03_label_flow_rejected`
- `04_capture_escape_rejected`
- `05_cost_bound_rejected`

## この family で確認できること

- authorized declassification は finite authority preorder の制約で通る
- unauthorized declassification は preorder constraint failure で止まる
- label flow violation は lattice / order constraint failure で止まる
- capture escape は capture subset と region outlives の failure で止まる
- cost overrun は simple bound failure で止まる

## 現在の評価

この family は repo-local alpha-ready current layer として成立しています。
ただし、これだけで final public type system、full dependent type theory、final public checker contract まで終わったとは言えません。

## 実行コマンド

```bash
python3 scripts/clean_near_end_samples.py run typing --format json
```

## detail

full sample code、共有 index theory、actual JSON output は `clean_near_end_typing_01_detail.md` を参照してください。
