# Report 0911 — Mir highlighter custom input

- Date: 2026-04-23 23:04 JST
- Author / agent: Codex
- Scope: `mir_hilight.html` UI extension, docs mirror, regression test, validation.
- Decision levels touched: repo-local helper UI only; no normative syntax or parser decision.

## 1. Objective

Extend `mir_hilight.html` so users can paste custom Mir-like code and preview it with the same browser-side highlighter used for embedded active samples.

Also answer and document the CSS basis: the viewer uses hand-written original CSS embedded in the HTML, not an external framework.

## 2. Scope and assumptions

The custom input is browser-local. It does not save code, send network requests, call a checker, or execute Mir. It reuses the current keyword list, tokenizer, and symbol extraction rules inside `mir_hilight.html`.

## 3. Documents consulted

- `mir_hilight.html`
- `scripts/tests/test_mir_hilight_html.py`
- `README.md`
- `Documentation.md`
- `docs/research_abstract/README.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `progress.md`

## 4. Actions taken

- Added a `Custom source` panel to `mir_hilight.html`.
- Added a textarea for custom code.
- Added buttons:
  - `Load current sample`
  - `Highlight custom code`
  - `Clear`
- Added live re-highlight while editing custom code after custom mode is active.
- Kept the viewer self-contained with no external scripts/stylesheets/network fetches.
- Added tests for custom input wiring.
- Updated docs to state that CSS is hand-written original CSS and that custom input UI must be updated when grammar/UI assumptions change.
- `tasks.md` 更新不要: current task map and blocker ordering did not change.

## 5. Files changed

- `mir_hilight.html`
- `scripts/tests/test_mir_hilight_html.py`
- `README.md`
- `Documentation.md`
- `docs/research_abstract/README.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `progress.md`
- `docs/reports/0911-mir-highlighter-custom-input.md`

## 6. Evidence / outputs / test results

TDD red check:

```text
$ python3 -m unittest scripts.tests.test_mir_hilight_html -v
FAILED (failures=1)
AssertionError: 'id="custom-source"' not found
```

Focused regression after implementation:

```text
$ python3 -m unittest scripts.tests.test_mir_hilight_html -v
Ran 5 tests in 0.005s
OK
```

Inline JavaScript syntax check:

```text
$ node --check /tmp/mir_hilight_inline.js
exit 0
```

Full Python test discovery:

```text
$ python3 -m unittest discover scripts/tests -v
Ran 115 tests in 0.236s
OK
```

Docs validation:

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 909 numbered report(s).
```

Whitespace check:

```text
$ git diff --check
exit 0
```

## 7. What changed in understanding

The highlighter remains a readability helper, not a checker. Custom input makes it useful for drafting and comparing syntax, but it does not imply that arbitrary pasted code is accepted by any current parser or verifier.

CSS basis is now explicit: no framework, no theme library, no external stylesheet; styles are original CSS variables and component rules in the single HTML file.

## 8. Open questions

- Should custom input be persisted in `localStorage`, or should it remain ephemeral for privacy and simplicity?
- Should a future generator own both embedded samples and the custom-input UI so manual HTML edits are avoided?
- Should the custom input eventually expose parser/checker output when a final public checker exists?

## 9. Suggested next prompt

Add a `scripts/build_mir_hilight_html.py` generator that rebuilds `mir_hilight.html` from active sample files and a small template, then require the checked-in HTML to match generated output in tests.
