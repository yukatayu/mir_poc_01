---
id: arch/06-project-product-layers
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, arch/01-strata, arch/02-boundary-contracts, arch/05-satellites, adr/ADR-0034, adr/ADR-0035, plan/01-phases]
summary: semantic strata・project/product responsibility layers・lifecycle phasesを分離する三軸mapとPL-0--PL-6の責任境界。
open_items: []
---

# 06 — Project/product responsibility layers and the three-axis map

## Canonical three axes

Mir / Mirrorea は次の三座標を別々に読む。番号が同じでも対応関係、成熟度、
owner、entry/exit を相互導出してはならない。対応は many-to-many である。

| Axis | Values | What it answers | Authority |
|---|---|---|---|
| semantic strata | `S0 Surface`, `S1 Core`, `S2 Trace`, `S3 Verify`, `S4 Projection`, `S5 Domain`, `S6 Host` | ある情報・意味・境界が何の責任か | `arch/01-strata` |
| project/product layers | `PL-0`--`PL-6` | project/product stackで誰が何を受け取り何を提供するか | この文書 |
| lifecycle phases | theory `T0`--`T2`, implementation `I1`--`I6` | 各責任をどこまで実現・受理したか | `plan/01-phases` |

`S6 Host` は current semantic axis の外界・実現境界であり、Host が domain
semantics を所有するという意味ではない。formal theory chapters が主に `S0`--`S5`
を扱うことは、`S6` が optional 又は別の lifecycle であることを意味しない。
旧 LAB の realization label `S0`--`S7` と `S7 Application` は current semantic
stratum ではない。

## Project/product responsibility map

Maturity はその PL の現在の evidence 状態を記すだけで、phase acceptance ではない。

| Layer | Responsibility | Admitted input | Produced output | Prohibited flow | Current maturity |
|---|---|---|---|---|---|
| **PL-0 Host / physical substrate** | OS、thread、process、network、storage、browser engine、renderer、GPU等の物理資源を提供する | typed deployment/resource request、adapterを通るbyte/device/provider request | process/byte/storage/frame/device result又はtyped host failure | address、process、session、certificate、device又はstorage identityからMir authority・Core・domain lawをmintしない | 外部基盤は存在するがMir/Mirrorea product acceptanceではない |
| **PL-1 Mir language and semantic kernel** | ordinary source、Core、ownership、authority、dependency、relation、effect/failure、lifetime、cut/patchの意味を検査・保持する | ordinary Mir sourceと明示されたpolicy/effect/resource intent | checked Core、verdict、semantic carrier requirement、obligation、source/provenance refs | transport、renderer、package、World等の都合をCore primitive又は意味の正本にしない | finite I1+ profileとaccepted I2の下位意味はevidence-backed。broad PHASE-I1は未受理 |
| **PL-2 Mirrorea distributed fabric** | checked meaningからper-locus artifact、generated communication、runtime、transport、将来durabilityを実現する | PL-1のchecked Core/artifact/communication planと、authorityを含まないdeployment binding | locus execution、typed cross-locus occurrence/failure、observer-safe fabric evidence | deployment、route、transport、sessionからedge、owner、grant、state、expected resultを発明せず、direct remote storeを作らない | in-process I2はofficial exit accepted。real transport I3はactive program内だがlifecycle未entry |
| **PL-3 Mir Browser / Host safe participant runtime** | third-party package admission、sandbox/resource limit、typed FFI/provider、View/input、local storage/consent境界を担う | package identity/provenance/content、requested capability/effect/resource、PL-2 participation contract | admitted又はrejected package instance、typed locus/grant request、observer-safe projection、typed command/effect/result/failure | package・browser・provider identityからauthorityをmintせず、untrusted packageをraw FFIへ接続せず、View/providerにauthoritative semanticsを渡さない | responsibility boundaryのみ。exact trust/resource contractはALIGN-2、実装はinactive I5 |
| **PL-4 Shared-Space / World-Web platform** | persistent・participatory・programmable・multiple-Viewなspace/object/relationを支える上位基盤の責任位置を保つ | PL-1のdomain-definable semantics、PL-2 fabric、PL-3 safe participation | 下位を再利用できる上位platform responsibilityと、将来のapplication向けpromise | World、URL、Portal等を下位primitiveにせず、address/link/discovery/publication/federation/governanceを本cutで選ばない | responsibility-only horizon。具体semantics/API/catalogは未設計 |
| **PL-5 Domain Kits and applications** | World、Room、Avatar、Bird、Portal、Game、Library、Page、Website等をMir library/applicationとして定義する | PL-1の一般semantic constructsと、必要に応じPL-2--4の提供境界 | domain law、kit、application、typed presentation/effect request | domain vocabularyをCoreへ昇格せず、個別applicationを下位completion条件にしない | sample/library evidenceのみ。canonical kit又はpublic productは未受理 |
| **PL-6 Reversed Library / knowledge-world project** | Reversed Libraryを別application/projectとして将来構成する | 将来owner programが採用する下位accepted interfaces | 独立したknowledge-world application evidence/product | Mirrorea、PL-4、I3、I4又はI5のcompletion条件、下位architecture、current queueにしない | separate inactive application horizon。具体product designはowner-reserved |

