# 2034 — Why Alpha-1 Looked Complete And FAQ015 Update

## Objective

ユーザからの
「それらが終わっていないのに α-1 までのフェーズが完了扱いになっていた理由は何か。計画が雑だったのか、目標設定とゴールイメージの共有不足だったのか」
という問いに対し、current docs / roadmap / snapshot wording を見直して、理由を定義・運用・共有不足の 3 軸で説明し、`tmp_faq/faq_015.md` に累積メモを追記する。

## Scope and assumptions

- scope は explanation / wording review / FAQ update に限る。
- 新規実装、仕様変更、phase relabel、progress reorganization は行わない。
- current docs に書かれている定義と運用から reason を説明する。

## Start state / dirty state

- branch: `docs/layered-repro-guide-001`
- start dirty state:
  - untracked: `docs/reports/1177-layered-repro-guide-001-readonly-repro-audit.md`
  - untracked: `docs/reports/2027-mir-bottom-layer-readonly-explanation-001.md`
- これらは今回の変更対象に含めない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `tmp_faq/faq_015.md`

## Actions taken

- `README.md` / `Documentation.md` / `progress.md` / `tasks.md` を再読した。
- `specs/18` と `plan/44` から practical alpha-1 の official definition と package slicing を再確認した。
- reason を
  1. metric/label semantics
  2. package slicing strategy
  3. user lower-bound mismatch
  の 3 軸で整理した。
- その summary を `tmp_faq/faq_015.md` に追記した。

## Files changed

- `tmp_faq/faq_015.md`
- `docs/reports/2034-why-alpha1-looked-complete-and-faq015-update.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,260p' tasks.md
sed -n '1,320p' specs/18-practical-alpha1-scope.md
sed -n '1,320p' plan/44-practical-alpha1-roadmap.md
```

## Evidence / outputs / test results

- `README.md`
  - repo は final public product 未完と明示している。
  - practical alpha-1 line も non-final floors と exact non-claim を前提に書かれている。
- `Documentation.md`
  - `repo-local alpha-ready current layer`、`current-scope evidence closeout`、`practical alpha-1 readiness`、`final public product` の 4 つを混同しないよう書かれている。
  - ただし reader が `100%` を capability-complete と誤読する余地は残る。
- `progress.md`
  - Stage A..F の `100%` は evidence closeout、`PA1-*` の `100%` は practical readiness 側の stage closeout と明記している。
  - それでも `PA1-1..7 100%` は強く進捗した印象を与える。
- `specs/18-practical-alpha1-scope.md`
  - practical alpha-1 は final public product ではなく library-first toolchain。
  - 各 `P-A1-*` は first floor / non-final practical floor として定義されている。
  - same-session runtime、final public API/ABI、distributed durable save/load などは close condition に入っていない。
- `plan/44-practical-alpha1-roadmap.md`
  - package slicing は narrow ratchet strategy で意図的。
  - product-preview や devtools も thin exact-evidence bundle として閉じており、operational completion ではない。

## What changed in understanding

- 問題は「完全に雑な計画」よりも、「narrow floor closeout を 100% と書く運用」と「user lower-bound の閉じ条件不足」の組み合わせだと整理できた。
- phase / package slicing 自体はかなり deliberate だが、名前と `100%` 表記は optimistic reading を誘発しやすい。
- user が自然に期待する alpha-1 lower bound と、specs/18 の formal close condition の間に gap がある。

## Open questions

- practical alpha-1 definition に `user lower-bound unmet list` を明示的に持たせるべきか。
- `PA1-* 100%` を今後 `first-floor closed` のような wording へ寄せるべきか。
- same-session runtime / typed external direct execution を next capability milestone に昇格するか。

## Suggested next prompt

`このズレを踏まえて、alpha-1 の capability-based milestone を artifact-based milestone から切り直してください。`

## Plan update status

- `plan/` 更新不要。
- 今回は原因分析のみで repository-memory decision は更新していない。

## Documentation.md update status

- `Documentation.md` 更新不要。
- wording review のみであり、新 actualization や stale reference repair はない。

## progress.md update status

- `progress.md` 更新不要。
- current progress snapshot の再読のみで、新 package close はない。

## tasks.md update status

- `tasks.md` 更新不要。
- current task map を変える新 blocker decision はない。

## samples_progress.md update status

- `samples_progress.md` 更新不要。
- sample progress actual change はない。

## Reviewer findings and follow-up

- reviewer / sub-agent は未使用。
- local doc reasoning task として閉じた。

## Skipped validations and reasons

- runtime / transport / save-load rerun
  - 今回は wording と milestone semantics の explanation task であり、新しい success claim を追加しないため未再実行。
- `check_source_hierarchy.py` / `validate_docs.py`
  - report 作成後に実行予定。

## Commit / push status

- committed and pushed:
  - `2be27cd21e3bca71bdd21c10ed200df0b374a56f`
  - message: `docs: explain alpha1 completion mismatch`
- current file may receive a small follow-up status-only commit if later push metadata needs explicit mirroring.

## Sub-agent session close status

- sub-agent session なし。
