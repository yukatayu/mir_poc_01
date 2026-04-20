# 二大問題 representative sample bundle

この directory は、二大問題それぞれの representative sample を
**簡潔な日本語 guide 付きで辿るための sample bundle 入口** である。

## 置き方

- ここに置く文書は final public tutorial ではない。
- 既存の corrected prototype / Lean artifact / parser companion / helper command を再配置せず、
  どこから見ればよいかを 1 本の guide にまとめる。
- final public parser / checker / runtime API、final public verifier contract、exhaustive shared-space catalog はここで fixed しない。

## bundles

- [Problem 1 typed / theorem / model-check](./problem1-typed-theorem-model-check.md)
  - representative sample `p06`
  - supporting sample `p10 / p11 / p12 / p15 / p16`
- [Problem 2 order / handoff / shared-space](./problem2-order-handoff-shared-space.md)
  - representative pair `p07 / p08`
  - witness strengthening contrast `p05`
  - reserve route `p09`
  - negative pair `p13 / p14`

## current recommendation

- まず各 bundle doc を読む。
- 各 bundle doc の冒頭にある `最短 quickstart` を使うと、
  `smoke` → `matrix` → `bundle` → parser companion inspector の 4 段で representative sample の見方を確認できる。
- helper から同じ導線を見たいときは
  `python3 scripts/current_l2_guided_samples.py quickstart problem1`
  または
  `python3 scripts/current_l2_guided_samples.py quickstart problem2`
  を使う。
- Problem 1 の theorem-first emitted-artifact loop を repo-local output dir に materialize したいときは
  `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
  を使う。
- Problem 2 の authoritative-room runnable scenario loop を repo-local output dir に materialize したいときは
  `python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
  を使う。
- Problem 2 の `auditable_authority_witness` reserve package を単独で materialize したいときは
  `python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness`
  を使う。
- Problem 2 の `delegated_rng_service` reserve package を単独で materialize したいときは
  `python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service`
  を使う。
- doc 側と helper 側の quickstart が揃っているかを narrow に確認したいときは
  `python3 scripts/current_l2_guided_samples.py quickstart-parity`
  を使う。
- representative sample floor を踏まえて、どの mixed gate をどの command から再開するかを
  まとめて見たいときは
  `python3 scripts/current_l2_guided_samples.py reopen-map`
  を使う。
- split closeout 後の remaining mixed gate と true user-spec residual を
  1 枚で圧縮して見たいときは
  `python3 scripts/current_l2_guided_samples.py residuals`
  を使う。
- executable loop 群を踏まえた current first line / mixed-gate lane / true user-spec residual / next self-driven queue を
  1 枚で見たいときは
  `python3 scripts/current_l2_guided_samples.py closeout`
  を使う。
- theorem-first external pilot / `auditable_authority_witness` / `delegated_rng_service` / model-check second-line reserve を、
  closeout 後の next reopen line として見たいときは
  `python3 scripts/current_l2_guided_samples.py reserve`
  を使う。
- parser companion surface / parser-side tranche / final parser-checker-runtime API residual を、
  mixed gate lane として見たいときは
  `python3 scripts/current_l2_guided_samples.py lane parser-side-residual`
  を使う。
- true user-spec residual を explicit hold line として見たいときは
  `python3 scripts/current_l2_guided_samples.py hold-line`
  を使う。
- syntax / modality の final marker lane だけを個別に見直したいときは
  `python3 scripts/current_l2_guided_samples.py lane syntax-modality-final-marker`
  を使う。
- 次に guide 内の `run-source-sample` / `bundle problem1|problem2` / `mapping` command を順に使う。
- 問題 1 / 問題 2 をまとめて手早く確認したいときは
  `python3 scripts/current_l2_guided_samples.py smoke-all`
  を使う。
- `smoke-all` は失敗時に `failed step` / 終了コード / 出力断片を compact に返し、
  command 自体も非ゼロ終了で止まるので、repo-local regression の入口として使いやすい。
- deeper theory や final public contract ではなく、current first line がどの sample で machine-check されているかを確かめる入口として使う。
