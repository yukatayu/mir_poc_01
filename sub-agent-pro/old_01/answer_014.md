2026-04-13時点の文書に基づくと、自然な収束形は「最初から parser / checker / runtime を対等な 3 本柱の public front door にする」ことでも、「compile-ready tranche を丸ごと 1 枚の統一 public surface にする」ことでもありません。いちばん自然なのは、**library-side の canonical contract を先に固め、その上に operational CLI を second gate として載せ、parser / checker / runtime の個別 surface は expert-facing の second line として後から絞って出す**という段階化です。current repo 自体も、already-public parser-free stack、crate-public but non-production tranche、repo-local helper surface の 3 bucket split を置き、first later cut を `run_current_l2_source_sample` / `CurrentL2SourceSampleRunReport` の runtime-led thin facade、later-support cut を `run_current_l2_runtime_skeleton` / `CurrentL2RuntimeSkeletonReport`、CLI をその上の Rust-side operational wrapper として読んでいます。    

## 提案する final public surface の形

私の提案は、**「推奨入口は統一、低レベル surface は分離」**です。
つまり、外から見せる正規の入口は 1 本に寄せる。一方で、物理的な crate / module 境界は `mir-ast` / `mir-semantics` / `mir-runtime` のように分けたまま保つ。これは、この repo 全体が subsystem と境界を明示的に分け、implicit coupling より explicit schema / versioned interface を優先する設計だからです。したがって、「public surface を統一するか、crate を分けるか」は二者択一ではなく、**top-level contract は統一、implementation/package boundary は分離**が自然です。   

概念的には、最終形は次のような 2 層にすると収まりがよいです。

```rust
// canonical public entry
run_source(source_input, run_options) -> SourceRunReport

// expert-facing later surfaces
run_program(program, runtime_host) -> RuntimeReport
check_program(program, checker_options) -> CheckerReport
parse_source(source_text, parser_options) -> ParseReport
```

ここで重要なのは、**最初に stable にするのは `run_source(...)` 型の runtime-led entry だけ**で、`run_program` / `check_program` / `parse_source` は second line として後から昇格させることです。current docs でも `run_current_l2_source_sample` が first candidate、`run_current_l2_runtime_skeleton` は support-only / later-support、public checker line も minimal API → command surface → shared output contract → parser-front boundary という docs-first progression を取っており、checker を runtime facade に最初から溶かす読みではありません。runtime を first、checker を second、parser を last にするのがいちばん無理がありません。    

## staged packaging strategy

### Stage A — いま維持すべき形

現在の 3 bucket split は崩さない方がよいです。already-public parser-free helper stack はそのまま stable bucket として維持し、source-facing な public pressure は `run_current_l2_source_sample` 側にだけ narrow にかける。`run_current_l2_runtime_skeleton`、`lower_current_l2_fixed_source_text`、`mir_ast::current_l2`、`run_program_to_completion`、`FixtureHostStub::run_program` は support-only tranche に留め、example/support modules や Python orchestration は repo-local helper surface のまま残す。この切り方を崩すと、`pub` visibility や test support が hidden promotion で final contract に化けやすいです。  

### Stage B — first real public library contract

次に固めるべきなのは、installed binary ではなく **library-side final public contract** です。ここでは `run_current_l2_source_sample` 型の runtime-led facade を一般化し、repo layout や accepted sample set に依存しない source-facing entry を stable にするのが自然です。報告型も `CurrentL2SourceSampleRunReport` の現在形をそのまま凍らせるのではなく、top-level の安定 report を作り、その下に lowered / checker / runtime の nested detail をぶら下げる方がよいです。current docs でも nested public report carrier は読めるが standalone public entry promotion はまだ行わない、という cut が取られています。つまり、**report は階層化してよいが、stable contract は top-level summary に限定する**のがよいです。  

### Stage C — expert-facing parser / checker / runtime surfaces

その後に expert-facing surface を切り出します。ここでは 3 つを対等に同時公開する必要はありません。順番は、`run_program` 相当の runtime skeleton surface、次に checker family、最後に parser front が自然です。checker line は docs 上でも minimal public checker API から command surface、shared output contract、public checker boundary、verifier handoff surfaceへと独立に段階化されており、runtime facade と一体で先に固定する設計ではありません。parser も stage 1 + stage 2 structural floor までの accepted cluster を narrow に保つ current line が明示されており、final parser grammar や generic shared entry は still later です。したがって、**final には別 module / crate を持ってよいが、昇格順は runtime → checker → parser**が自然です。   

### Stage D — installed binary / final CLI contract

CLI は second gate のまま扱うべきです。つまり installed binary は library-side contract の上に載る operational wrapper であり、library contract が固まる前に CLI を final host contract とみなすべきではありません。current docs でも later ordering は library first / CLI second で固定され、current shell concern は `mir-current-l2 run-source-sample <sample> --host-plan <path> --format pretty|json` に narrow に留めています。installed binary へ昇格させるのは、少なくとも library-side contract が source sample 固有の path / accepted-set hard-coding から切り離され、docs-first の host-facing I/O / adapter boundary がもう一段具体化した後が自然です。    

