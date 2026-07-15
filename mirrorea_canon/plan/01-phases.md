---
id: plan/01-phases
status: L1-fixed
maturity: draft
depends_on: [plan/00-gates, plan/02-operating-model, spec/06-conformance, meta/source-hierarchy, adr/ADR-0013]
summary: 実装フェーズ T0-T2 / I1-I6。各段階の正確なゴール、何が動き、どの程度実用的か、非宣言。
open_items: [OPEN-032]
---

# 01 — フェーズ計画

**唯一の実装状態の正本。** 他のどのファイルの存在も実装を意味しない。現在位置: **T0**。

## 理論フェーズ(実装凍結。例外は使い捨て spike のみ、main 不合流)

| Phase | ゴール(exit) | 動くもの / 実用性 |
|---|---|---|
| T0 語彙と決定 | G0 exit。canon 発効、LAB 格下げ、旧語彙注記 | 何も動かない。判断の基準器が立つ |
| T1 計算体系 | G1 exit + G2/G3 の statement 群。SCN 期待の最終化 | 紙と Lean statement。以後の全実装の仕様が確定 |
| T2 骨格証明 | OBL-020/021/002 の証明骨格、G5 statement 群 | Lean 上で核が回る。理論の破綻はここまでに露見する |

## 実装フェーズ

| Phase | ゴール(exit criteria) | 動くもの | 実用性 | 非宣言 |
|---|---|---|---|---|
| I1 参照実装 | mir-parse/check/elab/run が C-static+C-runtime 10/10、carrier 凍結(arch/04) | 単一プロセスで全 SCN | 教育・検証用。言語に触れる | 性能・分散・永続 |
| I2 多 locus | プロセス内 multi-place、生成通信の実 dispatch、devtools 最小 panel | ローカル toy world | 一人で遊べる箱庭 | 実網・耐障害 |
| I3 実 transport | 2 OS プロセス+実 socket、C-distributed(SCN-01/02/03/06) | LAN で双六が二人で遊べる | 最初の「本物が動く」点 | WAN・セキュリティ強度 |
| I4 永続と patch | save/load(local durable)、ライブ patch(SCN-09 を実セッションで) | 落として上げ直せる world | 継続世界の試作 | 分散 durable(R3/R4) |
| I5 射影と View | ブラウザ client への projection、View FFI(pose 契約)、viewer devtools | 人に見せられる仮想空間デモ | デモ可能な α | 最終 ABI・複数エンジン |
| I6 分散永続と連合 | R3/R4、複数サーバ、federation 入口、限定公開 | 招待制の常設小世界 | 限定公開 α | 一般公開・スケール保証 |

## T0/G0 phase-governance profile

`phase-governance/t0-g0` version 1 is the **only** T0 interpretation of the
universal JSON condition below. It is a phase-governance profile, not an
implementation, an `arch/03-toolchain` `mir-conform` tool output, or
an SCN conformance result under `spec/06-conformance`.

### Definition and scope

- **Profile identity:** `phase-governance/t0-g0`, version `1`, kind
  `phase-governance-evaluation`.
- **Normative source:** this document under ADR-0013. Let `B` be the commit
  that first records the single artifact recorded by `LAB:plan/155`.
  `profile.source_revision` must be the immediate first parent `C` of `B`.
  `C` must descend from the accepted repository evidence cut, contain the
  complete ADR-0013/profile-v1/INDEX adoption patch, have a first parent that
  lacks at least one of the exact ADR-0013 or profile-v1 blobs in `C`, and lack
  the artifact from its own tree. No other revision is valid.
- **Checks:** `g0-substantive-owner-record`,
  `g0-source-hierarchy-controls`, and `g0-demotion-audit-scope`.
- **Inputs:** the exact ADR-0013 owner record; the five ADR and
  `root/glossary` hashes plus `LAB:plan/153` evidence revision accepted by that
  record; `meta/source-hierarchy`; and the concrete LAB controls below.
- **Result:** `pass` when all checks pass; `pending` when required evidence is
  absent or ambiguous; `fail` when evidence conflicts or a required hash does
  not match. No other result value is valid.

`g0-substantive-owner-record` passes only when the cited ADR expressly accepts
the exact five ADRs as effective for G0, the named GLOSSARY baseline as
prepared, and the named LAB-demotion evidence as complete for its checkpoint.
`g0-source-hierarchy-controls` passes only when the `meta/source-hierarchy`
blob at `profile.source_revision` matches the accepted evidence-cut hash and
every following blob exists and matches the listed SHA-256. This is a
byte-level control pin, not a newly inferred semantic audit.

