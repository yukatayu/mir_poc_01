# practical-alpha1 packages

Each subdirectory in this root is a narrow practical alpha-1 package fixture.

- The current entrypoint is `package.mir.json`.
- The loader may receive either the package directory or the file path itself.
- `SRC-*` fixtures are parser/loader inputs.
- `CHK-*` fixtures are parser/loader inputs plus `alpha_local_checker_input` for the first practical checker floor.
- `alpha_local_checker_input` is sample/test expectation input for the non-final alpha-local checker floor, not a public package schema freeze.
- These fixtures are not current-scope alpha evidence sidecars.
