# CurrentL2IfcSecretExamples.lean

## 要約

- secret-key valid/invalid と explicit authority declassification を、valid/invalid witness 付きで固定する IFC concrete example 集。

## このファイルを置く理由

- Package 56 の first-fragment を label model の定義だけで止めず、secret-key valid/invalid と explicit authority declassification を mechanization-ready な concrete example として置く。valid pattern がなぜ通るか、invalid pattern がなぜ witness を持てないかを、payload preservation lemma と concrete witness で読めるようにする。
- valid pattern がなぜ通るか、invalid pattern がなぜ witness を持てないかを、sample-facing に追いやすい小さな補題と example で固定する。
- 生成された current-L2 sample stub と違い、このファイルは `sorry` ではなく実際に小さな証明を含む。
- ただし依然として helper-local / non-production cut に留める。目的は first mechanization-ready core を固定することであり、final public type system や verifier contract を凍らせることではない。
## WRK-0018 telemetry-effect dependency model

- experiment-local `Nat`/`Bool` configuration と一列の model row だけで、同じ modeled low position から低値依存 telemetry が同じ row を返す正例と、高値にも依存する telemetry が異なる row を返す固定負例を並べる。
- これは THM-005 の証明・反証、telemetry effect の実装、label lattice、declassification、occurrence provenance、devtools/export ABI ではない。有限モデルで、low-agreement だけでは高値依存の model export equality を導けないという前提境界を示す。
- 実験尾部は WRK-0018 の source-hash / marker guard で既存 IFC fragment と分離される。一般 helper、production behavior、sample workflow は追加しない。