| Control id | Blob path | SHA-256 |
| --- | --- | --- |
| `lab-root-canon-notice` | `CANON.md` | `92f4538cc71eceab4e65c7f6d8d11b496ef9b907e4869fea06c2fcc7fdeae209` |
| `lab-root-readme-notice` | `README.md` | `008fcbfbcf8d364bb7bd9ce078e446e841795c561650b5bcc8ec3e26570ab10f` |
| `lab-agents-notice` | `AGENTS.md` | `84754bf3d9855160e22b865421dbf6c18622a5a135a5ad77d1cc90d14ce2190b` |
| `lab-highlighter-legacy-vocabulary` | `mir_hilight.html` | `d1d81ec317b2cccff60234ee728129391902a2f804d60c80f5744bf0b8442aee` |
| `lab-clean-suite-classification` | `samples/clean-near-end/README.md` | `fcc6488461860af685a34139d3d953eb6c4f83e77ab23fe281b00b1483b518d9` |

These pins represent the LAB `CANON.md`/root `README.md`/`AGENTS.md` notices,
the highlighter's `world`/`game` legacy treatment, and clean-suite evidence
classification. They do not establish historical implementation-freeze
provenance or universal future compliance with the ongoing step-5 rule.
`g0-demotion-audit-scope` passes only when the cited ADR expressly says whether
the separate audit is waived or required; if required, it cannot pass before
the accepted scoped audit evidence is bound.

### Derived artifact binding

The only authorized production route is a one-off ephemeral evaluation that
produces one derived JSON artifact recorded by `LAB:plan/155`. No producer
implementation, generator, helper, schema, CI surface, Make target, evidence
lane, or report series is committed. The artifact is LAB evidence.

The version-1 artifact is one JSON object with **exactly** these top-level
members and no others:

```json
{
  "kind": "phase-governance-evaluation",
  "profile": {
    "id": "phase-governance/t0-g0",
    "version": 1,
    "source_id": "plan/01-phases",
    "source_revision": "<40-lowercase-hex-git-commit>",
    "source_sha256": "<64-lowercase-hex>"
  },
  "owner_record": {
    "id": "adr/ADR-0013",
    "path": "<repo-relative-blob-path>",
    "sha256": "<64-lowercase-hex>"
  },
  "accepted_evidence_cut": {
    "repository_revision": "6f96ce17e74173ca5d86ed76cee3db75d60dcbfe",
    "items": [
      {"id": "adr/ADR-0001", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"},
      {"id": "adr/ADR-0002", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"},
      {"id": "adr/ADR-0005", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"},
      {"id": "adr/ADR-0009", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"},
      {"id": "adr/ADR-0012", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"},
      {"id": "root/glossary", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"},
      {"id": "meta/source-hierarchy", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"},
      {"id": "LAB:plan/153", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"}
    ]
  },
  "current_controls": {
    "source_hierarchy": {"id": "meta/source-hierarchy", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"},
    "lab_controls": [
      {"id": "lab-root-canon-notice", "path": "CANON.md", "sha256": "<64-lowercase-hex-or-null>"},
      {"id": "lab-root-readme-notice", "path": "README.md", "sha256": "<64-lowercase-hex-or-null>"},
      {"id": "lab-agents-notice", "path": "AGENTS.md", "sha256": "<64-lowercase-hex-or-null>"},
      {"id": "lab-highlighter-legacy-vocabulary", "path": "mir_hilight.html", "sha256": "<64-lowercase-hex-or-null>"},
      {"id": "lab-clean-suite-classification", "path": "samples/clean-near-end/README.md", "sha256": "<64-lowercase-hex-or-null>"}
    ]
  },
  "checks": [
    {"id": "g0-substantive-owner-record", "result": "<pass|pending|fail>", "evidence": [{"id": "<required-id>", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"}]},
    {"id": "g0-source-hierarchy-controls", "result": "<pass|pending|fail>", "evidence": [{"id": "<required-id>", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex-or-null>"}]},
    {"id": "g0-demotion-audit-scope", "result": "<pass|pending|fail>", "evidence": [{"id": "<required-id>", "path": "<repo-relative-blob-path>", "sha256": "<64-lowercase-hex>"}]}
  ],
  "result": "<derived-pass|pending|fail>",
  "non_claims": ["SCN conformance", "C-static", "C-runtime", "C-distributed", "proof", "ADR effectivity", "Gate exit", "T1 entry"],
  "artifact_digest": {
    "algorithm": "sha256",
    "canonicalization": "RFC8785",
    "scope": "canonical-json-with-artifact_digest-omitted",
    "value": "<64-lowercase-hex>"
  }
}
```

