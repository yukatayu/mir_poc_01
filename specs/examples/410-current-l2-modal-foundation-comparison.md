# 410 — current L2 modal foundation comparison

## 目的

stage / later / always の partial basis と、
place / scope / authority / witness / durability を含む richer basis を切り分ける。

## comparison family

| family | strengths | likely gap for current repo |
|---|---|---|
| `lambda-circle-box` | stage / next / always、time-ordered normalization | place / scope / authority / witness / durability の multi-axis |
| guarded recursion / guarded lambda-calculus | productivity、later-style reasoning、coinductive stream line | authority / scope / witness decomposition をそのままは与えない |
| Modal Dependent Type Theory | richer modal dependent basis、Fitch-style line | current repo には still heavy |
| Multimodal Dependent Type Theory | multiple interacting modalities、mode theory | current repo には still heavier |

## current judgment

1. `lambda-circle-box` は partial basis candidate として有望である。
2. ただし full language foundation をこれだけで閉じるのは早い。
3. guarded / MDTT / MTT は stronger foundation candidate として comparison に残す。

## current wording

- stage / later / always の micro-coreには `lambda-circle-box` line を使ってよい
- full language の final modal foundation は OPEN

## what is not decided here

- final adopted calculus
- source-surface modality markers
- checker / theorem / runtime への concrete integration
