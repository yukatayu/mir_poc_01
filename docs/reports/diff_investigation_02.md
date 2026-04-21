# Report diff_investigation_02 — 旧資料 `sync_v3` と現行 repo 方針の差分調査

- Date: 2026-04-21
- Author / agent: Codex
- Scope: `旧資料_参考_ChatGPT_03_sync_v3/` の全 17 文書と、現行 repo の current status / 境界文書との比較調査
- Decision levels touched: 主に観察と比較。規範変更なし

## 1. Objective

`旧資料_参考_ChatGPT_03_sync_v3/` を読み、現行 repo の設計方針・到達点・後回し境界と比較したうえで、

- 何が大きく変わったのか
- どちらの整理の方が現在の repo には適しているのか
- 今から取り入れる価値があるもの、逆に今は取り込まない方がよいものは何か

を、現行 repo の整合性を最優先に日本語で整理する。

## 2. Scope and assumptions

- 本調査は **比較と考察のみ** を目的とする。
- 現行 code / `plan/` / `progress.md` / `tasks.md` / `specs/` の規範判断は変更しない。
- 旧資料は **historical / comparison material** として読む。現行採択事項として扱わない。
- 判断の基準は、現行 repo が明示している次の線に置く。
  - Mir current-L2 が主眼であること
  - Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform を分離可能に保つこと
  - `repo-local near-end` と `final public contract` を混同しないこと
  - parser-free PoC / helper-local preview / reserve line と public adoption を混同しないこと

## 3. Documents consulted

### 現行 repo 側

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/02-system-overview-and-positioning.md`
- `plan/03-decision-strengths-and-boundaries.md`
- `plan/06-surface-notation-status.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

### 旧資料側

- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_00.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_01.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_02.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_03.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_04.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_05.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_06.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_07.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_08.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_09.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_10.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_11.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_12.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_13.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_14.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_15.md`
- `旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_16.md`

## 4. Actions taken

1. AGENTS.md の読書順に従って、現行 repo の current status / decision boundary / invariant を再確認した。
2. 旧資料ディレクトリの全ファイルを列挙し、章構成と文量を把握した。
3. 旧資料を前半・後半に分けて sub-agent に並列読解させた。
4. 現行 repo 側の比較軸も別 sub-agent で整理した。
5. 手元でも代表章を直接精読し、特に次の差分を重点的に確認した。
   - monolithic vision と separable architecture の差
   - settled surface syntax と companion notation の差
   - `atomic_cut` / `durable_cut` / `barrier` の意味の差
   - theorem/model-check/tool binding の扱いの差
   - portal / multi-server / Live-State Web を今どこに置くかの差
6. 差分を「今の方が良い整理」「後で再評価する価値がある論点」「今は戻さない方がよい論点」に分類した。

補足:

- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要

## 5. Files changed

- 追加:
  - `docs/reports/diff_investigation_02.md`

## 6. Commands run and output excerpts

主要な確認コマンドと出力抜粋のみ記す。

```bash
$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Task baseline recorded.

$ git status --short
(no output)

$ find 旧資料_参考_ChatGPT_03_sync_v3 -maxdepth 2 -type f | sort
旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_00.md
...
旧資料_参考_ChatGPT_03_sync_v3/sync_lang_v3_16.md