### Stage E — artifact/exporter/host integration

artifact export、detached compare、formal-hook emitter、proof/model-check fan-out、repo-local bless/regression は、core binary の default contract に入れず separate gate に残すべきです。現状でも emitters / support modules / detached loop scripts は repo-local operational aid として扱われ、actual exporter API や aggregate export API は OPEN のままです。したがって、artifact output は final `run` command の基本責務ではなく、将来の separate subcommand か別 binary に切る方が自然です。  

## 質問ごとの答え

### 1. 統一 surface を最初から目指すべきか、別 public crate / module にするべきか

**統一すべきなのは「推奨入口」であって、「低レベル API の形」ではありません。**
最初から parser / checker / runtime を対等な public crate / module として前面に出すのは早いです。いま natural なのは、top-level では runtime-led の canonical entry を 1 本だけ立て、その背後に parser / checker / runtime の expert-facing surfaces を別 module / crate として後から切り出す形です。つまり、**front door は統一、expert door は分離**です。   

### 2. public operational CLI はいつ昇格させるべきか

昇格は **library-side final contract の後**です。
具体的には、`run_current_l2_source_sample` 系の facade が repo layout / accepted-set hard-coding / current `FixtureHostPlan` coupling から一段抽象化され、`run_current_l2_runtime_skeleton` 側の later-support cut も安定し、host-facing I/O / adapter boundary が raw `stdin/stdout` ではなく capability-scoped port / adapter boundary として整理された時点が自然です。今の current-L2 scoped concrete shell は良い actual shell concern ですが、installed binary / final host contract にそのまま昇格させるのはまだ早いです。    

### 3. JSON / pretty / artifact output / host-plan input は public contract にどう入れるのが自然か

ここは **同じ stability class に入れない**方がよいです。

* pretty は human-facing renderer に留める。stable contract の中心に置かない。
* JSON は CLI 側の machine-facing public contract 候補にしてよいが、**top-level summary を versioned に固定**し、nested internal carrier 全体をそのまま凍らせない方がよい。
* artifact output は別 gate。default `run` の戻り値や標準 JSON と混ぜず、separate subcommand / separate emitter に分けるのが自然。
* `--host-plan` は explicit advanced input mode として残してよいが、**final host contract の primary form にしない**方がよい。current docs でも `explicit_host_plan_input_mode` は shell concern だが、explicit `FixtureHostPlan` coupling を final contract のまま固定しない guard が明示されています。  

私なら final CLI contract は、`run` の標準出力を `pretty | json` に限定し、artifact は `emit-*` 系に分けます。`--host-plan` は `run --host-plan ...` の explicit expert mode に残し、将来は host provider / adapter contract が固まったらそちらを primary に寄せます。これは current narrow concern と later docs-first host-facing boundaryの両方に整合します。  

### 4. current runtime-led thin facade path は良い中間段階か

**はい。かなり良い中間段階です。**
理由は 3 つあります。第一に、parser carrier / semantic entry / source-backed execution を end-to-end で跨ぎつつも、support-only tranche と excluded bucket を保てる。第二に、CLI を library facade の wrapper として扱える。第三に、sample-facing theorem/model-check fan-out まで façade の public/report shape を変えずに進められている。これは「今 public にするもの」と「今は nested report / helper / repo-local に留めるもの」の分離に成功している、という意味でよい橋です。   

ただし注意点は 2 つです。
一つは、`run_current_l2_source_sample` をそのまま final source API とみなして sample path policy を凍らせないこと。もう一つは、`CurrentL2SourceSampleRunReport` の nested carriers を stable JSON schema そのものにしないことです。良い intermediate ですが、**final universal contract そのものではない**という読みを守る必要があります。 

## 今の repo で、まだ public にしない方がよいもの

次は、今の時点では final public surface に入れない方がよいです。

* `resolve_current_l2_source_sample_path` と hard-coded accepted sample set
* `lower_current_l2_fixed_source_text` と `mir_ast::current_l2` の partial parser carrier
* `static_gate_program_detailed`、`run_program_to_completion`、`FixtureHostStub::run_program` のような tranche-internal support
* `crates/mir-semantics/examples/*`、`examples/support/*`、`scripts/current_l2_source_sample_regression.py`、`scripts/current_l2_detached_loop.py` などの repo-local operational aid
* detached artifact exporter / aggregate exporter / compare helper の current schema と storage/path policy
* explicit `FixtureHostPlan` coupling を final host contract とみなすこと
* raw `stdin/stdout` を privileged primitive にした installed binary contract
* final parser grammar、public checker shared output contract、verifier handoff surface を current CLI と同時に確定すること。    

要するに、最終 public surface は **「1 本の source-facing runtime library contract」→「expert-facing parser/checker/runtime second line」→「installed CLI wrapper」→「artifact / host / verifier / adapter separate gates」** という順で収束させるのが自然です。今の thin facade / concrete shell の方向はこの収束形と整合しており、間違っているというより、**正しい順番で narrow に切れている**と見てよいです。  
