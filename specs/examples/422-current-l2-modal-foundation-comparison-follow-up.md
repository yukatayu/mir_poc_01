# 422 — current L2 modal foundation comparison follow-up

## 目的

`specs/examples/410-current-l2-modal-foundation-comparison.md`
の partial-basis note を前提に、

- partial basis candidate
- stronger candidate family
- current reduction
- stop line

を docs-first に整理する。

## source-backed floor

- `lambda-circle-box` は stage / later / always の micro-core では useful な partial basis candidate である。
- full language は place / scope / visibility / authority / witness / durability の multi-axis を要する。
- final foundation adoption は still OPEN である。

## comparison family

| family | current role | strengths | current gap |
|---|---|---|---|
| `lambda-circle-box` | partial basis candidate | stage / later / always、time-ordered normalization | multi-axis modality を直接は与えない |
| guarded lambda-calculus | first stronger adjacent candidate | productivity / later reasoning / stream-like line に強い | authority / witness / scope decomposition をそのままは与えない |
| Modal Dependent Type Theory | stronger candidate family | richer modal dependent basis、Fitch-style line | current repo では heavy |
| Multimodal Dependent Type Theory | stronger candidate family | multiple interacting modalities、mode theory | current repo では heavier だが multi-axis への適合が高い |

## current judgment

1. `lambda-circle-box` は **partial basis** に留める。
2. stronger candidate family は **guarded lambda-calculus と MDTT/MTT cluster** に reduce する。
3. current repo の multi-axis 読みでは、もし stronger candidate に lead を置くなら `MTT` family が最も自然だが、これは working hypothesis であって adoption ではない。

## stop line

current package では次を決めない。

- final adopted calculus
- source-surface modality markers
- checker / theorem / runtime への concrete integration
- full dependent calculus

## next promoted line

next promoted line は、
**modal promotion-threshold note**
に置く。

## what is not decided here

- guarded と MDTT/MTT の exact reduction timing
- place / authority / witness を独立 modality として internalize するか
- final foundation adoption
