"""Structural, type-sensitive equality for admitted logical records.

Python's bool/int coercion is not object-language equality. This helper assumes
that object inputs have already been restricted to the model's record classes;
it is not a safe decoder for arbitrary hostile Python objects.
"""
from dataclasses import fields, is_dataclass

def exact(a, b):
    if type(a) is not type(b):
        return False
    if is_dataclass(a):
        return all(exact(getattr(a, f.name), getattr(b, f.name)) for f in fields(a))
    if isinstance(a, (tuple, list)):
        return len(a) == len(b) and all(exact(x, y) for x, y in zip(a, b))
    if isinstance(a, dict):
        # Name-keyed records are normal. Matching also rejects True/1 aliases.
        return len(a) == len(b) and all(
            any(exact(k, k2) and exact(v, v2) for k2, v2 in b.items())
            for k, v in a.items()
        )
    if isinstance(a, (set, frozenset)):
        return len(a) == len(b) and all(any(exact(x, y) for y in b) for x in a)
    return a == b
