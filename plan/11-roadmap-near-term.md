# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
ここに書く step 数や task 数は厳密な約束ではなく、**rough estimate** である。

current immediate execution order は `plan/17-research-phases-and-autonomy-gates.md` と `progress.md` の phase section を優先する。
この文書は snapshot として、**current promoted line / checkpoint close / next reopen point** を短く保つ。

## current reading

- Phase 0 / 1 / 2 は maintenance tail である。
- Phase 3 は reserve path であり、later pressure が出たときだけ reopen する。
- Phase 4 は `specs/examples/121...125` までで current package close である。
- Phase 5 は `specs/examples/126...272` までで current package close である。theorem-line retained bridge は `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` を stop line にし、checker-side では minimal checker-cluster row core、fixture evidence attachment、optional `typed_reason_family_hint` attachment、`family_refs[]` minimal bundle、lightweight `coverage_state` まで source-backed に切り、`supported kind` summary は current matrix に足さない。actual checker payload family は `payload_family_kind + source_refs`、checker payload row family は `payload_family_ref + row_family_kind`、checker payload row detail は `payload_row_family_ref + row_source_ref + row_reason_kind`、checker payload row body は `row_body` variant-local slot bundle、checker payload supported kind summary は `payload_row_family_ref + supported_kind_scope + supported_kind_refs` まで docs-first に切れている。**next promoted line は `minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison`** である。
- shared-space finalization と final parser / public checker boundary は repo 全体では強い open question だが、current promoted line の直前 blocker ではない。

## いまから数 task の主眼

近い数 task の目的は、Phase 0 / 1 / 2 の closeout baseline を壊さずに、Phase 5 の current promoted line を theorem-line の先ではなく small decidable core / checker cluster 側で narrow に進めることである。

- first checker cut の 6 cluster を docs-only matrix としてどこまで stable に見せ、row core と evidence attachment をどこで切るかを比較する
- actual public checker API / final type system を早く既成事実化しない
- Phase 4 / Phase 2 / Phase 3 の checkpoint line を drift させない

## 次に自走で進める順番

### 1. minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison を Phase 5 の current promoted line として扱う

- theorem-line retained bridge は `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` で止め、low-level memory-order family は still later candidate に残す
- そのうえで、first checker cut の 6 cluster を docs-only checker-cluster matrix として見せる line は `coverage_state` で止め、actual checker payload family を `payload_family_kind + source_refs`、checker payload row family を `payload_family_ref + row_family_kind`、checker payload row detail を `payload_row_family_ref + row_source_ref + row_reason_kind`、checker payload row body を `row_body` variant-local slot bundle、checker payload supported kind summary を `payload_row_family_ref + supported_kind_scope + supported_kind_refs` まで切った current package を前提に、public checker payload schema をどこで docs-first に開くのが最小かを narrow に整理する
- rough weight: 中
- rough 所要: 1〜3 task / 2〜5日

### 2. cross-phase checkpoint maintenance を並走する

- `Documentation.md`、`plan/`、`progress.md`、`tasks.md`、research abstract の drift を抑える
- checkpoint close 済み package の current promoted line が読める状態を保つ
- rough weight: 低〜中
- rough 所要: 0.5〜1日 / checkpoint ごと

### 3. Phase 4 current package は checkpoint close として維持する

- authoritative room baseline、working subset row、minimal witness core、delegated-provider practical cut、control-plane threshold comparison は `specs/examples/121...125` までで current package close とみなしてよい
- stronger split や final catalog は、authority handoff / provider binding / activation frontier の concrete pressure が出たときだけ reopen 候補に残す
- rough weight: 低〜中
- rough 所要: 0〜1 task / drift 時のみ

### 4. detached validation loop は maintenance mode に戻す

- current self-driven friction reduction は checkpoint close とみなし、`reference update / bless` は final path policy / retention policy と接続する later candidate に残す
- rough weight: 低
- rough 所要: 0〜1 task

### 5. Phase 3 reserve path は later pressure が出たときだけ reopen する

- actual parser / public checker pressure
- Phase 5 inventory からの強い接続 need
- runtime / proof mirror need の増加

が出たときだけ reopen する。

## rough step estimate

| 目標 | rough step estimate | 注記 |
|---|---|---|
| minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison | 1〜3 task | current promoted line |
| cross-phase checkpoint maintenance | checkpoint ごとに 0.5〜1日 | mirror drift suppression |
| Phase 4 current package の checkpoint maintenance | 0〜1 task | current package close 維持 |
| detached validation loop residual maintenance | 0〜1 task | policy-dependent residual のみ |
| Phase 3 reserve path reopen 条件整理 | 0〜2 task | later pressure 時のみ |

## いま見えている later blocker / open question

current Phase 5 promoted line と checkpoint maintenance は引き続き self-driven に進めてよく、次の論点は後段 finalization の判断材料として保持する。

### 1. shared-space finalization

- activation family、authority placement、consistency catalog、fairness / RNG trust model、control-plane stronger split はまだ finalization しない
- current recommendation は `tasks.md` と `plan/12-open-problems-and-risks.md` を優先する

### 2. final parser grammar / public checker boundary

- companion notation は維持しつつ、minimal parser subset だけ later pressure 時に narrow に public 化する lineを保つ

### 3. actual type / theorem prover / protocol verifier actualization

- current lineは inventory と theorem-side retained bridge comparison に留める
- richer notebook payload / second consumer pressure finalization はまだ後段である

## いまの bottleneck

- `fixture authoring / elaboration` の独立 bottleneck
- shared-space final catalog を早く固定しすぎないこと
- parser / checker public boundary を premature に開かないこと
- theorem-line retained bridge を actual API と混同しないこと

## 近い将来の sequencing

1. Phase 5 later reopen の promoted line を進める
2. 同じ task の中で mirror drift を抑える
3. Phase 4 current package は checkpoint close を維持する
4. Phase 2 residual は practical need が出たときだけ扱う
5. Phase 3 reserve path は later pressure が出るまで reopen しない

## 今の working assumption

- current L2 semantics は大きく動かさない
- parser-free PoC は継続利用する
- detached validation loop は non-production helper として維持する
- shared-space の current working subset は final catalog ではない
- theorem-line retained bridge は symbolic threshold comparison を基調に扱い、minimal actual external contract field と actual exported / consumer-specific contract finalization を混同しない