$ wc -l 旧資料_参考_ChatGPT_03_sync_v3/*.md
...
3065 total
```

比較用に、現行 docs / plan / specs と旧資料の代表章を `sed` と `rg` で読んだ。
また、旧資料前半・後半、および現行比較軸整理について explorer sub-agent を起動して読解結果を回収した。

## 7. Evidence / outputs / test results

### 7.1 総評

最も大きい差は、**旧資料は「言語・ランタイム・ポータル・標準化・次世代 WWW の製品像」をかなり一体で語っている**のに対し、**現行 repo は「Mir current-L2 の意味論核と representative evidence をまず固める」ことに意図的に主眼を絞っている**点である。

この差は単なる文体差ではない。現行 repo は次を明示している。

- 4 系統は separable である
- current state は `repo-local near-end` であって final public completion ではない
- helper-local preview / reserve / parser-free PoC と public adoption を混ぜない
- `atomic_cut` や order family を過大説明しない

この点で、**現行の整理の方が、いまの repo の実装証跡と規範文書に整合している**。

### 7.2 旧資料の強み

旧資料には、今でも価値のある強みがある。

1. 人に伝わる大きな絵が強い。
   - 「通信 = 構文」「進化しながら壊れない分散 Web」「歩ける分散宇宙」など、外向き説明の軸が明快である。
2. 何を最終的に揃えたいかが具体的である。
   - trust tier、audit、portal、runtime、build/deploy、IDE、標準化、レジストリ、鍵失効、UX まで先回りしている。
3. 懸念点も比較的正直である。
   - SMT timeout、state-space 爆発、rank-N 注釈負荷、鍵管理、ラベル衝突などは、今読んでも有効な checklist である。

したがって旧資料は、**今の repo にそのまま戻す文書**ではなく、**将来の後段 workstream を考えるときの requirement mine** として読むのが適切である。

### 7.3 現行 repo の方が明らかに良い点

#### A. スコープの絞り方

旧資料:

- Mir / Mirrorea / portal / runtime / compiler / trust /標準化 /上位世界像が、かなり一続きに語られている。

現行:

- Mir current-L2 を主眼に据え、Mirrorea / Prism / Typed-Effects / shared-space を分離可能な track として扱う。

評価:

- **現行の方が良い。**
- 理由は、今の repo で実際に動いているものは current-L2 representative sample、Lean foundation、helper-local verification floor であり、旧資料の monolithic product vision を正本扱いすると「できていること」と「まだ未来のこと」が再び混ざるからである。

#### B. 表層構文へのコミットの仕方

旧資料:

- EBNF や reserved-word 群がかなり揃っており、`cache`、`barrier`、`watch`、`atomic_cut`、`durable_cut`、`portal`、`phase/await` まで、ほぼ language manual として読める。

現行:

- current-L2 は companion notation を維持しつつ、final parser grammar を deliberate に後ろへ置く。
- `perform`、explicit edge-row family、`require` / `ensure` などは semantic cluster として整理し、lexical freeze を避けている。

評価:

- **現行の方が良い。**
- 理由は、いま必要なのは visual polish の確定ではなく、semantic honesty と lowering friendliness の維持だからである。
- 旧資料のように表面を先に固めると、parser / checker / runtime / docs が drift したときに「表層だけ決まっていて中身が未確定」という悪い既成事実が生まれやすい。

#### C. `atomic_cut` と ordering の説明

旧資料:

- `atomic_cut` はしばしば多ノード線形化や handoff の核心として強く説明される。
- `durable_cut` や `barrier` もかなり settled な primitive として扱われる。

現行:

- `atomic_cut` は place-local rollback frontier を確定する最小 nucleus に留める。
- distributed finalization、durable family、higher-level ordering、low-level `memory_order` exact surface は後段へ送る。

評価:

- **現行の方がかなり良い。**
- 理由は、意味の上で言い過ぎないからである。
- 現行 invariant の「cut は明示的 decision boundary」「hidden rollback を持ち込まない」「scope を過大説明しない」と整合している。

#### D. 形式検証とツール連携の扱い

旧資料:

- Z3 / Coq / TLA+ / LLVM / Wasmtime / seccomp / TrustMgr までが、一本の必然的 stack として描かれる。
- ときに「ソース = 証明 = バイナリ」のような強い接続が前面に出る。

現行:

- Lean-first ではあるが、tool brand は fixed しない。
- theorem/model-check bridge は notebook-first、row-local、helper-local preview / threshold / reserve として段階化する。
- final public verifier contract は still later と明示する。

評価:

- **現行の方が良い。**
- 理由は、現在ある証跡に合わせて claim を細くしているからである。
- 旧資料の物語は魅力的だが、今の repo でそれを正本化すると、proof stub acceptance と completed discharge、preview artifact と public contract を混同しやすい。

#### E. safe evolution / ownership / lifetime invariant の明示

旧資料:

- `fallback`、`lease`、`gc_epoch`、`patch add/remove` を強く打ち出しており、「壊れても止まらず進化する」という方向はかなり明快である。
- 一方で、現行 repo が L0/L1 級の invariant として明示している
  - downstream addition を既定とすること
  - API shadowing を禁止すること
  - linear resource を duplication しないこと
  - lifetime degradation を monotone に保つこと
 については、旧資料では部分的には示唆されるが、現行ほど厳密に境界化されていない。

現行:

- `specs/01` と `specs/09` が、safe evolution の原則を architecture-wide invariant として明示している。
- 特に「upstream rewiring を既定にしない」「silent shadowing を許さない」「rollback と cut の境界を曖昧にしない」は、旧資料よりはるかに強く書き分けられている。

評価:

- **この点も現行の方が良い。**
- 理由は、進化機構を魅力的な feature として語るだけでなく、何をしてはいけないかまで先に固定しているからである。
- 旧資料の `patch + atomic_cut + lease GC` という統合的な絵は分かりやすいが、現行 repo の方が「安全な進化」の禁止事項と monotonicity を明示しており、後段での設計 drift に強い。

### 7.4 旧資料から今も拾う価値があるもの

以下は、**現行 repo の整合性を壊さずに「忘れない方がよい論点」**である。

#### 1. 人間向け説明原理

- 「通信 = 構文木のネスト」
- DAG discipline
- fail closed
- no hidden global state

これは final syntax として戻す必要はないが、**なぜこの project が ad-hoc API 群ではなく意味論を重視するのか**を説明する原理として今でも有効である。

#### 2. strong typing の設計態度

- full dependent core に一気に行かず、決定可能 fragment を厚くする
- IFC / trust / cost / lease を separate concern として扱う
- 注釈負荷と decidability の trade-off を正面から扱う

これは現行 `plan/18` の finite decidable fragment、first strong typing layer、Lean-first proof route と相性が良い。

#### 3. 後段 subsystem の requirement inventory

旧資料の次は、いまでも後段 track の要求棚卸しとして価値がある。

- portal / multi-server の具体的要求
- trust no-upcast / tamper-evident audit / append-only log
- key rotation / revocation / label registry
- IDE / debugger / observability UX
- benchmark / scalability / state explosion 対策

ただし、これらは **今すぐ current-L2 core に戻す項目ではない**。
Mirrorea / Typed-Effect / operational boundary / future packaging 側の reserve inventory として読むのが自然である。

補足:

- `handoff / order / authoritative-room` そのものは、現行 repo でも Problem 2 の **repo-local current line** としてすでに存在する。
- later に残っているのは、portal / multi-server の大域像、final wording、final public witness/provider contract、low-level `memory_order` exact surface である。

#### 4. ユースケース拡張カタログ

`sync_lang_v3_13.md` のような catalog は、現行 repo の twin peaks をぼかす形で正本化するのは避けたいが、次の用途には使える。

- future acceptance corpus の候補集
- 「何を eventually 説明できるようになりたいか」の要求一覧
- subsystem ごとの非機能要件の早見表

### 7.5 今は取り込まない方がよいもの

#### 1. monolithic product narrative

- Live-State Web
- portal standardization
- WASI / W3C / IETF まで含む一本道の roadmap

これらを今の main docs に戻すと、Mir current-L2 の現況を再び曖昧にする。

#### 2. settled language manual 的な EBNF

旧資料の EBNF は読みやすいが、現行 repo では final grammar が intentionally open である。
したがって、今これを「現行仕様」のように再導入するのは良くない。

#### 3. distributed / durable semantics の先取り

`atomic_cut` を global consistent cut や multi-node linearization の shorthand のように語る旧資料の癖は、現行 invariant と衝突しやすい。
この点は戻さない方がよい。

#### 4. concrete tool / backend / runtime brand の早期固定

現行 repo の良さは、brand-neutral / boundary-first / replaceable-layer を守っている点にある。
旧資料の具体 stack は参考にはなるが、今の規範 line に持ち込むべきではない。

### 7.6 「どちらが良いか」の判断

結論として、**現在の repo を進めるうえでは現行方針の方が良い**。

理由は次の 4 点である。

1. 実際に repo で再確認できる evidence に沿っている。
2. subsystem separation を守るので、後段 workstream の premature collapse を防げる。
3. helper-local preview / reserve / public contract の境界が明確で、過大主張を避けられる。
4. current-L2 の twin peaks を終盤まで押し切るために、論点を細く保てる。

一方で、**旧資料の方が良い点は「大きな絵を伝える力」と「将来必要になる operational checklist を早く見通している点」**である。
したがって、最も良い読み方は次のようになる。

- 規範判断の正本: `specs/`
- current snapshot: `Documentation.md`
- long-lived repository memory: `plan/`
- 旧資料の役割: future-facing comparison material / requirement inventory / external explanation hint

この二層読みが最も自然である。

## 8. What changed in understanding

今回の比較で特に明確になったのは、旧資料と現行 repo の差は「熱量」や「抽象度」の差ではなく、**設計責務の切り方そのものの差**だという点である。

以前は、

- 何を作りたいか
- そのためにどんな言語・ランタイム・ポータル・trust・監査・標準化が必要か

が 1 本のストーリーとして強く結ばれていた。

現行 repo では、それを次のように切り分け直している。

- まず Mir current-L2 の semantics / sample / proof boundary を固める
- Mirrorea / shared-space / operational runtime policy は separable に保つ
- helper-local 実装は evidence として使うが、public adoption とは言わない

この切り分けによって、現行 repo は以前よりもはるかに **正確で、巻き戻りに強い**。
同時に、旧資料が持っていた「遠景の要求」を忘れやすくなる危険もある。

したがって今後は、

- 規範線は現行のまま維持する
- ただし旧資料が先回りしていた後段要求は、比較資料として折に触れて参照する

という運用が最も良いと理解した。

## 9. Open questions

- 旧資料由来の requirement inventory を、将来どの層に再配置するのが最も自然か。
  - Mirrorea reserve
  - Typed-Effect Wiring Platform reserve
  - external positioning document
  - acceptance corpus backlog
- old `portal / multi-server / Live-State Web` narrative を、将来どの時点で public-facing explanation として再導入するのがよいか。
- old `trust / audit / registry / key rotation` 群を、Mir core ではなく operational boundary 側へどう切り戻して整理するのがよいか。
- 旧資料のユースケース catalog を、将来 `specs/examples/` や `docs/research_abstract/` のどこまで mirror すべきか。

## 10. Suggested next prompt

`旧資料 sync_v3` で今も価値があると判断した論点だけを抜き出して、現行 repo の層構造に合わせて「Mir core」「Mirrorea reserve」「Typed-Effect operational reserve」「外向き説明用」の 4 分類に整理した補助メモを、新規 report として作ってください。現行規範文書は変更せず、再配置案だけを丁寧に書いてください。
