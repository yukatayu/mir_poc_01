# WRK-0020 - Option admit carrier literal audit result

## status

This LAB memo records the registered falsifier for
`mirrorea_canon/working/WRK-0020-option-admit-carrier-literal-audit.md`.
The Canon working record is `L3-open, frozen`; this memo is not a Canon
carrier decision or a corrected experiment.

## registered execution

The registration at `fec0a0da` required running one exact command only after
push. The command wrapped its Python source in shell double quotes and included
the source-text assertion:

```text
assert 'fixture-side `OptionDecl.admit` handoff' in text['plan/07-parser-free-poc-stack.md']
```

The embedded backticks were not escaped for the enclosing shell double-quoted
argument. Bash therefore attempted command substitution before invoking Python.

## observed falsifier

The registered execution emitted:

```text
/usr/bin/bash: line 2: OptionDecl.admit: command not found
Traceback (most recent call last):
  File "<string>", line 1, in <module>
AssertionError
```

The shell fragment that printed `WRK-0020 registered literal audit: pass` was
outside the registered command and ran because the wrapper did not stop after
the assertion. It is not an audit success, and no source-literal conclusion is
retained.

## root-cause boundary

The failure arises at the shell-to-Python quoting boundary in the immutable
pre-registered command. It does not show that `theory/01`, `theory/06`, the
Surface grammar, the companion notation, `plan/07`, or e3 agree or disagree.
Escaping the backticks or re-running a repaired command would alter the frozen
pre-registration and is prohibited for WRK-0020.

## permitted follow-up

A future record may formulate a different source-audit command and its own
falsifier after independently checking duplication and scope. It must not
claim to repair, validate, or promote WRK-0020, and it must not choose an
Option/constraint/residual/other carrier merely because the prior command was
malformed.

## non-claims

No Core representation, fallback semantics, grammar, parser, current-L2
interpretation, runtime behavior, theorem/OBL premise, Gate/Phase state, or
public status follows from this failed command.
