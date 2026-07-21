# Host adapter ambient capability negative

`write_int@host_output` is declared with its required `HostWrite` capability,
but the transition's ambient row omits that capability. The checker must reject
the call before runtime.
