# product-alpha1 docker

This directory holds the Product/Public-ready alpha-1 Docker Compose TCP
transport fixture for `mirrorea-alpha transport --mode docker`.

- `docker-compose.product-alpha1.yml`
  - mounts the already-built `mirrorea-alpha` binary, the selected local session
    JSON file, and an output directory
  - uses `ubuntu:24.04` so the mounted host-built alpha binary has a compatible
    glibc baseline in this controlled fixture
  - runs a world server container and a participant client container over Docker
    Compose TCP
  - writes observer-safe endpoint reports to `world.json` and `participant.json`
  - gates the internal endpoint helper commands with fixture-only environment
    variables and a per-run fixture token; this is an accidental-use guard for
    the controlled Docker fixture, not a security boundary or advertised public
    CLI family
  - executes the already-built host CLI binary as transport fixture
    infrastructure only. It does not execute package-native code, and the
    package `NativeExecutionPolicy` remains `disabled`.

This is a controlled local Docker Compose alpha boundary. It does not imply
WAN/federation, production orchestration, final public transport ABI, or
distributed durable save/load.
