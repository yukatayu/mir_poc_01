# Phase 5 要約 — small decidable core と proof boundary

## 何をした phase か

Phase 5 は、
**current L2 / shared-space line で、何を local / structural / decidable core に残し、何を theorem prover / protocol verifier / runtime policy へ外出しするか**
を切る phase である。

ここでは強い型システムや public checker API をすぐに作るのではなく、
small decidable core の inventory と external handoff の cut を docs-first に整理した。

## current first choice

current package の first choice は次の 4-way split である。

- `core_static_checker`
- `theorem_prover_boundary`
- `protocol_verifier_boundary`
- `runtime_policy_boundary`

これにより、same-lineage floor や malformed / underdeclared rejection のような
local / structural / decidable なものだけを core に残し、
canonical normalization や witnessed draw protocol のような global property は外へ残す。

## `atomic_cut` の位置づけ

`atomic_cut` は current では
**place-local finalizing cut の最小核**
に留める。

つまり、

- local rollback frontier の structural floor

までは core checker / runtime representative に残すが、

- authority-serial ordering
- fairness witness
- replay obligation
- event-tree derived execution view

のような higher-level async control は別 boundary に残す。

## proof-obligation matrix

次段では、proof obligation 自体を docs 正本の matrix として見える化した。

代表的には次を分ける。

- fallback chain の canonical normalization / no re-promotion
- `try` / `atomic_cut` の rollback-cut non-interference
- authoritative room の witness / provider receipt consistency
- activation visibility の runtime control-plane policy

ここで重要なのは、
**current helper の row と proof obligation row は同じではない**
という点である。

## external handoff artifact の current cut

current cut では、external handoff artifact を actual API にせず、
**source evidence を参照する narrow row bundle sketch**
に留める。

つまり、

- detached static gate artifact
- `checked_reason_codes`
- shared-space witness bundle

は handoff row そのものではなく、
**proof obligation row が参照する source evidence**
に残す。

次段の threshold comparison では、
**mixed row bundle を current docs-only default に維持し、
boundary-specific handoff artifact と actual emitter は concrete consumer pressure が出たときだけ reopen する**
という cut まで固定した。

さらに current first practical candidate として、
**first concrete external consumer pressure は `theorem_prover_boundary` に置き、
`protocol_verifier_boundary` は second practical candidate、`runtime_policy_boundary` は later candidate**
と読むのが自然だと整理した。

その次段では、mixed row default を壊さずに、
**theorem line だけを `subject_kind + subject_ref + theorem_rows[]` の docs-only projection bundle として切る**
のが current first choice であると整理した。

さらに projection bundle の `evidence_refs` は、
actual path ではなく **`ref_kind + ref_id` の typed symbolic ref family**
へ整えるのが current first choice である。

ただし current phase では、これをまだ public checker API へは上げず、
**concrete theorem consumer pressure が出るまで docs-only bridge に留める**
のが current first choice である。

## まだやっていないこと

- public checker API の finalization
- theorem prover input schema の finalization
- protocol verifier input schema の finalization
- stable `evidence_refs` family をどこまで actual artifact ref に寄せるか
- concrete theorem consumer bridge に必要な minimum contract rows
- low-level memory-order family の導入

## この phase が次へ渡すもの

- small decidable core の current boundary
- global proof / protocol / runtime policy を別 line に残す cut
- proof-obligation matrix
- external handoff artifact の minimal sketch

これにより、Phase 6 以降で actual parser / checker / runtime を切るときも、
どこまでを core に ratchet してよいかを見失いにくくなる。