## Many-to-many cross-reference

| Capability cut | Semantic strata | Project/product layers | Lifecycle reading |
|---|---|---|---|
| ordinary sourceからchecked meaning | `S0`--`S3` | mainly `PL-1`; `PL-5/6` may author domain source | finite I1+ accepted; broad I1 unaccepted |
| per-locus artifactとgenerated communication | `S4`, with `S1--S3` obligations | `PL-1 -> PL-2` | official I2 exit accepted |
| real process/transport execution | `S4` realized over `S6` | `PL-2` over `PL-0` | bounded I3 program active; official I3 lifecycle unentered |
| package admission / View / provider | `S6` boundary consuming `S3--S5` facts | `PL-3` over `PL-0`, connected to `PL-2` | responsibility refinement in ALIGN-2; implementation inactive I5 |
| Shared-Space platform | mainly `S5`, realized through `S4/S6` | `PL-4` consuming `PL-1--3` | responsibility-only; no active implementation phase claim |
| domain kits / Reversed Library | `S5`, with lower checked dependencies | `PL-5` / separate `PL-6` | application-specific and not lower lifecycle acceptance |

Thus `S6 Host != PL-0`, `S5 Domain != PL-5`, and `I2 != PL-2`.
Semantic placement, product responsibility, and evidence maturity answer different questions.

## PL-4 Shared-Space / World-Web boundary

**Position in stack:** above Mir/Mirrorea and safe Browser/Host participation,
below concrete Domain Kits/applications and the separate Reversed Library project.

**Lower-layer requirements:** checked general semantics; owner/authority/lifetime/failure;
generated placement and communication; safe third-party participation; observer-safe
projection; checked evolution. These requirements do not prescribe a public wire or product UI.

**Upper-layer promises:** a future program may define persistent, participatory,
programmable, composable spaces/objects/relations with multiple Views without making
their domain nouns lower primitives.

**Non-primitives:** `World`, `Room`, `Avatar`, `Portal`, `Library`, `Page`, `Website`,
address, URL, marketplace and renderer are not introduced into Mir Core by this layer.

**Deferred questions:** concrete addressing, linking, discovery, publication,
federation, governance, package/origin format, storage catalog, public compatibility,
and product UX are **UNRESOLVED**.

**Future owner clarification:** a later bounded program must identify the concrete user
capability, threat/authority model, persistence/federation scope, interoperability and
public compatibility boundary before choosing those mechanisms. ALIGN-1 supplies no
implicit answer.

## Satellites and one-way dependency rule

PrismCascade and the Typed-Effect Wiring Platform remain the separable satellites of
`arch/05-satellites`; they are not numbered PLs and are not absorbed into PL-2 or PL-3.
They may connect through narrow typed provider/effect/trace seams. Their performance or
workflow needs cannot alter Mir semantic ownership, authority, failure, ordering,
observation, lifetime or patch meaning.

Upper-layer needs may request an explicit lower-layer capability through a later Canon
decision. They cannot silently become lower-layer semantic inputs. Lower realization
details may implement an admitted contract; they cannot become the meaning of a domain
or an authority source.

## Non-claims

This map chooses no package/origin/storage format, public grammar/CLI/API/ABI/wire,
browser engine, sandbox technology, transport, deployment topology, Unity/Unreal ABI,
domain kit, Shared-Space vocabulary/governance, or Reversed Library product. It changes
no accepted I2 runtime behavior, official lifecycle state, proof/OBL status, or public /
production claim. Detailed Browser/Host/View/provider trust and resource contracts are
the separate ALIGN-2 direct consumer.
