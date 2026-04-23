# Report 0910 — single HTML Mir sample highlighter

- Date: 2026-04-23 22:54 JST
- Author / agent: Codex
- Scope: repo-local sample readability helper, docs mirror, regression test, validation.
- Decision levels touched: helper/documentation current-L2 support only; no new normative language decision.

## 1. Objective

Add a root-level `mir_hilight.html` that works as a self-contained browser viewer for current active `.mir` samples.

Required behavior:

- single HTML file only at runtime;
- no external script, stylesheet, CDN, or network fetch;
- default Solarized Dark theme;
- selectable VS Code / GitHub / Dracula / Monokai style themes;
- line numbers;
- mobile-responsive layout;
- highlight reserved words separately from code-defined user symbols;
- document that syntax/token extraction must be updated when grammar or active sample paths change.

## 2. Scope and assumptions

The viewer targets active current samples under `samples/clean-near-end/**/*.mir`.
It intentionally excludes `samples/old/`, Lean files, old `.txt` prototypes, and generated proof artifacts.

The highlighter is a reader aid, not a parser, checker, verifier, or normative grammar. It performs local tokenization and symbol extraction inside the browser. It does not execute Mir code.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `docs/research_abstract/README.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `progress.md`

## 4. Actions taken

- Added `mir_hilight.html` at repo root.
- Embedded all 27 current active `.mir` samples from `samples/clean-near-end/**/*.mir`.
- Implemented inline CSS themes:
  - Solarized Dark
  - Solarized Light
  - VS Code Dark
  - GitHub Light
  - GitHub Dark
  - Dracula
  - Monokai
- Implemented inline JavaScript for:
  - sample list and filter;
  - theme switching;
  - path copy;
  - line-number rendering;
  - keyword highlighting;
  - user-defined symbol extraction and separate highlighting.
- Added regression tests in `scripts/tests/test_mir_hilight_html.py`.
- Updated `README.md`, `Documentation.md`, `docs/research_abstract/README.md`, `plan/09-helper-stack-and-responsibility-map.md`, and `progress.md`.
- `tasks.md` 更新不要: current task map の next package / blocker は変わらない。

## 5. Files changed

- `mir_hilight.html`
- `scripts/tests/test_mir_hilight_html.py`
- `README.md`
- `Documentation.md`
- `docs/research_abstract/README.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `progress.md`
- `docs/reports/0910-single-html-mir-sample-highlighter.md`

## 6. Evidence / outputs / test results

TDD red check:

```text
$ python3 -m unittest scripts.tests.test_mir_hilight_html -v
FAILED (errors=4)
FileNotFoundError: /home/yukatayu/dev/mir_poc_01/mir_hilight.html
```

The red failure was expected because the root HTML did not exist yet.

Generated HTML inventory check:

```text
$ python3 - <<'PY'
... parse mir_hilight.html sample JSON ...
PY
27
samples/clean-near-end/00_index_theories.mir
samples/clean-near-end/typing/05_cost_bound_rejected.mir
```

JavaScript syntax check:

```text
$ node --check /tmp/mir_hilight_inline.js
exit 0
```

Focused regression:

```text
$ python3 -m unittest scripts.tests.test_mir_hilight_html -v
test_documents_update_timing_for_grammar_changes ... ok
test_embeds_every_active_clean_near_end_mir_sample ... ok
test_single_file_viewer_exists_without_external_dependencies ... ok
test_supports_required_highlight_features_and_themes ... ok

Ran 4 tests in 0.003s
OK
```

Full Python test discovery:

```text
$ python3 -m unittest discover scripts/tests -v
Ran 114 tests in 0.240s
OK
```

Docs validation:

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 908 numbered report(s).
```

Whitespace check:

```text
$ git diff --check
exit 0
```

## 7. What changed in understanding

`mir_hilight.html` is best treated as a repo-local sample readability helper. It should not be promoted into a final parser or final public syntax contract.

The visual helper boundary is:

- active `.mir` sample viewer;
- embedded sample corpus;
- deterministic browser-side syntax coloring;
- no hidden network dependency;
- no normative language decision.

## 8. Open questions

- Should future active sample families outside `samples/clean-near-end/**/*.mir` be included by default, or should the viewer stay scoped to the active clean near-end suite?
- Should a small committed generator be added later to rebuild `mir_hilight.html` automatically when sample files change?
- Should final public docs eventually move this viewer behind a generated docs site rather than a root-level HTML file?

## 9. Suggested next prompt

Add a small maintainer command such as `python3 scripts/build_mir_hilight_html.py` that regenerates `mir_hilight.html` from active sample files, then update the regression test to require the checked-in HTML to match generated output.