Every evidence item is exactly an object with string `id`, string `path`, and
`sha256`. Except for the five `current_controls.lab_controls`, `sha256` is a
64-lowercase-hex string. A listed LAB control whose path is absent retains its
array position with `sha256: null`; every present control has a 64-lowercase-hex
string. Each check is exactly an object with string `id`, `result` in
`pass`/`pending`/`fail`, and `evidence`, an array of evidence items. No object
accepts additional members. `profile.version` is the only number and is integer
`1`; floats and non-finite numbers are forbidden.

`accepted_evidence_cut.repository_revision` is the repository evidence cut
`6f96ce17e74173ca5d86ed76cee3db75d60dcbfe`. Its `items` have this exact order:

1. `adr/ADR-0001`, `adr/ADR-0002`, `adr/ADR-0005`, `adr/ADR-0009`,
   `adr/ADR-0012` (each with its canonical blob path and accepted-cut hash);
2. `root/glossary` and `meta/source-hierarchy` (each with its canonical blob
   path and accepted-cut hash); and
3. `LAB:plan/153` with its LAB blob path and accepted-cut hash.

`profile.source_sha256` is SHA-256 of exact Git blob bytes at
`profile.source_revision:mirrorea_canon/plan/01-phases.md`.
`owner_record.path` is `mirrorea_canon/adr/ADR-0013.md`, and
`owner_record.sha256` is SHA-256 of exact Git blob bytes at that path and
`profile.source_revision`. `accepted_evidence_cut.items[*].sha256` is SHA-256
of the named blob at `accepted_evidence_cut.repository_revision`.
`current_controls.source_hierarchy.sha256` and every non-null
`current_controls.lab_controls[*].sha256` are SHA-256 of their named blobs at
`profile.source_revision`. Every repeated check-evidence item must have exactly
the same `id`, `path`, and `sha256` as its corresponding top-level binding.

`current_controls.source_hierarchy` is the `meta/source-hierarchy` item read
from `profile.source_revision`; its hash must equal the accepted-cut hash.
`current_controls.lab_controls` has the exact five-row table order above.

`checks` has this exact order and exact `evidence` array order:

1. `g0-substantive-owner-record`: owner record, `adr/ADR-0001`,
   `adr/ADR-0002`, `adr/ADR-0005`, `adr/ADR-0009`, `adr/ADR-0012`,
   `root/glossary`, `LAB:plan/153`.
2. `g0-source-hierarchy-controls`: `meta/source-hierarchy`, then the five
   LAB controls in the table order.
3. `g0-demotion-audit-scope`: owner record, `LAB:plan/153`.

The root `result` is derived, never independently selected: `fail` if any
check is `fail`; otherwise `pending` if any check is `pending`; otherwise
`pass`. A missing fixed-tree control or hash mismatch is `fail`. `pending` is
reserved for a declared non-conflicting prerequisite that is not yet bound or
whose canonical owner record is genuinely ambiguous. A forbidden member,
incorrect type/cardinality/order, malformed JSON, non-RFC-8785 input, or
incorrect digest makes an invalid artifact that consumers reject; it is not a
valid artifact whose root result is `fail`.

`non_claims` has this exact order: `SCN conformance`, `C-static`, `C-runtime`,
`C-distributed`, `proof`, `ADR effectivity`, `Gate exit`, `T1 entry`. The
artifact must not use the `conformance` carrier or SCN pass/fail fields.

For `artifact_digest`, omit the complete top-level `artifact_digest` member and
serialize the remainder according to RFC 8785 JSON Canonicalization Scheme as
UTF-8. The SHA-256 of those bytes is `artifact_digest.value`. Every listed file
hash is SHA-256 of exact Git blob bytes at the cited revision, never a working
tree read. A consumer must reject a stored digest that differs from the
calculated value.

### Exit boundary

A `pass` is necessary evidence for this T0 profile only. It does not accept
itself, exit G0, enter T1, make an additional ADR effective, or change the
current implementation state. G0 exit still requires a separate human
acceptance of the exact digest and a canonical exit record. ADR-0013 defers
that decision, so the current phase remains T0 even after a valid `pass`.

各 Phase の exit は、その Phase に canonically defined JSON profile の合否と
人間の受理で成立する。T0 は本節の `phase-governance/t0-g0` を用いる。SCN の
C-static / C-runtime / C-distributed 適合性 JSON は `spec/06-conformance` と
`arch/03-toolchain` の `mir-conform` contract に従う。**Phase を跨ぐ最適化の先取りは禁止**(BND-006 の意味保存を先に)。OPEN-032: I3 の transport 選定(候補: QUIC/WebTransport 系)は I2 exit 時に ADR で決定。
